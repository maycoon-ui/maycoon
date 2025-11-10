use crate::signal::{BoxedSignal, Ref, Signal};
use std::rc::Rc;

/// A signal wrapping another signal and applying a mapping function, when the inner value is requested.
/// The mapping function will be called every time the inner value is requested via [Signal::get].
/// This signal cannot be directly mutated. Use [MapSignal::signal] to get the inner signal.
///
/// Calling [Signal::set], [Signal::set_value], [Signal::listen] has no effect.
pub struct MapSignal<T: 'static, U: 'static> {
    signal: BoxedSignal<T>,
    map: Rc<dyn Fn(Ref<T>) -> Ref<U>>,
}

impl<T: 'static, U: 'static> MapSignal<T, U> {
    /// Create a new map signal using the given inner signal and mapping function.
    #[inline(always)]
    pub fn new(signal: BoxedSignal<T>, map: impl Fn(Ref<T>) -> Ref<U> + 'static) -> Self {
        Self {
            signal,
            map: Rc::new(map),
        }
    }

    /// Get the inner signal.
    ///
    /// Can be used to mutate the inner value.
    #[inline(always)]
    pub fn signal(&self) -> BoxedSignal<T> {
        self.signal.dyn_clone()
    }

    /// Get the inner signal's value, without applying the mapping function.
    #[inline(always)]
    pub fn get_unmapped(&self) -> Ref<'_, T> {
        self.signal.get()
    }
}

impl<T: 'static, U: 'static> Signal<U> for MapSignal<T, U> {
    #[inline(always)]
    fn get(&self) -> Ref<'_, U> {
        (self.map)(self.get_unmapped())
    }

    #[inline(always)]
    fn set_value(&self, _: U) {}

    #[inline(always)]
    fn listen(self, _: Box<dyn Fn(Ref<'_, U>)>) -> Self
    where
        Self: Sized,
    {
        self
    }

    #[inline(always)]
    fn notify(&self) {
        self.signal.notify();
    }

    #[inline(always)]
    fn dyn_clone(&self) -> Box<dyn Signal<U>> {
        Box::new(self.clone())
    }
}

impl<T: 'static, U: 'static> Clone for MapSignal<T, U> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            signal: self.signal.dyn_clone(),
            map: self.map.clone(),
        }
    }
}
