use wpihal_sys::{HAL_OpModeOption, HAL_RobotMode};

pub struct OpModeOption {
    /// Mode in which the opmode will run
    pub mode: HAL_RobotMode,
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
