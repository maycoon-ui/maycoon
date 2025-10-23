use maycoon_core::app::context::AppContext;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout;
use maycoon_core::layout::{Dimension, LayoutNode, LayoutStyle, LengthPercentageAuto, StyleNode};
use maycoon_core::signal::MaybeSignal;
use maycoon_core::vgi::kurbo::{Rect, RoundedRect, RoundedRectRadii, Stroke};
use maycoon_core::vgi::{Brush, Scene};
use maycoon_core::widget::{Widget, WidgetLayoutExt};
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
pub struct Checkbox {
    layout_style: MaybeSignal<LayoutStyle>,
    value: MaybeSignal<bool>,
    on_change: MaybeSignal<Update>,
}

impl Checkbox {
    /// Create a new checkbox with the given value.
    ///
    /// The value should be a signal, so it's mutable.
    pub fn new(value: impl Into<MaybeSignal<bool>>) -> Self {
        Self {
            layout_style: LayoutStyle {
                size: Vector2::<Dimension>::new(Dimension::length(20.0), Dimension::length(20.0)),
                margin: layout::Rect::<LengthPercentageAuto> {
                    left: LengthPercentageAuto::length(0.5),
                    right: LengthPercentageAuto::length(0.5),
                    top: LengthPercentageAuto::length(0.5),
                    bottom: LengthPercentageAuto::length(0.5),
                },
                ..Default::default()
            }
            .into(),
            value: value.into(),
            on_change: Update::empty().into(),
        }
    }

    /// Sets the value of the checkbox and returns itself.
    pub fn with_value(mut self, value: impl Into<MaybeSignal<bool>>) -> Self {
        self.value = value.into();
        self
    }

    /// Sets the update value to apply on changes.
    pub fn with_on_change(mut self, on_change: impl Into<MaybeSignal<Update>>) -> Self {
        self.on_change = on_change.into();
        self
    }
}

impl WidgetLayoutExt for Checkbox {
    fn set_layout_style(&mut self, layout_style: impl Into<MaybeSignal<LayoutStyle>>) {
        self.layout_style = layout_style.into();
    }
}

impl Widget for Checkbox {
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
            Some(&Stroke::new(3.0)),
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
            scene.draw_rounded_rect(
                &Brush::Solid(color),
                None,
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

    fn layout_style(&self) -> StyleNode {
        StyleNode {
            style: self.layout_style.get().clone(),
            children: Vec::new(),
        }
    }

    fn update(&mut self, layout: &LayoutNode, _: AppContext, info: &AppInfo) -> Update {
        let mut update = Update::empty();

        if let Some(cursor) = &info.cursor_pos
            && (cursor.x as f32 >= layout.layout.location.x
                && cursor.x as f32 <= layout.layout.location.x + layout.layout.size.width
                && cursor.y as f32 >= layout.layout.location.y
                && cursor.y as f32 <= layout.layout.location.y + layout.layout.size.height)
        {
            for (_, btn, el) in &info.buttons {
                if btn == &MouseButton::Left && *el == ElementState::Released {
                    update |= *self.on_change.get();
                    update |= Update::DRAW;

                    if let Some(sig) = self.value.as_signal() {
                        sig.set(!*sig.get());
                    }
                }
            }
        }

        update
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Checkbox")
    }
}
