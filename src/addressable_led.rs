use wpihal_sys::{HAL_AddressableLEDData, HAL_AddressableLEDHandle, HAL_FreeAddressableLED, HAL_InitializeAddressableLED, HAL_SetAddressableLEDBitTiming, HAL_SetAddressableLEDLength, HAL_SetAddressableLEDSyncTime, HAL_StartAddressableLEDOutput, HAL_StopAddressableLEDOutput, HAL_WriteAddressableLEDData};

use crate::{dio::DIO, error::HALResult, hal_call, Handle};

pub const ADDRESSABLE_LED_MAX_LENGTH: u32 = wpihal_sys::HAL_kAddressableLEDMaxLength;


pub type AddressableLEDData = HAL_AddressableLEDData;
#[derive(Debug, PartialEq, Eq)]
pub struct AddressableLED<'a>(HAL_AddressableLEDHandle, &'a DIO);
impl<'a> AddressableLED<'a> {
    /// Initialize an addressible LED strip handle using a digital handle.
    pub fn initialize(output_port: &'a DIO) -> HALResult<Self> {
        // TODO: make a real handle
        Ok(Self(hal_call!(HAL_InitializeAddressableLED(output_port.raw_handle()))?, output_port))
    }

    /// Sets the length of the LED strip.
    /// 
    /// THe max length is 5460 LEDs.
    pub fn set_length(&mut self, length: u32) -> HALResult<()> {
        Ok(hal_call!(HAL_SetAddressableLEDLength(self.0, length as i32))?)
    }

    /// Sets the led output data
    /// 
    /// Output data is buffered and synchronized such that this is safe to call even if output is enabled.
    pub fn write_data(&mut self, data: &[AddressableLEDData]) -> HALResult<()> {
        Ok(hal_call!(HAL_WriteAddressableLEDData(self.0, data.as_ptr(), data.len() as i32))?)
    }

    /// Sets the LED bit timing.
    /// By default the driver is setup to drive ws2812bs.
    /// 
    /// * `high_time_0_ns` - defaults to 400 ns
    /// * `low_time_0_ns` - defaults to 900 ns
    /// * `high_time_1_ns` - defaults to 900 ns
    /// * `low_time_1_ns` - defaults to 600 ns
    pub fn set_led_bit_timing(&mut self, high_time_0_ns: i32, low_time_0_ns: i32, high_time_1_ns: i32, low_time_1_ns: i32) -> HALResult<()> {
        Ok(hal_call!(HAL_SetAddressableLEDBitTiming(self.0, high_time_0_ns, low_time_0_ns, high_time_1_ns, low_time_1_ns))?)
    }

    /// Sets the sync time.
    /// 
    /// The sync time is the time to hold output so LEDs enable. Default set for WS2812B.
    /// 
    /// * `sync_time_us` - defaults to 280 us
    pub fn set_sync_time(&mut self, sync_time_us: i32) -> HALResult<()> {
        Ok(hal_call!(HAL_SetAddressableLEDSyncTime(self.0, sync_time_us))?)
    }

    /// Starts LED output.
    pub fn start_output(&mut self) -> HALResult<()> {
        Ok(hal_call!(HAL_StartAddressableLEDOutput(self.0))?)
    }

    /// Stops LED output.
    pub fn stop_output(&mut self) -> HALResult<()> {
        Ok(hal_call!(HAL_StopAddressableLEDOutput(self.0))?)
    }

}

impl<'a> Drop for AddressableLED<'a> {
    fn drop(&mut self) {
        unsafe { HAL_FreeAddressableLED(self.0); }
    }
}