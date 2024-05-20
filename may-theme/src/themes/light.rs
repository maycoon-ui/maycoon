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
        match id.as_str() {
            "may-widgets:Text" => WidgetScheme::Custom(Scheme::new(IndexMap::from([(
                "color".to_string(),
                SchemeValue::Paint(Paint::color(Color::rgb(64, 64, 64))),
            )]))),

            "may-widgets:Button" => WidgetScheme::Custom(Scheme::new(IndexMap::from([
                (
                    "color".to_string(),
                    SchemeValue::Paint(Paint::color(Color::rgb(40, 144, 255))),
                ),
                (
                    "color-hover".to_string(),
                    SchemeValue::Paint(Paint::color(Color::rgb(40, 170, 225))),
                ),
                (
                    "color-click".to_string(),
                    SchemeValue::Paint(Paint::color(Color::rgb(45, 250, 255))),
                ),
            ]))),

            _ => match widget_type {
                WidgetType::Other => {
                    todo!()
                }

                WidgetType::Interactive => {
                    todo!()
                }

                WidgetType::Content => {
                    todo!()
                }

                WidgetType::Disabled => {
                    todo!()
                }

                WidgetType::Container => {
                    todo!()
                }

                WidgetType::Collapsed => {
                    todo!()
                }
            },
        }
    }
}
