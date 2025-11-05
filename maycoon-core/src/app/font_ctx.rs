use indexmap::IndexMap;
use peniko::{Blob, FontData};
use std::sync::Arc;

/// A font manager for maycoon applications.
///
/// Can be used to load and access in-memory fonts or by system source.
///
/// If the default `include-noto-sans` feature is enabled, the default font is set to [Noto Sans](https://fonts.google.com/specimen/Noto+Sans).
#[derive(Clone, Debug)]
pub struct FontContext {
    default: String,
    fonts: IndexMap<String, FontData>,
}

impl FontContext {
    /// Create a new font context with the given default font name.
    ///
    /// Make sure to load the default font via [FontContext::load],
    /// before passing this context to the application runner.
    pub fn new(default: impl ToString) -> Self {
        Self {
            default: default.to_string(),
            fonts: IndexMap::new(),
        }
    }

    /// Loads a font with a custom name into the font context.
    ///
    /// If the font with the same name already exists, it will be overwritten and the old font will be returned.
    #[tracing::instrument(level = "trace", skip_all, fields(name = name.to_string()))]
    pub fn load(&mut self, name: impl ToString, font: FontData) -> Option<FontData> {
        tracing::trace!("loading font named {}", name.to_string());
        self.fonts.insert(name.to_string(), font)
    }

    /// Set the default font.
    ///
    /// **NOTE:** The font must be loaded before usage with [FontContext::load].
    pub fn set_default_font(&mut self, name: impl ToString) {
        self.default = name.to_string();
    }

    /// Get a font by a specified name. Returns [None] if the font could not be found.
    pub fn get(&self, name: impl ToString) -> Option<FontData> {
        self.fonts.get(&name.to_string()).cloned()
    }

    /// Removes a font by the given name and returns it or [None] if the font could not be found.
    pub fn remove(&mut self, name: impl ToString) -> Option<FontData> {
        self.fonts.swap_remove(&name.to_string())
    }

    /// Returns the default font. [Roboto](https://fonts.google.com/specimen/Roboto) by default.
    pub fn default_font(&self) -> &FontData {
        self.fonts
            .get(&self.default)
            .expect("Default font not found. Please load one via `FontContext::load`.")
    }
}

#[cfg(feature = "include-noto-sans")]
impl Default for FontContext {
    fn default() -> Self {
        let mut ctx = FontContext::new("Noto Sans".to_string());

        tracing::debug!("loading noto sans system font");
        ctx.load(
            "Noto Sans",
            FontData::new(Blob::new(Arc::new(crate::DEFAULT_FONT)), 0),
        );

        ctx
    }
}
