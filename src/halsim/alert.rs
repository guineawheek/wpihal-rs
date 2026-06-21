use core::mem::MaybeUninit;
use core::num::NonZeroI64;

use wpihal_sys::{HAL_AlertHandle, HALSIM_AlertInfo, HALSIM_GetAlerts, HALSIM_GetNumAlerts};
use wpiutil::wpistring::WPIString;

pub fn get_num_alerts() -> usize {
    unsafe { HALSIM_GetNumAlerts() as usize }
}

#[derive(Debug)]
#[repr(C)]
pub struct HalSimAlertInfo {
    handle: HAL_AlertHandle,
    group: WPIString,
    text: WPIString,
    active_start_time: Option<NonZeroI64>,
    level: crate::alert::AlertLevel,
}

impl HalSimAlertInfo {
    pub unsafe fn get_into(dest: &mut [MaybeUninit<Self>]) -> usize {
        assert_eq!(size_of::<HalSimAlertInfo>(), size_of::<HALSIM_AlertInfo>());
        // SAFETY: Due to extremely funny shenanigans, `HalSimAlertInfo` has the same size/layout as `HALSIM_AlertInfo`
        unsafe { HALSIM_GetAlerts(dest.as_mut_ptr().cast(), dest.len() as i32) as usize }
    }

    pub fn get() -> Vec<Self> {
        // we make sure the max alerts is slightly above the number of alerts
        // in the case that get_num_alerts is getting TOCTOU'ed
        let max_alerts = get_num_alerts() + 4;
        let mut output = Vec::with_capacity(max_alerts);
        unsafe {
            let filled = Self::get_into(output.spare_capacity_mut());
            output.set_len(filled.min(max_alerts));
        }

        output
    }

    pub const fn raw_handle(&self) -> HAL_AlertHandle {
        self.handle
    }

    pub fn group(&self) -> &str {
        &self.group
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn active_start_time(&self) -> Option<i64> {
        self.active_start_time.map(|v| v.get())
    }

    pub fn level(&self) -> crate::alert::AlertLevel {
        self.level
    }
}
