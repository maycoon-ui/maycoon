use crate::tasks::runner::TaskRunner;
use std::future::Future;
use std::sync::OnceLock;

pub use futures::future;

/// Contains the [TaskRunner] struct.
pub mod runner;

/// A handle to a running task which can be awaited in order to get its result.
pub type TaskHandle<T> = future::RemoteHandle<T>;

static RUNNER: OnceLock<TaskRunner> = OnceLock::new();

/// Returns the global [TaskRunner] or panics if it hasn't been initialized yet.
pub fn runner<'a>() -> &'a TaskRunner {
    try_runner().expect("Task runner not initialized yet")
}

/// Returns the global [TaskRunner] or [None] if it hasn't been initialized yet.
pub fn try_runner<'a>() -> Option<&'a TaskRunner> {
    RUNNER.get()
}

/// Spawns the given [Future] on the task runner thread pool and returns a [TaskHandle] to await for the result.
///
/// Panics if the task runner hasn't been initialized yet.
pub fn spawn<Fut>(fut: Fut) -> TaskHandle<Fut::Output>
where
    Fut: Future + Send + 'static,
    Fut::Output: Send,
{
    runner().run(fut)
}

/// Blocks the current thread until the given [Future] completes.
pub fn block_on<Fut>(fut: Fut) -> Fut::Output
where
    Fut: Future,
{
    futures::executor::block_on(fut)
}
