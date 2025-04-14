use crate::ext::WidgetLayoutExt;
use maycoon_core::app::context::AppContext;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{Dimension, LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::signal::MaybeSignal;
use maycoon_core::skrifa::instance::Size;
use maycoon_core::skrifa::raw::FileRef;
use maycoon_core::skrifa::setting::VariationSetting;
use maycoon_core::skrifa::MetadataProvider;
use maycoon_core::vg::peniko::{Brush, Fill};
use maycoon_core::vg::{peniko, Glyph, Scene};
use maycoon_core::widget::Widget;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use nalgebra::Vector2;
use std::ops::Deref;

/// Displays the given text with optional font, size and hinting.
///
/// See the [hello-world](https://github.com/maycoon-ui/maycoon/blob/master/examples/hello-world/src/main.rs) example for how to use it in practice.
///
/// ### Theming
/// You can style the text with the following properties:
/// - `color` - The color of the text.
/// - `color_invert` - The color to use when the `invert_color` property is set to `true` in the theme [`Globals`].
///
/// [`Globals`]: maycoon_theme::globals::Globals
pub struct Text {
    style: MaybeSignal<LayoutStyle>,
    text: MaybeSignal<String>,
    font: MaybeSignal<Option<String>>,
    font_size: MaybeSignal<f32>,
    hinting: MaybeSignal<bool>,
    line_gap: MaybeSignal<f32>,
}

impl Text {
    /// Create a new text widget with the given text.
    pub fn new(text: impl Into<MaybeSignal<String>>) -> Self {
        Self {
            style: LayoutStyle::default().into(),
            text: text.into(),
            font: None.into(),
            font_size: 30.0.into(),
            hinting: true.into(),
            line_gap: 7.5.into(),
        }
    }

    /// Set the hinting of the text.
    ///
    /// Hinting adjusts the display of an outline font so that it lines up with a rasterized grid.
    /// At low screen resolutions and font size, hinting can produce clearer text.
    pub fn with_hinting(mut self, hinting: impl Into<MaybeSignal<bool>>) -> Self {
        self.hinting = hinting.into();
        self
    }

    /// Set the font of the text.
    pub fn with_font(mut self, font: impl Into<MaybeSignal<Option<String>>>) -> Self {
        self.font = font.into();
        self
    }

    /// Set the font size of the text.
    pub fn with_font_size(mut self, size: impl Into<MaybeSignal<f32>>) -> Self {
        self.font_size = size.into();
        self
    }

    /// Set the line gap of the text.
    ///
    /// The line gap is the space between lines of text. Defaults to `7.5`.
    pub fn with_line_gap(mut self, gap: impl Into<MaybeSignal<f32>>) -> Self {
        self.line_gap = gap.into();
        self
    }
}

impl WidgetLayoutExt for Text {
    fn set_layout_style(&mut self, layout_style: impl Into<MaybeSignal<LayoutStyle>>) {
        self.style = layout_style.into();
    }
}

impl Widget for Text {
    fn render(
        &mut self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        layout_node: &LayoutNode,
        info: &AppInfo,
        _: AppContext,
    ) {
        let font_size = *self.font_size.get();
        let hinting = *self.hinting.get();

        let font_name = self.font.get();

        let font = if font_name.is_some() {
            info.font_context
                .get(font_name.deref().clone().unwrap())
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

        let color = if let Some(style) = theme.of(Self::widget_id(self)) {
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

        let line_gap = *self.line_gap.get();

        let charmap = font_ref.charmap();

        let mut pen_x = layout_node.layout.location.x;

        let mut pen_y = layout_node.layout.location.y + font_size;

        let text = self.text.get();

        scene
            .draw_glyphs(&font)
            .font_size(font_size)
            .brush(&Brush::Solid(color))
            .normalized_coords(bytemuck::cast_slice(location.coords()))
            .hint(hinting)
            .draw(
                &peniko::Style::Fill(Fill::NonZero),
                text.chars().filter_map(|c| {
                    if c == '\n' {
                        pen_y += line_height + line_gap;
                        pen_x = layout_node.layout.location.x;
                        return None;
                    }

                    let gid = charmap.map(c).unwrap_or_default();
                    let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                    let x = pen_x;

                    pen_x += advance;

                    Some(Glyph {
                        id: gid.to_u32(),
                        x,
                        y: pen_y,
                    })
                }),
            );
    }

    fn layout_style(&self) -> StyleNode {
        let text = self.text.get();

        let font_size = *self.font_size.get();

        let style = self.style.get().deref().clone();

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

    fn update(&mut self, _: &LayoutNode, _: AppContext, _: &AppInfo) -> Update {
        Update::empty()
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Text")
    }
}
