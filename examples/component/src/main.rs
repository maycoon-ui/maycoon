use crate::counter::Counter;
use maycoon::core::app::Application;
use maycoon::core::app::context::AppContext;
use maycoon::core::config::MayConfig;
use maycoon::core::layout::{AlignItems, Dimension, FlexDirection, LayoutStyle};
use maycoon::core::signal::state::StateSignal;
use maycoon::core::vgi::DefaultGraphics;
use maycoon::core::widget::{Widget, WidgetLayoutExt};
use maycoon::math::Vector2;
use maycoon::theme::theme::celeste::CelesteTheme;

mod counter;

struct MyApp;

impl<'a> Application<'a> for MyApp {
    type Theme = CelesteTheme;
    type Graphics = DefaultGraphics<'a>;
    type State = ();

    fn build(context: AppContext, _: Self::State) -> impl Widget {
        let counter = context.use_signal(StateSignal::new(0));

        Counter::new(counter).with_layout_style(LayoutStyle {
            size: Vector2::<Dimension>::new(Dimension::percent(1.0), Dimension::percent(1.0)),
            flex_direction: FlexDirection::Column,
            align_items: Some(AlignItems::Center),
            ..Default::default()
        })
    }

    fn config(&self) -> MayConfig<'a, Self::Theme, Self::Graphics> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run(())
}
