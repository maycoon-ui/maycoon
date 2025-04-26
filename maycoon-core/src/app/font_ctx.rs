use indexmap::IndexMap;
use peniko::{Blob, Font};
use std::sync::Arc;

/// A font manager for maycoon applications.
///
/// Can be used to load and access in-memory fonts or by system source.
///
/// If the default `include-noto-sans` feature is enabled, the default font is set to [Noto Sans](https://fonts.google.com/specimen/Noto+Sans).
#[derive(Clone, Debug)]
pub struct FontContext {
    default: String,
    fonts: IndexMap<String, Font>,
}

impl FontContext {
    /// Create a new font context with the given default font name.
    ///
    /// Make sure to load the default font via [FontContext::load],
    /// before passing this context to the application runner.
    pub fn new(default: String) -> Self {
        Self {
            default,
            fonts: IndexMap::new(),
        }
    }

    /// Loads a font with a custom name into the font context.
    ///
    /// If the font with the same name already exists, it will be overwritten and the old font will be returned.
    pub fn load(&mut self, name: impl ToString, font: Font) -> Option<Font> {
        self.fonts.insert(name.to_string(), font)
    }

    /// Loads a system font into the font context.
    /// The provided name must match the postscript name of the font.
    ///
    /// If a font with the same name is already loaded, it will be overwritten and the old font will be returned.
    ///
    /// Returns `None` if the font could not be loaded.
    ///
    /// **NOTE:** Not every postscript font is available on every system.
    pub fn load_system(
        &mut self,
        name: impl ToString,
        postscript_name: impl ToString,
    ) -> Option<()> {
        log::debug!("Loading system font: {}", postscript_name.to_string());

        let font = font_kit::source::SystemSource::new()
            .select_by_postscript_name(postscript_name.to_string().as_str())
            .ok()?
            .load()
            .ok()?
            .copy_font_data()?;

        self.load(name, Font::new(Blob::new(font), 0));

        Some(())
    }

    /// Set the default font.
    ///
    /// **NOTE:** The font must be loaded before usage with [FontContext::load].
    pub fn set_default_font(&mut self, name: impl ToString) {
        self.default = name.to_string();
    }

    /// Get a font by a specified name. Returns [None] if the font could not be found.
    pub fn get(&self, name: impl ToString) -> Option<Font> {
        self.fonts.get(&name.to_string()).cloned()
    }

    /// Removes a font by the given name and returns it or [None] if the font could not be found.
    pub fn remove(&mut self, name: impl ToString) -> Option<Font> {
        self.fonts.swap_remove(&name.to_string())
    }

    /// Returns the default font. [Roboto](https://fonts.google.com/specimen/Roboto) by default.
    pub fn default_font(&self) -> &Font {
        self.fonts
            .get(&self.default)
            .expect("Default font not found. Please load one via `FontContext::load`.")
    }
}

#[cfg(feature = "include-noto-sans")]
impl Default for FontContext {
    fn default() -> Self {
        let mut ctx = FontContext::new("Noto Sans".to_string());

        ctx.load(
            "Noto Sans",
            Font::new(Blob::new(Arc::new(crate::DEFAULT_FONT)), 0),
        );

        ctx
    }
}
