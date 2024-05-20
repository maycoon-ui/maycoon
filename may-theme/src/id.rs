use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct WidgetId(String);

impl WidgetId {
    pub const EMPTY: WidgetId = WidgetId(String::new());

    pub fn as_str(&self) -> &str {
        &self.0.as_str()
    }

    pub fn new(crate_name: impl Display, widget_name: impl Display) -> WidgetId {
        WidgetId(format!("{}:{}", crate_name, widget_name))
    }

    pub fn raw(id: impl ToString) -> Self {
        Self(id.to_string())
    }

    pub fn crate_name(&self) -> &str {
        self.0.split_once(':').unwrap().0
    }

    pub fn widget_name(&self) -> &str {
        self.0.split_once(':').unwrap().1
    }
}

impl From<(&str, &str)> for WidgetId {
    fn from(value: (&str, &str)) -> Self {
        Self::new(value.0, value.1)
    }
}
