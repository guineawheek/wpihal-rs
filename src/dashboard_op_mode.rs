pub type OpModeOption = wpihal_sys::HAL_OpModeOption;

pub fn initialize() {
    unsafe {
        wpihal_sys::_HALShim_InitializeDashboardOpMode();
    }
}

pub fn set_op_mode_options(options: &[OpModeOption]) {
    unsafe {
        wpihal_sys::_HALShim_SetDashboardOpModeOptions(options.as_ptr(), options.len());
    }
}
pub fn start_op_mode() {
    unsafe {
        wpihal_sys::_HALShim_StartDashboardOpMode();
    }
}

pub fn enable_op_mode() {
    unsafe {
        wpihal_sys::_HALShim_EnableDashboardOpMode();
    }
}

pub fn get_selected_opmode() {}
