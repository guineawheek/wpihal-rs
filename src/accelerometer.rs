use wpihal_sys::hal::{HAL_GetAccelerometerX, HAL_GetAccelerometerY, HAL_GetAccelerometerZ, HAL_SetAccelerometerActive, HAL_SetAccelerometerRange};
pub use wpihal_sys::hal::HAL_AccelerometerRange as AccelerometerRange;

///
/// Sets the accelerometer to active or standby mode.
///
/// It must be in standby mode to change any configuration.
///
/// @param active true to set to active, false for standby
///
#[inline(always)]
pub fn set_accelerometer_active(active: bool) {
    unsafe { HAL_SetAccelerometerActive(active as i32) }
}

///
/// Sets the range of values that can be measured (either 2, 4, or 8 g-forces).
//
/// The accelerometer should be in standby mode when this is called.
//
/// @param range the accelerometer range
///
pub fn set_accelerometer_range(range: AccelerometerRange) {
    unsafe { HAL_SetAccelerometerRange(range) }
}

///
/// Gets the x-axis acceleration.
//
/// This is a floating point value in units of 1 g-force.
//
/// @return the X acceleration
///
pub fn get_accelerometer_x() -> f64 {
    unsafe { HAL_GetAccelerometerX() }
}

///
/// Gets the y-axis acceleration.
//
/// This is a floating point value in units of 1 g-force.
//
/// @return the Y acceleration
///
pub fn get_accelerometer_y() -> f64 {
    unsafe { HAL_GetAccelerometerY() }
}

///
/// Gets the z-axis acceleration.
//
/// This is a floating point value in units of 1 g-force.
//
/// @return the Z acceleration
///
pub fn get_accelerometer_z() -> f64 {
    unsafe { HAL_GetAccelerometerZ() }
}