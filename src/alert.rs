use wpihal_sys::{HAL_AlertHandle, HAL_AlertLevel, HAL_DestroyAlert};

use wpiutil::wpistring::WPIString;

use crate::{error::HALResult, hal_call};

pub type AlertLevel = HAL_AlertLevel;

#[derive(Debug)]
pub struct Alert(HAL_AlertHandle);

impl Alert {
    /// Creates an alert.
    pub fn new(group: &str, text: &str, level: AlertLevel) -> HALResult<Self> {
        WPIString::Ok(Self(hal_call!(HAL_CreateAlert(.., .., level as _))?))
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
