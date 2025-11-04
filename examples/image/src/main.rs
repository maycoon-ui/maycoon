use maycoon::color::{Blob, ImageAlphaType, ImageData, ImageFormat};
use maycoon::core::app::Application;
use maycoon::core::app::context::AppContext;
use maycoon::core::config::MayConfig;
use maycoon::core::vgi::DefaultGraphics;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::image::Image;

const IMAGE_DATA: &[u8] = include_bytes!("../pelican.jpg");

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;
    type Graphics = DefaultGraphics;
    type State = ();

    fn build(_: AppContext, _: Self::State) -> impl Widget {
        Image::new(ImageData {
            data: Blob::from(
                image::load_from_memory(IMAGE_DATA)
                    .unwrap()
                    .into_rgba8()
                    .to_vec(),
            ),
            format: ImageFormat::Rgba8,
            alpha_type: ImageAlphaType::Alpha,
            width: 427,
            height: 640,
        })
    }

    fn config(&self) -> MayConfig<Self::Theme, Self::Graphics> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run(())
}
