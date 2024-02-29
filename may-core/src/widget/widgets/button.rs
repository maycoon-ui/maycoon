use crate::widget::interaction::Action;
use crate::widget::{PathMode, Sketch, Widget};
use femtovg::{Color, Paint, Path};
use may_theme::theme::Theme;
use taffy::{Dimension, Layout, Size, Style};

pub struct Button {
    children: Vec<Box<dyn Widget>>,
    style: Style,
    paint: Paint,
}

impl Button {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: Style {
                size: Size::<Dimension> {
                    width: Dimension::Percent(0.1),
                    height: Dimension::Percent(0.1),
                },
                ..Style::default()
            },
            paint: Paint::color(Color::rgb(95, 90, 120)),
        }
    }
}

impl Widget for Button {
    fn render(&mut self, layout: &Layout) -> Vec<Sketch> {
        let mut path = Path::new();
        path.rounded_rect(
            layout.location.x,
            layout.location.y,
            layout.size.width,
            layout.size.height,
            1.0,
        );

        vec![Sketch::Path(path, self.paint.clone(), PathMode::Fill)]
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

    fn interactive(&self) -> bool {
        true
    }

    fn interact(&mut self, actions: Vec<Action>) {
        for action in actions {
            match action {
                Action::Hover => {
                    self.paint.set_color(Color::rgb(65, 60, 120));
                }

                _ => {}
            }
        }
    }

    fn apply_theme(&mut self, theme: Box<dyn Theme>) {}
}
