use wpihal_sys::HAL_OpModeOption;

use crate::control_word::RobotMode;

/// will finish later....
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OpModeOption {
    /// Mode in which the opmode will run
    pub mode: RobotMode,
    /// name
    pub name: String,
    /// group
    pub group: String,
    /// description
    pub description: String,

    /// hex code or None for default
    pub text_color: Option<u32>,
    /// hex code or None for default
    pub background_color: Option<u32>,
}

/// Initializes the dashboard opmode controller.
pub fn initialize_dashboard_op_mode() {
    unsafe {
        wpihal_sys::_HALShim_InitializeDashboardOpMode();
    }
}

pub fn set_dashboard_op_mode_options(options: &[HAL_OpModeOption]) {
    unsafe {
        wpihal_sys::_HALShim_SetDashboardOpModeOptions(options.as_ptr(), options.len());
    }
}
pub fn start_dashboard_op_mode() {
    unsafe {
        wpihal_sys::_HALShim_StartDashboardOpMode();
    }
}

pub fn enable_dashboard_op_mode() {
    unsafe {
        wpihal_sys::_HALShim_EnableDashboardOpMode();
    }
}

pub fn get_selected_opmode() {}
