use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::state::State;
use maycoon_core::vg::Scene;
use maycoon_core::widget::Widget;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;

/// A dummy Widget that does nothing. Useful for testing.
///
/// ### Theming
/// It's a dummy widget.
pub struct DummyWidget;

impl<S: State> Widget<S> for DummyWidget {
    fn render(&mut self, _: &mut Scene, _: &mut dyn Theme, _: &AppInfo, _: &LayoutNode, _: &S) {}

    fn layout_style(&mut self, _: &S) -> StyleNode {
        StyleNode {
            style: LayoutStyle::default(),
            children: Vec::new(),
        }
    }

    fn update(&mut self, _: &LayoutNode, _: &mut S, _: &AppInfo) -> Update {
        Update::empty()
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "DummyWidget")
    }
}
