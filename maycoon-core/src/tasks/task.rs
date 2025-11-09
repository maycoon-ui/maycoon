use std::pin::Pin;
use std::task::{Context, Poll};

/// A task that **can** be spawned on a background thread.
///
/// The inner value requires to be [Send] and `static`.
pub trait Task<T: Send + 'static>: Future<Output = T> + Send + 'static {
    /// Returns if the task is finished or still pending.
    fn is_ready(&self) -> bool;

    /// Tries to take the inner value if the task is finished.
    ///
    /// Returns [None] if the task is still pending.
    ///
    /// This should only be called if you are sure the task is ready (check via [Task::is_ready]).
    /// Furthermore, calling this multiple times when the task is finished will probably raise a panic,
    /// so make sure to only call this once, if the task is finished.
    fn take(&mut self) -> Option<T>;
}

/// A task that is executed on the local thread.
///
/// Unlike [Task], the inner type does not need to be [Send].
pub trait LocalTask<T>: Future<Output = T> + 'static {
    /// Returns if the task is finished or still pending.
    fn is_ready(&self) -> bool;

    /// Tries to take the inner value if the task is finished.
    ///
    /// Returns [None] if the task is still pending.
    ///
    /// This should only be called if you are sure the task is ready (check via [Task::is_ready]).
    /// Furthermore, calling this multiple times when the task is finished will probably raise a panic,
    /// so make sure to only call this once, if the task is finished.
    fn take(&mut self) -> Option<T>;
}

/// A task that never completes.
///
/// This is the [Send]-safe variant of [LocalNeverTask].
#[derive(Debug)]
pub struct NeverTask<T: Send + 'static>(std::marker::PhantomData<T>);

impl<T: Send + 'static> NeverTask<T> {
    /// Creates a new [NeverTask].
    #[inline(always)]
    pub const fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<T: Send + 'static> Future for NeverTask<T> {
    type Output = T;

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl<T: Send + 'static> Task<T> for NeverTask<T> {
    #[inline(always)]
    fn is_ready(&self) -> bool {
        false
    }

    #[inline(always)]
    fn take(&mut self) -> Option<T> {
        None
    }
}

impl<T: Send + 'static> Default for NeverTask<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// A task that never completes.
///
/// This is the non-[Send]-safe variant of [NeverTask].
#[derive(Debug, Default)]
pub struct LocalNeverTask<T: 'static>(std::marker::PhantomData<T>);

impl<T: 'static> LocalNeverTask<T> {
    /// Create a new [LocalNeverTask].
    #[inline(always)]
    pub const fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<T: 'static> Future for LocalNeverTask<T> {
    type Output = T;

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl<T: 'static> LocalTask<T> for LocalNeverTask<T> {
    #[inline(always)]
    fn is_ready(&self) -> bool {
        false
    }

    #[inline(always)]
    fn take(&mut self) -> Option<T> {
        None
    }
}
