use maycoon_core::app::context::AppContext;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, LayoutStyle, StyleNode};
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
pub struct Canvas {
    painter: Box<dyn FnMut(&mut Scene, &AppInfo)>,
}

impl Canvas {
    /// Create a new Canvas widget from a painter function.
    pub fn new(painter: impl FnMut(&mut Scene, &AppInfo) + 'static) -> Self {
        Self {
            painter: Box::new(painter),
        }
    }

    /// Set a painter function and return itself.
    pub fn with_painter(mut self, painter: impl FnMut(&mut Scene, &AppInfo) + 'static) -> Self {
        self.painter = Box::new(painter);
        self
    }
}

impl Widget for Canvas {
    fn render(
        &mut self,
        scene: &mut Scene,
        _: &mut dyn Theme,
        _: &LayoutNode,
        info: &AppInfo,
        _: AppContext,
    ) {
        let mut canvas = Scene::new();

        (self.painter)(&mut canvas, info);

        scene.append(&canvas, None);
    }

    fn layout_style(&self) -> StyleNode {
        StyleNode {
            style: LayoutStyle::default(),
            children: Vec::new(),
        }
    }

    fn update(&mut self, _: &LayoutNode, _: AppContext, _: &AppInfo) -> Update {
        Update::DRAW | Update::LAYOUT
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Canvas")
    }
}
