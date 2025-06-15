use wpihal_sys::{
    HAL_GetBrownoutVoltage, HAL_GetCPUTemp, HAL_GetUserActive3V3, HAL_GetUserCurrent3V3,
    HAL_GetUserCurrentFaults3V3, HAL_GetUserVoltage3V3, HAL_GetVinVoltage,
    HAL_ResetUserCurrentFaults, HAL_SetBrownoutVoltage, HAL_SetUserRailEnabled3V3,
};

use crate::{error::HALResult, hal_call};

pub fn get_vin_voltage() -> HALResult<f64> {
    hal_call!(HAL_GetVinVoltage())
}

pub fn get_user_voltage_3v3() -> HALResult<f64> {
    hal_call!(HAL_GetUserVoltage3V3())
}

pub fn get_user_current_3v3() -> HALResult<f64> {
    hal_call!(HAL_GetUserCurrent3V3())
}

pub fn get_user_active_3v3() -> HALResult<bool> {
    Ok(hal_call!(HAL_GetUserActive3V3())? != 0)
}

pub fn get_user_current_faults_3v3() -> HALResult<u8> {
    Ok(hal_call!(HAL_GetUserCurrentFaults3V3())? as u8)
}

pub fn set_user_rail_enabled_3v3(enabled: bool) -> HALResult<()> {
    hal_call!(HAL_SetUserRailEnabled3V3(enabled as i32))
}

pub fn reset_user_current_faults() -> HALResult<()> {
    hal_call!(HAL_ResetUserCurrentFaults())
}

pub fn get_brownout_voltage() -> HALResult<f64> {
    hal_call!(HAL_GetBrownoutVoltage())
}

pub fn set_brownout_voltage(voltage: f64) -> HALResult<()> {
    hal_call!(HAL_SetBrownoutVoltage(voltage))
}

pub fn get_cpu_temp() -> HALResult<f64> {
    hal_call!(HAL_GetCPUTemp())
}
