use maycoon_core::signal::MaybeSignal;
use maycoon_core::widget::{BoxedWidget, Widget};

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
    fn set_layout_style(
        &mut self,
        layout_style: impl Into<MaybeSignal<maycoon_core::layout::LayoutStyle>>,
    );

    /// Sets the layout style of the widget and returns self.
    fn with_layout_style(
        mut self,
        layout_style: impl Into<MaybeSignal<maycoon_core::layout::LayoutStyle>>,
    ) -> Self
    where
        Self: Sized,
    {
        self.set_layout_style(layout_style);
        self
    }
}
