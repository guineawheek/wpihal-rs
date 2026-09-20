// Parts borrowed from https://github.com/first-rust-competition/first-rust-competition/blob/master/wpilib-sys/src/hal_call.rs

use core::fmt;
use std::{borrow::Cow, ffi::CStr};

use wpihal_sys::{
    HAL_GetErrorMessage, HAL_SendConsoleLine, HAL_SendError, HAL_SendProgramCrash,
    HAL_SetPrintErrorImpl,
};
use wpiutil::WPIStringRef;

use crate::hal_retcall;

/// Driver-station sent error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DsError<'a> {
    /// Is an error and not a warning
    pub is_error: bool,
    /// The error code
    pub code: i32,
    /// The details/error message
    pub details: &'a str,
    /// The location (if any)
    pub location: Option<&'a str>,
    /// The call stack (if any)
    pub call_stack: Option<&'a str>,
    /// Whether to print to stdout and the driver station
    pub print_stdout: bool,
}

impl<'a> DsError<'a> {
    /// Constructs a new [`DsError`] with the given error code and details.
    /// This defaults to being an error with no location, or call stack and with printing to stdout enabled.
    #[must_use]
    pub const fn new(code: i32, details: &'a str) -> Self {
        Self {
            is_error: true,
            code,
            details,
            location: None,
            call_stack: None,
            print_stdout: true,
        }
    }

    pub fn send(self) -> HALResult<()> {
        hal_retcall!(HAL_SendError(
            self.is_error as _,
            self.code,
            WPIStringRef::from(self.details).as_ref(),
            WPIStringRef::from(self.location.unwrap_or("")).as_ref(),
            WPIStringRef::from(self.call_stack.unwrap_or("")).as_ref(),
            1
        ))
    }

    pub const fn set_warning(mut self) -> Self {
        self.is_error = false;
        self
    }

    pub const fn set_error(mut self) -> Self {
        self.is_error = true;
        self
    }

    pub const fn set_location(mut self, location: &'a str) -> Self {
        self.location = Some(location);
        self
    }

    pub const fn set_call_stack(mut self, call_stack: &'a str) -> Self {
        self.call_stack = Some(call_stack);
        self
    }

    pub const fn set_print_stdout(mut self, print_stdout: bool) -> Self {
        self.print_stdout = print_stdout;
        self
    }
}

/// Sets the print error implementation to a given function pointer.
///
/// This adds in a trampoline function that converts the arguments into a `str` but if you dislike the extra atomic this adds just use the raw `-sys` export.
///
/// # Safety
/// Your `print_fn` pointer needs to be as valid as long as it is the print impl.
pub unsafe fn set_print_error_impl(print_fn: Option<fn(&str)>) {
    // yes this is dangling but every use of this atomic since will set this to a real value.
    static PRINT_FN: core::sync::atomic::AtomicPtr<usize> =
        core::sync::atomic::AtomicPtr::new(core::ptr::null_mut());
    unsafe extern "C" fn _wpihal_rs_print_error_trampoline(s: *const wpiutil::RawWPIString) {
        unsafe {
            let fn_ptr: fn(&str) =
                core::mem::transmute(PRINT_FN.load(core::sync::atomic::Ordering::Relaxed));

            fn_ptr(&WPIStringRef::new(s.read()));
        }
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
    hal_retcall!(HAL_SendConsoleLine(WPIStringRef::from(line).as_ref()))
}

/// Send a program crash to the driver station.
pub fn send_program_crash(details: &str, location: &str, call_stack: &str) -> HALResult<()> {
    hal_retcall!(HAL_SendProgramCrash(
        WPIStringRef::from(details).as_ref(),
        WPIStringRef::from(location).as_ref(),
        WPIStringRef::from(call_stack).as_ref(),
    ))
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
        let _ = DsError::new(self.0, &self.message()).send();
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
