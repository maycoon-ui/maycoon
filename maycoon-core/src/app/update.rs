use bitflags::bitflags;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

bitflags! {
    /// Update bitflags to define which part of the App should Update.
    ///
    /// Possible values:
    /// - **EVAL** - Re-evaluate the widget tree.
    /// - **DRAW** - Re-draw the widget tree.
    /// - **LAYOUT** - Re-layout the widget tree.
    /// - **FORCE** - Force the App to re-evaluate, re-draw and re-layout the widget tree.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Update: u8 {
        /// Re-evaluate the widget tree.
        const EVAL   = 0b00000001;
        /// Re-draw the widget tree.
        const DRAW   = 0b00000010;
        /// Re-layout the widget tree.
        const LAYOUT = 0b00000100;
        /// Force the App to re-evaluate, re-draw and re-layout the widget tree.
        const FORCE  = 0b00001000;
    }
}

/// Manages updates for the application lifecycle.
///
/// It's using atomic operations to ensure lockless thread-safety.
#[derive(Clone, Debug)]
pub struct UpdateManager {
    update: Arc<AtomicU8>,
}

impl UpdateManager {
    /// Creates a new `UpdateManager`.
    pub fn new() -> Self {
        Self {
            update: Arc::new(AtomicU8::new(0)),
        }
    }

    /// Inserts the given `Update` into the `UpdateManager` using bitwise OR.
    pub fn insert(&self, update: Update) {
        self.update.fetch_or(update.bits(), Ordering::AcqRel);
    }

    /// Removes the given `Update` from the `UpdateManager` using bitwise AND.
    pub fn remove(&self, update: Update) {
        self.update.fetch_and(!update.bits(), Ordering::AcqRel);
    }

    /// Returns the current `Update` of the `UpdateManager`.
    pub fn get(&self) -> Update {
        Update::from_bits(self.update.load(Ordering::Acquire))
            .expect("failed to decode update bits")
    }

    /// Sets the current `Update` of the `UpdateManager`.
    pub fn set(&self, update: Update) {
        self.update.store(update.bits(), Ordering::Release);
    }

    /// Clears the current `Update` flags of the `UpdateManager`.
    pub fn clear(&self) {
        self.update.store(0, Ordering::Release);
    }
}

impl Default for UpdateManager {
    fn default() -> Self {
        Self::new()
    }
}
