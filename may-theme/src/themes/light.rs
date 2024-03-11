use femtovg::Color;
use indexmap::IndexMap;

use crate::scheme::{Scheme, SchemeValue};
use crate::theme::{Theme, WidgetType};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LightTheme;

impl Theme for LightTheme {
    fn window_background(&self) -> Color {
        Color::rgb(255, 254, 254)
    }

    fn scheme_of(&self, id: String) -> Option<Scheme> {
        match id.as_str() {
            "may-widgets:Text" => Some(Scheme {
                primary_color: Color::black(),
                ..Default::default()
            }),
            "may-widgets:Button" => Some(Scheme {
                primary_background_color: Color::rgb(205, 190, 255),
                custom: {
                    let mut map = IndexMap::new();
                    map.insert(String::from("radius"), SchemeValue::Float(10.0));
                    map
                },
                ..Default::default()
            }),
            _ => None,
        }
    }

    fn default_scheme_of(&self, widget_type: WidgetType) -> Scheme {
        Scheme::default()
    }
}
