use crate::app::diagnostics::Diagnostics;
use crate::app::update::{Update, UpdateManager};
use crate::signal::actor::ActorSignal;
use crate::signal::eval::EvalSignal;
use crate::signal::fixed::FixedSignal;
use crate::signal::memoized::MemoizedSignal;
use crate::signal::rw::RwSignal;
use crate::signal::state::StateSignal;
use crate::signal::Signal;
use std::sync::Arc;
use vello::util::RenderContext;

/// The application context for managing the application lifecycle.
#[derive(Clone)]
pub struct AppContext {
    update: UpdateManager,
    diagnostics: Diagnostics,
    render: Arc<RenderContext>,
}

impl AppContext {
    /// Create a new application context using the given [UpdateManager].
    pub fn new(
        update: UpdateManager,
        diagnostics: Diagnostics,
        render: Arc<RenderContext>,
    ) -> Self {
        Self {
            update,
            diagnostics,
            render,
        }
    }

    /// Get the [Diagnostics] of the application.
    pub fn diagnostics(&self) -> Diagnostics {
        self.diagnostics.clone()
    }

    /// Get the [RenderContext] of the application.
    pub fn render_ctx(&self) -> Arc<RenderContext> {
        self.render.clone()
    }

    /// Get the [UpdateManager] of the application.
    pub fn update(&self) -> UpdateManager {
        self.update.clone()
    }

    /// Hook the given [Signal] to the [UpdateManager] of this application.
    ///
    /// This makes the signal reactive, so it will notify the renderer when the inner value changes.
    pub fn hook_signal<T: 'static, S: Signal<T>>(&self, signal: &mut S) {
        let update = self.update();

        signal.listen(Box::new(move |_| {
            update.insert(Update::EVAL);
        }));
    }

    /// Hook the given [Signal] to the [UpdateManager] of this application and return it inside an [Arc].
    ///
    /// See [AppContext::hook_signal] for more.
    pub fn use_signal<T: 'static, S: Signal<T>>(&self, mut signal: S) -> Arc<S> {
        self.hook_signal(&mut signal);

        Arc::new(signal)
    }

    /// Shortcut for creating and hooking a [StateSignal] into the application lifecycle.
    pub fn use_state<T: 'static>(&self, value: T) -> Arc<StateSignal<T>> {
        self.use_signal(StateSignal::new(value))
    }

    /// Shortcut for creating and hooking a [MemoizedSignal] into the application lifecycle.
    pub fn use_memoized<T: 'static>(
        &self,
        value: impl Fn() -> T + 'static,
    ) -> Arc<MemoizedSignal<T>> {
        self.use_signal(MemoizedSignal::new(value))
    }

    /// Shortcut for creating and hooking a [FixedSignal] into the application lifecycle.
    pub fn use_fixed<T: 'static>(&self, value: T) -> Arc<FixedSignal<T>> {
        self.use_signal(FixedSignal::new(value))
    }

    /// Shortcut for creating and hooking an [EvalSignal] into the application lifecycle.
    pub fn use_eval<T: 'static>(&self, eval: impl Fn() -> T + 'static) -> Arc<EvalSignal<T>> {
        self.use_signal(EvalSignal::new(eval))
    }

    /// Shortcut for creating and hooking a [RwSignal] into the application lifecycle.
    pub fn use_rw<T: 'static>(&self, value: T) -> Arc<RwSignal<T>> {
        self.use_signal(RwSignal::new(value))
    }

    /// Shortcut for creating and hooking an [ActorSignal] into the application lifecycle.
    pub fn use_actor<T: 'static>(&self, value: T) -> Arc<ActorSignal<T>> {
        self.use_signal(ActorSignal::new(value))
    }
}
