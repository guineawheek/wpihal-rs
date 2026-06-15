//! Rust adapter for [`RawWPIString`]
//!
//! Per Thad in https://github.com/wpilibsuite/allwpilib/pull/6299 the semantics are as follows:
//!
//! * WPILib will not have any APIs that manipulate a string allocated externally.
//!   This means [`RawWPIString`]` can be const, as across the boundary it is always const.
//! * If a WPILib API takes a `const RawWPIString*`, WPILib will not manipulate or attempt to free that string, and that string is treated as an input.
//!   It is up to the caller to handle that memory, WPILib will never hold onto that memory longer than the call.
//! * If a WPILib API takes a `RawWPIString*`, that string is an output.
//! * WPILib will allocate that API with [`WPI_AllocateString`], fill in the string, and return to the caller.
//!   When the caller is done with the string, they must free it with [`WPI_FreeString`].
//! * If an output struct contains a [`RawWPIString`] member, that member is considered read only, and should not be explicitly freed.
//! * The caller should call the free function for that struct.
//! * If an array of [`RawWPIString`]s are returned, each individual string is considered read only, and should not be explicitly freed.
//!   The free function for that array should be called by the caller.
//! * If an input struct containing a [`RawWPIString`], or an input array of [`RawWPIString`]s is passed to WPILib, the individual strings
//!   will not be manipulated or freed by WPILib, and the caller owns and should free that memory.
//! * Callbacks also follow these rules.
//!   The most common is a callback either getting passed a `const RawWPIString*` or a struct containing a RawWPIString.
//!   In both of these cases, the callback target should consider these strings read only, and not attempt to free them or manipulate them.

use core::str;
use std::{ffi::CStr, fmt::Display, marker::PhantomData, ops::Deref};

pub use wpiutil_sys::WPI_String as RawWPIString;
use wpiutil_sys::{WPI_AllocateString, WPI_FreeString};

/// A RawWPIString that needs to be freed internally with [`WPI_FreeString`].
/// This implements [`Drop`] so this is automatically handled for you.
///
/// This is primarily meant for WPILib APIs that directly allocate and return `RawWPIString`s
/// and need the result freed after use.
#[derive(Debug)]
#[repr(transparent)]
pub struct WPIString(RawWPIString);

impl WPIString {
    /// Allocates a new [`WPIString`] as a copy of an existing string.
    pub fn new(s: &str) -> Self {
        let mut wpi_str = RawWPIString::default();
        unsafe {
            WPI_AllocateString(&mut wpi_str, s.as_bytes().len());
        }
        Self(wpi_str)
    }

    /// Create from an existing [`RawWPIString`] existing struct.
    /// This is intended for WPILib function that write to a `*mut RawWPIString`.
    ///
    /// # Safety
    /// You must be sure that the passed [`RawWPIString`] points to free-able data.
    #[must_use]
    pub const unsafe fn from_raw(wpi_str: RawWPIString) -> Self {
        Self(wpi_str)
    }

    /// View of the underlying utf8 string as a [`str`].
    pub fn as_str<'a>(&'a self) -> &'a str {
        // SAFETY: We generally assume the underlying buffer is UTF-8.
        // If it's not, then that's probably a bug.
        //
        // No Thad, UTF-16LE is in fact a mental illness.
        unsafe {
            str::from_utf8_unchecked(core::slice::from_raw_parts(
                self.0.str_ as *const u8,
                self.0.len,
            ))
        }
    }
}

impl AsRef<RawWPIString> for WPIString {
    fn as_ref(&self) -> &RawWPIString {
        &self.0
    }
}

impl Deref for WPIString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Drop for WPIString {
    fn drop(&mut self) {
        unsafe {
            WPI_FreeString(&self.0);
        }
    }
}

impl Display for WPIString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_str())
    }
}

/// A "read-only" [`RawWPIString`].
///
/// These can be constructed with the [`From`] impls from [`str`] and [`CStr`], but are otherwise
/// not directly constructable as the associated [`RawWPIString`] is meant more as a reference to data.
pub struct WPIStringRef<'a> {
    inner: RawWPIString,
    _borrow: PhantomData<&'a str>,
}

impl WPIStringRef<'_> {
    /// New-constructor.
    /// Supplied for certain WPILib APIs that may return immutable references to strings.
    ///
    /// # Safety
    /// You are responsible for ensuring the data the associated [`RawWPIString`] points to
    /// in fact has the lifetime that [`WPIStringRef`] claims.
    pub const unsafe fn new(wpi_str: RawWPIString) -> Self {
        Self {
            inner: wpi_str,
            _borrow: PhantomData,
        }
    }
}

impl<'a> From<&'a CStr> for WPIStringRef<'a> {
    fn from(value: &'a CStr) -> Self {
        // SAFETY: we can bind directly to the cstr's lifetime
        unsafe {
            Self::new(RawWPIString {
                str_: value.as_ptr() as *const core::ffi::c_char,
                len: value.count_bytes(),
            })
        }
    }
}

impl<'a> From<&'a str> for WPIStringRef<'a> {
    fn from(value: &'a str) -> Self {
        // SAFETY: we can bind directly to the str's lifetime
        unsafe {
            Self::new(RawWPIString {
                str_: value.as_ptr() as *const core::ffi::c_char,
                len: value.as_bytes().len(),
            })
        }
    }
}

impl Deref for WPIStringRef<'_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        unsafe {
            str::from_utf8_unchecked(core::slice::from_raw_parts(
                self.inner.str_ as *const u8,
                self.inner.len,
            ))
        }
    }
}

impl AsRef<RawWPIString> for WPIStringRef<'_> {
    fn as_ref(&self) -> &RawWPIString {
        &self.inner
    }
}

impl Display for WPIStringRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}

/// Turns a `&'a str` or `&'a CStr` and yields an `&'a RawWPIString` which will coerce into a `*const RawWPIString`.
///
/// Cuts out a bit of boilerplate for when you need to insert strings into WPILib APIs and is zero-cost.
#[macro_export]
macro_rules! as_wpistr {
    ($s:expr) => {
        wpiutil::wpistring::WPIStringRef::from($s).as_ref()
    };
}
pub use as_wpistr;
