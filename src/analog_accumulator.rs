use wpihal_sys::{HAL_GetAccumulatorCount, HAL_GetAccumulatorOutput, HAL_GetAccumulatorValue, HAL_InitAccumulator, HAL_ResetAccumulator, HAL_SetAccumulatorCenter, HAL_SetAccumulatorDeadband};

use crate::{analog_input::AnalogInput, error::HALResult, hal_call, Handle};

/// FPGA-managed analog accumulator.
#[derive(Debug, PartialEq, Eq)]
pub struct AnalogAccumulator<'a>(&'a AnalogInput);

impl<'a> AnalogAccumulator<'a> {
    /// Initializes an accumulator attached to an analog input.
    /// 
    /// This function will fail if the analog input is not accumulator capable.
    pub fn initialize(analog_port_handle: &'a AnalogInput) -> HALResult<Self> {
        hal_call!(HAL_InitAccumulator(analog_port_handle.raw_handle()))?;
        Ok(Self(analog_port_handle))
    }

    /// Resets the accumulator to the initial value
    pub fn reset(&mut self) -> HALResult<()> {
        hal_call!(HAL_ResetAccumulator(self.0.raw_handle()))
    }
    /// Set the center value of the accumulator.
    ///
    /// The center value is subtracted from each A/D value before it is added to the
    /// accumulator. This is used for the center value of devices like gyros and
    /// accelerometers to make integration work and to take the device offset into
    /// account when integrating.
    ///
    /// This center value is based on the output of the oversampled and averaged
    /// source from channel 1. Because of this, any non-zero oversample bits will
    /// affect the size of the value for this field.
    pub fn set_center(&mut self, center: i32) -> HALResult<()> {
        hal_call!(HAL_SetAccumulatorCenter(self.0.raw_handle(), center))
    }

    /// Sets accumulator deadband.
    pub fn set_deadband(&mut self, deadband: i32) -> HALResult<()> {
        hal_call!(HAL_SetAccumulatorDeadband(self.0.raw_handle(), deadband))
    }

    /// Reads the accumulated value.
    pub fn get_value(&self) -> HALResult<i64> {
        hal_call!(HAL_GetAccumulatorValue(self.0.raw_handle()))
    }

    /// Reads the number of accumulated values since the last reset.
    pub fn get_count(&self) -> HALResult<i64> {
        hal_call!(HAL_GetAccumulatorCount(self.0.raw_handle()))
    }

    /// Reads both accumulated value and value count atomically from the FPGA, useful for averaging.
    pub fn get_output(&self) -> HALResult<(i64, i64)>  {
        let mut value = 0i64;
        let mut count= 0i64;
        hal_call!(HAL_GetAccumulatorOutput(self.0.raw_handle(), &mut value, &mut count))?;
        Ok((value, count))
    }
}
