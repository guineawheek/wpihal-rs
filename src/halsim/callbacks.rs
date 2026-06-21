use wpihal_sys::HAL_Value;

use crate::value::HALValue;

macro_rules! callback_trait {
    (
        $name:ident($($cb_arg_name:ident: $cb_arg_ty:ty),* $(,)?) $(-> $cb_ret: ty)?,
        |$cb:ident, $name_str:ident, $($arg_name:ident: $arg_ty:ty),* $(,)?| { $trampoline_map:stmt }
    ) => {
        paste::paste! {
            pub trait $name {
                #[doc = "Callback"]
                fn callback(&mut self, name: &str, $($cb_arg_name: $cb_arg_ty),* ) $(-> $cb_ret)?;
            }

            impl<T> $name for T where T: FnMut(&str, $($cb_arg_ty),*) $(-> $cb_ret)? {
                fn callback(&mut self, name: &str, $($cb_arg_name: $cb_arg_ty),*) $(-> $cb_ret)? {
                    (self)(name, $($cb_arg_name),*)
                }
            }

            /// Trampoline function
            pub unsafe extern "C" fn [< $name:snake _trampoline >] <C: $name>(
                name: *const core::ffi::c_char,
                param: *mut core::ffi::c_void,
                $(
                    $arg_name: $arg_ty
                ),*
            ) {
                let Some(ptr) = core::ptr::NonNull::new(param) else {
                    return;
                };
                // SAFETY: Wouldn't YOU like to find out how good wpilib maintains it code?
                unsafe {
                    let $cb = ptr.cast::<C>().as_mut();
                    let $name_str = str::from_utf8_unchecked(std::ffi::CStr::from_ptr(name).to_bytes());

                    {
                        $trampoline_map
                    }
                }
            }
        }
    };
}
pub(crate) use callback_trait;

callback_trait!(NotifyCallback(value: HALValue), |callback, name, value: *const HAL_Value| {
    callback.callback(name, value.read().into())
});

callback_trait!(BufferCallback(value: &mut [u8]), |callback, name, buffer: *mut u8, count: u32| {
    callback.callback(name, core::slice::from_raw_parts_mut(buffer, count as usize))
});

callback_trait!(ConstBufferCallback(value: &[u8]), |callback, name, buffer: *const u8, count: u32| {
    callback.callback(name, core::slice::from_raw_parts(buffer, count as usize))
});

callback_trait!(StringCallback(value: &str), |callback, name, s: *const core::ffi::c_char, size: usize| {
    callback.callback(name, str::from_utf8_unchecked(core::slice::from_raw_parts(s.cast::<u8>(), size)))
});

#[derive(Debug, Clone)]
enum Cancel {
    Simple(unsafe extern "C" fn(i32)),
    Indexed {
        index: i32,
        cancel: unsafe extern "C" fn(i32, i32),
    },
    Channeled {
        index: i32,
        channel: i32,
        cancel: unsafe extern "C" fn(i32, i32, i32),
    },
}

#[derive(Debug)]
pub struct CallbackHandle<C> {
    uid: i32,
    _callback: Box<C>,
    cancel: Cancel,
}

impl<C> CallbackHandle<C> {
    pub const fn new(uid: i32, callback: Box<C>, cancel: unsafe extern "C" fn(i32)) -> Self {
        Self {
            uid,
            _callback: callback,
            cancel: Cancel::Simple(cancel),
        }
    }
    pub const fn new_indexed(
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
    pub fn new_channeled(
        index: i32,
        channel: i32,
        uid: i32,
        callback: Box<C>,
        cancel: unsafe extern "C" fn(i32, i32, i32),
    ) -> Self {
        Self {
            uid,
            _callback: callback,
            cancel: Cancel::Channeled {
                index,
                channel,
                cancel,
            },
        }
    }
}

macro_rules! register_callback {
    (
        $register:ident,
        $cancel:ident,
        $trampoline:expr,
        $callback:expr,
        index: $index:expr,
        channel: $channel:expr,
        $(initial_notify: $initial_notify:expr,)?
        ) => {{
        let callback = Box::new($callback);
        let uid = unsafe {
            wpihal_sys::$register(
                $index,
                $channel,
                Some($trampoline),
                core::ptr::NonNull::from_ref(callback.as_ref()).cast::<core::ffi::c_void>().as_ptr(),
                $($initial_notify as i32,)?
            )
        };
        $crate::halsim::callbacks::CallbackHandle::new_channeled(
            $index,
            $channel,
            uid,
            callback,
            wpihal_sys::$cancel,
        )
    }};
    (
        $register:ident,
        $cancel:ident,
        $trampoline:expr,
        $callback:expr,
        index: $index:expr,
        $(initial_notify: $initial_notify:expr,)?
    ) => {{
        let callback = Box::new($callback);
        let uid = unsafe {
            wpihal_sys::$register(
                $index,
                Some($trampoline),
                core::ptr::NonNull::from_ref(callback.as_ref()).cast::<core::ffi::c_void>().as_ptr(),
                $($initial_notify as i32,)?
            )
        };
        $crate::halsim::callbacks::CallbackHandle::new_indexed(
            $index,
            uid,
            callback,
            wpihal_sys::$cancel,
        )
    }};
    ($register:ident, $cancel:ident, $trampoline:expr, $callback:expr, $(initial_notify: $initial_notify:expr,)?) => {{
        let callback = Box::new($callback);
        let uid = unsafe {
            wpihal_sys::$register(
                Some($trampoline),
                core::ptr::NonNull::from_ref(callback.as_ref()).cast::<core::ffi::c_void>().as_ptr(),
                $($initial_notify as i32,)?
            )
        };
        $crate::halsim::callbacks::CallbackHandle::new(
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
                Cancel::Channeled {
                    index,
                    channel,
                    cancel,
                } => cancel(index, channel, self.uid),
            }
        }
    }
}
