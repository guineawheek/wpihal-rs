use std::ffi::CStr;

use wpihal_sys::hal::{HAL_CTREPCMHandle, HAL_CheckCTREPCMSolenoidChannel, HAL_ClearAllCTREPCMStickyFaults, HAL_FireCTREPCMOneShot, HAL_FreeCTREPCM, HAL_GetCTREPCMClosedLoopControl, HAL_GetCTREPCMCompressor, HAL_GetCTREPCMCompressorCurrent, HAL_GetCTREPCMCompressorCurrentTooHighFault, HAL_GetCTREPCMCompressorCurrentTooHighStickyFault, HAL_GetCTREPCMCompressorNotConnectedFault, HAL_GetCTREPCMCompressorNotConnectedStickyFault, HAL_GetCTREPCMCompressorShortedFault, HAL_GetCTREPCMCompressorShortedStickyFault, HAL_GetCTREPCMPressureSwitch, HAL_GetCTREPCMSolenoidDisabledList, HAL_GetCTREPCMSolenoidVoltageFault, HAL_GetCTREPCMSolenoidVoltageStickyFault, HAL_GetCTREPCMSolenoids, HAL_InitializeCTREPCM, HAL_SetCTREPCMClosedLoopControl, HAL_SetCTREPCMOneShotDuration, HAL_SetCTREPCMSolenoids};

use crate::{error::{allocation_location_ptr, HALResult}, hal_call};

pub struct CTREPCM(HAL_CTREPCMHandle);

impl CTREPCM {
    pub fn initialize(module: i32, allocation_location: Option<&CStr>) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeCTREPCM(module, allocation_location_ptr(allocation_location)))?))
    }

    pub fn get_compressor(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCTREPCMCompressor(self.0))? != 0)
    }

    pub fn set_closed_loop_control(&mut self, enabled: bool) -> HALResult<()> {
        hal_call!(HAL_SetCTREPCMClosedLoopControl(self.0, enabled as i32))
    }

    pub fn get_closed_loop_control(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCTREPCMClosedLoopControl(self.0))? != 0)
    }

    pub fn get_pressure_switch(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCTREPCMPressureSwitch(self.0))? != 0)
    }

    pub fn get_compressor_current(&self) -> HALResult<f64> {
        hal_call!(HAL_GetCTREPCMCompressorCurrent(self.0))
    }

    pub fn get_compressor_current_too_high_fault(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCTREPCMCompressorCurrentTooHighFault(self.0))? != 0)
    }

    pub fn get_compressor_current_too_high_sticky_fault(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCTREPCMCompressorCurrentTooHighStickyFault(self.0))? != 0)
    }

    pub fn get_compressor_shorted_fault(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCTREPCMCompressorShortedFault(self.0))? != 0)
    }

    pub fn get_compressor_shorted_sticky_fault(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCTREPCMCompressorShortedStickyFault(self.0))? != 0)
    }

    pub fn get_compressor_not_connected_fault(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCTREPCMCompressorNotConnectedFault(self.0))? != 0)
    }

    pub fn get_compressor_not_connected_sticky_fault(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCTREPCMCompressorNotConnectedStickyFault(self.0))? != 0)
    }

    pub fn get_solenoids(&self) -> HALResult<u32> {
        Ok(hal_call!(HAL_GetCTREPCMSolenoids(self.0))? as u32)
    }

    pub fn set_solenoids(&mut self, mask: u32, values: u32) -> HALResult<()> {
        hal_call!(HAL_SetCTREPCMSolenoids(self.0, mask as i32, values as i32))
    }

    pub fn get_solenoid_disabled_list(&self) -> HALResult<u32> {
        Ok(hal_call!(HAL_GetCTREPCMSolenoidDisabledList(self.0))? as u32)
    }

    pub fn get_solenoid_voltage_fault(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCTREPCMSolenoidVoltageFault(self.0))? != 0)
    }

    pub fn get_solenoid_voltage_sticky_fault(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetCTREPCMSolenoidVoltageStickyFault(self.0))? != 0)
    }

    pub fn clear_all_sticky_faults(&mut self) -> HALResult<()> {
        hal_call!(HAL_ClearAllCTREPCMStickyFaults(self.0))
    }

    pub fn fire_one_shot(&mut self, index: u32) -> HALResult<()> {
        hal_call!(HAL_FireCTREPCMOneShot(self.0, index as i32))
    }

    pub fn set_one_shot_duration(&mut self, index: u32, duration_ms: u32) -> HALResult<()> {
        hal_call!(HAL_SetCTREPCMOneShotDuration(self.0, index as i32, duration_ms as i32))
    }


}

impl Drop for CTREPCM {
    fn drop(&mut self) {
        unsafe { HAL_FreeCTREPCM(self.0); }
    }
}

pub fn check_solenoid_channel(channel: i32) -> bool {
    unsafe { HAL_CheckCTREPCMSolenoidChannel(channel) != 0 }
}