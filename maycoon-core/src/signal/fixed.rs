use crate::signal::{Listener, Ref, Signal};
use std::rc::Rc;

/// A signal with a fixed value. Based on [Rc].
///
/// The inner value cannot be mutated and listeners do not exist.
///
/// Useful for testing purposes, but also used in `MaybeSignal::into_signal` to convert a fixed value into a signal.
pub struct FixedSignal<T: 'static> {
    value: Rc<T>,
}

impl<T: 'static> FixedSignal<T> {
    /// Creates a new fixed signal.
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(value),
        }
    }
}

impl<T: 'static> Signal<T> for FixedSignal<T> {
    #[inline(always)]
    fn get(&self) -> Ref<'_, T> {
        Ref::Borrow(self.value.as_ref())
    }

    #[inline(always)]
    fn set_value(&self, _: T) {}

    #[inline(always)]
    fn listen(&mut self, _: Listener<T>) {}

    #[inline(always)]
    fn notify(&self) {}

    #[inline(always)]
    fn dyn_clone(&self) -> Box<dyn Signal<T>> {
        Box::new(self.clone())
    }
}

impl<T: 'static> From<Rc<T>> for FixedSignal<T> {
    #[inline(always)]
    fn from(value: Rc<T>) -> Self {
        Self { value }
    }
}

impl<T: 'static> Clone for FixedSignal<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}
