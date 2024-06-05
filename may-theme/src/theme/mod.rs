use peniko::Color;

use crate::id::WidgetId;
use crate::style::{DefaultStyles, Style};

pub mod celeste;

pub trait Theme {
    fn of(&self, id: WidgetId) -> Option<Style>;
    fn defaults(&self) -> DefaultStyles;
    fn window_background(&self) -> Color;
}
