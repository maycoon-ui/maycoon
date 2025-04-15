use maycoon_core::app::context::AppContext;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::signal::MaybeSignal;
use maycoon_core::vg::kurbo::{Affine, Vec2};
use maycoon_core::vg::Scene;
use maycoon_core::widget::{Widget, WidgetLayoutExt};
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use vello_svg::vello;

/// Owned shareable image data.
pub type ImageData = vello::peniko::Image;

/// An image widget. Pretty self-explanatory.
///
/// See the [image](https://github.com/maycoon-ui/maycoon/blob/master/examples/image/src/main.rs) example for how to use it in practice.
///
/// ### Theming
/// The widget itself only draws the underlying image, so theming is useless.
pub struct Image {
    image: MaybeSignal<ImageData>,
    style: MaybeSignal<LayoutStyle>,
}

impl Image {
    /// Create an image widget from the given [ImageData].
    pub fn new(image: impl Into<MaybeSignal<ImageData>>) -> Self {
        Self {
            image: image.into(),
            style: LayoutStyle::default().into(),
        }
    }

    /// Set the image.
    pub fn with_image(mut self, image: impl Into<MaybeSignal<ImageData>>) -> Self {
        self.image = image.into();
        self
    }
}

impl WidgetLayoutExt for Image {
    fn set_layout_style(&mut self, layout_style: impl Into<MaybeSignal<LayoutStyle>>) {
        self.style = layout_style.into();
    }
}

impl Widget for Image {
    fn render(
        &mut self,
        scene: &mut Scene,
        _: &mut dyn Theme,
        layout_node: &LayoutNode,
        _: &AppInfo,
        _: AppContext,
    ) {
        let image = self.image.get();

        scene.draw_image(
            &image,
            Affine::translate(Vec2::new(
                layout_node.layout.location.x as f64,
                layout_node.layout.location.y as f64,
            )),
        );
    }

    fn layout_style(&self) -> StyleNode {
        StyleNode {
            style: self.style.get().clone(),
            children: Vec::new(),
        }
    }

    fn update(&mut self, _: &LayoutNode, _: AppContext, _: &AppInfo) -> Update {
        Update::empty()
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Image")
    }
}
