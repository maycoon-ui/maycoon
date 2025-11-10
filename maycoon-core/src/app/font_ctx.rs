use peniko::FontData;
use rpds::HashTrieMap;

/// A font manager for maycoon applications.
///
/// Can be used to load and access in-memory fonts or by system source.
///
/// If the default `include-noto-sans` feature is enabled, the default font is set to [Noto Sans](https://fonts.google.com/specimen/Noto+Sans).
#[derive(Clone, Debug)]
pub struct FontContext {
    default: String,
    fonts: HashTrieMap<String, FontData>,
}

impl FontContext {
    /// Create a new font context with the given default font name.
    ///
    /// Make sure to load the default font via [FontContext::load],
    /// before passing this context to the application runner.
    #[inline(always)]
    pub fn new(default: impl ToString) -> Self {
        Self {
            default: default.to_string(),
            fonts: HashTrieMap::new(),
        }
    }

    /// Loads a font with a custom name into the font context and return itself.
    ///
    /// If the font with the same name already exists, [None] is returned.
    #[inline(always)]
    #[tracing::instrument(level = "trace", skip_all, fields(name = name.to_string()))]
    pub fn load(self, name: impl ToString, font: FontData) -> Option<Self> {
        let name = name.to_string();

        tracing::trace!("loading font named {}", name);

        if self.fonts.contains_key(&name) {
            return None;
        }

        Some(Self {
            fonts: self.fonts.insert(name, font),
            default: self.default,
        })
    }

    /// Set the default font.
    ///
    /// **NOTE:** The font must be loaded before usage with [FontContext::load].
    #[inline(always)]
    pub fn set_default_font(&mut self, name: impl ToString) {
        self.default = name.to_string();
    }

    /// Get a font by a specified name. Returns [None] if the font could not be found.
    #[inline(always)]
    pub fn get(&self, name: impl ToString) -> Option<FontData> {
        self.fonts.get(&name.to_string()).cloned()
    }

    /// Removes a font by the given name and returns it or [None] if the font could not be found.
    #[inline(always)]
    pub fn remove(self, name: impl AsRef<str>) -> Option<Self> {
        if self.fonts.contains_key(name.as_ref()) {
            return None;
        }

        Some(Self {
            fonts: self.fonts.remove(name.as_ref()),
            default: self.default,
        })
    }

    /// Returns the default font. [Roboto](https://fonts.google.com/specimen/Roboto) by default.
    #[inline(always)]
    pub fn default_font(&self) -> &FontData {
        self.fonts
            .get(&self.default)
            .expect("Default font not found. Please load one via `FontContext::load`.")
    }
}

impl Default for FontContext {
    #[inline(always)]
    fn default() -> Self {
        tracing::debug!("loading noto sans system font");

        FontContext::new("Noto Sans".to_string())
            .load(
                "Noto Sans",
                FontData::new(
                    peniko::Blob::new(std::sync::Arc::new(crate::DEFAULT_FONT)),
                    0,
                ),
            )
            .unwrap()
    }
}
