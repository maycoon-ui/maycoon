use peniko::Color;

use crate::globals::Globals;
use crate::id::WidgetId;
use crate::style::{
    DefaultContainerStyles, DefaultInteractiveStyles, DefaultStyles, DefaultTextStyles, Style,
    StyleVal,
};
use crate::theme::Theme;

/// A smooth and minimalistic theme with a cold blue and purple touch.
#[derive(Debug, Clone)]
pub enum CelesteTheme {
    /// Use [CelesteTheme::light] to use the light Celeste theme.
    Light(Globals),
}

impl CelesteTheme {
    /// The Light Celeste Theme.
    pub fn light() -> Self {
        Self::Light(Globals::default())
    }
}

impl Theme for CelesteTheme {
    fn of(&self, id: WidgetId) -> Option<Style> {
        match id.namespace() {
            "may-widgets" => match id.id() {
                "Text" => Some(Style::from_values([
                    ("color".to_string(), StyleVal::Color(Color::BLACK)),
                    ("color_invert".to_string(), StyleVal::Color(Color::WHITE)),
                ])),
                // TODO: add button style
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
                Color::rgb8(90, 90, 200),
                Color::rgb8(120, 120, 230),
                Color::rgb8(110, 110, 210),
                Color::rgb8(60, 60, 60),
            ),
        )
    }

    fn window_background(&self) -> Color {
        Color::WHITE
    }

    fn globals(&self) -> &Globals {
        match &self {
            CelesteTheme::Light(globals) => globals,
        }
    }

    fn globals_mut(&mut self) -> &mut Globals {
        match self {
            CelesteTheme::Light(globals) => globals,
        }
    }
}
