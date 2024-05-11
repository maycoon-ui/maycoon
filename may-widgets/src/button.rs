use crate::CRATE_NAME;
use may_core::app::context::Context;
use may_core::app::update::Update;
use may_core::render::RenderCommand;
use may_core::state::State;
use may_core::widget::{Widget, WidgetLayoutNode, WidgetStyleNode};
use may_theme::id::WidgetId;
use may_theme::scheme::Scheme;
use may_theme::theme::WidgetType;

pub struct Button<S: State> {
    on_click: Box<dyn FnMut(&mut S) -> Update>,
}

impl<S: State> Widget<S> for Button<S> {
    fn render(&self, theme: Scheme, layout: WidgetLayoutNode) -> Vec<RenderCommand> {
        vec![]
    }

    fn id(&self) -> WidgetId {
        WidgetId::new(CRATE_NAME, "Button")
    }

    fn update(&mut self, state: &mut S, ctx: &Context) -> Update {
        Update::empty()
    }

    fn style_node(&self) -> WidgetStyleNode {
        todo!()
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Interactive
    }
}
