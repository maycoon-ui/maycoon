/// A trait to define the global state of an application.
pub trait State: 'static {}

/// An empty state. Useful for testing and examples.
pub struct EmptyState;

impl State for EmptyState {}

/// A value that's either dependent on the state or independent.
///
/// Use [Val::new_state] or the `val!()` macro from the `maycoon-macros` crate to create a new state dependent value.
///
/// Use [Val::new_val] or just `YourValue.into()` to create an independent value. You can also use `val!()`, because why not?
///
/// **NOTE for Widget Developers:** Inside the [Widget::update] method, all values must be invalidated using [Val::invalidate].
///
/// [Widget::update]: crate::widget::Widget::update
pub enum Val<S: State, T: 'static> {
    /// A state dependent value.
    ///
    /// The inner value may need to be re-computed, when invalid.
    State {
        /// The factory that produces the inner value.
        ///
        /// This is called in order to "compute" the value `T`.
        factory: Box<dyn Fn(&S) -> T>,
        /// The inner value.
        ///
        /// If computed (valid), this is `Some(T)`. If not computed (invalid), this is `None`.
        value: Option<T>,
    },

    /// An independent value.
    ///
    /// The inner value will always be the same and does not need any re-computation.
    ///
    /// More performant than state dependent values.
    Val(T),
}

impl<S: State, T: 'static> Val<S, T> {
    /// Create a new state dependent value from the given closure.
    pub fn new_state(factory: impl Fn(&S) -> T + 'static) -> Self {
        Self::State {
            factory: Box::new(factory),
            value: None,
        }
    }

    /// Create a new state independent value.
    pub fn new_val(value: T) -> Self {
        Self::Val(value)
    }

    /// Get the inner value of this [Val].
    ///
    /// This will compute the value if it's state dependent and not yet computed, so it will always return something.
    pub fn get(mut self, state: &S) -> T {
        if self.invalid() {
            self.compute(state);
        }

        self.value().unwrap()
    }

    /// Get a reference to the inner value of this [Val].
    ///
    /// This will compute the value if it's state dependent and not yet computed, so it will always return something.
    pub fn get_ref(&mut self, state: &S) -> &T {
        if self.invalid() {
            self.compute(state);
        }

        self.value_ref().unwrap()
    }

    /// Get a mutable reference to the inner value of this [Val].
    ///
    /// This will compute the value if it's state dependent and not yet computed, so it will always return something.
    pub fn get_mut(&mut self, state: &S) -> &mut T {
        if self.invalid() {
            self.compute(state);
        }

        self.value_mut().unwrap()
    }

    /// If the inner value is state dependent, compute it using the given state.
    pub fn compute(&mut self, state: &S) {
        match self {
            Val::State { factory, value } => {
                *value = Some(factory(state));
            },

            Val::Val(_) => (),
        }
    }

    /// Returns if the inner value is state dependent and not yet computed.
    pub fn invalid(&self) -> bool {
        self.value_ref().is_none()
    }

    /// Invalidates the inner value, if it's state dependent.
    ///
    /// This makes [Val::get] and other methods re-compute the value (if it's state dependent).
    pub fn invalidate(&mut self) {
        match self {
            Val::State { value, .. } => *value = None,
            Val::Val(_) => (),
        }
    }

    /// Returns the inner value or [None] if it's state dependent and not yet computed.
    pub fn value(self) -> Option<T> {
        match self {
            Val::State { value, .. } => value,
            Val::Val(v) => Some(v),
        }
    }

    /// Returns a reference to the inner value or [None] if it's state dependent and not yet computed.
    pub fn value_ref(&self) -> Option<&T> {
        match self {
            Val::State { value, .. } => value.as_ref(),
            Val::Val(v) => Some(v),
        }
    }

    /// Returns a mutable reference to the inner value or [None] if it's state dependent and not yet computed.
    pub fn value_mut(&mut self) -> Option<&mut T> {
        match self {
            Val::State { value, .. } => value.as_mut(),
            Val::Val(v) => Some(v),
        }
    }

    /// Applies a function to the inner value, transforming it into another value.
    ///
    /// Similar to the [Option::map] method.
    pub fn map<U, F: Fn(T) -> U + 'static>(self, f: F) -> Val<S, U> {
        match self {
            Val::State { factory, .. } => Val::<S, U>::State {
                factory: Box::new(move |state| f(factory(state))),
                value: None,
            },

            Val::Val(val) => Val::Val(f(val)),
        }
    }
}

impl<S: State, T> From<T> for Val<S, T> {
    fn from(value: T) -> Self {
        Self::new_val(value)
    }
}
