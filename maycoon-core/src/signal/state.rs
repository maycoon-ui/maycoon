use crate::signal::listener::ListenerRegister;
use crate::signal::{Listener, Ref, Signal};
use std::cell::RefCell;
use std::rc::Rc;

/// Simple signal implementation based on [Rc] and [RefCell] to get/set a value and notify listeners when it changes.
///
/// You can also mutate the inner value, but only in a set scope.
///
/// As this signal uses [RefCell], it's subject to possible runtime borrowing errors.
/// See the section about these errors in [Signal] for more.
pub struct StateSignal<T: 'static> {
    value: Rc<RefCell<T>>,
    listeners: ListenerRegister<T>,
}

impl<T: 'static> StateSignal<T> {
    /// Creates a new signal with the given value.
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
            listeners: ListenerRegister::new(),
        }
    }

    /// Mutate the inner value in a set scope.
    ///
    /// This scope is needed in order to notify the app for changes.
    #[inline(always)]
    #[tracing::instrument(skip_all)]
    pub fn mutate(&self, op: impl FnOnce(&mut T)) {
        op(&mut self.value.borrow_mut());
        self.notify();
    }
}

impl<T: 'static> Signal<T> for StateSignal<T> {
    #[inline(always)]
    fn get(&self) -> Ref<'_, T> {
        Ref::Ref(self.value.borrow())
    }

    #[inline(always)]
    fn set_value(&self, value: T) {
        *self.value.borrow_mut() = value;
    }

    #[inline(always)]
    fn listen(self, listener: Box<dyn Fn(Ref<'_, T>)>) -> Self
    where
        Self: Sized,
    {
        Self {
            listeners: self.listeners.register(Listener::new(listener)),
            ..self
        }
    }

    #[inline(always)]
    fn notify(&self) {
        self.listeners.notify(|| self.get());
    }

    #[inline(always)]
    fn dyn_clone(&self) -> Box<dyn Signal<T>> {
        Box::new(self.clone())
    }
}

impl<T: 'static> Clone for StateSignal<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            listeners: self.listeners.clone(),
        }
    }
}

#[cfg(all(test, feature = "test"))]
mod tests {
    use crate::signal::Signal;
    use crate::signal::state::StateSignal;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Tests the [StateSignal] implementation of [Signal].
    #[test]
    fn test_state_signal() {
        let sum = Rc::new(RefCell::new(0));
        let diff = Rc::new(RefCell::new(0));

        let sum_clone = sum.clone();
        let diff_clone = diff.clone();
        let signal = StateSignal::new(0)
            .listen(Box::new(move |val| {
                *sum_clone.borrow_mut() += *val;
            }))
            .listen(Box::new(move |val| {
                *diff_clone.borrow_mut() -= *val;
            }));

        let value = *signal.get();

        // value = value + 1 = 0 + 1 = 1
        signal.set(value + 1);

        // sum = sum + value = 0 + 1 = 1
        assert_eq!(*sum.borrow(), 1);

        // diff = sum - value = 0 - 1 = -1
        assert_eq!(*diff.borrow(), -1);

        // value = value - 1 = 1 - 1 = 0
        signal.mutate(|val| {
            *val -= 1;
        });

        // sum = sum + value = 1 + 0 = 1
        assert_eq!(*sum.borrow(), 1);

        // diff = diff + value = -1 + 0 = -1
        assert_eq!(*diff.borrow(), -1);
    }
}
