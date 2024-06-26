//! The classic counter app example.

use may_core::app::MayApp;
use may_core::app::update::Update;
use may_core::config::MayConfig;
use may_core::layout::{
    AlignItems, FlexDirection
    , LayoutStyle
    ,
};
use may_macro::State;
use may_widgets::button::Button;
use may_widgets::container::Container;
use may_widgets::text::Text;

#[derive(Default, State)]
struct MyState {
    count: i32,
}

fn main() {
    MayApp::new(MayConfig::default()).run(
        |state| {
            Container::new(vec![
                Box::new(
                    Button::new(Text::new("increase")).with_on_pressed(|s: &mut MyState| {
                        s.count += 1;
                        Update::DRAW
                    }),
                ),
                Box::new(Text::new(state.count)),
            ])
            .with_layout_style(LayoutStyle {
                align_items: Some(AlignItems::Center),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            })
        },
        MyState::default(),
    );
}
