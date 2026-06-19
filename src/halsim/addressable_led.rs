use crate::halsim::{CallbackHandle, ConstBufferCallback, NotifyCallback};

use super::halsim_value;

halsim_value!(AddressableLEDInitialized::<bool>(i32));
halsim_value!(AddressableLEDStart::<i32>(i32));
halsim_value!(AddressableLEDLength::<i32>(i32));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddressableLEDData;
impl AddressableLEDData {
    pub fn register_callback<C: ConstBufferCallback>(&self, callback: C) -> CallbackHandle<C> {
        crate::halsim::callbacks::register_callback!(
            HALSIM_RegisterAddressableLEDDataCallback,
            HALSIM_CancelAddressableLEDDataCallback,
            crate::halsim::const_buffer_callback_trampoline::<C>,
            callback
        )
    }

    pub fn get(
        &self,
        start: usize,
        data: &mut [crate::addressable_led::AddressableLEDData],
    ) -> usize {
        let start = start.min(i32::MAX as usize) as i32;
        let length = data.len().min(i32::MAX as usize) as i32;
        unsafe {
            wpihal_sys::HALSIM_GetAddressableLEDData(start, length, data.as_mut_ptr()) as usize
        }
    }

    pub fn set(&self, start: usize, data: &[crate::addressable_led::AddressableLEDData]) {
        let start = start.min(i32::MAX as usize) as i32;
        let length = data.len().min(i32::MAX as usize) as i32;
        unsafe {
            wpihal_sys::HALSIM_SetAddressableLEDData(start, length, data.as_ptr());
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddressableLED(pub i32);

impl AddressableLED {
    pub fn reset(&self) {
        unsafe {
            wpihal_sys::HALSIM_ResetAddressableLEDData(self.0);
        }
    }

    pub const fn initialized(&self) -> AddressableLEDInitialized {
        AddressableLEDInitialized(self.0)
    }

    pub const fn length(&self) -> AddressableLEDLength {
        AddressableLEDLength(self.0)
    }
}
