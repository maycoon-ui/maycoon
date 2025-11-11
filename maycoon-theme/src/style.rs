use peniko::{Brush, Color, Gradient};
use rpds::HashTrieMap;

/// Styling map for defining widget appearance.
#[derive(Clone, Debug)]
pub struct Style {
    map: HashTrieMap<&'static str, StyleVal>,
}

impl Style {
    /// Create a new empty style.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            map: HashTrieMap::new(),
        }
    }

    /// Create a style from an array of strings and style values.
    #[inline(always)]
    pub fn from_values(values: impl IntoIterator<Item = (&'static str, StyleVal)>) -> Self {
        Self {
            map: HashTrieMap::from_iter(values),
        }
    }

    /// Insert a style value with the given name into the style map and return the new style.
    #[inline(always)]
    pub fn with_value(self, name: &'static str, value: StyleVal) -> Self {
        Self {
            map: self.map.insert(name, value),
        }
    }

    /// Get a style value by name. Returns [None] if the value name does not exist.
    #[inline(always)]
    pub fn get(&self, name: &'static str) -> Option<StyleVal> {
        self.map.get(name).cloned()
    }

    /// Get a color style value by name. Returns [None] if the value name does not exist.
    #[inline(always)]
    pub fn get_color(&self, name: &'static str) -> Option<Color> {
        if let Some(val) = self.map.get(name) {
            match val {
                StyleVal::Color(color) => Some(*color),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get a gradient style value by name. Returns [None] if the value name does not exist.
    #[inline(always)]
    pub fn get_gradient(&self, name: &'static str) -> Option<Gradient> {
        if let Some(val) = self.map.get(name) {
            match val {
                StyleVal::Gradient(gradient) => Some(gradient.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get a brush style value by name. Returns [None] if the value name does not exist.
    #[inline(always)]
    pub fn get_brush(&self, name: &'static str) -> Option<Brush> {
        if let Some(val) = self.map.get(name) {
            match val {
                StyleVal::Brush(brush) => Some(brush.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get a float style value by name. Returns [None] if the value name does not exist.
    #[inline(always)]
    pub fn get_float(&self, name: &'static str) -> Option<f32> {
        if let Some(val) = self.map.get(name) {
            match val {
                StyleVal::Float(float) => Some(*float),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get an int style value by name. Returns [None] if the value name does not exist.
    #[inline(always)]
    pub fn get_int(&self, name: &'static str) -> Option<i32> {
        if let Some(val) = self.map.get(name) {
            match val {
                StyleVal::Int(int) => Some(*int),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get an unsized int style value by name. Returns [None] if the value name does not exist.
    #[inline(always)]
    pub fn get_uint(&self, name: &'static str) -> Option<u32> {
        if let Some(val) = self.map.get(name) {
            match val {
                StyleVal::UInt(uint) => Some(*uint),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get a bool style value by name. Returns [None] if the value name does not exist.
    #[inline(always)]
    pub fn get_bool(&self, name: &'static str) -> Option<bool> {
        if let Some(val) = self.map.get(name) {
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
    #[inline(always)]
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
    #[inline(always)]
    pub const fn new(
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
    #[inline(always)]
    pub const fn text(&self) -> &DefaultTextStyles {
        &self.text
    }

    /// Get the default styles for container widgets.
    #[inline(always)]
    pub const fn container(&self) -> &DefaultContainerStyles {
        &self.container
    }

    /// Get the default styles for interactive widgets.
    #[inline(always)]
    pub const fn interactive(&self) -> &DefaultInteractiveStyles {
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
    #[inline(always)]
    pub const fn new(foreground: Color, background: Color) -> Self {
        Self {
            foreground,
            background,
        }
    }

    /// Get the default foreground color.
    #[inline(always)]
    pub const fn foreground(&self) -> Color {
        self.foreground
    }

    /// Get the default background color.
    #[inline(always)]
    pub const fn background(&self) -> Color {
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
    #[inline(always)]
    pub const fn new(foreground: Color, background: Color) -> Self {
        Self {
            foreground,
            background,
        }
    }

    /// Get the default foreground color.
    #[inline(always)]
    pub const fn foreground(&self) -> Color {
        self.foreground
    }

    /// Get the default background color.
    #[inline(always)]
    pub const fn background(&self) -> Color {
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
    #[inline(always)]
    pub const fn new(active: Color, inactive: Color, hover: Color, disabled: Color) -> Self {
        Self {
            active,
            inactive,
            hover,
            disabled,
        }
    }

    /// Get the default active widget color.
    #[inline(always)]
    pub const fn active(&self) -> Color {
        self.active
    }

    /// Get the default inactive widget color.
    #[inline(always)]
    pub const fn inactive(&self) -> Color {
        self.inactive
    }

    /// Get the default on-hover widget color.
    #[inline(always)]
    pub const fn hover(&self) -> Color {
        self.hover
    }

    /// Get the default disabled widget color.
    #[inline(always)]
    pub const fn disabled(&self) -> Color {
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
