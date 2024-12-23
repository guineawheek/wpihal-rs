use std::ffi::CStr;

use wpihal_sys::{HAL_CheckREVPHModuleNumber, HAL_CheckREVPHSolenoidChannel, HAL_ClearREVPHStickyFaults, HAL_FireREVPHOneShot, HAL_FreeREVPH, HAL_GetREVPH5VVoltage, HAL_GetREVPHAnalogVoltage, HAL_GetREVPHCompressor, HAL_GetREVPHCompressorCurrent, HAL_GetREVPHFaults, HAL_GetREVPHPressureSwitch, HAL_GetREVPHSolenoidCurrent, HAL_GetREVPHSolenoidDisabledList, HAL_GetREVPHSolenoidVoltage, HAL_GetREVPHSolenoids, HAL_GetREVPHStickyFaults, HAL_GetREVPHVersion, HAL_GetREVPHVoltage, HAL_InitializeREVPH, HAL_REVPHCompressorConfig, HAL_REVPHFaults, HAL_REVPHHandle, HAL_REVPHStickyFaults, HAL_REVPHVersion, HAL_SetREVPHClosedLoopControlAnalog, HAL_SetREVPHClosedLoopControlDigital, HAL_SetREVPHClosedLoopControlDisabled, HAL_SetREVPHClosedLoopControlHybrid, HAL_SetREVPHCompressorConfig, HAL_SetREVPHSolenoids};

use crate::{error::{allocation_location_ptr, HALResult}, hal_call};

pub type REVPHCompressorConfig = HAL_REVPHCompressorConfig;
pub type REVPHVersion = HAL_REVPHVersion;
pub type REVPHFaults = HAL_REVPHFaults;
pub type REVPHStickyFaults = HAL_REVPHStickyFaults;

pub enum ClosedLoopControlMode {
    Disabled,
    Digital,
    Analog { min_voltage: f64, max_voltage: f64 },
    Hybrid { min_voltage: f64, max_voltage: f64 },
}

pub struct REVPH(HAL_REVPHHandle);

impl REVPH {
    pub fn initialize(module: i32, allocation_location: Option<&CStr>) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeREVPH(module, allocation_location_ptr(allocation_location)))?))
    }

    pub fn get_compressor(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetREVPHCompressor(self.0))? != 0)
    }

    pub fn set_compressor_config(&mut self, cfg: REVPHCompressorConfig) -> HALResult<()> {
        hal_call!(HAL_SetREVPHCompressorConfig(self.0, &cfg))
    }

    pub fn set_closed_loop_control(&mut self, mode: ClosedLoopControlMode) -> HALResult<()> {
        match mode {
            ClosedLoopControlMode::Disabled => hal_call!(HAL_SetREVPHClosedLoopControlDisabled(self.0)),
            ClosedLoopControlMode::Digital => hal_call!(HAL_SetREVPHClosedLoopControlDigital(self.0)),
            ClosedLoopControlMode::Analog { min_voltage, max_voltage } => hal_call!(HAL_SetREVPHClosedLoopControlAnalog(self.0, min_voltage, max_voltage)),
            ClosedLoopControlMode::Hybrid { min_voltage, max_voltage } => hal_call!(HAL_SetREVPHClosedLoopControlHybrid(self.0, min_voltage, max_voltage)),
        }
    }

    pub fn get_pressure_switch(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetREVPHPressureSwitch(self.0))? != 0)
    }

    pub fn get_compressor_current(&self) -> HALResult<f64> {
        hal_call!(HAL_GetREVPHCompressorCurrent(self.0))
    }

    pub fn get_analog_voltage(&self, channel: i32) -> HALResult<f64> {
        hal_call!(HAL_GetREVPHAnalogVoltage(self.0, channel))
    }

    pub fn get_voltage(&self) -> HALResult<f64> {
        hal_call!(HAL_GetREVPHVoltage(self.0))
    }

    pub fn get_5v_voltage(&self) -> HALResult<f64> {
        hal_call!(HAL_GetREVPH5VVoltage(self.0))
    }

    pub fn get_solenoid_current(&self) -> HALResult<f64> {
        hal_call!(HAL_GetREVPHSolenoidCurrent(self.0))
    }

    pub fn get_solenoid_voltage(&self) -> HALResult<f64> {
        hal_call!(HAL_GetREVPHSolenoidVoltage(self.0))
    }

    pub fn get_version(&self) -> HALResult<REVPHVersion> {
        let mut version = HAL_REVPHVersion::default();
        hal_call!(HAL_GetREVPHVersion(self.0, &mut version))?;
        Ok(version)
    }


    pub fn get_solenoids(&self) -> HALResult<u32> {
        Ok(hal_call!(HAL_GetREVPHSolenoids(self.0))? as u32)
    }

    pub fn set_solenoids(&mut self, mask: u32, values: u32) -> HALResult<()> {
        hal_call!(HAL_SetREVPHSolenoids(self.0, mask as i32, values as i32))
    }

    pub fn get_solenoid_disabled_list(&self) -> HALResult<u32> {
        Ok(hal_call!(HAL_GetREVPHSolenoidDisabledList(self.0))? as u32)
    }

    pub fn fire_one_shot(&mut self, index: i32, duration_ms: i32) -> HALResult<()> {
        hal_call!(HAL_FireREVPHOneShot(self.0, index, duration_ms))
    }

    pub fn get_faults(&self) -> HALResult<REVPHFaults> {
        let mut faults = HAL_REVPHFaults::default();
        hal_call!(HAL_GetREVPHFaults(self.0, &mut faults))?;
        Ok(faults)
    }

    pub fn get_sticky_faults(&self) -> HALResult<REVPHStickyFaults> {
        let mut faults = HAL_REVPHStickyFaults::default();
        hal_call!(HAL_GetREVPHStickyFaults(self.0, &mut faults))?;
        Ok(faults)
    }

    pub fn clear_sticky_faults(&mut self) -> HALResult<()> {
        hal_call!(HAL_ClearREVPHStickyFaults(self.0))
    }

    pub fn check_solenoid_channel(channel: i32) -> bool {
        unsafe { HAL_CheckREVPHSolenoidChannel(channel) != 0 }
    }

    pub fn check_module_number(module: i32) -> bool {
        unsafe { HAL_CheckREVPHModuleNumber(module) != 0 }
    }


}

impl Drop for REVPH {
    fn drop(&mut self) {
        unsafe { HAL_FreeREVPH(self.0); }
    }
}