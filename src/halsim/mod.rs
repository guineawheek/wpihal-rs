pub mod addressable_led;
pub mod callbacks;

pub use callbacks::*;

/// Trait for HALSIM callbacks.
pub trait HalSimValue<T> {
    fn register_callback<C: NotifyCallback>(
        &self,
        callback: C,
        initial_notify: bool,
    ) -> CallbackHandle<C>;
    fn get(&self) -> T;
    fn set(&self, value: T);
}

macro_rules! halsim_convert_get {
    ($e:expr => bool) => {
        $e != 0
    };
    ($e:expr => $t:ty) => {
        $e
    };
}

macro_rules! halsim_convert_set {
    ($e:expr => bool) => {
        $e as _
    };
    ($e:expr => $t:ty) => {
        $e
    };
}

macro_rules! halsim_value {
    ($name:ident::<$t:ty>$(($idx:ty))?) => {
        paste::paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $name $((pub $idx))?;
            impl $crate::halsim::HalSimValue<$t> for $name {
                fn register_callback<C: NotifyCallback>(&self, callback: C, initial_notify: bool) -> CallbackHandle<C> {
                    $crate::halsim::callbacks::register_callback!(
                        [< HALSIM_Register $name Callback >],
                        [< HALSIM_Cancel $name Callback >],
                        $crate::halsim::notify_callback_trampoline::<C>,
                        callback,
                        initial_notify,
                        $({ let v: $idx = self.0; v })?
                    )
                }

                fn get(&self) -> $t {
                    $crate::halsim::halsim_convert_get!(
                        unsafe {
                            wpihal_sys::[< HALSIM_Get $name >]($({ let v: $idx = self.0; v })?)
                        } => $t
                    )
                }

                fn set(&self, value: $t) {
                    unsafe {
                        wpihal_sys::[< HALSIM_Set $name >](
                            $({ let v: $idx = self.0; v },)?
                            $crate::halsim::halsim_convert_set!(value => $t)
                        );
                    }
                }
            }
        }
    };
}
pub(crate) use halsim_convert_get;
pub(crate) use halsim_convert_set;
pub(crate) use halsim_value;
