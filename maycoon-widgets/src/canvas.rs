use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::state::State;
use maycoon_core::vg::Scene;
use maycoon_core::widget::Widget;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;

/// A canvas widget to directly draw to the screen.
///
/// This is a very simplified version of "your own Widget" and you should only use it for simple cases.
///
/// ### Theming
/// The canvas cannot be themed, since it does not draw something on itself.
pub struct Canvas<S: State> {
    painter: Box<dyn FnMut(&mut Scene, &S)>,
}

impl<S: State> Canvas<S> {
    /// Create a new Canvas widget from a painter function.
    pub fn new(painter: impl FnMut(&mut Scene, &S) + 'static) -> Self {
        Self {
            painter: Box::new(painter),
        }
    }

    /// Set a painter function and return itself.
    pub fn with_painter(painter: impl FnMut(&mut Scene, &S) + 'static) -> Self {
        Self {
            painter: Box::new(painter),
        }
    }
}

impl<S: State> Widget<S> for Canvas<S> {
    fn render(
        &mut self,
        scene: &mut Scene,
        _: &mut dyn Theme,
        _: &AppInfo,
        _: &LayoutNode,
        state: &S,
    ) {
        let mut canvas = Scene::new();

        (self.painter)(&mut canvas, state);

        scene.append(&canvas, None);
    }

    fn layout_style(&mut self, _: &S) -> StyleNode {
        StyleNode {
            style: LayoutStyle::default(),
            children: Vec::new(),
        }
    }

    fn update(&mut self, _: &LayoutNode, _: &mut S, _: &AppInfo) -> Update {
        Update::DRAW | Update::LAYOUT
    }

    fn widget_id(&mut self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Canvas")
    }
}
