use std::thread::JoinHandle;

#[cfg(unix)]
use std::os::unix::thread::JoinHandleExt;

#[allow(unused)]
use wpihal_sys::{
    HAL_GetCurrentThreadPriority, HAL_GetThreadPriority, HAL_SetCurrentThreadPriority,
    HAL_SetThreadPriority, NativeThreadHandle,
};

use crate::error::HALResult;
use crate::hal_retcall;

/// Gets thread priority.
/// No-op on windows.
#[cfg(unix)]
pub fn get_thread_priority<T>(handle: &JoinHandle<T>) -> HALResult<i32> {
    hal_retcall!(HAL_GetThreadPriority(handle.as_pthread_t() as _; -> i32;))
}
/// Gets thread priority.
/// No-op on windows.
#[cfg(not(unix))]
pub fn get_thread_priority<T>(_handle: &JoinHandle<T>) -> HALResult<i32> {
    Ok(0)
}

/// Gets current thread priority.
pub fn get_current_thread_priority() -> HALResult<i32> {
    hal_retcall!(HAL_GetCurrentThreadPriority(;-> i32;))
}

/// Sets thread priority.
/// No-op on windows.
#[cfg(unix)]
pub fn set_thread_priority<T>(handle: &JoinHandle<T>, priority: i32) -> HALResult<()> {
    hal_retcall!(HAL_SetThreadPriority(handle.as_pthread_t() as _, priority))
}

/// Sets thread priority.
/// No-op on windows.
#[cfg(not(unix))]
pub fn set_thread_priority<T>(_handle: &JoinHandle<T>) -> HALResult<()> {
    Ok()
}

pub fn set_current_thread_priority(priority: i32) -> HALResult<()> {
    hal_retcall!(HAL_SetCurrentThreadPriority(priority))
}
