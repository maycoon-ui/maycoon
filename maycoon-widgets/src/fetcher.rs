use maycoon_core::app::context::AppContext;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::tasks::fetcher::Fetcher;
use maycoon_core::vgi::Scene;
use maycoon_core::widget::Widget;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use std::future::Future;

/// A widget to build an inner widget from an asynchronous task.
/// This is a [Widget] version of [Fetcher].
///
/// It is similar to the [FutureBuilder](https://api.flutter.dev/flutter/widgets/FutureBuilder-class.html) from Flutter.
///
/// ### Async + UI
/// When working with a future, you usually need to block on it to produce a value.
/// This would however block the main thread and freeze the whole application temporarily until the task is done.
/// Blocking is less than ideal for any UI application,
/// so the [Fetcher] and [WidgetFetcher] structures exist to run a task in the background,
/// while still providing the user with a smooth interface experience.
///
/// The [WidgetFetcher] will spawn the given task in the background.
/// While the task is still running/loading, the fetcher will still be able to produce a widget via a given factory function.
/// This factory function takes `Option<T>` as an argument, where `T` is the output of the spawned task.
/// If the task is not ready yet, the factory will be called with `None`, otherwise it will be called with `Some(T)`.
///
/// ### Spawning the task
/// By default, the task is spawned inside the [TaskRunner](maycoon_core::tasks::runner::TaskRunner) in a non-blocking manner.
/// If you want to spawn a blocking task (which is however, only supported on native platforms), use [WidgetFetcher::new_blocking]
///
/// **NOTE:** If the future is spawned on a thread pool or not,
/// is up to the [TaskRunner](maycoon_core::tasks::runner::TaskRunner) implementation.
/// Blocking tasks will always be spawned on a thread pool.
///
/// ### Note for the Web
/// On the web, blocking a task is not possible and will freeze the browser,
/// so only futures can be spawned.
/// Furthermore, the futures will be executed on the local thread,
/// since WebAssembly doesn't support threading out-of-the-box yet.
///
/// ### Workflow of a [WidgetFetcher].
/// 1. Run the task in the background using the [TaskRunner](tasks::runner::TaskRunner).
/// 2. Construct the widget with [None] passed into the factory function (while task is still loading).
/// 3. Once the task is done, the UI is updated with the new result and an [Update] is triggered.
///
/// ### Theming
/// The widget itself only draws the underlying widget, so theming is useless.
///
/// The [WidgetId] is equal to `maycoon-widgets:WidgetFetcher`.
pub struct WidgetFetcher<T: Send + 'static, W: Widget> {
    fetcher: Fetcher<T, W>,
    update: Update,
}

impl<T: Send + 'static, W: Widget> WidgetFetcher<T, W> {
    /// Creates a new [WidgetFetcher] with parameters:
    /// - `future`: The future to execute.
    /// - `update`: The update to trigger when the data is updated (when loading is done).
    /// - `render`: The function to render the widget. It takes a possible task result as the only parameter.
    ///
    /// Unlike [WidgetFetcher::new_blocking]. this will spawn a future in the background.
    /// It's up to the task runner implementation, if the task is spawned on a thread pool or not.
    #[inline(always)]
    pub fn new<Fut>(future: Fut, render: impl Fn(Option<T>) -> W + 'static, update: Update) -> Self
    where
        Fut: Future<Output = T> + Send + 'static,
    {
        Self {
            fetcher: Fetcher::spawn(future, render),
            update,
        }
    }

    /// Creates a new [WidgetFetcher] with parameters:
    /// - `func`: The blocking task to execute.
    /// - `update`: The update to trigger when the data is updated (when loading is done).
    /// - `render`: The function to render the widget. It takes a possible task result as the only parameter.
    ///
    /// Unlike [WidgetFetcher::new], this takes a function which will be run on a separate thread pool.
    /// This is only supported on native platforms.
    #[inline(always)]
    #[cfg(native)]
    pub fn new_blocking<F>(
        func: F,
        render: impl Fn(Option<T>) -> W + 'static,
        update: Update,
    ) -> Self
    where
        F: Fn() -> T + Send + 'static,
    {
        Self {
            fetcher: Fetcher::spawn_blocking(func, render),
            update,
        }
    }
}

impl<T: Send + 'static, W: Widget> Widget for WidgetFetcher<T, W> {
    #[inline(always)]
    fn render(
        &mut self,
        scene: &mut dyn Scene,
        theme: &mut dyn Theme,
        layout_node: &LayoutNode,
        info: &AppInfo,
        context: AppContext,
    ) {
        if let Some(widget) = self.fetcher.value_mut() {
            widget.render(scene, theme, layout_node, info, context.clone())
        }
    }

    #[inline(always)]
    fn layout_style(&self) -> StyleNode {
        if let Some(widget) = self.fetcher.value_ref() {
            widget.layout_style()
        } else {
            StyleNode {
                style: LayoutStyle::default(),
                children: Vec::new(),
            }
        }
    }

    #[inline(always)]
    fn update(&mut self, layout: &LayoutNode, context: AppContext, info: &AppInfo) -> Update {
        let mut update = Update::empty();

        // Make sure to update the widget if the task is about to be polled
        if self.fetcher.is_ready() {
            update.insert(self.update);
        }

        let widget = self.fetcher.fetch();

        widget.update(layout, context, info) | update
    }

    #[inline(always)]
    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "WidgetFetcher")
    }
}
