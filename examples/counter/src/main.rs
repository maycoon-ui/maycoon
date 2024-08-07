use maycoon::core::app::update::Update;
use maycoon::core::app::MayApp;
use maycoon::core::config::MayConfig;
use maycoon::core::layout::{AlignItems, Dimension, Display, FlexDirection, LayoutStyle};
use maycoon::macros::{val, State};
use maycoon::math::Vector2;
use maycoon::widgets::button::Button;
use maycoon::widgets::container::Container;
use maycoon::widgets::text::Text;

#[derive(State)]
struct MyState {
    count: i32,
}

fn main() {
    MayApp::new(MayConfig::default()).run(
        MyState { count: 0 },
        Container::new(vec![
            Box::new(Button::new(Text::new(val!("Increase"))).with_on_pressed(
                |state: &mut MyState| {
                    state.count += 1;
                    Update::DRAW
                },
            )),
            Box::new(Button::new(Text::new(val!("Decrease"))).with_on_pressed(
                |state: &mut MyState| {
                    state.count -= 1;
                    Update::DRAW
                },
            )),
            Box::new(Text::new(val!(|state: &MyState| state.count))),
        ])
        .with_layout_style(LayoutStyle {
            size: Vector2::<Dimension>::new(Dimension::Percent(1.0), Dimension::Percent(1.0)),
            flex_direction: FlexDirection::Column,
            align_items: Some(AlignItems::Center),
            ..Default::default()
        }),
    );
}
