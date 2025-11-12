use crate::reference::Ref;
use crate::signal::Signal;
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
    #[inline(always)]
    pub fn new(eval: impl Fn() -> T + 'static) -> Self {
        Self {
            eval: Rc::new(eval),
        }
    }
}

impl<T: 'static> Signal<T> for EvalSignal<T> {
    #[inline(always)]
    fn get(&self) -> Ref<'_, T> {
        Ref::Owned((self.eval)())
    }

    #[inline(always)]
    fn set_value(&self, _: T) {}

    #[inline(always)]
    fn listen(self, _: Box<dyn Fn(Ref<'_, T>)>) -> Self
    where
        Self: Sized,
    {
        self
    }

    #[inline(always)]
    fn notify(&self) {}

    #[inline(always)]
    fn dyn_clone(&self) -> Box<dyn Signal<T>> {
        Box::new(self.clone())
    }
}

impl<T: 'static> Clone for EvalSignal<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            eval: self.eval.clone(),
        }
    }
}

#[cfg(all(test, feature = "test"))]
mod tests {
    use crate::signal::Signal;
    use crate::signal::eval::EvalSignal;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Tests the [EvalSignal] implementation of [Signal].
    #[test]
    fn test_eval_signal() {
        let value = Rc::new(RefCell::new(0));

        let value_clone = value.clone();
        let signal = EvalSignal::new(move || {
            *value_clone.borrow_mut() += 1;
            *value_clone.borrow()
        });

        // signal = value = 1
        assert_eq!(*signal.get(), 1);
        assert_eq!(*value.borrow(), 1);

        // signal = value = 2
        assert_eq!(*signal.get(), 2);
        assert_eq!(*value.borrow(), 2);

        // signal = value = 3
        assert_eq!(*signal.get(), 3);
        assert_eq!(*value.borrow(), 3);
    }
}
