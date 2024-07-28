use may_core::app::info::AppInfo;
use may_core::app::update::Update;
use may_core::layout;
use may_core::layout::{LayoutNode, LayoutStyle, LengthPercentage, StyleNode};
use may_core::state::State;
use may_core::vg::kurbo::{Affine, Rect, RoundedRect, RoundedRectRadii, Vec2};
use may_core::vg::peniko::{Brush, Fill};
use may_core::vg::Scene;
use may_core::widget::Widget;
use may_core::window::{ElementState, MouseButton};
use may_theme::id::WidgetId;
use may_theme::theme::Theme;

/// A generic Button widget to make an interactive button-style widget with a child.
pub struct Button<S: State> {
    child: Box<dyn Widget<S>>,
    state: ButtonState,
    on_pressed: Box<dyn FnMut(&mut S) -> Update>,
    layout_style: LayoutStyle,
}

impl<S: State> Button<S> {
    /// Create a new button with the given child widget.
    pub fn new(child: impl Widget<S> + 'static) -> Self {
        Self {
            child: Box::new(child),
            state: ButtonState {
                active: false,
                hover: false,
                about_to_be_inactive: false,
            },
            on_pressed: Box::new(|_| Update::empty()),
            layout_style: LayoutStyle {
                padding: layout::Rect::<LengthPercentage> {
                    left: LengthPercentage::Length(12.0),
                    right: LengthPercentage::Length(12.0),
                    top: LengthPercentage::Length(2.0),
                    bottom: LengthPercentage::Length(10.0),
                },
                ..Default::default()
            },
        }
    }

    /// Sets the internal button state. Not recommended to use, unless you know what you're doing.
    pub fn with_state(mut self, state: ButtonState) -> Self {
        self.state = state;
        self
    }

    /// Sets the function to be called when the button is pressed.
    pub fn with_on_pressed(mut self, on_pressed: impl FnMut(&mut S) -> Update + 'static) -> Self {
        self.on_pressed = Box::new(on_pressed);
        self
    }

    /// Sets the layout style of the button.
    pub fn with_layout_style(mut self, layout_style: LayoutStyle) -> Self {
        self.layout_style = layout_style;
        self
    }
}

impl<S: State> Widget<S> for Button<S> {
    fn render(
        &mut self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        info: &AppInfo,
        layout_node: &LayoutNode,
        state: &S,
    ) {
        let brush = if let Some(style) = theme.of(self.widget_id()) {
            if self.state.active {
                style.get_brush("color_pressed").unwrap()
            } else if self.state.hover {
                style.get_brush("color_hovered").unwrap()
            } else {
                style.get_brush("color_idle").unwrap()
            }
        } else {
            Brush::Solid(if self.state.active {
                theme.defaults().interactive().active()
            } else if self.state.hover {
                theme.defaults().interactive().hover()
            } else {
                theme.defaults().interactive().inactive()
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

            self.child.render(
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
            style: self.layout_style.clone(),
            children: vec![self.child.layout_style(state)],
        }
    }

    fn update(&mut self, layout: &LayoutNode, state: &mut S, info: &AppInfo) -> Update {
        let mut update = Update::empty();
        let old_state = self.state;

        if old_state.about_to_be_inactive {
            self.state.about_to_be_inactive = false;
            self.state.active = false;
        }

        if let Some(cursor) = info.cursor_pos {
            self.state.hover = cursor.x as f32 >= layout.layout.location.x
                && cursor.x as f32 <= layout.layout.location.x + layout.layout.size.width
                && cursor.y as f32 >= layout.layout.location.y
                && cursor.y as f32 <= layout.layout.location.y + layout.layout.size.height;
        } else {
            self.state.hover = false;
        }

        if self.state.hover {
            for &(_, btn, state) in &info.buttons {
                if MouseButton::Left == btn {
                    match state {
                        ElementState::Pressed => {
                            self.state.active = true;
                        },

                        ElementState::Released => {
                            self.state.active = false;
                        },
                    }
                } else {
                    self.state.active = false;
                }
            }
        } else {
            self.state.active = false;
        }

        if self.state.active {
            update.insert((self.on_pressed)(state));
            self.state.about_to_be_inactive = true;
        }

        if self.state != old_state {
            update.insert(Update::DRAW);
        }

        update
    }

    fn widget_id(&mut self) -> WidgetId {
        WidgetId::new("may-widgets", "Button")
    }
}

/// The internal state of the button.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ButtonState {
    /// If the button is hovered on.
    pub hover: bool,
    /// If the button is active (pressed).
    pub active: bool,
    // todo: find a better way of handling click issue
    about_to_be_inactive: bool,
}
