use maycoon::core::app::Application;
use maycoon::core::app::context::AppContext;
use maycoon::core::config::MayConfig;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::text::Text;
use tracing_flame::FlameLayer;
use tracing_subscriber::filter::FilterFn;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, fmt};

const FILTER_TARGETS: [&str; 3] = ["wgpu", "naga", "winit"];

fn main() {
    let filter = FilterFn::new(|md| {
        for target in FILTER_TARGETS {
            if md.target().starts_with(target) {
                return false;
            }
        }

        true
    });

    let (flame, guard) = FlameLayer::with_file("tracing.folded").unwrap();

    tracing_subscriber::registry()
        .with(fmt::layer().compact().with_filter(filter.clone()))
        .with(flame.with_file_and_line(false).with_filter(filter))
        .init();

    MyApp.run(());

    guard.flush().unwrap();
}

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;
    type State = ();

    fn build(_: AppContext, _: Self::State) -> impl Widget {
        Text::new("Hello World!".to_string())
    }

    fn config(&self) -> MayConfig<Self::Theme> {
        MayConfig::default()
    }
}
