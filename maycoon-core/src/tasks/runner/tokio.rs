use crate::tasks::runner::TaskRunnerImpl;
use crate::tasks::task::{LocalTask, Task};
use crate::tasks::waker::noop_waker;
use fragile::Fragile;
use std::future::ready;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::task::{JoinHandle, LocalSet};

/// A [TaskRunnerImpl] implementation that uses [tokio].
#[derive(Debug)]
pub struct TaskRunner {
    rt: Runtime,
    local: Fragile<LocalSet>,
}

impl TaskRunner {
    /// Creates a new [TaskRunner] with the given configuration.
    ///
    /// Giving [None] to some config parameters will use the default value (that [tokio] uses).
    #[inline(always)]
    pub fn new(
        io: bool,
        stack_size: Option<usize>,
        workers: Option<usize>,
        max_io_events_per_tick: Option<usize>,
        max_blocking_threads: Option<usize>,
        event_interval: Option<u32>,
        global_queue_interval: Option<u32>,
        keep_alive: Option<Duration>,
    ) -> Self {
        let mut builder = Builder::new_multi_thread();

        if io {
            builder.enable_io();
        }

        if let Some(stack_size) = stack_size {
            builder.thread_stack_size(stack_size);
        }

        if let Some(workers) = workers {
            builder.worker_threads(workers);
        }

        if let Some(max_io_events_per_tick) = max_io_events_per_tick {
            builder.max_io_events_per_tick(max_io_events_per_tick);
        }

        if let Some(max_blocking_threads) = max_blocking_threads {
            builder.max_blocking_threads(max_blocking_threads);
        }

        if let Some(event_interval) = event_interval {
            builder.event_interval(event_interval);
        }

        if let Some(global_queue_interval) = global_queue_interval {
            builder.global_queue_interval(global_queue_interval);
        }

        if let Some(keep_alive) = keep_alive {
            builder.thread_keep_alive(keep_alive);
        }

        Self {
            rt: builder
                .enable_time()
                .build()
                .expect("Failed to build tokio runtime"),
            local: Fragile::new(LocalSet::new()),
        }
    }
}

impl TaskRunnerImpl for TaskRunner {
    type Task<T: Send + 'static> = TokioTask<T>;
    type LocalTask<T: 'static> = LocalTokioTask<T>;

    #[inline(always)]
    fn spawn<Fut>(&self, future: Fut) -> Self::Task<Fut::Output>
    where
        Fut: Future + Send + 'static,
        Fut::Output: Send + 'static,
    {
        TokioTask(self.rt.spawn(future))
    }

    fn spawn_local<Fut>(&self, future: Fut) -> Self::LocalTask<Fut::Output>
    where
        Fut: Future + 'static,
        Fut::Output: 'static,
    {
        LocalTokioTask(
            self.local
                .try_get()
                .expect("`spawn_local` must be called from the main thread")
                .spawn_local(future),
        )
    }

    #[inline(always)]
    fn spawn_blocking<R, F>(&self, func: F) -> Self::Task<R>
    where
        R: Send + 'static,
        F: FnOnce() -> R + Send + 'static,
    {
        TokioTask(self.rt.spawn_blocking(func))
    }

    #[inline(always)]
    fn block_on<Fut>(&self, future: Fut) -> Fut::Output
    where
        Fut: Future,
    {
        self.rt.block_on(future)
    }

    fn tick(&self) {
        self.block_on(async {
            self.local
                .try_get()
                .expect("`tick` must be called from the main thread")
                .run_until(async { ready(()).await })
                .await;
        });
    }
}

/// A [Task] implementation that uses [tokio].
pub struct TokioTask<T: Send + 'static>(JoinHandle<T>);

impl<T: Send + 'static> Task<T> for TokioTask<T> {
    #[inline(always)]
    fn is_ready(&self) -> bool {
        self.0.is_finished()
    }

    #[inline(always)]
    fn take(&mut self) -> Option<T> {
        let pinned = Pin::new(&mut self.0);
        let waker = noop_waker();
        let mut ctx = Context::from_waker(&waker);

        match pinned.poll(&mut ctx) {
            Poll::Ready(value) => Some(value.expect("Failed to poll tokio task")),
            Poll::Pending => None,
        }
    }
}

impl<T: Send + 'static> Future for TokioTask<T> {
    type Output = T;

    #[inline(always)]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let pinned = Pin::new(&mut self.0);

        match pinned.poll(cx) {
            Poll::Ready(res) => Poll::Ready(res.expect("Failed to join tokio task")),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// A [LocalTask] implementation that uses [tokio].
pub struct LocalTokioTask<T: 'static>(JoinHandle<T>);

impl<T: 'static> LocalTask<T> for LocalTokioTask<T> {
    #[inline(always)]
    fn is_ready(&self) -> bool {
        self.0.is_finished()
    }

    #[inline(always)]
    fn take(&mut self) -> Option<T> {
        let pinned = Pin::new(&mut self.0);
        let waker = noop_waker();
        let mut ctx = Context::from_waker(&waker);

        match pinned.poll(&mut ctx) {
            Poll::Ready(value) => Some(value.expect("Failed to poll tokio task")),
            Poll::Pending => None,
        }
    }
}

impl<T: 'static> Future for LocalTokioTask<T> {
    type Output = T;

    #[inline(always)]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let pinned = Pin::new(&mut self.0);

        match pinned.poll(cx) {
            Poll::Ready(res) => Poll::Ready(res.expect("Failed to join tokio task")),
            Poll::Pending => Poll::Pending,
        }
    }
}
