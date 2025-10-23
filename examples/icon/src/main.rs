use maycoon::core::app::Application;
use maycoon::core::app::context::AppContext;
use maycoon::core::config::MayConfig;
use maycoon::core::vgi::DefaultGraphics;
use maycoon::core::widget::Widget;
use maycoon::macros::svg_icon;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::icon::Icon;
use maycoon::widgets::icon::svg::SvgIcon;

struct MyApp;

impl<'a> Application<'a> for MyApp {
    type Theme = CelesteTheme;
    type Graphics = DefaultGraphics<'a>;
    type State = ();

    fn build(_: AppContext, _: Self::State) -> impl Widget {
        let icon: SvgIcon = svg_icon!("./assets/logo.svg");

        Icon::new(icon)
    }

    fn config(&self) -> MayConfig<'a, Self::Theme, Self::Graphics> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run(())
}
