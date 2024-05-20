use femtovg::Path;
use may_core::app::context::Context;
use may_core::app::update::Update;
use may_core::layout::{Dimension, Layout, Size, Style};
use may_core::render::RenderCommand;
use may_core::state::State;
use may_core::widget::{Widget, WidgetLayoutNode, WidgetStyleNode};
use may_core::window::{ElementState, MouseButton};
use may_theme::id::WidgetId;
use may_theme::scheme::{Scheme, WidgetScheme};
use may_theme::theme::{Theme, WidgetType};
use ui_math::point::Point;
use ui_math::rect::Rect;

pub struct Button<S: State> {
    on_click: Box<dyn FnMut(&mut S) -> Update>,
    on_hover: Box<dyn FnMut(&mut S) -> Update>,
    child: Box<dyn Widget<S>>,
    style: Style,
    hovering: bool,
    clicked: bool,
}

impl<S: State> Button<S> {
    pub fn new(child: impl Widget<S> + 'static) -> Button<S> {
        Button {
            on_click: Box::new(|_| Update::empty()),
            on_hover: Box::new(|_| Update::empty()),
            child: Box::new(child),
            style: Style {
                size: Size::<Dimension> {
                    width: Dimension::Length(200.0), // TODO: remove this
                    height: Dimension::Length(25.0),
                },
                ..Style::default()
            },
            hovering: false,
            clicked: false,
        }
    }

    pub fn with_child(mut self, child: impl Widget<S> + 'static) -> Button<S> {
        self.child = Box::new(child);
        self
    }

    pub fn with_style(mut self, style: Style) -> Button<S> {
        self.style = style;
        self
    }

    pub fn with_on_click<F>(mut self, on_click: F) -> Button<S>
    where
        F: (FnMut(&mut S) -> Update) + 'static,
    {
        self.on_click = Box::new(on_click);
        self
    }

    pub fn with_on_hover<F>(mut self, on_hover: F) -> Button<S>
    where
        F: (FnMut(&mut S) -> Update) + 'static,
    {
        self.on_hover = Box::new(on_hover);
        self
    }
}

impl<S: State> Widget<S> for Button<S> {
    fn render(
        &self,
        style: WidgetScheme,
        layout: WidgetLayoutNode,
        theme: &Box<dyn Theme>,
    ) -> Vec<RenderCommand> {
        let mut commands = vec![RenderCommand::Fill {
            path: {
                let mut path = Path::new();

                path.rect(
                    layout.layout.location.x,
                    layout.layout.location.y,
                    layout.layout.size.width,
                    layout.layout.size.height,
                );

                path.close();

                path
            },
            paint: style.get(
                |g| {
                    if self.hovering {
                        g.primary_background.clone()
                    } else {
                        g.secondary_background.clone()
                    }
                },
                |c| {
                    c.get(if self.hovering {
                        "color-hover"
                    } else if self.clicked {
                        "color-click"
                    } else {
                        "color"
                    })
                    .unwrap()
                    .clone()
                    .to_paint()
                    .unwrap()
                },
            ),
        }];

        commands.append(&mut self.child.render(
            theme.scheme_of(self.child.id(), self.child.widget_type()),
            layout.children[0].clone(),
            theme,
        ));

        commands
    }

    fn id(&self) -> WidgetId {
        WidgetId::new("may-widgets", "Button")
    }

    fn update(&mut self, state: &mut S, ctx: &Context, layout: &Layout) -> Update {
        let mut update = Update::empty();

        if let Some(pos) = ctx.app_info.cursor_pos {
            let rect = Rect::new(
                layout.location.x,
                layout.location.y,
                layout.size.width,
                layout.size.height,
            );

            if rect.contains(&Point::new(pos.x, pos.y)) {
                update.insert(Update::DRAW);

                update.insert((self.on_hover)(state));

                self.hovering = true;

                if ctx
                    .app_info
                    .buttons
                    .contains(&(ElementState::Pressed, MouseButton::Left))
                    || self.clicked
                {
                    update.insert((self.on_click)(state));
                    self.clicked = true;
                } else {
                    update.insert(Update::DRAW);
                    self.clicked = false;
                }
            } else {
                if self.hovering {
                    update.insert(Update::DRAW);
                    self.hovering = false;
                }
            }
        }

        update
    }

    fn style_node(&self) -> WidgetStyleNode {
        // TODO: adjust style to child
        WidgetStyleNode {
            style: self.style.clone(),
            children: vec![self.child.style_node()],
        }
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Interactive
    }
}
