// Parts borrowed from https://github.com/first-rust-competition/first-rust-competition/blob/master/wpilib-sys/src/hal_call.rs

use core::fmt;
use std::{borrow::Cow, ffi::{CStr, CString}};

use wpihal_sys::{HAL_GetErrorMessage, HAL_SendConsoleLine, HAL_SendError};

/// Sends a warning to the driver station.
pub fn send_warning(code: i32, details: &CStr) -> HALResult<()> {
    let v = unsafe {
        HAL_SendError(
            0,
            code,
            0,
            details.as_ptr(),
            c"".as_ptr(),
            c"".as_ptr(),
            1
        )
    };
    if v != 0 { Err(HALError(v)) } else { Ok(()) }
}

/// Sends an error to the driver station.
pub fn send_error(code: i32, details: &CStr) -> HALResult<()> {
    let v = unsafe {
        HAL_SendError(
            1,
            code,
            0,
            details.as_ptr(),
            c"".as_ptr(),
            c"".as_ptr(),
            1
        )
    };
    if v != 0 { Err(HALError(v)) } else { Ok(()) }
}
// We don't bother with HAL_SetPrintErrorImpl because frankly it's kinda nuts.

pub fn send_console_line(line: &str) -> HALResult<()> {
    let v = unsafe { HAL_SendConsoleLine(CString::new(line).unwrap().as_ptr()) };
    if v != 0 { Err(HALError(v)) } else { Ok(()) }
}

/// Converts an Option<&CStr> into an allocation location pointer.
/// These are nullable.
/// 
/// These are used throughout the HAL to provide helpful messages on double allocation.
pub fn allocation_location_ptr(allocation_location: Option<&CStr>) -> *const i8 {
    match allocation_location {
        Some(s) => s.as_ptr(),
        None => core::ptr::null()
    }
}

/// represents a hal error returned from wpilib
#[derive(Copy, Clone)]
pub struct HALError(pub i32);

impl HALError {
    /// Get the HAL error message associated with this error code.
    /// In traditional WPILib, this would be printed to the driver
    /// station whenever an error occured. The resulting string may
    /// not be valid UTF-8.
    pub fn message(&self) -> Cow<str> {
        let const_char_ptr = unsafe { HAL_GetErrorMessage(self.0) };
        let c_str = unsafe { CStr::from_ptr(const_char_ptr) };
        c_str.to_string_lossy()
    }


    /// Sends this error to the driver station.
    /// The location and callStack fields are set to be blank.
    pub fn send_error(&self) {
        unsafe {
            let details = HAL_GetErrorMessage(self.0);
            send_error(self.0, CStr::from_ptr(details));
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

pub type HALResult<T> = Result<T, HALError>;