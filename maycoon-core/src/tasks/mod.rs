use crate::config::TasksConfig;
use crate::tasks::runner::{Task, TaskRunner};
use std::sync::OnceLock;

/// Contains the [TaskRunner] struct.
pub mod runner;

static RUNNER: OnceLock<TaskRunner> = OnceLock::new();

/// Initializes the task runner with the given config. Panics if the task runner is already initialized.
pub fn init(config: TasksConfig) {
    RUNNER
        .set(TaskRunner::new(config))
        .expect("Task runner already initialized")
}

/// Try to get the global task runner. Returns [None] if the task runner is not initialized yet.
pub fn try_runner<'a>() -> Option<&'a TaskRunner> {
    RUNNER.get()
}

/// Get the global task runner. Panics if the task runner is not initialized yet.
pub fn runner<'a>() -> &'a TaskRunner {
    try_runner().expect("Task runner not initialized")
}

/// Block on the given future on the task runner. Panics if the task runner is not initialized yet.
///
/// If the task runner is not initialized (e.g. if no task runner config is specified), [pollster] will be used.
pub fn block_on<F>(fut: F) -> F::Output
where
    F: Future,
{
    if let Some(runner) = try_runner() {
        runner.block_on(fut)
    } else {
        pollster::block_on(fut)
    }
}

/// Spawn the given future on the task runner. Panics if the task runner is not initialized yet.
pub fn spawn<F>(fut: F) -> Task<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    runner().spawn(fut)
}

/// Spawn the given blocking function on the task runner. Panics if the task runner is not initialized yet.
pub fn spawn_blocking<F, R>(fut: F) -> Task<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    runner().spawn_blocking(fut)
}
