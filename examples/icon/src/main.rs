use maycoon::core::app::MayApp;
use maycoon::core::config::MayConfig;
use maycoon::macros::{svg_icon, State};
use maycoon::widgets::icon::svg::SvgIcon;
use maycoon::widgets::icon::Icon;

#[derive(State)]
struct MyState;

fn main() {
    let icon: SvgIcon = svg_icon!("./assets/logo.svg");

    MayApp::new(MayConfig::default()).run(MyState, Icon::new(icon));
}
