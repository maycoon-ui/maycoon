use may_core::app::info::AppInfo;
use may_core::app::update::Update;
use may_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use may_core::state::State;
use may_core::vg::glyph::Glyph;
use may_core::vg::peniko::{Brush, Fill};
use may_core::vg::skrifa::instance::Size;
use may_core::vg::skrifa::raw::FileRef;
use may_core::vg::skrifa::setting::VariationSetting;
use may_core::vg::skrifa::MetadataProvider;
use may_core::vg::{peniko, Scene};
use may_core::widget::Widget;
use may_theme::id::WidgetId;
use may_theme::theme::Theme;

/// A text widget.
///
/// It's text, what do you expect?
pub struct Text {
    style: LayoutStyle,
    text: String,
    font: Option<String>,
    font_size: f32,
}

impl Text {
    /// Create a new text widget with the given text.
    pub fn new(text: impl ToString) -> Self {
        Self {
            style: LayoutStyle::default(),
            text: text.to_string(),
            font: None,
            font_size: 16.0,
        }
    }

    /// Set the font of the text.
    pub fn with_font(mut self, font: impl ToString) -> Self {
        self.font = Some(font.to_string());
        self
    }

    /// Set the font size of the text.
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set the layout style of the text.
    pub fn with_layout(mut self, style: LayoutStyle) -> Self {
        self.style = style;
        self
    }
}

impl<S: State> Widget<S> for Text {
    fn render(
        &self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        info: &AppInfo,
        layout_node: &LayoutNode,
    ) {
        let font = if self.font.is_some() {
            info.font_context
                .get(self.font.clone().unwrap())
                .expect("Font not found")
        } else {
            info.font_context.default_font().clone()
        };

        let font_ref = {
            let file_ref = FileRef::new(font.data.as_ref()).expect("Failed to load font data");
            match file_ref {
                FileRef::Font(font) => Some(font),
                FileRef::Collection(collection) => collection.get(font.index).ok(),
            }
        }
        .expect("Failed to load font ref");

        let brush = if let Some(style) = theme.of(<Text as Widget<S>>::widget_id(self)) {
            style.get_brush("color").expect("Failed to get brush")
        } else {
            Brush::Solid(theme.defaults().text().foreground())
        };

        let location = font_ref.axes().location::<&[VariationSetting; 0]>(&[]);

        let metrics = font_ref.metrics(Size::new(self.font_size), &location);

        let glyph_metrics = font_ref.glyph_metrics(Size::new(self.font_size), &location);

        let line_height = metrics.ascent + metrics.descent + metrics.leading;

        let charmap = font_ref.charmap();

        let mut pen_x = layout_node.layout.location.x;

        let mut pen_y = layout_node.layout.location.y + self.font_size;

        scene
            .draw_glyphs(&font)
            .font_size(self.font_size)
            .brush(&brush)
            .glyph_transform(None)
            .normalized_coords(location.coords())
            .hint(false)
            .draw(
                &peniko::Style::Fill(Fill::EvenOdd),
                self.text.chars().filter_map(|ch| {
                    if ch == '\n' {
                        pen_y += line_height;
                        pen_x = 0.0;
                        return None;
                    }
                    let gid = charmap.map(ch).unwrap_or_default();
                    let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                    let x = pen_x;
                    pen_x += advance;
                    Some(Glyph {
                        id: gid.to_u16() as u32,
                        x,
                        y: pen_y,
                    })
                }),
            );
    }

    fn layout_style(&self) -> StyleNode {
        StyleNode {
            style: self.style.clone(),
            children: Vec::new(),
        }
    }

    fn update(&mut self, _: &LayoutNode, _: &mut S, _: &AppInfo) -> Update {
        Update::empty()
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("may-widgets", "Text")
    }
}
