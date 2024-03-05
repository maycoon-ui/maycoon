use femtovg::{Paint, Path};
use mint::Vector2;
use taffy::{Layout, Style};

use may_theme::theme::Theme;

use crate::widget::interaction::InteractionInfo;
use crate::widget::update::UpdateMode;

pub mod interaction;
pub mod update;

pub trait Widget {
    fn render(&mut self, layout: &Layout, theme: &Box<dyn Theme>) -> Vec<Sketch>;
    fn update(&mut self, info: &InteractionInfo, update: &mut UpdateMode);
    fn children(&self) -> &Vec<Box<dyn Widget>>;
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Widget>>;
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;

    fn with_children(mut self, new_children: Vec<Box<dyn Widget>>) -> Self
    where
        Self: Sized,
    {
        let children = self.children_mut();
        children.extend(new_children);
        self
    }

    fn with_child(mut self, child: Box<dyn Widget>) -> Self
    where
        Self: Sized,
    {
        let children = self.children_mut();
        children.push(child);
        self
    }

    fn push_child(&mut self, child: Box<dyn Widget>) {
        let children = self.children_mut();
        children.push(child);
    }

    fn set_children(&mut self, new_children: Vec<Box<dyn Widget>>) {
        let children = self.children_mut();
        children.clear();
        children.extend(new_children);
    }

    fn with_style(mut self, new_style: Style) -> Self
    where
        Self: Sized,
    {
        let style = self.style_mut();
        *style = new_style;
        self
    }

    fn set_style(&mut self, new_style: Style) {
        let style = self.style_mut();
        *style = new_style;
    }
}

pub struct DummyWidget {
    children: Vec<Box<dyn Widget>>,
    style: Style,
}

impl DummyWidget {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: Style::default(),
        }
    }
}

impl Widget for DummyWidget {
    fn render(&mut self, _: &Layout, _: &Box<dyn Theme>) -> Vec<Sketch> {
        Vec::new()
    }

    fn update(&mut self, _: &InteractionInfo, _: &mut UpdateMode) {}

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

pub enum PathMode {
    Fill,
    Stroke,
}

pub enum Sketch {
    Path(Path, Paint, PathMode),
    Text(String, Vector2<f32>, Paint, PathMode),
}
