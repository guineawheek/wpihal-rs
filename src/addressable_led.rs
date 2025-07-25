use std::ffi::CStr;

use wpihal_sys::{
    HAL_AddressableLEDColorOrder, HAL_AddressableLEDData, HAL_AddressableLEDHandle,
    HAL_FreeAddressableLED, HAL_InitializeAddressableLED, HAL_SetAddressableLEDData,
    HAL_SetAddressableLEDLength,
};

use crate::{
    error::{HALResult, allocation_location_ptr},
    hal_call,
};

pub const ADDRESSABLE_LED_MAX_LENGTH: u32 = wpihal_sys::HAL_kAddressableLEDMaxLength;

pub type AddressableLEDData = HAL_AddressableLEDData;

/// Addressable LEDs.
///
/// Underlying impl is NOT thread-safe.
#[derive(Debug, PartialEq, Eq)]
pub struct AddressableLED(HAL_AddressableLEDHandle);

impl AddressableLED {
    /// Initialize an addressible LED strip handle using a digital handle.
    pub fn initialize(channel: i32, allocation_location: Option<&CStr>) -> HALResult<Self> {
        // TODO: make a real handle
        Ok(Self(hal_call!(HAL_InitializeAddressableLED(
            channel,
            allocation_location_ptr(allocation_location)
        ))?))
    }

    /// Sets the buffer start of the LED strip.
    ///
    /// THe max length is 1024 LEDs.
    pub fn set_start(&mut self, start: usize) -> HALResult<()> {
        Ok(hal_call!(HAL_SetAddressableLEDLength(
            self.0,
            start as i32
        ))?)
    }

    /// Sets the length of the LED strip.
    ///
    /// THe max length is 1024 LEDs.
    pub fn set_length(&mut self, length: u32) -> HALResult<()> {
        Ok(hal_call!(HAL_SetAddressableLEDLength(
            self.0,
            length as i32
        ))?)
    }
}

/// Updates the led output data buffer.
pub fn set_data(
    start: usize,
    order: HAL_AddressableLEDColorOrder,
    data: &[AddressableLEDData],
) -> HALResult<()> {
    Ok(hal_call!(HAL_SetAddressableLEDData(
        start as i32,
        data.len() as i32,
        order,
        data.as_ptr(),
    ))?)
}

impl Drop for AddressableLED {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeAddressableLED(self.0);
        }
    }
}
