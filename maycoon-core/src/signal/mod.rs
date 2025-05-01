use crate::app::context::AppContext;
use crate::reference::Ref;
use crate::signal::fixed::FixedSignal;
use crate::signal::map::MapSignal;
use std::sync::Arc;

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

/// Contains the [rw::RwSignal] signal.
pub mod rw;

/// Contains the [actor::ActorSignal] signal.
pub mod actor;

/// Listener function for [Signal].
pub type Listener<T> = Box<dyn Fn(Ref<T>)>;

/// [Arc] reference to a [Signal].
pub type ArcSignal<T> = Arc<dyn Signal<T>>;

/// Base signal trait.
///
/// Signals store values of type `T` and notify listeners when they change.
///
/// **NOTE:** By default, signals don't have any listeners. To "hook" a signal into the application cycle, call [use_signal].
///
/// [use_signal]: AppContext::use_signal
pub trait Signal<T: 'static>: 'static {
    /// Get a reference to the current value of the signal.
    fn get(&self) -> Ref<T>;

    /// Set the value of the signal.
    ///
    /// **NOTE:** This does not notify listeners, use [set] instead.
    fn set_value(&self, value: T);

    /// Add a listener to the signal, which will be called when the signal's value changes.
    fn listen(&mut self, listener: Listener<T>);

    /// Notify listeners that the signal's value has changed.
    /// May also be called manually to update listeners.
    fn notify(&self);

    /// Set the value of the signal and notify listeners.
    fn set(&self, value: T) {
        self.set_value(value);
        self.notify();
    }

    /// Converts the signal into a [MaybeSignal].
    fn maybe(self: &Arc<Self>) -> MaybeSignal<T>
    where
        Self: Sized,
    {
        MaybeSignal::signal(self.clone())
    }

    /// Converts this signal into a [MaybeSignal] and applies the given mapping function.
    ///
    /// See [MaybeSignal::map] for more.
    fn map<U: 'static>(self: Arc<Self>, map: impl Fn(Ref<T>) -> Ref<U> + 'static) -> MaybeSignal<U>
    where
        Self: Sized,
    {
        self.maybe().map(map)
    }

    /// Hooks the signal into the given [AppContext].
    ///
    /// Required for the signal to become reactive with the app lifecycle.
    fn hook(self, context: &AppContext) -> Arc<Self>
    where
        Self: Sized,
    {
        context.use_signal(self)
    }
}

/// A value which may be a signal or a fixed value.
#[derive(Clone)]
pub enum MaybeSignal<T> {
    /// A signal.
    Signal(ArcSignal<T>),
    /// A fixed value wrapped inside an [Arc].
    Value(Arc<T>),
}

impl<T: 'static> MaybeSignal<T> {
    /// Wrap a [Signal] inside a [MaybeSignal].
    pub fn signal(signal: ArcSignal<T>) -> Self {
        Self::Signal(signal)
    }

    /// Wrap a value inside a [MaybeSignal].
    pub fn value(value: T) -> Self {
        Self::Value(Arc::new(value))
    }

    /// Get a reference to the current value.
    ///
    /// If the value is a signal, the signal's current value is returned,
    /// otherwise a [Ref::Arc] of the value is returned.
    pub fn get(&self) -> Ref<T> {
        match self {
            MaybeSignal::Signal(signal) => signal.get(),
            MaybeSignal::Value(value) => Ref::Arc(value.clone()),
        }
    }

    /// Converts the [MaybeSignal] into a [ArcSignal].
    ///
    /// If the value is a [MaybeSignal::Value], a [FixedSignal] is created.
    pub fn into_signal(self) -> ArcSignal<T> {
        match self {
            MaybeSignal::Signal(signal) => signal,
            MaybeSignal::Value(value) => {
                Arc::new(FixedSignal::new(Arc::into_inner(value).unwrap()))
            },
        }
    }

    /// Converts the [MaybeSignal] into an [ArcSignal] if it is a [MaybeSignal::Signal].
    pub fn as_signal(&self) -> Option<ArcSignal<T>> {
        match self {
            MaybeSignal::Signal(signal) => Some(signal.clone()),
            _ => None,
        }
    }

    /// Applies the given mapping function to the signal.
    ///
    /// Returns a [MaybeSignal] containing a [MapSignal] which maps the inner value of the signal.
    pub fn map<U: 'static>(self, map: impl Fn(Ref<T>) -> Ref<U> + 'static) -> MaybeSignal<U> {
        let signal = self.into_signal();

        MaybeSignal::signal(Arc::new(MapSignal::new(signal, map)))
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

impl<T: 'static> From<ArcSignal<T>> for MaybeSignal<T> {
    fn from(signal: ArcSignal<T>) -> Self {
        Self::signal(signal)
    }
}

impl<'a, T: 'static> From<&'a ArcSignal<T>> for MaybeSignal<T> {
    fn from(value: &'a ArcSignal<T>) -> Self {
        Self::signal(value.clone())
    }
}
