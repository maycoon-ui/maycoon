use crate::signal::{Listener, Ref, Signal};
use arc_swap::ArcSwap;
use std::sync::Arc;

/// Simple signal implementation based on [ArcSwap] to load/store a value and notify listeners when it changes.
pub struct StateSignal<T> {
    value: ArcSwap<T>,
    listeners: Vec<Listener<T>>,
}

impl<T> StateSignal<T> {
    /// Creates a new signal with the given value.
    pub fn new(value: T) -> Self {
        Self {
            value: ArcSwap::new(Arc::new(value)),
            listeners: Vec::with_capacity(1),
        }
    }
}

impl<T: 'static> Signal<T> for StateSignal<T> {
    fn get(&self) -> Ref<T> {
        Ref::Arc(self.value.load().clone())
    }

    fn set_value(&self, value: T) {
        self.value.store(Arc::new(value));
    }

    fn listen(&mut self, listener: Listener<T>) {
        self.listeners.push(listener);
    }

    fn notify(&self) {
        for listener in &self.listeners {
            listener(self.get());
        }
    }
}
