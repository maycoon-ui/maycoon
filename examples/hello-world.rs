//! Demonstrates how to render a simple text widget.

use may_core::app::MayApp;
use may_core::config::MayConfig;
use may_macro::State;
use may_theme::theme::celeste::CelesteTheme;
use may_widgets::text::Text;

#[derive(State)]
struct MyState;

fn main() {
    MayApp::new(MayConfig {
        theme: CelesteTheme::light(),
        window: Default::default(),
        render: Default::default(),
    })
    .run(|_| Text::new("Hello World!"), MyState {});
}
