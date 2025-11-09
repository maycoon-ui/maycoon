use std::fmt::{Debug, Display, Formatter};

/// An identifier for a widget. This is not for instantiated widgets, but for the widget types in general.
/// It contains a namespace, which should be the crate name and the id of the widget.
///
/// ```
/// # use maycoon_theme::id::WidgetId;
/// WidgetId::new("fancy_text_widget", "FancyText");
/// ```
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct WidgetId {
    namespace: &'static str,
    id: &'static str,
}

impl WidgetId {
    /// Create a new widget id by a namespace and custom id.
    /// The namespace should be the crate name and the id should be the widget type name.
    ///
    /// Example:
    /// ```
    /// # use maycoon_theme::id::WidgetId;
    /// let id = WidgetId::new("my_crate", "MyWidget");
    /// ```
    #[inline(always)]
    pub fn new(namespace: &'static str, id: &'static str) -> Self {
        Self { namespace, id }
    }

    /// Returns the namespace of the widget id.
    #[inline(always)]
    pub fn namespace(&self) -> &'static str {
        self.namespace
    }

    /// Returns the actual widget id.
    #[inline(always)]
    pub fn id(&self) -> &'static str {
        self.id
    }
}

impl Display for WidgetId {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO: use const formatting/writing when const format is stable
        write!(f, "{}:{}", self.namespace, self.id)
    }
}
