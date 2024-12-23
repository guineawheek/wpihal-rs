use wpihal_sys::{HAL_DutyCycleHandle, HAL_FreeDutyCycle, HAL_GetDutyCycleFPGAIndex, HAL_GetDutyCycleFrequency, HAL_GetDutyCycleHighTime, HAL_GetDutyCycleOutput, HAL_GetDutyCycleOutputScaleFactor, HAL_InitializeDutyCycle, HAL_SetDutyCycleSimDevice, HAL_SimDeviceHandle};

use crate::{analog_trigger::{AnalogTrigger, AnalogTriggerType}, dio::DIO, error::HALResult, hal_call, Handle};


#[derive(Debug, PartialEq, Eq)]
pub struct DutyCycle<'a> {
    handle: HAL_DutyCycleHandle,
    src: DutyCycleSource<'a>
}

#[derive(Debug, PartialEq, Eq)]
enum DutyCycleSource<'a> {
    Unknown,
    DigitalHandle(&'a DIO),
    AnalogTriggerHandle(&'a AnalogTrigger<'a>),
}

impl<'a> DutyCycle<'a> {
    pub fn initialize_from_dio(dio: &'a DIO) -> HALResult<Self> {
        let handle = hal_call!(HAL_InitializeDutyCycle(dio.raw_handle(), AnalogTriggerType::kInWindow))?;
        Ok(Self { handle, src: DutyCycleSource::DigitalHandle(dio) })
    }

    pub fn initialize_from_analog_trigger(trg: &'a AnalogTrigger<'a>, trg_type: AnalogTriggerType) -> HALResult<Self> {
        let handle = hal_call!(HAL_InitializeDutyCycle(trg.raw_handle(), trg_type))?;
        Ok(Self { handle, src: DutyCycleSource::AnalogTriggerHandle(trg) })
    }

    pub fn set_sim_device(&mut self, handle: HAL_SimDeviceHandle) {
        unsafe { HAL_SetDutyCycleSimDevice(self.handle, handle); }
    }

    pub fn get_frequency(&self) -> HALResult<i32> {
        hal_call!(HAL_GetDutyCycleFrequency(self.handle))
    }

    pub fn get_output(&self) -> HALResult<f64> {
        hal_call!(HAL_GetDutyCycleOutput(self.handle))
    }

    pub fn get_high_time(&self) -> HALResult<i32> {
        hal_call!(HAL_GetDutyCycleHighTime(self.handle))
    }

    pub fn get_output_scale_factor(&self) -> HALResult<i32> {
        hal_call!(HAL_GetDutyCycleOutputScaleFactor(self.handle))
    }

    pub fn get_fpga_index(&self) -> HALResult<i32> {
        hal_call!(HAL_GetDutyCycleFPGAIndex(self.handle))
    }
}

impl<'a> Drop for DutyCycle<'a> {
    fn drop(&mut self) {
        unsafe { HAL_FreeDutyCycle(self.handle); }
    }
}

impl<'a> Handle<HAL_DutyCycleHandle> for DutyCycle<'a> {
    unsafe fn raw_handle(&self) -> HAL_DutyCycleHandle {
        self.handle
    }

    unsafe fn from_raw_handle(handle: HAL_DutyCycleHandle) -> Self {
        Self {
            handle,
            src: DutyCycleSource::Unknown
        }
    }
}