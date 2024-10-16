use maycoon::color::kurbo::{Affine, Circle, Point, Stroke};
use maycoon::color::{Brush, Color};
use maycoon::core::app::MayApp;
use maycoon::core::config::MayConfig;
use maycoon::macros::State;
use maycoon::widgets::canvas::Canvas;

#[derive(State)]
struct MyState;

fn main() {
    MayApp::new(MayConfig::default()).run(
        MyState,
        Canvas::new(|scene, _| {
            scene.stroke(
                &Stroke::new(10.0),
                Affine::default(),
                &Brush::Solid(Color::GREEN),
                None,
                &Circle::new(Point::new(100.0, 100.0), 50.0),
            );
        }),
    );
}
