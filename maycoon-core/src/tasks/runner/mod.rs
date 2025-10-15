use crate::config::TasksConfig;

/// Contains the [tokio_runner::TokioRunner] struct.
#[cfg(feature = "tokio-runner")]
pub mod tokio_runner;

/// The task runner enum.
///
/// Contains the [TokioRunner](tokio_runner::TokioRunner) if the 'tokio-runner' feature is enabled.
#[derive(Debug)]
pub enum TaskRunner {
    /// The tokio task runner.
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
        return Self::Tokio(tokio_runner::TokioRunner::new(_config));

        #[cfg(any(not(feature = "tokio-runner")))]
        Self::None
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
    pub async fn spawn<F>(&self, _fut: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        match self {
            #[cfg(feature = "tokio-runner")]
            TaskRunner::Tokio(runner) => runner.spawn(_fut).await,
            TaskRunner::None => {
                panic!("No task runner feature selected! Please select one (e.g. 'tokio-runner').")
            },
        }
    }

    /// Spawns the given blocking function.
    pub async fn spawn_blocking<F, R>(&self, _fut: F) -> R
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        match self {
            #[cfg(feature = "tokio-runner")]
            TaskRunner::Tokio(runner) => runner.spawn_blocking(_fut).await,
            TaskRunner::None => {
                panic!("No task runner feature selected! Please select one (e.g. 'tokio-runner').")
            },
        }
    }
}
