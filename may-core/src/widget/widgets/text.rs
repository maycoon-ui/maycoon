use crate::widget::interaction::{InteractionInfo};
use crate::widget::{PathMode, Sketch, Widget};
use femtovg::{Color, FontId, Paint};
use may_theme::theme::Theme;
use mint::Vector2;
use taffy::{Layout, Style};

pub struct Text {
    text: String,
    children: Vec<Box<dyn Widget>>,
    style: Style,
    paint: Paint,
}

impl Text {
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
            children: Vec::new(),
            style: Style::default(),
            paint: Paint::color(Color::black()),
        }
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.paint.set_font_size(size);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.paint.set_color(color);
        self
    }

    pub fn with_paint(mut self, paint: Paint) -> Self {
        self.paint = paint;
        self
    }

    pub fn with_text(mut self, text: impl ToString) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn with_anti_alias(mut self, anti_alias: bool) -> Self {
        self.paint.set_anti_alias(anti_alias);
        self
    }

    pub fn with_font(mut self, fonts: &[FontId]) -> Self {
        self.paint.set_font(fonts);
        self
    }
}

impl Widget for Text {
    fn render(&mut self, layout: &Layout, theme: &Box<dyn Theme>, _: &InteractionInfo) -> Vec<Sketch> {
        vec![Sketch::Text(
            self.text.clone(),
            Vector2::<f32> {
                x: layout.location.x + layout.size.width,
                y: layout.location.y + layout.size.height,
            },
            self.paint.clone(),
            PathMode::Fill,
        )]
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
