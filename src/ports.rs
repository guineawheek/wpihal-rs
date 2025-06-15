use wpihal_sys::{
    HAL_GetNumAccumulators, HAL_GetNumAddressableLEDs, HAL_GetNumAnalogInputs,
    HAL_GetNumAnalogOutputs, HAL_GetNumAnalogTriggers, HAL_GetNumCTREPCMModules,
    HAL_GetNumCTREPDPChannels, HAL_GetNumCTREPDPModules, HAL_GetNumCTRESolenoidChannels,
    HAL_GetNumCounters, HAL_GetNumDigitalChannels, HAL_GetNumDigitalHeaders,
    HAL_GetNumDigitalPWMOutputs, HAL_GetNumDutyCycles, HAL_GetNumEncoders, HAL_GetNumPWMChannels,
    HAL_GetNumPWMHeaders, HAL_GetNumREVPDHChannels, HAL_GetNumREVPDHModules,
    HAL_GetNumREVPHChannels, HAL_GetNumREVPHModules, HAL_GetNumRelayChannels,
    HAL_GetNumRelayHeaders,
};

pub fn get_num_accumulators() -> i32 {
    unsafe { HAL_GetNumAccumulators() }
}

pub fn get_num_analog_triggers() -> i32 {
    unsafe { HAL_GetNumAnalogTriggers() }
}

pub fn get_num_analog_inputs() -> i32 {
    unsafe { HAL_GetNumAnalogInputs() }
}

pub fn get_num_analog_outputs() -> i32 {
    unsafe { HAL_GetNumAnalogOutputs() }
}

pub fn get_num_counters() -> i32 {
    unsafe { HAL_GetNumCounters() }
}

pub fn get_num_digital_headers() -> i32 {
    unsafe { HAL_GetNumDigitalHeaders() }
}

pub fn get_num_pwm_headers() -> i32 {
    unsafe { HAL_GetNumPWMHeaders() }
}

pub fn get_num_digital_channels() -> i32 {
    unsafe { HAL_GetNumDigitalChannels() }
}

pub fn get_num_pwm_channels() -> i32 {
    unsafe { HAL_GetNumPWMChannels() }
}

pub fn get_num_digital_pwm_outputs() -> i32 {
    unsafe { HAL_GetNumDigitalPWMOutputs() }
}

pub fn get_num_encoders() -> i32 {
    unsafe { HAL_GetNumEncoders() }
}

pub fn get_num_relay_channels() -> i32 {
    unsafe { HAL_GetNumRelayChannels() }
}

pub fn get_num_relay_headers() -> i32 {
    unsafe { HAL_GetNumRelayHeaders() }
}

pub fn get_num_ctre_pcm_modules() -> i32 {
    unsafe { HAL_GetNumCTREPCMModules() }
}

pub fn get_num_ctre_solenoid_channels() -> i32 {
    unsafe { HAL_GetNumCTRESolenoidChannels() }
}

pub fn get_num_ctre_pdp_modules() -> i32 {
    unsafe { HAL_GetNumCTREPDPModules() }
}

pub fn get_num_ctre_pdp_channels() -> i32 {
    unsafe { HAL_GetNumCTREPDPChannels() }
}

pub fn get_num_rev_pdh_modules() -> i32 {
    unsafe { HAL_GetNumREVPDHModules() }
}

pub fn get_num_rev_pdh_channels() -> i32 {
    unsafe { HAL_GetNumREVPDHChannels() }
}

pub fn get_num_rev_ph_modules() -> i32 {
    unsafe { HAL_GetNumREVPHModules() }
}

pub fn get_num_rev_ph_channels() -> i32 {
    unsafe { HAL_GetNumREVPHChannels() }
}

pub fn get_num_duty_cycles() -> i32 {
    unsafe { HAL_GetNumDutyCycles() }
}

pub fn get_num_addressable_leds() -> i32 {
    unsafe { HAL_GetNumAddressableLEDs() }
}
