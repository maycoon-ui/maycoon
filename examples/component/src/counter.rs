use maycoon::core::app::context::AppContext;
use maycoon::core::app::update::Update;
use maycoon::core::component::Component;
use maycoon::core::layout::LayoutStyle;
use maycoon::core::reference::Ref;
use maycoon::core::signal::eval::EvalSignal;
use maycoon::core::signal::{ArcSignal, MaybeSignal};
use maycoon::core::widget::{Widget, WidgetLayoutExt};
use maycoon::theme::id::WidgetId;
use maycoon::widgets::button::Button;
use maycoon::widgets::container::Container;
use maycoon::widgets::text::Text;

pub struct Counter {
    counter: ArcSignal<i32>,
    layout: MaybeSignal<LayoutStyle>,
}

impl Counter {
    pub fn new(counter: ArcSignal<i32>) -> Self {
        Counter {
            counter,
            layout: LayoutStyle::default().into(),
        }
    }
}

impl Component for Counter {
    fn build(&self, context: AppContext) -> impl Widget + 'static {
        let counter = self.counter.clone();

        Container::new(vec![
            {
                let counter = counter.clone();

                Box::new(
                    Button::new(Text::new("Increase".to_string())).with_on_pressed(
                        MaybeSignal::signal(context.use_signal(EvalSignal::new(move || {
                            counter.set(*counter.get() + 1);
                            Update::DRAW
                        }))),
                    ),
                )
            },
            {
                let counter = counter.clone();

                Box::new(
                    Button::new(Text::new("Decrease".to_string())).with_on_pressed(
                        MaybeSignal::signal(context.use_signal(EvalSignal::new(move || {
                            counter.set(*counter.get() - 1);
                            Update::DRAW
                        }))),
                    ),
                )
            },
            Box::new(Text::new(
                MaybeSignal::signal(counter).map(|i| Ref::Owned(i.to_string())),
            )),
        ])
        .with_layout_style(self.layout.get().clone())
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("my-example", "Counter")
    }
}

impl WidgetLayoutExt for Counter {
    fn set_layout_style(&mut self, layout_style: impl Into<MaybeSignal<LayoutStyle>>) {
        self.layout = layout_style.into();
    }
}
