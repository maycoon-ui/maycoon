use indexmap::IndexMap;
use peniko::{Blob, Font};
use std::sync::Arc;

#[cfg(feature = "include-noto-sans")]
const NOTO_SANS: &[u8] = include_bytes!("../../../assets/Noto Sans Font/NotoSans-VariableFont.ttf");

/// The font context. Contains font data.
#[derive(Clone, Debug)]
pub struct FontContext {
    default: Font,
    fonts: IndexMap<String, Font>,
}

impl FontContext {
    /// Insert a font with a custom name.
    ///
    /// If the font with the same name already exists, it will be overwritten and the old font will be returned.
    pub fn insert(&mut self, name: impl ToString, font: Font) -> Option<Font> {
        self.fonts.insert(name.to_string(), font)
    }

    /// Insert a system font with the given name which is loaded via the postscript name.
    ///
    /// If the font with the same name already exists, it will be overwritten and the old font will be returned.
    ///
    /// Also returns `None` if the font could not be loaded.
    ///
    /// Not recommended to use, since it's system dependent.
    pub fn insert_system_font(
        &mut self,
        name: impl ToString,
        postscript_name: impl ToString,
    ) -> Option<Font> {
        log::trace!("Loading system font: {}", postscript_name.to_string());

        let font = font_kit::source::SystemSource::new()
            .select_by_postscript_name(postscript_name.to_string().as_str())
            .ok()?
            .load()
            .ok()?
            .copy_font_data()?;

        self.fonts
            .insert(name.to_string(), Font::new(Blob::new(font), 0))
    }

    /// Get a font by a specified name. Returns [`None`] if the font could not be found.
    pub fn get(&self, name: impl ToString) -> Option<Font> {
        self.fonts.get(&name.to_string()).cloned()
    }

    /// Removes a font. Returns [`None`] if the font does not exist.
    pub fn remove(&mut self, name: impl ToString) -> Option<()> {
        self.fonts.swap_remove(&name.to_string()).map(|_| ())
    }

    /// Returns the default font. [Roboto](https://fonts.google.com/specimen/Roboto) by default.
    pub fn default_font(&self) -> &Font {
        &self.default
    }
}

#[cfg(feature = "include-noto-sans")]
impl Default for FontContext {
    fn default() -> Self {
        Self {
            default: Font::new(Blob::new(Arc::new(NOTO_SANS)), 0),
            fonts: IndexMap::new(),
        }
    }
}
