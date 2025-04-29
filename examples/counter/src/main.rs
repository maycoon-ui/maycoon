use maycoon::core::app::context::AppContext;
use maycoon::core::app::update::Update;
use maycoon::core::app::Application;
use maycoon::core::config::MayConfig;
use maycoon::core::layout::{AlignItems, Dimension, FlexDirection, LayoutStyle};
use maycoon::core::reference::Ref;
use maycoon::core::signal::eval::EvalSignal;
use maycoon::core::signal::state::StateSignal;
use maycoon::core::signal::Signal;
use maycoon::core::widget::{Widget, WidgetLayoutExt};
use maycoon::math::Vector2;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::button::Button;
use maycoon::widgets::container::Container;
use maycoon::widgets::text::Text;

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;
    type State = ();

    fn build(context: AppContext, _: Self::State) -> impl Widget {
        let counter = context.use_signal(StateSignal::new(0));

        Container::new(vec![
            {
                let counter = counter.clone();

                Box::new(
                    Button::new(Text::new("Increase".to_string())).with_on_pressed(
                        EvalSignal::new(move || {
                            counter.set(*counter.get() + 1);

                            Update::DRAW
                        })
                        .hook(&context)
                        .maybe(),
                    ),
                )
            },
            {
                let counter = counter.clone();

                Box::new(
                    Button::new(Text::new("Decrease".to_string())).with_on_pressed(
                        EvalSignal::new(move || {
                            counter.set(*counter.get() - 1);

                            Update::DRAW
                        })
                        .hook(&context)
                        .maybe(),
                    ),
                )
            },
            Box::new(Text::new(counter.map(|i| Ref::Owned(i.to_string())))),
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
    MyApp.run(())
}
