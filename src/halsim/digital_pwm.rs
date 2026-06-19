use wpihal_sys::HALSIM_FindDigitalPWMForChannel;

use crate::halsim::halsim_data;

halsim_data!(DigitalPWM {
    initialized: bool,
    duty_cycle: f64,
    pin: i32,
});

impl DigitalPWM {
    pub fn find_for_channel(channel: i32) -> Self {
        unsafe { Self(HALSIM_FindDigitalPWMForChannel(channel)) }
    }
}
