use femtovg::{Color, Paint, Path};
use taffy::{Layout, Style};

use crate::widget::{PathMode, Sketch, Widget};
use crate::widget::interaction::Action;

pub struct Rectangle {
    paint: Paint,
    children: Vec<Box<dyn Widget>>,
    style: Style,
}

impl Rectangle {
    pub fn new(paint: Paint) -> Self {
        Self {
            paint,
            children: vec![],
            style: Style::default(),
        }
    }
}

impl Widget for Rectangle {
    fn render(&mut self, layout: &Layout) -> Vec<Sketch> {
        let mut path = Path::new();
        path.rect(
            layout.location.x,
            layout.location.y,
            layout.size.width,
            layout.size.height,
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
        false
    }

    fn interact(&mut self, _actions: Vec<Action>) {}
}
