use maycoon::core::app::MayApp;
use maycoon::core::config::MayConfig;
use maycoon::core::vg::peniko;
use maycoon::macros::{val, State};
use maycoon::math::Vector2;
use maycoon::widgets::image::Image;

#[derive(State)]
struct MyState;

fn main() {
    let image_data = image::open("image/pelican.jpg")
        .unwrap()
        .into_rgba8()
        .to_vec();

    MayApp::new(MayConfig::default()).run(
        MyState,
        Image::new(val!(move |_: &MyState| (
            image_data.clone(),
            peniko::Format::Rgba8,
            Vector2::new(427, 640)
        ))),
    )
}
