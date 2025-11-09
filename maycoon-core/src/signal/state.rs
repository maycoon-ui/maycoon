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
        self.mutate(move |old| *old = value);
    }

    #[inline(always)]
    fn listen(&mut self, listener: Listener<T>) {
        self.listeners.register(listener);
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
