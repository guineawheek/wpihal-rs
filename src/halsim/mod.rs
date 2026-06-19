pub mod addressable_led;
pub mod alert;
pub mod analog_in;
pub mod can;
pub mod ctre_pcm;
pub mod digital_pwm;
pub mod dio;

// pub mod driver_station;
pub mod duty_cycle;
pub mod encoder;
pub mod i2c;
pub mod imu;
pub mod mock_hooks;

// not completed
pub mod driver_station;
pub mod notifier;
pub mod power_distribution;
pub mod pwm;
pub mod rev_ph;
pub mod roborio;
pub mod sim_device;

pub mod callbacks;

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
    ($name:ident::<$t:ty>($($idx:ty $(, $chn:ty)?)?)) => {
        paste::paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $name $((pub $idx $(, pub $chn)?))?;
            impl $name {
                pub fn register_callback<C: $crate::halsim::callbacks::NotifyCallback>(
                    &self,
                    callback: C,
                    initial_notify: bool
                ) -> $crate::halsim::callbacks::CallbackHandle<C> {
                    $crate::halsim::callbacks::register_callback!(
                        [< HALSIM_Register $name Callback >],
                        [< HALSIM_Cancel $name Callback >],
                        $crate::halsim::callbacks::notify_callback_trampoline::<C>,
                        callback,
                        initial_notify,
                        $(
                            { let v: $idx = self.0; v }
                            $(, { let v: $chn = self.1; v})?
                        )?
                    )
                }

                pub fn get(&self) -> $t {
                    $crate::halsim::halsim_convert_get!(
                        unsafe {
                            wpihal_sys::[< HALSIM_Get $name >](
                                $(
                                    { let v: $idx = self.0; v }
                                    $(, { let v: $chn = self.1; v})?
                                )?
                            )
                        } => $t
                    )
                }

                pub fn set(&self, value: $t) {
                    unsafe {
                        wpihal_sys::[< HALSIM_Set $name >](
                            $(
                                { let v: $idx = self.0; v },
                                $({ let v: $chn = self.1; v},)?
                            )?

                            $crate::halsim::halsim_convert_set!(value => $t)
                        );
                    }
                }
            }
        }
    };
}

macro_rules! halsim_data {
    (
        $name:ident {
            $($aname:ident: $aty:ty),*
            $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub i32);

        paste::paste! {
            $(
                crate::halsim::halsim_value!([<$name $aname:camel>]::<$aty>(i32));
            )*

            impl $name {
                pub fn reset_data(&self) {
                    unsafe {
                        wpihal_sys::[<HALSIM_Reset $name Data>](self.0);
                    }
                }

                $(
                    pub const fn $aname(&self) -> [<$name $aname:camel>] {
                        [<$name $aname:camel>](self.0)
                    }
                )*
            }
        }

    };
}

pub(crate) use halsim_convert_get;
pub(crate) use halsim_convert_set;
pub(crate) use halsim_data;
pub(crate) use halsim_value;
