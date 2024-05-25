pub struct WidgetId {
    pub namespace: String,
    pub name: String,
}

impl WidgetId {
    pub fn new(namespace: impl ToString, name: impl ToString) -> WidgetId {
        WidgetId {
            namespace: namespace.to_string(),
            name: name.to_string(),
        }
    }
}
