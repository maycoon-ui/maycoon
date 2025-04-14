use crate::ext::{WidgetChildrenExt, WidgetLayoutExt};
use maycoon_core::app::context::AppContext;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::signal::MaybeSignal;
use maycoon_core::vg::Scene;
use maycoon_core::widget::{BoxedWidget, Widget};
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;

/// A container widget that can display and layout multiple child widgets.
///
/// The layout of the children (row, column, etc.) depends on the [LayoutStyle] of the container.
///
/// See the [counter](https://github.com/maycoon-ui/maycoon/blob/master/examples/counter/src/main.rs) example for how to use it in practice.
///
/// ### Theming
/// The container widget doesn't actually draw anything but the child widgets, so theming is useless.
#[derive(Default)]
pub struct Container {
    style: MaybeSignal<LayoutStyle>,
    children: Vec<BoxedWidget>,
}

impl Container {
    /// Creates a new container with given children.
    pub fn new(children: Vec<BoxedWidget>) -> Self {
        Self {
            style: LayoutStyle::default().into(),
            children: children.into_iter().collect(),
        }
    }
}

impl WidgetChildrenExt for Container {
    fn set_children(&mut self, children: Vec<BoxedWidget>) {
        self.children = children.into_iter().collect();
    }

    fn add_child(&mut self, child: impl Widget + 'static) {
        self.children.push(Box::new(child));
    }
}

impl WidgetLayoutExt for Container {
    fn set_layout_style(&mut self, layout_style: impl Into<MaybeSignal<LayoutStyle>>) {
        self.style = layout_style.into();
    }
}

impl Widget for Container {
    fn render(
        &mut self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        layout_node: &LayoutNode,
        info: &AppInfo,
        context: AppContext,
    ) {
        for (i, child) in self.children.iter_mut().enumerate() {
            child.render(
                scene,
                theme,
                &layout_node.children[i],
                info,
                context.clone(),
            );
        }
    }

    fn layout_style(&self) -> StyleNode {
        let style = self.style.get().clone();

        let mut children = Vec::with_capacity(self.children.len());

        for child in &self.children {
            children.push(child.layout_style());
        }

        StyleNode { style, children }
    }

    fn update(&mut self, layout: &LayoutNode, context: AppContext, info: &AppInfo) -> Update {
        let mut update = Update::empty();

        for (i, child) in self.children.iter_mut().enumerate() {
            update.insert(child.update(&layout.children[i], context.clone(), info));
        }

        update
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Container")
    }
}
