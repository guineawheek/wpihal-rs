use wpihal_sys::hal::HAL_GetSystemClockTicksPerMicrosecond;

/// this is the higher level package
/// i guess


/// roboRIO accelerometer functions
pub mod accelerometer;
/// addressable ws2812 leds
pub mod addressable_led;
/// analog accumulator
pub mod analog_accumulator;
/// analog gyro (my condolences)
pub mod analog_gyro;
/// analog input
pub mod analog_input;
/// analog output
pub mod analog_output;
/// analog trigger
pub mod analog_trigger;
/// can bus
pub mod can;
/// can api
pub mod can_api;
/// counter
pub mod counter;

/// Error handling
pub mod error;


/// Wraps a C/C++ HAL function call that looks like `T foo(arg1, arg2, arg3, ... , int32_t* status)`
/// and turns that status into a `HALResult<T>`, with a non-zero status code returning in
/// the `Err` variant.
#[macro_export]
macro_rules! hal_call {
    ($function:ident($($arg:expr),* $(,)?)) => {{
        let mut status = 0;
        let result = unsafe { $function($(
            $arg,
        )* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err(crate::error::HALError::from(status)) }
    }};
    ($namespace:path, $function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $namespace::$function($(
            $arg,
        )* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err(crate::error::HALError::from(status)) }
    }};
}

pub fn get_system_clock_ticker_per_microsecond() -> i32 {
    unsafe { HAL_GetSystemClockTicksPerMicrosecond() }
}