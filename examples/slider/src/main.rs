use maycoon::core::app::context::AppContext;
use maycoon::core::app::Application;
use maycoon::core::config::MayConfig;
use maycoon::core::layout::{AlignItems, Dimension, FlexDirection, LayoutStyle};
use maycoon::core::reference::Ref;
use maycoon::core::signal::state::StateSignal;
use maycoon::core::signal::{MaybeSignal, Signal};
use maycoon::core::widget::{Widget, WidgetLayoutExt};
use maycoon::math::Vector2;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::container::Container;
use maycoon::widgets::slider::Slider;
use maycoon::widgets::text::Text;

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;

    fn build(context: AppContext) -> impl Widget {
        let value = context.use_signal(StateSignal::new(0.0f32));

        Container::new(vec![
            Box::new(Slider::new(MaybeSignal::signal(value.clone()))),
            Box::new(Text::new(value.map(|i| Ref::Owned(i.to_string())))),
        ])
        .with_layout_style(LayoutStyle {
            size: Vector2::<Dimension>::new(Dimension::percent(1.0), Dimension::percent(1.0)),
            flex_direction: FlexDirection::Column,
            align_items: Some(AlignItems::Center),
            ..Default::default()
        })
    }

    fn config(&self) -> MayConfig<Self::Theme> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run()
}
