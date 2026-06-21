use crate::halsim::callbacks::{
    BufferCallback, CallbackHandle, ConstBufferCallback, buffer_callback_trampoline,
    const_buffer_callback_trampoline,
};

use crate::halsim::halsim_data;

halsim_data!(I2C { initialized: bool });

impl I2C {
    pub fn register_read_callback<C: BufferCallback>(&self, callback: C) -> CallbackHandle<C> {
        crate::halsim::callbacks::register_callback!(
            HALSIM_RegisterI2CReadCallback,
            HALSIM_CancelI2CReadCallback,
            buffer_callback_trampoline::<C>,
            callback,
            index: self.0,
        )
    }

    pub fn register_write_callback<C: ConstBufferCallback>(
        &self,
        callback: C,
    ) -> CallbackHandle<C> {
        crate::halsim::callbacks::register_callback!(
            HALSIM_RegisterI2CWriteCallback,
            HALSIM_CancelI2CWriteCallback,
            const_buffer_callback_trampoline::<C>,
            callback,
            index: self.0,
        )
    }
}
