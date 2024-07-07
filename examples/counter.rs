//! The classic counter app example.

use may_core::app::MayApp;
use may_core::app::update::Update;
use may_core::config::MayConfig;
<<<<<<< Updated upstream
use may_core::layout::{
    AlignItems, FlexDirection
    , LayoutStyle
    ,
};
use may_macro::State;
=======
use may_core::layout::{AlignItems, Dimension, FlexDirection, LayoutStyle, Size};
use may_macro::{val, State};
>>>>>>> Stashed changes
use may_widgets::button::Button;
use may_widgets::container::Container;
use may_widgets::text::Text;

#[derive(Default, State)]
struct MyState {
    count: i32,
}

fn main() {
<<<<<<< Updated upstream
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
=======
    MayApp::new(MayConfig::default()).run::<MyState, _>(
        Container::new(vec![
            Box::new(Button::new(Text::new(val!("Increase"))).with_on_pressed(
                |s: &mut MyState| {
                    s.count += 1;
                    Update::DRAW
                },
            )),
            Box::new(Button::new(Text::new(val!("Decrease"))).with_on_pressed(
                |s: &mut MyState| {
                    s.count -= 1;
                    Update::DRAW
                },
            )),
            Box::new(Text::new(val!(|state: &MyState| {
                state.count.to_string()
            }))),
        ])
        .with_layout_style(LayoutStyle {
            align_items: Some(AlignItems::Center),
            flex_direction: FlexDirection::Column,
            size: Size::<Dimension> {
                width: Dimension::Percent(1.0), // 1.0 = 100 %
                height: Dimension::Auto,
            },
            ..Default::default()
        }),
>>>>>>> Stashed changes
        MyState::default(),
    );
}
