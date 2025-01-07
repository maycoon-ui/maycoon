use maycoon_core::state::{State, Val};
use maycoon_core::widget::Widget;

/// An extension trait for widgets with a single child widget.
pub trait WidgetChildExt<S: State, W: Widget<S> + 'static> {
    /// Sets the child widget of the widget.
    fn set_child(&mut self, child: impl Into<Val<S, W>>);

    /// Sets the child widget of the widget and returns self.
    fn with_child(mut self, child: impl Into<Val<S, W>>) -> Self
    where
        Self: Sized,
    {
        self.set_child(child);
        self
    }
}

/// An extension trait for widgets with multiple child widgets.
pub trait WidgetChildrenExt<S: State> {
    /// Sets the child widgets of the widget.
    fn set_children(&mut self, children: Vec<Val<S, impl Widget<S> + 'static>>);

    /// Sets the child widgets of the widget and returns self.
    fn with_children(mut self, children: Vec<Val<S, impl Widget<S> + 'static>>) -> Self
    where
        Self: Sized,
    {
        self.set_children(children);
        self
    }

    /// Adds a child widget to the widget.
    fn add_child<W: Widget<S> + 'static>(&mut self, child: impl Into<Val<S, W>>);

    /// Adds a child widget to the widget and returns self.
    fn with_child<W: Widget<S> + 'static>(mut self, child: impl Into<Val<S, W>>) -> Self
    where
        Self: Sized,
    {
        self.add_child(child);
        self
    }
}

/// An extension trait for widgets with a layout style.
pub trait WidgetLayoutExt<S: State> {
    /// Sets the layout style of the widget.
    fn set_layout_style(
        &mut self,
        layout_style: impl Into<Val<S, maycoon_core::layout::LayoutStyle>>,
    );

    /// Sets the layout style of the widget and returns self.
    fn with_layout_style(
        mut self,
        layout_style: impl Into<Val<S, maycoon_core::layout::LayoutStyle>>,
    ) -> Self
    where
        Self: Sized,
    {
        self.set_layout_style(layout_style);
        self
    }
}
