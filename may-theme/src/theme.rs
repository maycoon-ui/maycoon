use crate::id::WidgetId;
use crate::style::{Style, StyleVal};
use pathfinder_color::ColorU;

pub trait Theme {
    fn of(&mut self, id: WidgetId) -> Style;
    fn get(&self, key: &str) -> Option<&StyleVal>;
    fn set(&mut self, key: &str, value: StyleVal);
    fn window_background(&self) -> ColorU;
}
