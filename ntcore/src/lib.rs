//! # ntcore -- NetworkTables support.
//! 
//! This directly wraps the NetworkTables `ntcore` library.
//! 
//! ## Performance
//! 
//! Given that at its core `ntcore` is a C++ library and this is a Rust library binding to its C interface 
//! there is a compromise made for usability over performance.
//! 
//! In practice, this means that `ntcore` itself will often allocate buffers between the C api and its own internals
//! while this Rust library will often allocate buffers so they conform to Rust's semantics.
//! This is mostly irrelevant for primitives like integers and floats, but may be more apparent for strings
//! and arrays, which need allocation to copy from [`Vec`]s and [`String`]s to `std::vector` and `WPI_String`.
//! 
//! If you want the absolute best possible performance, use a pure Rust library.

use ntcore_sys::{NT_Entry, NT_Inst, NT_Topic, NT_Value, WPI_String};
use wpiutil::wpistring::WPIString;

trait ToWpiString {
    fn to_wpistring(self) -> WPIString;
}
trait ToWpiStringSys {
    fn to_wpistring_sys(&self) -> &WPI_String;
}

impl ToWpiString for WPI_String {
    fn to_wpistring(self) -> WPIString {
        WPIString::from_raw(
            // SAFETY: these are derived from the same struct so these should have the same layout
            unsafe { core::mem::transmute(self) }
        )
    }
}

impl ToWpiStringSys for WPIString {
    fn to_wpistring_sys(&self) -> &WPI_String {
        unsafe {
            core::mem::transmute(self.as_raw())
        }
    }
}

fn none_if<T>(predicate: bool, value: T) -> Option<T> {
    if predicate { None } else { Some(value) }
}


pub use ntcore_sys::NtType;
/// Holds a NetworkTables value.
pub struct NtValue {
    pub last_change: i64,
    pub server_time: i64,
    pub value: NtValueData,
}

impl From<&NT_Value> for NtValue {
    fn from(value: &NT_Value) -> Self {
        let ret = Self {
            last_change: value.last_change,
            server_time: value.server_time,
            value: NtValueData::from(value),
        };
        ret
    }
}


macro_rules! mk_slice {
    ($field:expr, $ptr_ty:ty) => {
        core::slice::from_raw_parts(($field).arr as *const $ptr_ty, ($field).size)
    };
}

/// NetworkTables value data, in Rust types.
#[derive(Debug, Clone, PartialEq)]
pub enum NtValueData {
    Unassigned,
    Boolean(bool),
    Double(f64),
    String(String),
    Raw(Vec<u8>),
    BooleanArray(Vec<bool>),
    DoubleArray(Vec<f64>),
    StringArray(Vec<String>),
    Rpc(Vec<u8>),
    Integer(i64),
    Float(f32),
    IntegerArray(Vec<i64>),
    FloatArray(Vec<f32>),
}

impl From<&NT_Value> for NtValueData {
    fn from(value: &NT_Value) -> Self {
        unsafe {
            match value.type_ {
                NtType::Unassigned => NtValueData::Unassigned,
                NtType::Boolean => NtValueData::Boolean(value.data.v_boolean != 0),
                NtType::Double => NtValueData::Double(value.data.v_double),
                NtType::String => NtValueData::String(value.data.v_string.to_wpistring().to_string()),
                NtType::Raw => {
                    let data = core::slice::from_raw_parts(value.data.v_raw.data as *const u8, value.data.v_raw.size);
                    NtValueData::Raw(data.to_vec())
                }
                NtType::BooleanArray => {
                    //let data = core::slice::from_raw_parts(value.data.arr_boolean.arr as *const i32, value.data.arr_boolean.size);
                    NtValueData::BooleanArray(mk_slice!(value.data.arr_boolean, i32).iter().map(|&v| v != 0).collect())
                }
                NtType::DoubleArray => {
                    NtValueData::DoubleArray(mk_slice!(value.data.arr_double, f64).to_vec())
                }
                NtType::StringArray => {
                    let data = mk_slice!(value.data.arr_string, WPI_String);
                    NtValueData::StringArray(data.iter().map(|s| s.to_wpistring().to_string()).collect())
                }
                NtType::Rpc => {
                    let data = core::slice::from_raw_parts(value.data.v_raw.data as *const u8, value.data.v_raw.size);
                    NtValueData::Rpc(data.to_vec())
                }
                NtType::Integer => NtValueData::Integer(value.data.v_int),
                NtType::Float => NtValueData::Float(value.data.v_float),
                NtType::IntegerArray => NtValueData::IntegerArray(mk_slice!(value.data.arr_int, i64).to_vec()),
                NtType::FloatArray => NtValueData::FloatArray(mk_slice!(value.data.arr_float, f32).to_vec()),
            }
        }
    }
}

