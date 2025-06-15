use wpihal_sys::{HAL_ReportUsage, WPI_String};
use wpiutil::wpistring::WPIString;

/// Reports usage of a resource of interest.  Repeated calls for the same
/// resource name replace the previous report.
///
/// * resource       the used resource name; convention is to suffix with
///                  "[instanceNum]" for multiple instances of the same
///                  resource
/// * data           arbitrary associated data string
///
/// Returns a mystery handle. Yay!
pub fn report(resource: &str, data: &str) -> i32 {
    let resource_str = WPIString::from_str(resource);
    let data_str = WPIString::from_str(data);
    unsafe {
        let resource_ptr = core::mem::transmute::<_, *const WPI_String>(&resource_str as *const _);
        let data_ptr = core::mem::transmute::<_, *const WPI_String>(&data_str as *const _);
        HAL_ReportUsage(resource_ptr, data_ptr)
    }
}
