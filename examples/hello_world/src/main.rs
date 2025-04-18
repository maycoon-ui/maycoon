use maycoon::core::app::context::AppContext;
use maycoon::core::app::Application;
use maycoon::core::config::MayConfig;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::text::Text;

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;

    fn build(_: AppContext) -> impl Widget {
        Text::new("Hello World".to_string())
    }

    fn config(&self) -> MayConfig<Self::Theme> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run()
}
