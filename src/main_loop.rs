use std::ffi::c_void;

use wpihal_sys::{HAL_ExitMain, HAL_HasMain, HAL_RunMain, HAL_SetMain};

pub fn set_main(
    param: *mut c_void,
    main_fn: unsafe extern "C" fn(*mut c_void),
    exit_fn: unsafe extern "C" fn(*mut c_void),
) {
    unsafe {
        HAL_SetMain(param, Some(main_fn), Some(exit_fn));
    }
}

pub fn has_main() -> bool {
    unsafe { HAL_HasMain() != 0 }
}

pub fn run_main() {
    unsafe {
        HAL_RunMain();
    }
}

pub fn exit_main() {
    unsafe {
        HAL_ExitMain();
    }
}
