use wpihal_sys::hal::{HAL_AnalogInputHandle, HAL_GetAccumulatorCount, HAL_GetAccumulatorOutput, HAL_GetAccumulatorValue, HAL_InitAccumulator, HAL_ResetAccumulator, HAL_SetAccumulatorCenter, HAL_SetAccumulatorDeadband};

use crate::{analog_input::AnalogInput, error::HALResult, hal_call};


pub struct AnalogAccumulator(HAL_AnalogInputHandle);

impl AnalogAccumulator {
    pub fn initialize(analog_port_handle: AnalogInput) -> HALResult<Self> {
        hal_call!(HAL_InitAccumulator(analog_port_handle.0))?;
        Ok(Self(analog_port_handle.0))
    }

    pub fn reset(&mut self) -> HALResult<()> {
        hal_call!(HAL_ResetAccumulator(self.0))
    }

    pub fn set_center(&mut self, center: i32) -> HALResult<()> {
        hal_call!(HAL_SetAccumulatorCenter(self.0, center))
    }

    pub fn set_deadband(&mut self, deadband: i32) -> HALResult<()> {
        hal_call!(HAL_SetAccumulatorDeadband(self.0, deadband))
    }

    pub fn get_value(&self) -> HALResult<i64> {
        hal_call!(HAL_GetAccumulatorValue(self.0))
    }

    pub fn get_count(&self) -> HALResult<i64> {
        hal_call!(HAL_GetAccumulatorCount(self.0))
    }

    /// returns (value, count)
    pub fn get_output(&self) -> HALResult<(i64, i64)>  {
        let mut value = 0i64;
        let mut count= 0i64;
        hal_call!(HAL_GetAccumulatorOutput(self.0, &mut value, &mut count))?;
        Ok((value, count))
    }

}