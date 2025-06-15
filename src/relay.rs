use std::ffi::CStr;

use wpihal_sys::{
    HAL_CheckRelayChannel, HAL_FreeRelayPort, HAL_GetRelay, HAL_InitializeRelayPort,
    HAL_PortHandle, HAL_RelayHandle, HAL_SetRelay,
};

use crate::{
    error::{HALResult, allocation_location_ptr},
    hal_call,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Relay(HAL_RelayHandle);

impl Relay {
    pub fn initialize(
        handle: HAL_PortHandle,
        forward: bool,
        allocation_location: Option<&CStr>,
    ) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeRelayPort(
            handle,
            forward as i32,
            allocation_location_ptr(allocation_location)
        ))?))
    }

    pub fn check_channel(channel: i32) -> bool {
        unsafe { HAL_CheckRelayChannel(channel) != 0 }
    }

    pub fn set(&mut self, on: bool) -> HALResult<()> {
        hal_call!(HAL_SetRelay(self.0, on as i32))
    }

    pub fn get(&mut self) -> HALResult<bool> {
        Ok(hal_call!(HAL_GetRelay(self.0))? != 0)
    }
}

impl Drop for Relay {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeRelayPort(self.0);
        }
    }
}
