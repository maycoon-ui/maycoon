use maycoon_core::app::context::AppContext;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout;
use maycoon_core::layout::{Dimension, LayoutNode, LayoutStyle, LengthPercentageAuto, StyleNode};
use maycoon_core::signal::MaybeSignal;
use maycoon_core::vgi::kurbo::{Circle, Point, Rect, RoundedRect, RoundedRectRadii, Stroke};
use maycoon_core::vgi::{Brush, Scene};
use maycoon_core::widget::{Widget, WidgetLayoutExt};
use maycoon_core::window::{ElementState, MouseButton};
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use nalgebra::Vector2;

/// A switch widget to toggle between two states.
///
/// Similar to a checkbox, but with slightly different visuals.
///
/// See the [switch](https://github.com/maycoon-ui/maycoon/blob/master/examples/switch/src/main.rs) example for how to use it in practice.
///
/// ### Theming
/// Styling the checkbox requires following properties:
/// - `color_unchecked` -  The color of the switch, when it's not checked (inner value is false).
/// - `color_checked` - The color of the switch, when it's checked (inner value is true).
///
/// The [WidgetId] is equal to `maycoon-widgets:Switch`.
pub struct Switch {
    layout: MaybeSignal<LayoutStyle>,
    value: MaybeSignal<bool>,
    on_change: MaybeSignal<Update>,
}

impl Switch {
    /// Create a new switch with the given value.
    ///
    /// The value should be a signal, so it's mutable.
    #[inline(always)]
    pub fn new(value: impl Into<MaybeSignal<bool>>) -> Self {
        Self {
            layout: LayoutStyle {
                size: Vector2::new(Dimension::length(60.0), Dimension::length(30.0)),
                margin: layout::Rect::<LengthPercentageAuto> {
                    left: LengthPercentageAuto::length(2.5),
                    right: LengthPercentageAuto::length(2.5),
                    top: LengthPercentageAuto::length(2.5),
                    bottom: LengthPercentageAuto::length(2.5),
                },
                ..Default::default()
            }
            .into(),
            value: value.into(),
            on_change: Update::empty().into(),
        }
    }

    /// Sets the value of the checkbox and returns self.
    #[inline(always)]
    pub fn with_value(mut self, value: impl Into<MaybeSignal<bool>>) -> Self {
        self.value = value.into();
        self
    }

    /// Sets the update value to apply on changes and returns self.
    #[inline(always)]
    pub fn with_on_change(mut self, update: impl Into<MaybeSignal<Update>>) -> Self {
        self.on_change = update.into();
        self
    }
}

impl WidgetLayoutExt for Switch {
    #[inline(always)]
    fn set_layout_style(&mut self, layout_style: impl Into<MaybeSignal<LayoutStyle>>) {
        self.layout = layout_style.into();
    }
}

impl Widget for Switch {
    fn render(
        &mut self,
        scene: &mut dyn Scene,
        theme: &mut dyn Theme,
        layout_node: &LayoutNode,
        _: &AppInfo,
        _: AppContext,
    ) {
        let checked = *self.value.get();

        let color = if let Some(style) = theme.of(self.widget_id()) {
            if checked {
                style.get_color("color_checked").unwrap()
            } else {
                style.get_color("color_unchecked").unwrap()
            }
        } else if checked {
            theme.defaults().interactive().active()
        } else {
            theme.defaults().interactive().inactive()
        };

        scene.draw_rounded_rect(
            &Brush::Solid(color),
            None,
            Some(&Stroke::new(5.0)),
            &RoundedRect::from_rect(
                Rect::new(
                    layout_node.layout.location.x as f64,
                    layout_node.layout.location.y as f64,
                    (layout_node.layout.location.x + layout_node.layout.size.width) as f64,
                    (layout_node.layout.location.y + layout_node.layout.size.height) as f64,
                ),
                RoundedRectRadii::from_single_radius(60.0),
            ),
        );

        let offset = if checked { 15.0 } else { -15.0 };

        scene.draw_circle(
            &Brush::Solid(color),
            None,
            None,
            &Circle::new(
                Point::new(
                    (layout_node.layout.location.x + (layout_node.layout.size.width / 2.0)) as f64
                        + offset,
                    (layout_node.layout.location.y + (layout_node.layout.size.height / 2.0)) as f64,
                ),
                10.0,
            ),
        );
    }

    #[inline(always)]
    fn layout_style(&self) -> StyleNode {
        StyleNode {
            style: self.layout.get().clone(),
            children: Vec::new(),
        }
    }

    fn update(&mut self, layout: &LayoutNode, _: AppContext, info: &AppInfo) -> Update {
        let mut update = Update::empty();

        if let Some(cursor) = info.cursor_pos
            && layout::intersects(cursor, &layout.layout)
        {
            for (_, btn, el) in &info.buttons {
                if btn == &MouseButton::Left && *el == ElementState::Released {
                    update |= *self.on_change.get();
                    update |= Update::DRAW;

                    if let Some(sig) = self.value.as_signal() {
                        let checked = *sig.get();
                        sig.set(!checked);
                    }
                }
            }
        }

        update
    }

    #[inline(always)]
    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Switch")
    }
}
