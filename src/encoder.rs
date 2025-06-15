use wpihal_sys::{
    HAL_EncoderEncodingType, HAL_EncoderHandle, HAL_EncoderIndexingType, HAL_FreeEncoder,
    HAL_GetEncoder, HAL_GetEncoderDecodingScaleFactor, HAL_GetEncoderDirection,
    HAL_GetEncoderDistance, HAL_GetEncoderDistancePerPulse, HAL_GetEncoderEncodingScale,
    HAL_GetEncoderEncodingType, HAL_GetEncoderFPGAIndex, HAL_GetEncoderPeriod, HAL_GetEncoderRate,
    HAL_GetEncoderRaw, HAL_GetEncoderStopped, HAL_InitializeEncoder, HAL_ResetEncoder,
    HAL_SetEncoderDistancePerPulse, HAL_SetEncoderMaxPeriod, HAL_SetEncoderMinRate,
    HAL_SetEncoderReverseDirection, HAL_SetEncoderSimDevice,
};

use crate::{error::HALResult, hal_call, sim_device::SimDevice};

pub type IndexingType = HAL_EncoderIndexingType;
pub type EncodingType = HAL_EncoderEncodingType;

#[derive(Debug, PartialEq, Eq)]
pub struct Encoder {
    handle: HAL_EncoderHandle,
    a_channel: i32,
    b_channel: i32,
}

impl<'a> Encoder {
    pub fn initialize(
        a_channel: i32,
        b_channel: i32,
        reverse_dir: bool,
        encoding: EncodingType,
    ) -> HALResult<Self> {
        let handle = hal_call!(HAL_InitializeEncoder(
            a_channel,
            b_channel,
            reverse_dir as i32,
            encoding
        ))?;

        Ok(Self {
            handle,
            a_channel,
            b_channel,
        })
    }

    pub fn set_sim_device(&mut self, device: &SimDevice) {
        unsafe {
            HAL_SetEncoderSimDevice(self.handle, device.handle());
        }
    }

    pub fn get(&self) -> HALResult<i32> {
        hal_call!(HAL_GetEncoder(self.handle))
    }

    pub fn get_raw(&self) -> HALResult<i32> {
        hal_call!(HAL_GetEncoderRaw(self.handle))
    }

    pub fn get_encoding_scale(&self) -> HALResult<i32> {
        hal_call!(HAL_GetEncoderEncodingScale(self.handle))
    }

    pub fn reset_encoder(&mut self) -> HALResult<()> {
        hal_call!(HAL_ResetEncoder(self.handle))
    }

    pub fn get_period(&self) -> HALResult<f64> {
        hal_call!(HAL_GetEncoderPeriod(self.handle))
    }

    pub fn set_max_period(&mut self, max_period: f64) -> HALResult<()> {
        hal_call!(HAL_SetEncoderMaxPeriod(self.handle, max_period))
    }

    pub fn get_stopped(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetEncoderStopped(self.handle))? != 0)
    }

    pub fn get_direction(&self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetEncoderDirection(self.handle))? != 0)
    }

    pub fn get_distance(&self) -> HALResult<f64> {
        hal_call!(HAL_GetEncoderDistance(self.handle))
    }

    pub fn get_rate(&self) -> HALResult<f64> {
        hal_call!(HAL_GetEncoderRate(self.handle))
    }

    pub fn set_min_rate(&mut self, min_rate: f64) -> HALResult<()> {
        hal_call!(HAL_SetEncoderMinRate(self.handle, min_rate))
    }

    pub fn set_distance_per_pulse(&mut self, distance_per_pulse: f64) -> HALResult<()> {
        hal_call!(HAL_SetEncoderDistancePerPulse(
            self.handle,
            distance_per_pulse
        ))
    }

    pub fn set_reverse_direction(&mut self, reverse_dir: bool) -> HALResult<()> {
        hal_call!(HAL_SetEncoderReverseDirection(
            self.handle,
            reverse_dir as i32
        ))
    }

    pub fn set_samples_to_average(&mut self, samples_to_average: i32) -> HALResult<()> {
        hal_call!(HAL_SetEncoderReverseDirection(
            self.handle,
            samples_to_average
        ))
    }

    pub fn get_fpga_index(&self) -> HALResult<i32> {
        hal_call!(HAL_GetEncoderFPGAIndex(self.handle))
    }

    pub fn get_decoding_scale_factor(&self) -> HALResult<f64> {
        hal_call!(HAL_GetEncoderDecodingScaleFactor(self.handle))
    }

    pub fn get_distance_per_pulse(&self) -> HALResult<f64> {
        hal_call!(HAL_GetEncoderDistancePerPulse(self.handle))
    }

    pub fn get_encoding_type(&self) -> HALResult<EncodingType> {
        hal_call!(HAL_GetEncoderEncodingType(self.handle))
    }

    pub unsafe fn raw_handle(&self) -> HAL_EncoderHandle {
        self.handle
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeEncoder(self.handle);
        }
    }
}
