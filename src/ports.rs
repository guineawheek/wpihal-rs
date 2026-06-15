use wpihal_sys::{
    HAL_GetNumAddressableLEDs, HAL_GetNumAnalogInputs, HAL_GetNumCTREPCMModules,
    HAL_GetNumCTREPDPChannels, HAL_GetNumCTREPDPModules, HAL_GetNumCTRESolenoidChannels,
    HAL_GetNumCanBuses, HAL_GetNumCounters, HAL_GetNumDigitalChannels, HAL_GetNumDigitalPWMOutputs,
    HAL_GetNumDutyCycles, HAL_GetNumEncoders, HAL_GetNumInterrupts, HAL_GetNumPWMChannels,
    HAL_GetNumREVPDHChannels, HAL_GetNumREVPDHModules, HAL_GetNumREVPHChannels,
    HAL_GetNumREVPHModules, HAL_GetNumSmartIo,
};

pub fn get_num_can_buses() -> usize {
    unsafe { HAL_GetNumCanBuses() as usize }
}

pub fn get_num_smart_io() -> usize {
    unsafe { HAL_GetNumSmartIo() as usize }
}

pub fn get_num_analog_inputs() -> usize {
    unsafe { HAL_GetNumAnalogInputs() as usize }
}

pub fn get_num_counters() -> usize {
    unsafe { HAL_GetNumCounters() as usize }
}

pub fn get_num_digital_channels() -> usize {
    unsafe { HAL_GetNumDigitalChannels() as usize }
}

pub fn get_num_pwm_channels() -> usize {
    unsafe { HAL_GetNumPWMChannels() as usize }
}

pub fn get_num_digital_pwm_outputs() -> usize {
    unsafe { HAL_GetNumDigitalPWMOutputs() as usize }
}

pub fn get_num_encoders() -> usize {
    unsafe { HAL_GetNumEncoders() as usize }
}

pub fn get_num_interrupts() -> usize {
    unsafe { HAL_GetNumInterrupts() as usize }
}

pub fn get_num_ctre_pcm_modules() -> usize {
    unsafe { HAL_GetNumCTREPCMModules() as usize }
}

pub fn get_num_ctre_solenoid_channels() -> usize {
    unsafe { HAL_GetNumCTRESolenoidChannels() as usize }
}

pub fn get_num_ctre_pdp_modules() -> usize {
    unsafe { HAL_GetNumCTREPDPModules() as usize }
}

pub fn get_num_ctre_pdp_channels() -> usize {
    unsafe { HAL_GetNumCTREPDPChannels() as usize }
}

pub fn get_num_rev_pdh_modules() -> usize {
    unsafe { HAL_GetNumREVPDHModules() as usize }
}

pub fn get_num_rev_pdh_channels() -> usize {
    unsafe { HAL_GetNumREVPDHChannels() as usize }
}

pub fn get_num_rev_ph_modules() -> usize {
    unsafe { HAL_GetNumREVPHModules() as usize }
}

pub fn get_num_rev_ph_channels() -> usize {
    unsafe { HAL_GetNumREVPHChannels() as usize }
}

pub fn get_num_duty_cycles() -> usize {
    unsafe { HAL_GetNumDutyCycles() as usize }
}

pub fn get_num_addressable_leds() -> usize {
    unsafe { HAL_GetNumAddressableLEDs() as usize }
}
