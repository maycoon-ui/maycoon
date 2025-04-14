use maycoon::color::{Blob, ImageFormat};
use maycoon::core::app::context::AppContext;
use maycoon::core::app::Application;
use maycoon::core::config::MayConfig;
use maycoon::core::signal::fixed::FixedSignal;
use maycoon::core::signal::MaybeSignal;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::image::{Image, ImageData};

const IMAGE_DATA: &[u8] = include_bytes!("../pelican.jpg");

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;

    fn build(context: AppContext) -> impl Widget {
        let image = context.use_signal(FixedSignal::new(ImageData::new(
            Blob::from(
                image::load_from_memory(IMAGE_DATA)
                    .unwrap()
                    .into_rgba8()
                    .to_vec(),
            ),
            ImageFormat::Rgba8,
            427,
            640,
        )));

        Image::new(MaybeSignal::signal(image))
    }

    fn config(&self) -> MayConfig<Self::Theme> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run()
}
