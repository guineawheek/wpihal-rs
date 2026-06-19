use std::ffi::CStr;

use wpihal_sys::HAL_Value;

use crate::value::HALValue;

macro_rules! callback_trait {
    ($name:ident::$trampoline_name:ident($($arg_name:ident: $arg_ty:ty),+ $(,)?) -> $value_ty:ty {
        $trampoline_map:expr
    }) => {

        pub trait $name {
            #[doc = "Callback"]
            fn callback(&mut self, name: &str, value: $value_ty);
        }

        impl<T> $name for T where T: FnMut(&str, $value_ty) {
            fn callback(&mut self, name: &str, value: $value_ty) {
                (self)(name, value);
            }
        }

        /// Trampoline function
        pub unsafe extern "C" fn $trampoline_name<C: $name>(
            name: *const core::ffi::c_char,
            param: *mut core::ffi::c_void,
            $(
                $arg_name: $arg_ty
            ),+
        ) {
            let Some(ptr) = core::ptr::NonNull::new(param) else {
                return;
            };
            // SAFETY: Wouldn't YOU like to find out how good wpilib maintains it code?
            unsafe {
                ptr.cast::<C>().as_mut().callback(
                    str::from_utf8_unchecked(CStr::from_ptr(name).to_bytes()),
                    $trampoline_map
                );
            }
        }
    };
}

callback_trait!(NotifyCallback::notify_callback_trampoline(value: *const HAL_Value) -> HALValue {
    value.read().into()
});

callback_trait!(BufferCallback::buffer_callback_trampoline(buffer: *mut u8, count: u32) -> &mut [u8] {
    core::slice::from_raw_parts_mut(buffer, count as usize)
});

callback_trait!(ConstBufferCallback::const_buffer_callback_trampoline(buffer: *const u8, count: u32) -> &[u8] {
    core::slice::from_raw_parts(buffer, count as usize)
});

#[derive(Debug, Clone)]
enum Cancel {
    Simple(unsafe extern "C" fn(i32)),
    Indexed {
        index: i32,
        cancel: unsafe extern "C" fn(i32, i32),
    },
}

#[derive(Debug)]
pub struct CallbackHandle<C> {
    uid: i32,
    _callback: Box<C>,
    cancel: Cancel,
}

impl<C> CallbackHandle<C> {
    pub fn new(uid: i32, callback: Box<C>, cancel: unsafe extern "C" fn(i32)) -> Self {
        Self {
            uid,
            _callback: callback,
            cancel: Cancel::Simple(cancel),
        }
    }
    pub fn new_indexed(
        index: i32,
        uid: i32,
        callback: Box<C>,
        cancel: unsafe extern "C" fn(i32, i32),
    ) -> Self {
        Self {
            uid,
            _callback: callback,
            cancel: Cancel::Indexed { index, cancel },
        }
    }
}

macro_rules! register_callback {
    ($register:ident, $cancel:ident, $trampoline:expr, $callback:expr, $initial_notify:expr, $index:expr) => {{
        let callback = Box::new($callback);
        let uid = unsafe {
            wpihal_sys::$register(
                $index,
                Some($trampoline),
                core::ptr::NonNull::from_ref(callback.as_ref()).cast::<core::ffi::c_void>().as_ptr(),
                $initial_notify as i32,
            )
        };
        $crate::halsim::CallbackHandle::new_indexed(
            $index,
            uid,
            callback,
            wpihal_sys::$cancel,
        )
    }};
    ($register:ident, $cancel:ident, $trampoline:expr, $callback:expr $(, $initial_notify:expr)?) => {{
        let callback = Box::new($callback);
        let uid = unsafe {
            wpihal_sys::$register(
                Some($trampoline),
                core::ptr::NonNull::from_ref(callback.as_ref()).cast::<core::ffi::c_void>().as_ptr(),
                $($initial_notify as i32,)?
            )
        };
        $crate::halsim::CallbackHandle::new(
            uid,
            callback,
            wpihal_sys::$cancel,
        )
    }};
}
pub(crate) use register_callback;

impl<C> Drop for CallbackHandle<C> {
    fn drop(&mut self) {
        unsafe {
            match self.cancel {
                Cancel::Simple(cancel) => cancel(self.uid),
                Cancel::Indexed { index, cancel } => cancel(index, self.uid),
            }
        }
    }
}
