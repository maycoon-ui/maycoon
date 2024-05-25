use crate::app::update::Update;
use crate::render::CanvasContext;
use crate::state::State;
use may_theme::id::WidgetId;
use may_theme::theme::Theme;
use taffy::{Layout, Style};

pub trait Widget<S: State> {
    fn render(&self, canvas: &mut CanvasContext, layout: &LayoutNode, theme: &mut dyn Theme);
    fn layout_style(&self) -> StyleNode;
    fn widget_id(&self) -> WidgetId;
    fn update(&mut self, layout: &LayoutNode, state: &mut S) -> Update;
}

pub struct StyleNode {
    pub style: Style,
    pub children: Vec<StyleNode>,
}

#[derive(Clone, Debug)]
pub struct LayoutNode {
    pub layout: Layout,
    pub children: Vec<LayoutNode>,
}
