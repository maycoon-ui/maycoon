use std::any::TypeId;
use std::fmt::Debug;

use crate::scheme::ColorScheme;

pub trait Theme: Debug {
    fn default_scheme(&self) -> ColorScheme;
    fn window_scheme(&self) -> ColorScheme;
    fn scheme_of(&self, id: TypeId) -> ColorScheme;
}

#[derive(Debug)]
pub struct DummyTheme;

impl Theme for DummyTheme {
    fn default_scheme(&self) -> ColorScheme {
        ColorScheme::default()
    }

    fn window_scheme(&self) -> ColorScheme {
        ColorScheme::default()
    }

    fn scheme_of(&self, id: TypeId) -> ColorScheme {
        self.default_scheme()
    }
}
