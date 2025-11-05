use crate::reference::Ref;
use std::rc::Rc;
use std::sync::Arc;

/// A collection of listeners.
///
/// This is basically just a wrapper around `Vec<Listener<T>>>`.
pub struct ListenerRegister<T> {
    listeners: Vec<Listener<T>>,
}

impl<T> ListenerRegister<T> {
    /// Create a new empty listener register.
    ///
    /// By default, the listener register has a capacity of 1,
    /// meaning that it can hold one listener without reallocating.
    pub fn new() -> Self {
        Self {
            listeners: Vec::with_capacity(1),
        }
    }

    /// Register a new listener.
    pub fn register(&mut self, listener: Listener<T>) {
        self.listeners.push(listener);
    }

    /// Notify all listeners with the values produced by `factory`.
    pub fn notify<'a>(&self, factory: impl Fn() -> Ref<'a, T>)
    where
        T: 'a,
    {
        for listener in &self.listeners {
            listener.call(factory());
        }
    }
}

impl<T> Default for ListenerRegister<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for ListenerRegister<T> {
    fn clone(&self) -> Self {
        Self {
            listeners: self.listeners.clone(),
        }
    }
}

/// A collection of listeners that can be sent to other threads.
///
/// This is basically just a wrapper around `Vec<SendListener<T>>>`.
pub struct SendListenerRegistry<T> {
    listeners: Vec<SendListener<T>>,
}

impl<T> SendListenerRegistry<T> {
    /// Create a new empty listener register.
    ///
    /// By default, the listener register has a capacity of 1,
    /// meaning that it can hold one listener without reallocating.
    pub fn new() -> Self {
        Self {
            listeners: Vec::with_capacity(1),
        }
    }

    /// Register a new listener.
    pub fn register(&mut self, listener: SendListener<T>) {
        self.listeners.push(listener);
    }

    /// Notify all listeners with the values produced by `factory`.
    pub fn notify<'a>(&self, factory: impl Fn() -> Ref<'a, T>)
    where
        T: 'a,
    {
        for listener in &self.listeners {
            listener.call(factory());
        }
    }
}

impl<T> Default for SendListenerRegistry<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for SendListenerRegistry<T> {
    fn clone(&self) -> Self {
        Self {
            listeners: self.listeners.clone(),
        }
    }
}

/// A listener that listens to a signal value.
///
/// This is basically just a wrapper around `Rc<dyn Fn(Ref<'_, T>)>>`.
///
/// This type is not `Send` and therefore cannot be sent to other threads.
///
/// It trades less overhead for not being `Send`.
///
/// Use [SendListener] if you need a `Send`-safe listener.
pub struct Listener<T> {
    function: Rc<dyn Fn(Ref<'_, T>)>,
}

impl<T> Listener<T> {
    /// Create a new listener.
    pub fn new(function: impl Fn(Ref<'_, T>) + 'static) -> Self {
        Self {
            function: Rc::new(function),
        }
    }

    /// Create a new listener from a raw reference counted function.
    pub fn from_rc(function: Rc<dyn Fn(Ref<'_, T>)>) -> Self {
        Self { function }
    }

    /// Call the listener with a value.
    pub fn call(&self, value: Ref<'_, T>) {
        (self.function)(value)
    }
}

impl<T> Default for Listener<T> {
    fn default() -> Self {
        Self {
            function: Rc::new(|_| {}),
        }
    }
}

impl<T> Clone for Listener<T> {
    fn clone(&self) -> Self {
        Self {
            function: self.function.clone(),
        }
    }
}

/// A listener that listens to a signal value.
///
/// This is basically just a wrapper around `Arc<dyn Fn(Ref<'_, T>)>>`.
///
/// This type is `Send` and therefore can be sent to other threads.
///
/// It trades more overhead for being `Send`.
///
/// Use [Listener] if you don't need a `Send`-safe listener.
pub struct SendListener<T> {
    function: Arc<dyn Fn(Ref<'_, T>)>,
}

impl<T> SendListener<T> {
    /// Create a new listener.
    pub fn new(function: impl Fn(Ref<'_, T>) + 'static) -> Self {
        Self {
            function: Arc::new(function),
        }
    }

    /// Create a new listener from a raw atomically reference counted function.
    pub fn from_arc(function: Arc<dyn Fn(Ref<'_, T>)>) -> Self {
        Self { function }
    }

    /// Call the listener with a value.
    pub fn call(&self, value: Ref<'_, T>) {
        (self.function)(value)
    }
}

impl<T> Default for SendListener<T> {
    fn default() -> Self {
        Self {
            function: Arc::new(|_| {}),
        }
    }
}

impl<T> Clone for SendListener<T> {
    fn clone(&self) -> Self {
        Self {
            function: self.function.clone(),
        }
    }
}
