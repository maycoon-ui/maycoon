use maycoon::core::app::MayApp;
use maycoon::core::config::MayConfig;
use maycoon::macros::State;
use maycoon::widgets::text::Text;

#[derive(State)]
struct MyState;

fn main() {
    MayApp::new(MayConfig::default()).run(MyState, Text::new("Hello, World!".to_string()));
}
