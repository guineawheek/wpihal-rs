use std::{ffi::{c_void, CStr, CString}, path::Path};

use wpihal_sys::{HAL_LoadExtensions, HAL_LoadOneExtension, HAL_OnShutdown, HAL_RegisterExtension, HAL_RegisterExtensionListener, HAL_SetShowExtensionsNotFoundMessages};

use crate::HAL_rust_wpihal_linkage_trampoline;


pub fn load_one_extension(library: &Path) -> i32 {
    let s = CString::new(library.as_os_str().to_string_lossy().as_bytes()).unwrap();
    unsafe { HAL_LoadOneExtension(s.as_ptr()) }
}

pub fn load_extensions() -> i32 {
    unsafe { HAL_LoadExtensions() }
}

pub fn register_extension(name: &CStr, data: *mut c_void) {
    unsafe { HAL_RegisterExtension(name.as_ptr(), data);}
}
// register_extension_listener omitted

/// this makes the simplifying assumption that the meta-parameter passed is not used.
/// if you really care call HAL_RegisterExtension directly.
pub fn register_extension_listener(f: fn(&CStr, *mut c_void)) {
    unsafe {
        HAL_RegisterExtensionListener(f as *mut c_void, Some(HAL_rust_wpihal_extension_callback_wrapper));
    }
}


#[allow(non_snake_case)]
unsafe extern "C" fn HAL_rust_wpihal_extension_callback_wrapper(f: *mut c_void, name: *const i8, data: *mut c_void) {
    unsafe {
        let f: fn(&CStr, *mut c_void) = core::mem::transmute(f);
        f(CStr::from_ptr(name), data);
    }
}

pub fn set_show_extensions_not_found_messages(show_message: bool) {
    unsafe { HAL_SetShowExtensionsNotFoundMessages(show_message as i32); }
}

pub fn on_shutdown(f: fn()) {
    unsafe {
        HAL_OnShutdown(f as *mut c_void, Some(HAL_rust_wpihal_linkage_trampoline));
    }
}

// HAL_OnShutdown omitted