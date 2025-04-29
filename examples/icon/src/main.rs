use maycoon::core::app::context::AppContext;
use maycoon::core::app::Application;
use maycoon::core::config::MayConfig;
use maycoon::core::widget::Widget;
use maycoon::macros::svg_icon;
use maycoon::theme::theme::celeste::CelesteTheme;
use maycoon::widgets::icon::svg::SvgIcon;
use maycoon::widgets::icon::Icon;

struct MyApp;

impl Application for MyApp {
    type Theme = CelesteTheme;
    type State = ();

    fn build(_: AppContext, _: Self::State) -> impl Widget {
        let icon: SvgIcon = svg_icon!("./assets/logo.svg");

        Icon::new(icon)
    }

    fn config(&self) -> MayConfig<Self::Theme> {
        MayConfig::default()
    }
}

fn main() {
    MyApp.run(())
}
