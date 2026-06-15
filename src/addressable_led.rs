use std::ffi::CStr;

use wpihal_sys::{
    HAL_AddressableLEDColorOrder, HAL_AddressableLEDData, HAL_AddressableLEDHandle,
    HAL_FreeAddressableLED, HAL_InitializeAddressableLED, HAL_SetAddressableLEDData,
    HAL_SetAddressableLEDLength, HAL_SetAddressableLEDStart,
};

use crate::{
    error::{HALResult, allocation_location_ptr},
    hal_call,
};

/// The maximum number of addressible LEDs that can be controlled.
pub const ADDRESSABLE_LED_MAX_LENGTH: u32 = wpihal_sys::HAL_ADDRESSABLE_LED_MAX_LEN;

/// Structure for holding RGB LED data.
pub type AddressableLEDData = HAL_AddressableLEDData;
/// The ordering that color data is transmitted onto the WS2812.
pub type AddressibleLEDColorOrder = HAL_AddressableLEDColorOrder;

/// Addressable LEDs.
///
/// Underlying impl is NOT thread-safe.
#[derive(Debug, PartialEq, Eq)]
pub struct AddressableLED(HAL_AddressableLEDHandle);

impl AddressableLED {
    /// Initialize an addressible LED strip handle using a digital handle.
    pub fn initialize(channel: i32, allocation_location: Option<&CStr>) -> HALResult<Self> {
        // TODO: make a real handle
        hal_call!(HAL_InitializeAddressableLED(
            channel,
            allocation_location_ptr(allocation_location)
        ))
        .map(Self)
    }

    /// Sets the buffer start of the LED strip.
    ///
    /// THe max length is 1024 LEDs.
    pub fn set_start(&mut self, start: usize) -> HALResult<()> {
        hal_call!(HAL_SetAddressableLEDStart(self.0, start as i32))
    }

    /// Sets the length of the LED strip.
    ///
    /// THe max length is 1024 LEDs.
    pub fn set_length(&mut self, length: u32) -> HALResult<()> {
        hal_call!(HAL_SetAddressableLEDLength(self.0, length as i32))
    }
}

/// Updates the led output data buffer.
///
/// All addressible LEDs pull from the buffer set here.
pub fn set_data(
    start: usize,
    order: HAL_AddressableLEDColorOrder,
    data: &[AddressableLEDData],
) -> HALResult<()> {
    hal_call!(HAL_SetAddressableLEDData(
        start as i32,
        data.len() as i32,
        order,
        data.as_ptr(),
    ))
}

impl Drop for AddressableLED {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeAddressableLED(self.0);
        }
    }
}
