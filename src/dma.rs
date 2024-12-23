use wpihal_sys::{HAL_AddDMAAnalogAccumulator, HAL_AddDMAAnalogInput, HAL_AddDMAAveragedAnalogInput, HAL_AddDMACounter, HAL_AddDMACounterPeriod, HAL_AddDMADigitalSource, HAL_AddDMADutyCycle, HAL_AddDMAEncoder, HAL_ClearDMAExternalTriggers, HAL_ClearDMASensors, HAL_DMAHandle, HAL_DMASample, HAL_FreeDMA, HAL_GetDMASampleAnalogAccumulator, HAL_GetDMASampleAnalogInputRaw, HAL_GetDMASampleAveragedAnalogInputRaw, HAL_GetDMASampleCounter, HAL_GetDMASampleCounterPeriod, HAL_GetDMASampleDigitalSource, HAL_GetDMASampleDutyCycleOutputRaw, HAL_GetDMASampleEncoderPeriodRaw, HAL_GetDMASampleEncoderRaw, HAL_GetDMASampleTime, HAL_InitializeDMA, HAL_ReadDMA, HAL_SetDMAExternalTrigger, HAL_SetDMAPause, HAL_SetDMATimedTrigger, HAL_SetDMATimedTriggerCycles, HAL_StartDMA, HAL_StopDMA};

use crate::{analog_accumulator::AnalogAccumulator, analog_input::AnalogInput, counter::Counter, dio::{DigitalSource, DIO}, duty_cycle::DutyCycle, encoder::Encoder, error::{HALError, HALResult}, hal_call, Handle};

#[derive(Debug, Clone, Copy)]
pub enum DMAError {
    DMATimeout,
    DMAError,
    HALError(HALError)
}

impl core::fmt::Display for DMAError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DMAError::DMATimeout => {
                write!(f, "DMAError::DMATimeout")
            }
            DMAError::DMAError => {
                write!(f, "DMAError::DMAError")
            }
            DMAError::HALError(halerror) => {
                write!(f, "DMAError::HALError {} ", halerror)
            }
        }
    }
}

impl core::error::Error for DMAError {}

impl From<HALError> for DMAError {
    fn from(value: HALError) -> Self {
        DMAError::HALError(value)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct DMASample(pub HAL_DMASample);

impl DMASample {
    pub fn get_sample_time(&self) -> HALResult<u64> {
        hal_call!(HAL_GetDMASampleTime(&self.0))
    }

    pub fn get_encoder_raw(&self, encoder: &Encoder) -> HALResult<i32> {
        hal_call!(HAL_GetDMASampleEncoderRaw(&self.0, encoder.raw_handle()))
    }

    pub fn get_encoder_period_raw(&self, encoder: &Encoder) -> HALResult<i32> {
        hal_call!(HAL_GetDMASampleEncoderPeriodRaw(&self.0, encoder.raw_handle()))
    }

    pub fn get_counter(&self, counter: &Counter) -> HALResult<i32> {
        hal_call!(HAL_GetDMASampleCounter(&self.0, counter.raw_handle()))
    }

    pub fn get_counter_period(&self, counter: &Counter) -> HALResult<i32> {
        hal_call!(HAL_GetDMASampleCounterPeriod(&self.0, counter.raw_handle()))
    }

    pub fn get_digital_source(&self, digital_source: &DIO) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetDMASampleDigitalSource(&self.0, digital_source.raw_handle()))? != 0)
    }

    pub fn get_analog_input_raw(&self, analog_input: &AnalogInput) -> HALResult<i32> {
        hal_call!(HAL_GetDMASampleAnalogInputRaw(&self.0, analog_input.raw_handle()))
    }

    pub fn get_averaged_analog_input_raw(&self, analog_input: &AnalogInput) -> HALResult<i32> {
        hal_call!(HAL_GetDMASampleAveragedAnalogInputRaw(&self.0, analog_input.raw_handle()))
    }

    pub fn get_analog_accumulator(&self, analog_acc: &AnalogAccumulator) -> HALResult<(i64, i64)> {
        let mut count = 0i64;
        let mut value = 0i64;
        hal_call!(HAL_GetDMASampleAnalogAccumulator(&self.0, analog_acc.raw_handle(), &mut count, &mut value))?;
        Ok((count, value))
    }

    pub fn get_duty_cycle_raw(&self, duty_cycle: &DutyCycle) -> HALResult<i32> {
        hal_call!(HAL_GetDMASampleDutyCycleOutputRaw(&self.0, duty_cycle.raw_handle()))
    }
}


