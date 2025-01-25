use peniko::{color::palette, Color};

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
    /// Use [`CelesteTheme::light`] to use the light Celeste theme.
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
            "maycoon-widgets" => match id.id() {
                "Text" => Some(Style::from_values([
                    ("color".to_string(), StyleVal::Color(palette::css::BLACK)),
                    (
                        "color_invert".to_string(),
                        StyleVal::Color(palette::css::WHITE),
                    ),
                ])),

                "Button" => Some(Style::from_values([
                    (
                        "color_idle".to_string(),
                        StyleVal::Color(Color::from_rgb8(150, 170, 250)),
                    ),
                    (
                        "color_pressed".to_string(),
                        StyleVal::Color(Color::from_rgb8(130, 150, 230)),
                    ),
                    (
                        "color_hovered".to_string(),
                        StyleVal::Color(Color::from_rgb8(140, 160, 240)),
                    ),
                ])),

                "Checkbox" => Some(Style::from_values([
                    (
                        "color_checked".to_string(),
                        StyleVal::Color(Color::from_rgb8(130, 130, 230)),
                    ),
                    (
                        "color_unchecked".to_string(),
                        StyleVal::Color(Color::from_rgb8(170, 170, 250)),
                    ),
                ])),

                "Slider" => Some(Style::from_values([
                    (
                        "color".to_string(),
                        StyleVal::Color(Color::from_rgb8(130, 130, 230)),
                    ),
                    (
                        "color_ball".to_string(),
                        StyleVal::Color(Color::from_rgb8(170, 170, 250)),
                    ),
                ])),

                _ => None,
            },
            _ => None,
        }
    }

    fn defaults(&self) -> DefaultStyles {
        DefaultStyles::new(
            DefaultTextStyles::new(palette::css::BLACK, palette::css::WHITE_SMOKE),
            DefaultContainerStyles::new(palette::css::ANTIQUE_WHITE, palette::css::WHITE),
            DefaultInteractiveStyles::new(
                Color::from_rgb8(130, 150, 230),
                Color::from_rgb8(150, 170, 250),
                Color::from_rgb8(140, 160, 240),
                Color::from_rgb8(110, 110, 110),
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
