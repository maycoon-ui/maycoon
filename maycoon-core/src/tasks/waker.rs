use std::task::{RawWaker, RawWakerVTable, Waker};

/// Creates a no-op waker.
///
/// Should only be used in certain cases.
///
/// Mostly used in [crate::tasks::task::Task::take].
pub fn noop_waker() -> Waker {
    unsafe { Waker::from_raw(noop_raw_waker()) }
}

/// Create a no-op raw waker.
fn noop_raw_waker() -> RawWaker {
    RawWaker::new(
        std::ptr::null(),
        &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
    )
}

/// No-op clone.
fn clone(_: *const ()) -> RawWaker {
    noop_raw_waker()
}

/// No-op wake.
fn wake(_: *const ()) {}

/// No-op wake-by-ref.
fn wake_by_ref(_: *const ()) {}

/// No-op drop.
fn drop(_: *const ()) {}
