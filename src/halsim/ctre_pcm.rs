use crate::halsim::{halsim_data, halsim_value};

halsim_value!(CTREPCMSolenoidOutput::<bool>(i32, i32));

halsim_data!(CTREPCM {
    initialized: bool,
    compressor_on: bool,
    closed_loop_enabled: bool,
    pressure_switch: bool,
    compressor_current: f64,
});

impl CTREPCM {
    pub const fn solenoid_output(&self, channel: i32) -> CTREPCMSolenoidOutput {
        CTREPCMSolenoidOutput(self.0, channel)
    }
}
