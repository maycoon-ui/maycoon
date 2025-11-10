use crate::reference::Ref;
use crate::signal::Signal;
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
}

impl<T: 'static> MemoizedSignal<T> {
    #[inline(always)]
    /// Create a new memoized signal using the given factory function.
    pub fn new(factory: impl Fn() -> T + 'static) -> Self {
        Self {
            inner: Rc::new(OnceCell::new()),
            factory: Rc::new(factory),
        }
    }

    #[inline(always)]
    /// Returns if the value has been initialized or not.
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

impl<T: 'static> Clone for MemoizedSignal<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            factory: self.factory.clone(),
        }
    }
}
