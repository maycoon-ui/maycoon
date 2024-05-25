use crate::id::WidgetId;
use crate::style::{DefaultInteractiveStyle, DefaultStyles, DefaultTextStyle, Style, StyleVal};
use crate::theme::Theme;
use indexmap::IndexMap;
use pathfinder_color::ColorU;

pub struct LightTheme {
    globals: IndexMap<String, StyleVal>,
}

impl LightTheme {
    pub fn new() -> Self {
        Self {
            globals: IndexMap::new(),
        }
    }
}

impl Theme for LightTheme {
    fn of(&mut self, id: WidgetId) -> Style {
        Style::new(
            DefaultStyles::new(
                DefaultTextStyle::new(ColorU::black(), ColorU::white()),
                DefaultInteractiveStyle::default(),
                Default::default(),
            ),
            IndexMap::from([]),
        )
    }

    fn get(&self, key: &str) -> Option<&StyleVal> {
        self.globals.get(&key.to_string())
    }

    fn set(&mut self, key: &str, value: StyleVal) {
        self.globals.insert(key.to_string(), value);
    }

    fn window_background(&self) -> ColorU {
        ColorU::white()
    }
}
