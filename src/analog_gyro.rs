use std::ffi::CStr;

use wpihal_sys::hal::{HAL_CalibrateAnalogGyro, HAL_FreeAnalogGyro, HAL_GetAnalogGyroAngle, HAL_GetAnalogGyroCenter, HAL_GetAnalogGyroOffset, HAL_GetAnalogGyroRate, HAL_GyroHandle, HAL_InitializeAnalogGyro, HAL_ResetAnalogGyro, HAL_SetAnalogGyroDeadband, HAL_SetAnalogGyroParameters, HAL_SetAnalogGyroVoltsPerDegreePerSecond, HAL_SetupAnalogGyro};

use crate::{analog_input::AnalogInput, error::{allocation_location_ptr, HALResult}, hal_call};


/// analog gyro handle (i'm so sorry)
pub struct AnalogGyro(HAL_GyroHandle);

impl AnalogGyro {
    /// Initializes an analog gyro.
    /// 
    /// * handle: the analog input port
    /// * allocation_location: optional location where allocation is occuring, for debugging purposes
    pub fn initialize(handle: AnalogInput, allocation_location: Option<&CStr>) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeAnalogGyro(handle.0, allocation_location_ptr(allocation_location)))?))
    }

    /// sets up the gyro for the kop gyro
    pub fn setup(&mut self) -> HALResult<()> {
        hal_call!(HAL_SetupAnalogGyro(self.0))
    }

    pub fn set_parameters(&mut self, volts_per_degree_per_second: f64, offset: f64, center: i32) -> HALResult<()> {
        hal_call!(HAL_SetAnalogGyroParameters(self.0, volts_per_degree_per_second, offset, center))
    }

    pub fn set_volts_per_degree_per_second(&mut self, volts_per_degree_per_second: f64) -> HALResult<()> {
        hal_call!(HAL_SetAnalogGyroVoltsPerDegreePerSecond(self.0, volts_per_degree_per_second))
    }

    pub fn reset(&mut self) -> HALResult<()> {
        hal_call!(HAL_ResetAnalogGyro(self.0))
    }

    /// Calibrates the analog gyro.
    ///
    /// This happens by calculating the average value of the gyro over 5 seconds, and
    /// setting that as the center. Note that this call blocks for 5 seconds to
    /// perform this.
    pub fn calibrate(&mut self) -> HALResult<()> {
        hal_call!(HAL_CalibrateAnalogGyro(self.0))
    }

    /// voltage deadband
    pub fn set_deadband(&mut self, volts: f64) -> HALResult<()> {
        hal_call!(HAL_SetAnalogGyroDeadband(self.0, volts))
    }

    /// degrees
    pub fn get_angle(&self) -> HALResult<f64> {
        hal_call!(HAL_GetAnalogGyroAngle(self.0))
    }

    /// degrees/second
    pub fn get_rate(&self) -> HALResult<f64> {
        hal_call!(HAL_GetAnalogGyroRate(self.0))
    }

    /// calibration offset
    pub fn get_offset(&self) -> HALResult<f64> {
        hal_call!(HAL_GetAnalogGyroOffset(self.0))
    }

    /// calibration center
    pub fn get_center(&self) -> HALResult<i32> {
        hal_call!(HAL_GetAnalogGyroCenter(self.0))
    }
}

impl Drop for AnalogGyro {
    fn drop(&mut self) {
        unsafe { HAL_FreeAnalogGyro(self.0); }
    }
}