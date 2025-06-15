use std::ffi::CStr;

use wpihal_sys::{
    HAL_AllocateDigitalPWM, HAL_CheckDIOChannel, HAL_DigitalHandle, HAL_DigitalPWMHandle,
    HAL_FreeDIOPort, HAL_FreeDigitalPWM, HAL_GetDIO, HAL_GetDIODirection, HAL_InitializeDIOPort,
    HAL_IsAnyPulsing, HAL_IsPulsing, HAL_Pulse, HAL_PulseMultiple, HAL_SetDIO, HAL_SetDIOSimDevice,
    HAL_SetDigitalPWMDutyCycle, HAL_SetDigitalPWMOutputChannel, HAL_SetDigitalPWMPPS,
    HAL_SetDigitalPWMRate,
};

use crate::{
    Handle,
    error::{HALResult, allocation_location_ptr},
    hal_call,
    sim_device::SimDevice,
};

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DigitalInputFilterIndex {
    None = 0,
    Filter0 = 1,
    Filter1 = 2,
    Filter2 = 3,
}
impl From<i32> for DigitalInputFilterIndex {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Filter0,
            2 => Self::Filter1,
            3 => Self::Filter2,
            _ => Self::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DIO(HAL_DigitalHandle);

impl DIO {
    pub fn initialize(
        channel: i32,
        input: bool,
        allocation_location: Option<&CStr>,
    ) -> HALResult<DIO> {
        Ok(Self(hal_call!(HAL_InitializeDIOPort(
            channel,
            input as i32,
            allocation_location_ptr(allocation_location)
        ))?))
    }

    pub fn set_sim_device(&mut self, handle: &SimDevice) {
        unsafe {
            HAL_SetDIOSimDevice(self.0, handle.handle());
        }
    }

    pub fn set(&mut self, value: bool) -> HALResult<()> {
        hal_call!(HAL_SetDIO(self.0, value as i32))
    }

    pub fn set_direction(&mut self, value: bool) -> HALResult<()> {
        hal_call!(HAL_SetDIO(self.0, value as i32))
    }

    pub fn get(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetDIO(self.0))? != 0)
    }

    pub fn get_direction(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetDIODirection(self.0))? != 0)
    }

    pub fn pulse(&mut self, pulse_length_seconds: f64) -> HALResult<()> {
        hal_call!(HAL_Pulse(self.0, pulse_length_seconds))
    }

    pub fn pulse_multiple(channel_mask: u32, pulse_length_seconds: f64) -> HALResult<()> {
        hal_call!(HAL_PulseMultiple(channel_mask, pulse_length_seconds))
    }

    pub fn is_pulsing(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_IsPulsing(self.0))? != 0)
    }

    pub fn is_any_pulsing() -> HALResult<bool> {
        Ok(hal_call!(HAL_IsAnyPulsing())? != 0)
    }

    pub fn check_channel(channel: i32) -> bool {
        unsafe { HAL_CheckDIOChannel(channel) != 0 }
    }
}

impl Drop for DIO {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeDIOPort(self.0);
        }
    }
}

impl Handle<HAL_DigitalHandle> for DIO {
    unsafe fn raw_handle(&self) -> HAL_DigitalHandle {
        self.0
    }

    unsafe fn from_raw_handle(handle: HAL_DigitalHandle) -> Self {
        Self(handle)
    }
}

pub struct DigitalPWM(HAL_DigitalPWMHandle);

impl DigitalPWM {
    pub fn initialize() -> HALResult<DigitalPWM> {
        Ok(Self(hal_call!(HAL_AllocateDigitalPWM())?))
    }

    pub fn set_rate(rate: f64) -> HALResult<()> {
        hal_call!(HAL_SetDigitalPWMRate(rate))
    }

    pub fn set_duty_cycle(&mut self, duty_cycle: f64) -> HALResult<()> {
        hal_call!(HAL_SetDigitalPWMDutyCycle(self.0, duty_cycle))
    }

    pub fn set_pps(&mut self, duty_cycle: f64) -> HALResult<()> {
        hal_call!(HAL_SetDigitalPWMPPS(self.0, duty_cycle))
    }

    pub fn set_output_channel(&mut self, channel: i32) -> HALResult<()> {
        hal_call!(HAL_SetDigitalPWMOutputChannel(self.0, channel))
    }
}

impl Drop for DigitalPWM {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeDigitalPWM(self.0);
        }
    }
}
