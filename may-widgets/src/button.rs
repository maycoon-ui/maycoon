use femtovg::{Paint, Path};

use may_core::layout::{Dimension, Layout, Size, Style};
use may_core::widget::interaction::{cursor_hovering_over, InteractionInfo, is_key_pressed, is_mouse_button_pressed};
use may_core::widget::update::Update;
use may_core::widget::{PathMode, Sketch, Widget};
use may_core::window::keyboard::Key;
use may_core::window::{ElementState, MouseButton};
use may_theme::scheme::SchemeValue;
use may_theme::theme::{Theme, WidgetType};

pub struct Button<'a> {
    children: Vec<Box<dyn Widget<'a> + 'a>>,
    style: Style,
    on_press: Box<dyn FnMut() + Send>,
    hover: bool,
    pressed: bool,
}

impl<'a> Button<'a> {
    pub fn new(child: impl Into<Box<dyn Widget<'a> + 'a>>) -> Self {
        Self {
            children: vec![child.into()],
            style: Style {
                size: Size::<Dimension> {
                    width: Dimension::Length(100.0),
                    height: Dimension::Length(60.0),
                },
                ..Default::default()
            },
            hover: false,
            pressed: false,
            on_press: Box::new(|| {}),
        }
    }

    pub fn on_press(mut self, on_press: impl FnMut() + Send + 'static) -> Self {
        self.on_press = Box::new(on_press);
        self
    }

    pub fn id() -> String {
        String::from("may-widgets:Button")
    }
}

impl<'a> Widget<'a> for Button<'a> {
    fn render(&mut self, layout: &Layout, theme: &Box<dyn Theme>) -> Vec<Sketch> {
        let scheme = theme
            .scheme_of(Button::id())
            .unwrap_or(theme.default_scheme_of(WidgetType::Interactive));

        let background = if self.hover {
            if self.pressed {
                scheme.get("background_pressed")
                    .unwrap_or(&SchemeValue::Paint(Paint::default()))
                    .as_paint()
                    .unwrap()
                    .clone()
            } else {
                scheme.get("background_hover")
                    .unwrap_or(&SchemeValue::Paint(Paint::default()))
                    .as_paint()
                    .unwrap()
                    .clone()
            }
        } else {
            scheme.get("background")
                .unwrap_or(&SchemeValue::Paint(Paint::default()))
                .as_paint()
                .unwrap()
                .clone()
        };

        let mut rect = Path::new();
        rect.rect(
            layout.location.x,
            layout.location.y,
            layout.size.width,
            layout.size.height,
        );

        vec![Sketch::Path(
            rect,
            background,
            PathMode::Fill,
        )]
    }

    fn update(&mut self, info: &InteractionInfo, layout: &Layout) -> Update {
        if cursor_hovering_over(info, layout) {
            self.hover = true;
            self.pressed = is_mouse_button_pressed(info, MouseButton::Left, ElementState::Pressed);
            Update::DRAW
        } else {
            Update::empty()
        }
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