impl NtValueData {
    /// Creates a raw [`NT_Value`] structure.
    /// 
    /// Note that the FFI calls that take this will ostensibly copy the data into its own structures.
    /// 
    /// Ostensibly.
    pub fn as_nt_value<'a>(&'a self) -> NtValueDataGuard<'a> {
        type NtValueUnion = ntcore_sys::NT_Value__bindgen_ty_1;
        let mut data: NtValueUnion = NtValueUnion::default();
        let mut storage = ArrayStorage::None;
        
        match self {
            NtValueData::Unassigned => {}
            NtValueData::Boolean(v) => {
                data.v_boolean = *v as i32;
            }
            NtValueData::Double(v) => {
                data.v_double = *v;
            }
            NtValueData::String(s) => {
                data.v_string = WPI_String {
                    str_: s.as_ptr() as *const libc::c_char,
                    len: s.as_bytes().len(),
                };
            }
            NtValueData::Raw(items) | NtValueData::Rpc(items) => {
                data.v_raw = ntcore_sys::NT_Value__bindgen_ty_1__bindgen_ty_1 {
                    data: items.as_ptr() as *mut u8,
                    size: items.len(),
                };
            }
            NtValueData::BooleanArray(items) => {
                let arr: Vec<i32> = items.iter().map(|v| *v as i32).collect();
                data.arr_boolean = ntcore_sys::NT_Value__bindgen_ty_1__bindgen_ty_2 {
                    arr: arr.as_ptr() as *mut i32,
                    size: arr.len(),
                };
                storage = ArrayStorage::Boolean(arr);

            }
            NtValueData::DoubleArray(items) => {
                data.arr_double = ntcore_sys::NT_Value__bindgen_ty_1__bindgen_ty_3 {
                    arr: items.as_ptr() as *mut f64,
                    size: items.len(),
                };
            }
            NtValueData::StringArray(items) => {
                let arr: Vec<WPI_String> = items.iter().map(|v| {
                    WPI_String { str_: v.as_ptr() as *const libc::c_char, len: v.as_bytes().len() }
                }).collect();
                data.arr_string = ntcore_sys::NT_Value__bindgen_ty_1__bindgen_ty_6 {
                    arr: arr.as_ptr() as *mut WPI_String,
                    size: arr.len(),
                };
                storage = ArrayStorage::String(arr);
            }
            NtValueData::Integer(v) => {
                data.v_int = *v;
            }
            NtValueData::Float(v) => {
                data.v_float = *v;
            }
            NtValueData::IntegerArray(items) => {
                data.arr_int = ntcore_sys::NT_Value__bindgen_ty_1__bindgen_ty_5 {
                    arr: items.as_ptr() as *mut i64,
                    size: items.len(),
                };
            }
            NtValueData::FloatArray(items) => {
                data.arr_float = ntcore_sys::NT_Value__bindgen_ty_1__bindgen_ty_4 {
                    arr: items.as_ptr() as *mut f32,
                    size: items.len(),
                };
            }
        };


        NtValueDataGuard {
            _r: self,
            _storage: storage,
            nt_value: NT_Value {
                type_: self.value_type(),
                last_change: 0,
                server_time: 0,
                data,
            }
        }
    }

    pub fn value_type(&self) -> NtType {
        match self {
            Self::Unassigned => NtType::Unassigned,
            Self::Boolean(_) => NtType::Boolean,
            Self::Double(_) => NtType::Double,
            Self::String(_) => NtType::String,
            Self::Raw(_) => NtType::Raw,
            Self::BooleanArray(_) => NtType::BooleanArray,
            Self::DoubleArray(_) => NtType::DoubleArray,
            Self::StringArray(_) => NtType::StringArray,
            Self::Rpc(_) => NtType::Rpc,
            Self::Integer(_) => NtType::Integer,
            Self::Float(_) => NtType::Float,
            Self::IntegerArray(_) => NtType::IntegerArray,
            Self::FloatArray(_) => NtType::FloatArray
        }
    }
}

