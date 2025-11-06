use crate::app::context::AppContext;
use crate::app::runner::MayRunner;
use crate::config::MayConfig;
use crate::plugin::PluginManager;
use crate::vgi::VectorGraphicsInterface;
use crate::widget::Widget;
use maycoon_theme::theme::Theme;

/// Contains diagnostics data for the application.
pub mod diagnostics;

/// Contains the font context structure.
pub mod font_ctx;

/// Contains the application handler.
pub mod handler;

/// Contains the application information structure.
pub mod info;

/// Contains the update mode bitflag.
pub mod update;

/// Contains the [AppContext] structure for access to the application lifecycle.
pub mod context;

/// Contains the [MayRunner] structure to create and run an application using `winit`.
pub mod runner;

/// The main application interface.
///
/// Contains basic functions for the [MayRunner] to create and run an application.
pub trait Application: Sized {
    /// The theme of the application and its widgets.
    ///
    /// See [maycoon_theme::theme] for built-in themes.
    type Theme: Theme;

    // TODO: Change to default type, once (associated type defaults)[https://github.com/rust-lang/rust/issues/29661] is stabilized.
    /// The vector graphics interface to use for rendering.
    ///
    /// See [VectorGraphicsInterface] for more.
    type Graphics: VectorGraphicsInterface;

    /// The global state of the application.
    type State;

    /// Renders/builds the application's widgets.
    ///
    /// This function will be passed to the [MayRunner] to create and run the application.
    fn build(context: AppContext, state: Self::State) -> impl Widget;

    /// Returns the [MayConfig] for the application.
    fn config(&self) -> MayConfig<Self::Theme, Self::Graphics>;

    /// Builds and returns the [PluginManager] for the application.
    fn plugins(&self) -> PluginManager<Self::Theme, Self::Graphics> {
        PluginManager::new()
    }

    /// Initializes the backend application data.
    ///
    /// This function is called before the actual launch of the app.
    ///
    /// The default implementation just initializes the task runner.
    fn init(&self) {
        #[cfg(feature = "tokio-runner")]
        {
            tracing::info!("initializing tokio task runner");
            crate::tasks::init(crate::tasks::runner::TaskRunner::Tokio(
                crate::tasks::runner::tokio::TaskRunner::new(
                    true, None, None, None, None, None, None, None,
                ),
            ));
        }
    }

    /// Runs the application using the [MayRunner].
    ///
    /// Override this method if you want to use a custom event loop.
    fn run(self, state: Self::State) {
        let config = self.config();

        tracing::info_span!("init").in_scope(|| self.init());

        tracing::info!("launching application runner with config {config:?}");
        MayRunner::<Self::Theme, Self::Graphics>::new(config).run(
            state,
            Self::build,
            self.plugins(),
        );
    }
}
