use std::ops::Deref;

use wpihal_sys::HAL_OpModeOption;
use wpiutil::{RawWPIString, WPIString, WPIStringRef};

/// will finish later....
#[repr(transparent)]
pub struct OpModeOption(pub(crate) HAL_OpModeOption);
/*
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
pub background_color: Option<u32>,*/

impl OpModeOption {
    pub unsafe fn new_ref<'a>(
        id: i64,
        name: &'a str,
        group: &'a str,
        description: &'a str,
        text_color: Option<u32>,
        background_color: Option<u32>,
    ) -> HAL_OpModeOption {
        unsafe {
            HAL_OpModeOption {
                id,
                name: WPIStringRef::from(name).as_handle(),
                group: WPIStringRef::from(group).as_handle(),
                description: WPIStringRef::from(description).as_handle(),
                textColor: text_color.unwrap_or(0) as _,
                backgroundColor: background_color.unwrap_or(0) as _,
            }
        }
    }

    /// create new/owned
    pub fn new(
        id: i64,
        name: &str,
        group: &str,
        description: &str,
        text_color: Option<u32>,
        background_color: Option<u32>,
    ) -> Self {
        Self(HAL_OpModeOption {
            id,
            name: WPIString::new(name).into(),
            group: WPIString::new(group).into(),
            description: WPIString::new(description).into(),
            textColor: text_color.unwrap_or(0) as _,
            backgroundColor: background_color.unwrap_or(0) as _,
        })
    }

    /// # Safety
    /// This struct must stay alive long enough
    pub const unsafe fn as_handle(&self) -> HAL_OpModeOption {
        unsafe { (&raw const self.0).read() }
    }

    pub const fn id(&self) -> i64 {
        self.0.id
    }

    pub const fn name(&self) -> &str {
        unsafe { self.0.name.as_str() }
    }

    pub const fn group(&self) -> &str {
        unsafe { self.0.group.as_str() }
    }

    pub const fn description(&self) -> &str {
        unsafe { self.0.description.as_str() }
    }

    pub const fn text_color(&self) -> Option<u32> {
        match self.0.textColor {
            0 => None,
            other => Some(other as _),
        }
    }

    pub const fn background_color(&self) -> Option<u32> {
        match self.0.backgroundColor {
            0 => None,
            other => Some(other as _),
        }
    }
}

impl Clone for OpModeOption {
    fn clone(&self) -> Self {
        Self::new(
            self.id(),
            self.name(),
            self.group(),
            self.description(),
            self.text_color(),
            self.background_color(),
        )
    }
}

impl Drop for OpModeOption {
    fn drop(&mut self) {
        unsafe {
            drop(WPIString::from_raw(core::mem::replace(
                &mut self.0.name,
                RawWPIString::default(),
            )));
            drop(WPIString::from_raw(core::mem::replace(
                &mut self.0.group,
                RawWPIString::default(),
            )));
            drop(WPIString::from_raw(core::mem::replace(
                &mut self.0.description,
                RawWPIString::default(),
            )));
        }
    }
}

impl core::fmt::Debug for OpModeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpModeOption").finish()
    }
}

#[derive(Debug)]
pub struct OpModeOptionsArray(pub(crate) *mut OpModeOption, pub(crate) usize);
impl Deref for OpModeOptionsArray {
    type Target = [OpModeOption];

    fn deref(&self) -> &Self::Target {
        if self.0.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.0, self.1) }
        }
    }
}
impl Drop for OpModeOptionsArray {
    fn drop(&mut self) {
        unsafe {
            wpihal_sys::HALSIM_FreeOpModeOptionsArray(self.0.cast(), self.1);
        }
    }
}

/// Initializes the dashboard opmode controller.
pub fn initialize_dashboard_op_mode() {
    unsafe {
        wpihal_sys::_HALShim_InitializeDashboardOpMode();
    }
}

pub fn set_dashboard_op_mode_options(options: &[OpModeOption]) {
    unsafe {
        wpihal_sys::_HALShim_SetDashboardOpModeOptions(options.as_ptr().cast(), options.len());
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