/// the whole point is to keep the underlying Vecs alive while the data guard is alive.
#[allow(unused)]
#[derive(Debug)]
enum ArrayStorage {
    None,
    Boolean(Vec<i32>),
    String(Vec<WPI_String>),
}

/// This is intended to be plugged into raw C calls.
/// 
/// # Safety
/// The returned [`NT_Value`] must not outlive the calling structure within FFI, and must not be mutated.
pub struct NtValueDataGuard<'a> {
    _r: &'a NtValueData,
    _storage: ArrayStorage,
    nt_value: NT_Value,
}

impl<'a> AsRef<NT_Value> for NtValueDataGuard<'a> {
    fn as_ref(&self) -> &NT_Value {
        &self.nt_value
    }
}

/// Iterator over an NT value queue
#[derive(Debug, PartialEq, Eq)]
pub struct NtValueQueueIter<'a> {
    q: &'a NtValueQueue,
    i: usize
}

impl<'a> Iterator for NtValueQueueIter<'a> {
    type Item = NtValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.q.count {
            None
        } else {
            let value = unsafe { self.q.ptr.add(self.i).read() };
            self.i += 1;
            Some(NtValue::from(&value))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NtValueQueue {
    ptr: *mut NT_Value,
    count: usize,
}

impl NtValueQueue {
    pub fn iter<'a>(&'a self) -> NtValueQueueIter<'a> {
        NtValueQueueIter { q: self, i: 0 }
    }
}

unsafe impl Send for NtValueQueue {}
impl Drop for NtValueQueue {
    fn drop(&mut self) {
        unsafe {
            ntcore_sys::NT_DisposeValueArray(self.ptr, self.count);
        }
    }
}

pub struct Handle {}

bitflags::bitflags! {
    pub struct NtEntryFlags : u32 {
        const PERSISTENT = ntcore_sys::NtEntryFlags::Persistent as u32;
        const RETAINED = ntcore_sys::NtEntryFlags::Retained as u32;
        const UNCACHED = ntcore_sys::NtEntryFlags::Uncached as u32;
    }
}

pub struct NtEntry(NT_Entry);

impl NtEntry {
    /// Gets the name of the entry.
    /// Returns [`None`] it's an invalid handle.
    pub fn name(&self) -> Option<WPIString> {
        let mut wpi_str = WPI_String::default();
        unsafe {
            ntcore_sys::NT_GetEntryName(self.0, &mut wpi_str);
        }
        let s = wpi_str.to_wpistring();
        none_if(s.is_empty(), s)
    }

    /// Gets the type for the specified key, if existant.
    pub fn entry_type(&self) -> NtType {
        unsafe { ntcore_sys::NT_GetEntryType(self.0) }
    }

    /// Gets the last time the entry was changed, or [`None`] if the handle is invalid.
    pub fn last_changed(&self) -> Option<u64> {
        let v = unsafe { ntcore_sys::NT_GetEntryLastChange(self.0) };
        none_if(v == 0, v)
    }

