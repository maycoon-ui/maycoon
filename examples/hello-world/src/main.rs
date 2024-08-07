use maycoon::core::app::MayApp;
use maycoon::core::config::MayConfig;
use maycoon::macros::{val, State};
use maycoon::widgets::text::Text;

#[derive(State)]
struct MyState;

fn main() {
    MayApp::new(MayConfig::default()).run(MyState, Text::new(val!("Hello, World!")));
}
