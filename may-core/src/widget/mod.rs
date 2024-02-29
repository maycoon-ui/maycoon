pub mod interaction;
pub mod widgets;

#[cfg(feature = "canvas")]
pub mod canvas;

use crate::widget::interaction::Action;
use femtovg::{Paint, Path};
use may_theme::theme::Theme;
use mint::Vector2;
use taffy::{Layout, Style};

pub trait Widget {
    fn render(&mut self, layout: &Layout) -> Vec<Sketch>;
    fn children(&self) -> &Vec<Box<dyn Widget>>;
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Widget>>;
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;
    fn interactive(&self) -> bool;
    fn interact(&mut self, actions: Vec<Action>);
    fn apply_theme(&mut self, theme: Box<dyn Theme>);

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

pub enum PathMode {
    Fill,
    Stroke,
}

pub enum Sketch {
    Path(Path, Paint, PathMode),
    Text(String, Vector2<f32>, Paint, PathMode),
}
