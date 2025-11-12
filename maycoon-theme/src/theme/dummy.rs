use crate::globals::Globals;
use crate::id::WidgetId;
use crate::style::{
    DefaultContainerStyles, DefaultInteractiveStyles, DefaultStyles, DefaultTextStyles, Style,
};
use crate::theme::Theme;
use peniko::Color;

/// A dummy theme that only returns [None] or [Color::WHITE].
///
/// Useful for testing.
#[derive(Debug, Copy, Clone, Default)]
pub struct DummyTheme(Globals);

impl Theme for DummyTheme {
    fn of(&self, _id: WidgetId) -> Option<Style> {
        None
    }

    fn defaults(&self) -> DefaultStyles {
        DefaultStyles::new(
            DefaultTextStyles::new(Color::WHITE, Color::WHITE),
            DefaultContainerStyles::new(Color::WHITE, Color::WHITE),
            DefaultInteractiveStyles::new(Color::WHITE, Color::WHITE, Color::WHITE, Color::WHITE),
        )
    }

    fn window_background(&self) -> Color {
        Color::WHITE
    }

    fn globals(&self) -> &Globals {
        &self.0
    }

    fn globals_mut(&mut self) -> &mut Globals {
        &mut self.0
    }
}
