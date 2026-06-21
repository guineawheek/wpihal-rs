use wpihal_sys::{HAL_MatchInfo, HAL_OpModeOption};

use crate::halsim::callbacks::callback_trait;

use crate::halsim::{halsim_accessor, impl_register_callback};
use crate::op_mode::{OpModeOption, OpModeOptionsArray};

macro_rules! halsim_ds {
    (
        $name:ident {
            $($aname:ident: $aty:ty),*
            $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name;

        paste::paste! {
            $(
                crate::halsim::halsim_value!([<$name $aname:camel>]::<$aty>());
            )*

            impl $name {
                pub fn reset_data(&self) {
                    unsafe {
                        wpihal_sys::[<HALSIM_Reset $name Data>]();
                    }
                }

                $(
                    pub const fn $aname(&self) -> [<$name $aname:camel>] {
                        [<$name $aname:camel>]
                    }
                )*
            }
        }

    };
}

callback_trait!(OpModeOptionsCallback(value: &[OpModeOption]), |callback, name, opmodes: *const HAL_OpModeOption, count: i32| {{
    if opmodes.is_null() {
        return;
    }
    callback.callback(name, core::slice::from_raw_parts(opmodes.cast(), count as usize));
}});

macro_rules! joystick_callback {
    ($name:ident) => {
        paste::paste! {
            callback_trait!(
                [<Joystick $name Callback>](joystick_num: i32, value: &wpihal_sys::[<HAL_Joystick $name>]),
                |callback, name, joystick_num: i32, value: *const wpihal_sys::[<HAL_Joystick $name>]| {{
                let Some(value) = value.as_ref() else {
                    return;
                };
                callback.callback(name, joystick_num, value);
            }});
        }
    };
}
joystick_callback!(Axes);
joystick_callback!(POVs);
joystick_callback!(Buttons);
joystick_callback!(Touchpads);
joystick_callback!(Descriptor);
callback_trait!(
    JoystickLedsCallback(joystick_num: i32, value: i32),
    |callback, name, joystick_num: i32, leds: i32| {
        callback.callback(name, joystick_num, leds)
    }
);
callback_trait!(
    JoystickRumblesCallback(joystick_num: i32, rumbles: crate::driver_station::JoystickRumble),
    |callback, name, joystick_num: i32, left_rumble: i32, right_rumble: i32, left_trigger_rumble: i32, right_trigger_rumble: i32| {{
        let rumbles = crate::driver_station::JoystickRumble::new(
            left_rumble,
            right_rumble,
            left_trigger_rumble,
            right_trigger_rumble,
        );
        callback.callback(name, joystick_num, rumbles);
    }}
);
callback_trait!(
    MatchInfoCallback(value: &HAL_MatchInfo),
    |callback, name, match_info: *const HAL_MatchInfo| {{
        if let Some(match_info) = match_info.as_ref() {
            callback.callback(name, match_info);
        }
    }}
);
callback_trait!(
    GameDataCallback(value: crate::driver_station::GameData),
    |callback, name, game_data: *const wpihal_sys::HAL_GameData| {{
        let Some(game_data) = game_data.as_ref() else {
            return;
        };
        callback.callback(name, core::mem::transmute(*game_data));
    }}
);

halsim_ds!(DriverStation {
    enabled: bool,
    robot_mode: crate::driver_station::RobotMode,
    e_stop: bool,
    fms_attached: bool,
    ds_attached: bool,
    alliance_station_id: crate::driver_station::AllianceStationId,
    match_time: f64,
    op_mode: i64,
});

pub struct DriverStationNewData;
impl_register_callback!(DriverStationNewData()<crate::halsim::callbacks, NotifyCallback>);
halsim_accessor!(DriverStation(), new_data);
impl DriverStation {
    pub fn notify_new_data(&self) {
        unsafe {
            wpihal_sys::HALSIM_NotifyDriverStationNewData();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpModeOptions;
impl_register_callback!(OpModeOptions()<crate::halsim::driver_station, OpModeOptionsCallback>);

impl OpModeOptions {
    pub fn get() -> OpModeOptionsArray {
        unsafe {
            let mut len = 0;
            let ptr = wpihal_sys::HALSIM_GetOpModeOptions(&mut len);
            OpModeOptionsArray(ptr.cast(), len as usize)
        }
    }
}

macro_rules! halsim_joystick_register_handler {
    ($name:ident) => {
        paste::paste!{
            impl $name {
                pub fn register_callback<C: [<$name Callback>]>(
                    &self,
                    callback: C,
                    initial_notify: bool
                ) -> $crate::halsim::callbacks::CallbackHandle<C> {
                    let callback = Box::new(callback);
                    let uid = unsafe {
                        wpihal_sys::[< HALSIM_Register $name Callback >](
                            self.0,
                            Some($crate::halsim::driver_station::[<$name:snake _callback_trampoline>]::<C>),
                            core::ptr::NonNull::from_ref(callback.as_ref()).cast::<core::ffi::c_void>().as_ptr(),
                            initial_notify as i32,
                        )
                    };
                    $crate::halsim::callbacks::CallbackHandle::new(
                        uid,
                        callback,
                        wpihal_sys::[< HALSIM_Cancel $name Callback >]
                    )
                }
            }
        }
    };
}
macro_rules! halsim_ref_access {
    ($name:ident($($idx:ty)?), $t:ty) => {
        paste::paste! {
            impl $name {
                pub fn get(&self) -> $t {
                    unsafe {
                        let mut data = core::mem::MaybeUninit::uninit();
                        wpihal_sys::[< HALSIM_Get $name>]($(self.0 as $idx,)? data.as_mut_ptr());
                        data.assume_init()
                    }
                }

                pub fn set(&self, value: &$t) {
                    unsafe {
                        wpihal_sys::[< HALSIM_Set $name >]($(self.0 as $idx,)? value);
                    }
                }
            }
        }
    };
}

macro_rules! halsim_joystick {
    (
        $name:ident {
            $($aname:ident: $aty:ty),*
            $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub i32);

        paste::paste! {
            impl $name {
                $(
                    pub const fn $aname(&self) -> [<$name $aty>] {
                        [<$name $aty>](self.0)
                    }
                )*
            }
            $(
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub struct [<$name $aty>](pub i32);
                halsim_joystick_register_handler!([<$name $aty>]);
                halsim_ref_access!([<$name $aty>](i32), wpihal_sys::[<HAL_ $name $aty>]);
            )+
        }
    };
}
halsim_joystick!(Joystick {
    axes: Axes,
    povs: POVs,
    buttons: Buttons,
    touchpads: Touchpads,
    descriptor: Descriptor,
});
halsim_accessor!(Joystick(i32), rumbles);
halsim_accessor!(Joystick(i32), leds);

impl Joystick {
    pub fn set_button(&self, button: i32, state: bool) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickButton(self.0, button, state as _);
        }
    }

    pub fn set_axis(&self, axis: i32, value: f64) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickAxis(self.0, axis, value);
        }
    }

    pub fn set_pov(&self, pov: i32, value: wpihal_sys::HAL_JoystickPOV) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickPOV(self.0, pov, value);
        }
    }

    pub fn set_buttons_value(&self, buttons: u64) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickButtonsValue(self.0, buttons);
        }
    }

    pub fn set_axes_available(&self, available: u16) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickAxesAvailable(self.0, available);
        }
    }

    pub fn set_povs_available(&self, available: u8) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickPOVsAvailable(self.0, available);
        }
    }

    pub fn set_buttons_available(&self, available: u64) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickButtonsAvailable(self.0, available);
        }
    }

    pub fn get_availables(&self) -> (u16, u64, u8) {
        let mut axes = 0u16;
        let mut buttons = 0u64;
        let mut povs = 0u8;
        unsafe {
            wpihal_sys::HALSIM_GetJoystickAvailables(self.0, &mut axes, &mut buttons, &mut povs);
        }
        (axes, buttons, povs)
    }

    pub fn set_touchpad_counts(&self, touchpad_count: u8, finger_count: &[u8]) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickTouchpadCounts(
                self.0,
                touchpad_count,
                finger_count.as_ptr(),
            );
        }
    }

    pub fn set_touchpad_finger(&self, touchpad: i32, finger: i32, down: bool, x: f64, y: f64) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickTouchpadFinger(self.0, touchpad, finger, down as _, x, y);
        }
    }

    pub fn set_is_gamepad(&self, is_gamepad: bool) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickIsGamepad(self.0, is_gamepad as _);
        }
    }

    pub fn set_gamepad_type(&self, gamepad_type: i32) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickGamepadType(self.0, gamepad_type);
        }
    }

    pub fn set_name(&self, name: &str) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickName(self.0, wpiutil::as_wpistr!(name));
        }
    }

    pub fn set_supported_outputs(&self, supported_outputs: i32) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickSupportedOutputs(self.0, supported_outputs);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JoystickRumbles(pub i32);
