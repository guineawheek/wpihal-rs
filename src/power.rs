use wpihal_sys::{HAL_GetBrownoutVoltage, HAL_GetCPUTemp, HAL_GetUserActive3V3, HAL_GetUserActive5V, HAL_GetUserActive6V, HAL_GetUserCurrent3V3, HAL_GetUserCurrent5V, HAL_GetUserCurrent6V, HAL_GetUserCurrentFaults3V3, HAL_GetUserCurrentFaults5V, HAL_GetUserCurrentFaults6V, HAL_GetUserVoltage3V3, HAL_GetUserVoltage5V, HAL_GetUserVoltage6V, HAL_GetVinCurrent, HAL_GetVinVoltage, HAL_ResetUserCurrentFaults, HAL_SetBrownoutVoltage, HAL_SetUserRailEnabled3V3, HAL_SetUserRailEnabled5V, HAL_SetUserRailEnabled6V};

use crate::{error::HALResult, hal_call};


pub fn get_vin_voltage() -> HALResult<f64> {
    hal_call!(HAL_GetVinVoltage())
}

pub fn get_vin_current() -> HALResult<f64> {
    hal_call!(HAL_GetVinCurrent())
}

pub fn get_user_voltage_6v() -> HALResult<f64> {
    hal_call!(HAL_GetUserVoltage6V())
}

pub fn get_user_current_6v() -> HALResult<f64> {
    hal_call!(HAL_GetUserCurrent6V())
}

pub fn get_user_active_6v() -> HALResult<bool> {
    Ok(hal_call!(HAL_GetUserActive6V())? != 0)
}

pub fn get_user_current_faults_6v() -> HALResult<u8> {
    Ok(hal_call!(HAL_GetUserCurrentFaults6V())? as u8)
}

pub fn set_user_rail_enabled_6v(enabled: bool) -> HALResult<()> {
    hal_call!(HAL_SetUserRailEnabled6V(enabled as i32))
}

pub fn get_user_voltage_5v() -> HALResult<f64> {
    hal_call!(HAL_GetUserVoltage5V())
}

pub fn get_user_current_5v() -> HALResult<f64> {
    hal_call!(HAL_GetUserCurrent5V())
}

pub fn get_user_active_5v() -> HALResult<bool> {
    Ok(hal_call!(HAL_GetUserActive5V())? != 0)
}

pub fn get_user_current_faults_5v() -> HALResult<u8> {
    Ok(hal_call!(HAL_GetUserCurrentFaults5V())? as u8)
}

pub fn set_user_rail_enabled_5v(enabled: bool) -> HALResult<()> {
    hal_call!(HAL_SetUserRailEnabled5V(enabled as i32))
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