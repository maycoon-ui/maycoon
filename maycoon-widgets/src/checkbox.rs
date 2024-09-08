use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout;
use maycoon_core::layout::{Dimension, LayoutNode, LayoutStyle, LengthPercentageAuto, StyleNode};
use maycoon_core::state::{State, Val};
use maycoon_core::vg::kurbo::{Affine, Rect, RoundedRect, RoundedRectRadii, Stroke};
use maycoon_core::vg::peniko::{Brush, Fill};
use maycoon_core::vg::Scene;
use maycoon_core::widget::Widget;
use maycoon_core::window::{ElementState, MouseButton};
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use nalgebra::Vector2;

/// A checkbox widget. Changes state when it's clicked.
///
/// See the [checkbox](https://github.com/maycoon-ui/maycoon/blob/master/examples/checkbox/src/main.rs) example for how to use it in practice.
///
/// ### Theming
/// Styling the checkbox require following properties:
/// - `color_unchecked` -  The color of the checkbox, when it's not checked (inner value is false).
/// - `color_checked` - The color of the checkbox, when it's checked (inner value is true).
pub struct Checkbox<S: State> {
    layout_style: Val<S, LayoutStyle>,
    on_changed: Box<dyn FnMut(&mut S) -> Update>,
    value: Val<S, bool>,
}

impl<S: State> Checkbox<S> {
    /// Create a new checkbox with the given value.
    ///
    /// The value should be state dependent, so you can mutate it on change.
    pub fn new(value: Val<S, bool>) -> Self {
        Self {
            layout_style: Val::new_val(LayoutStyle {
                size: Vector2::<Dimension>::new(Dimension::Length(20.0), Dimension::Length(20.0)),
                margin: layout::Rect::<LengthPercentageAuto> {
                    left: LengthPercentageAuto::Length(0.5),
                    right: LengthPercentageAuto::Length(0.5),
                    top: LengthPercentageAuto::Length(0.5),
                    bottom: LengthPercentageAuto::Length(0.5),
                },
                ..Default::default()
            }),
            on_changed: Box::new(|_| Update::empty()),
            value,
        }
    }

    /// Sets the function to be called when the checkbox is clicked/changed.
    ///
    /// You should mutate the inner value of the checkbox using the provided state.
    pub fn with_on_changed(mut self, on_changed: impl FnMut(&mut S) -> Update + 'static) -> Self {
        self.on_changed = Box::new(on_changed);
        self
    }

    /// Sets the layout style of the checkbox and returns itself.
    pub fn with_layout_style(mut self, layout_style: impl Into<Val<S, LayoutStyle>>) -> Self {
        self.layout_style = layout_style.into();
        self
    }

    /// Sets the value of the checkbox and returns itself.
    ///
    /// The [Val] should be state dependent, so you can mutate it on change.
    pub fn with_value(mut self, value: impl Into<Val<S, bool>>) -> Self {
        self.value = value.into();
        self
    }
}

impl<S: State> Widget<S> for Checkbox<S> {
    fn render(
        &mut self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        _: &AppInfo,
        layout_node: &LayoutNode,
        state: &S,
    ) {
        let checked = *self.value.get_ref(state);

        let color = if let Some(style) = theme.of(self.widget_id()) {
            if checked {
                style.get_color("color_checked").unwrap()
            } else {
                style.get_color("color_unchecked").unwrap()
            }
        } else {
            if checked {
                theme.defaults().interactive().active()
            } else {
                theme.defaults().interactive().inactive()
            }
        };

        scene.stroke(
            &Stroke::new(3.0),
            Affine::default(),
            &Brush::Solid(color),
            None,
            &RoundedRect::from_rect(
                Rect::new(
                    layout_node.layout.location.x as f64,
                    layout_node.layout.location.y as f64,
                    (layout_node.layout.location.x + layout_node.layout.size.width) as f64,
                    (layout_node.layout.location.y + layout_node.layout.size.height) as f64,
                ),
                RoundedRectRadii::from_single_radius(5.0),
            ),
        );

        if checked {
            scene.fill(
                Fill::NonZero,
                Affine::default(),
                &Brush::Solid(color),
                None,
                &RoundedRect::from_rect(
                    Rect::new(
                        layout_node.layout.location.x as f64 + 5.0,
                        layout_node.layout.location.y as f64 + 5.0,
                        (layout_node.layout.location.x + layout_node.layout.size.width) as f64
                            - 5.0,
                        (layout_node.layout.location.y + layout_node.layout.size.height) as f64
                            - 5.0,
                    ),
                    RoundedRectRadii::from_single_radius(2.5),
                ),
            );
        }
    }

    fn layout_style(&mut self, state: &S) -> StyleNode {
        StyleNode {
            style: self.layout_style.get_ref(state).clone(),
            children: Vec::new(),
        }
    }

    fn update(&mut self, layout: &LayoutNode, state: &mut S, info: &AppInfo) -> Update {
        self.value.invalidate();
        self.layout_style.invalidate();

        let mut update = Update::empty();

        if let Some(cursor) = &info.cursor_pos {
            if cursor.x as f32 >= layout.layout.location.x
                && cursor.x as f32 <= layout.layout.location.x + layout.layout.size.width
                && cursor.y as f32 >= layout.layout.location.y
                && cursor.y as f32 <= layout.layout.location.y + layout.layout.size.height
            {
                for (_, btn, el) in &info.buttons {
                    match btn {
                        MouseButton::Left => {
                            if *el == ElementState::Released {
                                update |= (self.on_changed)(state);
                            }
                        },

                        _ => (),
                    }
                }
            }
        }

        update
    }

    fn widget_id(&mut self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Checkbox")
    }
}
