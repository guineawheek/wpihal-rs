/// now in nanoseconds.
pub fn now_default() -> i64 {
    unsafe { wpiutil_sys::WPI_NowDefault() }
}

/// Sets a new monotonic timer.
pub fn set_now_impl(now_impl: extern "C" fn() -> i64) {
    unsafe {
        wpiutil_sys::WPI_SetNowImpl(Some(now_impl));
    }
}

/// Now in nanoseconds. Monotonic.
pub fn now() -> i64 {
    unsafe { wpiutil_sys::WPI_Now() }
}

/// System time in nanoseconds since epoch. May not be monotonic.
pub fn system_time() -> i64 {
    unsafe { wpiutil_sys::WPI_GetSystemTime() }
}
