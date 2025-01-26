use indexmap::IndexMap;
use peniko::{Blob, Font};

/// The font context. Contains font data.
#[derive(Clone, Debug)]
pub struct FontContext {
    default: Font,
    fonts: IndexMap<String, Font>,
}

impl FontContext {
    /// Insert a font with a custom name.
    pub fn insert(&mut self, name: impl ToString, font: Font) {
        self.fonts.insert(name.to_string(), font);
    }

    /// Insert a system font with the given name which is loaded via the postscript name.
    pub fn insert_system_font(
        &mut self,
        name: impl ToString,
        postscript_name: impl ToString,
    ) -> Option<()> {
        let font = font_kit::source::SystemSource::new()
            .select_by_postscript_name(postscript_name.to_string().as_str())
            .ok()?
            .load()
            .ok()?
            .copy_font_data()?;

        self.fonts
            .insert(name.to_string(), Font::new(Blob::new(font), 0))?;

        Some(())
    }

    /// Get a font by a specified name. Returns [None] if the font could not be found.
    pub fn get(&self, name: impl ToString) -> Option<Font> {
        self.fonts.get(&name.to_string()).cloned()
    }

    /// Removes a font. Returns [None] if the font does not exist.
    pub fn remove(&mut self, name: impl ToString) -> Option<()> {
        self.fonts.swap_remove(&name.to_string()).map(|_| ())
    }

    /// Returns the default font. [Roboto](https://fonts.google.com/specimen/Roboto) by default.
    pub fn default_font(&self) -> &Font {
        &self.default
    }
}

impl Default for FontContext {
    fn default() -> Self {
        // TODO: better way to get default font
        let default_font = font_kit::source::SystemSource::new()
            .select_by_postscript_name("ArialMT")
            .expect("Failed to select default font")
            .load()
            .expect("Failed to load default font")
            .copy_font_data()
            .expect("Failed to copy default font");

        Self {
            default: Font::new(Blob::new(default_font), 0),
            fonts: IndexMap::new(),
        }
    }
}
