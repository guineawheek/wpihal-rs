use std::{
    ffi::{CStr, CString, c_char},
    marker::PhantomData,
};

use wpihal_sys::{
    HAL_CreateSimDevice, HAL_CreateSimValue, HAL_CreateSimValueEnum, HAL_CreateSimValueEnumDouble,
    HAL_FreeSimDevice, HAL_GetSimDeviceName, HAL_GetSimValue, HAL_ResetSimValue, HAL_SetSimValue,
    HAL_SimDeviceHandle, HAL_SimValueHandle, HAL_Value,
};

use crate::value::HALValue;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum SimValueDirection {
    Input = 0,
    Output = 1,
    Bidir = 2,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SimDevice(HAL_SimDeviceHandle);

impl SimDevice {
    pub fn new(name: &CStr) -> Option<Self> {
        unsafe {
            match HAL_CreateSimDevice(name.as_ptr()) {
                0 => None,
                otherwise => Some(Self(otherwise)),
            }
        }
    }

    pub fn handle(&self) -> HAL_SimDeviceHandle {
        self.0
    }

    pub fn get_device_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(HAL_GetSimDeviceName(self.0)) }
    }

    pub fn create_sim_hal_value(
        &self,
        name: &CStr,
        direction: SimValueDirection,
        initial_value: &HALValue,
    ) -> Option<SimHalValue> {
        unsafe {
            let initial_value: HAL_Value = initial_value.clone().into();
            match HAL_CreateSimValue(self.0, name.as_ptr(), direction as i32, &initial_value) {
                0 => None,
                otherwise => Some(SimHalValue(otherwise)),
            }
        }
    }

    pub fn create_enum(
        &self,
        name: &CStr,
        direction: SimValueDirection,
        options: &[&CStr],
        initial_index: usize,
    ) -> Option<SimHalValue> {
        unsafe {
            match HAL_CreateSimValueEnum(
                self.0,
                name.as_ptr(),
                direction as i32,
                options.len() as i32,
                options.as_ptr() as *mut *const c_char,
                initial_index as i32,
            ) {
                0 => None,
                otherwise => Some(SimHalValue(otherwise)),
            }
        }
    }

    pub fn create_enum_double(
        &self,
        name: &CStr,
        direction: SimValueDirection,
        options: &[&CStr],
        option_values: &[f64],
        initial_index: usize,
    ) -> Option<SimHalValue> {
        unsafe {
            match HAL_CreateSimValueEnumDouble(
                self.0,
                name.as_ptr(),
                direction as i32,
                options.len() as i32,
                options.as_ptr() as *mut *const c_char,
                option_values.as_ptr(),
                initial_index as i32,
            ) {
                0 => None,
                otherwise => Some(SimHalValue(otherwise)),
            }
        }
    }

    pub fn create_sim_value<T: Into<HALValue>, D: ValueDirection>(
        &self,
        name: &str,
        initial_value: T,
    ) -> Option<SimValue<T, D>> {
        let cname = to_cstring(name);
        let handle = self.create_sim_hal_value(&cname, D::DIRECTION, &initial_value.into())?;
        Some(SimValue {
            handle,
            p: PhantomData,
        })
    }

    pub fn create_sim_enum<T: SimEnum, D: ValueDirection>(
        &self,
        name: &str,
        initial_value: T,
    ) -> Option<SimValue<T, D>> {
        let cname = to_cstring(name);
        let options = T::VARIANTS
            .iter()
            .map(|v| to_cstring(v.name()))
            .collect::<Vec<CString>>();
        let mut options_ptr = options
            .iter()
            .map(|c| c.as_ptr())
            .collect::<Vec<*const std::ffi::c_char>>();
        let handle = if T::HAS_VALUES {
            let option_values = T::VARIANTS.iter().map(T::value).collect::<Vec<f64>>();
            unsafe {
                HAL_CreateSimValueEnumDouble(
                    self.0,
                    cname.as_ptr(),
                    D::DIRECTION as _,
                    options.len() as i32,
                    options_ptr.as_mut_ptr(),
                    option_values.as_ptr(),
                    initial_value.index() as i32,
                )
            }
        } else {
            unsafe {
                HAL_CreateSimValueEnum(
                    self.0,
                    cname.as_ptr(),
                    D::DIRECTION as _,
                    options.len() as i32,
                    options_ptr.as_mut_ptr(),
                    initial_value.index() as i32,
                )
            }
        };
        if handle == 0 {
            None
        } else {
            Some(SimValue {
                handle: SimHalValue(handle),
                p: PhantomData,
            })
        }
    }
}

impl Drop for SimDevice {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeSimDevice(self.0);
        }
    }
}

