use maycoon_core::app::context::AppContext;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::tasks;
use maycoon_core::tasks::runner::Task;
use maycoon_core::vgi::Scene;
use maycoon_core::widget::Widget;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use std::future::Future;

/// Widget builder to fetch data from an asynchronous task. The [TaskRunner](tasks::runner::TaskRunner) needs to be initialized.
/// This is similar to the [FutureBuilder](https://api.flutter.dev/flutter/widgets/FutureBuilder-class.html) from Flutter.
///
/// ### Async + UI
/// Normally, a UI Application is not able to asynchronously spawn tasks and will block the UI thread instead of running tasks in the background.
/// The [WidgetFetcher] is able to spawn asynchronous tasks on a global runner and construct a widget based on the result of the task.
/// You can use it to fetch asynchronous data and either return something like a loading screen or the actual data.
///
/// ### Workflow of a [WidgetFetcher].
/// 1. Run the task in the background using the [TaskRunner](tasks::runner::TaskRunner).
/// 2. Construct the widget with [None] as the result (task is still loading).
/// 3. Once the task is done, update the UI with the new result and trigger an [Update].
///
/// ### Theming
/// The widget itself only draws the underlying widget, so theming is useless.
pub struct WidgetFetcher<T: Send + Unpin + 'static, W: Widget, F: Fn(Option<T>) -> W> {
    task: Option<Task<T>>,
    render: F,
    widget: Option<W>,
    update: Update,
}

impl<T: Send + Unpin + 'static, W: Widget, F: Fn(Option<T>) -> W> WidgetFetcher<T, W, F> {
    /// Creates a new [WidgetFetcher] with parameters:
    /// - `future`: The future to execute.
    /// - `update`: The update to trigger when the data is updated (from loading to done).
    /// - `render`: The function to render the widget. The first parameter is the result of the future and the second parameter is the mutable app state.
    pub fn new<Fut>(future: Fut, update: Update, render: F) -> Self
    where
        Fut: Future<Output = T> + Send + 'static,
    {
        let task = tasks::spawn(future);

        Self {
            task: Some(task),
            render,
            widget: None,
            update,
        }
    }

    /// Checks if the task is finished and blocks on it if it is.
    ///
    /// Returns [Some] if the task is completed, otherwise [None].
    pub fn try_block_on(&mut self) -> Option<T> {
        if self.task.is_some() && self.task.as_ref().unwrap().is_ready() {
            let task = self.task.take().unwrap();
            let value = tasks::block_on(task);

            Some(value)
        } else {
            None
        }
    }
}

impl<T: Send + Unpin + 'static, W: Widget, F: Fn(Option<T>) -> W> Widget
    for WidgetFetcher<T, W, F>
{
    fn render(
        &mut self,
        scene: &mut dyn Scene,
        theme: &mut dyn Theme,
        layout_node: &LayoutNode,
        info: &AppInfo,
        context: AppContext,
    ) {
        if let Some(widget) = &mut self.widget {
            widget.render(scene, theme, layout_node, info, context.clone())
        }
    }

    fn layout_style(&self) -> StyleNode {
        if let Some(widget) = &self.widget {
            widget.layout_style()
        } else {
            StyleNode {
                style: LayoutStyle::default(),
                children: Vec::new(),
            }
        }
    }

    fn update(&mut self, layout: &LayoutNode, context: AppContext, info: &AppInfo) -> Update {
        let mut update = Update::empty();

        if let Some(value) = self.try_block_on() {
            self.widget = Some((self.render)(Some(value)));
            update = self.update;
        } else if self.widget.is_none() {
            self.widget = Some((self.render)(None));
            update = self.update;
        }

        // Widget is guaranteed to be some at this point
        self.widget.as_mut().unwrap().update(layout, context, info) | update
    }

    fn widget_id(&self) -> WidgetId {
        if let Some(widget) = &self.widget {
            widget.widget_id()
        } else {
            WidgetId::new("maycoon-widgets", "WidgetFetcher")
        }
    }
}
