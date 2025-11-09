use maycoon_core::app::context::AppContext;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::signal::MaybeSignal;
use maycoon_core::vgi::{ImageBrush, ImageData, Scene};
use maycoon_core::widget::{Widget, WidgetLayoutExt};
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use nalgebra::Vector2;

/// An image widget. Pretty self-explanatory.
///
/// See the [image](https://github.com/maycoon-ui/maycoon/blob/master/examples/image/src/main.rs) example for how to use it in practice.
///
/// ### Theming
/// The widget itself only draws the underlying image, so theming is useless.
///
/// The [WidgetId] is equal to `maycoon-widgets:Image`.
pub struct Image {
    image: MaybeSignal<ImageData>,
    style: MaybeSignal<LayoutStyle>,
}

impl Image {
    /// Create an image widget from the given [ImageData].
    #[inline(always)]
    pub fn new(image: impl Into<MaybeSignal<ImageData>>) -> Self {
        Self {
            image: image.into(),
            style: LayoutStyle::default().into(),
        }
    }

    /// Set the image.
    #[inline(always)]
    pub fn with_image(mut self, image: impl Into<MaybeSignal<ImageData>>) -> Self {
        self.image = image.into();
        self
    }
}

impl WidgetLayoutExt for Image {
    #[inline(always)]
    fn set_layout_style(&mut self, layout_style: impl Into<MaybeSignal<LayoutStyle>>) {
        self.style = layout_style.into();
    }
}

impl Widget for Image {
    #[inline(always)]
    fn render(
        &mut self,
        scene: &mut dyn Scene,
        _: &mut dyn Theme,
        layout_node: &LayoutNode,
        _: &AppInfo,
        _: AppContext,
    ) {
        let image = self.image.get();
        let brush = ImageBrush::new(image.clone());

        scene.draw_image(
            &brush,
            None,
            Vector2::new(layout_node.layout.location.x, layout_node.layout.location.y),
        );
    }

    #[inline(always)]
    fn layout_style(&self) -> StyleNode {
        StyleNode {
            style: self.style.get().clone(),
            children: Vec::new(),
        }
    }

    #[inline(always)]
    fn update(&mut self, _: &LayoutNode, _: AppContext, _: &AppInfo) -> Update {
        Update::empty()
    }

    #[inline(always)]
    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Image")
    }
}
