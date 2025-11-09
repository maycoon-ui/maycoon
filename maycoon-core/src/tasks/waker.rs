use std::task::{RawWaker, RawWakerVTable, Waker};

/// Creates a no-op waker.
///
/// Should only be used in certain cases.
///
/// Mostly used in [crate::tasks::task::Task::take].
#[inline(always)]
pub fn noop_waker() -> Waker {
    unsafe { Waker::from_raw(noop_raw_waker()) }
}

/// Create a no-op raw waker.
#[inline(always)]
fn noop_raw_waker() -> RawWaker {
    RawWaker::new(
        std::ptr::null(),
        &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
    )
}

/// No-op clone.
#[inline(always)]
fn clone(_: *const ()) -> RawWaker {
    noop_raw_waker()
}

/// No-op wake.
#[inline(always)]
fn wake(_: *const ()) {}

/// No-op wake-by-ref.
#[inline(always)]
fn wake_by_ref(_: *const ()) {}

/// No-op drop.
#[inline(always)]
fn drop(_: *const ()) {}
