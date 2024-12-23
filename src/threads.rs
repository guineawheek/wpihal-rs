
use std::thread::JoinHandle;
#[cfg(unix)]
use std::os::unix::thread::JoinHandleExt;

use wpihal_sys::{HAL_GetCurrentThreadPriority, HAL_GetThreadPriority, HAL_SetCurrentThreadPriority, HAL_SetThreadPriority, NativeThreadHandle};

use crate::error::HALResult;
use crate::hal_call;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadPriority {
    pub priority: i32,
    pub real_time: bool,
}

/// Gets thread priority.
/// No-op on windows.
#[cfg(unix)]
pub fn get_thread_priority<T>(handle: &JoinHandle<T>) -> HALResult<ThreadPriority> {
    let pthread_t = handle.as_pthread_t() as NativeThreadHandle;
    let mut is_real_time: i32 = 0;
    let priority = hal_call!(HAL_GetThreadPriority(pthread_t, &mut is_real_time))?;

    Ok(ThreadPriority { priority, real_time: is_real_time != 0 })
}
#[cfg(not(unix))]
pub fn get_thread_priority<T>(handle: &JoinHandle<T>) -> HALResult<ThreadPriority> {
    Ok(ThreadPriority { priority: 0, real_time: false })
}

pub fn get_current_thread_priority() -> HALResult<ThreadPriority> {
    let mut is_real_time: i32 = 0;
    let priority = hal_call!(HAL_GetCurrentThreadPriority(&mut is_real_time))?;

    Ok(ThreadPriority { priority, real_time: is_real_time != 0 })
}

/// Sets thread priority.
/// No-op on windows.
pub fn set_thread_priority<T>(handle: &JoinHandle<T>, priority: ThreadPriority) -> HALResult<bool> {
    let pthread_t = handle.as_pthread_t() as NativeThreadHandle;
    Ok(hal_call!(HAL_SetThreadPriority(pthread_t, priority.real_time as i32, priority.priority))? != 0)
}

#[cfg(not(unix))]
pub fn set_thread_priority<T>(_handle: &JoinHandle<T>) -> HALResult<bool> {
    Ok(true)
}

pub fn set_current_thread_priority(priority: ThreadPriority) -> HALResult<bool> {
    Ok(hal_call!(HAL_SetCurrentThreadPriority(priority.real_time as i32, priority.priority))? != 0)
}