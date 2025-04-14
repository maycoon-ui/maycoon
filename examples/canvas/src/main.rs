use maycoon::color::color::palette;
use maycoon::color::kurbo::{Affine, Circle, Point, Stroke};
use maycoon::color::Brush;
use maycoon::core::app::context::AppContext;
use maycoon::core::app::Application;
use maycoon::core::config::MayConfig;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::canvas::Canvas;

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;

    fn build(_: AppContext) -> impl Widget {
        Canvas::new(|scene, _| {
            scene.stroke(
                &Stroke::new(10.0),
                Affine::default(),
                &Brush::Solid(palette::css::GREEN),
                None,
                &Circle::new(Point::new(100.0, 100.0), 50.0),
            );
        })
    }

    fn config(&self) -> MayConfig<Self::Theme> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run()
}
