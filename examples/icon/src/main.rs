use maycoon::core::app::MayApp;
use maycoon::core::config::MayConfig;
use maycoon::macros::{svg_icon, State};
use maycoon::widgets::icon::Icon;

#[derive(State)]
struct MyState;

fn main() {
    MayApp::new(MayConfig::default()).run(MyState, Icon::new(svg_icon!("./assets/logo.svg")));
}
