use maycoon::core::app::context::AppContext;
use maycoon::core::app::update::Update;
use maycoon::core::app::Application;
use maycoon::core::config::MayConfig;
use maycoon::core::signal::Signal;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::animator::Animator;
use maycoon::widgets::text::Text;
use std::time::Duration;

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;

    fn build(context: AppContext) -> impl Widget {
        let font_size = context.use_state(0.0);

        Animator::new(
            Duration::from_millis(2000),
            Text::new("Hello World!".to_string()).with_font_size(font_size.maybe()),
            move |_, f| {
                font_size.set(f * 30.0);

                Update::DRAW
            },
        )
    }

    fn config(&self) -> MayConfig<Self::Theme> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run()
}
