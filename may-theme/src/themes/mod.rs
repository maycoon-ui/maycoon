use femtovg::Color;

use crate::id::WidgetId;
use crate::scheme::Scheme;
use crate::theme::{Theme, WidgetType};

pub mod light;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum MayTheme {
    #[default]
    Light,
}

impl Theme for MayTheme {
    fn window_background(&self) -> Color {
        match self {
            MayTheme::Light => light::LightTheme::window_background(&light::LightTheme),
        }
    }

    fn scheme_of(&self, id: WidgetId) -> Option<Scheme> {
        match self {
            MayTheme::Light => light::LightTheme::scheme_of(&light::LightTheme, id),
        }
    }

    fn default_scheme_of(&self, widget_type: WidgetType) -> Scheme {
        match self {
            MayTheme::Light => {
                light::LightTheme::default_scheme_of(&light::LightTheme, widget_type)
            }
        }
    }
}
