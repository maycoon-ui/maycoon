use crate::tasks::waker;
use std::task::{Context, Poll};

/// A task that **can** be spawned on a background thread.
///
/// The inner value requires to be [Send] and static (and optionally [Unpin]).
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
    fn take(&mut self) -> Option<T>
    where
        Self: Unpin,
    {
        let pinned = std::pin::Pin::new(self);
        let waker = waker::noop_waker();
        let mut cx = Context::from_waker(&waker);

        match pinned.poll(&mut cx) {
            Poll::Ready(val) => Some(val),
            Poll::Pending => None,
        }
    }
}

/// A task that is executed on the local thread.
///
/// Unlike [Task], the inner type does not need to be [Send].
pub trait LocalTask<T>: Future<Output = T> {
    /// Returns if the task is finished or still pending.
    fn is_ready(&self) -> bool;

    /// Tries to take the inner value if the task is finished.
    ///
    /// Returns [None] if the task is still pending.
    ///
    /// This should only be called if you are sure the task is ready (check via [Task::is_ready]).
    /// Furthermore, calling this multiple times when the task is finished will probably raise a panic,
    /// so make sure to only call this once, if the task is finished.
    fn take(&mut self) -> Option<T>
    where
        Self: Unpin,
    {
        let pinned = std::pin::Pin::new(self);
        let waker = waker::noop_waker();
        let mut cx = Context::from_waker(&waker);

        match pinned.poll(&mut cx) {
            Poll::Ready(val) => Some(val),
            Poll::Pending => None,
        }
    }
}
