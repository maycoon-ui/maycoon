use femtovg::{Paint, Path};

use may_core::layout::{Dimension, Layout, Size, Style};
use may_core::widget::interaction::InteractionInfo;
use may_core::widget::update::Update;
use may_core::widget::{PathMode, Sketch, Widget};
use may_theme::scheme::SchemeValue;
use may_theme::theme::{Theme, WidgetType};

pub struct Button {
    children: Vec<Box<dyn Widget>>,
    style: Style,
    hover: bool,
}

impl Button {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self {
            children: vec![child],
            style: Style {
                size: Size::<Dimension> {
                    width: Dimension::Length(100.0),
                    height: Dimension::Length(60.0),
                },
                ..Default::default()
            },
            hover: false,
        }
    }

    pub fn id() -> String {
        String::from("may-widgets:Button")
    }
}

impl Widget for Button {
    fn render(&mut self, layout: &Layout, theme: &Box<dyn Theme>) -> Vec<Sketch> {
        let scheme = theme
            .scheme_of(Button::id())
            .unwrap_or(theme.default_scheme_of(WidgetType::Interactive));

        let mut rect = Path::new();
        rect.rounded_rect(
            layout.location.x,
            layout.location.y,
            layout.size.width,
            layout.size.height,
            scheme
                .custom
                .get(&String::from("radius"))
                .unwrap_or(&SchemeValue::Float(10.0))
                .as_float()
                .unwrap(),
        );

        vec![Sketch::Path(
            rect,
            Paint::color(scheme.primary_background_color),
            PathMode::Fill,
        )]
    }

    fn update(&mut self, info: &InteractionInfo, layout: &Layout) -> Update {
        Update::empty()
    }

    fn children(&self) -> &Vec<Box<dyn Widget>> {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn Widget>> {
        &mut self.children
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}
