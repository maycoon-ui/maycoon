use may_core::app::info::AppInfo;
use may_core::app::update::Update;
use may_core::app::MayApp;
use may_core::config::MayConfig;
use may_core::layout::{LayoutNode, StyleNode};
use may_core::state::State;
use may_core::vg::kurbo::{Circle, Point};
use may_core::vg::Scene;
use may_core::widget::Widget;
use may_theme::theme::celeste::CelesteTheme;
use may_theme::theme::Theme;
use may_widgets::text::Text;
use peniko::{Brush, Color, Fill};

struct MyState {}

impl State for MyState {}

fn main() {
    MayApp::new(MayConfig {
        theme: CelesteTheme::Light,
        window: Default::default(),
        render: Default::default(),
    })
    .run(Text::new("Hello"), MyState {});
}
