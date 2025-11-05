use crate::tasks::task::{LocalTask, Task};
use std::fmt::Debug;

/// Contains implementations of [Task], [LocalTask] and [TaskRunnerImpl] using [tokio].
#[cfg(all(native, feature = "tokio-runner"))]
pub mod tokio;

/// A task runner that can be used to spawn tasks.
///
/// The fields will only be available, if the corresponding feature is enabled.
#[derive(Debug)]
pub enum TaskRunner {
    /// A task runner that uses the [tokio] runtime.
    #[cfg(feature = "tokio-runner")]
    Tokio(tokio::TaskRunner),
    /// Not a real task runner. Panics on any method call.
    None,
}

impl TaskRunner {
    /// Spawns a task, possibly in the background.
    ///
    /// Returns a task handle to the future.
    ///
    /// Panics, when no task runner feature is enabled.
    #[tracing::instrument(skip_all)]
    pub fn spawn<Fut>(&self, future: Fut) -> impl Task<Fut::Output>
    where
        Fut: Future + Send + 'static,
        Fut::Output: Send + 'static,
    {
        match self {
            #[cfg(feature = "tokio-runner")]
            TaskRunner::Tokio(rt) => rt.spawn(future),
            _ => panic!(
                "No valid task runner feature selected. Please select a `-runner` feature (e.g. `tokio-runner`)."
            ),
        }
    }

    /// Spawns a blocking task in the background.
    ///
    /// Returns a task handle to the operation.
    ///
    /// Panics, when no task runner feature is enabled.
    #[cfg(native)]
    #[tracing::instrument(skip_all)]
    pub fn spawn_blocking<R, F>(&self, func: F) -> impl Task<R>
    where
        R: Send + 'static,
        F: FnOnce() -> R + Send + 'static,
    {
        match self {
            #[cfg(feature = "tokio-runner")]
            TaskRunner::Tokio(rt) => rt.spawn_blocking(func),
            _ => panic!(
                "No valid task runner feature selected. Please select a `-runner` feature (e.g. `tokio-runner`)."
            ),
        }
    }

    /// Blocks on the given future, until it's completed.
    ///
    /// Panics, when no task runner feature is enabled.
    #[cfg(native)]
    #[tracing::instrument(skip_all)]
    pub fn block_on<Fut>(&self, future: Fut) -> Fut::Output
    where
        Fut: Future,
    {
        match self {
            #[cfg(feature = "tokio-runner")]
            TaskRunner::Tokio(rt) => rt.block_on(future),
            _ => panic!(
                "No valid task runner feature selected. Please select a `-runner` feature (e.g. `tokio-runner`)."
            ),
        }
    }
}

/// A trait that provides a task runner implementation.
pub trait TaskRunnerImpl: Debug + 'static {
    /// The task type, that this task runner implementation uses.
    type Task<T: Send + 'static>: Task<T>;

    /// The local task type, that this task runner implementation uses.
    type LocalTask<T>: LocalTask<T>;

    /// Spawns a task, possibly in the background.
    ///
    /// Returns a task handle to the future.
    fn spawn<Fut>(&self, future: Fut) -> Self::Task<Fut::Output>
    where
        Fut: Future + Send + 'static,
        Fut::Output: Send + 'static;

    /// Spawns a blocking task in the background.
    ///
    /// Returns a task handle to the operation.
    fn spawn_blocking<R, F>(&self, func: F) -> Self::Task<R>
    where
        R: Send + 'static,
        F: FnOnce() -> R + Send + 'static;

    /// Blocks on the given future, until it's completed.
    fn block_on<Fut>(&self, future: Fut) -> Fut::Output
    where
        Fut: Future;
}
