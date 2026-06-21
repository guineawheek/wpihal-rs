use std::{ffi::CStr, mem::MaybeUninit};

use wpihal_sys::{HAL_NotifierHandle, HALSIM_GetNotifierInfo, HALSIM_NotifierInfo};

pub fn get_next_notifier_timeout() -> u64 {
    unsafe { wpihal_sys::HALSIM_GetNextNotifierTimeout() }
}

pub fn get_num_notifiers() -> usize {
    unsafe { wpihal_sys::HALSIM_GetNumNotifiers().max(0) as usize }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct NotifierInfo(HALSIM_NotifierInfo);
impl NotifierInfo {
    pub unsafe fn get_into(dest: &mut [MaybeUninit<Self>]) -> usize {
        assert_eq!(size_of::<Self>(), size_of::<HALSIM_NotifierInfo>());
        // SAFETY: Due to extremely funny shenanigans, `HalSimAlertInfo` has the same size/layout as `HALSIM_AlertInfo`
        unsafe { HALSIM_GetNotifierInfo(dest.as_mut_ptr().cast(), dest.len() as i32) as usize }
    }

    pub fn get() -> Vec<Self> {
        // we make sure the max entries is slightly above the number of entries
        // in the case that the count is getting TOCTOU'ed
        let max_alerts = get_num_notifiers() + 4;
        let mut output = Vec::with_capacity(max_alerts);
        unsafe {
            let filled = Self::get_into(output.spare_capacity_mut());
            output.set_len(filled.min(max_alerts));
        }

        output
    }

    pub const fn raw_handle(&self) -> HAL_NotifierHandle {
        self.0.handle
    }

    pub const fn name(&self) -> &str {
        // SAFETY: the FFI code WILL always yield a nul byte; a
        // and it seemingly constrained to ascii
        unsafe {
            str::from_utf8_unchecked(CStr::from_ptr((&raw const self.0.name).cast()).to_bytes())
        }
    }

    pub const fn alarm_time(&self) -> u64 {
        self.0.alarmTime
    }

    pub const fn interval_time(&self) -> u64 {
        self.0.intervalTime
    }

    pub const fn overrun_count(&self) -> i32 {
        self.0.overrunCount
    }
}
