use crate::tasks::runner::TaskRunnerImpl;
use crate::tasks::task::{LocalTask, Task};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;

/// A [TaskRunnerImpl] implementation that uses [tokio].
#[derive(Debug)]
pub struct TaskRunner {
    rt: Runtime,
}

impl TaskRunner {
    /// Creates a new [TaskRunner] with the given configuration.
    ///
    /// Giving [None] to some config parameters will use the default value (that [tokio] uses).
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
        }
    }
}

impl TaskRunnerImpl for TaskRunner {
    type Task<T: Send + Unpin + 'static> = TokioTask<T>;
    type LocalTask<T: Unpin + 'static> = LocalTokioTask<T>;

    fn spawn<Fut>(&self, future: Fut) -> Self::Task<Fut::Output>
    where
        Fut: Future + Send + Unpin + 'static,
        Fut::Output: Send + Unpin + 'static,
    {
        TokioTask(self.rt.spawn(future))
    }

    fn spawn_blocking<R, F>(&self, func: F) -> Self::Task<R>
    where
        R: Send + Unpin + 'static,
        F: FnOnce() -> R + Send + Unpin + 'static,
    {
        TokioTask(self.rt.spawn_blocking(func))
    }

    fn block_on<Fut>(&self, future: Fut) -> Fut::Output
    where
        Fut: Future,
    {
        self.rt.block_on(future)
    }
}

/// A [Task] implementation that uses [tokio].
pub struct TokioTask<T: Send + 'static>(JoinHandle<T>);

impl<T: Send + Unpin + 'static> Task<T> for TokioTask<T> {
    fn is_ready(&self) -> bool {
        self.0.is_finished()
    }
}

impl<T: Send + Unpin + 'static> Future for TokioTask<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let pinned = Pin::new(&mut self.0);

        match pinned.poll(cx) {
            Poll::Ready(res) => Poll::Ready(res.expect("Failed to join tokio task")),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// A [LocalTask] implementation that uses [tokio].
pub struct LocalTokioTask<T: Unpin + 'static>(JoinHandle<T>);

impl<T: Unpin + 'static> LocalTask<T> for LocalTokioTask<T> {
    fn is_ready(&self) -> bool {
        self.0.is_finished()
    }
}

impl<T: Unpin + 'static> Future for LocalTokioTask<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let pinned = Pin::new(&mut self.0);

        match pinned.poll(cx) {
            Poll::Ready(res) => Poll::Ready(res.expect("Failed to join tokio task")),
            Poll::Pending => Poll::Pending,
        }
    }
}
