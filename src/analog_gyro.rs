use std::ffi::CStr;

use wpihal_sys::{HAL_CalibrateAnalogGyro, HAL_FreeAnalogGyro, HAL_GetAnalogGyroAngle, HAL_GetAnalogGyroCenter, HAL_GetAnalogGyroOffset, HAL_GetAnalogGyroRate, HAL_InitializeAnalogGyro, HAL_ResetAnalogGyro, HAL_SetAnalogGyroDeadband, HAL_SetAnalogGyroParameters, HAL_SetAnalogGyroVoltsPerDegreePerSecond, HAL_SetupAnalogGyro};

use crate::{analog_input::AnalogInput, error::{allocation_location_ptr, HALResult}, hal_call, Handle};

/// Raw analog gyro handle value
pub use wpihal_sys::HAL_GyroHandle as GyroHandle;


/// Analog gyro (i'm so sorry)
#[derive(Debug, PartialEq, Eq)]
pub struct AnalogGyro<'a>(GyroHandle, &'a AnalogInput);

impl<'a> AnalogGyro<'a> {
    /// Initializes an analog gyro.
    /// 
    /// * `handle` - the analog input port handle
    /// * `allocation_location`: optional location where allocation is occuring, for debugging purposes
    /// 
    /// The analog port must be accumulator capable.
    pub fn initialize(handle: &'a AnalogInput, allocation_location: Option<&CStr>) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeAnalogGyro(handle.raw_handle(), allocation_location_ptr(allocation_location)))?, handle))
    }

    /// Sets up the gyro for the KOP analog gyro
    /// (my condolences)
    pub fn setup(&mut self) -> HALResult<()> {
        hal_call!(HAL_SetupAnalogGyro(self.0))
    }

    /// Sets analog gyro parameters
    pub fn set_parameters(&mut self, volts_per_degree_per_second: f64, offset: f64, center: i32) -> HALResult<()> {
        hal_call!(HAL_SetAnalogGyroParameters(self.0, volts_per_degree_per_second, offset, center))
    }

    /// Sets the volts/dps scaling
    pub fn set_volts_per_degree_per_second(&mut self, volts_per_degree_per_second: f64) -> HALResult<()> {
        hal_call!(HAL_SetAnalogGyroVoltsPerDegreePerSecond(self.0, volts_per_degree_per_second))
    }

    /// Resets the value to 0
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

impl<'a> Drop for AnalogGyro<'a> {
    fn drop(&mut self) {
        unsafe { HAL_FreeAnalogGyro(self.0); }
    }
}