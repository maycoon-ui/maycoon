use dashmap::DashMap;
use peniko::{Blob, Font};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct FontContext {
    default: Font,
    fonts: DashMap<String, Font>,
}

impl FontContext {
    pub fn insert(&mut self, name: impl ToString, font: Font) {
        self.fonts.insert(name.to_string(), font);
    }

    pub fn get(&self, name: impl ToString) -> Option<Font> {
        self.fonts.get(&name.to_string()).map(|el| el.clone())
    }

    pub fn remove(&mut self, name: impl ToString) {
        self.fonts.remove(&name.to_string());
    }

    pub fn default_font(&self) -> &Font {
        &self.default
    }
}

impl Default for FontContext {
    fn default() -> Self {
        Self {
            default: Font::new(
                Blob::new(Arc::new(include_bytes!(
                    "../../../assets/data/Roboto-Regular.ttf"
                ))),
                0,
            ),
            fonts: DashMap::new(),
        }
    }
}
