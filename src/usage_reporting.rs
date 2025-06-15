use std::ffi::CStr;

use wpihal_sys::HAL_Report;
pub use wpihal_sys::usage_reporting::*;

pub fn report(resource: ResourceType, instance_number: i32) -> i64 {
    unsafe { HAL_Report(resource as i32, instance_number, 0, core::ptr::null()) }
}

pub fn report_with_context(
    resource: ResourceType,
    instance_number: i32,
    context: i32,
    feature: Option<&CStr>,
) -> i64 {
    let ptr = match feature {
        Some(f) => f.as_ptr(),
        None => core::ptr::null(),
    };
    unsafe { HAL_Report(resource as i32, instance_number, context, ptr) }
}
