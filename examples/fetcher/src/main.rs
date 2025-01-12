use maycoon::core::app::update::Update;
use maycoon::core::app::MayApp;
use maycoon::core::config::{MayConfig, TasksConfig, WindowConfig};
use maycoon::macros::State;
use maycoon::math::Vector2;
use maycoon::widgets::fetcher::WidgetFetcher;
use maycoon::widgets::text::Text;
use serde::Deserialize;

#[derive(State)]
struct MyState;

fn main() {
    MayApp::new(MayConfig {
        window: WindowConfig {
            // Make the window size larger to fit the quote
            size: Vector2::new(1200.0, 600.0),
            title: "Maycoon Fetcher Example".to_string(),
            ..Default::default()
        },
        // IMPORTANT: Enable the task runner
        tasks: Some(TasksConfig::default()),
        ..Default::default()
    })
    .run(
        MyState,
        WidgetFetcher::new(get_random_quote(), Update::DRAW, |data, _| {
            if let Some(data) = data {
                Text::new(format!(" \"{}\" \n - {}", data.quote, data.author))
            } else {
                Text::new(" Loading Quote...".to_string())
            }
        }),
    );
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
