use crate::halsim::halsim_data;

halsim_data!(DutyCycle {
    initialized: bool,
    frequency: f64,
    output: f64,
});
