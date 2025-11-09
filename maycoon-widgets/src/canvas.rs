use maycoon_core::app::context::AppContext;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::vgi::Scene;
use maycoon_core::widget::Widget;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;

/// A canvas widget to directly draw to the screen.
///
/// This is a very simplified version of "your own Widget" and you should only use it for simple cases.
///
/// ### Theming
/// The canvas cannot be themed, since it does not draw something on itself.
///
/// The [WidgetId] is equal to `maycoon-widgets:Canvas`.
pub struct Canvas {
    painter: Box<dyn FnMut(&mut dyn Scene, &AppInfo)>,
}

impl Canvas {
    /// Create a new Canvas widget from a painter function.
    #[inline(always)]
    pub fn new(painter: impl FnMut(&mut dyn Scene, &AppInfo) + 'static) -> Self {
        Self {
            painter: Box::new(painter),
        }
    }

    /// Set a painter function and return itself.
    #[inline(always)]
    pub fn with_painter(mut self, painter: impl FnMut(&mut dyn Scene, &AppInfo) + 'static) -> Self {
        self.painter = Box::new(painter);
        self
    }
}

impl Widget for Canvas {
    #[inline(always)]
    fn render(
        &mut self,
        scene: &mut dyn Scene,
        _: &mut dyn Theme,
        _: &LayoutNode,
        info: &AppInfo,
        _: AppContext,
    ) {
        let mut canvas = scene.dyn_clone();
        canvas.reset();

        (self.painter)(canvas.as_mut(), info);

        scene.append(canvas.as_ref(), None);
    }

    #[inline(always)]
    fn layout_style(&self) -> StyleNode {
        StyleNode {
            style: LayoutStyle::default(),
            children: Vec::new(),
        }
    }

    #[inline(always)]
    fn update(&mut self, _: &LayoutNode, _: AppContext, _: &AppInfo) -> Update {
        Update::DRAW | Update::LAYOUT
    }

    #[inline(always)]
    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Canvas")
    }
}