halsim_joystick_register_handler!(JoystickRumbles);
impl JoystickRumbles {
    pub fn get(&self) -> crate::driver_station::JoystickRumble {
        let mut left_rumble = 0;
        let mut right_rumble = 0;
        let mut left_trigger_rumble = 0;
        let mut right_trigger_rumble = 0;

        unsafe {
            wpihal_sys::HALSIM_GetJoystickRumbles(
                self.0,
                &mut left_rumble,
                &mut right_rumble,
                &mut left_trigger_rumble,
                &mut right_trigger_rumble,
            );
        }

        crate::driver_station::JoystickRumble::new(
            left_rumble,
            right_rumble,
            left_trigger_rumble,
            right_trigger_rumble,
        )
    }

    pub fn set(&self, rumbles: &crate::driver_station::JoystickRumble) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickRumbles(
                self.0,
                rumbles.left_rumble.into(),
                rumbles.right_rumble.into(),
                rumbles.left_trigger_rumble.into(),
                rumbles.right_trigger_rumble.into(),
            );
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JoystickLeds(pub i32);
halsim_joystick_register_handler!(JoystickLeds);
impl JoystickLeds {
    pub fn get(&self) -> i32 {
        unsafe {
            let mut data = core::mem::MaybeUninit::uninit();
            wpihal_sys::HALSIM_GetJoystickLeds(self.0, data.as_mut_ptr());
            data.assume_init()
        }
    }

