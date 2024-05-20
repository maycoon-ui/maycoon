use femtovg::{Color, FontId, Paint};
use may_core::app::context::Context;
use may_core::app::update::Update;
use may_core::layout::{Layout, Style};
use may_core::render::RenderCommand;
use may_core::state::State;
use may_core::widget::{Widget, WidgetLayoutNode, WidgetStyleNode};
use may_theme::colors;
use may_theme::id::WidgetId;
use may_theme::scheme::{Scheme, SchemeValue, WidgetScheme};
use may_theme::theme::{Theme, WidgetType};

#[derive(Default, Debug, Clone)]
pub struct Text {
    text: String,
    style: Style,
    fonts: Option<Vec<FontId>>,
    color: Option<Color>,
    size: Option<f32>,
}

impl Text {
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
            style: Default::default(),
            fonts: None,
            color: None,
            size: None,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn size(&self) -> Option<f32> {
        self.size
    }

    pub fn fonts(&self) -> Option<&Vec<FontId>> {
        self.fonts.as_ref()
    }

    pub fn color(&self) -> Option<Color> {
        self.color
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

impl<S: State> Widget<S> for Text {
    fn render(
        &self,
        style: WidgetScheme,
        layout: WidgetLayoutNode,
        _: &Box<dyn Theme>,
    ) -> Vec<RenderCommand> {
        let mut paint = style.get(
            |g| g.primary_foreground.clone(),
            |c| c.get("color").unwrap().clone().to_paint().unwrap(),
        );

        if let Some(color) = self.color {
            paint.set_color(color);
        }

        if let Some(size) = self.size {
            paint.set_font_size(size);
        }

        if let Some(fonts) = &self.fonts {
            paint.set_font(fonts.as_slice());
        }

        vec![RenderCommand::FillText {
            text: self.text.clone(),
            x: layout.layout.location.x,
            y: layout.layout.location.y + paint.font_size(),
            paint,
        }]
    }

    fn id(&self) -> WidgetId {
        WidgetId::new("may-widgets", "Text")
    }

    fn update(&mut self, _: &mut S, _: &Context, _: &Layout) -> Update {
        Update::empty()
    }

    fn style_node(&self) -> WidgetStyleNode {
        WidgetStyleNode {
            style: self.style.clone(),
            ..Default::default()
        }
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Content
    }
}
