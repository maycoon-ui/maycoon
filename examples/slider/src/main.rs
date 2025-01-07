use maycoon::core::app::update::Update;
use maycoon::core::app::MayApp;
use maycoon::core::config::MayConfig;
use maycoon::core::layout::{AlignItems, Dimension, FlexDirection, LayoutStyle};
use maycoon::core::state::Val;
use maycoon::macros::{val, State};
use maycoon::math::Vector2;
use maycoon::widgets::container::Container;
use maycoon::widgets::ext::WidgetLayoutExt;
use maycoon::widgets::slider::Slider;
use maycoon::widgets::text::Text;

#[derive(State)]
struct MyState {
    value: f32,
}

fn main() {
    MayApp::new(MayConfig::default()).run(
        MyState { value: 0.0 },
        Container::new(vec![
            Val::new_val(Box::new(Slider::new(
                val!(|state: &MyState| state.value),
                |state, new| {
                    state.value = new;
                    Update::DRAW
                },
            ))),
            Val::new_val(Box::new(Text::new(val!(|state: &MyState| state
                .value
                .to_string())))),
        ])
        .with_layout_style(LayoutStyle {
            size: Vector2::<Dimension>::new(Dimension::Percent(1.0), Dimension::Percent(1.0)),
            flex_direction: FlexDirection::Column,
            align_items: Some(AlignItems::Center),
            ..Default::default()
        }),
    );
}
