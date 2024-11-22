use wpihal_sys::hal::{HAL_AddressableLEDData, HAL_AddressableLEDHandle, HAL_DigitalHandle, HAL_FreeAddressableLED, HAL_InitializeAddressableLED, HAL_SetAddressableLEDBitTiming, HAL_SetAddressableLEDLength, HAL_SetAddressableLEDSyncTime, HAL_StartAddressableLEDOutput, HAL_StopAddressableLEDOutput, HAL_WriteAddressableLEDData};

use crate::{error::HALResult, hal_call};

pub const ADDRESSABLE_LED_MAX_LENGTH: u32 = wpihal_sys::hal::HAL_kAddressableLEDMaxLength;


pub type AddressableLEDData = HAL_AddressableLEDData;
pub struct AddressableLED(HAL_AddressableLEDHandle);
impl AddressableLED {
    pub fn initialize(output_port: HAL_DigitalHandle) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeAddressableLED(output_port))?))
    }

    pub fn set_length(&mut self, length: u32) -> HALResult<()> {
        Ok(hal_call!(HAL_SetAddressableLEDLength(self.0, length as i32))?)
    }

    pub fn write_data(&mut self, data: &[AddressableLEDData]) -> HALResult<()> {
        Ok(hal_call!(HAL_WriteAddressableLEDData(self.0, data.as_ptr(), data.len() as i32))?)
    }

    pub fn set_led_bit_timing(&mut self, high_time_0_ns: i32, low_time_0_ns: i32, high_time_1_ns: i32, low_time_1_ns: i32) -> HALResult<()> {
        Ok(hal_call!(HAL_SetAddressableLEDBitTiming(self.0, high_time_0_ns, low_time_0_ns, high_time_1_ns, low_time_1_ns))?)
    }

    pub fn set_sync_time(&mut self, sync_time_us: i32) -> HALResult<()> {
        Ok(hal_call!(HAL_SetAddressableLEDSyncTime(self.0, sync_time_us))?)
    }

    pub fn start_output(&mut self) -> HALResult<()> {
        Ok(hal_call!(HAL_StartAddressableLEDOutput(self.0))?)
    }

    pub fn stop_output(&mut self) -> HALResult<()> {
        Ok(hal_call!(HAL_StopAddressableLEDOutput(self.0))?)
    }

}

impl Drop for AddressableLED {
    fn drop(&mut self) {
        unsafe { HAL_FreeAddressableLED(self.0); }
    }
}