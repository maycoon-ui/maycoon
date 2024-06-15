use may_core::app::MayApp;
use may_core::config::MayConfig;
use may_core::state::State;
use may_theme::theme::celeste::CelesteTheme;
use may_widgets::button::Button;
use may_widgets::text::Text;

struct MyState {}

impl State for MyState {}

fn main() {
    MayApp::new(MayConfig {
        theme: CelesteTheme::light(),
        window: Default::default(),
        render: Default::default(),
    })
    .run(Button::new(Text::new("Hello")), MyState {});
}
