use indexmap::IndexMap;
use pathfinder_color::{ColorF, ColorU};

pub struct Style {
    map: IndexMap<String, StyleVal>,
    defaults: DefaultStyles,
}

impl Style {
    pub fn new(defaults: DefaultStyles, map: IndexMap<String, StyleVal>) -> Self {
        Self { map, defaults }
    }

    pub fn get(&self, key: impl ToString) -> Option<&StyleVal> {
        self.map.get(&key.to_string())
    }

    pub fn defaults(&self) -> &DefaultStyles {
        &self.defaults
    }
}

pub enum StyleVal {
    ColorF(ColorF),
    ColorU(ColorU),
    String(String),
    Int(i32),
    Bool(bool),
    Float(f32),
    List(Vec<StyleVal>),
    Map(IndexMap<String, StyleVal>),
    None,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DefaultStyles {
    text: DefaultTextStyle,
    interactive: DefaultInteractiveStyle,
    container: DefaultContainerStyle,
}

impl DefaultStyles {
    pub fn new(
        text: DefaultTextStyle,
        interactive: DefaultInteractiveStyle,
        container: DefaultContainerStyle,
    ) -> Self {
        Self {
            text,
            interactive,
            container,
        }
    }
    pub fn text(&self) -> &DefaultTextStyle {
        &self.text
    }

    pub fn interactive(&self) -> &DefaultInteractiveStyle {
        &self.interactive
    }

    pub fn container(&self) -> &DefaultContainerStyle {
        &self.container
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DefaultTextStyle {
    primary: ColorU,
    secondary: ColorU,
}

impl DefaultTextStyle {
    pub fn new(primary: ColorU, secondary: ColorU) -> Self {
        Self { primary, secondary }
    }
    pub fn primary(&self) -> ColorU {
        self.primary
    }
    pub fn secondary(&self) -> ColorU {
        self.secondary
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DefaultInteractiveStyle {
    enabled: ColorU,
    disabled: ColorU,
    active: ColorU,
    inactive: ColorU,
}

impl DefaultInteractiveStyle {
    pub fn new(enabled: ColorU, disabled: ColorU, active: ColorU, inactive: ColorU) -> Self {
        Self {
            enabled,
            disabled,
            active,
            inactive,
        }
    }
    pub fn enabled(&self) -> ColorU {
        self.enabled
    }

    pub fn disabled(&self) -> ColorU {
        self.disabled
    }

    pub fn active(&self) -> ColorU {
        self.active
    }

    pub fn inactive(&self) -> ColorU {
        self.inactive
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DefaultContainerStyle {
    background: ColorU,
    foreground: ColorU,
}

impl DefaultContainerStyle {
    pub fn new(background: ColorU, foreground: ColorU) -> Self {
        Self {
            background,
            foreground,
        }
    }
    pub fn background(&self) -> ColorU {
        self.background
    }

    pub fn foreground(&self) -> ColorU {
        self.foreground
    }
}
