use crate::config::TasksConfig;
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;

/// A task runner using [tokio] as runtime.
#[derive(Debug)]
pub struct TokioRunner {
    rt: Runtime,
}

impl TokioRunner {
    /// Initializes the tokio task runner with the given config.
    pub(crate) fn new(config: TasksConfig) -> Self {
        let mut builder = Builder::new_multi_thread();

        Self {
            rt: builder
                .enable_all()
                .worker_threads(config.workers.get())
                .thread_stack_size(config.stack_size)
                .build()
                .expect("Failed to create tokio runtime"),
        }
    }

    /// Blocks on the given future.
    pub(crate) fn block_on<F>(&self, fut: F) -> F::Output
    where
        F: Future,
    {
        self.rt.block_on(fut)
    }

    /// Spawns the given future.
    pub(crate) fn spawn<F>(&self, fut: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.rt.spawn(fut)
    }

    /// Spawns the given blocking function.
    pub(crate) fn spawn_blocking<F, R>(&self, f: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        self.rt.spawn_blocking(f)
    }
}
