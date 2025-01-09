use core::str;
use std::{ffi::CStr, fmt::Display, mem::ManuallyDrop, ops::Deref};

use wpihal_sys::{WPI_AllocateString, WPI_FreeString, WPI_String};

/// A WPI_String that needs to be freed internally with [`WPI_FreeString`].
/// This implements [`Drop`] so this is automatically handled for you.
/// 
/// Per Thad in https://github.com/wpilibsuite/allwpilib/pull/6299 the semantics are as follows:
/// 
/// * WPILib will not have any APIs that manipulate a string allocated externally.
///   This means WPI_String can be const, as across the boundary it is always const.
/// * If a WPILib API takes a `const WPI_String*`, WPILib will not manipulate or attempt to free that string, and that string is treated as an input.
///   It is up to the caller to handle that memory, WPILib will never hold onto that memory longer than the call.
/// * If a WPILib API takes a `WPI_String*`, that string is an output.
/// * WPILib will allocate that API with [`WPI_AllocateString()`], fill in the string, and return to the caller.
///   When the caller is done with the string, they must free it with WPI_FreeString().
/// * If an output struct contains a [`WPI_String`] member, that member is considered read only, and should not be explicitly freed.
/// * The caller should call the free function for that struct.
/// * If an array of [`WPI_String`]s are returned, each individual string is considered read only, and should not be explicitly freed.
///   The free function for that array should be called by the caller.
/// * If an input struct containing a [`WPI_String`], or an input array of [`WPI_String`]s is passed to WPILib, the individual strings will not be manipulated or freed by WPILib, and the caller owns and should free that memory.
///   Callbacks also follow these rules.
///   The most common is a callback either getting passed a const WPI_String* or a struct containing a WPI_String.
///   In both of these cases, the callback target should consider these strings read only, and not attempt to free them or manipulate them.
#[derive(Debug)]
pub struct WPIString(WPI_String);

impl WPIString {
    /// Creates a new WPIString from a [`CStr`] without allocation.
    /// The WPIStringRef must not live longer than the CStr.
    pub fn from_cstr<'a>(s: &'a CStr) -> WPIStringRef<'a> {
        ManuallyDrop::new(WPIString(WPI_String { str_: s.as_ptr(), len: s.count_bytes() }))
    }

    /// Creates a new WPIString from a [`str`] without allocation.
    /// The WPIStringRef must not live longer than the cstr.
    pub fn from_str<'a>(s: &'a str) -> WPIStringRef<'a> {
        ManuallyDrop::new(WPIString(WPI_String { 
            str_: s.as_ptr() as *const std::os::raw::c_char,
            len: s.as_bytes().len()
        }))
    }

    /// Allocates a new WPIString as a copy.
    pub fn new(s: &str) -> Self {
        let mut wpi_str = WPI_String::default();
        unsafe {
            WPI_AllocateString(&mut wpi_str, s.as_bytes().len());
        }         
        Self(wpi_str)
    }


    pub fn from_raw(wpi_str: WPI_String) -> Self {
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

/// This is just ManuallyDrop<WPIString> and is passed in cases where the constructor should *not* drop the struct
pub type WPIStringRef<'a> = core::mem::ManuallyDrop<WPIString>;

impl Deref for WPIString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Drop for WPIString {
    fn drop(&mut self) {
        unsafe { WPI_FreeString(&self.0 as *const WPI_String); }
    }
}

impl Display for WPIString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_str())
    }
}