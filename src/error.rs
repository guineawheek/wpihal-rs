// Parts borrowed from https://github.com/first-rust-competition/first-rust-competition/blob/master/wpilib-sys/src/hal_call.rs

use core::fmt;
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
};

use wpihal_sys::{HAL_GetErrorMessage, HAL_SendConsoleLine, HAL_SendError, HAL_SetPrintErrorImpl};

use crate::hal_retcall;

/// Sends a warning to the driver station.
pub fn send_warning(code: i32, details: &CStr) -> HALResult<()> {
    hal_retcall!(HAL_SendError(
        0,
        code,
        0,
        details.as_ptr(),
        c"".as_ptr(),
        c"".as_ptr(),
        1
    ))
}

/// Sends an error to the driver station.
pub fn send_error(code: i32, details: &CStr) -> HALResult<()> {
    hal_retcall!(HAL_SendError(
        1,
        code,
        0,
        details.as_ptr(),
        c"".as_ptr(),
        c"".as_ptr(),
        1
    ))
}

/// Sets the print error implementation to a given function pointer.
///
/// This adds in a trampoline function that converts the arguments into a `str` but if you dislike the extra atomic this adds just use the raw `-sys` export.
///
/// # Safety
/// Be careful.
pub unsafe fn set_print_error_impl(print_fn: Option<fn(&str)>) {
    // yes this is dangling but every use of this atomic since will set this to a real value.
    static PRINT_FN: core::sync::atomic::AtomicPtr<usize> =
        core::sync::atomic::AtomicPtr::new(core::ptr::null_mut());
    unsafe extern "C" fn _wpihal_rs_print_error_trampoline(
        ptr: *const core::ffi::c_char,
        len: usize,
    ) {
        unsafe {
            let fn_ptr: fn(&str) =
                core::mem::transmute(PRINT_FN.load(core::sync::atomic::Ordering::Relaxed));
            fn_ptr(str::from_utf8_unchecked(core::slice::from_raw_parts(
                ptr as _, len,
            )));
        };
    }

    unsafe {
        HAL_SetPrintErrorImpl(print_fn.map(|ptr| {
            PRINT_FN.store(ptr as *mut usize, core::sync::atomic::Ordering::Relaxed);
            _wpihal_rs_print_error_trampoline as _
        }));
    }
}

/// Send a line to the driver station console.
pub fn send_console_line(line: &str) -> HALResult<()> {
    let c_line = CString::new(line).unwrap();
    hal_retcall!(HAL_SendConsoleLine(c_line.as_ptr()))
}

/// Converts an Option<&CStr> into an allocation location pointer.
/// These are nullable.
///
/// These are used throughout the HAL to provide helpful messages on double allocation.
pub fn allocation_location_ptr(allocation_location: Option<&CStr>) -> *const core::ffi::c_char {
    match allocation_location {
        Some(s) => s.as_ptr(),
        None => core::ptr::null(),
    }
}

/// represents a hal error returned from wpilib
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct HALError(pub i32);

impl HALError {
    /// Get the HAL error message associated with this error code.
    /// In traditional WPILib, this would be printed to the driver
    /// station whenever an error occured. The resulting string may
    /// not be valid UTF-8.
    pub fn message(&'_ self) -> Cow<'_, str> {
        let const_char_ptr = unsafe { HAL_GetErrorMessage(self.0) };
        let c_str = unsafe { CStr::from_ptr(const_char_ptr) };
        c_str.to_string_lossy()
    }

    /// Sends this error to the driver station.
    /// The location and callStack fields are set to be blank.
    pub fn send_error(&self) {
        unsafe {
            let details = HAL_GetErrorMessage(self.0);
            send_error(self.0, CStr::from_ptr(details)).ok();
        }
    }
}

impl fmt::Debug for HALError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "HalError {{ {} }}", self.message())
    }
}

impl fmt::Display for HALError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: \"{}\"!", self.message())
    }
}

impl std::error::Error for HALError {
    fn description(&self) -> &str {
        "Error in the HAL"
    }
}

impl From<i32> for HALError {
    fn from(code: i32) -> Self {
        HALError(code)
    }
}

/// create status code
#[must_use]
pub const fn result_as_i32(result: HALResult<()>) -> i32 {
    match result {
        Ok(()) => 0,
        Err(e) => e.0,
    }
}

impl embedded_hal::digital::Error for HALError {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

impl embedded_hal::pwm::Error for HALError {
    fn kind(&self) -> embedded_hal::pwm::ErrorKind {
        embedded_hal::pwm::ErrorKind::Other
    }
}

pub type HALResult<T> = Result<T, HALError>;
