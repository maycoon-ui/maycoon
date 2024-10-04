use peniko::Font;
use winit::dpi::{LogicalPosition, LogicalSize, Position, Size};
use winit::event_loop::EventLoopBuilder;
use winit::window::WindowAttributes;

use maycoon_theme::theme::Theme;

use crate::app::font_ctx::FontContext;
use crate::app::handler::AppHandler;
use crate::config::MayConfig;
use crate::state::State;
use crate::widget::Widget;

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

/// The core Application structure.
pub struct MayApp<T: Theme> {
    config: MayConfig<T>,
    font_ctx: FontContext,
}

impl<T: Theme> MayApp<T> {
    /// Create a new App with the given [MayConfig].
    pub fn new(config: MayConfig<T>) -> Self {
        Self {
            config,
            font_ctx: FontContext::default(),
        }
    }

    /// Insert a new font into the font context.
    pub fn with_font(mut self, name: impl ToString, font: Font) -> Self {
        self.font_ctx.insert(name, font);
        self
    }

    /// Run the application with given widget and state.
    pub fn run<S, W>(self, state: S, widget: W)
    where
        S: State,
        W: Widget<S>,
    {
        let event_loop = EventLoopBuilder::default()
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

        event_loop
            .run_app(&mut AppHandler::new(
                attrs,
                self.config,
                widget,
                state,
                self.font_ctx,
            ))
            .expect("Failed to run event loop");
    }
}
