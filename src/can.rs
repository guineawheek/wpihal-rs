use wpihal_sys::{
    HAL_CAN_CloseStreamSession, HAL_CAN_GetCANStatus, HAL_CAN_IS_FRAME_11BIT,
    HAL_CAN_IS_FRAME_REMOTE, HAL_CAN_OpenStreamSession, HAL_CAN_ReadStreamSession,
    HAL_CAN_ReceiveMessage, HAL_CAN_SEND_PERIOD_NO_REPEAT, HAL_CAN_SEND_PERIOD_STOP_REPEATING,
    HAL_CAN_SendMessage, HAL_CAN_TIMEOUT, HAL_CANMessage, HAL_CANReceiveMessage,
    HAL_CANStreamMessage, HAL_WARN_CANSessionMux_SocketBufferFull,
    HAL_WARN_CANSessionMux_TxQueueFull,
};

use crate::{error::HALResult, hal_call};

pub type CANStreamMessage = HAL_CANStreamMessage;
/// Send the message but do not repeat it periodically.
pub const SEND_PERIOD_NO_REPEAT: i32 = HAL_CAN_SEND_PERIOD_NO_REPEAT as i32;
/// Stop repeating the message.
pub const SEND_PERIOD_STOP_REPEATING: i32 = HAL_CAN_SEND_PERIOD_STOP_REPEATING;
/// This bit is set in the [`CANStreamMessage::messageID`] field if the frame is a remote (RTR) frame.
pub const CAN_IS_FRAME_REMOTE: u32 = HAL_CAN_IS_FRAME_REMOTE;
/// This bit is set in the [`CANStreamMessage::messageID`] field if the frame has an 11-bit CAN 2.0a arbitration ID.
pub const CAN_IS_FRAME_11BIT: u32 = HAL_CAN_IS_FRAME_11BIT;
/// Happens if the bus is disconnected. [`send_message`] again later.
pub const ERR_SOCKET_BUFFER_FULL: i32 = HAL_WARN_CANSessionMux_SocketBufferFull as i32;
/// Happens if the TX queue is full. [`send_message`] again later.
pub const ERR_TX_QUEUE_FULL: i32 = HAL_WARN_CANSessionMux_TxQueueFull as i32;
/// CAN receive has timed out
pub const ERR_CAN_TIMEOUT: i32 = HAL_CAN_TIMEOUT;

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
    pub handle: i32,
    pub capacity: u32,
    pub bus_id: i32,
}

impl StreamSession {
    pub fn open(
        bus_id: i32,
        message_id: u32,
        message_id_mask: u32,
        max_messages: u32,
    ) -> HALResult<StreamSession> {
        let handle = hal_call!(HAL_CAN_OpenStreamSession(
            bus_id,
            message_id,
            message_id_mask,
            max_messages
        ))?;
        Ok(StreamSession {
            handle: handle,
            capacity: max_messages,
            bus_id,
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

impl Drop for StreamSession {
    fn drop(&mut self) {
        unsafe {
            HAL_CAN_CloseStreamSession(self.handle);
        }
    }
}

pub fn send_message(
    bus_id: i32,
    message_id: u32,
    msg: &HAL_CANMessage,
    period_ms: i32,
) -> HALResult<()> {
    hal_call!(HAL_CAN_SendMessage(
        bus_id,
        message_id,
        msg as *const HAL_CANMessage,
        period_ms
    ))
}

pub fn receive_message(bus_id: i32, message_id: u32) -> HALResult<HAL_CANReceiveMessage> {
    let mut msg = HAL_CANReceiveMessage::default();
    hal_call!(HAL_CAN_ReceiveMessage(bus_id, message_id, &mut msg))?;
    Ok(msg)
}

pub fn get_can_status(bus_id: i32) -> HALResult<CANStatus> {
    let mut can_status = CANStatus {
        percent_bus_utilization: 0_f32,
        bus_off_count: 0,
        tx_full_count: 0,
        receive_error_count: 0,
        transmit_error_count: 0,
    };
    hal_call!(HAL_CAN_GetCANStatus(
        bus_id,
        &mut can_status.percent_bus_utilization,
        &mut can_status.bus_off_count,
        &mut can_status.tx_full_count,
        &mut can_status.receive_error_count,
        &mut can_status.transmit_error_count
    ))?;
    Ok(can_status)
}
