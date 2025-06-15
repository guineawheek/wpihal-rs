use wpihal_sys::{
    HAL_CleanInterrupts, HAL_InitializeInterrupts, HAL_InterruptHandle,
    HAL_ReadInterruptFallingTimestamp, HAL_ReadInterruptRisingTimestamp,
    HAL_ReleaseWaitingInterrupt, HAL_RequestInterrupts, HAL_SetInterruptUpSourceEdge,
    HAL_WaitForInterrupt, HAL_WaitForMultipleInterrupts,
};

use crate::{dio::DigitalSource, error::HALResult, hal_call};

pub struct Interrupts(HAL_InterruptHandle);

impl Interrupts {
    pub fn initialize() -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeInterrupts())?))
    }

    pub fn wait_for_interrupt(&self, timeout: f64, ignore_previous: bool) -> HALResult<i64> {
        hal_call!(HAL_WaitForInterrupt(
            self.0,
            timeout,
            ignore_previous as i32
        ))
    }

    pub fn wait_for_multiple_interrupts(
        &self,
        mask: u64,
        timeout: f64,
        ignore_previous: bool,
    ) -> HALResult<i64> {
        hal_call!(HAL_WaitForMultipleInterrupts(
            self.0,
            mask as i64,
            timeout,
            ignore_previous as i32
        ))
    }

    pub fn read_interrupt_rising_timestamp(&self) -> HALResult<i64> {
        hal_call!(HAL_ReadInterruptRisingTimestamp(self.0))
    }

    pub fn read_interrupt_falling_timestamp(&self) -> HALResult<i64> {
        hal_call!(HAL_ReadInterruptFallingTimestamp(self.0))
    }

    pub fn request_interrupts(&mut self, digital_source: DigitalSource) -> HALResult<()> {
        hal_call!(HAL_RequestInterrupts(
            self.0,
            digital_source.raw_handle(),
            digital_source.analog_trigger_type()
        ))
    }

    pub fn set_interrupt_up_source_edge(&mut self, rising: bool, falling: bool) -> HALResult<()> {
        hal_call!(HAL_SetInterruptUpSourceEdge(
            self.0,
            rising as i32,
            falling as i32
        ))
    }

    pub fn release_waiting_interrupt(&self) -> HALResult<()> {
        hal_call!(HAL_ReleaseWaitingInterrupt(self.0))
    }
}

impl Drop for Interrupts {
    fn drop(&mut self) {
        unsafe {
            HAL_CleanInterrupts(self.0);
        }
    }
}
