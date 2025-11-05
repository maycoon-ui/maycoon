use crate::app::context::AppContext;
use crate::reference::Ref;
use crate::signal::fixed::FixedSignal;
use crate::signal::listener::Listener;
use crate::signal::map::MapSignal;
use std::rc::Rc;

/// Contains the [FixedSignal] signal.
pub mod fixed;

/// Contains the [memoized::MemoizedSignal] signal.
pub mod memoized;

/// Contains the [state::StateSignal] signal.
pub mod state;

/// Contains the [MapSignal] signal.
pub mod map;

/// Contains the [eval::EvalSignal] signal.
pub mod eval;

/// Contains the [Listener] listener.
pub mod listener;

/// A [Signal] in a [Box].
pub type BoxedSignal<T> = Box<dyn Signal<T>>;

/// Base signal trait.
///
/// Signals store values of type `T` and notify listeners when they change.
///
/// **NOTE:** By default, signals don't have any listeners. To "hook" a signal into the application cycle, call [use_signal].
///
/// # Avoiding Borrowing Errors
/// Be careful not to write something like `signal.set(*signal.get());`,
/// as many signals that use [Rc] or [RefCell] might panic, due to the value already being borrowed
/// (by the `signal.get();` call). Write `let value = *signal.get();` or `let value = signal.clone()`
/// and then `signal.set(value);` instead.
///
/// [use_signal]: AppContext::use_signal
/// [RefCell]: std::cell::RefCell
pub trait Signal<T: 'static>: 'static {
    /// Get a reference to the current value of the signal.
    fn get(&self) -> Ref<'_, T>;

    /// Set the value of the signal.
    ///
    /// **NOTE:** This does not notify listeners, use [set] instead.
    fn set_value(&self, value: T);

    /// Add a listener to the signal, which will be called when the inner value changes.
    fn listen(&mut self, listener: Listener<T>);

    /// Notify listeners that the inner value has changed.
    /// May also be called manually to update listeners.
    fn notify(&self);

    /// Set the value of the signal and notify listeners.
    fn set(&self, value: T) {
        tracing::trace_span!("set signal").in_scope(|| {
            self.set_value(value);
            self.notify();
        });
    }

    /// Converts the signal into a [MaybeSignal].
    fn maybe(&self) -> MaybeSignal<T>
    where
        Self: Sized,
    {
        MaybeSignal::signal(self.dyn_clone())
    }

    /// Converts this signal into a [MapSignal] and applies the given mapping function.
    fn map<U: 'static>(&self, map: impl Fn(Ref<T>) -> Ref<U> + 'static) -> MapSignal<T, U>
    where
        Self: Sized,
    {
        MapSignal::new(self.dyn_clone(), map)
    }

    /// Hooks the signal into the given [AppContext].
    ///
    /// Required for the signal to become reactive with the app lifecycle.
    fn hook(self, context: &AppContext) -> Self
    where
        Self: Sized,
    {
        context.use_signal(self)
    }

    /// Clones the signal into a `Box<dyn Signal<T>>`.
    ///
    /// This is an object-safe alternative to simply making the signal [Clone]-dependent.
    fn dyn_clone(&self) -> Box<dyn Signal<T>>;
}

/// A value which may be a signal or a fixed value.
pub enum MaybeSignal<T: 'static> {
    /// A signal.
    Signal(BoxedSignal<T>),
    /// A fixed value wrapped inside a [Rc].
    Value(Rc<T>),
}

impl<T: 'static> MaybeSignal<T> {
    /// Wrap a [Signal] inside a [MaybeSignal].
    pub fn signal(signal: BoxedSignal<T>) -> Self {
        Self::Signal(signal)
    }

    /// Wrap a value inside a [MaybeSignal].
    pub fn value(value: T) -> Self {
        Self::Value(Rc::new(value))
    }

    /// Get a reference to the current value.
    ///
    /// If the value is a signal, the signal's current value is returned,
    /// otherwise a [Ref::Rc] of the value is returned.
    pub fn get(&self) -> Ref<'_, T> {
        match self {
            MaybeSignal::Signal(signal) => signal.get(),
            MaybeSignal::Value(value) => Ref::Rc(value.clone()),
        }
    }

    /// Converts the [MaybeSignal] into an [BoxedSignal] if it is a [MaybeSignal::Signal].
    pub fn as_signal(&self) -> Option<BoxedSignal<T>> {
        match self {
            MaybeSignal::Signal(signal) => Some(signal.dyn_clone()),
            _ => None,
        }
    }

    /// Converts the [MaybeSignal] into a [BoxedSignal].
    ///
    /// If the value is a [MaybeSignal::Value], a [FixedSignal] is created.
    pub fn into_signal(self) -> BoxedSignal<T> {
        match self {
            MaybeSignal::Signal(signal) => signal,
            MaybeSignal::Value(value) => Box::new(FixedSignal::from(value)),
        }
    }

    /// Applies the given mapping function to the signal.
    ///
    /// Returns a [MaybeSignal] containing a [MapSignal] which maps the inner value of the signal.
    pub fn map<U: 'static>(self, map: impl Fn(Ref<T>) -> Ref<U> + 'static) -> MaybeSignal<U> {
        let signal = self.into_signal();

        MaybeSignal::signal(Box::new(MapSignal::new(signal, map)))
    }
}

impl<T: Default + 'static> Default for MaybeSignal<T> {
    fn default() -> Self {
        Self::value(T::default())
    }
}

impl<T: 'static> From<T> for MaybeSignal<T> {
    fn from(value: T) -> Self {
        Self::value(value)
    }
}

impl<T: 'static> From<BoxedSignal<T>> for MaybeSignal<T> {
    fn from(signal: BoxedSignal<T>) -> Self {
        Self::signal(signal)
    }
}

impl<'a, T: 'static> From<&'a BoxedSignal<T>> for MaybeSignal<T> {
    fn from(value: &'a BoxedSignal<T>) -> Self {
        Self::signal(value.dyn_clone())
    }
}

impl<T: 'static, U: 'static> From<MapSignal<T, U>> for MaybeSignal<U> {
    fn from(value: MapSignal<T, U>) -> Self {
        Self::signal(Box::new(value))
    }
}
