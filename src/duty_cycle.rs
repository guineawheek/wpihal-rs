use std::ffi::CStr;

use wpihal_sys::{
    HAL_DutyCycleHandle, HAL_FreeDutyCycle, HAL_GetDutyCycleFPGAIndex, HAL_GetDutyCycleFrequency,
    HAL_GetDutyCycleHighTime, HAL_GetDutyCycleOutput, HAL_GetDutyCycleOutputScaleFactor,
    HAL_InitializeDutyCycle, HAL_SetDutyCycleSimDevice,
};

use crate::{
    Handle,
    error::{HALResult, allocation_location_ptr},
    hal_call,
    sim_device::SimDevice,
};

#[derive(Debug, PartialEq, Eq)]
pub struct DutyCycle {
    handle: HAL_DutyCycleHandle,
    channel: i32,
}

impl DutyCycle {
    pub fn initialize(channel: i32, allocation_location: Option<&CStr>) -> HALResult<Self> {
        let handle = hal_call!(HAL_InitializeDutyCycle(
            channel,
            allocation_location_ptr(allocation_location)
        ))?;
        Ok(Self { handle, channel })
    }

    pub fn set_sim_device(&mut self, handle: &SimDevice) {
        unsafe {
            HAL_SetDutyCycleSimDevice(self.handle, handle.handle());
        }
    }

    pub fn get_frequency(&self) -> HALResult<i32> {
        hal_call!(HAL_GetDutyCycleFrequency(self.handle))
    }

    pub fn get_output(&self) -> HALResult<f64> {
        hal_call!(HAL_GetDutyCycleOutput(self.handle))
    }

    pub fn get_high_time(&self) -> HALResult<i32> {
        hal_call!(HAL_GetDutyCycleHighTime(self.handle))
    }

    pub fn get_output_scale_factor(&self) -> HALResult<i32> {
        hal_call!(HAL_GetDutyCycleOutputScaleFactor(self.handle))
    }

    pub fn get_fpga_index(&self) -> HALResult<i32> {
        hal_call!(HAL_GetDutyCycleFPGAIndex(self.handle))
    }
}

impl Drop for DutyCycle {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeDutyCycle(self.handle);
        }
    }
}

impl Handle<HAL_DutyCycleHandle> for DutyCycle {
    unsafe fn raw_handle(&self) -> HAL_DutyCycleHandle {
        self.handle
    }

    unsafe fn from_raw_handle(handle: HAL_DutyCycleHandle) -> Self {
        Self {
            handle,
            channel: -1,
        }
    }
}
