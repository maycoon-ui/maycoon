use crate::app::context::AppContext;
use crate::app::info::AppInfo;
use crate::app::update::Update;
use crate::layout::{LayoutNode, LayoutStyle, StyleNode};
use crate::widget::{BoxedWidget, Widget};
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use std::ops::{Deref, DerefMut};
use vello::Scene;

/// A trait for creating a [Widget] from simple functions.
///
/// Simplifies the creation of your own widgets.
pub trait Component {
    /// Builds the inner widget.
    fn build(&self, context: AppContext) -> impl Widget + 'static;

    /// The id of this widget/component.
    fn widget_id(&self) -> WidgetId;

    /// Composes this component into a [Widget] using [ComposedWidget].
    fn compose(self) -> ComposedWidget<Self>
    where
        Self: Sized,
    {
        ComposedWidget {
            component: self,
            widget: None,
        }
    }
}

/// A [Widget] created from a [Component].
pub struct ComposedWidget<C: Component> {
    component: C,
    widget: Option<BoxedWidget>,
}

impl<C: Component> ComposedWidget<C> {
    /// Creates a new [ComposedWidget] from a [Component].
    pub fn new(component: C) -> Self {
        Self {
            component,
            widget: None,
        }
    }
}

impl<C: Component> Widget for ComposedWidget<C> {
    fn render(
        &mut self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        layout_node: &LayoutNode,
        info: &AppInfo,
        context: AppContext,
    ) {
        if let Some(widget) = &mut self.widget {
            widget.render(scene, theme, layout_node, info, context)
        } else {
            self.widget = Some(Box::new(self.component.build(context.clone())));
        }
    }

    fn layout_style(&self) -> StyleNode {
        if let Some(widget) = &self.widget {
            widget.layout_style()
        } else {
            StyleNode {
                style: LayoutStyle::default(),
                children: Vec::new(),
            }
        }
    }

    fn update(&mut self, layout: &LayoutNode, context: AppContext, info: &AppInfo) -> Update {
        if let Some(widget) = &mut self.widget {
            widget.update(layout, context, info)
        } else {
            self.widget = Some(Box::new(self.component.build(context.clone())));
            Update::FORCE
        }
    }

    fn widget_id(&self) -> WidgetId {
        self.component.widget_id()
    }
}

impl<C: Component> Deref for ComposedWidget<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.component
    }
}

impl<C: Component> DerefMut for ComposedWidget<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component
    }
}