/// idempotent
fn to_cstring(s: &str) -> CString {
    match CString::new(s.as_bytes()) {
        Ok(cs) => {
            return cs;
        }
        Err(e) => {
            let position = e.nul_position();
            let mut v = e.into_vec();
            v.truncate(position);
            CString::new(v).unwrap()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimHalValue(HAL_SimValueHandle);

impl SimHalValue {
    pub fn get(&self) -> HALValue {
        let mut data = HAL_Value::default();
        unsafe { HAL_GetSimValue(self.0, &mut data) };
        data.into()
    }

    pub fn set(&self, value: &HALValue) {
        let value: HAL_Value = value.clone().into();
        unsafe {
            HAL_SetSimValue(self.0, &value);
        }
    }

    pub fn reset(&self) {
        unsafe {
            HAL_ResetSimValue(self.0);
        }
    }
}

pub trait ValueDirection {
    const DIRECTION: SimValueDirection;
}
pub trait InputDirection {}
pub trait OutputDirection {}

pub struct Input;
impl ValueDirection for Input {
    const DIRECTION: SimValueDirection = SimValueDirection::Input;
}
impl InputDirection for Input {}
pub struct Output;
impl ValueDirection for Output {
    const DIRECTION: SimValueDirection = SimValueDirection::Output;
}
impl OutputDirection for Output {}
pub struct Bidirectional;
impl ValueDirection for Bidirectional {
    const DIRECTION: SimValueDirection = SimValueDirection::Bidir;
}
impl InputDirection for Bidirectional {}
impl OutputDirection for Bidirectional {}

/// Enum value trait
pub trait SimEnum: Sized + Clone + Default + PartialEq + 'static {
    /// list of all possible variants.
    ///
    /// The index of each instance as they appear in here
    /// corresponds to their underlying index assigned internally.
    ///
    /// No more than [`i32::MAX`] variants are supported.
    const VARIANTS: &'static [Self];

    /// Gets the index of the given variant.
    /// Default impl is an O(n), probably not great.
    ///
    /// If the variant isn't found in [`Self::variants`], this returns 0.
    fn index(&self) -> usize {
        Self::VARIANTS
            .iter()
            .enumerate()
            .find_map(|(i, v)| (v == self).then_some(i))
            .unwrap_or(0)
    }

    /// Name associated with a given sim-enum instance.
    /// This is 'static because this value should not change.
    fn name(&self) -> &'static str;

    /// whether to associate enum variants with their values.
    const HAS_VALUES: bool = false;
    /// Value associated with the given variant.
    /// Only useful if `HAS_VALUES` is also true.
    fn value(&self) -> f64 {
        0.0
    }
}

/// Implements a sim-enum for a given enum.
///
/// ```ignore
/// #[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
/// enum Example {
///     #[default]
///     Default,
///     Value1,
///     Value2,
/// }
/// impl_sim_enum!(Example {
///     Default -> "Default",
///     Value1 -> "Value 1",
///     Value2 -> "Value 2",
/// });
///
/// // you can also omit the values and it'll stringify the variants;
/// impl_sim_enum!(Example {
///     Default,
///     Value1,
///     Value2,
/// });
///
/// // if ANY value has an f64 in it, the enum is now marked as an "enum double",
/// // and HAS_VALUES will be true.
/// impl_sim_enum!(Example {
///     Default,
///     Value1 = 6.7 -> "number",
///     Value2,
/// });
/// ```
///
/// Should this have been a derive proc macro? yeah probably
#[macro_export]
macro_rules! impl_sim_enum {
    ($e:ident {
        $($variant:ident $( = $value:literal)? $(-> $name:literal)?),+ $(,)?
    }) => {
        impl SimEnum for $e {
            const VARIANTS: &'static [$e] = &[$(Self::$variant),+];

            #[allow(unused)]
            fn index(&self) -> usize {
                let mut i = 0;
                $(
                    #[allow(non_snake_case)]
                    let $variant = i;
                    i += 1;
                )+
                match self {
                    $(
                        Self::$variant => $variant,
                    )+
                }
            }

            fn name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => [$($name,)? stringify!($variant)][0],
                    )+
                }
            }

            const HAS_VALUES: bool = [0.0_f64, $($($value,)?)+].len() > 1;
            fn value(&self) -> f64 {
                match self {
                    $(
                        Self::$variant => [$($value,)? 0.0_f64][0],
                    )+
                }
            }
        }
    };
}

pub struct SimValue<T, D> {
    handle: SimHalValue,
    p: PhantomData<(T, D)>,
}

macro_rules! impl_primitive {
    ($t:ty, $getter:ident) => {
        impl<D: InputDirection> SimValue<$t, D> {
            /// Gets the value.
            ///
            /// Returns the default value if the underlying [`SimValue`] holds an invalid value.
            pub fn get(&self) -> $t {
                self.handle.get().$getter().unwrap_or_default()
            }
        }

        impl<D: OutputDirection> SimValue<$t, D> {
            /// Sets the value.
            pub fn set(&self, value: $t) {
                self.handle.set(&HALValue::from(value));
            }
            /// Resets the value (used for incremental sensor values)
            pub fn reset(&self) {
                self.handle.reset();
            }
        }
    };
}
impl_primitive!(bool, get_bool);
impl_primitive!(f64, get_double);
impl_primitive!(i32, get_int);
impl_primitive!(i64, get_long);

impl<E: SimEnum, D: InputDirection> SimValue<E, D> {
    /// Gets the corresponding enum value.
    pub fn get(&self) -> E {
        self.handle.get().get_enum().map_or_else(E::default, |idx| {
            E::VARIANTS.get(idx as usize).cloned().unwrap_or_default()
        })
    }
}

impl<E: SimEnum, D: InputDirection> SimValue<E, D> {
    /// Sets the corresponding enum value.
    pub fn set(&self, value: E) {
        self.handle.set(&HALValue::Enum(value.index() as i32));
    }
}
