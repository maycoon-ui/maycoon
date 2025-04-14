use crate::reference::Ref;
use crate::signal::{Listener, Signal};

/// A signal that evaluates a function to get the value.
///
/// The evaluation function will be called every time the value is requested via [Signal::get],
/// so it's recommended to avoid expensive operations.
pub struct EvalSignal<T> {
    eval: Box<dyn Fn() -> T>,
}

impl<T> EvalSignal<T> {
    /// Create a new eval signal using the given evaluation function.
    pub fn new(eval: impl Fn() -> T + 'static) -> Self {
        Self {
            eval: Box::new(eval),
        }
    }
}

impl<T: 'static> Signal<T> for EvalSignal<T> {
    fn get(&self) -> Ref<T> {
        Ref::Owned((self.eval)())
    }

    fn set_value(&self, _: T) {}

    fn listen(&mut self, _: Listener<T>) {}

    fn notify(&self) {}
}
