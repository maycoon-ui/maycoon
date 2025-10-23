use maycoon::core::app::Application;
use maycoon::core::app::context::AppContext;
use maycoon::core::app::update::Update;
use maycoon::core::config::MayConfig;
use maycoon::core::signal::Signal;
use maycoon::core::signal::eval::EvalSignal;
use maycoon::core::vgi::DefaultGraphics;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::gesture_detector::GestureDetector;
use maycoon::widgets::text::Text;

struct MyApp;

impl<'a> Application<'a> for MyApp {
    type Theme = CelesteTheme;
    type Graphics = DefaultGraphics<'a>;
    type State = ();

    fn build(context: AppContext, _: Self::State) -> impl Widget {
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

    fn config(&self) -> MayConfig<'a, Self::Theme, Self::Graphics> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run(())
}
