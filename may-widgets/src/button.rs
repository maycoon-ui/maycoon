use may_core::app::info::AppInfo;
use may_core::app::update::Update;
use may_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use may_core::state::State;
use may_core::vg::kurbo::{Affine, Point, Rect, RoundedRect, RoundedRectRadii, Size};
use may_core::vg::peniko::{Brush, Fill};
use may_core::vg::Scene;
use may_core::widget::Widget;
use may_core::window::MouseButton;
use may_theme::id::WidgetId;
use may_theme::theme::Theme;

pub struct Button<S: State> {
    child: Box<dyn Widget<S>>,
    state: ButtonState,
    on_pressed: Box<dyn FnMut(&mut S) -> Update>,
    on_state_change: Box<dyn FnMut(&mut S, ButtonState) -> Update>,
    layout_style: LayoutStyle,
}

impl<S: State> Widget<S> for Button<S> {
    fn render(
        &self,
        scene: &mut Scene,
        theme: &mut dyn Theme,
        info: &AppInfo,
        layout_node: &LayoutNode,
    ) {
        let transform = Affine::scale_non_uniform(
            layout_node.layout.location.x as f64,
            layout_node.layout.location.y as f64,
        );

        let brush = if let Some(style) = theme.of(self.widget_id()) {
            match self.state {
                ButtonState::Hovered => style.get_brush("color_hovered").unwrap(),
                ButtonState::Pressed => style.get_brush("color_pressed").unwrap(),
                ButtonState::Idle => style.get_brush("color_idle").unwrap(),
            }
        } else {
            Brush::Solid(match self.state {
                ButtonState::Hovered => theme.defaults().interactive().hover(),
                ButtonState::Pressed => theme.defaults().interactive().active(),
                ButtonState::Idle => theme.defaults().interactive().inactive(),
            })
        };

        scene.fill(
            Fill::NonZero,
            transform,
            &brush,
            None,
            &RoundedRect::from_rect(
                Rect::from_center_size(
                    Point::new(
                        layout_node.layout.location.x as f64,
                        layout_node.layout.location.y as f64,
                    ),
                    Size::new(
                        layout_node.layout.size.width as f64,
                        layout_node.layout.size.height as f64,
                    ),
                ),
                RoundedRectRadii::from_single_radius(10.0),
            ),
        );

        {
            let mut child_scene = Scene::new();

            self.child.render(
                &mut child_scene,
                theme,
                info,
                layout_node.children.first().unwrap(),
            );

            scene.append(&mut child_scene, None);
        }
    }

    fn layout_style(&self) -> StyleNode {
        StyleNode {
            style: self.layout_style.clone(),
            children: vec![self.child.layout_style()],
        }
    }

    fn update(&mut self, layout: &LayoutNode, state: &mut S, info: &AppInfo) -> Update {
        let mut btn_state = self.state;
        let mut update = Update::empty();

        if let Some(mouse) = info.cursor_pos.as_ref() {
            if mouse.x as f32 >= layout.layout.location.x
                && mouse.y as f32 >= layout.layout.location.y
                && mouse.x as f32 <= layout.layout.location.x + layout.layout.size.width
                && mouse.y as f32 <= layout.layout.location.y + layout.layout.size.height
            {
                btn_state = ButtonState::Hovered;

                for (_, event, el_state) in &info.buttons {
                    if MouseButton::Left == *event && el_state.is_pressed() {
                        btn_state = ButtonState::Pressed;
                        break;
                    }
                }
            }
        }

        if self.state != btn_state {
            update.set((self.on_state_change)(state, btn_state));
        }

        if ButtonState::Pressed == btn_state {
            update.set((self.on_pressed)(state));
        }

        self.state = btn_state;

        update
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("may-widgets", "Button")
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ButtonState {
    Hovered,
    Pressed,
    Idle,
}
