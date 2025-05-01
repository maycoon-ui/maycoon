use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};

/// Represents a reference to a value of type `T`.
///
/// Due to Rust's temporal borrowing rules,
/// returning a reference to a value may not be always possible,
/// so this enum is used to represent one by having multiple variants for multiple types of references.
pub enum Ref<'a, T> {
    /// An owned value. Useful for [Copy] types.
    Owned(T),
    /// A borrowed reference.
    Borrow(&'a T),
    /// An [Arc] reference.
    Arc(Arc<T>),
    /// A [RwLockReadGuard] reference.
    ReadGuard(RwLockReadGuard<'a, T>),
    /// A [parking_lot::RwLockReadGuard] reference.
    ParkingLotReadGuard(parking_lot::RwLockReadGuard<'a, T>),
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Ref::Owned(value) => value,
            Ref::Borrow(value) => value,
            Ref::Arc(value) => value,
            Ref::ReadGuard(value) => value,
            Ref::ParkingLotReadGuard(value) => value,
        }
    }
}

impl<'a, T> From<T> for Ref<'a, T> {
    fn from(value: T) -> Self {
        Ref::Owned(value)
    }
}

impl<'a, T> From<&'a T> for Ref<'a, T> {
    fn from(value: &'a T) -> Self {
        Ref::Borrow(value)
    }
}

impl<'a, T> From<Arc<T>> for Ref<'a, T> {
    fn from(value: Arc<T>) -> Self {
        Ref::Arc(value)
    }
}

impl<'a, T> From<RwLockReadGuard<'a, T>> for Ref<'a, T> {
    fn from(value: RwLockReadGuard<'a, T>) -> Self {
        Ref::ReadGuard(value)
    }
}

impl<'a, T> From<parking_lot::RwLockReadGuard<'a, T>> for Ref<'a, T> {
    fn from(value: parking_lot::RwLockReadGuard<'a, T>) -> Self {
        Ref::ParkingLotReadGuard(value)
    }
}

/// Represents a mutable reference to a value of type `T`.
///
/// Due to Rust's temporal borrowing rules,
/// returning a reference to a value may not be always possible,
/// so this enum is used to represent one by having multiple variants for multiple types of mutable references.
pub enum MutRef<'a, T> {
    /// A borrowed mutable reference.
    Borrow(&'a mut T),
    /// A [RwLockWriteGuard] mutable reference.
    WriteGuard(RwLockWriteGuard<'a, T>),
    /// A [parking_lot::RwLockWriteGuard] mutable reference.
    ParkingLotWriteGuard(parking_lot::RwLockWriteGuard<'a, T>),
}

impl<'a, T> Deref for MutRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            MutRef::Borrow(value) => value,
            MutRef::WriteGuard(value) => value,
            MutRef::ParkingLotWriteGuard(value) => value,
        }
    }
}

impl<'a, T> DerefMut for MutRef<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MutRef::Borrow(value) => value,
            MutRef::WriteGuard(value) => value,
            MutRef::ParkingLotWriteGuard(value) => value,
        }
    }
}

impl<'a, T> From<&'a mut T> for MutRef<'a, T> {
    fn from(value: &'a mut T) -> Self {
        MutRef::Borrow(value)
    }
}

impl<'a, T> From<RwLockWriteGuard<'a, T>> for MutRef<'a, T> {
    fn from(value: RwLockWriteGuard<'a, T>) -> Self {
        MutRef::WriteGuard(value)
    }
}

impl<'a, T> From<parking_lot::RwLockWriteGuard<'a, T>> for MutRef<'a, T> {
    fn from(value: parking_lot::RwLockWriteGuard<'a, T>) -> Self {
        MutRef::ParkingLotWriteGuard(value)
    }
}
