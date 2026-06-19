use std::{
    ffi::{CStr, c_void},
    time::Duration,
};

use error::{HALError, HALResult};
use wpihal_sys::{
    HAL_Bool, HAL_GetBrownedOut, HAL_GetComments, HAL_GetCommsDisableCount, HAL_GetLastError,
    HAL_GetMonotonicTime, HAL_GetRSLState, HAL_GetRuntimeType, HAL_GetSerialNumber,
    HAL_GetSystemActive, HAL_GetSystemTimeValid, HAL_GetTeamNumber, HAL_Initialize,
    HAL_RuntimeType, HAL_Shutdown, HAL_SimPeriodicAfter, HAL_SimPeriodicBefore,
};
use wpiutil::wpistring::WPIString;

/// this is the higher level package
/// i guess

/// addressable ws2812 leds
pub mod addressable_led;
/// alerts
pub mod alert;
/// analog input
pub mod analog_input;
/// can bus
pub mod can;
/// can api
pub mod can_api;
/// Control words
pub mod control_word;
/// counter
pub mod counter;
/// ctre pcm
pub mod ctre_pcm;
/// digital i/o
pub mod dio;
/// driver station data
pub mod driver_station;
/// duty cycle input
pub mod duty_cycle;
/// quadrature encoders
pub mod encoder;
/// HAL extensions
pub mod extensions;
/// halsim hooks
pub mod halsim;
/// I2C transactions
pub mod i2c;
/// Integrated IMU
pub mod imu;
/// main loop management
pub mod main_loop;
/// notifiers
pub mod notifier;
/// Op modes
pub mod op_mode;
/// ports
pub mod ports;
/// power
pub mod power;
/// power distribution
pub mod power_distribution;
/// PWM output
pub mod pwm;
/// rev pneumatic hub
pub mod rev_ph;
/// serial ports
pub mod serial_port;
/// simdevice
pub mod sim_device;
/// Threads
pub mod threads;
/// usage reporting
pub mod usage_reporting;
/// HALValue
pub mod value;

/*
dma
errors
extensions
frcusagereporting
halbase
*/

/// Error handling
pub mod error;

/// Trait for a struct that wraps a handle value
pub trait Handle<T> {
    /// Fetches the raw handle.
    /// Unsafe because usage of the raw handle can violate ownership.
    unsafe fn raw_handle(&self) -> T;
    /// Creates a new instance of the struct from a raw handle.
    /// Unsafe because usage of the raw handle can violate ownership --
    /// in particular, dropping the new object may cause double-frees.
    unsafe fn from_raw_handle(handle: T) -> Self;
}

pub(crate) const fn hal_bool(b: HAL_Bool) -> bool {
    b != 0
}

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
        if status == 0 { Ok(result) } else { Err($crate::error::HALError::from(status)) }
    }};
    ($namespace:path, $function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $namespace::$function($(
            $arg,
        )* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err($crate::error::HALError::from(status)) }
    }};
}

/// Wraps a C/C++ HAL function call of the form `HAL_Status foo(arg1, ..., ret: *mut T)`
/// and turns that into a `HALResult<T>`, with a non-zero status code returning in the `Err` variant.
#[macro_export]
macro_rules! hal_retcall {
    ($function:ident($($prev_arg:expr),* $(,)?) -> $out_ty:ty) => {{
        let mut out = core::mem::MaybeUninit::<$out_ty>::zeroed();
        #[allow(unused_unsafe)]
        let status = unsafe {
            $function(
                $($prev_arg,)*
                out.as_mut_ptr(),
            )
        };
        if status == 0 {
            #[allow(unused_unsafe)]
            unsafe {
                Ok(out.assume_init())
            }
        } else {
            Err($crate::error::HALError::from(status))
        }
    }};
    ($function:ident($($arg:expr),* $(,)?)) => {{
        let status = unsafe { $function($($arg),*) };
        if status == 0 {
            Ok(())
        } else {
            Err($crate::error::HALError::from(status))
        }
    }};
}

/// unlike the actual hal call this allocates.
/// mostly to prevent clobbering later on.
pub fn get_last_error() -> (HALError, String) {
    let mut status = wpihal_sys::HAL_USE_LAST_ERROR;
    unsafe {
        let cs = CStr::from_ptr(HAL_GetLastError(&mut status));
        (HALError(status), cs.to_string_lossy().to_string())
    }
}

pub fn get_serial_number() -> WPIString {
    let mut s: wpiutil::wpistring::RawWPIString = Default::default();
    unsafe {
        HAL_GetSerialNumber(&mut s);
        WPIString::from_raw(s)
    }
}

pub fn get_comments() -> WPIString {
    let mut s: wpiutil::wpistring::RawWPIString = Default::default();
    unsafe {
        HAL_GetComments(&mut s);
        WPIString::from_raw(s)
    }
}

pub fn get_team_number() -> i32 {
    unsafe { HAL_GetTeamNumber() }
}

pub fn get_runtime_type() -> HAL_RuntimeType {
    unsafe { HAL_GetRuntimeType() }
}

pub fn get_system_active() -> HALResult<bool> {
    hal_call!(HAL_GetSystemActive()).map(hal_bool)
}

pub fn get_browned_out() -> HALResult<bool> {
    hal_call!(HAL_GetBrownedOut()).map(hal_bool)
}

pub fn get_comms_disable_count() -> HALResult<i32> {
    hal_call!(HAL_GetCommsDisableCount())
}

pub fn get_monotonic_time() -> u64 {
    unsafe { HAL_GetMonotonicTime() }
}

pub fn get_monotonic_duration() -> Duration {
    Duration::from_micros(get_monotonic_time())
}

pub fn get_rsl_state() -> HALResult<bool> {
    hal_call!(HAL_GetRSLState()).map(hal_bool)
}

pub fn get_system_time_valid() -> HALResult<bool> {
    hal_call!(HAL_GetSystemTimeValid()).map(hal_bool)
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HALInitializationMode {
    /// Try to kill an existing HAL from another program, if not successful, error
    TryKillExisting = 0,
    /// Force kill a HAL from another program.
    ForceKillExisting = 1,
    /// Just warn if another HAL exists and cannot be killed. Will likely result in undefined behavior.
    WarnIfExisting = 2,
}

pub fn initialize(timeout: i32, mode: HALInitializationMode) -> bool {
    unsafe { HAL_Initialize(timeout, mode as i32) != 0 }
}

pub fn initialize_common() -> bool {
    unsafe { HAL_Initialize(500, 0) != 0 }
}

pub fn shutdown() {
    unsafe {
        HAL_Shutdown();
    }
}

pub fn sim_periodic_before() {
    unsafe {
        HAL_SimPeriodicBefore();
    }
}

pub fn sim_periodic_after() {
    unsafe {
        HAL_SimPeriodicAfter();
    }
}

unsafe extern "C" fn param_as_fn_trampoline(param: *mut c_void) {
    unsafe {
        let f: fn() = core::mem::transmute(param);
        f()
    }
}
