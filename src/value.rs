use wpihal_sys::{HAL_Type, HAL_Value, HAL_Value__bindgen_ty_1};

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum HALValue {
    Unassigned = 0x0,
    Boolean(bool) = 0x01,
    Double(f64) = 0x02,
    Enum(i32) = 0x04,
    Int(i32) = 0x08,
    Long(i64) = 0x10,
}

impl HALValue {
    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(v) => Some(*v),
            _ => None,
        }
    }

    pub fn get_double(&self) -> Option<f64> {
        match self {
            Self::Double(v) => Some(*v),
            _ => None,
        }
    }

    pub fn get_enum(&self) -> Option<i32> {
        match self {
            Self::Enum(v) => Some(*v),
            _ => None,
        }
    }

    pub fn get_int(&self) -> Option<i32> {
        match self {
            Self::Int(v) => Some(*v),
            _ => None,
        }
    }

    pub fn get_long(&self) -> Option<i64> {
        match self {
            Self::Long(v) => Some(*v),
            _ => None,
        }
    }
}

impl From<HAL_Value> for HALValue {
    fn from(value: HAL_Value) -> Self {
        unsafe {
            match value.type_ {
                HAL_Type::HAL_UNASSIGNED => HALValue::Unassigned,
                HAL_Type::HAL_BOOLEAN => HALValue::Boolean(value.data.v_boolean != 0),
                HAL_Type::HAL_DOUBLE => HALValue::Double(value.data.v_double),
                HAL_Type::HAL_ENUM => HALValue::Enum(value.data.v_enum),
                HAL_Type::HAL_INT => HALValue::Int(value.data.v_int),
                HAL_Type::HAL_LONG => HALValue::Long(value.data.v_long),
            }
        }
    }
}

impl From<HALValue> for HAL_Value {
    fn from(value: HALValue) -> Self {
        match value {
            HALValue::Unassigned => HAL_Value {
                data: HAL_Value__bindgen_ty_1 { v_boolean: 0 },
                type_: HAL_Type::HAL_UNASSIGNED,
            },
            HALValue::Boolean(v) => HAL_Value {
                data: HAL_Value__bindgen_ty_1 {
                    v_boolean: v as i32,
                },
                type_: HAL_Type::HAL_BOOLEAN,
            },
            HALValue::Double(v) => HAL_Value {
                data: HAL_Value__bindgen_ty_1 { v_double: v },
                type_: HAL_Type::HAL_DOUBLE,
            },
            HALValue::Enum(v) => HAL_Value {
                data: HAL_Value__bindgen_ty_1 { v_enum: v },
                type_: HAL_Type::HAL_ENUM,
            },
            HALValue::Int(v) => HAL_Value {
                data: HAL_Value__bindgen_ty_1 { v_int: v },
                type_: HAL_Type::HAL_INT,
            },
            HALValue::Long(v) => HAL_Value {
                data: HAL_Value__bindgen_ty_1 { v_long: v },
                type_: HAL_Type::HAL_LONG,
            },
        }
    }
}

impl TryFrom<HALValue> for bool {
    type Error = ();

    fn try_from(value: HALValue) -> Result<Self, Self::Error> {
        value.get_bool().ok_or(())
    }
}

impl From<bool> for HALValue {
    fn from(value: bool) -> Self {
        HALValue::Boolean(value)
    }
}

impl TryFrom<HALValue> for f64 {
    type Error = ();

    fn try_from(value: HALValue) -> Result<Self, Self::Error> {
        value.get_double().ok_or(())
    }
}

impl From<f64> for HALValue {
    fn from(value: f64) -> Self {
        HALValue::Double(value)
    }
}

impl TryFrom<HALValue> for i32 {
    type Error = ();

    fn try_from(value: HALValue) -> Result<Self, Self::Error> {
        value.get_int().ok_or(())
    }
}

impl From<i32> for HALValue {
    fn from(value: i32) -> Self {
        HALValue::Int(value)
    }
}

impl TryFrom<HALValue> for i64 {
    type Error = ();

    fn try_from(value: HALValue) -> Result<Self, Self::Error> {
        value.get_long().ok_or(())
    }
}

impl From<i64> for HALValue {
    fn from(value: i64) -> Self {
        HALValue::Long(value)
    }
}
