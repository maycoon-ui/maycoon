use peniko::{Brush, Color};

use crate::id::WidgetId;
use crate::style::{
    DefaultContainerStyles, DefaultInteractiveStyles, DefaultStyles, DefaultTextStyles, Style,
    StyleVal,
};
use crate::theme::Theme;

/// A smooth and minimalistic theme with a cold blue and purple touch.
#[derive(Debug, Copy, Clone, Default)]
pub enum CelesteTheme {
    /// The Light Celeste Theme.
    #[default]
    Light,
}

impl Theme for CelesteTheme {
    fn of(&self, id: WidgetId) -> Option<Style> {
        match id.namespace() {
            "may-widgets" => match id.id() {
                "Text" => Some(Style::from_values([(
                    "color".to_string(),
                    StyleVal::Brush(Brush::Solid(Color::BLACK)),
                )])),
                _ => None,
            },
            _ => None,
        }
    }

    fn defaults(&self) -> DefaultStyles {
        DefaultStyles::new(
            DefaultTextStyles::new(Color::BLACK, Color::WHITE_SMOKE),
            DefaultContainerStyles::new(Color::ANTIQUE_WHITE, Color::WHITE),
            DefaultInteractiveStyles::new(
                Color::DARK_VIOLET,
                Color::BLUE_VIOLET,
                Color::rgb8(138, 146, 226),
                Color::LAVENDER_BLUSH,
            ),
        )
    }

    fn window_background(&self) -> Color {
        Color::WHITE
    }
}
