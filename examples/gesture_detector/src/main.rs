use maycoon::core::app::update::Update;
use maycoon::core::app::MayApp;
use maycoon::core::config::MayConfig;
use maycoon::macros::{val, State};
use maycoon::widgets::gesture_detector::GestureDetector;
use maycoon::widgets::text::Text;

#[derive(State)]
struct MyState {
    interaction: String,
}

fn main() {
    MayApp::new(MayConfig::default()).run(
        MyState {
            interaction: "None".to_string(),
        },
        GestureDetector::new(Text::new(val!(|state: &MyState| state.interaction.clone())))
            .with_on_hover(|state| {
                state.interaction = "Hovering".to_string();
                Update::DRAW
            })
            .with_on_release(|state| {
                state.interaction = "Released".to_string();
                Update::DRAW
            })
            .with_on_press(|state| {
                state.interaction = "Pressed".to_string();
                Update::DRAW
            }),
    );
}
