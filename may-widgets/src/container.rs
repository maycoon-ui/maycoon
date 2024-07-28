use may_core::app::info::AppInfo;
use may_core::app::update::Update;
use may_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use may_core::state::State;
use may_core::vg::Scene;
use may_core::widget::Widget;
use may_theme::id::WidgetId;
use may_theme::theme::Theme;

/// A container widget that can display and layout multiple child widgets.
///
/// The layout of the children (row, column, etc.) depends on the [LayoutStyle] of the container.
pub struct Container<S: State> {
    style: LayoutStyle,
    children: Vec<Box<dyn Widget<S>>>,
}

impl<S: State> Container<S> {
    /// Creates a new container with given children.
    pub fn new(children: Vec<Box<dyn Widget<S>>>) -> Self {
        Self {
            style: LayoutStyle::default(),
            children,
        }
    }

    /// Sets the layout style of the container.
    pub fn with_layout_style(mut self, layout_style: LayoutStyle) -> Self {
        self.style = layout_style;
        self
    }
}

impl<S: State> Widget<S> for Container<S> {
    fn render(
        &mut self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        info: &AppInfo,
        layout_node: &LayoutNode,
        state: &S,
    ) {
        for (i, child) in self.children.iter_mut().enumerate() {
            child.render(scene, theme, info, &layout_node.children[i], state);
        }
    }

    fn layout_style(&mut self, state: &S) -> StyleNode {
        StyleNode {
            style: self.style.clone(),
            children: self
                .children
                .iter_mut()
                .map(|el| el.layout_style(state))
                .collect(),
        }
    }

    fn update(&mut self, layout: &LayoutNode, state: &mut S, info: &AppInfo) -> Update {
        let mut update = Update::empty();

        for (i, child) in self.children.iter_mut().enumerate() {
            update.insert(child.update(&layout.children[i], state, info));
        }

        update
    }

    fn widget_id(&mut self) -> WidgetId {
        WidgetId::new("may-widgets", "Container")
    }
}
