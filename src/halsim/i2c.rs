use crate::halsim::callbacks::{
    BufferCallback, CallbackHandle, ConstBufferCallback, buffer_callback_trampoline,
    const_buffer_callback_trampoline,
};

use crate::halsim::halsim_data;

halsim_data!(I2C { initialized: bool });

impl I2C {
    pub fn register_read_callback<C: BufferCallback>(&self, callback: C) -> CallbackHandle<C> {
        let callback = Box::new(callback);
        let uid = unsafe {
            wpihal_sys::HALSIM_RegisterI2CReadCallback(
                self.0,
                Some(buffer_callback_trampoline::<C>),
                core::ptr::NonNull::from_ref(callback.as_ref())
                    .cast::<core::ffi::c_void>()
                    .as_ptr(),
            )
        };
        CallbackHandle::new_indexed(
            self.0,
            uid,
            callback,
            wpihal_sys::HALSIM_CancelI2CReadCallback,
        )
    }

    pub fn register_write_callback<C: ConstBufferCallback>(
        &self,
        callback: C,
    ) -> CallbackHandle<C> {
        let callback = Box::new(callback);
        let uid = unsafe {
            wpihal_sys::HALSIM_RegisterI2CWriteCallback(
                self.0,
                Some(const_buffer_callback_trampoline::<C>),
                core::ptr::NonNull::from_ref(callback.as_ref())
                    .cast::<core::ffi::c_void>()
                    .as_ptr(),
            )
        };
        CallbackHandle::new_indexed(
            self.0,
            uid,
            callback,
            wpihal_sys::HALSIM_CancelI2CWriteCallback,
        )
    }
}
