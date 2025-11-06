use crate::tasks::runner::TaskRunner;
use crate::tasks::task::Task;
use std::sync::OnceLock;

/// Contains the [TaskRunner] and related structures.
pub mod runner;

/// Contains the [Task] and [task::LocalTask] traits.
pub mod task;

/// Contains a dummy implementation of a waker. Used in [Task::take].
pub mod waker;

/// The global task runner.
static RUNNER: OnceLock<TaskRunner> = OnceLock::new();

/// Initializes the global task runner.
///
/// Panics if the task runner is already initialized.
pub fn init(runner: TaskRunner) {
    RUNNER.set(runner).expect("Task runner already initialized");
}

/// Try to get the global task runner.
/// Returns [None] if the task runner is not initialized.
pub fn try_runner<'a>() -> Option<&'a TaskRunner> {
    RUNNER.get()
}

/// Get the global task runner.
///
/// Panics if the task runner is not initialized.
pub fn runner<'a>() -> &'a TaskRunner {
    RUNNER.get().expect("Task runner not initialized")
}

/// Spawns a task on the global task runner.
///
/// If this actually spawns a task on a background task or just spawns a local task,
/// depends on the [TaskRunner] type that is in use.
pub fn spawn<Fut>(future: Fut) -> Box<dyn Task<Fut::Output>>
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    runner().spawn(future)
}

/// Spawns a blocking task on the global task runner.
///
/// This will always spawn a task in a background thread,
/// as local spawning would block the thread.
///
/// Only available on native platforms.
#[cfg(native)]
pub fn spawn_blocking<R, F>(func: F) -> Box<dyn Task<R>>
where
    R: Send + 'static,
    F: FnOnce() -> R + Send + 'static,
{
    runner().spawn_blocking(func)
}

/// Blocks the current thread on the global task runner until the future is ready.
///
/// Only available on native platforms.
#[cfg(native)]
pub fn block_on<Fut>(future: Fut) -> Fut::Output
where
    Fut: Future,
{
    runner().block_on(future)
}
