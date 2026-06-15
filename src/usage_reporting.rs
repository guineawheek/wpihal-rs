use wpihal_sys::HAL_ReportUsage;
use wpiutil::wpistring::as_wpistr;

/// Reports usage of a resource of interest.  Repeated calls for the same
/// resource name replace the previous report.
///
/// * resource       the used resource name; convention is to suffix with
///                  "[instanceNum]" for multiple instances of the same
///                  resource
/// * data           arbitrary associated data string
///
/// Returns a mystery handle. Yay!
pub fn report(resource: &str, data: &str) {
    unsafe {
        HAL_ReportUsage(as_wpistr!(resource), as_wpistr!(data));
    }
}
