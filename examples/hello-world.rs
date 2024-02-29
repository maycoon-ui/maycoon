use may_core::app::context::AppContext;
use may_core::app::MayApp;
use may_core::app::page::Page;
use may_core::config::AppConfig;
use may_core::layout::{Dimension, Size, Style};
use may_core::vg::{Color, Paint};
use may_core::widget::Widget;
use may_core::widget::widgets::button::Button;

struct MyPage;

impl Page for MyPage {
    fn init(&self, ctx: &mut AppContext) {}

    fn render(&self, ctx: &mut AppContext) -> Box<dyn Widget> {
        Box::new(
            Button::new(),
        )
    }
}

fn main() {
    MayApp::new(
        AppConfig::default(),
        MyPage
    ).run();
}
