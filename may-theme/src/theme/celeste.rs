use peniko::Color;

use crate::id::WidgetId;
use crate::style::{
    DefaultContainerStyles, DefaultInteractiveStyles, DefaultStyles, DefaultTextStyles, Style,
};
use crate::theme::Theme;

#[derive(Debug, Copy, Clone, Default)]
pub enum CelesteTheme {
    #[default]
    Light,
}

impl Theme for CelesteTheme {
    fn of(&self, id: WidgetId) -> Option<Style> {
        match id.namespace() {
            "may-widgets" => match id.id() {
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
        Color::WHITE_SMOKE
    }
}
