use wpihal_sys::HAL_RuntimeType;

use crate::halsim::callbacks::{CallbackHandle, register_callback};

/// sets whether the codebase should think it's running in sim or not
pub fn set_runtime_type(runtime_type: HAL_RuntimeType) {
    unsafe {
        wpihal_sys::HALSIM_SetRuntimeType(runtime_type);
    }
}

/// waits for program start
pub fn wait_for_program_start() {
    unsafe {
        wpihal_sys::HALSIM_WaitForProgramStart();
    }
}

pub fn set_program_started(started: bool) {
    unsafe {
        wpihal_sys::HALSIM_SetProgramStarted(started as _);
    }
}
pub fn get_program_started() -> bool {
    unsafe { wpihal_sys::HALSIM_GetProgramStarted() != 0 }
}

pub fn set_program_state(control_word: crate::control_word::ControlWord) {
    unsafe {
        wpihal_sys::HALSIM_SetProgramState(control_word.into());
    }
}

pub fn get_program_state() -> crate::control_word::ControlWord {
    unsafe {
        let mut word = wpihal_sys::HAL_ControlWord::default();
        wpihal_sys::HALSIM_GetProgramState(&mut word);
        word.into()
    }
}

pub fn restart_timing() {
    unsafe {
        wpihal_sys::HALSIM_RestartTiming();
    }
}

pub fn pause_timing() {
    unsafe {
        wpihal_sys::HALSIM_PauseTiming();
    }
}

pub fn resume_timing() {
    unsafe {
        wpihal_sys::HALSIM_ResumeTiming();
    }
}

pub fn is_timing_paused() -> bool {
    unsafe { wpihal_sys::HALSIM_IsTimingPaused() != 0 }
}

pub fn step_timing(delta: u64) {
    unsafe {
        wpihal_sys::HALSIM_StepTiming(delta);
    }
}

pub fn step_timing_async(delta: u64) {
    unsafe {
        wpihal_sys::HALSIM_StepTimingAsync(delta);
    }
}

pub fn set_send_error(handler: wpihal_sys::HALSIM_SendErrorHandler) {
    unsafe {
        wpihal_sys::HALSIM_SetSendError(handler);
    }
}

pub fn set_send_console_line(handler: wpihal_sys::HALSIM_SendConsoleLineHandler) {
    unsafe {
        wpihal_sys::HALSIM_SetSendConsoleLine(handler);
    }
}

pub trait SimPeriodCallback {
    fn callback(&mut self);
}

impl<T: FnMut()> SimPeriodCallback for T {
    fn callback(&mut self) {
        (self)();
    }
}

unsafe extern "C" fn sim_period_callback_trampoline<C: SimPeriodCallback>(
    param: *mut core::ffi::c_void,
) {
    let Some(ptr) = core::ptr::NonNull::new(param) else {
        return;
    };
    unsafe {
        ptr.cast::<C>().as_mut().callback();
    }
}

pub fn register_sim_periodic_before_callback<C: SimPeriodCallback>(
    callback: C,
) -> CallbackHandle<C> {
    register_callback!(
        HALSIM_RegisterSimPeriodicBeforeCallback,
        HALSIM_CancelSimPeriodicBeforeCallback,
        sim_period_callback_trampoline::<C>,
        callback,
    )
}

pub fn register_sim_periodic_after_callback<C: SimPeriodCallback>(
    callback: C,
) -> CallbackHandle<C> {
    register_callback!(
        HALSIM_RegisterSimPeriodicAfterCallback,
        HALSIM_CancelSimPeriodicAfterCallback,
        sim_period_callback_trampoline::<C>,
        callback,
    )
}
