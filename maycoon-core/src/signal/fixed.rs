use crate::signal::{Listener, Ref, Signal};

/// A signal with a fixed value.
///
/// The inner value cannot be changed and listeners do not exist.
/// Useful for testing purposes.
#[derive(Clone)]
pub struct FixedSignal<T> {
    value: T,
}

impl<T> FixedSignal<T> {
    /// Creates a new fixed signal.
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: 'static> Signal<T> for FixedSignal<T> {
    fn get(&self) -> Ref<'_, T> {
        Ref::Borrow(&self.value)
    }

    fn set_value(&self, _: T) {}

    fn listen(&mut self, _: Listener<T>) {}

    fn notify(&self) {}
}
