use crate::app::update::{Update, UpdateManager};
use crate::signal::Signal;
use std::sync::Arc;

/// The application context for managing the application lifecycle.
#[derive(Clone, Debug)]
pub struct AppContext {
    update: UpdateManager,
}

impl AppContext {
    /// Create a new application context using the given [UpdateManager].
    pub fn new(update: UpdateManager) -> Self {
        Self { update }
    }

    /// Get the [UpdateManager] of this application context.
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
}
