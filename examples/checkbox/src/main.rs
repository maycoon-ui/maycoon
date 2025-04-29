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
use maycoon::widgets::checkbox::Checkbox;
use maycoon::widgets::container::Container;
use maycoon::widgets::text::Text;

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;
    type State = ();

    fn build(context: AppContext, _: Self::State) -> impl Widget {
        let checked = context.use_signal(StateSignal::new(false));

        Container::new(vec![
            {
                let checked = checked.clone();

                Box::new(Checkbox::new(MaybeSignal::signal(checked)))
            },
            {
                let checked = checked.clone();

                Box::new(Text::new(checked.map(|val| Ref::Owned(val.to_string()))))
            },
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
