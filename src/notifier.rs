use wpihal_sys::{
    HAL_AcknowledgeNotifierAlarm, HAL_CancelNotifierAlarm, HAL_CreateNotifier, HAL_DestroyNotifier,
    HAL_GetNotifierOverrun, HAL_NotifierHandle, HAL_SetNotifierAlarm, HAL_SetNotifierName,
};
use wpiutil::as_wpistr;

use crate::{error::HALResult, hal_call};

pub struct Notifier(HAL_NotifierHandle);

impl Notifier {
    pub fn initialize() -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_CreateNotifier())?))
    }

    pub fn set_name(&mut self, name: &str) -> HALResult<()> {
        hal_call!(HAL_SetNotifierName(self.0, as_wpistr!(name)))
    }

    pub fn set_alarm(
        &mut self,
        time_us: u64,
        interval_us: u64,
        absolute: bool,
        ack: bool,
    ) -> HALResult<()> {
        hal_call!(HAL_SetNotifierAlarm(
            self.0,
            time_us,
            interval_us,
            absolute as _,
            ack as _
        ))
    }

    pub fn cancel_alarm(&mut self, ack: bool) -> HALResult<()> {
        hal_call!(HAL_CancelNotifierAlarm(self.0, ack as _))
    }

    pub fn acknowledge(&mut self) -> HALResult<()> {
        hal_call!(HAL_AcknowledgeNotifierAlarm(self.0))
    }

    pub fn overrun_count(&self) -> HALResult<i32> {
        hal_call!(HAL_GetNotifierOverrun(self.0))
    }

    pub unsafe fn from_raw_handle(handle: HAL_NotifierHandle) -> Self {
        Self(handle)
    }
}

impl Drop for Notifier {
    fn drop(&mut self) {
        unsafe {
            HAL_DestroyNotifier(self.0);
        }
    }
}
