use std::fmt::{Debug, Display, Formatter};

/// An identifier for a widget. This is not for instantiated widgets, but for the widget types in general.
/// It contains a namespace, which should be the crate name and the id of the widget.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct WidgetId {
    namespace: String,
    id: String,
}

impl WidgetId {
    /// Create a new widget id by a namespace and custom id.
    /// The namespace should be the crate name and the id should be the widget type name.
    ///
    /// Example:
    /// ```
    /// let id = may_theme::id::WidgetId::new("my_crate", "MyWidget");
    /// ```
    pub fn new(namespace: impl ToString, id: impl ToString) -> Self {
        Self {
            namespace: namespace.to_string(),
            id: id.to_string(),
        }
    }

    /// Returns the namespace of the widget id.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the actual widget id.
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Display for WidgetId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.id)
    }
}
