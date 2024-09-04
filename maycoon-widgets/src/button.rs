use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout;
use maycoon_core::layout::{LayoutNode, LayoutStyle, LengthPercentage, StyleNode};
use maycoon_core::state::{State, Val};
use maycoon_core::vg::kurbo::{Affine, Rect, RoundedRect, RoundedRectRadii, Vec2};
use maycoon_core::vg::peniko::{Brush, Fill};
use maycoon_core::vg::Scene;
use maycoon_core::widget::Widget;
use maycoon_core::window::{ElementState, MouseButton};
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;

/// An interactive area with a child widget that runs a closure when pressed.
///
/// ### Theming
/// Styling buttons require following properties:
/// - `color_pressed` -  The color of the button when pressed.
/// - `color_idle` - The color of the button when not pressed and not hovered (idling).
/// - `color_hovered` - The color of the button when hovered on.
pub struct Button<S: State, W: Widget<S> + 'static> {
    child: Val<S, W>,
    state: ButtonState,
    on_pressed: Box<dyn FnMut(&mut S) -> Update>,
    layout_style: Val<S, LayoutStyle>,
}

impl<S: State, W: Widget<S> + 'static> Button<S, W> {
    /// Create a new button with the given child widget.
    pub fn new(child: impl Into<Val<S, W>>) -> Self {
        Self {
            child: child.into(),
            state: ButtonState::Idle,
            on_pressed: Box::new(|_| Update::empty()),
            layout_style: LayoutStyle {
                padding: layout::Rect::<LengthPercentage> {
                    left: LengthPercentage::Length(12.0),
                    right: LengthPercentage::Length(12.0),
                    top: LengthPercentage::Length(2.0),
                    bottom: LengthPercentage::Length(10.0),
                },
                ..Default::default()
            }
            .into(),
        }
    }

    /// Sets the function to be called when the button is pressed.
    pub fn with_on_pressed(mut self, on_pressed: impl FnMut(&mut S) -> Update + 'static) -> Self {
        self.on_pressed = Box::new(on_pressed);
        self
    }

    /// Sets the layout style of the button.
    pub fn with_layout_style(mut self, layout_style: impl Into<Val<S, LayoutStyle>>) -> Self {
        self.layout_style = layout_style.into();
        self
    }
}

impl<S: State, W: Widget<S>> Widget<S> for Button<S, W> {
    fn render(
        &mut self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        info: &AppInfo,
        layout_node: &LayoutNode,
        state: &S,
    ) {
        let brush = if let Some(style) = theme.of(self.widget_id()) {
            match self.state {
                ButtonState::Idle => Brush::Solid(style.get_color("color_idle").unwrap()),
                ButtonState::Hovered => Brush::Solid(style.get_color("color_hovered").unwrap()),
                ButtonState::Pressed => Brush::Solid(style.get_color("color_pressed").unwrap()),
                ButtonState::Released => Brush::Solid(style.get_color("color_hovered").unwrap()),
            }
        } else {
            Brush::Solid(match self.state {
                ButtonState::Idle => theme.defaults().interactive().inactive(),
                ButtonState::Hovered => theme.defaults().interactive().hover(),
                ButtonState::Pressed => theme.defaults().interactive().active(),
                ButtonState::Released => theme.defaults().interactive().hover(),
            })
        };

        scene.fill(
            Fill::NonZero,
            Affine::default(),
            &brush,
            None,
            &RoundedRect::from_rect(
                Rect::new(
                    layout_node.layout.location.x as f64,
                    layout_node.layout.location.y as f64,
                    (layout_node.layout.location.x + layout_node.layout.size.width) as f64,
                    (layout_node.layout.location.y + layout_node.layout.size.height) as f64,
                ),
                RoundedRectRadii::from_single_radius(10.0),
            ),
        );

        {
            theme.globals_mut().invert_text_color = true;

            let mut child_scene = Scene::new();

            self.child.get_mut(state).render(
                &mut child_scene,
                theme,
                info,
                &layout_node.children[0],
                state,
            );

            scene.append(
                &child_scene,
                Some(Affine::translate(Vec2::new(
                    layout_node.layout.location.x as f64,
                    layout_node.layout.location.y as f64,
                ))),
            );

            theme.globals_mut().invert_text_color = false;
        }
    }

    fn layout_style(&mut self, state: &S) -> StyleNode {
        StyleNode {
            style: self.layout_style.get_ref(state).clone(),
            children: vec![self.child.get_mut(state).layout_style(state)],
        }
    }

    fn update(&mut self, layout: &LayoutNode, state: &mut S, info: &AppInfo) -> Update {
        let mut update = Update::empty();
        let old_state = self.state;

        // check for hovering
        if let Some(cursor) = info.cursor_pos {
            if cursor.x as f32 >= layout.layout.location.x
                && cursor.x as f32 <= layout.layout.location.x + layout.layout.size.width
                && cursor.y as f32 >= layout.layout.location.y
                && cursor.y as f32 <= layout.layout.location.y + layout.layout.size.height
            {
                // fixes state going to hover if the button is pressed but not yet released
                if self.state != ButtonState::Pressed {
                    self.state = ButtonState::Hovered;
                }

                // check for click
                for (_, btn, el) in &info.buttons {
                    if *btn == MouseButton::Left {
                        match el {
                            ElementState::Pressed => {
                                self.state = ButtonState::Pressed;
                            },

                            // actually fire the event if the button is released
                            ElementState::Released => {
                                self.state = ButtonState::Released;
                                update |= (self.on_pressed)(state);
                            },
                        }
                    }
                }
            } else {
                // cursor not in area, so button is idle
                self.state = ButtonState::Idle;
            }
        } else {
            // cursor is not in window, so button is idle
            self.state = ButtonState::Idle;
        }

        // update on state change, due to re-coloring
        if old_state != self.state {
            update |= Update::DRAW;
        }

        update
    }

    fn widget_id(&mut self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Button")
    }
}

/// The internal state of the button.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ButtonState {
    /// The button is idling (inactive).
    Idle,
    /// The cursor is hovering over the button.
    Hovered,
    /// The cursor is hovering over the button and the left click button is pressed.
    Pressed,
    /// The cursor is hovering over the button and the left click button is released.
    /// This is when the `on_pressed` function is called.
    Released,
}
