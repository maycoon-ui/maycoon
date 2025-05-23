use maycoon::color::{Blob, ImageFormat};
use maycoon::core::app::context::AppContext;
use maycoon::core::app::Application;
use maycoon::core::config::MayConfig;
use maycoon::core::signal::fixed::FixedSignal;
use maycoon::core::signal::Signal;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::image::{Image, ImageData};

const IMAGE_DATA: &[u8] = include_bytes!("../pelican.jpg");

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;
    type State = ();

    fn build(context: AppContext, _: Self::State) -> impl Widget {
        let image = FixedSignal::new(ImageData::new(
            Blob::from(
                image::load_from_memory(IMAGE_DATA)
                    .unwrap()
                    .into_rgba8()
                    .to_vec(),
            ),
            ImageFormat::Rgba8,
            427,
            640,
        ))
        .hook(&context);

        Image::new(image.maybe())
    }

    fn config(&self) -> MayConfig<Self::Theme> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run(())
}
