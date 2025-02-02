use maycoon::core::app::MayApp;
use maycoon::core::config::MayConfig;
use maycoon::macros::{val, State};
use maycoon::widgets::icon::{Icon, SvgIcon};

const ICON_SOURCE: &str = include_str!("../../../assets/logo.svg");

#[derive(State)]
struct MyState;

fn main() {
    MayApp::new(MayConfig::default()).run(
        MyState,
        Icon::new(val!(SvgIcon::new(ICON_SOURCE).expect("Failed to load icon"))),
    );
}
