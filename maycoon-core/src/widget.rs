use vello::Scene;

use crate::app::context::AppContext;
use crate::app::info::AppInfo;
use crate::app::update::Update;
use crate::layout::{LayoutNode, StyleNode};
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;

/// A boxed widget.
pub type BoxedWidget = Box<dyn Widget>;

/// The base trait for all widgets.
pub trait Widget {
    /// Render the widget to the canvas.
    fn render(
        &mut self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        layout_node: &LayoutNode,
        info: &AppInfo,
        context: AppContext,
    );

    /// Return the layout style node for layout computation.
    fn layout_style(&self) -> StyleNode;

    /// Update the widget state with given info and layout. Returns if the app should be updated.
    fn update(&mut self, layout: &LayoutNode, context: AppContext, info: &AppInfo) -> Update;

    /// Return the widget id.
    fn widget_id(&self) -> WidgetId;
}
