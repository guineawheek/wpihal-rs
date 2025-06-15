use wpihal_sys::{
    HAL_CAN_CloseStreamSession, HAL_CAN_GetCANStatus, HAL_CAN_OpenStreamSession,
    HAL_CAN_ReadStreamSession, HAL_CAN_ReceiveMessage, HAL_CAN_SEND_PERIOD_NO_REPEAT,
    HAL_CAN_SEND_PERIOD_STOP_REPEATING, HAL_CAN_SendMessage, HAL_CANStreamMessage,
};

use crate::{error::HALResult, hal_call};

pub type CANStreamMessage = HAL_CANStreamMessage;
pub const SEND_PERIOD_NO_REPEAT: i32 = HAL_CAN_SEND_PERIOD_NO_REPEAT as i32;
pub const SEND_PERIOD_STOP_REPEATING: i32 = HAL_CAN_SEND_PERIOD_STOP_REPEATING;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CANStatus {
    pub percent_bus_utilization: f32,
    pub bus_off_count: u32,
    pub tx_full_count: u32,
    pub receive_error_count: u32,
    pub transmit_error_count: u32,
}

#[derive(PartialEq, Eq, Debug)]
pub struct StreamSession {
    handle: u32,
    capacity: u32,
}

impl StreamSession {
    pub fn open(
        message_id: u32,
        message_id_mask: u32,
        max_messages: u32,
    ) -> HALResult<StreamSession> {
        let mut session_handle = 0_u32;
        hal_call!(HAL_CAN_OpenStreamSession(
            &mut session_handle,
            message_id,
            message_id_mask,
            max_messages
        ))?;
        Ok(StreamSession {
            handle: session_handle,
            capacity: max_messages,
        })
    }

    pub fn read_into(&self, messages: &mut [CANStreamMessage]) -> HALResult<usize> {
        let mut messages_read = 0_u32;
        let max_msg = messages.len().min(self.capacity as usize) as u32;
        hal_call!(HAL_CAN_ReadStreamSession(
            self.handle,
            messages.as_mut_ptr(),
            max_msg,
            &mut messages_read
        ))?;
        Ok(messages_read as usize)
    }
}

pub fn send_message(message_id: u32, data: &[u8], period_ms: i32) -> HALResult<()> {
    let size = data.len().min(8) as u8;
    hal_call!(HAL_CAN_SendMessage(
        message_id,
        data.as_ptr(),
        size,
        period_ms
    ))
}

pub fn receive_message(message_id_mask: u32) -> HALResult<CANStreamMessage> {
    let mut msg = CANStreamMessage {
        messageID: 0,
        timeStamp: 0,
        data: [0u8; 8],
        dataSize: 0,
    };
    hal_call!(HAL_CAN_ReceiveMessage(
        &mut msg.messageID,
        message_id_mask,
        msg.data.as_mut_ptr(),
        &mut msg.dataSize,
        &mut msg.timeStamp
    ))?;
    Ok(msg)
}

pub fn get_can_status() -> HALResult<CANStatus> {
    let mut can_status = CANStatus {
        percent_bus_utilization: 0_f32,
        bus_off_count: 0,
        tx_full_count: 0,
        receive_error_count: 0,
        transmit_error_count: 0,
    };
    hal_call!(HAL_CAN_GetCANStatus(
        &mut can_status.percent_bus_utilization,
        &mut can_status.bus_off_count,
        &mut can_status.tx_full_count,
        &mut can_status.receive_error_count,
        &mut can_status.transmit_error_count
    ))?;
    Ok(can_status)
}

impl Drop for StreamSession {
    fn drop(&mut self) {
        unsafe {
            HAL_CAN_CloseStreamSession(self.handle);
        }
    }
}
