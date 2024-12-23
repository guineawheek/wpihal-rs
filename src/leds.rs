use wpihal_sys::{HAL_GetRadioLEDState, HAL_RadioLEDState, HAL_SetRadioLEDState};

use crate::{error::HALResult, hal_call};

pub type RadioLEDState = HAL_RadioLEDState;
pub fn set_radio_led_state(state: RadioLEDState) -> HALResult<()> {
    hal_call!(HAL_SetRadioLEDState(state))
}

pub fn get_radio_led_state() -> HALResult<RadioLEDState> {
    hal_call!(HAL_GetRadioLEDState())
}