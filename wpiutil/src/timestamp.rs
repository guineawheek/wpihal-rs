/// now in microseconds
pub fn now_default() -> u64 {
    unsafe { wpiutil_sys::WPI_NowDefault() }
}

/// Sets a new monotonic timer. 
pub fn set_now_impl(now_impl: extern "C" fn() -> u64) {
    unsafe {
        wpiutil_sys::WPI_SetNowImpl(Some(now_impl));
    }
}

/// Now in micros. Monotonic.
pub fn now() -> u64 {
    unsafe { wpiutil_sys::WPI_Now() }
}

/// System time in microseconds since epoch. May not be monotonic.
pub fn system_time() -> u64 {
    unsafe { wpiutil_sys::WPI_GetSystemTime() }
}