use crate::app::context::AppContext;
use crate::app::runner::MayRunner;
use crate::config::MayConfig;
use crate::plugin::PluginManager;
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

    /// Renders/builds the application's widgets.
    ///
    /// This function will be passed to the [MayRunner] to create and run the application.
    fn build(context: AppContext) -> impl Widget;

    /// Returns the [MayConfig] for the application.
    fn config(&self) -> MayConfig<Self::Theme>;

    /// Builds and returns the [PluginManager] for the application.
    fn plugins(&self) -> PluginManager<Self::Theme> {
        PluginManager::new()
    }

    /// Runs the application using the [MayRunner].
    ///
    /// Override this method if you want to use a custom event loop.
    fn run(self) {
        MayRunner::<Self::Theme>::new(self.config()).run(Self::build, self.plugins());
    }
}
