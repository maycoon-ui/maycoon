use vello::Scene;

use may_theme::id::WidgetId;
use may_theme::theme::Theme;

use crate::app::info::AppInfo;
use crate::app::update::Update;
use crate::layout::{LayoutNode, StyleNode};
use crate::state::State;

/// The base trait for all widgets.
pub trait Widget<S: State> {
    /// Render the widget to the canvas.
    fn render(
        &self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        info: &AppInfo,
        layout_node: &LayoutNode,
        state: &S,
    );
    /// Return the layout style node for layout computation.
    fn layout_style(&self, state: &S) -> StyleNode;
    /// Update the widget state with given info and layout. Returns if the app should be updated.
    fn update(&mut self, layout: &LayoutNode, state: &mut S, info: &AppInfo) -> Update;
    /// Return the widget id.
    fn widget_id(&self) -> WidgetId;
}
