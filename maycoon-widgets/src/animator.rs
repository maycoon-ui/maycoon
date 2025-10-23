use maycoon_core::app::context::AppContext;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, StyleNode};
use maycoon_core::vgi::Scene;
use maycoon_core::widget::Widget;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use std::time::{Duration, Instant};

/// A widget that animates another widget using an animation function.
pub struct Animator<W: Widget, A: Fn(&mut W, f32) -> Update> {
    start: Instant,
    duration: Duration,
    widget: W,
    animation: A,
}

impl<W: Widget, A: Fn(&mut W, f32) -> Update> Animator<W, A> {
    /// Creates a new animator widget with the given duration, child widget and animation function.
    ///
    /// The animation function is called with a value between `0.0` and `1.0` based on the elapsed time since the start of the animation
    /// and the total duration of the animation.
    pub fn new(duration: Duration, widget: W, animation: A) -> Self {
        Self {
            start: Instant::now(),
            duration,
            widget,
            animation,
        }
    }
}

impl<W: Widget, A: Fn(&mut W, f32) -> Update> Widget for Animator<W, A> {
    fn render(
        &mut self,
        scene: &mut dyn Scene,
        theme: &mut dyn Theme,
        layout_node: &LayoutNode,
        info: &AppInfo,
        context: AppContext,
    ) {
        self.widget.render(scene, theme, layout_node, info, context);
    }

    fn layout_style(&self) -> StyleNode {
        self.widget.layout_style()
    }

    fn update(&mut self, layout: &LayoutNode, context: AppContext, info: &AppInfo) -> Update {
        let elapsed = self.start.elapsed();

        let mut update = self.widget.update(layout, context, info);

        if elapsed < self.duration {
            let f = elapsed.as_secs_f32() / self.duration.as_secs_f32();

            update.insert((self.animation)(&mut self.widget, f));
        }

        update
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Animator")
    }
}
