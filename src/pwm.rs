use std::ffi::CStr;

use wpihal_sys::{
    HAL_CheckPWMChannel, HAL_DigitalHandle, HAL_FreePWMPort, HAL_GetPWMPulseTimeMicroseconds,
    HAL_InitializePWMPort, HAL_SetPWMOutputPeriod, HAL_SetPWMPulseTimeMicroseconds,
    HAL_SetPWMSimDevice,
};

use crate::{
    error::{HALResult, allocation_location_ptr},
    hal_call,
    sim_device::SimDevice,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum PWMOutputPeriod {
    Period5ms = 0,
    Period10ms = 1,
    Period20ms = 3,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PWM(HAL_DigitalHandle);

impl PWM {
    pub fn initialize(channel: i32, allocation_location: Option<&CStr>) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializePWMPort(
            channel,
            allocation_location_ptr(allocation_location)
        ))?))
    }

    pub fn set_sim_device(&mut self, handle: &SimDevice) {
        unsafe {
            HAL_SetPWMSimDevice(self.0, handle.handle());
        }
    }

    pub fn check_channel(channel: i32) -> bool {
        unsafe { HAL_CheckPWMChannel(channel) != 0 }
    }

    pub fn set_pulse_time_microseconds(&mut self, pulse_time_us: i32) -> HALResult<()> {
        hal_call!(HAL_SetPWMPulseTimeMicroseconds(self.0, pulse_time_us))
    }

    pub fn get_pulse_time_microseconds(&self) -> HALResult<i32> {
        hal_call!(HAL_GetPWMPulseTimeMicroseconds(self.0))
    }

    pub fn set_output_period(&mut self, period: PWMOutputPeriod) -> HALResult<()> {
        hal_call!(HAL_SetPWMOutputPeriod(self.0, period as i32))
    }
}

impl Drop for PWM {
    fn drop(&mut self) {
        unsafe {
            HAL_FreePWMPort(self.0);
        }
    }
}
