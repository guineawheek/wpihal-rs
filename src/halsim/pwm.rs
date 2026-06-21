use crate::halsim::halsim_data;

halsim_data!(PWM {
    initialized: bool,
    pulse_microsecond: i32,
    output_period: i32,
});
