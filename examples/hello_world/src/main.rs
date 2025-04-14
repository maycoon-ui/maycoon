use maycoon::core::app::context::AppContext;
use maycoon::core::app::Application;
use maycoon::core::config::MayConfig;
use maycoon::core::signal::state::StateSignal;
use maycoon::core::signal::MaybeSignal;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::text::Text;

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;

    fn build(context: AppContext) -> impl Widget {
        let value = context.use_signal(StateSignal::new("Hello World ".to_string()));

        Text::new(MaybeSignal::signal(value))
    }

    fn config(&self) -> MayConfig<Self::Theme> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run()
}
