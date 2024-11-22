use std::ffi::CStr;

use wpihal_sys::hal::{HAL_AnalogOutputHandle, HAL_CheckAnalogOutputChannel, HAL_FreeAnalogOutputPort, HAL_GetAnalogOutput, HAL_InitializeAnalogOutputPort, HAL_PortHandle, HAL_SetAnalogOutput};

use crate::{error::{allocation_location_ptr, HALResult}, hal_call};


pub struct AnalogOutput(HAL_AnalogOutputHandle);

impl AnalogOutput {
    pub fn initialize(port: HAL_PortHandle, allocation_location: Option<&CStr>) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeAnalogOutputPort(port, allocation_location_ptr(allocation_location)))?))
    }

    pub fn set(&mut self, voltage: f64) -> HALResult<()> {
        hal_call!(HAL_SetAnalogOutput(self.0, voltage))
    }

    pub fn get(&self) -> HALResult<f64> {
        hal_call!(HAL_GetAnalogOutput(self.0))
    }

    pub unsafe fn free(self) {
        // entirely possible this is Droppable
        unsafe { HAL_FreeAnalogOutputPort(self.0); }
    }
}

pub fn check_analog_output_channel(channel: i32) -> bool {
    unsafe { HAL_CheckAnalogOutputChannel(channel) != 0 }
}