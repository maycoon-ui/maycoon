use crate::app::context::AppContext;
use crate::app::font_ctx::FontContext;
use crate::app::handler::AppHandler;
use crate::app::update::UpdateManager;
use crate::config::MayConfig;
use crate::plugin::PluginManager;
use crate::widget::Widget;
use maycoon_theme::theme::Theme;
use peniko::Font;
use winit::dpi::{LogicalPosition, LogicalSize, Position, Size};
use winit::event_loop::EventLoopBuilder;
use winit::window::WindowAttributes;

/// The core Application structure.
pub struct MayRunner<T: Theme> {
    config: MayConfig<T>,
    font_ctx: FontContext,
}

impl<T: Theme> MayRunner<T> {
    /// Create a new App with the given [MayConfig].
    pub fn new(config: MayConfig<T>) -> Self {
        // init task runner
        if let Some(config) = &config.tasks {
            log::info!("Initializing task runner.");

            crate::tasks::runner::TaskRunner::new(config.stack_size, config.workers)
                .expect("Failed to create task runner")
                .init()
                .expect("Failed to init task runner");
        }

        Self {
            config,
            font_ctx: FontContext::default(),
        }
    }

    /// Loads a new font into the font context.
    ///
    /// See [FontContext::load] for more.
    pub fn with_font(mut self, name: impl ToString, font: Font) -> Self {
        self.font_ctx.load(name, font);
        self
    }

    /// Loads a new system font into the font context.
    ///
    /// See [FontContext::load_system] for more.
    pub fn with_system_font(mut self, name: impl ToString, postscript_name: impl ToString) -> Self {
        self.font_ctx.load_system(name, postscript_name);
        self
    }

    /// Set the font context. Can be used to configure fonts.
    pub fn with_font_context(mut self, font_ctx: FontContext) -> Self {
        self.font_ctx = font_ctx;
        self
    }

    /// Run the application with given widget and state.
    pub fn run<W, F>(mut self, builder: F, mut plugins: PluginManager<T>)
    where
        W: Widget,
        F: FnOnce(AppContext) -> W,
    {
        let mut event_loop = EventLoopBuilder::default()
            .build()
            .expect("Failed to create event loop");

        let mut attrs = WindowAttributes::default()
            .with_inner_size(LogicalSize::new(
                self.config.window.size.x,
                self.config.window.size.y,
            ))
            .with_resizable(self.config.window.resizable)
            .with_enabled_buttons(self.config.window.buttons)
            .with_title(self.config.window.title.clone())
            .with_maximized(self.config.window.maximized)
            .with_visible(self.config.window.visible)
            .with_transparent(self.config.window.transparent)
            .with_blur(self.config.window.blur)
            .with_decorations(self.config.window.decorations)
            .with_window_icon(self.config.window.icon.clone())
            .with_content_protected(self.config.window.content_protected)
            .with_window_level(self.config.window.level)
            .with_active(self.config.window.active)
            .with_cursor(self.config.window.cursor.clone());

        // since `with_max_inner_size()` doesn't support `Option` values, we need to manually set it
        attrs.max_inner_size = self
            .config
            .window
            .max_size
            .map(|v| Size::Logical(LogicalSize::new(v.x, v.y)));

        // since `with_min_inner_size()` doesn't support `Option` values, we need to manually set it
        attrs.min_inner_size = self
            .config
            .window
            .min_size
            .map(|v| Size::Logical(LogicalSize::new(v.x, v.y)));

        // since `with_position()` doesn't support `Option` values, we need to manually set it
        attrs.position = self
            .config
            .window
            .position
            .map(|v| Position::Logical(LogicalPosition::new(v.x, v.y)));

        // since `with_resize_increments()` doesn't support `Option` values, we need to manually set it
        attrs.resize_increments = self
            .config
            .window
            .resize_increments
            .map(|v| Size::Logical(LogicalSize::new(v.x, v.y)));

        log::info!("Launching Application...");

        let update = UpdateManager::new();

        let widget = builder(AppContext::new(update.clone()));

        plugins.run(|pl| pl.init(&mut event_loop, &update, &mut attrs, &mut self.config));

        event_loop
            .run_app(&mut AppHandler::new(
                attrs,
                self.config,
                widget,
                self.font_ctx,
                update,
                plugins,
            ))
            .expect("Failed to run event loop");
    }
}
