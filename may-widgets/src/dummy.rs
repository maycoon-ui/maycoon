use may_core::app::info::AppInfo;
use may_core::app::update::Update;
use may_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use may_core::state::State;
use may_core::vg::Scene;
use may_core::widget::Widget;
use may_theme::id::WidgetId;
use may_theme::theme::Theme;

/// A dummy Widget that does nothing. Useful for testing.
pub struct DummyWidget;

impl<S: State> Widget<S> for DummyWidget {
    fn render(&self, _: &mut Scene, _: &mut dyn Theme, _: &AppInfo, _: &LayoutNode, _: &S) {}

    fn layout_style(&self, _: &S) -> StyleNode {
        StyleNode {
            style: LayoutStyle::default(),
            children: Vec::new(),
        }
    }

    fn update(&mut self, _: &LayoutNode, _: &mut S, _: &AppInfo) -> Update {
        Update::empty()
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("may-widgets", "DummyWidget")
    }
}
