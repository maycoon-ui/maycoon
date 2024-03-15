use may_core::app::context::AppContext;
use may_core::app::page::Page;
use may_core::app::MayApp;
use may_core::config::AppConfig;
use may_core::widget::{DummyWidget, Widget};
use may_widgets::button::Button;
use may_widgets::text::Text;

struct MyPage {
    count: i32,
}

impl Page for MyPage {
    fn init(&self, _: &mut AppContext) {}

    fn render(&mut self, ctx: &mut AppContext) -> Box<dyn Widget> {
        Box::new(Button::new(
            Text::new(self.count)
        ).on_press(|| {
        }))
    }
}

fn main() {
    MayApp::new(AppConfig::default(), MyPage { count: 0 }).run();
}
