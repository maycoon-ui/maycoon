use may_core::app::context::AppContext;
use may_core::app::page::Page;
use may_core::app::MayApp;
use may_core::config::AppConfig;
use may_core::widget::{DummyWidget, Widget};

struct MyPage;

impl Page for MyPage {
    fn init(&self, _: &mut AppContext) {}

    fn render(&self, _: &mut AppContext) -> Box<dyn Widget> {
        Box::new(DummyWidget::new())
    }
}

fn main() {
    MayApp::new(AppConfig::default(), MyPage).run();
}
