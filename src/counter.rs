use std::ffi::CStr;

use wpihal_sys::{
    HAL_CounterHandle, HAL_FreeCounter, HAL_GetCounter, HAL_GetCounterPeriod,
    HAL_GetCounterStopped, HAL_InitializeCounter, HAL_ResetCounter,
    HAL_SetCounterEdgeConfiguration, HAL_SetCounterMaxPeriod,
};

use crate::{
    error::{HALResult, allocation_location_ptr},
    hal_call,
};

pub struct Counter {
    handle: HAL_CounterHandle,
    channel: i32,
}

impl Counter {
    pub fn initialize(
        channel: i32,
        rising_edge: bool,
        allocation_location: Option<&CStr>,
    ) -> HALResult<Self> {
        let handle = hal_call!(HAL_InitializeCounter(
            channel,
            rising_edge as i32,
            allocation_location_ptr(allocation_location),
        ))?;
        Ok(Self { handle, channel })
    }

    pub fn channel(&self) -> i32 {
        self.channel
    }

    pub fn set_edge_configuration(&mut self, rising_edge: bool) -> HALResult<()> {
        hal_call!(HAL_SetCounterEdgeConfiguration(
            self.handle,
            rising_edge as i32
        ))
    }

    pub fn reset(&mut self) -> HALResult<()> {
        hal_call!(HAL_ResetCounter(self.handle))
    }

    pub fn get(&self) -> HALResult<i32> {
        hal_call!(HAL_GetCounter(self.handle))
    }

    pub fn get_period(&self) -> HALResult<f64> {
        hal_call!(HAL_GetCounterPeriod(self.handle))
    }

    pub fn set_max_period(&mut self, max_period: f64) -> HALResult<()> {
        hal_call!(HAL_SetCounterMaxPeriod(self.handle, max_period))
    }

    pub fn get_stopped(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCounterStopped(self.handle))? != 0)
    }

    pub fn raw_handle(&self) -> HAL_CounterHandle {
        self.handle
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeCounter(self.handle);
        }
    }
}
