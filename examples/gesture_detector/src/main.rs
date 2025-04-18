use maycoon::core::app::context::AppContext;
use maycoon::core::app::update::Update;
use maycoon::core::app::Application;
use maycoon::core::config::MayConfig;
use maycoon::core::signal::eval::EvalSignal;
use maycoon::core::signal::Signal;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::gesture_detector::GestureDetector;
use maycoon::widgets::text::Text;

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;

    fn build(context: AppContext) -> impl Widget {
        GestureDetector::new(Text::new("Gesture Detector".to_string()))
            .with_on_hover(
                EvalSignal::new(move || {
                    println!("Hovered");
                    Update::DRAW
                })
                .hook(&context)
                .maybe(),
            )
            .with_on_release(
                EvalSignal::new(move || {
                    println!("Release");
                    Update::DRAW
                })
                .hook(&context)
                .maybe(),
            )
            .with_on_press(
                EvalSignal::new(move || {
                    println!("Press");
                    Update::DRAW
                })
                .hook(&context)
                .maybe(),
            )
    }

    fn config(&self) -> MayConfig<Self::Theme> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run()
}
