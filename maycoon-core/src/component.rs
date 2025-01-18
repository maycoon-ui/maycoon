use crate::app::info::AppInfo;
use crate::app::update::Update;
use crate::layout::{LayoutNode, StyleNode};
use crate::state::State;
use crate::widget::Widget;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use vello::Scene;

/// The core trait for all components.
///
/// A [Component] is composed out of an already existing [Widget].
/// The advantage of using a component is that you can easily create your own widgets
/// with your own logic without handling rendering, updating and other things.
///
/// In order to make it usable, you'll need to call [Component::build] on it,
/// to convert it into a real widget (more specifically the [ComponentAdapter]).
/// This is required to not confuse the compiler with overlapping traits.
pub trait Component<S: State> {
    /// Returns a mutable reference to the inner [Widget].
    fn get_widget(&mut self) -> &mut impl Widget<S>;

    /// Get the ID of your component as a [WidgetId].
    fn get_widget_id(&self) -> WidgetId;

    /// Converts the component into a widget by wrapping it in a [ComponentAdapter].
    fn build(self) -> ComponentAdapter<S, Self>
    where
        Self: Sized,
    {
        ComponentAdapter::new(self)
    }
}

/// An adapter around a [Component] to make it usable as a [Widget].
///
/// You can still call methods of the inner [Component], due to [Deref coercion](Deref)
pub struct ComponentAdapter<S: State, C: Component<S>> {
    inner: C,
    phantom: PhantomData<S>,
}

impl<S: State, C: Component<S>> ComponentAdapter<S, C> {
    /// Creates a new [ComponentAdapter] by using the given [Component].
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<S: State, C: Component<S>> Deref for ComponentAdapter<S, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S: State, C: Component<S>> DerefMut for ComponentAdapter<S, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<S: State, C: Component<S>> Widget<S> for ComponentAdapter<S, C> {
    fn render(
        &mut self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        info: &AppInfo,
        layout_node: &LayoutNode,
        state: &S,
    ) {
        self.inner
            .get_widget()
            .render(scene, theme, info, layout_node, state)
    }

    fn layout_style(&mut self, state: &S) -> StyleNode {
        self.inner.get_widget().layout_style(state)
    }

    fn update(&mut self, layout: &LayoutNode, state: &mut S, info: &AppInfo) -> Update {
        self.inner.get_widget().update(layout, state, info)
    }

    fn widget_id(&self) -> WidgetId {
        self.inner.get_widget_id()
    }
}
