use maycoon::color::Brush;
use maycoon::color::color::palette;
use maycoon::color::kurbo::{Circle, Point, Stroke};
use maycoon::core::app::Application;
use maycoon::core::app::context::AppContext;
use maycoon::core::config::MayConfig;
use maycoon::core::vgi::DefaultGraphics;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::canvas::Canvas;

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;
    type Graphics = DefaultGraphics;
    type State = ();

    fn build(_: AppContext, _: Self::State) -> impl Widget {
        Canvas::new(|scene, _| {
            scene.draw_circle(
                &Brush::Solid(palette::css::GREEN),
                None,
                Some(&Stroke::new(10.0)),
                &Circle::new(Point::new(100.0, 100.0), 50.0),
            );
        })
    }

    fn config(&self) -> MayConfig<Self::Theme, Self::Graphics> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run(())
}
