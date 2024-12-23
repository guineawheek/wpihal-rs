use std::{ffi::{c_void, CStr}, time::Duration};

use error::{HALError, HALResult};
use wpihal_sys::{HAL_ExpandFPGATime, HAL_GetBrownedOut, HAL_GetComments, HAL_GetCommsDisableCount, HAL_GetFPGAButton, HAL_GetFPGATime, HAL_GetFPGAVersion, HAL_GetLastError, HAL_GetPort, HAL_GetPortWithModule, HAL_GetRSLState, HAL_GetRuntimeType, HAL_GetSerialNumber, HAL_GetSystemActive, HAL_GetSystemClockTicksPerMicrosecond, HAL_GetSystemTimeValid, HAL_GetTeamNumber, HAL_Initialize, HAL_PortHandle, HAL_RuntimeType, HAL_Shutdown, HAL_SimPeriodicAfter, HAL_SimPeriodicBefore, WPI_String};
use wpistring::AllocatedWPIString;

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
/// ctre pcm
pub mod ctre_pcm;
/// digital i/o
pub mod dio;
/// DMA
pub mod dma;
/// driver station data
pub mod driver_station;
/// duty cycle input
pub mod duty_cycle;
/// quadrature encoders
pub mod encoder;
/// HAL extensions
pub mod extensions;
/// usage reporting
pub mod usage_reporting;
/// I2C transactions (may freeze your rio)
pub mod i2c;
/// interrupts
pub mod interrupts;
/// Radio leds
pub mod leds;
/// main loop management
pub mod main_loop;
/// notifiers
pub mod notifier;
/// ports
pub mod ports;
/// power
pub mod power;

/*
dma
errors
extensions
frcusagereporting
halbase
*/

/// Error handling
pub mod error;
pub mod wpistring;

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

pub fn get_system_clock_ticks_per_microsecond() -> i32 {
    unsafe { HAL_GetSystemClockTicksPerMicrosecond() }
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


pub fn get_fpga_version() -> HALResult<i32> {
    hal_call!(HAL_GetFPGAVersion())
}

pub fn get_serial_number() -> AllocatedWPIString {
    let mut s: WPI_String = Default::default();
    unsafe { HAL_GetSerialNumber(&mut s); }
    AllocatedWPIString::new(s)
}

pub fn get_comments() -> AllocatedWPIString {
    let mut s: WPI_String = Default::default();
    unsafe { HAL_GetComments(&mut s); }
    AllocatedWPIString::new(s)
}

pub fn get_team_number() -> i32 {
    unsafe { HAL_GetTeamNumber() }
}

pub fn get_runtime_type() -> HAL_RuntimeType {
    unsafe { HAL_GetRuntimeType() }
}

pub fn get_fpga_button() -> HALResult<bool> {
    Ok(hal_call!(HAL_GetFPGAButton())? != 0)
}

pub fn get_system_active() -> HALResult<bool> {
    Ok(hal_call!(HAL_GetSystemActive())? != 0)
}

pub fn get_browned_out() -> HALResult<bool> {
    Ok(hal_call!(HAL_GetBrownedOut())? != 0)
}

pub fn get_comms_disable_count() -> HALResult<i32> {
    hal_call!(HAL_GetCommsDisableCount())
}

pub fn get_port(channel: i32) -> HAL_PortHandle {
    unsafe { HAL_GetPort(channel) }
}

pub fn get_port_with_module(module: i32, channel: i32) -> HAL_PortHandle {
    unsafe { HAL_GetPortWithModule(module, channel) }
}

pub fn get_fpga_time() -> HALResult<u64> {
    hal_call!(HAL_GetFPGATime())
}

pub fn get_fpga_duration() -> HALResult<Duration> {
    Ok(Duration::from_micros(get_fpga_time()?))
}

pub fn expand_fpga_time(lower: u32) -> HALResult<u64> {
    hal_call!(HAL_ExpandFPGATime(lower))
}

pub fn get_rsl_state() -> HALResult<bool> {
    Ok(hal_call!(HAL_GetRSLState())? != 0)
}

pub fn get_system_time_valid() -> HALResult<bool> {
    Ok(hal_call!(HAL_GetSystemTimeValid())? != 0)
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
    unsafe { HAL_Shutdown(); }
}

pub fn sim_periodic_before() {
    unsafe { HAL_SimPeriodicBefore(); }
}

pub fn sim_periodic_after() {
    unsafe { HAL_SimPeriodicAfter(); }
}

#[allow(non_snake_case)]
unsafe extern "C" fn HAL_rust_wpihal_linkage_trampoline(param: *mut c_void) {
    unsafe {
        let f: fn() = core::mem::transmute(param);
        f()
    }
}