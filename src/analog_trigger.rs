use wpihal_sys::hal::{HAL_AnalogInputHandle, HAL_AnalogTriggerType, HAL_CleanAnalogTrigger, HAL_GetAnalogTriggerFPGAIndex, HAL_GetAnalogTriggerInWindow, HAL_GetAnalogTriggerOutput, HAL_GetAnalogTriggerTriggerState, HAL_InitializeAnalogTrigger, HAL_InitializeAnalogTriggerDutyCycle, HAL_SetAnalogTriggerAveraged, HAL_SetAnalogTriggerFiltered, HAL_SetAnalogTriggerLimitsDutyCycle, HAL_SetAnalogTriggerLimitsRaw, HAL_SetAnalogTriggerLimitsVoltage};

use crate::{error::HALResult, hal_call};

pub type AnalogTriggerType = HAL_AnalogTriggerType;

pub struct AnalogTrigger(HAL_AnalogInputHandle);

impl AnalogTrigger {
    pub fn initialize_analog(handle: HAL_AnalogInputHandle) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeAnalogTrigger(handle))?))
    }

    pub fn initialize_duty_cycle(handle: HAL_AnalogInputHandle) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeAnalogTriggerDutyCycle(handle))?))
    }

    pub fn set_limits_raw(&mut self, lower: i32, upper: i32) -> HALResult<()> {
        hal_call!(HAL_SetAnalogTriggerLimitsRaw(self.0, lower, upper))
    }

    pub fn set_limits_voltage(&mut self, lower: f64, upper: f64) -> HALResult<()> {
        hal_call!(HAL_SetAnalogTriggerLimitsVoltage(self.0, lower, upper))
    }

    pub fn set_limits_duty_cycle(&mut self, lower: f64, upper: f64) -> HALResult<()> {
        hal_call!(HAL_SetAnalogTriggerLimitsDutyCycle(self.0, lower, upper))
    }

    pub fn set_averaged(&mut self, averaged: bool) -> HALResult<()> {
        hal_call!(HAL_SetAnalogTriggerAveraged(self.0, averaged as i32))
    }

    pub fn set_filtered(&mut self, filtered: bool) -> HALResult<()> {
        hal_call!(HAL_SetAnalogTriggerFiltered(self.0, filtered as i32))
    }

    pub fn get_in_window(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetAnalogTriggerInWindow(self.0))? != 0)
    }

    pub fn get_trigger_state(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetAnalogTriggerTriggerState(self.0))? != 0)
    }

    pub fn get_output(&self, trigger_type: HAL_AnalogTriggerType) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetAnalogTriggerOutput(self.0, trigger_type))? != 0)
    }

    pub fn get_fpga_index(&self) -> HALResult<i32> {
        hal_call!(HAL_GetAnalogTriggerFPGAIndex(self.0))
    }

}

impl Drop for AnalogTrigger {
    fn drop(&mut self) {
        unsafe { HAL_CleanAnalogTrigger(self.0); }
    }
}