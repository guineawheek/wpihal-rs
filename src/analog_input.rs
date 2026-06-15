use std::ffi::CStr;

use wpihal_sys::{
    HAL_CheckAnalogInputChannel, HAL_CheckAnalogModule, HAL_FreeAnalogInputPort,
    HAL_GetAnalogValue, HAL_GetAnalogValueToVolts, HAL_GetAnalogVoltage, HAL_GetAnalogVoltsToValue,
    HAL_InitializeAnalogInputPort, HAL_SetAnalogInputSimDevice,
};

use crate::{
    Handle,
    error::{HALResult, allocation_location_ptr},
    hal_call,
    sim_device::SimDevice,
};

/// Raw analog input handle
pub use wpihal_sys::HAL_AnalogInputHandle as AnalogInputHandle;

#[derive(Debug, PartialEq, Eq)]
pub struct AnalogInput(AnalogInputHandle);

impl AnalogInput {
    pub fn initialize(channel: i32, allocation_location: Option<&CStr>) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeAnalogInputPort(
            channel,
            allocation_location_ptr(allocation_location)
        ))?))
    }

    /// Sets the sim device
    pub fn set_sim_device(&mut self, handle: &SimDevice) {
        unsafe {
            HAL_SetAnalogInputSimDevice(self.0, handle.handle());
        }
    }

    pub fn get_value(&self) -> HALResult<i32> {
        hal_call!(HAL_GetAnalogValue(self.0))
    }

    pub fn get_volts_to_value(&self, voltage: f64) -> HALResult<i32> {
        hal_call!(HAL_GetAnalogVoltsToValue(self.0, voltage))
    }

    pub fn get_voltage(&self) -> HALResult<f64> {
        hal_call!(HAL_GetAnalogVoltage(self.0))
    }

    pub fn get_value_to_volts(&self, raw_value: i32) -> HALResult<f64> {
        hal_call!(HAL_GetAnalogValueToVolts(self.0, raw_value))
    }

    /// Checks that an analog module index is valid.
    /// Likely a holdover from the cRIO era.
    pub fn check_module(module: i32) -> bool {
        unsafe { HAL_CheckAnalogModule(module) != 0 }
    }

    /// Checsk that an analog input channel is valid.
    pub fn check_input_channel(channel: i32) -> bool {
        unsafe { HAL_CheckAnalogInputChannel(channel) != 0 }
    }
}

impl Drop for AnalogInput {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeAnalogInputPort(self.0);
        }
    }
}

impl Handle<AnalogInputHandle> for AnalogInput {
    unsafe fn raw_handle(&self) -> AnalogInputHandle {
        self.0
    }

    unsafe fn from_raw_handle(handle: AnalogInputHandle) -> Self {
        Self(handle)
    }
}
