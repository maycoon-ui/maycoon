use crate::signal::{Listener, Ref, Signal};
use std::sync::Arc;

/// A signal with a fixed value.
///
/// The inner value cannot be changed and listeners do not exist.
/// Useful for testing purposes.
#[derive(Clone)]
pub struct FixedSignal<T> {
    value: Arc<T>,
}

impl<T> FixedSignal<T> {
    /// Creates a new fixed signal.
    pub fn new(value: T) -> Self {
        Self {
            value: Arc::new(value),
        }
    }
}

impl<T: 'static> Signal<T> for FixedSignal<T> {
    fn get(&self) -> Ref<T> {
        Ref::Arc(self.value.clone())
    }

    fn set_value(&self, _: T) {}

    fn listen(&mut self, _: Listener<T>) {}

    fn notify(&self) {}
}
