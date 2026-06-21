use wpihal_sys::{
    HAL_GameData, HAL_GetAllJoystickData, HAL_GetAllianceStation, HAL_GetGameData,
    HAL_GetJoystickAxes, HAL_GetJoystickButtons, HAL_GetJoystickDescriptor,
    HAL_GetJoystickGamepadType, HAL_GetJoystickIsGamepad, HAL_GetJoystickName, HAL_GetJoystickPOVs,
    HAL_GetJoystickSupportedOutputs, HAL_GetJoystickTouchpads, HAL_GetMatchInfo, HAL_GetMatchTime,
    HAL_GetOutputsEnabled, HAL_ObserveUserProgramStarting, HAL_RefreshDSData, HAL_SetJoystickLeds,
    HAL_SetJoystickRumble,
};
use wpiutil::wpistring::WPIString;

use crate::{
    error::{HALError, HALResult},
    hal_call, hal_retcall,
};

pub use wpihal_sys::HAL_AllianceStationID as AllianceStationId;
pub use wpihal_sys::HAL_JoystickAxes as JoystickAxes;
pub use wpihal_sys::HAL_JoystickButtons as JoystickButtons;
pub use wpihal_sys::HAL_JoystickDescriptor as JoystickDescriptor;
pub use wpihal_sys::HAL_JoystickPOVs as JoystickPOVs;
pub use wpihal_sys::HAL_JoystickTouchpads as JoystickTouchpads;
pub use wpihal_sys::HAL_MatchInfo as MatchInfo;
pub use wpihal_sys::HAL_MatchType as MatchType;
pub use wpihal_sys::HAL_RobotMode as RobotMode;

pub fn get_alliance_station() -> HALResult<AllianceStationId> {
    hal_call!(HAL_GetAllianceStation())
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct JoystickRumble {
    pub left_rumble: u16,
    pub right_rumble: u16,
    pub left_trigger_rumble: u16,
    pub right_trigger_rumble: u16,
}

impl JoystickRumble {
    pub const fn new(
        left_rumble: i32,
        right_rumble: i32,
        left_trigger_rumble: i32,
        right_trigger_rumble: i32,
    ) -> Self {
        Self {
            left_rumble: left_rumble as u16,
            right_rumble: right_rumble as u16,
            left_trigger_rumble: left_trigger_rumble as u16,
            right_trigger_rumble: right_trigger_rumble as u16,
        }
    }
}

/// Holding struct for all joystick data.
#[derive(Debug, Clone, Default)]
pub struct AllJoystickData {
    pub axes: JoystickAxes,
    pub povs: JoystickPOVs,
    pub buttons: JoystickButtons,
    pub touchpads: JoystickTouchpads,
}

/// Joystick of a given index, e.g. `Joystick(0)` is joystick 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Joystick(pub i32);

impl Joystick {
    pub fn axes(&self) -> HALResult<JoystickAxes> {
        hal_retcall!(HAL_GetJoystickAxes(self.0) -> JoystickAxes)
    }

    pub fn povs(&self) -> HALResult<JoystickPOVs> {
        hal_retcall!(HAL_GetJoystickPOVs(self.0) -> JoystickPOVs)
    }

    pub fn buttons(&self) -> HALResult<JoystickButtons> {
        hal_retcall!(HAL_GetJoystickButtons(self.0) -> JoystickButtons)
    }

    pub fn touchpads(&self) -> HALResult<JoystickTouchpads> {
        hal_retcall!(HAL_GetJoystickTouchpads(self.0) -> JoystickTouchpads)
    }

    pub fn all_data(&self) -> AllJoystickData {
        let mut data = AllJoystickData::default();
        unsafe {
            HAL_GetAllJoystickData(
                self.0,
                &mut data.axes,
                &mut data.povs,
                &mut data.buttons,
                &mut data.touchpads,
            );
        }
        data
    }

    pub fn descriptor(&self) -> HALResult<JoystickDescriptor> {
        hal_retcall!(HAL_GetJoystickDescriptor(self.0) -> JoystickDescriptor)
    }

    pub fn is_gamepad(&self) -> bool {
        unsafe { HAL_GetJoystickIsGamepad(self.0) != 0 }
    }

    pub fn gamepad_type(&self) -> i32 {
        unsafe { HAL_GetJoystickGamepadType(self.0) }
    }

    /// Get supported outputs.
    /// This appears to be internally sourced from `GenericHID`'s `SupportedOutputs` bitflag.
    pub fn supported_outputs(&self) -> i32 {
        unsafe { HAL_GetJoystickSupportedOutputs(self.0) }
    }

    /// Gets the joystick name.
    pub fn name(&self) -> WPIString {
        unsafe { WPIString::from_raw_ctx(|name| HAL_GetJoystickName(name, self.0)) }
    }

    pub fn set_rumble(&self, rumbles: &JoystickRumble) -> HALResult<()> {
        hal_retcall!(HAL_SetJoystickRumble(
            self.0,
            rumbles.left_rumble as i32,
            rumbles.right_rumble as i32,
            rumbles.left_trigger_rumble as i32,
            rumbles.right_trigger_rumble as i32
        ))
    }

    pub fn set_leds(&self, rgb: u32) -> HALResult<()> {
        hal_retcall!(HAL_SetJoystickLeds(self.0, rgb as _))
    }
}

const GAME_DATA_LEN: usize = 9;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct GameData([u8; GAME_DATA_LEN]);
impl GameData {
    /// # Safety
    /// You gotta be sure that `data.gameData` is utf8.
    pub unsafe fn new(data: HAL_GameData) -> Self {
        Self(data.gameData.map(|v| v as _))
    }

    pub fn from_str(s: &str) -> Self {
        let data = &s.as_bytes()[..s.floor_char_boundary(s.len().min(GAME_DATA_LEN))];
        let mut dest = [0_u8; _];
        dest[..data.len()].copy_from_slice(data);

        Self(dest)
    }

    /// inner
    pub const fn as_bytes(&self) -> [u8; GAME_DATA_LEN] {
        self.0
    }

    /// print as string
    pub const fn as_str<'a>(&'a self) -> &'a str {
        let mut len = 0;
        // find null terminator
        while len < self.0.len() && self.0[len] != 0 {
            len += 1;
        }

        // SAFETY: i trust wpilib to give me utf8.
        unsafe { core::str::from_utf8_unchecked(self.0.split_at_unchecked(len).0) }
    }

    /// Get new game data.
    pub fn get() -> HALResult<Self> {
        // SAFETY: i trust wpilib to not give me anything not utf8
        unsafe { hal_retcall!(HAL_GetGameData() -> HAL_GameData).map(|v| Self::new(v)) }
    }
}

