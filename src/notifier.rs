use std::ffi::CStr;

use wpihal_sys::{
    HAL_CancelNotifierAlarm, HAL_CleanNotifier, HAL_InitializeNotifier, HAL_NotifierHandle,
    HAL_SetNotifierName, HAL_SetNotifierThreadPriority, HAL_StopNotifier, HAL_UpdateNotifierAlarm,
    HAL_WaitForNotifierAlarm,
};

use crate::{error::HALResult, hal_call};

pub struct Notifier(HAL_NotifierHandle);

impl Notifier {
    pub fn initialize() -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeNotifier())?))
    }

    pub fn set_thread_priority(real_time: bool, priority: i32) -> HALResult<bool> {
        Ok(hal_call!(HAL_SetNotifierThreadPriority(real_time as i32, priority))? != 0)
    }

    pub fn set_name(&mut self, name: &CStr) -> HALResult<()> {
        hal_call!(HAL_SetNotifierName(self.0, name.as_ptr()))
    }

    pub fn stop(&mut self) -> HALResult<()> {
        hal_call!(HAL_StopNotifier(self.0))
    }

    pub fn update_alarm(&mut self, trigger_time: u64) -> HALResult<()> {
        hal_call!(HAL_UpdateNotifierAlarm(self.0, trigger_time))
    }

    pub fn cancel_alarm(&mut self) -> HALResult<()> {
        hal_call!(HAL_CancelNotifierAlarm(self.0))
    }

    pub fn wait_for_alarm(&self) -> HALResult<u64> {
        hal_call!(HAL_WaitForNotifierAlarm(self.0))
    }
}

impl Drop for Notifier {
    fn drop(&mut self) {
        unsafe {
            HAL_CleanNotifier(self.0);
        }
    }
}
