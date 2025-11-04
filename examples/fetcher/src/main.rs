use maycoon::core::app::Application;
use maycoon::core::app::context::AppContext;
use maycoon::core::app::update::Update;
use maycoon::core::config::MayConfig;
use maycoon::core::vgi::DefaultGraphics;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::fetcher::WidgetFetcher;
use maycoon::widgets::text::Text;
use serde::Deserialize;

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;
    type Graphics = DefaultGraphics;
    type State = ();

    fn build(_: AppContext, _: Self::State) -> impl Widget {
        WidgetFetcher::new(get_random_quote(), Update::DRAW, |data| {
            if let Some(data) = data {
                Text::new(format!(" \"{}\" \n - {}", data.quote, data.author))
            } else {
                Text::new(" Loading Quote...".to_string())
            }
        })
    }

    fn config(&self) -> MayConfig<Self::Theme, Self::Graphics> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run(())
}

#[derive(Deserialize)]
struct Quote {
    quote: String,
    author: String,
}

async fn get_random_quote() -> Quote {
    surf::get("https://dummyjson.com/quotes/random")
        .await
        .expect("Failed to fetch quote")
        .body_json::<Quote>()
        .await
        .expect("Failed to parse quote")
}
