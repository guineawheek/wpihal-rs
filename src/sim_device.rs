use std::ffi::CStr;

use wpihal_sys::{HAL_CreateSimDevice, HAL_CreateSimValue, HAL_CreateSimValueEnum, HAL_CreateSimValueEnumDouble, HAL_FreeSimDevice, HAL_GetSimDeviceName, HAL_GetSimValue, HAL_SetSimValue, HAL_SimDeviceHandle, HAL_SimValueHandle, HAL_Value};

use crate::value::HALValue;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum SimValueDirection {
    Input = 0,
    Output = 1,
    Bidir = 2,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SimDevice(HAL_SimDeviceHandle);

impl SimDevice {
    pub fn new(name: &CStr) -> Option<Self> {
         unsafe { 
            match HAL_CreateSimDevice(name.as_ptr()) {
                0 => None,
                otherwise => Some(Self(otherwise))
            }
        }
    }


    pub fn handle(&self) -> HAL_SimDeviceHandle {
        self.0
    }

    pub fn get_device_name(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(HAL_GetSimDeviceName(self.0))
        }
    }

    pub fn create_sim_value(&self, name: &CStr, direction: SimValueDirection, initial_value: &HALValue) -> Option<SimValue> {
        unsafe {
            let initial_value: HAL_Value = initial_value.clone().into();
            match HAL_CreateSimValue(self.0, name.as_ptr(), direction as i32, &initial_value) {
                0 => None,
                otherwise => Some(SimValue(otherwise))
            }
        }
    }

    pub fn create_enum(&self, name: &CStr, direction: SimValueDirection, options: &[&CStr], initial_index: usize) -> Option<SimValue> {
        unsafe {
            match HAL_CreateSimValueEnum(
                self.0,
                name.as_ptr(),
                direction as i32,
                options.len() as i32,
                options.as_ptr() as *mut *const i8,
                initial_index as i32
            ) {
                0 => None,
                otherwise => Some(SimValue(otherwise))
            }
        }
    }

    pub fn create_enum_double(&self, name: &CStr, direction: SimValueDirection, options: &[&CStr], option_values: &[f64], initial_index: usize) -> Option<SimValue> {
        unsafe {
            match HAL_CreateSimValueEnumDouble(
                self.0,
                name.as_ptr(),
                direction as i32,
                options.len() as i32,
                options.as_ptr() as *mut *const i8,
                option_values.as_ptr(),
                initial_index as i32
            ) {
                0 => None,
                otherwise => Some(SimValue(otherwise))
            }
        }
    }
}

impl Drop for SimDevice {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeSimDevice(self.0);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimValue(HAL_SimValueHandle); 

impl SimValue {
    pub fn get(&self) -> HALValue {
        let mut data = HAL_Value::default();
        unsafe { HAL_GetSimValue(self.0, &mut data) };
        data.into()
    }

    pub fn set(&self, value: &HALValue) {

        let value: HAL_Value = value.clone().into();
        unsafe {
            HAL_SetSimValue(self.0, &value);
        }
    }
}