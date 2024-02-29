use femtovg::Color;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ColorScheme {
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,

    pub background_primary: Color,
    pub background_secondary: Color,
    pub background_tertiary: Color,

    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_tertiary: Color,
}
