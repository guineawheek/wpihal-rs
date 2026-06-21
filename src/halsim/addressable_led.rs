use crate::halsim::callbacks::{CallbackHandle, ConstBufferCallback};

use super::halsim_data;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddressableLEDBuffer;
impl AddressableLEDBuffer {
    pub fn register_callback<C: ConstBufferCallback>(&self, callback: C) -> CallbackHandle<C> {
        crate::halsim::callbacks::register_callback!(
            HALSIM_RegisterAddressableLEDDataCallback,
            HALSIM_CancelAddressableLEDDataCallback,
            crate::halsim::callbacks::const_buffer_callback_trampoline::<C>,
            callback,
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

halsim_data!(AddressableLED {
    initialized: bool,
    start: i32,
    length: i32
});
