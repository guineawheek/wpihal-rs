use wpihal_sys::{HAL_CANDeviceType, HAL_CANHandle, HAL_CANManufacturer, HAL_CleanCAN, HAL_GetCANPacketBaseTime, HAL_InitializeCAN, HAL_ReadCANPacketNew, HAL_ReadCANPacketTimeout, HAL_StopCANPacketRepeating, HAL_WriteCANPacket, HAL_WriteCANPacketRepeating, HAL_WriteCANRTRFrame};

use crate::{can::CANStreamMessage, error::HALResult, hal_call};

pub fn get_can_packet_base_time() -> u32 {
    unsafe { HAL_GetCANPacketBaseTime() }
}

pub type CANManufacturer = HAL_CANManufacturer;
pub type CANDeviceType = HAL_CANDeviceType;
pub type CANHandle = HAL_CANHandle;

/// Used in conjunction with the high-level CAN API.
/// Notably, this only includes the 10-bit API id.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CANPacket {
    data: [u8; 8],
    length: u8,
    api_id: u16,
    timestamp: u64
}

impl CANPacket {
    pub fn new(data: &[u8], api_id: u16) -> Self {
        Self::new_with_timestamp(data, api_id, 0)
    }
    pub fn new_with_timestamp(data: &[u8], api_id: u16, timestamp: u64) -> Self {
        let length = data.len().min(8);
        let mut data_buf =  [0u8; 8];
        data_buf.copy_from_slice(&data[..length]);
        Self {
            data: data_buf,
            length: length as u8,
            api_id: api_id,
            timestamp,
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

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

impl From<CANStreamMessage> for CANPacket {
    fn from(value: CANStreamMessage) -> Self {
        CANPacket { 
            data: value.data, 
            length: value.dataSize, 
            api_id: ((value.messageID >> 6) & 0x3ff) as u16,
            timestamp: value.timeStamp as u64,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CAN(CANHandle);

impl CAN {
    pub fn initialize(manufacturer: CANManufacturer, device_id: u8, device_type: CANDeviceType) -> HALResult<CAN> {
        Ok(Self(hal_call!(HAL_InitializeCAN(manufacturer, device_id as i32, device_type))?))
    }

    pub fn write_packet(&self, packet: &CANPacket) -> HALResult<()> {
        hal_call!(HAL_WriteCANPacket(self.0, packet.data.as_ptr(), packet.length as i32, packet.api_id as i32))
    }

    pub fn write_repeating(&self, packet: &CANPacket, repeat_ms: i32) -> HALResult<()> {
        hal_call!(HAL_WriteCANPacketRepeating(self.0, packet.data.as_ptr(), packet.length as i32, packet.api_id as i32, repeat_ms))
    }

    pub fn write_rtr(&self, length: u8, api_id: u16) -> HALResult<()> {
        hal_call!(HAL_WriteCANRTRFrame(self.0, length as i32, api_id as i32))
    }

    pub fn stop_repeating(&self, api_id: u16) -> HALResult<()> {
        hal_call!(HAL_StopCANPacketRepeating(self.0, api_id as i32))
    }

    pub fn read_can_packet_new(&self, api_id: u16) -> HALResult<CANPacket> {
        let mut data = [0u8; 8];
        let mut length = 0i32;
        let mut timestamp = 0u64;

        hal_call!(HAL_ReadCANPacketNew(self.0, api_id as i32, data.as_mut_ptr(), &mut length, &mut timestamp))?;

        Ok(CANPacket { data, length: length as u8, api_id, timestamp })
    }

    pub fn read_can_packet_latest(&self, api_id: u16) -> HALResult<CANPacket> {
        let mut data = [0u8; 8];
        let mut length = 0i32;
        let mut timestamp = 0u64;

        hal_call!(HAL_ReadCANPacketNew(self.0, api_id as i32, data.as_mut_ptr(), &mut length, &mut timestamp))?;

        Ok(CANPacket { data, length: length as u8, api_id, timestamp })
    }

    pub fn read_can_packet_timeout(&self, api_id: u16, timeout_ms: i32) -> HALResult<CANPacket> {
        let mut data = [0u8; 8];
        let mut length = 0i32;
        let mut timestamp = 0u64;

        hal_call!(HAL_ReadCANPacketTimeout(self.0, api_id as i32, data.as_mut_ptr(), &mut length, &mut timestamp, timeout_ms))?;

        Ok(CANPacket { data, length: length as u8, api_id, timestamp })
    }
}

impl Drop for CAN {
    fn drop(&mut self) {
        unsafe { HAL_CleanCAN(self.0); }
    }
}