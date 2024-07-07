use may_core::app::info::AppInfo;
use may_core::app::update::Update;
use may_core::layout;
use may_core::layout::{Dimension, LayoutNode, LayoutStyle, StyleNode};
use may_core::state::{State, StateVal};
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
pub struct Text<S: State> {
    style: LayoutStyle,
    text: StateVal<S, String>,
    font: Option<String>,
    font_size: f32,
    hinting: bool,
}

impl<S: State> Text<S> {
    /// Create a new text widget with the given text.
    pub fn new(text: StateVal<S, impl ToString + 'static>) -> Self {
        Self {
            style: LayoutStyle::default(),
            text: text.map(|s| s.to_string()),
            font: None,
            font_size: 30.0,
            hinting: true,
        }
    }

    /// Set the hinting of the text.
    ///
    /// Hinting adjusts the display of an outline font so that it lines up with a rasterized grid.
    /// At low screen resolutions and font size, hinting can produce clearer text.
    pub fn with_hinting(mut self, hinting: bool) -> Self {
        self.hinting = hinting;
        self
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

impl<S: State> Widget<S> for Text<S> {
    fn render(
        &self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        info: &AppInfo,
        layout_node: &LayoutNode,
        state: &S,
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
        .expect("Failed to load font reference");

        let color = if let Some(style) = theme.of(<Text<S> as Widget<S>>::widget_id(self)) {
            if theme.globals().invert_text_color {
                style.get_color("color_invert").unwrap()
            } else {
                style.get_color("color").unwrap()
            }
        } else {
            theme.defaults().text().foreground()
        };

        let location = font_ref.axes().location::<&[VariationSetting; 0]>(&[]);

        let metrics = font_ref.metrics(Size::new(self.font_size), &location);

        let glyph_metrics = font_ref.glyph_metrics(Size::new(self.font_size), &location);

        let line_height = metrics.ascent + metrics.descent + metrics.leading;

        let charmap = font_ref.charmap();

        let mut pen_x = layout_node.layout.location.x;

        let mut pen_y = layout_node.layout.location.y + self.font_size;

        let text = self.text.get(state);

        scene
            .draw_glyphs(&font)
            .font_size(self.font_size)
            .brush(&Brush::Solid(color))
            .normalized_coords(location.coords())
            .hint(self.hinting)
            .draw(
                &peniko::Style::Fill(Fill::NonZero),
                text.chars().filter_map(|c| {
                    if c == '\n' {
                        pen_y += line_height;
                        pen_x = layout_node.layout.location.x;
                        return None;
                    }
                    let gid = charmap.map(c).unwrap_or_default();
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

    fn layout_style(&self, state: &S) -> StyleNode {
        let text = self.text.get(state);

        StyleNode {
            style: LayoutStyle {
                size: layout::Size::<Dimension> {
                    width: Dimension::Length(self.font_size * text.len() as f32),
                    height: Dimension::Length(self.font_size),
                },
                ..self.style
            },
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
