use wpihal_sys::{
    HAL_Acceleration3d, HAL_EulerAngles3d, HAL_GetIMUAcceleration, HAL_GetIMUEulerAnglesFlat,
    HAL_GetIMUEulerAnglesLandscape, HAL_GetIMUEulerAnglesPortrait, HAL_GetIMUGyroRates,
    HAL_GetIMUQuaternion, HAL_GetIMUYawFlat, HAL_GetIMUYawLandscape, HAL_GetIMUYawPortrait,
    HAL_GyroRate3d, HAL_Quaternion,
};

use crate::{error::HALResult, hal_call};

pub fn get_imu_acceleration() -> HALResult<HAL_Acceleration3d> {
    let mut data = HAL_Acceleration3d::default();
    hal_call!(HAL_GetIMUAcceleration(&mut data))?;
    Ok(data)
}

pub fn get_imu_gyro_rates() -> HALResult<HAL_GyroRate3d> {
    let mut data = HAL_GyroRate3d::default();
    hal_call!(HAL_GetIMUGyroRates(&mut data))?;
    Ok(data)
}

pub fn get_imu_euler_angles_flat() -> HALResult<HAL_EulerAngles3d> {
    let mut data = HAL_EulerAngles3d::default();
    hal_call!(HAL_GetIMUEulerAnglesFlat(&mut data))?;
    Ok(data)
}

pub fn get_imu_euler_angles_landscape() -> HALResult<HAL_EulerAngles3d> {
    let mut data = HAL_EulerAngles3d::default();
    hal_call!(HAL_GetIMUEulerAnglesLandscape(&mut data))?;
    Ok(data)
}

pub fn get_imu_euler_angles_portrait() -> HALResult<HAL_EulerAngles3d> {
    let mut data = HAL_EulerAngles3d::default();
    hal_call!(HAL_GetIMUEulerAnglesPortrait(&mut data))?;
    Ok(data)
}

pub fn get_imu_quaternion() -> HALResult<HAL_Quaternion> {
    let mut data = HAL_Quaternion::default();
    hal_call!(HAL_GetIMUQuaternion(&mut data))?;
    Ok(data)
}

pub fn get_imu_yaw_flat() -> (f64, i64) {
    let mut timestamp = 0_i64;
    let value = unsafe { HAL_GetIMUYawFlat(&mut timestamp) };
    (value, timestamp)
}

pub fn get_imu_yaw_landscape() -> (f64, i64) {
    let mut timestamp = 0_i64;
    let value = unsafe { HAL_GetIMUYawLandscape(&mut timestamp) };
    (value, timestamp)
}

pub fn get_imu_yaw_portrait() -> (f64, i64) {
    let mut timestamp = 0_i64;
    let value = unsafe { HAL_GetIMUYawPortrait(&mut timestamp) };
    (value, timestamp)
}
