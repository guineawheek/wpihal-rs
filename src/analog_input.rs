use std::ffi::CStr;

use wpihal_sys::hal::{HAL_AnalogInputHandle, HAL_CheckAnalogInputChannel, HAL_CheckAnalogModule, HAL_FreeAnalogInputPort, HAL_GetAnalogAverageBits, HAL_GetAnalogAverageValue, HAL_GetAnalogAverageVoltage, HAL_GetAnalogLSBWeight, HAL_GetAnalogOffset, HAL_GetAnalogOversampleBits, HAL_GetAnalogSampleRate, HAL_GetAnalogValue, HAL_GetAnalogValueToVolts, HAL_GetAnalogVoltage, HAL_GetAnalogVoltsToValue, HAL_InitializeAnalogInputPort, HAL_IsAccumulatorChannel, HAL_PortHandle, HAL_SetAnalogAverageBits, HAL_SetAnalogInputSimDevice, HAL_SetAnalogOversampleBits, HAL_SetAnalogSampleRate, HAL_SimDeviceHandle};

use crate::{error::{allocation_location_ptr, HALResult}, hal_call};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AnalogInput(pub HAL_AnalogInputHandle);

impl AnalogInput {
    pub fn initialize(port: HAL_PortHandle, allocation_location: Option<&CStr>) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeAnalogInputPort(port, allocation_location_ptr(allocation_location)))?))
    }

    /// this can make things Explode, especially since this is cloneaable.
    pub unsafe fn free(self) {
        unsafe { HAL_FreeAnalogInputPort(self.0); }
    }

    pub fn is_accumulator_channel(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_IsAccumulatorChannel(self.0))? != 0)
    }

    pub fn set_sim_device(&mut self, handle: HAL_SimDeviceHandle) {
        unsafe { HAL_SetAnalogInputSimDevice(self.0, handle); }
    }

    pub fn set_sample_rate(samples_per_second: f64) -> HALResult<()> {
        hal_call!(HAL_SetAnalogSampleRate(samples_per_second))
    }

    pub fn get_sample_rate() -> HALResult<f64> {
        hal_call!(HAL_GetAnalogSampleRate())
    }

    pub fn set_average_bits(&mut self, bits: i32) -> HALResult<()> {
        hal_call!(HAL_SetAnalogAverageBits(self.0, bits))
    }

    pub fn get_average_bits(&self) -> HALResult<i32> {
        hal_call!(HAL_GetAnalogAverageBits(self.0))
    }

    pub fn set_oversample_bits(&mut self, bits: i32) -> HALResult<()> {
        hal_call!(HAL_SetAnalogOversampleBits(self.0, bits))
    }

    pub fn get_oversample_bits(&self) -> HALResult<i32> {
        hal_call!(HAL_GetAnalogOversampleBits(self.0))
    }

    pub fn get_value(&self) -> HALResult<i32> {
        hal_call!(HAL_GetAnalogValue(self.0))
    }

    pub fn get_average_value(&self) -> HALResult<i32> {
        hal_call!(HAL_GetAnalogAverageValue(self.0))
    }

    pub fn get_volts_to_value(&self, voltage: f64) -> HALResult<i32> {
        hal_call!(HAL_GetAnalogVoltsToValue(self.0, voltage))
    }

    pub fn get_voltage(&self) -> HALResult<f64> {
        hal_call!(HAL_GetAnalogVoltage(self.0))
    }

    pub fn get_average_voltage(&self) -> HALResult<f64> {
        hal_call!(HAL_GetAnalogAverageVoltage(self.0))
    }

    pub fn get_lsb_weight(&self) -> HALResult<i32> {
        hal_call!(HAL_GetAnalogLSBWeight(self.0))
    }

    pub fn get_offset(&self) -> HALResult<i32> {
        hal_call!(HAL_GetAnalogOffset(self.0))
    }

    pub fn get_value_to_volts(&self, raw_value: i32) -> HALResult<f64> {
        hal_call!(HAL_GetAnalogValueToVolts(self.0, raw_value))
    }

}

pub fn check_analog_module(module: i32) -> bool {
    unsafe { HAL_CheckAnalogModule(module) != 0 }
}

pub fn check_analog_input_channel(channel: i32) -> bool {
    unsafe { HAL_CheckAnalogInputChannel(channel) != 0 }
}