    pub fn set(&self, value: i32) {
        unsafe {
            wpihal_sys::HALSIM_SetJoystickLeds(self.0, value);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchInfo;
impl_register_callback!(MatchInfo()<crate::halsim::driver_station, MatchInfoCallback>);
halsim_ref_access!(MatchInfo(), wpihal_sys::HAL_MatchInfo);
impl MatchInfo {
    /// up to first 64 bytes (or however many is still valid utf8) is accepted
    pub fn set_event_name(&self, event_name: &str) {
        unsafe {
            let trunc_name = str::from_utf8_unchecked(
                &event_name.as_bytes()[..event_name.floor_char_boundary(64)],
            );
            wpihal_sys::HALSIM_SetEventName(wpiutil::as_wpistr!(trunc_name));
        };
    }

    pub fn set_match_type(&self, match_type: wpihal_sys::HAL_MatchType) {
        unsafe {
            wpihal_sys::HALSIM_SetMatchType(match_type);
        }
    }

    pub fn set_match_number(&self, match_number: i32) {
        unsafe {
            wpihal_sys::HALSIM_SetMatchNumber(match_number);
        }
    }

    pub fn set_replay_number(&self, replay_number: i32) {
        unsafe {
            wpihal_sys::HALSIM_SetReplayNumber(replay_number);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameData;
impl_register_callback!(GameData()<crate::halsim::driver_station, GameDataCallback>);
impl GameData {
    pub fn get(&self) -> crate::driver_station::GameData {
        unsafe {
            let mut data = Default::default();
            wpihal_sys::HALSIM_GetGameData(&mut data);
            crate::driver_station::GameData::new(data)
        }
    }

    pub fn set(&self, game_data: crate::driver_station::GameData) {
        unsafe {
            wpihal_sys::HALSIM_SetGameData(&game_data.into());
        }
    }
}
