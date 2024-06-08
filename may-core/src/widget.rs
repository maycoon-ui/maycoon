use may_theme::id::WidgetId;
use vello::Scene;

use may_theme::theme::Theme;

use crate::app::info::AppInfo;
use crate::app::update::Update;
use crate::layout::{LayoutNode, StyleNode};
use crate::state::State;

pub trait Widget<S: State> {
    fn render(
        &self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        info: &AppInfo,
        layout_node: &LayoutNode,
    );
    fn layout_style(&self) -> StyleNode;
    fn update(&mut self, layout: &LayoutNode, state: &mut S, info: &AppInfo) -> Update;
    fn widget_id(&self) -> WidgetId;
}
