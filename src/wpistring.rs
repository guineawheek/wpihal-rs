use core::str;

use wpihal_sys::{hal::WPI_String, hal::WPI_FreeString};

/// A WPI_String that needs to be freed internally with [`WPI_FreeString`].
/// This implements [`Drop`] so this is automatically handled for you
pub struct AllocatedWPIString(WPI_String);

impl AllocatedWPIString {
    pub fn new(wpi_str: WPI_String) -> Self {
        Self(wpi_str)
    }

    /// View of the underlying utf8 string as a str.
    /// This assumes that the underlying const char* is, in fact, a utf8 string.
    pub fn as_str<'a>(&'a self) -> &'a str {
        unsafe {
            // Ostensibly, this is a utf8 string. Ostensibly.
            // If this explodes, it's not my skill issue.
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