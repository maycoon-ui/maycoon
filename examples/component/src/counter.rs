use maycoon::core::app::update::Update;
use maycoon::core::component::Component;
use maycoon::core::layout::LayoutStyle;
use maycoon::core::state::{State, Val};
use maycoon::core::widget::Widget;
use maycoon::theme::id::WidgetId;
use maycoon::widgets::button::Button;
use maycoon::widgets::container::Container;
use maycoon::widgets::ext::WidgetLayoutExt;
use maycoon::widgets::text::Text;

/// A counter component which basically just wraps a container with some widgets.
pub struct Counter<S: State> {
    inner: Container<S>,
}

impl<S: State> Counter<S> {
    pub fn new(
        count: impl Into<Val<S, i32>>,
        increase: impl Fn(&mut S) + 'static,
        decrease: impl Fn(&mut S) + 'static,
    ) -> Self {
        let count = count.into();

        Self {
            inner: Container::new(vec![
                Val::new_val(Box::new(
                    Button::new(Text::new("Increase".to_string())).with_on_pressed(
                        move |state: &mut S| {
                            increase(state);
                            Update::DRAW
                        },
                    ),
                )),
                Val::new_val(Box::new(
                    Button::new(Text::new("Decrease".to_string())).with_on_pressed(
                        move |state: &mut S| {
                            decrease(state);
                            Update::DRAW
                        },
                    ),
                )),
                Val::new_val(Box::new(Text::new(count.map(|i| i.to_string())))),
            ]),
        }
    }
}

impl<S: State> WidgetLayoutExt<S> for Counter<S> {
    fn set_layout_style(&mut self, layout_style: impl Into<Val<S, LayoutStyle>>) {
        self.inner.set_layout_style(layout_style);
    }
}

impl<S: State> Component<S> for Counter<S> {
    fn get_widget(&mut self) -> &mut impl Widget<S> {
        &mut self.inner
    }

    fn get_widget_id(&self) -> WidgetId {
        WidgetId::new("my-example", "Counter")
    }
}
