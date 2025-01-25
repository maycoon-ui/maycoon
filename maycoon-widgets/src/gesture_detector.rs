use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, StyleNode};
use maycoon_core::state::{State, Val};
use maycoon_core::vg::Scene;
use maycoon_core::widget::Widget;
use maycoon_core::window::{ElementState, MouseButton};
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;

/// A widget to detect gestures like pressing or releasing the left mouse button.
/// It can also contain a child widget.
///
/// The [`GestureDetector`] has different callbacks that are called on different events:
/// - `on_press` is called when the left mouse button is pressed.
/// - `on_release` is called when the left mouse button is released.
/// - `on_hover` is called when the mouse cursor hovers over the widget.
///
/// ### Theming
/// The [`GestureDetector`] should not be themed and does not draw anything on itself.
/// It just contains the given child widget.
pub struct GestureDetector<S: State, W: Widget<S> + 'static> {
    child: Val<S, W>,
    on_press: Box<dyn Fn(&mut S) -> Update>,
    on_release: Box<dyn Fn(&mut S) -> Update>,
    on_hover: Box<dyn Fn(&mut S) -> Update>,
}

impl<S: State, W: Widget<S>> GestureDetector<S, W> {
    /// Creates a new [`GestureDetector`] with the given child widget.
    pub fn new(child: impl Into<Val<S, W>>) -> Self {
        Self {
            child: child.into(),
            on_press: Box::new(|_| Update::empty()),
            on_release: Box::new(|_| Update::empty()),
            on_hover: Box::new(|_| Update::empty()),
        }
    }

    /// Sets the child widget of the [`GestureDetector`] and returns self.
    pub fn with_child(mut self, child: impl Into<Val<S, W>>) -> Self {
        self.child = child.into();
        self
    }

    /// Sets the `on_press` callback of the [`GestureDetector`] and returns self.
    pub fn with_on_press(mut self, on_press: impl Fn(&mut S) -> Update + 'static) -> Self {
        self.on_press = Box::new(on_press);
        self
    }

    /// Sets the `on_release` callback of the [`GestureDetector`] and returns self.
    pub fn with_on_release(mut self, on_release: impl Fn(&mut S) -> Update + 'static) -> Self {
        self.on_release = Box::new(on_release);
        self
    }

    /// Sets the `on_hover` callback of the [`GestureDetector`] and returns self.
    pub fn with_on_hover(mut self, on_hover: impl Fn(&mut S) -> Update + 'static) -> Self {
        self.on_hover = Box::new(on_hover);
        self
    }

    /// Call the `on_hover` callback of the [`GestureDetector`].
    pub fn on_hover(&mut self, state: &mut S) -> Update {
        (self.on_hover)(state)
    }

    /// Call the `on_press` callback of the [`GestureDetector`].
    pub fn on_press(&mut self, state: &mut S) -> Update {
        (self.on_press)(state)
    }

    /// Call the `on_release` callback of the [`GestureDetector`].
    pub fn on_release(&mut self, state: &mut S) -> Update {
        (self.on_release)(state)
    }
}

impl<S: State, W: Widget<S>> Widget<S> for GestureDetector<S, W> {
    fn render(
        &mut self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        info: &AppInfo,
        layout_node: &LayoutNode,
        state: &S,
    ) {
        self.child
            .get_mut(state)
            .render(scene, theme, info, layout_node, state)
    }

    fn layout_style(&mut self, state: &S) -> StyleNode {
        self.child.get_mut(state).layout_style(state)
    }

    fn update(&mut self, layout: &LayoutNode, state: &mut S, info: &AppInfo) -> Update {
        let mut update = Update::empty();

        if let Some(cursor) = info.cursor_pos {
            if cursor.x as f32 >= layout.layout.location.x
                && cursor.x as f32 <= layout.layout.location.x + layout.layout.size.width
                && cursor.y as f32 >= layout.layout.location.y
                && cursor.y as f32 <= layout.layout.location.y + layout.layout.size.height
            {
                update |= self.on_hover(state);

                // check for click
                for (_, btn, el) in &info.buttons {
                    if *btn == MouseButton::Left {
                        match el {
                            ElementState::Pressed => {
                                update |= self.on_press(state);
                            },

                            ElementState::Released => {
                                update |= self.on_release(state);
                            },
                        }
                    }
                }
            }
        }

        update |= self.child.get_mut(state).update(layout, state, info);

        update
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "GestureDetector")
    }
}
