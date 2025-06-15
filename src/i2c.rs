use wpihal_sys::{
    HAL_CloseI2C, HAL_I2CPort, HAL_InitializeI2C, HAL_ReadI2C, HAL_TransactionI2C, HAL_WriteI2C,
};

use crate::{error::HALResult, hal_call};

pub type I2CPort = HAL_I2CPort;

#[derive(Debug, PartialEq, Eq)]
pub struct I2C(I2CPort);

impl I2C {
    pub fn initialize(port: I2CPort) -> HALResult<Self> {
        hal_call!(HAL_InitializeI2C(port))?;
        Ok(Self(port))
    }

    pub fn transaction(&mut self, addr: i32, tx: &[u8], rx: &mut [u8]) -> i32 {
        unsafe {
            HAL_TransactionI2C(
                self.0,
                addr,
                tx.as_ptr(),
                tx.len() as i32,
                rx.as_mut_ptr(),
                rx.len() as i32,
            )
        }
    }

    pub fn write(&mut self, addr: i32, tx: &[u8]) -> i32 {
        unsafe { HAL_WriteI2C(self.0, addr, tx.as_ptr(), tx.len() as i32) }
    }

    pub fn read(&mut self, addr: i32, rx: &mut [u8]) -> i32 {
        unsafe { HAL_ReadI2C(self.0, addr, rx.as_mut_ptr(), rx.len() as i32) }
    }

    pub fn port(&self) -> I2CPort {
        self.0
    }
}

impl Drop for I2C {
    fn drop(&mut self) {
        unsafe {
            HAL_CloseI2C(self.0);
        }
    }
}
