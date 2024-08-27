/// A trait to define the global state of an application.
pub trait State: 'static {}

/// A wrapper around a value factory and the value, if already computed.
///
/// A [StateVak] can be state dependent or independent.
///
/// You can either use [StateVal::new] or the `val!()` macro from the `may-macro` crate to create a new state value.
///
/// # Examples
///
/// **State Dependent**: `StateVal::new(|state| state.my_message)`.
///
/// **State Independent**: `StateVal::new(|_| "Hello World!")`.
pub struct StateVal<S: State, T> {
    factory: Box<dyn Fn(&S) -> T>,
    value: Option<T>,
}

impl<S: State, T: 'static> StateVal<S, T> {
    /// Create a new state value from a closure.
    ///
    /// It's recommended to use the `val!()` macro from the `may-macro` crate to create a new state value instead of directly calling [StateVal::new].
    pub fn new(f: impl Fn(&S) -> T + 'static) -> Self {
        Self {
            factory: Box::new(f),
            value: None,
        }
    }

    /// Computes and sets the internal value using the given state.
    pub fn compute(&mut self, state: &S) {
        self.value = Some((self.factory)(state));
    }

    /// Returns [Some(T)] if the value is already computed, [None] otherwise.
    pub fn value(self) -> Option<T> {
        self.value
    }

    /// Returns [Some(&T)] if the value is already computed, [None] otherwise.
    pub fn value_ref(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Returns [Some(&mut T)] if the value is already computed, [None] otherwise.
    pub fn value_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut()
    }

    /// Returns the factory used to compute the value.
    pub fn factory(self) -> Box<dyn Fn(&S) -> T> {
        self.factory
    }

    /// Returns the factory as a reference that is used to compute the value.
    pub fn factory_ref(&self) -> &Box<dyn Fn(&S) -> T> {
        &self.factory
    }

    /// Clears the internal value.
    ///
    /// After this, you will need to call [StateVal::compute] to recompute the value.
    pub fn clear(&mut self) {
        self.value = None;
    }

    /// Returns the computed value if it exists.
    ///
    /// Otherwise, computes the value and returns it.
    pub fn get(mut self, state: &S) -> T {
        if self.value.is_none() {
            self.compute(state);
        }

        self.value.unwrap()
    }

    /// Returns a reference to the computed value if it exists.
    ///
    /// Otherwise, computes the value and returns a reference to it.
    pub fn get_ref(&mut self, state: &S) -> &T {
        self.compute(state);

        self.value.as_ref().unwrap()
    }

    /// Maps the value using the given function.
    ///
    /// This is similar to [Option::map] and other mapping functions.
    pub fn map<N: 'static>(self, f: impl Fn(T) -> N + 'static) -> StateVal<S, N> {
        StateVal::new(move |state| f((self.factory)(state)))
    }
}
