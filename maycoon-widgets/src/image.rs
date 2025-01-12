use crate::ext::WidgetLayoutExt;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::state::{State, Val};
use maycoon_core::vg::kurbo::{Affine, Vec2};
use maycoon_core::vg::peniko::{Blob, Format};
use maycoon_core::vg::{peniko, Scene};
use maycoon_core::widget::Widget;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use nalgebra::Vector2;

/// An image widget. Pretty self-explanatory.
///
/// See the [image](https://github.com/maycoon-ui/maycoon/blob/master/examples/image/src/main.rs) example for how to use it in practice.
///
/// ### Theming
/// The widget itself only draws the underlying image, so theming is useless.
pub struct Image<S: State> {
    image: Val<S, peniko::Image>,
    style: Val<S, LayoutStyle>,
}

impl<S: State> Image<S> {
    /// Create an image widget from the given [ImageData].
    pub fn new(image: impl Into<Val<S, ImageData>>) -> Self {
        Self {
            image: image.into().map(|data| {
                peniko::Image::new(
                    Blob::from(data.image),
                    data.format,
                    data.size.x,
                    data.size.y,
                )
            }),
            style: LayoutStyle::default().into(),
        }
    }

    /// Set the image.
    pub fn with_image(mut self, image: impl Into<Val<S, ImageData>>) -> Self {
        self.image = image.into().map(|data| {
            peniko::Image::new(
                Blob::from(data.image),
                data.format,
                data.size.x,
                data.size.y,
            )
        });
        self
    }
}

impl<S: State> WidgetLayoutExt<S> for Image<S> {
    fn set_layout_style(&mut self, layout_style: impl Into<Val<S, LayoutStyle>>) {
        self.style = layout_style.into();
    }
}

impl<S: State> Widget<S> for Image<S> {
    fn render(
        &mut self,
        scene: &mut Scene,
        _: &mut dyn Theme,
        _: &AppInfo,
        layout_node: &LayoutNode,
        state: &S,
    ) {
        let image = self.image.get_ref(state);

        scene.draw_image(
            image,
            Affine::translate(Vec2::new(
                layout_node.layout.location.x as f64,
                layout_node.layout.location.y as f64,
            )),
        );
    }

    fn layout_style(&mut self, state: &S) -> StyleNode {
        StyleNode {
            style: self.style.get_ref(state).clone(),
            children: Vec::new(),
        }
    }

    fn update(&mut self, _: &LayoutNode, _: &mut S, _: &AppInfo) -> Update {
        self.image.invalidate();
        self.style.invalidate();
        Update::empty()
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "image")
    }
}

/// Contains data about the image itself, size and the image format.
///
/// If you have trouble with getting the image in the right format, use the `image` crate for converting it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageData {
    image: Vec<u8>,
    size: Vector2<u32>,
    format: Format,
}

impl ImageData {
    /// Creates a new [ImageData] from the image itself, its size and the image format.
    pub fn new(image: Vec<u8>, size: Vector2<u32>, format: Format) -> Self {
        Self {
            image,
            size,
            format,
        }
    }
}
