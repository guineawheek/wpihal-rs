use crate::halsim::halsim_data;

halsim_data!(Encoder {
    initialized: bool,
    count: i32,
    rate: f64,
    reset: bool,
    direction: bool,
    reverse_direction: bool,
    distance_per_pulse: f64
});
