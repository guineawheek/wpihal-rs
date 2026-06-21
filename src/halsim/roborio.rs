use wpiutil::{WPIString, as_wpistr};

macro_rules! halsim_rc_value {
    ($name:ident, &str) => {
        paste::paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $name;
            impl $name {
                pub fn register_callback<C: crate::halsim::callbacks::StringCallback>(
                    &self,
                    callback: C,
                    initial_notify: bool
                ) -> $crate::halsim::callbacks::CallbackHandle<C> {
                    $crate::halsim::callbacks::register_callback!(
                        [< HALSIM_Register $name Callback >],
                        [< HALSIM_Cancel $name Callback >],
                        $crate::halsim::callbacks::string_callback_trampoline::<C>,
                        callback,
                        initial_notify: initial_notify,
                    )
                }

                pub fn get(&self) -> WPIString {
                    unsafe {
                        WPIString::from_raw_ctx(|out| wpihal_sys::[< HALSIM_Get $name >](out))
                    }
                }

                pub fn set(&self, value: &str) {
                    unsafe {
                        wpihal_sys::[< HALSIM_Set $name >](as_wpistr!(value));
                    }
                }
            }
        }
    };
    ($name:ident, $aty:ty) => {
        $crate::halsim::halsim_value!($name::<$aty>());
    };
}

macro_rules! halsim_rc {
    (
        $name:ident {
            $($aname:ident($acamel:ident): $aty:ty),*
            $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name;

        paste::paste! {
            $(
                halsim_rc_value!([<$name $acamel>], $aty);
            )*

            impl $name {
                pub fn reset_data(&self) {
                    unsafe {
                        wpihal_sys::[<HALSIM_Reset $name Data>]();
                    }
                }

                $(
                    pub const fn $aname(&self) -> [<$name $acamel>] {
                        [<$name $acamel>]
                    }
                )*
            }
        }

    };
}

halsim_rc!(RoboRio {
    vin_voltage(VInVoltage): f64,
    user_voltage_3v3(UserVoltage3V3): f64,
    user_current_3v3(UserCurrent3V3): f64,
    user_active_3v3(UserActive3V3): bool,
    user_faults_3v3(UserFaults3V3): i32,
    brownout_voltage(BrownoutVoltage): f64,
    team_number(TeamNumber): i32,
    serial_number(SerialNumber): &str,
    comments(Comments): &str,
    cpu_temp(CPUTemp): f64,
});

impl RoboRio {}
