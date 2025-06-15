use wpihal_sys::{
    HAL_CANDeviceType, HAL_CANFlags, HAL_CANHandle, HAL_CANManufacturer, HAL_CANMessage,
    HAL_CANReceiveMessage, HAL_CleanCAN, HAL_InitializeCAN, HAL_ReadCANPacketLatest,
    HAL_ReadCANPacketNew, HAL_ReadCANPacketTimeout, HAL_StopCANPacketRepeating, HAL_WriteCANPacket,
    HAL_WriteCANPacketRepeating, HAL_WriteCANRTRFrame,
};

use crate::{error::HALResult, hal_call};

pub type CANManufacturer = HAL_CANManufacturer;
pub type CANDeviceType = HAL_CANDeviceType;
pub type CANHandle = HAL_CANHandle;

/// Used in conjunction with the high-level CAN API.
/// Notably, this only includes the 10-bit API id.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CANMessage {
    flags: i32,
    length: u8,
    data: [u8; 64],
    api_id: u16,
    timestamp: Option<u64>,
}

impl CANMessage {
    pub fn new(data: &[u8], api_id: u16, brs: bool, fd: bool) -> Self {
        let length = data.len().min(64);
        let mut data_buf = [0u8; 64];
        data_buf.copy_from_slice(&data[..length]);
        let mut flags = 0;
        if brs {
            flags |= HAL_CANFlags::HAL_CAN_FD_BITRATESWITCH as i32;
        }
        if fd {
            flags |= HAL_CANFlags::HAL_CAN_FD_DATALENGTH as i32;
        }

        Self {
            flags,
            length: length as u8,
            data: data_buf,
            api_id,
            timestamp: None,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..self.length as usize]
    }

    pub fn length(&self) -> u8 {
        self.length
    }

    pub fn api_id(&self) -> u16 {
        self.api_id
    }

    pub fn timestamp(&self) -> Option<u64> {
        self.timestamp
    }

    pub fn brs(&self) -> bool {
        self.flags | HAL_CANFlags::HAL_CAN_FD_BITRATESWITCH as i32 != 0
    }

    pub fn fd(&self) -> bool {
        self.flags | HAL_CANFlags::HAL_CAN_FD_DATALENGTH as i32 != 0
    }

    pub const fn as_hal_canmessage(&self) -> HAL_CANMessage {
        HAL_CANMessage {
            flags: self.flags,
            dataSize: self.length,
            data: self.data,
        }
    }

    pub const fn from_hal_recv_message(api_id: u16, value: &HAL_CANReceiveMessage) -> Self {
        Self {
            flags: value.message.flags,
            length: value.message.dataSize,
            data: value.message.data,
            api_id,
            timestamp: Some(value.timeStamp),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CAN(CANHandle);

impl CAN {
    pub fn initialize(
        bus_id: i32,
        manufacturer: CANManufacturer,
        device_id: u8,
        device_type: CANDeviceType,
    ) -> HALResult<CAN> {
        Ok(Self(hal_call!(HAL_InitializeCAN(
            bus_id,
            manufacturer,
            device_id as i32,
            device_type
        ))?))
    }

    pub fn write_packet(&self, message: &CANMessage) -> HALResult<()> {
        hal_call!(HAL_WriteCANPacket(
            self.0,
            message.api_id() as i32,
            &message.as_hal_canmessage() as *const HAL_CANMessage,
        ))
    }

    pub fn write_repeating(&self, message: &CANMessage, repeat_ms: i32) -> HALResult<()> {
        hal_call!(HAL_WriteCANPacketRepeating(
            self.0,
            message.api_id as i32,
            &message.as_hal_canmessage() as *const HAL_CANMessage,
            repeat_ms
        ))
    }

    pub fn write_rtr(&self, message: &CANMessage) -> HALResult<()> {
        hal_call!(HAL_WriteCANRTRFrame(
            self.0,
            message.api_id as i32,
            &message.as_hal_canmessage() as *const HAL_CANMessage
        ))
    }

    pub fn stop_repeating(&self, api_id: u16) -> HALResult<()> {
        hal_call!(HAL_StopCANPacketRepeating(self.0, api_id as i32))
    }

    pub fn read_can_packet_new(&self, api_id: u16) -> HALResult<CANMessage> {
        let mut recv = HAL_CANReceiveMessage::default();

        hal_call!(HAL_ReadCANPacketNew(self.0, api_id as i32, &mut recv,))?;

        Ok(CANMessage::from_hal_recv_message(api_id, &recv))
    }

    pub fn read_can_packet_latest(&self, api_id: u16) -> HALResult<CANMessage> {
        let mut recv = HAL_CANReceiveMessage::default();

        hal_call!(HAL_ReadCANPacketLatest(self.0, api_id as i32, &mut recv,))?;

        Ok(CANMessage::from_hal_recv_message(api_id, &recv))
    }

    pub fn read_can_packet_timeout(&self, api_id: u16, timeout_ms: i32) -> HALResult<CANMessage> {
        let mut recv = HAL_CANReceiveMessage::default();

        hal_call!(HAL_ReadCANPacketTimeout(
            self.0,
            api_id as i32,
            &mut recv,
            timeout_ms,
        ))?;

        Ok(CANMessage::from_hal_recv_message(api_id, &recv))
    }
}

impl Drop for CAN {
    fn drop(&mut self) {
        unsafe {
            HAL_CleanCAN(self.0);
        }
    }
}
