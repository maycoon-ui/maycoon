use crate::plugin::MyPlugin;
use maycoon::core::app::context::AppContext;
use maycoon::core::app::Application;
use maycoon::core::config::MayConfig;
use maycoon::core::plugin::PluginManager;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::text::Text;

pub mod plugin;

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;

    fn build(_: AppContext) -> impl Widget {
        Text::new("Drop a file!".to_string())
    }

    fn config(&self) -> MayConfig<Self::Theme> {
        MayConfig::default()
    }

    fn plugins(&self) -> PluginManager<Self::Theme> {
        let mut plugins = PluginManager::new();

        plugins.register(MyPlugin);

        plugins
    }
}

fn main() {
    MyApp.run()
}
