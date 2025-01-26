use indexmap::IndexMap;
use peniko::{Brush, Color, Gradient};

/// Styling map for defining widget appearance.
#[derive(Clone, Debug)]
pub struct Style {
    map: IndexMap<String, StyleVal>,
}

impl Style {
    /// Create a new empty style.
    pub fn new() -> Self {
        Self {
            map: IndexMap::with_capacity(16),
        }
    }

    /// Create a style from an array of strings and style values.
    pub fn from_values(values: impl IntoIterator<Item = (String, StyleVal)>) -> Self {
        Self {
            map: IndexMap::from_iter(values),
        }
    }

    /// Removes the style value from the map with the give name.
    pub fn remove(&mut self, name: impl ToString) {
        self.map.swap_remove(&name.to_string());
    }

    /// Insert a style value with the given name into the style map.
    pub fn with_value(mut self, name: impl ToString, value: StyleVal) -> Self {
        self.map.insert(name.to_string(), value);
        self
    }

    /// Set a style value by name.
    pub fn set(&mut self, name: impl ToString, value: StyleVal) {
        self.map.insert(name.to_string(), value);
    }

    /// Set a color style value by name.
    pub fn set_color(&mut self, name: impl ToString, color: Color) {
        self.map.insert(name.to_string(), StyleVal::Color(color));
    }

    /// Set a gradient style value by name.
    pub fn set_gradient(&mut self, name: impl ToString, gradient: Gradient) {
        self.map
            .insert(name.to_string(), StyleVal::Gradient(gradient));
    }

    /// Set a bool style value by name.
    pub fn set_bool(&mut self, name: impl ToString, value: bool) {
        self.map.insert(name.to_string(), StyleVal::Bool(value));
    }

    /// Set a brush style value by name.
    pub fn set_brush(&mut self, name: impl ToString, brush: Brush) {
        self.map.insert(name.to_string(), StyleVal::Brush(brush));
    }

    /// Set a float style value by name.
    pub fn set_float(&mut self, name: impl ToString, value: f32) {
        self.map.insert(name.to_string(), StyleVal::Float(value));
    }

    /// Set an int style value by name.
    pub fn set_int(&mut self, name: impl ToString, value: i32) {
        self.map.insert(name.to_string(), StyleVal::Int(value));
    }

    /// Set an unsized int style value by name.
    pub fn set_uint(&mut self, name: impl ToString, value: u32) {
        self.map.insert(name.to_string(), StyleVal::UInt(value));
    }

    /// Get a style value by name. Returns [None] if the value name does not exist.
    pub fn get(&self, name: impl ToString) -> Option<StyleVal> {
        self.map.get(&name.to_string()).cloned()
    }

    /// Get a color style value by name. Returns [None] if the value name does not exist.
    pub fn get_color(&self, name: impl ToString) -> Option<Color> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val {
                StyleVal::Color(color) => Some(*color),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get a gradient style value by name. Returns [None] if the value name does not exist.
    pub fn get_gradient(&self, name: impl ToString) -> Option<Gradient> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val {
                StyleVal::Gradient(gradient) => Some(gradient.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get a brush style value by name. Returns [None] if the value name does not exist.
    pub fn get_brush(&self, name: impl ToString) -> Option<Brush> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val {
                StyleVal::Brush(brush) => Some(brush.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get a float style value by name. Returns [None] if the value name does not exist.
    pub fn get_float(&self, name: impl ToString) -> Option<f32> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val {
                StyleVal::Float(float) => Some(*float),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get an int style value by name. Returns [None] if the value name does not exist.
    pub fn get_int(&self, name: impl ToString) -> Option<i32> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val {
                StyleVal::Int(int) => Some(*int),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get an unsized int style value by name. Returns [None] if the value name does not exist.
    pub fn get_uint(&self, name: impl ToString) -> Option<u32> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val {
                StyleVal::UInt(uint) => Some(*uint),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get a bool style value by name. Returns [None] if the value name does not exist.
    pub fn get_bool(&self, name: impl ToString) -> Option<bool> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val {
                StyleVal::Bool(bool) => Some(*bool),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

/// Default widget styles.
#[derive(Clone, PartialEq, Debug)]
pub struct DefaultStyles {
    text: DefaultTextStyles,
    container: DefaultContainerStyles,
    interactive: DefaultInteractiveStyles,
}

impl DefaultStyles {
    /// Create new default styles with given styles.
    pub fn new(
        text: DefaultTextStyles,
        container: DefaultContainerStyles,
        interactive: DefaultInteractiveStyles,
    ) -> Self {
        Self {
            text,
            container,
            interactive,
        }
    }

    /// Get the default styles for text widgets.
    pub fn text(&self) -> &DefaultTextStyles {
        &self.text
    }

    /// Get the default styles for container widgets.
    pub fn container(&self) -> &DefaultContainerStyles {
        &self.container
    }

    /// Get the default styles for interactive widgets.
    pub fn interactive(&self) -> &DefaultInteractiveStyles {
        &self.interactive
    }
}

/// The default text widget styles.
#[derive(Clone, PartialEq, Debug)]
pub struct DefaultTextStyles {
    foreground: Color,
    background: Color,
}

impl DefaultTextStyles {
    /// Create new default text styles with given colors.
    pub fn new(foreground: Color, background: Color) -> Self {
        Self {
            foreground,
            background,
        }
    }

    /// Get the default foreground color.
    pub fn foreground(&self) -> Color {
        self.foreground
    }

    /// Get the default background color.
    pub fn background(&self) -> Color {
        self.background
    }
}

/// The default container widget styles.
#[derive(Clone, PartialEq, Debug)]
pub struct DefaultContainerStyles {
    foreground: Color,
    background: Color,
}

impl DefaultContainerStyles {
    /// Create new default container styles with given colors.
    pub fn new(foreground: Color, background: Color) -> Self {
        Self {
            foreground,
            background,
        }
    }

    /// Get the default foreground color.
    pub fn foreground(&self) -> Color {
        self.foreground
    }

    /// Get the default background color.
    pub fn background(&self) -> Color {
        self.background
    }
}

/// The default interactive widget styles.
#[derive(Clone, PartialEq, Debug)]
pub struct DefaultInteractiveStyles {
    active: Color,
    inactive: Color,
    hover: Color,
    disabled: Color,
}

impl DefaultInteractiveStyles {
    /// Create new default interactive styles with given colors.
    pub fn new(active: Color, inactive: Color, hover: Color, disabled: Color) -> Self {
        Self {
            active,
            inactive,
            hover,
            disabled,
        }
    }

    /// Get the default active widget color.
    pub fn active(&self) -> Color {
        self.active
    }

    /// Get the default inactive widget color.
    pub fn inactive(&self) -> Color {
        self.inactive
    }

    /// Get the default on-hover widget color.
    pub fn hover(&self) -> Color {
        self.hover
    }

    /// Get the default disabled widget color.
    pub fn disabled(&self) -> Color {
        self.disabled
    }
}

/// A style value.
#[derive(Clone, Debug)]
pub enum StyleVal {
    /// A color style value.
    Color(Color),
    /// A gradient style value.
    Gradient(Gradient),
    /// A brush style value.
    Brush(Brush),
    /// A float style value.
    Float(f32),
    /// An int style value.
    Int(i32),
    /// An unsized int style value.
    UInt(u32),
    /// A bool style value.
    Bool(bool),
}
