/// Global theme values for all widgets to use.
///
/// This may be used to invert the text color of widgets that are inside a container.
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Globals {
    /// Invert text color.
    pub invert_text_color: bool,
}
