use may_core::app::MayApp;
use may_core::config::AppConfig;
use may_core::state::State;
use may_widgets::button::Button;
use may_widgets::text::Text;

struct MyState {}

impl State for MyState {}

fn main() {
    MayApp::new(AppConfig::default()).run(Button::new(Text::new("Hello World!")), MyState {});
}
