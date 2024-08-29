use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{Dimension, LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::state::{State, Val};
use maycoon_core::vg::glyph::Glyph;
use maycoon_core::vg::peniko::{Brush, Fill};
use maycoon_core::vg::skrifa::instance::Size;
use maycoon_core::vg::skrifa::raw::FileRef;
use maycoon_core::vg::skrifa::setting::VariationSetting;
use maycoon_core::vg::skrifa::MetadataProvider;
use maycoon_core::vg::{peniko, Scene};
use maycoon_core::widget::Widget;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use nalgebra::Vector2;

/// Displays the given text with optional font, size and hinting.
pub struct Text<S: State> {
    style: Val<S, LayoutStyle>,
    text: Val<S, String>,
    font: Val<S, Option<String>>,
    font_size: Val<S, f32>,
    hinting: Val<S, bool>,
}

impl<S: State> Text<S> {
    /// Create a new text widget with the given text.
    pub fn new(text: impl Into<Val<S, String>>) -> Self {
        Self {
            style: LayoutStyle::default().into(),
            text: text.into(),
            font: None.into(),
            font_size: 30.0.into(),
            hinting: true.into(),
        }
    }

    /// Set the hinting of the text.
    ///
    /// Hinting adjusts the display of an outline font so that it lines up with a rasterized grid.
    /// At low screen resolutions and font size, hinting can produce clearer text.
    pub fn with_hinting(mut self, hinting: impl Into<Val<S, bool>>) -> Self {
        self.hinting = hinting.into();
        self
    }

    /// Set the font of the text.
    pub fn with_font(mut self, font: impl Into<Val<S, String>>) -> Self {
        self.font = font.into().map(|s| Some(s));
        self
    }

    /// Set the font size of the text.
    pub fn with_font_size(mut self, size: impl Into<Val<S, f32>>) -> Self {
        self.font_size = size.into();
        self
    }

    /// Set the layout style of the text.
    pub fn with_layout(mut self, style: impl Into<Val<S, LayoutStyle>>) -> Self {
        self.style = style.into();
        self
    }
}

impl<S: State> Widget<S> for Text<S> {
    fn render(
        &mut self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        info: &AppInfo,
        layout_node: &LayoutNode,
        state: &S,
    ) {
        let font_size = *self.font_size.get_ref(state);
        let hinting = *self.hinting.get_ref(state);
        let font_name = self.font.get_ref(state);

        let font = if font_name.is_some() {
            info.font_context
                .get(font_name.clone().unwrap())
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

        let metrics = font_ref.metrics(Size::new(font_size), &location);

        let glyph_metrics = font_ref.glyph_metrics(Size::new(font_size), &location);

        let line_height = metrics.ascent + metrics.descent + metrics.leading;

        let charmap = font_ref.charmap();

        let mut pen_x = layout_node.layout.location.x;

        let mut pen_y = layout_node.layout.location.y + font_size;

        let text = self.text.get_ref(state);

        scene
            .draw_glyphs(&font)
            .font_size(font_size)
            .brush(&Brush::Solid(color))
            .normalized_coords(location.coords())
            .hint(hinting)
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

    fn layout_style(&mut self, state: &S) -> StyleNode {
        let text = self.text.get_ref(state);

        let font_size = *self.font_size.get_ref(state);

        let style = self.style.get_ref(state).clone();

        StyleNode {
            style: LayoutStyle {
                size: Vector2::new(
                    Dimension::Length(font_size * text.len() as f32),
                    Dimension::Length(font_size),
                ),
                ..style
            },
            children: Vec::new(),
        }
    }

    fn update(&mut self, _: &LayoutNode, _: &mut S, _: &AppInfo) -> Update {
        self.text.invalidate();
        self.font.invalidate();
        self.hinting.invalidate();
        self.font_size.invalidate();
        self.style.invalidate();
        Update::empty()
    }

    fn widget_id(&mut self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Text")
    }
}
