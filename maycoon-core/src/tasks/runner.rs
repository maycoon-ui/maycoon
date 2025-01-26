use crate::tasks::{TaskHandle, RUNNER};
use futures::executor::{ThreadPool, ThreadPoolBuilder};
use futures::task::SpawnExt;
use std::future::Future;
use std::num::NonZeroUsize;

/// The Maycoon Task Runner for running asynchronous tasks using a thread pool from the [`futures`] crate.
///
/// Initialize it using the [`TaskConfig`](crate::config::TasksConfig) inside the [`MayConfig`](crate::config::MayConfig).
pub struct TaskRunner {
    pool: ThreadPool,
}

impl TaskRunner {
    /// Creates a new [`TaskRunner`] with the given thread stack size and worker/thread count.
    pub fn new(stack_size: usize, workers: NonZeroUsize) -> Result<Self, futures::io::Error> {
        Ok(Self {
            pool: ThreadPoolBuilder::new()
                .stack_size(stack_size)
                .pool_size(workers.get())
                .name_prefix("maycoon-worker-")
                .create()?,
        })
    }

    /// Runs the given [`Future`] on the task runner thread pool and returns the output.
    pub fn run<Fut>(&self, fut: Fut) -> TaskHandle<Fut::Output>
    where
        Fut: Future + Send + 'static,
        Fut::Output: Send,
    {
        self.pool
            .spawn_with_handle(fut)
            .expect("Failed to spawn task")
    }

    /// Initialize the global task runner. This cannot be called twice. Will return [`None`] if the task runner is already initialized.
    pub fn init(self) -> Option<()> {
        RUNNER.set(self).ok()
    }
}
