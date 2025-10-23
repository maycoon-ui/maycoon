use maycoon_core::vgi::svg::{Error, Options, Tree};

/// An SVG icon rendered.
pub struct SvgIcon(Tree);

impl SvgIcon {
    /// Creates a new icon from the given SVG source.
    ///
    /// Returns [Err] if the SVG could not be parsed.
    pub fn new(source: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(Self(Tree::from_data(source.as_ref(), &Options::default())?))
    }

    /// Creates a new icon from the given SVG source as string.
    ///
    /// Returns [Err] if the SVG could not be parsed.
    pub fn new_str(source: impl AsRef<str>) -> Result<Self, Error> {
        Ok(Self(Tree::from_str(source.as_ref(), &Options::default())?))
    }

    /// Returns the underlying [Tree].
    pub fn tree(&self) -> &Tree {
        &self.0
    }
}
