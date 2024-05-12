use femtovg::Color;

use crate::id::WidgetId;
use crate::scheme::{GenericScheme, Scheme, WidgetScheme};
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

    fn scheme_of(&self, id: WidgetId, widget_type: WidgetType) -> WidgetScheme {
        match self {
            MayTheme::Light => light::LightTheme::scheme_of(&light::LightTheme, id, widget_type),
        }
    }
}
