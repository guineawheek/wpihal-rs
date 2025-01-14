
/*

this doesn't use VISA to query the usb serial ports
but honestly if you're using rust, just use a 3rd party crate

*/

use std::ffi::{c_char, CStr};

use wpihal_sys::{HAL_CloseSerial, HAL_DisableSerialTermination, HAL_EnableSerialTermination, HAL_FlushSerial, HAL_GetSerialBytesReceived, HAL_GetSerialFD, HAL_InitializeSerialPort, HAL_InitializeSerialPortDirect, HAL_ReadSerial, HAL_SerialPort, HAL_SerialPortHandle, HAL_SetSerialBaudRate, HAL_SetSerialDataBits, HAL_SetSerialFlowControl, HAL_SetSerialParity, HAL_SetSerialReadBufferSize, HAL_SetSerialStopBits, HAL_SetSerialTimeout, HAL_SetSerialWriteBufferSize, HAL_SetSerialWriteMode, HAL_WriteSerial};

use crate::{error::HALResult, hal_call};

pub type SerialPortIndex = HAL_SerialPort;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum WriteMode {
    FlushOnAccess = 1,
    FlushWhenFull = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FlowControl {
    None = 0,
    XonXoff = 1,
    RtsCts = 2,
    DtrDsr = 3,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SerialPort(HAL_SerialPortHandle);

impl SerialPort {
    pub fn initialize(port: SerialPortIndex) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeSerialPort(port))?))
    }

    pub fn initialize_direct(port: SerialPortIndex, name: &CStr) -> HALResult<Self> {
        Ok(Self(hal_call!(HAL_InitializeSerialPortDirect(port, name.as_ptr()))?))
    }

    pub fn get_fd(&self) -> HALResult<std::os::raw::c_int> {
        hal_call!(HAL_GetSerialFD(self.0))
    }

    pub fn set_baud_rate(&mut self, baud: i32) -> HALResult<()> {
        hal_call!(HAL_SetSerialBaudRate(self.0, baud))
    }

    pub fn set_data_bits(&mut self, bits: i32) -> HALResult<()> {
        hal_call!(HAL_SetSerialDataBits(self.0, bits))
    }

    pub fn set_parity(&mut self, parity: i32) -> HALResult<()> {
        hal_call!(HAL_SetSerialParity(self.0, parity))
    }

    pub fn set_stop_bits(&mut self, stop_bits: i32) -> HALResult<()> {
        hal_call!(HAL_SetSerialStopBits(self.0, stop_bits))
    }

    pub fn set_write_mode(&mut self, mode: WriteMode) -> HALResult<()> {
        hal_call!(HAL_SetSerialWriteMode(self.0, mode as i32))
    }

    pub fn set_flow_control(&mut self, flow: FlowControl) -> HALResult<()> {
        hal_call!(HAL_SetSerialFlowControl(self.0, flow as i32))
    }

    pub fn set_timeout(&mut self, timeout: f64) -> HALResult<()> {
        hal_call!(HAL_SetSerialTimeout(self.0, timeout))
    }

    pub fn enable_termination(&mut self, terminator: u8) -> HALResult<()> {
        hal_call!(HAL_EnableSerialTermination(self.0, terminator as c_char))
    }

    pub fn disable_termination(&mut self) -> HALResult<()> {
        hal_call!(HAL_DisableSerialTermination(self.0))
    }

    pub fn set_read_buffer_size(&mut self, size: i32) -> HALResult<()> {
        hal_call!(HAL_SetSerialReadBufferSize(self.0, size))
    }

    pub fn set_write_buffer_size(&mut self, size: i32) -> HALResult<()> {
        hal_call!(HAL_SetSerialWriteBufferSize(self.0, size))
    }

    pub fn get_bytes_received(&self) -> HALResult<i32> {
        hal_call!(HAL_GetSerialBytesReceived(self.0))
    }

    pub fn read(&self, buffer: &mut [u8]) -> HALResult<usize> {
        Ok(hal_call!(HAL_ReadSerial(self.0, buffer.as_mut_ptr() as *mut c_char, buffer.len() as i32))? as usize)
    }

    pub fn write(&self, buffer: &[u8]) -> HALResult<usize> {
        Ok(hal_call!(HAL_WriteSerial(self.0, buffer.as_ptr() as *mut c_char, buffer.len() as i32))? as usize)
    }

    pub fn flush(&self) -> HALResult<()> {
        hal_call!(HAL_FlushSerial(self.0))
    }
}

impl Drop for SerialPort {
    fn drop(&mut self) {
        unsafe { HAL_CloseSerial(self.0); }
    }
}