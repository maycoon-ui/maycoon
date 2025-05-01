use crate::reference::{MutRef, Ref};
use crate::signal::{Listener, Signal};
use parking_lot::RwLock;

/// A signal based on [RwLock] to load/store a value and notify listeners when it changes.
///
/// The difference between this signal and [StateSignal] is that this signal can be directly mutated using [get_mut].
/// However, this introduces extra overhead and the same risks as [RwLock].
pub struct RwSignal<T> {
    lock: RwLock<T>,
    listeners: Vec<Listener<T>>,
}

impl<T> RwSignal<T> {
    /// Create a new [RwSignal] with the given value.
    pub fn new(value: T) -> Self {
        Self {
            lock: RwLock::new(value),
            listeners: Vec::with_capacity(1),
        }
    }

    /// Get a mutable reference to the value.
    ///
    /// **NOTE:** Multiple mutable reference cannot co-exist and will lead to deadlocks.
    pub fn get_mut(&self) -> MutRef<T> {
        MutRef::ParkingLotWriteGuard(self.lock.write())
    }

    /// Check if the signal is locked.
    pub fn is_locked(&self) -> bool {
        self.lock.is_locked()
    }
}

impl<T: 'static> Signal<T> for RwSignal<T> {
    fn get(&self) -> Ref<T> {
        Ref::ParkingLotReadGuard(self.lock.read())
    }

    fn set_value(&self, value: T) {
        *self.get_mut() = value;
    }

    fn listen(&mut self, listener: Listener<T>) {
        self.listeners.push(listener);
    }

    fn notify(&self) {
        for listener in &self.listeners {
            listener(self.get());
        }
    }
}
