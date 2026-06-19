use crate::halsim::halsim_data;

halsim_data!(Encoder {
    initialized: bool,
    count: i32,
    period: f64,
    reset: bool,
    max_period: f64,
    direction: bool,
    reverse_direction: bool,
    samples_to_average: i32,
    distance_per_pulse: f64
});
