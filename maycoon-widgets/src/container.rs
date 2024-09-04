use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::state::{State, Val};
use maycoon_core::vg::Scene;
use maycoon_core::widget::Widget;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;

/// A container widget that can display and layout multiple child widgets.
///
/// The layout of the children (row, column, etc.) depends on the [LayoutStyle] of the container.
///
/// ### Theming
/// The container widget doesn't actually draw anything but the child widgets, so theming is useless.
pub struct Container<S: State> {
    style: Val<S, LayoutStyle>,
    children: Vec<Val<S, Box<dyn Widget<S>>>>,
}

impl<S: State> Container<S> {
    /// Creates a new container with given children.
    pub fn new(children: Vec<Val<S, Box<dyn Widget<S>>>>) -> Self {
        Self {
            style: LayoutStyle::default().into(),
            children,
        }
    }

    /// Sets the layout style of the container.
    pub fn with_layout_style(mut self, layout_style: impl Into<Val<S, LayoutStyle>>) -> Self {
        self.style = layout_style.into();
        self
    }

    /// Adds the given children to the container.
    pub fn add_children(&mut self, children: Vec<Val<S, Box<dyn Widget<S>>>>) {
        self.children.extend(children);
    }

    /// Adds the given children to the container and returns itself.
    pub fn with_children(mut self, children: Vec<Val<S, Box<dyn Widget<S>>>>) -> Self {
        self.add_children(children);
        self
    }

    /// Adds the given child to the container.
    pub fn add_child(&mut self, child: impl Into<Val<S, Box<dyn Widget<S>>>>) {
        self.children.push(child.into());
    }

    /// Adds the given child to the container and returns itself.
    pub fn with_child(mut self, child: impl Into<Val<S, Box<dyn Widget<S>>>>) -> Self {
        self.add_child(child);
        self
    }
}

impl<S: State> Default for Container<S> {
    fn default() -> Self {
        Self::new(Vec::new())
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
            let child = child.get_mut(state);
            child.render(scene, theme, info, &layout_node.children[i], state);
        }
    }

    fn layout_style(&mut self, state: &S) -> StyleNode {
        let style = self.style.get_ref(state).clone();

        let mut children = Vec::with_capacity(self.children.len());

        for child in &mut self.children {
            let child = child.get_mut(state);

            children.push(child.layout_style(state));
        }

        StyleNode { style, children }
    }

    fn update(&mut self, layout: &LayoutNode, state: &mut S, info: &AppInfo) -> Update {
        self.style.invalidate();

        let mut update = Update::empty();

        for (i, child) in self.children.iter_mut().enumerate() {
            let child = child.get_mut(state);
            update.insert(child.update(&layout.children[i], state, info));
        }

        update
    }

    fn widget_id(&mut self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Container")
    }
}
