use crate::plugin::MyPlugin;
use maycoon::core::app::Application;
use maycoon::core::app::context::AppContext;
use maycoon::core::config::MayConfig;
use maycoon::core::plugin::PluginManager;
use maycoon::core::vgi::DefaultGraphics;
use maycoon::core::widget::Widget;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::text::Text;

pub mod plugin;

fn main() {
    MyApp.run(())
}

struct MyApp;

impl<'a> Application<'a> for MyApp {
    type Theme = CelesteTheme;
    type Graphics = DefaultGraphics<'a>;
    type State = ();

    fn build(_: AppContext, _: Self::State) -> impl Widget {
        Text::new("Drop a file!".to_string())
    }

    fn config(&self) -> MayConfig<'a, Self::Theme, Self::Graphics> {
        MayConfig::default()
    }

    fn plugins(&self) -> PluginManager<'a, Self::Theme, Self::Graphics> {
        let mut plugins = PluginManager::new();

        plugins.register(MyPlugin);

        plugins
    }
}
