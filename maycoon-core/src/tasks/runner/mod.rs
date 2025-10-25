use crate::config::TasksConfig;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Contains the [tokio_runner::TokioRunner] struct.
#[cfg(feature = "tokio-runner")]
pub mod tokio_runner;

/// The task runner enum.
///
/// Contains the [TokioRunner](tokio_runner::TokioRunner) if the 'tokio-runner' feature is enabled.
#[derive(Debug)]
pub enum TaskRunner {
    /// The [tokio] task runner.
    #[cfg(feature = "tokio-runner")]
    Tokio(tokio_runner::TokioRunner),

    /// No task runner selected. Panics on any method call, but blocks on futures.
    ///
    /// Panics on any method call.
    None,
}

impl TaskRunner {
    /// Initializes the task runner with the given config.
    pub fn new(_config: TasksConfig) -> Self {
        #[cfg(feature = "tokio-runner")]
        {
            tracing::info!("creating tokio task runner");
            Self::Tokio(tokio_runner::TokioRunner::new(_config))
        }

        #[cfg(not(feature = "tokio-runner"))]
        {
            tracing::warn!("no task runner feature selected, but task runner config specified");
            Self::None
        }
    }

    /// Blocks on the given future.
    pub fn block_on<F>(&self, _fut: F) -> F::Output
    where
        F: Future,
    {
        match self {
            #[cfg(feature = "tokio-runner")]
            TaskRunner::Tokio(runner) => runner.block_on(_fut),

            TaskRunner::None => {
                panic!("No task runner feature selected! Please select one (e.g. 'tokio-runner').")
            },
        }
    }

    /// Spawns the given future.
    pub fn spawn<F>(&self, _fut: F) -> Task<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        match self {
            #[cfg(feature = "tokio-runner")]
            TaskRunner::Tokio(runner) => Task::Tokio(runner.spawn(_fut)),

            TaskRunner::None => {
                panic!("No task runner feature selected! Please select one (e.g. 'tokio-runner').")
            },
        }
    }

    /// Spawns the given blocking function.
    pub fn spawn_blocking<F, R>(&self, _fut: F) -> Task<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        match self {
            #[cfg(feature = "tokio-runner")]
            TaskRunner::Tokio(runner) => Task::Tokio(runner.spawn_blocking(_fut)),

            TaskRunner::None => {
                panic!("No task runner feature selected! Please select one (e.g. 'tokio-runner').")
            },
        }
    }
}

/// A task representing a future that can be polled.
///
/// This struct mainly exist to have a universal interface for the different task types of the different task runners.
pub enum Task<T> {
    /// A tokio task. Only available if the 'tokio-runner' feature is enabled.
    #[cfg(feature = "tokio-runner")]
    Tokio(tokio::task::JoinHandle<T>),
    /// Not a real task. Will panic on poll.
    None(PhantomData<T>),
}

impl<T: Unpin> Task<T> {
    /// Returns true if the task is ready (completed).
    pub fn is_ready(&self) -> bool {
        match self {
            #[cfg(feature = "tokio-runner")]
            Task::Tokio(task) => task.is_finished(),

            Task::None(_) => {
                panic!("No task runner feature selected! Please select one (e.g. 'tokio-runner').")
            },
        }
    }
}

impl<T: Unpin> Future for Task<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.get_mut() {
            #[cfg(feature = "tokio-runner")]
            Task::Tokio(task) => {
                let pinned = Pin::new(task);

                pinned
                    .poll(_cx)
                    .map(|res| res.expect("Failed to poll task"))
            },

            Task::None(_) => {
                panic!("No task runner feature selected! Please select one (e.g. 'tokio-runner').")
            },
        }
    }
}
