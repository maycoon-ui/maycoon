use crate::CRATE_NAME;
use femtovg::Paint;
use may_core::app::context::Context;
use may_core::app::update::Update;
use may_core::layout::Style;
use may_core::render::RenderCommand;
use may_core::state::State;
use may_core::widget::{Widget, WidgetLayoutNode, WidgetStyleNode};
use may_theme::colors;
use may_theme::id::WidgetId;
use may_theme::scheme::{Scheme, SchemeValue};
use may_theme::theme::WidgetType;

#[derive(Default, Debug, Clone)]
pub struct Text {
    text: String,
    style: Style,
    paint: Option<Paint>,
}

impl Text {
    pub fn new(text: String) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }

    pub fn with_paint(mut self, paint: Paint) -> Self {
        self.paint = Some(paint);
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn paint(&self) -> &Option<Paint> {
        &self.paint
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

impl<S: State> Widget<S> for Text {
    fn render(&self, theme: Scheme, layout: WidgetLayoutNode) -> Vec<RenderCommand> {
        vec![RenderCommand::FillText {
            text: self.text.clone(),
            paint: self.paint.clone().unwrap_or(
                theme
                    .get("color")
                    .unwrap_or(&SchemeValue::Paint(Paint::color(colors::BLACK)))
                    .as_paint()
                    .expect("Value needs to be `Paint`")
                    .clone(),
            ),
            x: layout.layout.location.x,
            y: layout.layout.location.y,
        }]
    }

    fn id(&self) -> WidgetId {
        WidgetId::new(CRATE_NAME, "Text")
    }

    fn update(&mut self, _: &mut S, _: &Context) -> Update {
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
