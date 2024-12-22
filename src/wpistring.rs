use core::str;
use std::fmt::Display;

use wpihal_sys::{WPI_String, WPI_FreeString};

/// A WPI_String that needs to be freed internally with [`WPI_FreeString`].
/// This implements [`Drop`] so this is automatically handled for you.
/// 
/// This is read-only because for wpihal, strings are never constructed by the API consumer
/// but rather handed to them and told to figure it out.
#[derive(Debug)]
pub struct AllocatedWPIString(WPI_String);

impl AllocatedWPIString {
    pub fn new(wpi_str: WPI_String) -> Self {
        Self(wpi_str)
    }

    /// View of the underlying utf8 string as a str.
    /// This assumes that the underlying const char* is, in fact, a utf8 string.
    /// 
    /// Which it should be. If it isn't, that's a WPILib bug.
    pub fn as_str<'a>(&'a self) -> &'a str {
        unsafe {
            str::from_utf8_unchecked(
                core::slice::from_raw_parts(self.0.str_ as *const u8, self.0.len)
            )
        }
    }
}

impl Drop for AllocatedWPIString {
    fn drop(&mut self) {
        unsafe { WPI_FreeString(&self.0 as *const WPI_String); }
    }
}

impl Display for AllocatedWPIString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_str())
    }
}