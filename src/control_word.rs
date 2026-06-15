use wpihal_sys::{
    HAL_CONTROLWORD_DS_ATTACHED_MASK, HAL_CONTROLWORD_ENABLED_MASK, HAL_CONTROLWORD_ESTOP_MASK,
    HAL_CONTROLWORD_FMS_ATTACHED_MASK, HAL_CONTROLWORD_OPMODE_HASH_MASK,
    HAL_CONTROLWORD_ROBOT_MODE_MASK, HAL_CONTROLWORD_ROBOT_MODE_SHIFT, HAL_GetControlWord,
    HAL_RobotMode,
};

use crate::error::{HALError, HALResult};

pub type RobotMode = HAL_RobotMode;

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
            | (((robot_mode as u64) << HAL_CONTROLWORD_ROBOT_MODE_SHIFT) &  // NOLINT
       HAL_CONTROLWORD_ROBOT_MODE_MASK)
            | (if enabled {
                HAL_CONTROLWORD_ENABLED_MASK
            } else {
                0
            })
            | (if e_stop {
                HAL_CONTROLWORD_ESTOP_MASK
            } else {
                0
            })
            | (if fms_attached {
                HAL_CONTROLWORD_FMS_ATTACHED_MASK
            } else {
                0
            })
            | (if ds_attached {
                HAL_CONTROLWORD_DS_ATTACHED_MASK
            } else {
                0
            });

        Self(value)
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

    pub const fn robot_mode(&self) -> RobotMode {}

    pub fn enabled(&self) -> bool {
        self.0 & HAL_C
    }

    pub fn autonomous(&self) -> bool {
        self.0 & 0b10 != 0
    }

    pub fn test(&self) -> bool {
        self.0 & 0b100 != 0
    }

    pub fn estop(&self) -> bool {
        self.0 & 0b1000 != 0
    }

    pub fn fms_attached(&self) -> bool {
        self.0 & 0b10000 != 0
    }

    pub fn ds_attached(&self) -> bool {
        self.0 & 0b100000 != 0
    }
}

pub fn get_control_word() -> HALResult<ControlWord> {
    unsafe {
        let mut word: HAL_ControlWord = core::mem::transmute(0u32);
        match HAL_GetControlWord(&mut word) {
            0 => Ok(core::mem::transmute(word)),
            err => Err(HALError(err)),
        }
    }
}
