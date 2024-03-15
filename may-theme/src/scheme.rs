use femtovg::{Color, Paint};
use indexmap::IndexMap;

/// The style of a widget.
///
/// This may contain colors and other properties that influence the appearance of a widget.
#[derive(Clone, Debug, Default)]
pub struct Scheme {
    properties: IndexMap<String, SchemeValue>,
}

impl Scheme {
    pub fn new(properties: IndexMap<String, SchemeValue>) -> Self {
        Self { properties }
    }

    pub fn empty() -> Self {
        Self { properties: IndexMap::new() }
    }

    pub fn get(&self, key: &str) -> Option<&SchemeValue> {
        self.properties.get(&key.to_string())
    }
}

/// A custom value to define styles.
#[derive(Clone, Debug, Default)]
pub enum SchemeValue {
    /// A simple color value
    Color(Color),

    /// A string value
    String(String),

    /// A paint value
    Paint(Paint),

    /// A float value. Used for e.g. opacity.
    Float(f32),

    /// An integer value.
    Int(i32),

    /// A boolean value.
    Bool(bool),

    /// A list of values.
    List(Vec<SchemeValue>),

    #[default]
    None,
}

impl SchemeValue {
    pub fn as_color(&self) -> Option<Color> {
        match self {
            SchemeValue::Color(color) => Some(*color),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            SchemeValue::String(string) => Some(string.clone()),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            SchemeValue::Float(float) => Some(*float),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        match self {
            SchemeValue::Int(int) => Some(*int),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SchemeValue::Bool(bool) => Some(*bool),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<SchemeValue>> {
        match self {
            SchemeValue::List(list) => Some(list),
            _ => None,
        }
    }

    pub fn as_paint(&self) -> Option<&Paint> {
        match self {
            SchemeValue::Paint(paint) => Some(paint),
            _ => None,
        }
    }

    pub fn as_none(&self) -> Option<()> {
        match self {
            SchemeValue::None => Some(()),
            _ => None,
        }
    }
}
