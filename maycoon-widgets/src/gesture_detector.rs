use maycoon_core::app::context::AppContext;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, StyleNode};
use maycoon_core::signal::MaybeSignal;
use maycoon_core::vg::Scene;
use maycoon_core::widget::{BoxedWidget, Widget};
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
pub struct GestureDetector {
    child: BoxedWidget,
    on_press: MaybeSignal<Update>,
    on_release: MaybeSignal<Update>,
    on_hover: MaybeSignal<Update>,
}

impl GestureDetector {
    /// Creates a new [`GestureDetector`] with the given child widget.
    pub fn new(child: impl Widget + 'static) -> Self {
        Self {
            child: Box::new(child),
            on_press: MaybeSignal::value(Update::empty()),
            on_release: MaybeSignal::value(Update::empty()),
            on_hover: MaybeSignal::value(Update::empty()),
        }
    }

    /// Sets the child widget of the [`GestureDetector`] and returns self.
    pub fn with_child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Box::new(child);
        self
    }

    /// Sets the `on_press` callback of the [`GestureDetector`] and returns self.
    pub fn with_on_press(mut self, on_press: impl Into<MaybeSignal<Update>>) -> Self {
        self.on_press = on_press.into();
        self
    }

    /// Sets the `on_release` callback of the [`GestureDetector`] and returns self.
    pub fn with_on_release(mut self, on_release: impl Into<MaybeSignal<Update>>) -> Self {
        self.on_release = on_release.into();
        self
    }

    /// Sets the `on_hover` callback of the [`GestureDetector`] and returns self.
    pub fn with_on_hover(mut self, on_hover: impl Into<MaybeSignal<Update>>) -> Self {
        self.on_hover = on_hover.into();
        self
    }

    /// Call the `on_hover` callback of the [`GestureDetector`].
    pub fn on_hover(&mut self) -> Update {
        *self.on_hover.get()
    }

    /// Call the `on_press` callback of the [`GestureDetector`].
    pub fn on_press(&mut self) -> Update {
        *self.on_press.get()
    }

    /// Call the `on_release` callback of the [`GestureDetector`].
    pub fn on_release(&mut self) -> Update {
        *self.on_release.get()
    }
}

impl Widget for GestureDetector {
    fn render(
        &mut self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        layout_node: &LayoutNode,
        info: &AppInfo,
        context: AppContext,
    ) {
        self.child.render(scene, theme, layout_node, info, context)
    }

    fn layout_style(&self) -> StyleNode {
        self.child.layout_style()
    }

    fn update(&mut self, layout: &LayoutNode, context: AppContext, info: &AppInfo) -> Update {
        let mut update = Update::empty();

        if let Some(cursor) = info.cursor_pos {
            if cursor.x as f32 >= layout.layout.location.x
                && cursor.x as f32 <= layout.layout.location.x + layout.layout.size.width
                && cursor.y as f32 >= layout.layout.location.y
                && cursor.y as f32 <= layout.layout.location.y + layout.layout.size.height
            {
                update |= self.on_hover();

                // check for click
                for (_, btn, el) in &info.buttons {
                    if *btn == MouseButton::Left {
                        match el {
                            ElementState::Pressed => {
                                update |= self.on_press();
                            },

                            ElementState::Released => {
                                update |= self.on_release();
                            },
                        }
                    }
                }
            }
        }

        update |= self.child.update(layout, context, info);

        update
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "GestureDetector")
    }
}
