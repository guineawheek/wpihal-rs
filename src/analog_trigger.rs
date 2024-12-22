use wpihal_sys::{HAL_AnalogTriggerHandle, HAL_AnalogTriggerType, HAL_CleanAnalogTrigger, HAL_GetAnalogTriggerFPGAIndex, HAL_GetAnalogTriggerInWindow, HAL_GetAnalogTriggerOutput, HAL_GetAnalogTriggerTriggerState, HAL_InitializeAnalogTrigger, HAL_InitializeAnalogTriggerDutyCycle, HAL_SetAnalogTriggerAveraged, HAL_SetAnalogTriggerFiltered, HAL_SetAnalogTriggerLimitsDutyCycle, HAL_SetAnalogTriggerLimitsRaw, HAL_SetAnalogTriggerLimitsVoltage};

use crate::{analog_input::AnalogInput, duty_cycle::DutyCycle, error::HALResult, hal_call, Handle};

pub type AnalogTriggerType = HAL_AnalogTriggerType;
pub type AnalogTriggerHandle = HAL_AnalogTriggerHandle;

#[derive(Debug, PartialEq, Eq)]
enum AnalogTriggerInput<'a> {
    Unknown,
    AnalogInput(&'a AnalogInput),
    DutyCycle(&'a DutyCycle<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct AnalogTrigger<'a>(AnalogTriggerHandle, AnalogTriggerInput<'a>);

impl<'a> AnalogTrigger<'a> {
    pub fn initialize_analog(handle: &'a AnalogInput) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeAnalogTrigger(handle.raw_handle()))?, AnalogTriggerInput::AnalogInput(handle)))
    }

    // TODO: this is wrong
    pub fn initialize_duty_cycle(handle: &'a DutyCycle<'a>) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeAnalogTriggerDutyCycle(handle.raw_handle()))?, AnalogTriggerInput::DutyCycle(handle)))
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

impl<'a> Drop for AnalogTrigger<'a> {
    fn drop(&mut self) {
        unsafe { HAL_CleanAnalogTrigger(self.0); }
    }
}

impl<'a> Handle<AnalogTriggerHandle> for AnalogTrigger<'a> {
    unsafe fn raw_handle(&self) -> AnalogTriggerHandle {
        self.0
    }

    unsafe fn from_raw_handle(handle: AnalogTriggerHandle) -> Self {
        Self(handle, AnalogTriggerInput::Unknown)
    }
}