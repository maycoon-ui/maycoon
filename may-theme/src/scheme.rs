use femtovg::Color;
use indexmap::IndexMap;

/// The style of a widget.
///
/// This may contain colors and other properties that influence the appearance of a widget.
#[derive(Clone, Debug, Default)]
pub struct Scheme {
    /// The primary color of a widget
    pub primary_color: Color,

    /// The secondary color of a widget
    pub secondary_color: Color,

    /// The background color of a widget
    pub primary_background_color: Color,

    /// The secondary background color of a widget
    pub secondary_background_color: Color,

    /// The foreground color of a widget
    pub primary_foreground_color: Color,

    /// The secondary foreground color of a widget
    pub secondary_foreground_color: Color,

    /// The border color of a widget
    pub primary_border_color: Color,

    /// The secondary border color of a widget
    pub secondary_border_color: Color,

    /// Custom properties that don't fit in the primary/secondary categories.
    pub custom: IndexMap<String, SchemeValue>,
}

/// A custom value to define styles.
#[derive(Clone, Debug, Default)]
pub enum SchemeValue {
    /// A simple color value
    Color(Color),

    /// A string value
    String(String),

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

    pub fn as_none(&self) -> Option<()> {
        match self {
            SchemeValue::None => Some(()),
            _ => None,
        }
    }
}
