use crate::widget::interaction::InteractionInfo;
use crate::widget::update::Update;
use femtovg::{Paint, Path};
use may_theme::theme::Theme;
use mint::Vector2;
use taffy::{Layout, Style};

pub mod interaction;
pub mod update;

pub trait Widget<'a>: Send {
    fn render(&mut self, layout: &Layout, theme: &Box<dyn Theme>) -> Vec<Sketch>;
    fn update(&mut self, info: &InteractionInfo, layout: &Layout) -> Update;
    fn children(&self) -> &Vec<Box<dyn Widget<'a> + 'a>>;
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Widget<'a> + 'a>>;
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;

    fn with_children(mut self, new_children: Vec<Box<dyn Widget<'a> + 'a>>) -> Self
    where
        Self: Sized,
    {
        let children = self.children_mut();
        children.extend(new_children);
        self
    }

    fn with_child(mut self, child: Box<dyn Widget<'a> + 'a>) -> Self
    where
        Self: Sized,
    {
        let children = self.children_mut();
        children.push(child);
        self
    }

    fn push_child(&mut self, child: Box<dyn Widget<'a> + 'a>) {
        let children = self.children_mut();
        children.push(child);
    }

    fn set_children(&mut self, new_children: Vec<Box<dyn Widget<'a> + 'a>>) {
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

impl<'a, T: Widget<'a> + 'a> From<T> for Box<dyn Widget<'a> + 'a> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

pub struct DummyWidget<'a> {
    children: Vec<Box<dyn Widget<'a> + 'a>>,
    style: Style,
}

impl<'a> DummyWidget<'a> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: Style::default(),
        }
    }
}

impl<'a> Widget<'a> for DummyWidget<'a> {
    fn render(&mut self, _: &Layout, _: &Box<dyn Theme>) -> Vec<Sketch> {
        Vec::new()
    }

    fn update(&mut self, info: &InteractionInfo, layout: &Layout) -> Update {
        Update::empty()
    }

    fn children(&self) -> &Vec<Box<dyn Widget<'a> + 'a>> {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn Widget<'a> + 'a>> {
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
