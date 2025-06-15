use wpihal_sys::{HAL_CloseSPI, HAL_ConfigureSPIAutoStall, HAL_ForceSPIAutoRead, HAL_FreeSPIAuto, HAL_GetSPIAutoDroppedCount, HAL_GetSPIHandle, HAL_GetSPIMode, HAL_InitSPIAuto, HAL_InitializeSPI, HAL_ReadSPI, HAL_ReadSPIAutoReceivedData, HAL_SPIMode, HAL_SPIPort, HAL_SetSPIAutoTransmitData, HAL_SetSPIChipSelectActiveHigh, HAL_SetSPIChipSelectActiveLow, HAL_SetSPIHandle, HAL_SetSPIMode, HAL_SetSPISpeed, HAL_StartSPIAutoRate, HAL_StartSPIAutoTrigger, HAL_StopSPIAuto, HAL_TransactionSPI, HAL_WriteSPI};

use crate::{dio::DigitalSource, error::HALResult, hal_call};

pub type SPIPort = HAL_SPIPort;
pub type SPIMode = HAL_SPIMode;

#[derive(Debug, PartialEq, Eq)]
pub struct SPI(HAL_SPIPort);

impl SPI {
    pub fn initialize(port: SPIPort) -> HALResult<SPI> {
        hal_call!(HAL_InitializeSPI(port))?;
        Ok(Self(port))
    }

    pub fn transaction(&mut self, tx: &[u8], rx: &mut [u8]) -> i32 {
        if tx.len() != rx.len() {
            return -1;
        }
        unsafe {
            HAL_TransactionSPI(
                self.0,
                tx.as_ptr(), 
                rx.as_mut_ptr(),
                rx.len() as i32,
            )
        }
    }

    pub fn write(&mut self, tx: &[u8]) -> i32 {
        unsafe {
            HAL_WriteSPI(self.0, tx.as_ptr(), tx.len() as i32)
        }
    }

    pub fn read(&mut self, rx: &mut [u8]) -> i32 {
        unsafe {
            HAL_ReadSPI(self.0, rx.as_mut_ptr(), rx.len() as i32)
        }
    }

    pub fn set_speed(&mut self, speed: i32) {
        unsafe { HAL_SetSPISpeed(self.0, speed); }
    }

    pub fn set_spi_mode(&mut self, mode: SPIMode) {
        unsafe { HAL_SetSPIMode(self.0, mode); }
    }

    pub fn get_spi_mode(&self) -> SPIMode {
        unsafe { HAL_GetSPIMode(self.0) }
    }

    pub fn set_chip_select_active_high(&mut self) -> HALResult<()> {
        hal_call!(HAL_SetSPIChipSelectActiveHigh(self.0))
    }

    pub fn set_chip_select_active_low(&mut self) -> HALResult<()> {
        hal_call!(HAL_SetSPIChipSelectActiveLow(self.0))
    }

    pub fn get_handle(&self) -> i32 {
        unsafe { HAL_GetSPIHandle(self.0) }
    }

    pub fn set_handle(&mut self, handle: i32)  {
        unsafe { HAL_SetSPIHandle(self.0, handle); }
    }

    pub fn port(&self) -> SPIPort {
        self.0
    }

    pub fn spi_auto(&'_ self, buffer_size: i32) -> HALResult<SPIAuto<'_>> {
        hal_call!(HAL_InitSPIAuto(self.0, buffer_size))?;
        Ok(SPIAuto(self))
    }

}

impl Drop for SPI {
    fn drop(&mut self) {
        unsafe { HAL_CloseSPI(self.0); }
    }
}

pub struct SPIAuto<'a>(&'a SPI);

impl<'a> SPIAuto<'a> {
    pub fn start_rate(&self, period: f64) -> HALResult<()> {
        hal_call!(HAL_StartSPIAutoRate(self.port(), period))
    }

    pub fn start_trigger(&self, digital_source: DigitalSource, trigger_rising: bool, trigger_falling: bool) -> HALResult<()> {
        hal_call!(HAL_StartSPIAutoTrigger(
            self.port(),
            digital_source.raw_handle(),
            digital_source.analog_trigger_type(),
            trigger_rising as i32,
            trigger_falling as i32
        ))
    }

    pub fn stop(&self) -> HALResult<()> {
        hal_call!(HAL_StopSPIAuto(self.port()))
    }

    pub fn set_transmit_data(&self, data: &[u8], zero_size: i32) -> HALResult<()> {
        hal_call!(HAL_SetSPIAutoTransmitData(self.port(), data.as_ptr(), data.len() as i32, zero_size))
    }
    
    pub fn force_read(&self) -> HALResult<()> {
        hal_call!(HAL_ForceSPIAutoRead(self.port()))
    }

    pub fn read_received_data(&self, data: &mut [u32], timeout: f64) -> HALResult<usize> {
        Ok(hal_call!(HAL_ReadSPIAutoReceivedData(self.port(), data.as_mut_ptr(), data.len() as i32, timeout))? as usize)
    }

    pub fn get_dropped_count(&self) -> HALResult<i32> {
        hal_call!(HAL_GetSPIAutoDroppedCount(self.port()))
    }

    pub fn configure_auto_stall(&self, cs_to_sclk_ticks: i32, stall_ticks: i32, pow_2_bytes_per_read: i32) -> HALResult<()> {
        hal_call!(HAL_ConfigureSPIAutoStall(self.port(), cs_to_sclk_ticks, stall_ticks, pow_2_bytes_per_read))
    }

    pub fn port(&self) -> SPIPort {
        self.0.0
    }
}

impl<'a> Drop for SPIAuto<'a> {
    fn drop(&mut self) {
        hal_call!(HAL_FreeSPIAuto(self.0.0)).ok();
    }
}