use crate::reference::Ref;
use crate::signal::Signal;
use crate::signal::listener::{Listener, ListenerRegister};
use std::cell::OnceCell;
use std::rc::Rc;

/// A signal for creating a value once, when requested.
/// The value is immutable after creation.
/// Calling [Signal::set], [Signal::set_value], [Signal::listen] or [Signal::notify] has no effect.
///
/// **NOTE:** The inner factory function will only be called **once**, when the value is requested via [Signal::get].
pub struct MemoizedSignal<T: 'static> {
    inner: Rc<OnceCell<T>>,
    factory: Rc<dyn Fn() -> T>,
    listeners: ListenerRegister<T>,
}

impl<T: 'static> MemoizedSignal<T> {
    #[inline(always)]
    /// Create a new memoized signal using the given factory function.
    pub fn new(factory: impl Fn() -> T + 'static) -> Self {
        Self {
            inner: Rc::new(OnceCell::new()),
            factory: Rc::new(factory),
            listeners: ListenerRegister::new(),
        }
    }

    /// Returns if the value has been initialized or not.
    #[inline(always)]
    pub fn is_init(&self) -> bool {
        self.inner.get().is_some()
    }
}

impl<T: 'static> Signal<T> for MemoizedSignal<T> {
    #[inline(always)]
    fn get(&self) -> Ref<'_, T> {
        if !self.is_init() {
            self.inner.set((self.factory)()).ok().unwrap();

            self.notify();
        }

        Ref::Borrow(self.inner.get().unwrap())
    }

    #[inline(always)]
    fn set_value(&self, _: T) {}

    #[inline(always)]
    fn listen(self, listener: Box<dyn Fn(Ref<'_, T>)>) -> Self
    where
        Self: Sized,
    {
        Self {
            inner: self.inner,
            factory: self.factory,
            listeners: self.listeners.register(Listener::new(listener)),
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

impl<T: 'static> Clone for MemoizedSignal<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            factory: self.factory.clone(),
            listeners: self.listeners.clone(),
        }
    }
}

#[cfg(all(test, feature = "test"))]
mod tests {
    use crate::signal::Signal;
    use crate::signal::memoized::MemoizedSignal;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Tests the [MemoizedSignal] implementation of [Signal].
    #[test]
    fn test_memoized_signal() {
        let sum = Rc::new(RefCell::new(0));
        let diff = Rc::new(RefCell::new(0));

        let sum_clone = sum.clone();
        let diff_clone = diff.clone();
        let signal = MemoizedSignal::new(|| 1)
            .listen(Box::new(move |val| {
                *sum_clone.borrow_mut() += *val;
            }))
            .listen(Box::new(move |val| {
                *diff_clone.borrow_mut() -= *val;
            }));

        // sum = 0
        assert_eq!(*sum.borrow(), 0);
        // diff = 0
        assert_eq!(*diff.borrow(), 0);

        // signal = 1
        assert_eq!(*signal.get(), 1);

        // sum = sum + signal = 0 + 1 = 1
        assert_eq!(*sum.borrow(), 1);
        // diff = diff - signal = 0 - 1 = -1
        assert_eq!(*diff.borrow(), -1);
    }
}
