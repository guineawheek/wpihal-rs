use wpihal_sys::HAL_REVPHCompressorConfigType;

use crate::halsim::{halsim_data, halsim_value};

halsim_value!(REVPHSolenoidOutput::<bool>(i32, i32));

halsim_data!(REVPH {
    initialized: bool,
    compressor_on: bool,
    compressor_config_type: HAL_REVPHCompressorConfigType,
    pressure_switch: bool,
    compressor_current: f64,
});

impl REVPH {
    pub const fn solenoid_output(&self, channel: i32) -> REVPHSolenoidOutput {
        REVPHSolenoidOutput(self.0, channel)
    }

    pub fn get_all_solenoids(&self) -> u8 {
        unsafe {
            let mut out = 0;
            wpihal_sys::HALSIM_GetREVPHAllSolenoids(self.0, &mut out);
            out
        }
    }

    pub fn set_all_solenoids(&self, value: u8) {
        unsafe {
            wpihal_sys::HALSIM_SetREVPHAllSolenoids(self.0, value);
        }
    }
}
