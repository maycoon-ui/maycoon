use crate::app::context::AppContext;
use crate::app::info::AppInfo;
use crate::app::update::Update;
use crate::layout::{LayoutNode, LayoutStyle, StyleNode};
use crate::signal::MaybeSignal;
use crate::vgi::Scene;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;

/// A boxed widget.
pub type BoxedWidget = Box<dyn Widget>;

/// The base trait for all widgets.
pub trait Widget {
    /// Render the widget to the canvas.
    fn render(
        &mut self,
        scene: &mut dyn Scene,
        theme: &mut dyn Theme,
        layout_node: &LayoutNode,
        info: &AppInfo,
        context: AppContext,
    );

    /// Return the layout style node for layout computation.
    fn layout_style(&self) -> StyleNode;

    /// Update the widget state with given info and layout. Returns if the app should be updated.
    fn update(&mut self, layout: &LayoutNode, context: AppContext, info: &AppInfo) -> Update;

    /// Return the widget id.
    fn widget_id(&self) -> WidgetId;
}

/// An extension trait for widgets with a single child widget.
pub trait WidgetChildExt {
    /// Sets the child widget of the widget.
    fn set_child(&mut self, child: impl Widget + 'static);

    /// Sets the child widget of the widget and returns self.
    fn with_child(mut self, child: impl Widget + 'static) -> Self
    where
        Self: Sized,
    {
        self.set_child(child);
        self
    }
}

/// An extension trait for widgets with multiple child widgets.
pub trait WidgetChildrenExt {
    /// Sets the child widgets of the widget.
    fn set_children(&mut self, children: Vec<BoxedWidget>);

    /// Sets the child widgets of the widget and returns self.
    fn with_children(mut self, children: Vec<BoxedWidget>) -> Self
    where
        Self: Sized,
    {
        self.set_children(children);
        self
    }

    /// Adds a child widget to the widget.
    fn add_child(&mut self, child: impl Widget + 'static);

    /// Adds a child widget to the widget and returns self.
    fn with_child(mut self, child: impl Widget + 'static) -> Self
    where
        Self: Sized,
    {
        self.add_child(child);
        self
    }
}

/// An extension trait for widgets with a layout style.
pub trait WidgetLayoutExt {
    /// Sets the layout style of the widget.
    fn set_layout_style(&mut self, layout_style: impl Into<MaybeSignal<LayoutStyle>>);

    /// Sets the layout style of the widget and returns self.
    fn with_layout_style(mut self, layout_style: impl Into<MaybeSignal<LayoutStyle>>) -> Self
    where
        Self: Sized,
    {
        self.set_layout_style(layout_style);
        self
    }
}
