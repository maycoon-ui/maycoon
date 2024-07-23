use may_core::app::info::AppInfo;
use may_core::app::update::Update;
use may_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use may_core::state::{State, StateVal};
use may_core::vg::kurbo::{Affine, Vec2};
use may_core::vg::peniko::{Blob, Format};
use may_core::vg::{peniko, Scene};
use may_core::widget::Widget;
use may_theme::id::WidgetId;
use may_theme::theme::Theme;
use nalgebra::Vector2;

/// An image widget. Pretty self-explanatory.
pub struct Image<S: State> {
    image: StateVal<S, peniko::Image>,
    style: LayoutStyle,
}

impl<S: State> Image<S> {
    /// Create a new image widget from image data (in the specified format), the format itself and the image size.
    /// If you have trouble with getting the image in the right format, use the `image` crate for converting to the right format.
    pub fn new(image: StateVal<S, (Vec<u8>, Format, Vector2<u32>)>) -> Self {
        Self {
            image: image.map(|(data, format, size)| {
                peniko::Image::new(Blob::from(data), format, size.x, size.y)
            }),
            style: LayoutStyle::default(),
        }
    }

    /// Set the layout style.
    pub fn with_style(mut self, style: LayoutStyle) -> Self {
        self.style = style;
        self
    }
}

impl<S: State> Widget<S> for Image<S> {
    fn render(
        &self,
        scene: &mut Scene,
        _: &mut dyn Theme,
        _: &AppInfo,
        layout_node: &LayoutNode,
        state: &S,
    ) {
        let image = self.image.get(state);

        scene.draw_image(
            &image,
            Affine::translate(Vec2::new(
                layout_node.layout.location.x as f64,
                layout_node.layout.location.y as f64,
            )),
        );
    }

    fn layout_style(&self, _: &S) -> StyleNode {
        StyleNode {
            style: self.style.clone(),
            children: Vec::new(),
        }
    }

    fn update(&mut self, _: &LayoutNode, _: &mut S, _: &AppInfo) -> Update {
        Update::empty()
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("may-widgets", "image")
    }
}
