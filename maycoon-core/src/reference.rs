use std::ops::Deref;
use std::sync::RwLockReadGuard;

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
    /// A reference of a [std::cell::Ref].
    Ref(std::cell::Ref<'a, T>),
    /// A [RwLockReadGuard] reference.
    ReadGuard(RwLockReadGuard<'a, T>),
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        match self {
            Ref::Owned(value) => value,
            Ref::Borrow(value) => value,
            Ref::Ref(value) => value,
            Ref::ReadGuard(value) => value,
        }
    }
}

impl<'a, T> From<T> for Ref<'a, T> {
    #[inline(always)]
    fn from(value: T) -> Self {
        Ref::Owned(value)
    }
}

impl<'a, T> From<&'a T> for Ref<'a, T> {
    #[inline(always)]
    fn from(value: &'a T) -> Self {
        Ref::Borrow(value)
    }
}

impl<'a, T> From<std::cell::Ref<'a, T>> for Ref<'a, T> {
    #[inline(always)]
    fn from(value: std::cell::Ref<'a, T>) -> Self {
        Ref::Ref(value)
    }
}

impl<'a, T> From<RwLockReadGuard<'a, T>> for Ref<'a, T> {
    #[inline(always)]
    fn from(value: RwLockReadGuard<'a, T>) -> Self {
        Ref::ReadGuard(value)
    }
}

#[cfg(all(test, feature = "test"))]
mod tests {
    use crate::reference::Ref;
    use std::cell::RefCell;
    use std::fmt::Debug;
    use std::sync::RwLock;

    #[test_case::test_case(1; "with int")]
    #[test_case::test_case("Test".to_string(); "with string")]
    #[test_case::test_case(vec![1, 2, 3]; "with vector")]
    fn test_ref<T: Clone + PartialEq + Debug>(value: T) {
        let r_bor = Ref::Borrow(&value);
        assert_eq!(*r_bor, value);

        let r_own = Ref::Owned(value.clone());
        assert_eq!(*r_own, value);

        let refcell = RefCell::new(value.clone());
        let r_ref = Ref::Ref(refcell.borrow());
        assert_eq!(*r_ref, value);

        let rwlock = RwLock::new(value.clone());
        let r_rw = Ref::ReadGuard(rwlock.read().unwrap());
        assert_eq!(*r_rw, value);
    }

    /// Tests if the size of [Ref] is correct.
    ///
    /// Useful for not accidentally increasing the stack size of [Ref].
    #[test]
    fn test_ref_size() {
        const SIZE: usize = 24;

        assert_eq!(size_of::<Ref<'_, u8>>(), SIZE);
    }
}
