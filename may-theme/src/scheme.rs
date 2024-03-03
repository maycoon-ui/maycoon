use femtovg::Color;
use indexmap::IndexMap;

#[derive(Clone, Debug, Default)]
pub struct ColorScheme {
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,

    pub background_primary: Color,
    pub background_secondary: Color,
    pub background_tertiary: Color,

    pub foreground_primary: Color,
    pub foreground_secondary: Color,
    pub foreground_tertiary: Color,

    pub custom: IndexMap<String, Color>,
}
