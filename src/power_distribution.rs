use std::ffi::CStr;

use wpihal_sys::{
    HAL_CheckPowerDistributionChannel, HAL_CheckPowerDistributionModule,
    HAL_CleanPowerDistribution, HAL_ClearPowerDistributionStickyFaults,
    HAL_FreePowerDistributionStreamData, HAL_GetPowerDistributionAllChannelCurrents,
    HAL_GetPowerDistributionChannelCurrent, HAL_GetPowerDistributionFaults,
    HAL_GetPowerDistributionModuleNumber, HAL_GetPowerDistributionNumChannels,
    HAL_GetPowerDistributionStickyFaults, HAL_GetPowerDistributionStreamData,
    HAL_GetPowerDistributionSwitchableChannel, HAL_GetPowerDistributionTemperature,
    HAL_GetPowerDistributionTotalCurrent, HAL_GetPowerDistributionTotalEnergy,
    HAL_GetPowerDistributionTotalPower, HAL_GetPowerDistributionType,
    HAL_GetPowerDistributionVersion, HAL_GetPowerDistributionVoltage,
    HAL_InitializePowerDistribution, HAL_PowerDistributionChannelData, HAL_PowerDistributionFaults,
    HAL_PowerDistributionHandle, HAL_PowerDistributionStickyFaults, HAL_PowerDistributionType,
    HAL_PowerDistributionVersion, HAL_ResetPowerDistributionTotalEnergy,
    HAL_SetPowerDistributionSwitchableChannel, HAL_StartPowerDistributionStream,
    HAL_StopPowerDistributionStream,
};

use crate::{
    error::{HALResult, allocation_location_ptr},
    hal_call,
};

pub type PowerDistributionType = HAL_PowerDistributionType;
pub type PowerDistributionVersion = HAL_PowerDistributionVersion;
pub type PowerDistributionFaults = HAL_PowerDistributionFaults;
pub type PowerDistributionStickyFaults = HAL_PowerDistributionStickyFaults;
pub type PowerDistributionChannelData = HAL_PowerDistributionChannelData;

#[derive(Debug, PartialEq, Eq)]
pub struct PowerDistribution(HAL_PowerDistributionHandle);

impl PowerDistribution {
    pub fn initialize(
        module_number: i32,
        pd_type: PowerDistributionType,
        allocation_location: Option<&CStr>,
    ) -> HALResult<PowerDistribution> {
        Ok(Self(hal_call!(HAL_InitializePowerDistribution(
            module_number,
            pd_type,
            allocation_location_ptr(allocation_location),
        ))?))
    }

    pub fn get_module_number(&self) -> HALResult<i32> {
        hal_call!(HAL_GetPowerDistributionModuleNumber(self.0))
    }

    pub fn check_channel(&self, channel: i32) -> bool {
        unsafe { HAL_CheckPowerDistributionChannel(self.0, channel) != 0 }
    }

    pub fn check_module(module: i32, pd_type: PowerDistributionType) -> bool {
        unsafe { HAL_CheckPowerDistributionModule(module, pd_type) != 0 }
    }

    pub fn get_type(&self) -> HALResult<PowerDistributionType> {
        hal_call!(HAL_GetPowerDistributionType(self.0))
    }

    pub fn get_num_channels(&self) -> HALResult<i32> {
        hal_call!(HAL_GetPowerDistributionNumChannels(self.0))
    }

    pub fn get_temperature(&self) -> HALResult<f64> {
        hal_call!(HAL_GetPowerDistributionTemperature(self.0))
    }

    pub fn get_voltage(&self) -> HALResult<f64> {
        hal_call!(HAL_GetPowerDistributionVoltage(self.0))
    }

    pub fn get_current(&self, channel: i32) -> HALResult<f64> {
        hal_call!(HAL_GetPowerDistributionChannelCurrent(self.0, channel))
    }

    pub fn get_all_channel_currents(&self, currents: &mut [f64]) -> HALResult<()> {
        hal_call!(HAL_GetPowerDistributionAllChannelCurrents(
            self.0,
            currents.as_mut_ptr(),
            currents.len() as i32
        ))
    }

    pub fn get_total_current(&self) -> HALResult<f64> {
        hal_call!(HAL_GetPowerDistributionTotalCurrent(self.0))
    }

    pub fn get_total_power(&self) -> HALResult<f64> {
        hal_call!(HAL_GetPowerDistributionTotalPower(self.0))
    }

    pub fn get_total_energy(&self) -> HALResult<f64> {
        hal_call!(HAL_GetPowerDistributionTotalEnergy(self.0))
    }

    pub fn reset_total_energy(&mut self) -> HALResult<()> {
        hal_call!(HAL_ResetPowerDistributionTotalEnergy(self.0))
    }

    pub fn clear_sticky_faults(&mut self) -> HALResult<()> {
        hal_call!(HAL_ClearPowerDistributionStickyFaults(self.0))
    }

    pub fn set_switchable_channel(&mut self, enabled: bool) -> HALResult<()> {
        hal_call!(HAL_SetPowerDistributionSwitchableChannel(
            self.0,
            enabled as i32
        ))
    }

    pub fn get_switchable_channel(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetPowerDistributionSwitchableChannel(self.0))? != 0)
    }

    pub fn get_version(&self) -> HALResult<PowerDistributionVersion> {
        let mut version = PowerDistributionVersion::default();
        hal_call!(HAL_GetPowerDistributionVersion(self.0, &mut version))?;
        Ok(version)
    }

    pub fn get_faults(&self) -> HALResult<PowerDistributionFaults> {
        let mut faults = PowerDistributionFaults::default();
        hal_call!(HAL_GetPowerDistributionFaults(self.0, &mut faults))?;
        Ok(faults)
    }

    pub fn get_sticky_faults(&self) -> HALResult<PowerDistributionStickyFaults> {
        let mut faults = PowerDistributionStickyFaults::default();
        hal_call!(HAL_GetPowerDistributionStickyFaults(self.0, &mut faults))?;
        Ok(faults)
    }

    pub fn start_stream(&mut self) -> HALResult<()> {
        hal_call!(HAL_StartPowerDistributionStream(self.0))
    }

    pub fn stop_stream(&mut self) -> HALResult<()> {
        hal_call!(HAL_StopPowerDistributionStream(self.0))
    }

    pub fn get_stream(&self) -> HALResult<PowerDistributionStreamData> {
        let mut count = 0i32;
        let ptr = hal_call!(HAL_GetPowerDistributionStreamData(self.0, &mut count))?;
        Ok(PowerDistributionStreamData {
            ptr,
            len: count as usize,
        })
    }
}

impl Drop for PowerDistribution {
    fn drop(&mut self) {
        unsafe {
            HAL_CleanPowerDistribution(self.0);
        }
    }
}

pub struct PowerDistributionStreamData {
    ptr: *mut HAL_PowerDistributionChannelData,
    len: usize,
}

impl PowerDistributionStreamData {
    pub fn as_slice(&self) -> &[PowerDistributionChannelData] {
        unsafe { core::slice::from_raw_parts(self.ptr as *const _, self.len) }
    }
}

impl Drop for PowerDistributionStreamData {
    fn drop(&mut self) {
        unsafe {
            HAL_FreePowerDistributionStreamData(self.ptr, self.len as i32);
        }
    }
}
