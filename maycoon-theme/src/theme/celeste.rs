use peniko::{Color, color::palette};

use crate::globals::Globals;
use crate::id::WidgetId;
use crate::style::{
    DefaultContainerStyles, DefaultInteractiveStyles, DefaultStyles, DefaultTextStyles, Style,
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

impl Default for CelesteTheme {
    fn default() -> Self {
        Self::light()
    }
}

impl Theme for CelesteTheme {
    fn of(&self, id: WidgetId) -> Option<Style> {
        match id.namespace() {
            "maycoon-widgets" => match id.id() {
                "Text" => Some(Style::from_values(light::TEXT)),
                "Button" => Some(Style::from_values(light::BUTTON)),
                "Checkbox" => Some(Style::from_values(light::CHECKBOX)),
                "Slider" => Some(Style::from_values(light::SLIDER)),
                _ => None,
            },
            _ => None,
        }
    }

    #[inline(always)]
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

    #[inline(always)]
    fn window_background(&self) -> Color {
        Color::WHITE
    }

    #[inline(always)]
    fn globals(&self) -> &Globals {
        match &self {
            CelesteTheme::Light(globals) => globals,
        }
    }

    #[inline(always)]
    fn globals_mut(&mut self) -> &mut Globals {
        match self {
            CelesteTheme::Light(globals) => globals,
        }
    }
}

mod light {
    use crate::style::StyleVal;
    use peniko::Color;
    use peniko::color::palette;

    pub const TEXT: [(&str, StyleVal); 2] = [
        ("color", StyleVal::Color(palette::css::BLACK)),
        ("color_invert", StyleVal::Color(palette::css::WHITE)),
    ];

    pub const BUTTON: [(&str, StyleVal); 3] = [
        (
            "color_idle",
            StyleVal::Color(Color::from_rgb8(150, 170, 250)),
        ),
        (
            "color_pressed",
            StyleVal::Color(Color::from_rgb8(130, 150, 230)),
        ),
        (
            "color_hovered",
            StyleVal::Color(Color::from_rgb8(140, 160, 240)),
        ),
    ];

    pub const CHECKBOX: [(&str, StyleVal); 2] = [
        (
            "color_checked",
            StyleVal::Color(Color::from_rgb8(130, 130, 230)),
        ),
        (
            "color_unchecked",
            StyleVal::Color(Color::from_rgb8(170, 170, 250)),
        ),
    ];

    pub const SLIDER: [(&str, StyleVal); 2] = [
        ("color", StyleVal::Color(Color::from_rgb8(130, 130, 230))),
        (
            "color_ball",
            StyleVal::Color(Color::from_rgb8(170, 170, 250)),
        ),
    ];
}
