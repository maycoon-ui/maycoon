use crate::ext::WidgetLayoutExt;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout;
use maycoon_core::layout::{Dimension, LayoutNode, LayoutStyle, LengthPercentageAuto, StyleNode};
use maycoon_core::state::{State, Val};
use maycoon_core::vg::kurbo::{Affine, Circle, Point, Rect, RoundedRect, RoundedRectRadii};
use maycoon_core::vg::peniko::{Brush, Fill};
use maycoon_core::vg::Scene;
use maycoon_core::widget::Widget;
use maycoon_core::window::MouseButton;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use nalgebra::Vector2;

/// A slider widget to control a floating point value between `0.0` and `1.0`.
///
/// ### Theming
/// You can style the slider using following properties:
/// - `color` - The color of the slider bar.
/// - `color_ball` - The color of the slider ball.
pub struct Slider<S: State> {
    layout_style: Val<S, LayoutStyle>,
    value: Val<S, f32>,
    on_change: Box<dyn FnMut(&mut S, f32) -> Update>,
    dragging: bool,
}

impl<S: State> Slider<S> {
    /// Create a new Slider widget from a value (should be state bound) and an `on_change` callback.
    pub fn new(
        value: impl Into<Val<S, f32>>,
        on_change: impl FnMut(&mut S, f32) -> Update + 'static,
    ) -> Self {
        Self {
            layout_style: Val::new_val(LayoutStyle {
                size: Vector2::<Dimension>::new(Dimension::Length(100.0), Dimension::Length(10.0)),
                margin: layout::Rect::<LengthPercentageAuto> {
                    left: LengthPercentageAuto::Length(10.0),
                    right: LengthPercentageAuto::Length(0.0),
                    top: LengthPercentageAuto::Length(10.0),
                    bottom: LengthPercentageAuto::Length(10.0),
                },
                ..Default::default()
            }),
            value: value.into(),
            on_change: Box::new(on_change),
            dragging: false,
        }
    }

    /// Sets the layout style of the slider and returns itself.
    pub fn with_value(mut self, value: impl Into<Val<S, f32>>) -> Self {
        self.value = value.into();
        self
    }

    /// Sets the function to be called when the slider is clicked/changed.
    pub fn with_on_change(
        mut self,
        on_change: impl FnMut(&mut S, f32) -> Update + 'static,
    ) -> Self {
        self.on_change = Box::new(on_change);
        self
    }
}

impl<S: State> WidgetLayoutExt<S> for Slider<S> {
    fn set_layout_style(&mut self, layout_style: impl Into<Val<S, LayoutStyle>>) {
        self.layout_style = layout_style.into();
    }
}

impl<S: State> Widget<S> for Slider<S> {
    fn render(
        &mut self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        _: &AppInfo,
        layout_node: &LayoutNode,
        state: &S,
    ) {
        let value = *self.value.get_ref(state);

        let brush = if let Some(style) = theme.of(self.widget_id()) {
            Brush::Solid(style.get_color("color").unwrap())
        } else {
            Brush::Solid(theme.defaults().interactive().inactive())
        };

        let ball_brush = if let Some(style) = theme.of(self.widget_id()) {
            Brush::Solid(style.get_color("color_ball").unwrap())
        } else {
            Brush::Solid(theme.defaults().interactive().active())
        };

        let circle_radius = layout_node.layout.size.height as f64 / 1.15;

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
                RoundedRectRadii::from_single_radius(20.0),
            ),
        );

        scene.fill(
            Fill::NonZero,
            Affine::default(),
            &ball_brush,
            None,
            &Circle::new(
                Point::new(
                    (layout_node.layout.location.x + layout_node.layout.size.width * value) as f64,
                    (layout_node.layout.location.y + layout_node.layout.size.height / 2.0) as f64,
                ),
                circle_radius,
            ),
        );
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

        if let Some(cursor) = info.cursor_pos {
            if cursor.x as f32 >= layout.layout.location.x
                && cursor.x as f32 <= layout.layout.location.x + layout.layout.size.width
                && cursor.y as f32 >= layout.layout.location.y
                && cursor.y as f32 <= layout.layout.location.y + layout.layout.size.height
            {
                for (_, btn, el_state) in &info.buttons {
                    if btn == &MouseButton::Left && el_state.is_pressed() {
                        self.dragging = el_state.is_pressed();
                    }
                }

                if self.dragging {
                    let new_value =
                        (cursor.x as f32 - layout.layout.location.x) / layout.layout.size.width;

                    update.insert((self.on_change)(state, new_value));
                    update.insert(Update::DRAW);
                }
            }
        } else {
            self.dragging = false;
        }

        update
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Slider")
    }
}
