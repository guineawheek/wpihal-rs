#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(unused)]

use core::mem::ManuallyDrop;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl From<NTCoreRS_Vec> for String {
    fn from(value: NTCoreRS_Vec) -> Self {
        unsafe { String::from_raw_parts(value.data as *mut u8, value.len, value.capacity) }
    }
}

macro_rules! from_vec {
    ($t:ty) => {
        impl From<ManuallyDrop<Vec<$t>>> for NTCoreRS_Vec {
            fn from(mut value: ManuallyDrop<Vec<$t>>) -> Self {
                NTCoreRS_Vec {
                    data: value.as_mut_ptr() as *mut std::ffi::c_char,
                    len: value.len(),
                    capacity: value.capacity(),
                }
            }
        }

        impl From<NTCoreRS_Vec> for Vec<$t> {
            fn from(value: NTCoreRS_Vec) -> Self {
                unsafe { Vec::from_raw_parts(value.data as *mut $t, value.len, value.capacity) }
            }
        }
    };
}
from_vec!(bool);
from_vec!(std::ffi::c_char);
from_vec!(u8);
from_vec!(NT_Handle);
from_vec!(f32);
from_vec!(f64);
from_vec!(i64);
from_vec!(String);
from_vec!(NTCoreRS_Value);
from_vec!(NTCoreRS_TopicInfo);
