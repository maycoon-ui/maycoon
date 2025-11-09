use crate::app::context::AppContext;
use crate::app::info::AppInfo;
use crate::app::update::Update;
use crate::layout::{LayoutNode, LayoutStyle, StyleNode};
use crate::signal::MaybeSignal;
use crate::vgi::Scene;
use crate::widget::{BoxedWidget, Widget, WidgetChildExt, WidgetChildrenExt, WidgetLayoutExt};
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

/// A trait for creating a [Widget] from simple functions.
///
/// Simplifies the creation of your own widgets.
pub trait Component {
    /// Builds the inner widget.
    fn build(&self, context: AppContext) -> impl Widget + 'static;

    /// The id of this widget/component.
    fn widget_id(&self) -> WidgetId;

    /// Composes this component into a [Widget] using [ComposedWidget].
    #[inline(always)]
    fn compose(self) -> Composed<Self>
    where
        Self: Sized,
    {
        Composed {
            component: self,
            widget: None,
        }
    }
}

/// A [Widget] created from a [Component].
pub struct Composed<C: Component> {
    component: C,
    widget: Option<BoxedWidget>,
}

impl<C: Component> Composed<C> {
    /// Creates a new [Composed] widget from a [Component].
    #[inline(always)]
    pub fn new(component: C) -> Self {
        Self {
            component,
            widget: None,
        }
    }
}

impl<C: Component> Widget for Composed<C> {
    #[inline(always)]
    fn render(
        &mut self,
        scene: &mut dyn Scene,
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

    #[inline(always)]
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

    #[inline(always)]
    fn update(&mut self, layout: &LayoutNode, context: AppContext, info: &AppInfo) -> Update {
        if let Some(widget) = &mut self.widget {
            widget.update(layout, context, info)
        } else {
            self.widget = Some(Box::new(self.component.build(context.clone())));
            Update::FORCE
        }
    }

    #[inline(always)]
    fn widget_id(&self) -> WidgetId {
        self.component.widget_id()
    }
}

impl<C: Component + WidgetChildrenExt> WidgetChildrenExt for Composed<C> {
    #[inline(always)]
    fn set_children(&mut self, children: Vec<BoxedWidget>) {
        self.component.set_children(children)
    }

    #[inline(always)]
    fn with_children(self, children: Vec<BoxedWidget>) -> Self
    where
        Self: Sized,
    {
        self.component.with_children(children).compose()
    }

    #[inline(always)]
    fn add_child(&mut self, child: impl Widget + 'static) {
        self.component.add_child(child)
    }

    #[inline(always)]
    fn with_child(self, child: impl Widget + 'static) -> Self
    where
        Self: Sized,
    {
        self.component.with_child(child).compose()
    }
}

impl<C: Component + WidgetChildExt> WidgetChildExt for Composed<C> {
    #[inline(always)]
    fn set_child(&mut self, child: impl Widget + 'static) {
        self.component.set_child(child)
    }

    #[inline(always)]
    fn with_child(self, child: impl Widget + 'static) -> Self
    where
        Self: Sized,
    {
        self.component.with_child(child).compose()
    }
}

impl<C: Component + WidgetLayoutExt> WidgetLayoutExt for Composed<C> {
    #[inline(always)]
    fn set_layout_style(&mut self, layout_style: impl Into<MaybeSignal<LayoutStyle>>) {
        self.component.set_layout_style(layout_style)
    }

    #[inline(always)]
    fn with_layout_style(self, layout_style: impl Into<MaybeSignal<LayoutStyle>>) -> Self
    where
        Self: Sized,
    {
        self.component.with_layout_style(layout_style).compose()
    }
}

impl<C: Component + Clone> Clone for Composed<C> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            component: self.component.clone(),
            widget: None,
        }
    }
}

impl<C: Component + Debug> Debug for Composed<C> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComposedWidget")
            .field("component", &self.component)
            .field("widget", &self.widget.as_ref().map(|_| "?".to_string()))
            .finish()
    }
}

impl<C: Component> Deref for Composed<C> {
    type Target = C;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.component
    }
}

impl<C: Component> DerefMut for Composed<C> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component
    }
}
