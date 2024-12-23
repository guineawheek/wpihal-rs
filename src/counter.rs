use wpihal_sys::{HAL_ClearCounterDownSource, HAL_ClearCounterUpSource, HAL_CounterHandle, HAL_Counter_Mode, HAL_FreeCounter, HAL_GetCounter, HAL_GetCounterDirection, HAL_GetCounterPeriod, HAL_GetCounterSamplesToAverage, HAL_GetCounterStopped, HAL_Handle, HAL_InitializeCounter, HAL_ResetCounter, HAL_SetCounterAverageSize, HAL_SetCounterDownSource, HAL_SetCounterDownSourceEdge, HAL_SetCounterExternalDirectionMode, HAL_SetCounterMaxPeriod, HAL_SetCounterPulseLengthMode, HAL_SetCounterReverseDirection, HAL_SetCounterSamplesToAverage, HAL_SetCounterSemiPeriodMode, HAL_SetCounterUpDownMode, HAL_SetCounterUpSource, HAL_SetCounterUpSourceEdge, HAL_SetCounterUpdateWhenEmpty};

use crate::{analog_trigger::AnalogTriggerType, error::HALResult, hal_call};


pub type CounterMode = HAL_Counter_Mode;

pub struct Counter {
    handle: HAL_CounterHandle,
    index: i32
}

impl Counter {
    pub fn initialize(mode: HAL_Counter_Mode) -> HALResult<Self> {
        let mut index: i32 = 0;
        let handle = hal_call!(HAL_InitializeCounter(mode, &mut index))?;
        Ok(Self { handle, index })
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn set_average_size(&mut self, size: i32) -> HALResult<()> {
        hal_call!(HAL_SetCounterAverageSize(self.handle, size))
    }

    pub fn set_up_source(&mut self, digital_source_handle: HAL_Handle, analog_trigger_type: AnalogTriggerType) -> HALResult<()> {
        hal_call!(HAL_SetCounterUpSource(self.handle, digital_source_handle, analog_trigger_type))
    }

    pub fn set_up_source_edge(&mut self, rising_edge: bool, falling_edge: bool) -> HALResult<()> {
        hal_call!(HAL_SetCounterUpSourceEdge(self.handle, rising_edge as i32, falling_edge as i32))
    }

    pub fn clear_up_source(&mut self) -> HALResult<()> {
        hal_call!(HAL_ClearCounterUpSource(self.handle))
    }

    pub fn set_down_source(&mut self, digital_source_handle: HAL_Handle, analog_trigger_type: AnalogTriggerType) -> HALResult<()> {
        hal_call!(HAL_SetCounterDownSource(self.handle, digital_source_handle, analog_trigger_type))
    }

    pub fn set_down_source_edge(&mut self, rising_edge: bool, falling_edge: bool) -> HALResult<()> {
        hal_call!(HAL_SetCounterDownSourceEdge(self.handle, rising_edge as i32, falling_edge as i32))
    }

    pub fn clear_down_source(&mut self) -> HALResult<()> {
        hal_call!(HAL_ClearCounterDownSource(self.handle))
    }

    pub fn set_up_down_mode(&mut self) -> HALResult<()> {
        hal_call!(HAL_SetCounterUpDownMode(self.handle))
    }

    pub fn set_external_direction_mode(&mut self) -> HALResult<()> {
        hal_call!(HAL_SetCounterExternalDirectionMode(self.handle))
    }

    pub fn set_semi_period_mode(&mut self, high_semi_period: bool) -> HALResult<()> {
        hal_call!(HAL_SetCounterSemiPeriodMode(self.handle, high_semi_period as i32))
    }

    pub fn set_pulse_length_mode(&mut self, threshold: f64) -> HALResult<()> {
        hal_call!(HAL_SetCounterPulseLengthMode(self.handle, threshold))
    }

    pub fn get_samples_to_average(&self) -> HALResult<i32> {
        hal_call!(HAL_GetCounterSamplesToAverage(self.handle))
    }

    pub fn set_samples_to_average(&mut self, samples_to_average: i32) -> HALResult<()> {
        hal_call!(HAL_SetCounterSamplesToAverage(self.handle, samples_to_average))
    }

    pub fn reset(&mut self) -> HALResult<()> {
        hal_call!(HAL_ResetCounter(self.handle))
    }

    pub fn get(&self) -> HALResult<i32> {
        hal_call!(HAL_GetCounter(self.handle))
    }

    pub fn get_period(&self) -> HALResult<f64> {
        hal_call!(HAL_GetCounterPeriod(self.handle))
    }

    pub fn set_max_period(&mut self, max_period: f64) -> HALResult<()> {
        hal_call!(HAL_SetCounterMaxPeriod(self.handle, max_period))
    }

    pub fn set_update_when_empty(&mut self, enabled: bool) -> HALResult<()> {
        hal_call!(HAL_SetCounterUpdateWhenEmpty(self.handle, enabled as i32))
    }

    pub fn get_stopped(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCounterStopped(self.handle))? != 0)
    }

    pub fn get_direction(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCounterDirection(self.handle))? != 0)
    }

    pub fn set_reverse_direction(&mut self, reverse_direction: bool) -> HALResult<()> {
        hal_call!(HAL_SetCounterReverseDirection(self.handle, reverse_direction as i32))
    }

    pub unsafe fn raw_handle(&self) -> HAL_CounterHandle {
        self.handle
    }

}

impl Drop for Counter {
    fn drop(&mut self) {
        unsafe { HAL_FreeCounter(self.handle); }
    }
}