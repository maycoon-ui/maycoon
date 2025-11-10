use crate::reference::Ref;
use rpds::{Vector, VectorSync};
use std::rc::Rc;
use std::sync::Arc;

/// A collection of listeners.
///
/// This is basically just a wrapper around `Vec<Listener<T>>>`.
pub struct ListenerRegister<T> {
    listeners: Vector<Listener<T>>,
}

impl<T> ListenerRegister<T> {
    /// Create a new empty listener register.
    ///
    /// By default, the listener register has a capacity of 1,
    /// meaning that it can hold one listener without reallocating.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            listeners: Vector::new(),
        }
    }

    /// Register a new listener.
    #[inline(always)]
    pub fn register(self, listener: Listener<T>) -> Self {
        Self {
            listeners: self.listeners.push_back(listener),
        }
    }

    /// Notify all listeners with the values produced by `factory`.
    #[inline(always)]
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
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for ListenerRegister<T> {
    #[inline(always)]
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
    listeners: VectorSync<SendListener<T>>,
}

impl<T> SendListenerRegistry<T> {
    /// Create a new empty listener register.
    ///
    /// By default, the listener register has a capacity of 1,
    /// meaning that it can hold one listener without reallocating.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            listeners: VectorSync::new_sync(),
        }
    }

    /// Register a new listener.
    #[inline(always)]
    pub fn register(self, listener: SendListener<T>) -> Self {
        Self {
            listeners: self.listeners.push_back(listener),
        }
    }

    /// Notify all listeners with the values produced by `factory`.
    #[inline(always)]
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
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for SendListenerRegistry<T> {
    #[inline(always)]
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
    #[inline(always)]
    pub fn new(function: impl Fn(Ref<'_, T>) + 'static) -> Self {
        Self {
            function: Rc::new(function),
        }
    }

    /// Create a new listener from a raw reference counted function.
    #[inline(always)]
    pub fn from_rc(function: Rc<dyn Fn(Ref<'_, T>)>) -> Self {
        Self { function }
    }

    /// Call the listener with a value.
    #[inline(always)]
    pub fn call(&self, value: Ref<'_, T>) {
        (self.function)(value)
    }
}

impl<T> Default for Listener<T> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            function: Rc::new(|_| {}),
        }
    }
}

impl<T> Clone for Listener<T> {
    #[inline(always)]
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
    #[inline(always)]
    pub fn new(function: impl Fn(Ref<'_, T>) + 'static) -> Self {
        Self {
            function: Arc::new(function),
        }
    }

    /// Create a new listener from a raw atomically reference counted function.
    #[inline(always)]
    pub fn from_arc(function: Arc<dyn Fn(Ref<'_, T>)>) -> Self {
        Self { function }
    }

    /// Call the listener with a value.
    #[inline(always)]
    pub fn call(&self, value: Ref<'_, T>) {
        (self.function)(value)
    }
}

impl<T> Default for SendListener<T> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            function: Arc::new(|_| {}),
        }
    }
}

impl<T> Clone for SendListener<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            function: self.function.clone(),
        }
    }
}
