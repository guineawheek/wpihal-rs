use std::ffi::CStr;

use wpihal_sys::{
    HAL_CheckPWMChannel, HAL_DigitalHandle, HAL_FreePWMPort, HAL_GetPWMConfigMicroseconds,
    HAL_GetPWMCycleStartTime, HAL_GetPWMEliminateDeadband, HAL_GetPWMLoopTiming,
    HAL_GetPWMPosition, HAL_GetPWMPulseTimeMicroseconds, HAL_GetPWMSpeed, HAL_InitializePWMPort,
    HAL_LatchPWMZero, HAL_PortHandle, HAL_SetPWMAlwaysHighMode, HAL_SetPWMConfigMicroseconds,
    HAL_SetPWMDisabled, HAL_SetPWMEliminateDeadband, HAL_SetPWMPeriodScale, HAL_SetPWMPosition,
    HAL_SetPWMPulseTimeMicroseconds, HAL_SetPWMSpeed,
};

use crate::{
    error::{HALResult, allocation_location_ptr},
    hal_call,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct PWMConfig {
    pub max: i32,
    pub deadband_max: i32,
    pub center: i32,
    pub deadband_min: i32,
    pub min: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PWM(HAL_DigitalHandle);

impl PWM {
    pub fn initialize(port: HAL_PortHandle, allocation_location: Option<&CStr>) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializePWMPort(
            port,
            allocation_location_ptr(allocation_location)
        ))?))
    }

    pub fn check_channel(channel: i32) -> bool {
        unsafe { HAL_CheckPWMChannel(channel) != 0 }
    }

    pub fn set_config(&mut self, config: &PWMConfig) -> HALResult<()> {
        hal_call!(HAL_SetPWMConfigMicroseconds(
            self.0,
            config.max,
            config.deadband_max,
            config.center,
            config.deadband_min,
            config.min
        ))
    }

    pub fn get_config(&self) -> HALResult<PWMConfig> {
        let mut cfg = PWMConfig::default();

        hal_call!(HAL_GetPWMConfigMicroseconds(
            self.0,
            &mut cfg.max,
            &mut cfg.deadband_max,
            &mut cfg.center,
            &mut cfg.deadband_min,
            &mut cfg.min
        ))?;
        Ok(cfg)
    }

    pub fn set_eliminate_deadband(&mut self, eliminate_deadband: bool) -> HALResult<()> {
        hal_call!(HAL_SetPWMEliminateDeadband(
            self.0,
            eliminate_deadband as i32
        ))
    }

    pub fn get_eliminate_deadband(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetPWMEliminateDeadband(self.0))? != 0)
    }

    pub fn set_pulse_time_microseconds(&mut self, pulse_time_us: i32) -> HALResult<()> {
        hal_call!(HAL_SetPWMPulseTimeMicroseconds(self.0, pulse_time_us))
    }

    pub fn set_speed(&mut self, speed: f64) -> HALResult<()> {
        hal_call!(HAL_SetPWMSpeed(self.0, speed))
    }

    pub fn set_position(&mut self, position: f64) -> HALResult<()> {
        hal_call!(HAL_SetPWMPosition(self.0, position))
    }

    pub fn disable(&mut self) -> HALResult<()> {
        hal_call!(HAL_SetPWMDisabled(self.0))
    }

    pub fn get_pulse_time_microseconds(&self) -> HALResult<i32> {
        hal_call!(HAL_GetPWMPulseTimeMicroseconds(self.0))
    }

    pub fn get_speed(&self) -> HALResult<f64> {
        hal_call!(HAL_GetPWMSpeed(self.0))
    }

    pub fn get_position(&self) -> HALResult<f64> {
        hal_call!(HAL_GetPWMPosition(self.0))
    }

    pub fn latch_zero(&mut self) -> HALResult<()> {
        hal_call!(HAL_LatchPWMZero(self.0))
    }

    pub fn set_period_scale(&mut self, squelch_mask: i32) -> HALResult<()> {
        hal_call!(HAL_SetPWMPeriodScale(self.0, squelch_mask))
    }

    pub fn set_always_high(&mut self) -> HALResult<()> {
        hal_call!(HAL_SetPWMAlwaysHighMode(self.0))
    }

    pub fn get_loop_timing() -> HALResult<i32> {
        hal_call!(HAL_GetPWMLoopTiming())
    }

    pub fn get_cycle_start_time() -> HALResult<u64> {
        hal_call!(HAL_GetPWMCycleStartTime())
    }
}

impl Drop for PWM {
    fn drop(&mut self) {
        unsafe {
            HAL_FreePWMPort(self.0);
        }
    }
}
