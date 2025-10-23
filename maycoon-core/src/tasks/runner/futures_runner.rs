use crate::config::TasksConfig;
use futures::executor::{ThreadPool, ThreadPoolBuilder};
use futures::task::SpawnExt;

/// A task runner using [ThreadPool] as runtime backend.
#[derive(Debug)]
pub struct FuturesRunner {
    pool: ThreadPool,
}

impl FuturesRunner {
    /// Initializes the futures task runner with the given config.
    pub(crate) fn new(config: TasksConfig) -> Self {
        Self {
            pool: ThreadPoolBuilder::new()
                .stack_size(config.stack_size)
                .pool_size(config.workers.get())
                .create()
                .expect("Failed to create futures thread pool"),
        }
    }

    /// Blocks on the given future.
    pub(crate) fn block_on<F>(&self, fut: F) -> F::Output
    where
        F: Future,
    {
        futures::executor::block_on(fut)
    }

    /// Spawns the given future.
    pub(crate) async fn spawn<F>(&self, fut: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.pool
            .spawn_with_handle(fut)
            .expect("Failed to spawn task")
            .await
    }

    /// Spawns the given blocking function. For the futures runner this is the same as [spawn].
    pub(crate) async fn spawn_blocking<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        self.spawn(async { f() }).await
    }
}
