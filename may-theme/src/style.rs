use dashmap::DashMap;
use peniko::{Brush, Color, Gradient};

pub struct Style {
    map: DashMap<String, StyleVal>,
}

impl Style {
    pub fn new() -> Self {
        Self {
            map: DashMap::with_capacity(16),
        }
    }

    pub fn set(&mut self, name: impl ToString, value: StyleVal) {
        self.map.insert(name.to_string(), value);
    }

    pub fn set_color(&mut self, name: impl ToString, color: Color) {
        self.map.insert(name.to_string(), StyleVal::Color(color));
    }

    pub fn set_gradient(&mut self, name: impl ToString, gradient: Gradient) {
        self.map
            .insert(name.to_string(), StyleVal::Gradient(gradient));
    }

    pub fn set_brush(&mut self, name: impl ToString, brush: Brush) {
        self.map.insert(name.to_string(), StyleVal::Brush(brush));
    }

    pub fn set_float(&mut self, name: impl ToString, value: f32) {
        self.map.insert(name.to_string(), StyleVal::Float(value));
    }

    pub fn set_int(&mut self, name: impl ToString, value: i32) {
        self.map.insert(name.to_string(), StyleVal::Int(value));
    }

    pub fn set_uint(&mut self, name: impl ToString, value: u32) {
        self.map.insert(name.to_string(), StyleVal::UInt(value));
    }

    pub fn get(&self, name: impl ToString) -> Option<StyleVal> {
        self.map.get(&name.to_string()).map(|val| val.clone())
    }

    pub fn get_color(&self, name: impl ToString) -> Option<Color> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val.value() {
                StyleVal::Color(color) => Some(color.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_gradient(&self, name: impl ToString) -> Option<Gradient> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val.value() {
                StyleVal::Gradient(gradient) => Some(gradient.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_brush(&self, name: impl ToString) -> Option<Brush> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val.value() {
                StyleVal::Brush(brush) => Some(brush.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_float(&self, name: impl ToString) -> Option<f32> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val.value() {
                StyleVal::Float(float) => Some(*float),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_int(&self, name: impl ToString) -> Option<i32> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val.value() {
                StyleVal::Int(int) => Some(*int),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_uint(&self, name: impl ToString) -> Option<u32> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val.value() {
                StyleVal::UInt(uint) => Some(*uint),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_bool(&self, name: impl ToString) -> Option<bool> {
        if let Some(val) = self.map.get(&name.to_string()) {
            match val.value() {
                StyleVal::Bool(bool) => Some(*bool),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct DefaultStyles {
    text: DefaultTextStyles,
    container: DefaultContainerStyles,
    interactive: DefaultInteractiveStyles,
}

impl DefaultStyles {
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

    pub fn text(&self) -> &DefaultTextStyles {
        &self.text
    }

    pub fn container(&self) -> &DefaultContainerStyles {
        &self.container
    }

    pub fn interactive(&self) -> &DefaultInteractiveStyles {
        &self.interactive
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct DefaultTextStyles {
    foreground: Color,
    background: Color,
}

impl DefaultTextStyles {
    pub fn new(foreground: Color, background: Color) -> Self {
        Self {
            foreground,
            background,
        }
    }

    pub fn foreground(&self) -> Color {
        self.foreground
    }

    pub fn background(&self) -> Color {
        self.background
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct DefaultContainerStyles {
    foreground: Color,
    background: Color,
}

impl DefaultContainerStyles {
    pub fn new(foreground: Color, background: Color) -> Self {
        Self {
            foreground,
            background,
        }
    }

    pub fn foreground(&self) -> Color {
        self.foreground
    }

    pub fn background(&self) -> Color {
        self.background
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct DefaultInteractiveStyles {
    active: Color,
    inactive: Color,
    hover: Color,
    disabled: Color,
}

impl DefaultInteractiveStyles {
    pub fn new(active: Color, inactive: Color, hover: Color, disabled: Color) -> Self {
        Self {
            active,
            inactive,
            hover,
            disabled,
        }
    }

    pub fn active(&self) -> Color {
        self.active
    }

    pub fn inactive(&self) -> Color {
        self.inactive
    }

    pub fn hover(&self) -> Color {
        self.hover
    }

    pub fn disabled(&self) -> Color {
        self.disabled
    }
}

#[derive(Clone, Debug)]
pub enum StyleVal {
    Color(Color),
    Gradient(Gradient),
    Brush(Brush),
    Float(f32),
    Int(i32),
    UInt(u32),
    Bool(bool),
}
