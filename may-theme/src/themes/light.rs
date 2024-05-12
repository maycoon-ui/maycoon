use femtovg::{Color, Paint};
use indexmap::IndexMap;

use crate::colors;
use crate::id::WidgetId;
use crate::scheme::{GenericScheme, Scheme, SchemeValue, WidgetScheme};
use crate::theme::{Theme, WidgetType};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LightTheme;

impl Theme for LightTheme {
    fn window_background(&self) -> Color {
        Color::rgb(253, 251, 255)
    }

    fn scheme_of(&self, id: WidgetId, widget_type: WidgetType) -> WidgetScheme {
        WidgetScheme::Generic(GenericScheme::default())
    }
}
