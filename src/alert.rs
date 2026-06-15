use wpihal_sys::{
    HAL_AlertHandle, HAL_CreateAlert, HAL_DestroyAlert, HAL_GetAlertText, HAL_IsAlertActive,
    HAL_SetAlertActive, HAL_SetAlertText,
};

use wpiutil::{
    as_wpistr,
    wpistring::{RawWPIString, WPIString, WPIStringRef},
};

use crate::{error::HALResult, hal_call};

pub use wpihal_sys::HAL_AlertLevel as AlertLevel;

#[derive(Debug)]
pub struct Alert(HAL_AlertHandle);

impl Alert {
    /// Creates an alert.
    pub fn new(group: &str, text: &str, level: AlertLevel) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_CreateAlert(
            as_wpistr!(group),
            as_wpistr!(text),
            level as _
        ))?))
    }

    pub fn set_active(&mut self, active: bool) -> HALResult<()> {
        hal_call!(HAL_SetAlertActive(self.0, active as _))
    }

    pub fn is_active(&self) -> HALResult<bool> {
        hal_call!(HAL_IsAlertActive(self.0)).map(|v| v != 0)
    }

    pub fn set_text(&mut self, new_text: &str) -> HALResult<()> {
        hal_call!(HAL_SetAlertText(
            self.0,
            WPIStringRef::from(new_text).as_ref()
        ))
    }

    pub fn get_text(&self) -> HALResult<WPIString> {
        let mut out = RawWPIString::default();
        // SAFETY: wpihal allocates the string so we are responsible for its deallocation
        hal_call!(HAL_GetAlertText(self.0, &mut out)).map(|()| unsafe { WPIString::from_raw(out) })
    }
}

impl Drop for Alert {
    fn drop(&mut self) {
        // SAFETY: we exclusively own our handle.
        unsafe {
            HAL_DestroyAlert(self.0);
        }
    }
}
