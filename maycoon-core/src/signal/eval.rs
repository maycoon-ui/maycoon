use crate::reference::Ref;
use crate::signal::{Listener, Signal};
use std::rc::Rc;

/// A signal that evaluates a function to get the value.
///
/// The evaluation function will be called every time the value is requested via [Signal::get],
/// so it's recommended to avoid expensive operations.
pub struct EvalSignal<T: 'static> {
    eval: Rc<dyn Fn() -> T>,
}

impl<T: 'static> EvalSignal<T> {
    /// Create a new eval signal using the given evaluation function.
    pub fn new(eval: impl Fn() -> T + 'static) -> Self {
        Self {
            eval: Rc::new(eval),
        }
    }
}

impl<T: 'static> Signal<T> for EvalSignal<T> {
    fn get(&self) -> Ref<'_, T> {
        Ref::Owned((self.eval)())
    }

    fn set_value(&self, _: T) {}

    fn listen(&mut self, _: Listener<T>) {}

    fn notify(&self) {}

    fn dyn_clone(&self) -> Box<dyn Signal<T>> {
        Box::new(self.clone())
    }
}

impl<T: 'static> Clone for EvalSignal<T> {
    fn clone(&self) -> Self {
        Self {
            eval: self.eval.clone(),
        }
    }
}
