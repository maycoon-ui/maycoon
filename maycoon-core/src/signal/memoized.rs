use crate::reference::Ref;
use crate::signal::{Listener, Signal};
use std::sync::OnceLock;

/// A signal for creating a value once, when requested.
/// The value is immutable after creation.
/// Calling [Signal::set], [Signal::set_value], [Signal::listen] or [Signal::notify] has no effect.
///
/// **NOTE:** The inner factory function will only be called once the value is requested via [Signal::get].
pub struct MemoizedSignal<T> {
    inner: OnceLock<T>,
    factory: Box<dyn Fn() -> T>,
}

impl<T> MemoizedSignal<T> {
    /// Create a new memoized signal using the given factory function.
    pub fn new(factory: impl Fn() -> T + 'static) -> Self {
        Self {
            inner: OnceLock::new(),
            factory: Box::new(factory),
        }
    }
}

impl<T: 'static> Signal<T> for MemoizedSignal<T> {
    fn get(&self) -> Ref<T> {
        Ref::Borrow(self.inner.get_or_init(&self.factory))
    }

    fn set_value(&self, _: T) {}

    fn listen(&mut self, _: Listener<T>) {}

    fn notify(&self) {}
}