#[derive(Debug)]
pub struct DMA(HAL_DMAHandle);

impl DMA {
    pub fn initialize() -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeDMA())?))
    }

    pub fn pause(&mut self) -> HALResult<()> {
        hal_call!(HAL_SetDMAPause(self.0, 1))
    }

    pub fn resume(&mut self) -> HALResult<()> {
        hal_call!(HAL_SetDMAPause(self.0, 0))
    }

    pub fn set_timed_trigger(&mut self, period_seconds: f64) -> HALResult<()> {
        hal_call!(HAL_SetDMATimedTrigger(self.0, period_seconds))
    }

    pub fn set_timed_trigger_cycles(&mut self, fpga_cycles: u32) -> HALResult<()> {
        hal_call!(HAL_SetDMATimedTriggerCycles(self.0, fpga_cycles))
    }

    pub fn add_encoder(&mut self, encoder: &Encoder) -> HALResult<()> {
        hal_call!(HAL_AddDMAEncoder(self.0, encoder.raw_handle()))
    }

    pub fn add_encoder_period(&mut self, encoder: &Encoder) -> HALResult<()> {
        hal_call!(HAL_AddDMAEncoder(self.0, encoder.raw_handle()))
    }

    pub fn add_counter(&mut self, counter: &Counter) -> HALResult<()> {
        hal_call!(HAL_AddDMACounter(self.0, counter.raw_handle()))
    }

    pub fn add_counter_period(&mut self, counter: &Counter) -> HALResult<()> {
        hal_call!(HAL_AddDMACounterPeriod(self.0, counter.raw_handle()))
    }

    pub fn add_digital_source(&mut self, digital_source: &DIO) -> HALResult<()> {
        hal_call!(HAL_AddDMADigitalSource(self.0, digital_source.raw_handle()))
    }

    pub fn add_analog_input(&mut self, analog_input: &AnalogInput) -> HALResult<()> {
        hal_call!(HAL_AddDMAAnalogInput(self.0, analog_input.raw_handle()))
    }

    pub fn add_averaged_analog_input(&mut self, analog_input: &AnalogInput) -> HALResult<()> {
        hal_call!(HAL_AddDMAAveragedAnalogInput(self.0, analog_input.raw_handle()))
    }

    pub fn add_analog_accumulator(&mut self, analog_acc: &AnalogAccumulator) -> HALResult<()> {
        hal_call!(HAL_AddDMAAnalogAccumulator(self.0, analog_acc.raw_handle()))
    }

    pub fn add_duty_cycle(&mut self, duty_cycle: &DutyCycle) -> HALResult<()> {
        hal_call!(HAL_AddDMADutyCycle(self.0, duty_cycle.raw_handle()))
    }

    pub fn set_external_trigger(&mut self, digital_source: &DigitalSource, rising: bool, falling: bool) -> HALResult<i32> {
        hal_call!(HAL_SetDMAExternalTrigger(self.0, digital_source.raw_handle(), digital_source.analog_trigger_type(), rising as i32, falling as i32))
    }

    pub fn clear_sensors(&mut self) -> HALResult<()> {
        hal_call!(HAL_ClearDMASensors(self.0))
    }

    pub fn clear_external_triggers(&mut self) -> HALResult<()> {
        hal_call!(HAL_ClearDMAExternalTriggers(self.0))
    }

    pub fn start(&mut self, queue_depth: i32) -> HALResult<()> {
        hal_call!(HAL_StartDMA(self.0, queue_depth))        
    }

    pub fn stop(&mut self) -> HALResult<()> {
        hal_call!(HAL_StopDMA(self.0))        
    }
    // HAL_GetDMADirectPointer is not implemented due to lack of chipobject.

    pub fn read(&mut self, timeout_seconds: f64) -> Result<(DMASample, i32), DMAError> {
        let mut remaining_out = 0i32;
        let mut sample = HAL_DMASample::default();
        match hal_call!(HAL_ReadDMA(self.0, &mut sample, timeout_seconds, &mut remaining_out))? {
            wpihal_sys::HAL_DMAReadStatus::HAL_DMA_OK => {
                Ok((DMASample(sample), remaining_out))
            }
            wpihal_sys::HAL_DMAReadStatus::HAL_DMA_TIMEOUT => {
                Err(DMAError::DMATimeout)
            }
            wpihal_sys::HAL_DMAReadStatus::HAL_DMA_ERROR => {
                Err(DMAError::DMAError)
            }
        }

    }


}

impl Drop for DMA {
    fn drop(&mut self) {
        unsafe { HAL_FreeDMA(self.0); }
    }
}