use femtovg::{Color, Paint};
use indexmap::IndexMap;

use crate::colors;
use crate::id::WidgetId;
use crate::scheme::{Scheme, SchemeValue};
use crate::theme::{Theme, WidgetType};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LightTheme;

impl Theme for LightTheme {
    fn window_background(&self) -> Color {
        Color::rgb(253, 251, 255)
    }

    fn scheme_of(&self, id: WidgetId) -> Option<Scheme> {
        match id.as_str() {
            "may-widgets:Text" => Some(Scheme::new(IndexMap::from([(
                "color".to_string(),
                SchemeValue::Paint(Paint::color(colors::BLACK)),
            )]))),
            "may-widgets:Button" => Some(Scheme::new(IndexMap::from([
                (
                    "background".to_string(),
                    SchemeValue::Paint(Paint::color(Color::rgb(200, 200, 250))),
                ),
                (
                    "background_hover".to_string(),
                    SchemeValue::Paint(Paint::color(Color::rgb(220, 220, 250))),
                ),
                (
                    "background_pressed".to_string(),
                    SchemeValue::Paint(Paint::color(Color::rgb(190, 190, 250))),
                ),
            ]))),
            _ => None,
        }
    }

    fn default_scheme_of(&self, widget_type: WidgetType) -> Scheme {
        Scheme::default()
    }
}
