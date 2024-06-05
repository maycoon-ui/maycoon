use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct WidgetId {
    namespace: String,
    id: String,
}

impl WidgetId {
    pub fn new(namespace: impl ToString, id: impl ToString) -> Self {
        Self {
            namespace: namespace.to_string(),
            id: id.to_string(),
        }
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Display for WidgetId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}.{}", self.namespace, self.id))
    }
}
