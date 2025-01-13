/// now in microseconds
pub fn now_default() -> u64 {
    unsafe { wpiutil_sys::WPI_NowDefault() }
}

pub fn shutdown_now_rio() {
    unsafe { wpiutil_sys::WPI_Impl_ShutdownNowRio(); }
}

pub fn set_now_impl(now_impl: extern "C" fn() -> u64) {
    unsafe {
        wpiutil_sys::WPI_SetNowImpl(Some(now_impl));
    }
}

pub fn now() -> u64 {
    unsafe { wpiutil_sys::WPI_Now() }
}

pub fn system_time() -> u64 {
    unsafe { wpiutil_sys::WPI_GetSystemTime() }
}