impl From<GameData> for HAL_GameData {
    fn from(value: GameData) -> Self {
        Self {
            gameData: value.0.map(|v| v as _),
        }
    }
}

/// Return the approximate match time.
///
/// The FMS does not send an official match
/// time to the robots, but does send an approximate match time. The value will
/// count down the time remaining in the current period (auto or teleop).
/// Warning: This is not an official time (so it cannot be used to dispute ref
/// calls or guarantee that a function will trigger before the match ends).
///
/// When connected to the real field, this number only changes in full integer
/// increments, and always counts down.
///
/// When the DS is in practice mode, this number is a floating point number,
/// and counts down.
///
/// When the DS is in teleop or autonomous mode, this number is a floating
/// point number, and counts up.
///
/// Simulation matches DS behavior without an FMS connected.
///
/// @param[out] status the error code, or 0 for success
/// @return Time remaining in current match period (auto or teleop) in seconds
pub fn get_match_time() -> HALResult<f64> {
    hal_call!(HAL_GetMatchTime())
}

pub fn get_outputs_enabled() -> bool {
    unsafe { HAL_GetOutputsEnabled() != 0 }
}

pub fn get_match_info() -> HALResult<MatchInfo> {
    let mut match_info = MatchInfo::default();
    unsafe {
        match HAL_GetMatchInfo(&mut match_info) {
            0 => Ok(match_info),
            err => Err(HALError(err)),
        }
    }
}

pub fn refresh_ds_data() -> bool {
    unsafe { HAL_RefreshDSData() != 0 }
}

// do not wrap ProvideNewData/RemoveNewData

/// Sets the program starting flag in the DS.
///
/// This is what changes the DS to showing robot code ready.
pub fn observe_user_program_starting() {
    unsafe {
        HAL_ObserveUserProgramStarting();
    }
}
