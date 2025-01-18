use crate::counter::Counter;
use maycoon::core::app::MayApp;
use maycoon::core::component::Component;
use maycoon::core::config::MayConfig;
use maycoon::core::layout::{AlignItems, Dimension, FlexDirection, LayoutStyle};
use maycoon::macros::{val, State};
use maycoon::math::Vector2;
use maycoon::widgets::ext::WidgetLayoutExt;

mod counter;

#[derive(Default, State)]
struct MyState {
    count: i32,
}

fn main() {
    MayApp::new(MayConfig::default()).run(
        MyState::default(),
        Counter::new(
            val!(|state: &MyState| state.count),
            |state| state.count += 1,
            |state| state.count -= 1,
        )
        .with_layout_style(LayoutStyle {
            size: Vector2::<Dimension>::new(Dimension::Percent(1.0), Dimension::Percent(1.0)),
            flex_direction: FlexDirection::Column,
            align_items: Some(AlignItems::Center),
            ..Default::default()
        })
        .build(),
    );
}
