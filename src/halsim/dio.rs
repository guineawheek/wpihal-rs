use crate::halsim::halsim_data;

halsim_data!(DIO {
    initialized: bool,
    value: bool,
    pulse_length: f64,
    is_input: bool,
    filter_index: i32,
});
