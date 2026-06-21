use wpihal_sys::{
    HAL_CANMessage, HAL_CANReceiveMessage, HAL_CANStreamHandle, HAL_CANStreamMessage,
};

use crate::{
    error::{HALError, result_as_i32},
    halsim::callbacks::{CallbackHandle, callback_trait, register_callback},
};

macro_rules! can_register_callback {
    ($name:ident) => {
        paste::paste! {
            can_register_callback!($name, [<$name:snake>]);
        }
    };
    ($name:ident, $snake:ident) => {
        paste::paste! {
            pub fn [<register_ $snake _callback>]<C: [<Can $name Callback>]>(callback: C) -> CallbackHandle<C> {
                register_callback!(
                    [<HALSIM_RegisterCan $name Callback>],
                    [<HALSIM_CancelCan $name Callback>],
                    [<can_ $name:snake _callback_trampoline>]::<C>,
                    callback,
                )
            }
        }

    };
}

callback_trait!(
    CanSendMessageCallback(
        bus_id: i32,
        message_id: u32,
        message: &HAL_CANMessage,
        period: i32,
    ) -> Result<(), HALError>,
    |callback,
     name,
     bus_id: i32,
     message_id: u32,
     message: *const HAL_CANMessage,
     period_ms: i32,
     status: *mut i32| {{
        let Some(message_ref) = message.as_ref() else {
            return;
        };

        *status = result_as_i32(callback.callback(
            name,
            bus_id,
            message_id,
            message_ref,
            period_ms,
        ));
    }}
);

callback_trait!(
    CanReceiveMessageCallback(
        bus_id: i32,
        message_id: u32,
    ) -> Result<HAL_CANReceiveMessage, HALError>,
    |callback,
     name,
     bus_id: i32,
     message_id: u32,
     message: *mut HAL_CANReceiveMessage,
     status: *mut i32| {{
        let Some(message) = message.as_mut() else {
            return;
        };
        let Some(status) = core::ptr::NonNull::new(status) else {
            return;
        };

        let result = callback.callback(
            name,
            bus_id,
            message_id,
        );

        match result {
            Ok(msg) => {
                status.write(0);
                *message = msg;
            }
            Err(e) => {
                status.write(e.0);
            }
        }
    }}
);

callback_trait!(
    CanOpenStreamCallback(
        bus_id: i32,
        message_id: u32,
        message_id_mask: u32,
        max_messages: u32,
    ) -> Result<HAL_CANStreamHandle, HALError>,
    |callback,
     name,
     stream_handle: *mut HAL_CANStreamHandle,
     bus_id: i32,
     message_id: u32,
     message_id_mask: u32,
     max_messages: u32,
     status: *mut i32| {{
        let Some(status) = core::ptr::NonNull::new(status) else {
            return;
        };

        let result = callback.callback(
            name,
            bus_id,
            message_id,
            message_id_mask,
            max_messages
        );

        match result {
            Ok(handle) => {
                status.write(0);
                *stream_handle = handle;
            }
            Err(e) => {
                status.write(e.0);
            }
        }
    }}
);

callback_trait!(
    CanCloseStreamCallback(handle: HAL_CANStreamHandle),
    |callback, name, handle: HAL_CANStreamHandle| { callback.callback(name, handle) }
);
callback_trait!(
    CanReadStreamCallback(
        handle: HAL_CANStreamHandle,
        messages: &mut [HAL_CANStreamMessage],
    ) -> Result<u32, HALError>,
    |callback,
     name,
     stream_handle: HAL_CANStreamHandle,
     messages: *mut HAL_CANStreamMessage,
     messages_to_read: u32,
     messages_read: *mut u32,
     status: *mut i32| {{
        let Some(messages) = core::ptr::NonNull::new(messages) else {
            return;
        };
        let Some(messages_read) = core::ptr::NonNull::new(messages_read) else {
            return;
        };
        let Some(status) = core::ptr::NonNull::new(status) else {
            return;
        };

        let result = callback.callback(
            name,
            stream_handle,
            core::slice::from_raw_parts_mut(messages.as_ptr(), messages_to_read as usize),
        );

        match result {
            Ok(read) => {
                messages_read.write(read);
                status.write(0);
            }
            Err(e) => {
                status.write(e.0);
            }
        }
    }}
);

callback_trait!(
    CanGetCANStatusCallback(bus_id: i32) -> Result<crate::can::CANStatus, HALError>,
    |callback,
     name,
     bus_id: i32,
     percent_bus_utilization: *mut f32,
     bus_off_count: *mut u32,
     tx_full_count: *mut u32,
     receive_error_count: *mut u32,
     transmit_error_count: *mut u32,
     status: *mut i32| {{
        let Some(status) = core::ptr::NonNull::new(status) else {
            return;
        };
        match callback.callback(name, bus_id) {
            Ok(can_status) => {
                let _ = core::ptr::NonNull::new(percent_bus_utilization).map(|p| p.write(can_status.percent_bus_utilization));
                let _ = core::ptr::NonNull::new(bus_off_count).map(|p| p.write(can_status.bus_off_count));
                let _ = core::ptr::NonNull::new(tx_full_count).map(|p| p.write(can_status.tx_full_count));
                let _ = core::ptr::NonNull::new(receive_error_count).map(|p| p.write(can_status.receive_error_count));
                let _ = core::ptr::NonNull::new(transmit_error_count).map(|p| p.write(can_status.transmit_error_count));

            }
            Err(e) => {
                status.write(e.0);
            }
        }
    }}
);

// Unlike most other callbacks, these callbacks do control how wpilib
// accesses things like CAN in simulation.
// this lets you hook wpilib's CAN backend to Whatever You Want;
// which is mildly useful in the roborio era but perhaps less useful in a socketcan world.

// register_send_message_callback
can_register_callback!(SendMessage);
// register_receive_message_callback
can_register_callback!(ReceiveMessage);
// register_open_stream_callback
can_register_callback!(OpenStream);
// register_close_stream_callback
can_register_callback!(CloseStream);
// register_read_stream_callback
can_register_callback!(ReadStream);
// register_get_can_status_callback
can_register_callback!(GetCANStatus, get_can_status);
