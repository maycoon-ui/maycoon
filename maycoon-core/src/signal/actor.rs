use crate::reference::{MutRef, Ref};
use crate::signal::rw::RwSignal;
use crate::signal::{Listener, Signal};
use parking_lot::Mutex;

/// An action to run on the inner value.
pub type Action<T> = Box<dyn Fn(MutRef<T>)>;

/// A signal to run actions on the inner value if unlocked.
///
/// The issue with [RwSignal] is that it may cause deadlocks if not used properly.
/// [ActorSignal] tries to fix this by running actions you push beforehand as soon as the signal is unlocked and the value requires an update.
/// This avoids deadlocks, but can still cause the pushed actions to not run at all, if the signal is constantly locked.
pub struct ActorSignal<T> {
    rw: RwSignal<T>,
    actions: Mutex<Vec<Action<T>>>,
}

impl<T> ActorSignal<T> {
    /// Creates a new [ActorSignal] with the given value.
    pub fn new(value: T) -> Self {
        Self {
            rw: RwSignal::new(value),
            actions: Mutex::new(Vec::new()),
        }
    }

    /// Get a mutable reference to the value.
    ///
    /// This calls [ActorSignal::run] if the inner value is not locked yet and returns the final value.
    pub fn get_mut(&self) -> MutRef<T> {
        let value = self.rw.get_mut();

        if !self.is_locked() {
            self.run();
        }

        value
    }

    /// Check if the inner value is locked.
    pub fn is_locked(&self) -> bool {
        self.rw.is_locked()
    }

    /// Push an action to run on the inner value.
    ///
    /// The action will be run as soon as the inner value is unlocked and requires an update.
    pub fn act(&self, action: Action<T>) {
        self.actions.lock().push(action);
    }

    /// Run all pushed actions.
    ///
    /// This can cause deadlocks if [Self::is_locked] returns true.
    pub fn run(&self) {
        for action in self.actions.lock().drain(..) {
            action(self.get_mut());
        }
    }
}

impl<T: 'static> Signal<T> for ActorSignal<T> {
    fn get(&self) -> Ref<T> {
        if !self.is_locked() {
            self.run();
        }

        self.rw.get()
    }

    fn set_value(&self, value: T) {
        self.rw.set_value(value);
    }

    fn listen(&mut self, listener: Listener<T>) {
        self.rw.listen(listener);
    }

    fn notify(&self) {
        self.rw.notify();
    }
}
