use wpihal_sys::{HAL_AllianceStationID, HAL_ControlWord, HAL_GetAllianceStation, HAL_GetControlWord, HAL_GetJoystickAxes, HAL_GetJoystickAxisType, HAL_GetJoystickDescriptor, HAL_GetJoystickIsXbox, HAL_GetJoystickName, HAL_GetJoystickPOVs, HAL_GetJoystickType, HAL_GetMatchInfo, HAL_GetMatchTime, HAL_GetOutputsEnabled, HAL_JoystickAxes, HAL_JoystickButtons, HAL_JoystickDescriptor, HAL_JoystickPOVs, HAL_MatchInfo, HAL_MatchType, HAL_ObserveUserProgramAutonomous, HAL_ObserveUserProgramDisabled, HAL_ObserveUserProgramStarting, HAL_ObserveUserProgramTeleop, HAL_ObserveUserProgramTest, HAL_RefreshDSData, HAL_SetJoystickOutputs, WPI_String};

use crate::{error::{HALError, HALResult}, hal_call, wpistring::WPIString};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ControlWord(pub u32);
impl ControlWord {
    pub fn enabled(&self) -> bool {
        self.0 & 0b1 != 0
    }

    pub fn autonomous(&self) -> bool {
        self.0 & 0b10 != 0
    }

    pub fn test(&self) -> bool {
        self.0 & 0b100 != 0
    }

    pub fn estop(&self) -> bool {
        self.0 & 0b1000 != 0
    }

    pub fn fms_attached(&self) -> bool {
        self.0 & 0b10000 != 0
    }

    pub fn ds_attached(&self) -> bool {
        self.0 & 0b100000 != 0
    }

    pub fn reserved(&self) -> u32 {
        self.0 >> 6
    }
}

pub fn get_control_word() -> HALResult<ControlWord> {
    unsafe {
        let mut word: HAL_ControlWord = core::mem::transmute(0u32);
        match HAL_GetControlWord(&mut word) {
            0 => Ok(core::mem::transmute(word)),
            err => Err(HALError(err))
        }
    }
}

pub fn get_alliance_station() -> HALResult<AllianceStationID> {
    hal_call!(HAL_GetAllianceStation())
}

pub fn get_joystick_axes(joystick_num: i32) -> HALResult<JoystickAxes> {
    unsafe {
        let mut axes: HAL_JoystickAxes = core::mem::zeroed();
        match HAL_GetJoystickAxes(joystick_num, &mut axes) {
            0 => Ok(axes),
            err => Err(HALError(err))
        }
    }
}

pub fn get_joystick_povs(joystick_num: i32) -> HALResult<JoystickPOVs> {
    unsafe {
        let mut povs: HAL_JoystickPOVs = core::mem::zeroed();
        match HAL_GetJoystickPOVs(joystick_num, &mut povs) {
            0 => Ok(povs),
            err => Err(HALError(err))
        }
    }
}

pub fn get_joystick_buttons(joystick_num: i32) -> HALResult<JoystickPOVs> {
    unsafe {
        let mut povs: HAL_JoystickPOVs = core::mem::zeroed();
        match HAL_GetJoystickPOVs(joystick_num, &mut povs) {
            0 => Ok(povs),
            err => Err(HALError(err))
        }
    }
}

pub fn get_joystick_descriptor(joystick_num: i32) -> HALResult<JoystickDescriptor> {
    unsafe {
        let mut desc: HAL_JoystickDescriptor = core::mem::zeroed();
        match HAL_GetJoystickDescriptor(joystick_num, &mut desc) {
            0 => Ok(desc),
            err => Err(HALError(err))
        }
    }
}

pub fn get_joystick_is_xbox(joystick_num: i32) -> bool {
    unsafe { HAL_GetJoystickIsXbox(joystick_num) != 0 }
}


pub fn get_joystick_type(joystick_num: i32) -> i32 {
    unsafe { HAL_GetJoystickType(joystick_num) }
}

pub fn get_joystick_name(joystick_num: i32) -> WPIString {
    let mut name = WPI_String::default(); 
    unsafe { HAL_GetJoystickName(&mut name, joystick_num); }
    WPIString::from_raw(name)
}

pub fn get_joystick_axis_type(joystick_num: i32, axis: i32) -> i32 {
    unsafe { HAL_GetJoystickAxisType(joystick_num, axis) }
}

pub fn set_joystick_outputs(joystick_num: i32, outputs: u64, left_rumble: u16, right_rumble: u16) -> HALResult<()> {
    unsafe {
        match HAL_SetJoystickOutputs(joystick_num, outputs as i64, left_rumble as i32, right_rumble as i32) {
            0 => Ok(()),
            err => Err(HALError(err))
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
            err => Err(HALError(err))
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
    unsafe { HAL_ObserveUserProgramStarting(); }
}

/// 
/// Sets the disabled flag in the DS.
/// 
/// This is used for the DS to ensure the robot is properly responding to its
/// state request. Ensure this gets called about every 50ms, or the robot will be
/// disabled by the DS.
pub fn observe_user_program_disabled() {
    unsafe { HAL_ObserveUserProgramDisabled(); }
}

pub fn observe_user_program_autonomous() {
    unsafe { HAL_ObserveUserProgramAutonomous(); }
}

pub fn observe_user_program_teleop() {
    unsafe { HAL_ObserveUserProgramTeleop(); }
}

pub fn observe_user_program_test() {
    unsafe { HAL_ObserveUserProgramTest(); }
}


pub type AllianceStationID = HAL_AllianceStationID;
pub type MatchType = HAL_MatchType;
pub type JoystickAxes = HAL_JoystickAxes;
pub type JoystickPOVs = HAL_JoystickPOVs;
pub type JoystickButtons = HAL_JoystickButtons;
pub type JoystickDescriptor = HAL_JoystickDescriptor;
pub type MatchInfo = HAL_MatchInfo;