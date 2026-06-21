use crate::halsim::{halsim_data, halsim_value};

halsim_value!(PowerDistributionCurrent::<f64>(i32, i32));

halsim_data!(PowerDistribution {
    initialized: bool,
    temperature: f64,
    voltage: f64,
});

impl PowerDistribution {
    pub const fn current(&self, channel: i32) -> PowerDistributionCurrent {
        PowerDistributionCurrent(self.0, channel)
    }

    pub fn get_all_currents(&self, currents: &mut [f64]) {
        unsafe {
            wpihal_sys::HALSIM_GetPowerDistributionAllCurrents(
                self.0,
                currents.as_mut_ptr(),
                currents.len().max(i32::MAX as usize) as i32,
            );
        }
    }

    pub fn set_all_currents(&self, currents: &[f64]) {
        unsafe {
            wpihal_sys::HALSIM_SetPowerDistributionAllCurrents(
                self.0,
                currents.as_ptr(),
                currents.len().max(i32::MAX as usize) as i32,
            );
        }
    }
}
