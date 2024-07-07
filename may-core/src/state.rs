/// A trait to define the global state of an application.
pub trait State: 'static {}

/// A wrapper around a closure that takes a reference to a [State] and returns a value.
///
/// The returned value may be dependent or independent of the given state object.
///
/// You can either use [StateVal::new] or the `val!()` macro from the `may-macro` crate to create a new state value.
///
/// # Examples
///
/// **State Dependent**: `StateVal::new(|state| state.my_message)`.
///
/// **State Independent**: `StateVal::new(|_| "Hello World!")`.
pub struct StateVal<S: State, T>(Box<dyn Fn(&S) -> T>);

impl<S: State, T: 'static> StateVal<S, T> {
    /// Create a new state value from a closure.
    ///
    /// It's recommended to use the `val!()` macro from the `may-macro` crate to create a new state value instead of directly calling [StateVal::new].
    pub fn new(f: impl Fn(&S) -> T + 'static) -> Self {
        Self(Box::new(f))
    }

    /// Get the value using the given state.
    ///
    /// Use this for widget building and not direct app logic.
    pub fn get(&self, state: &S) -> T {
        self.0(state)
    }

    /// Maps the state value to the given type using the given closure.
    ///
    /// This is similar to [Option::map] and can be used to transform the state value to a different type.
    pub fn map<N: 'static>(self, f: impl Fn(T) -> N + 'static) -> StateVal<S, N> {
        StateVal::<S, N>::new(move |state| f(self.get(state)))
    }
}
