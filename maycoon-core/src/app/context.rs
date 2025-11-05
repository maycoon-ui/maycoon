use crate::app::diagnostics::Diagnostics;
use crate::app::update::{Update, UpdateManager};
use crate::signal::Signal;
use crate::signal::eval::EvalSignal;
use crate::signal::fixed::FixedSignal;
use crate::signal::listener::Listener;
use crate::signal::memoized::MemoizedSignal;
use crate::signal::state::StateSignal;
use tracing::instrument;

/// The application context for managing the application lifecycle.
#[derive(Clone)]
pub struct AppContext {
    update: UpdateManager,
    diagnostics: Diagnostics,
}

impl AppContext {
    /// Create a new application context using the given [UpdateManager].
    pub fn new(update: UpdateManager, diagnostics: Diagnostics) -> Self {
        Self {
            update,
            diagnostics,
        }
    }

    /// Get the [Diagnostics] of the application.
    pub fn diagnostics(&self) -> Diagnostics {
        self.diagnostics
    }

    /// Get the [UpdateManager] of the application.
    pub fn update(&self) -> UpdateManager {
        self.update.clone()
    }

    /// Make the application exit by setting [Update::EXIT].
    pub fn exit(&self) {
        self.update.insert(Update::EXIT);
    }

    /// Hook the given [Signal] to the [UpdateManager] of this application.
    ///
    /// This makes the signal reactive, so it will notify the renderer when the inner value changes.
    #[instrument(level = "trace", skip_all, fields(signal = std::any::type_name::<T>()))]
    pub fn hook_signal<T: 'static, S: Signal<T>>(&self, signal: &mut S) {
        let update = self.update();
        signal.listen(Listener::new(move |_| update.insert(Update::EVAL)));
    }

    /// Hook the given [Signal] to the [UpdateManager] of this application and return it.
    ///
    /// See [AppContext::hook_signal] for more.
    pub fn use_signal<T: 'static, S: Signal<T>>(&self, mut signal: S) -> S {
        self.hook_signal(&mut signal);

        signal
    }

    /// Shortcut for creating and hooking a [StateSignal] into the application lifecycle.
    pub fn use_state<T: 'static>(&self, value: T) -> StateSignal<T> {
        self.use_signal(StateSignal::new(value))
    }

    /// Shortcut for creating and hooking a [MemoizedSignal] into the application lifecycle.
    pub fn use_memoized<T: 'static>(&self, value: impl Fn() -> T + 'static) -> MemoizedSignal<T> {
        self.use_signal(MemoizedSignal::new(value))
    }

    /// Shortcut for creating and hooking a [FixedSignal] into the application lifecycle.
    pub fn use_fixed<T: 'static>(&self, value: T) -> FixedSignal<T> {
        self.use_signal(FixedSignal::new(value))
    }

    /// Shortcut for creating and hooking an [EvalSignal] into the application lifecycle.
    pub fn use_eval<T: 'static>(&self, eval: impl Fn() -> T + 'static) -> EvalSignal<T> {
        self.use_signal(EvalSignal::new(eval))
    }
}
