use maycoon::core::app::update::Update;
use maycoon::core::app::MayApp;
use maycoon::core::config::MayConfig;
use maycoon::core::layout::{AlignItems, Dimension, FlexDirection, LayoutStyle};
use maycoon::core::state::Val;
use maycoon::macros::{val, State};
use maycoon::math::Vector2;
use maycoon::widgets::checkbox::Checkbox;
use maycoon::widgets::container::Container;
use maycoon::widgets::text::Text;

#[derive(State)]
struct MyState {
    checked: bool,
}

fn main() {
    MayApp::new(MayConfig::default()).run(
        MyState { checked: false },
        Container::new(vec![
            Val::new_val(Box::new(
                Checkbox::new(val!(|state: &MyState| state.checked)).with_on_changed(|state| {
                    state.checked = !state.checked;
                    Update::DRAW
                }),
            )),
            Val::new_val(Box::new(Text::new(val!(
                |state: &MyState| if state.checked {
                    "Checked".to_string()
                } else {
                    "Unchecked".to_string()
                }
            )))),
        ])
        .with_layout_style(LayoutStyle {
            size: Vector2::<Dimension>::new(Dimension::Percent(1.0), Dimension::Percent(1.0)),
            flex_direction: FlexDirection::Column,
            align_items: Some(AlignItems::Center),
            ..Default::default()
        }),
    );
}
