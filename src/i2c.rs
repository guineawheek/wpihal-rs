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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct I2CError(pub i32);
impl I2CError {
    pub fn from_code(code: i32) -> Result<(), Self> {
        if code < 0 {
            Err(Self(code))
        } else {
            Ok(())
        }
    }
}

impl core::fmt::Display for I2CError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl embedded_hal::i2c::Error for I2CError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        embedded_hal::i2c::ErrorKind::Other
    }
}

impl embedded_hal::i2c::ErrorType for I2C {
    type Error = I2CError;
}

impl embedded_hal::i2c::I2c for I2C {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let mut prev_op: Option<&[u8]> = None;
        for op in operations {
            match (prev_op, op) {
                (None, embedded_hal::i2c::Operation::Read(rb)) => {
                    I2CError::from_code(self.read(address as i32, rb))?;
                }
                (None, embedded_hal::i2c::Operation::Write(wb)) => {
                    prev_op = Some(wb);
                }
                (Some(wb), embedded_hal::i2c::Operation::Read(rb)) => {
                    I2CError::from_code(self.transaction(address as i32, wb, rb))?;
                    prev_op = None;
                }
                (Some(wb0), embedded_hal::i2c::Operation::Write(wb1)) => {
                    I2CError::from_code(self.write(address as i32, wb0))?;
                    prev_op = Some(wb1);
                }
            }
        }
        if let Some(wb) = prev_op {
            I2CError::from_code(self.write(address as i32, wb))?;
        }
        Ok(())
    }
}