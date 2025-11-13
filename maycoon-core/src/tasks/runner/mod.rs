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
    /// A dummy task runner that will panic on any method call.
    /// Useful if you want to disable the task runner feature,
    /// but still want the build script to work (e.g. for libraries).
    #[cfg(feature = "dummy-runner")]
    Dummy,
}

impl TaskRunner {
    /// Spawns a task, possibly in the background.
    ///
    /// Returns a task handle to the future.
    ///
    /// Panics, when no valid task runner is enabled.
    #[inline(always)]
    #[tracing::instrument(skip_all)]
    pub fn spawn<Fut>(&self, _future: Fut) -> Box<dyn Task<Fut::Output>>
    where
        Fut: Future + Send + 'static,
        Fut::Output: Send + 'static,
    {
        match self {
            #[cfg(feature = "tokio-runner")]
            TaskRunner::Tokio(rt) => Box::new(rt.spawn(_future)),

            _ => {
                panic!(
                    "No valid task runner feature selected. Please select a `-runner` feature (e.g. `tokio-runner`)."
                );

                // Required for code to compile
                #[allow(unreachable_code)]
                Box::new(crate::tasks::task::NeverTask::new())
            },
        }
    }

    /// Spawns a local task, that will run on the current thread.
    ///
    /// Returns a task handle to the future.
    ///
    /// Panics, when no valid task runner is enabled.
    #[inline(always)]
    #[tracing::instrument(skip_all)]
    pub fn spawn_local<Fut>(&self, _future: Fut) -> Box<dyn LocalTask<Fut::Output>>
    where
        Fut: Future + 'static,
        Fut::Output: 'static,
    {
        match self {
            #[cfg(feature = "tokio-runner")]
            TaskRunner::Tokio(rt) => Box::new(rt.spawn_local(_future)),

            _ => {
                panic!(
                    "No valid task runner feature selected. Please select a `-runner` feature (e.g. `tokio-runner`)."
                );

                // Required for code to compile
                #[allow(unreachable_code)]
                Box::new(crate::tasks::task::LocalNeverTask::new())
            },
        }
    }

    /// Spawns a blocking task in the background.
    ///
    /// Returns a task handle to the operation.
    ///
    /// Panics, when no valid task runner is enabled.
    #[inline(always)]
    #[cfg(native)]
    #[tracing::instrument(skip_all)]
    pub fn spawn_blocking<R, F>(&self, _func: F) -> Box<dyn Task<R>>
    where
        R: Send + 'static,
        F: FnOnce() -> R + Send + 'static,
    {
        match self {
            #[cfg(feature = "tokio-runner")]
            TaskRunner::Tokio(rt) => Box::new(rt.spawn_blocking(_func)),

            _ => {
                panic!(
                    "No valid task runner feature selected. Please select a `-runner` feature (e.g. `tokio-runner`)."
                );

                // Required for code to compile
                #[allow(unreachable_code)]
                Box::new(crate::tasks::task::NeverTask::new())
            },
        }
    }

    /// Blocks on the given future, until it's completed.
    ///
    /// Panics, when no valid task runner is enabled.
    #[inline(always)]
    #[cfg(native)]
    #[tracing::instrument(skip_all)]
    pub fn block_on<Fut>(&self, _future: Fut) -> Fut::Output
    where
        Fut: Future,
    {
        match self {
            #[cfg(feature = "tokio-runner")]
            TaskRunner::Tokio(rt) => rt.block_on(_future),

            _ => panic!(
                "No valid task runner feature selected. Please select a `-runner` feature (e.g. `tokio-runner`)."
            ),
        }
    }

    /// Allows the task runner to perform work on the current thread for one tick.
    ///
    /// The task runner should return as soon as possible (after the first `await` point is reached),
    /// to avoid blocks and freezes.
    #[inline(always)]
    #[tracing::instrument(skip_all)]
    pub fn tick(&self) {
        match self {
            #[cfg(feature = "tokio-runner")]
            TaskRunner::Tokio(rt) => rt.tick(),

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
    type LocalTask<T: 'static>: LocalTask<T>;

    /// Spawns a task, possibly in the background.
    ///
    /// Returns a task handle to the future.
    fn spawn<Fut>(&self, future: Fut) -> Self::Task<Fut::Output>
    where
        Fut: Future + Send + 'static,
        Fut::Output: Send + 'static;

    /// Spawns a local task, that will run on the current thread.
    ///
    /// The task will be executed by the application event loop via [TaskRunnerImpl::tick].
    ///
    /// Returns a task handle to the future.
    fn spawn_local<Fut>(&self, future: Fut) -> Self::LocalTask<Fut::Output>
    where
        Fut: Future + 'static,
        Fut::Output: 'static;

    /// Spawns a blocking task in the background.
    ///
    /// Returns a task handle to the operation.
    #[cfg(native)]
    fn spawn_blocking<R, F>(&self, func: F) -> Self::Task<R>
    where
        R: Send + 'static,
        F: FnOnce() -> R + Send + 'static;

    /// Blocks on the given future, until it's completed.
    #[cfg(native)]
    fn block_on<Fut>(&self, future: Fut) -> Fut::Output
    where
        Fut: Future;

    /// Allows the task runner to perform work on the current thread for one tick.
    ///
    /// The task runner should return as soon as possible (after the first `await` point is reached),
    /// to avoid blocks and freezes.
    fn tick(&self);
}
