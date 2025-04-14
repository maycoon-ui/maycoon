use crate::signal::{ArcSignal, Listener, Ref, Signal};

/// A signal wrapping another signal and applying a mapping function, when the inner value is requested.
/// The mapping function will be called every time the inner value is requested via [Signal::get].
/// This signal cannot be directly mutated. Use [MapSignal::signal] to get the inner signal.
///
/// Calling [Signal::set], [Signal::set_value], [Signal::listen] or [Signal::notify] has no effect.
pub struct MapSignal<T, U> {
    signal: ArcSignal<T>,
    map: Box<dyn Fn(Ref<T>) -> Ref<U>>,
}

impl<T: 'static, U> MapSignal<T, U> {
    /// Create a new map signal using the given inner signal and mapping function.
    pub fn new(signal: ArcSignal<T>, map: impl Fn(Ref<T>) -> Ref<U> + 'static) -> Self {
        Self {
            signal,
            map: Box::new(map),
        }
    }

    /// Get the inner signal.
    ///
    /// Can be used to mutate the inner value.
    pub fn signal(&self) -> ArcSignal<T> {
        self.signal.clone()
    }

    /// Get the inner signal's value, without applying the mapping function.
    pub fn get_unmapped(&self) -> Ref<T> {
        self.signal.get()
    }
}

impl<T: 'static, U: 'static> Signal<U> for MapSignal<T, U> {
    fn get(&self) -> Ref<U> {
        (self.map)(self.get_unmapped())
    }

    fn set_value(&self, _: U) {}

    fn listen(&mut self, _: Listener<U>) {}

    fn notify(&self) {
        self.signal.notify();
    }
}
