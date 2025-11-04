use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, RwLockReadGuard};

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
    /// A reference counted reference.
    Rc(Rc<T>),
    /// A reference of a [std::cell::Ref].
    Ref(std::cell::Ref<'a, T>),
    /// An [Arc] reference.
    Arc(Arc<T>),
    /// A [RwLockReadGuard] reference.
    ReadGuard(RwLockReadGuard<'a, T>),
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Ref::Owned(value) => value,
            Ref::Borrow(value) => value,
            Ref::Rc(value) => value,
            Ref::Ref(value) => value,
            Ref::Arc(value) => value,
            Ref::ReadGuard(value) => value,
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

impl<'a, T> From<Rc<T>> for Ref<'a, T> {
    fn from(value: Rc<T>) -> Self {
        Ref::Rc(value)
    }
}

impl<'a, T> From<std::cell::Ref<'a, T>> for Ref<'a, T> {
    fn from(value: std::cell::Ref<'a, T>) -> Self {
        Ref::Ref(value)
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