    /// Gets the value for the entry.
    /// 
    /// If unassigned or invalid, this will return a value with [`NtValueData::Unassigned`].
    pub fn value(&self) -> NtValue {
        let mut value = NT_Value::default();
        unsafe { ntcore_sys::NT_GetEntryValue(self.0, &mut value); }
        let ret = NtValue::from(&value);
        unsafe { ntcore_sys::NT_DisposeValue(&mut value); }
        ret
    }

    /// Sets the default entry value.
    /// 
    /// Returns true if success, false on "name already exists"
    /// 
    /// This is thread-safe.
    pub fn set_default_value(&self, value: &NtValueData) -> bool {
        unsafe {
            let guard = value.as_nt_value();
            ntcore_sys::NT_SetDefaultEntryValue(self.0, guard.as_ref()) != 0
        }
    }

    /// Sets the entry value.
    /// 
    /// Returns true if success, false on type mismatch.
    /// 
    /// This is thread-safe.
    pub fn set_value(&self, value: &NtValueData) -> bool {
        unsafe {
            let guard = value.as_nt_value();
            ntcore_sys::NT_SetEntryValue(self.0, guard.as_ref()) != 0
        }
    }

    /// Sets the entry flags set on this entry.
    pub fn set_flags(&self, flags: NtEntryFlags) {
        unsafe {
            ntcore_sys::NT_SetEntryFlags(self.0, flags.bits());
        }
    }

    /// Gets the entry flags set on this entry.
    pub fn get_flags(&self) -> NtEntryFlags {
        unsafe {
            NtEntryFlags::from_bits_retain(ntcore_sys::NT_GetEntryFlags(self.0))
        }
    }

    /// Returns new entry values since last call.
    pub fn read_queue_value(&self) -> NtValueQueue {
        // what the hell is an "subscriber or entry handle"
        // oh nvm lol
        let mut count = 0;
        unsafe {
            let ptr = ntcore_sys::NT_ReadQueueValue(self.0, &mut count);
            NtValueQueue { ptr, count }
        }
    }
}

pub struct NtInstance {
    handle: NT_Inst,
    droppable: bool
}

impl Default for NtInstance {
    fn default() -> Self {
        Self {
            handle: unsafe { ntcore_sys::NT_GetDefaultInstance() },
            droppable: false,
        }
    }
}

impl NtInstance {
    pub fn new() -> Self {
        Self {
            handle: unsafe { ntcore_sys::NT_CreateInstance() },
            droppable: true,
        }
    }
    // NT_GetInstanceFromHandle

    pub fn get_entry(&self, name: &str) -> Option<NtEntry> {
        let entry = unsafe {
            ntcore_sys::NT_GetEntry(self.handle, WPIString::from_str(name).to_wpistring_sys())
        };
        if entry != 0 {
            Some(NtEntry(entry))
        } else {
            None
        }
    }

    pub fn get_topics(&self, prefix: &str, types: impl Iterator<Item = NtType>) -> Vec<NtTopic> {
        let nt_type: libc::c_uint = types.fold(0, |acc, elem| {
            acc | elem as libc::c_uint
        });

        let prefix_str = WPIString::from_str(prefix);
        let mut count = 0;
        unsafe {
            let topics = ntcore_sys::NT_GetTopics(self.handle, prefix_str.to_wpistring_sys(), nt_type, &mut count);
            let ret = core::slice::from_raw_parts(topics, count).iter().cloned().map(NtTopic).collect()
            ntcore_sys::NT_DisposeTopicInfo(info);
            ret
        }
    }
}

pub struct NtTopic(NT_Topic);

impl NtTopic {
}

pub struct NtTopicInfo {
    pub topic: NtTopic,
    pub name: String,
    pub data_type: NtType,
    pub type_str: String,
    pub properties: String,
}