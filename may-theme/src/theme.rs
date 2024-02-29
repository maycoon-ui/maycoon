use crate::scheme::ColorScheme;
use std::any::TypeId;
use std::fmt::Debug;

pub trait Theme: Debug {
    fn default_scheme(&self) -> ColorScheme;
    fn scheme_of(&self, id: TypeId) -> ColorScheme;
}

#[derive(Debug)]
pub struct DummyTheme;

impl Theme for DummyTheme {
    fn default_scheme(&self) -> ColorScheme {
        ColorScheme {
            primary: Default::default(),
            secondary: Default::default(),
            tertiary: Default::default(),
            background_primary: Default::default(),
            background_secondary: Default::default(),
            background_tertiary: Default::default(),
            text_primary: Default::default(),
            text_secondary: Default::default(),
            text_tertiary: Default::default(),
        }
    }

    fn scheme_of(&self, id: TypeId) -> ColorScheme {
        self.default_scheme()
    }
}
