use wpihal_sys::{
    HAL_CONTROLWORD_DS_ATTACHED_MASK, HAL_CONTROLWORD_ENABLED_MASK, HAL_CONTROLWORD_ESTOP_MASK,
    HAL_CONTROLWORD_FMS_ATTACHED_MASK, HAL_CONTROLWORD_OPMODE_HASH_MASK,
    HAL_CONTROLWORD_ROBOT_MODE_MASK, HAL_CONTROLWORD_ROBOT_MODE_SHIFT, HAL_ControlWord,
    HAL_GetControlWord, HAL_GetUncachedControlWord, HAL_RobotMode,
};

use crate::{error::HALResult, hal_retcall};

pub use wpihal_sys::HAL_RobotMode as RobotMode;

const fn gate(cond: bool, value: u64) -> u64 {
    if cond { value } else { 0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ControlWord(u64);
impl ControlWord {
    pub const fn new(
        op_mode_hash: i64,
        robot_mode: RobotMode,
        enabled: bool,
        e_stop: bool,
        fms_attached: bool,
        ds_attached: bool,
    ) -> Self {
        let value = ((op_mode_hash as u64) & HAL_CONTROLWORD_OPMODE_HASH_MASK)
            | (((robot_mode as u64) << HAL_CONTROLWORD_ROBOT_MODE_SHIFT)
                & HAL_CONTROLWORD_ROBOT_MODE_MASK)
            | gate(enabled, HAL_CONTROLWORD_ENABLED_MASK)
            | gate(e_stop, HAL_CONTROLWORD_ESTOP_MASK)
            | gate(fms_attached, HAL_CONTROLWORD_FMS_ATTACHED_MASK)
            | gate(ds_attached, HAL_CONTROLWORD_DS_ATTACHED_MASK);

        Self(value)
    }

    /// Gets the current control word.
    pub fn get() -> HALResult<Self> {
        hal_retcall!(HAL_GetControlWord() -> HAL_ControlWord).map(Self::from)
    }

    /// Gets the current uncached control word.
    pub fn get_uncached() -> HALResult<Self> {
        hal_retcall!(HAL_GetUncachedControlWord() -> HAL_ControlWord).map(Self::from)
    }

    pub const fn op_mode_hash(&self) -> i64 {
        (self.0 & HAL_CONTROLWORD_OPMODE_HASH_MASK) as i64
    }

    pub const fn op_mode_id(&self) -> Option<i64> {
        if (self.0 & HAL_CONTROLWORD_OPMODE_HASH_MASK) == 0 {
            None
        } else {
            Some(
                (self.0 & (HAL_CONTROLWORD_OPMODE_HASH_MASK | HAL_CONTROLWORD_ROBOT_MODE_MASK))
                    as i64,
            )
        }
    }

    pub const fn robot_mode(&self) -> HAL_RobotMode {
        let idx: i32 =
            ((self.0 & HAL_CONTROLWORD_ROBOT_MODE_MASK) >> HAL_CONTROLWORD_ROBOT_MODE_SHIFT) as i32;
        // SAFETY: literally not enough bits to screw this up
        unsafe { core::mem::transmute(idx) }
    }

    pub const fn enabled(&self) -> bool {
        self.0 & HAL_CONTROLWORD_ENABLED_MASK != 0
    }

    pub const fn estop(&self) -> bool {
        self.0 & HAL_CONTROLWORD_ESTOP_MASK != 0
    }

    pub const fn fms_attached(&self) -> bool {
        self.0 & HAL_CONTROLWORD_FMS_ATTACHED_MASK != 0
    }

    pub const fn ds_attached(&self) -> bool {
        self.0 & HAL_CONTROLWORD_DS_ATTACHED_MASK != 0
    }
}

impl From<HAL_ControlWord> for ControlWord {
    fn from(value: HAL_ControlWord) -> Self {
        Self(value.value as _)
    }
}
impl From<ControlWord> for HAL_ControlWord {
    fn from(value: ControlWord) -> Self {
        Self {
            value: value.0 as _,
        }
    }
}
