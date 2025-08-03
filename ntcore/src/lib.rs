//! # ntcore -- NetworkTables support.
//!
//! This directly wraps the NetworkTables `ntcore` library.
//!
//! ## Performance
//!
//! Given that at its core `ntcore` is a C++ library and this is a Rust library, there is a compromise made
//! for usability over performance.
//!
//! Namely, data returned from the C++ API will get copied into a [`Vec`], rather than directly used.
//!
//! This is still faster than using the C API, which tends towards undermaintained and ends up allocating an intermediate buffer
//! that we'd have to copy a 2nd time into a [`Vec`] or [`String`].
//!
//! If you want the absolute best possible performance, use a pure Rust library.
//!
//! ## Thread safety
//! All calls are thread safe, hence general use of `&self`.
//!
//! ## Memory safety
//!
//! Memory passed in and out through FFI should remain valid, and returned data should always be owned by Rust.
//!
//! Just...don't look too hard at the guts, alright?
#![warn(missing_docs)]

use std::mem::ManuallyDrop;

use ntcore_sys::{
    NTCoreRS_Value, NT_DestroyInstance, NT_Entry, NT_Handle, NT_Inst, NT_Topic, NT_Value,
    WPI_String,
};

macro_rules! new_vec {
    ($t: ty, $data:expr, $count:expr) => {{
        if $data.is_null() {
            return ManuallyDrop::new(vec![<$t>::default(); $count]).into();
        }
        let mut v = ManuallyDrop::new(Vec::<$t>::with_capacity($count));
        core::ptr::copy_nonoverlapping($data as *const $t, v.as_mut_ptr(), $count);
        v.set_len($count);
        v.into()
    }};
}

/// This function gets passed to the C++ shim to allocate and disassemble [`Vec`]s.
unsafe extern "C" fn ntcore_rs_alloc_vec(
    atype: ntcore_sys::NTCoreRS_AllocType,
    data: *const libc::c_void,
    count: usize,
) -> ntcore_sys::NTCoreRS_Vec {
    unsafe {
        match atype {
            ntcore_sys::NTCoreRS_AllocType::Bool => {
                if data.is_null() {
                    return ManuallyDrop::new(vec![bool::default(); count]).into();
                }
                let mut v = ManuallyDrop::new(Vec::<bool>::with_capacity(count));
                let data = data as *const ntcore_sys::NT_Bool;
                for (i, ent) in v.spare_capacity_mut().iter_mut().enumerate() {
                    ent.write(data.add(i).read() != 0);
                }
                v.set_len(count);
                v.into()
            }
            ntcore_sys::NTCoreRS_AllocType::Double => new_vec!(f64, data, count),
            ntcore_sys::NTCoreRS_AllocType::Char => new_vec!(libc::c_char, data, count),
            ntcore_sys::NTCoreRS_AllocType::Integer => new_vec!(i64, data, count),
            ntcore_sys::NTCoreRS_AllocType::Float => new_vec!(f32, data, count),
            ntcore_sys::NTCoreRS_AllocType::String => {
                if data.is_null() {
                    return ManuallyDrop::new(vec![String::default(); count]).into();
                }
                let mut v = ManuallyDrop::new(Vec::<String>::with_capacity(count));
                let data = data as *const ntcore_sys::WPI_String;
                for (i, ent) in v.spare_capacity_mut().iter_mut().enumerate() {
                    let v = data.add(i).read();
                    let s = str::from_utf8_unchecked(core::slice::from_raw_parts(
                        v.str_ as *const u8,
                        v.len,
                    ))
                    .to_string();
                    ent.write(s);
                }
                v.set_len(count);
                v.into()
            }
            ntcore_sys::NTCoreRS_AllocType::Value => {
                new_vec!(ntcore_sys::NTCoreRS_Value, data, count)
            }
            ntcore_sys::NTCoreRS_AllocType::Handle => new_vec!(ntcore_sys::NT_Handle, data, count),
        }
    }
}

unsafe extern "C" fn ntcore_rs_readqueue_construct(
    conv: ntcore_sys::NTCoreRS_Value_Convert,
    arr: *const libc::c_void,
    count: usize,
) -> ntcore_sys::NTCoreRS_Vec {
    let mut v = ManuallyDrop::new(Vec::<NtValue>::with_capacity(count));
    let arr = arr as *const ntcore_sys::NTCoreRS_Value;
    let Some(conv) = conv else {
        return ntcore_sys::NTCoreRS_Vec {
            data: v.as_mut_ptr() as *mut libc::c_char,
            len: v.len(),
            capacity: v.capacity(),
        };
    };
    for (i, ent) in v.spare_capacity_mut().iter_mut().enumerate() {
        ent.write(conv(VEC_ALLOC, arr.add(i) as *const libc::c_void).into());
    }

    ntcore_sys::NTCoreRS_Vec {
        data: v.as_mut_ptr() as *mut libc::c_char,
        len: v.len(),
        capacity: v.capacity(),
    }
}

unsafe extern "C" fn ntcore_rs_insttopicinfo_construct(
    conv: ntcore_sys::NTCoreRS_TopicInfo_Convert,
    arr: *const libc::c_void,
    count: usize,
) -> ntcore_sys::NTCoreRS_Vec {
    let mut v = ManuallyDrop::new(Vec::<NtTopicInfo>::with_capacity(count));
    let arr = arr as *const ntcore_sys::NTCoreRS_Value;
    let Some(conv) = conv else {
        return ntcore_sys::NTCoreRS_Vec {
            data: v.as_mut_ptr() as *mut libc::c_char,
            len: v.len(),
            capacity: v.capacity(),
        };
    };
    for (i, ent) in v.spare_capacity_mut().iter_mut().enumerate() {
        ent.write(conv(VEC_ALLOC, arr.add(i) as *const libc::c_void).into());
    }

    ntcore_sys::NTCoreRS_Vec {
        data: v.as_mut_ptr() as *mut libc::c_char,
        len: v.len(),
        capacity: v.capacity(),
    }
}

const VEC_ALLOC: ntcore_sys::NTCoreRS_Allocator = Some(ntcore_rs_alloc_vec);

fn none_if<T>(predicate: bool, value: T) -> Option<T> {
    if predicate {
        None
    } else {
        Some(value)
    }
}

pub use ntcore_sys::NtType;

/// Holds a NetworkTables value.
#[derive(Debug, Clone, PartialEq)]
pub struct NtValue {
    /// The last time the value was changed in milliseconds.
    pub last_change: i64,
    /// The server time in milliseconds.
    pub server_time: i64,
    /// The actual value of the data.
    pub value: NtValueData,
}

impl From<NTCoreRS_Value> for NtValue {
    fn from(value: NTCoreRS_Value) -> Self {
        let ret = Self {
            last_change: value.last_change,
            server_time: value.server_time,
            value: unsafe {
                match value.type_ {
                    NtType::Unassigned => NtValueData::Unassigned,
                    NtType::Boolean => NtValueData::Boolean(value.data.v_boolean),
                    NtType::Double => NtValueData::Double(value.data.v_double),
                    NtType::String => NtValueData::String(value.data.buf.into()),
                    NtType::Raw => NtValueData::Raw(value.data.buf.into()),
                    NtType::BooleanArray => NtValueData::BooleanArray(value.data.buf.into()),
                    NtType::DoubleArray => NtValueData::DoubleArray(value.data.buf.into()),
                    NtType::StringArray => NtValueData::StringArray(value.data.buf.into()),
                    NtType::Rpc => NtValueData::Rpc(value.data.buf.into()),
                    NtType::Integer => NtValueData::Integer(value.data.v_int),
                    NtType::Float => NtValueData::Float(value.data.v_float),
                    NtType::IntegerArray => NtValueData::IntegerArray(value.data.buf.into()),
                    NtType::FloatArray => NtValueData::FloatArray(value.data.buf.into()),
                }
            },
        };
        ret
    }
}

/// NetworkTables value data, in Rust types.
/// These do not hold any foreign memory.
#[derive(Debug, Clone, PartialEq)]
pub enum NtValueData {
    /// Unassigned/invalid
    Unassigned,
    /// boolean
    Boolean(bool),
    /// double
    Double(f64),
    /// string
    String(String),
    /// raw vector
    Raw(Vec<u8>),
    /// boolean array
    BooleanArray(Vec<bool>),
    /// double array
    DoubleArray(Vec<f64>),
    /// string array
    StringArray(Vec<String>),
    /// RPC data
    Rpc(Vec<u8>),
    /// int64 (not supported on all implementations)
    Integer(i64),
    /// float32 (not supported on all implementations)
    Float(f32),
    /// int64 array
    IntegerArray(Vec<i64>),
    /// float array
    FloatArray(Vec<f32>),
}

impl NtValueData {
    /// Creates a raw [`NT_Value`] structure for use when setting data, and returns a guard to ensure its validity.
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
                let arr: Vec<WPI_String> = items
                    .iter()
                    .map(|v| WPI_String {
                        str_: v.as_ptr() as *const libc::c_char,
                        len: v.as_bytes().len(),
                    })
                    .collect();
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
                type_: self.nt_type(),
                last_change: 0,
                server_time: 0,
                data,
            },
        }
    }

    /// Returns the [`NtType`] of the data.
    pub fn nt_type(&self) -> NtType {
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
            Self::FloatArray(_) => NtType::FloatArray,
        }
    }
}

/// Holds vec data for NT_Value
#[allow(unused)]
#[derive(Debug)]
enum ArrayStorage {
    None,
    Boolean(Vec<i32>),
    String(Vec<WPI_String>),
}

/// Ensures that a referentiable [`NT_Value`] holds valid pointers.
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

bitflags::bitflags! {
    /// Bitflag of NT entry params.
    pub struct NtEntryFlags : u32 {
        /// Persistent
        const PERSISTENT = ntcore_sys::NtEntryFlags::Persistent as u32;
        /// Retained
        const RETAINED = ntcore_sys::NtEntryFlags::Retained as u32;
        /// Uncached
        const UNCACHED = ntcore_sys::NtEntryFlags::Uncached as u32;
    }
}

bitflags::bitflags! {
    /// Bitflag of possible [`NtType`]s that could be considered valid.
    pub struct NtTypeSet: u32 {
        /// Unassigned
        const UNASSIGNED = ntcore_sys::NtType::Unassigned as u32;
        /// Boolean
        const BOOLEAN = ntcore_sys::NtType::Boolean as u32;
        /// Double
        const DOUBLE = ntcore_sys::NtType::Double as u32;
        /// String
        const STRING = ntcore_sys::NtType::String as u32;
        /// Raw
        const RAW = ntcore_sys::NtType::Raw as u32;
        /// Boolean array
        const BOOLEAN_ARRAY = ntcore_sys::NtType::BooleanArray as u32;
        /// Double array
        const DOUBLE_ARRAY = ntcore_sys::NtType::DoubleArray as u32;
        /// String array
        const STRING_ARRAY = ntcore_sys::NtType::StringArray as u32;
        /// RPC
        const RPC = ntcore_sys::NtType::Rpc as u32;
        /// Int64
        const INTEGER = ntcore_sys::NtType::Integer as u32;
        /// Float
        const FLOAT = ntcore_sys::NtType::Float as u32;
        /// Integer array
        const INTEGER_ARRAY = ntcore_sys::NtType::IntegerArray as u32;
        /// Float array
        const FLOAT_ARRAY = ntcore_sys::NtType::FloatArray as u32;
    }
}

/// A NetworkTables sentry.
#[repr(transparent)]
pub struct NtEntry(NT_Entry);

impl NtEntry {
    /// Gets the name of the entry.
    /// Returns [`None`] it's an invalid handle.
    pub fn name(&self) -> Option<String> {
        let s: String = unsafe { ntcore_sys::NTCoreRS_GetEntryName(self.0, VEC_ALLOC).into() };
        none_if(s.is_empty(), s)
    }

    /// Gets the type for the specified key, if existant.
    pub fn entry_type(&self) -> NtType {
        // thin wrapper over nt::GetEntryType
        unsafe { ntcore_sys::NT_GetEntryType(self.0) }
    }

    /// Gets the last time the entry was changed, or [`None`] if the handle is invalid.
    pub fn last_changed(&self) -> Option<u64> {
        // thin wrapper over nt::GetEntryLastChange
        let v = unsafe { ntcore_sys::NT_GetEntryLastChange(self.0) };
        none_if(v == 0, v)
    }

    /// Gets the value for the entry.
    ///
    /// If unassigned or invalid, this will return a value with [`NtValueData::Unassigned`].
    pub fn value(&self) -> NtValue {
        unsafe { ntcore_sys::NTCoreRS_GetEntryValue(self.0, VEC_ALLOC).into() }
    }

    /// Sets the default entry value.
    ///
    /// Returns true if success, false on "name already exists"
    ///
    /// This is thread-safe.
    pub fn set_default_value(&self, value: &NtValueData) -> bool {
        // existentially ok with using ConvertFromC
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
        unsafe { NtEntryFlags::from_bits_retain(ntcore_sys::NT_GetEntryFlags(self.0)) }
    }

    /// Returns new entry values since last call.
    pub fn read_queue_value(&self) -> Vec<NtValue> {
        unsafe {
            let ret = ntcore_sys::NTCoreRS_ReadQueueValue(
                self.0,
                0,
                VEC_ALLOC,
                Some(ntcore_rs_readqueue_construct),
            );
            Vec::from_raw_parts(ret.data as *mut NtValue, ret.len, ret.capacity)
        }
    }

    pub fn exists(&self) -> bool {
        unsafe { ntcore_sys::NT_GetTopicExists(self.0) != 0 }
    }
}

fn str_pair(s: &str) -> (*const libc::c_char, usize) {
    (s.as_ptr() as *const libc::c_char, s.len())
}

#[derive(Debug, PartialEq, Eq)]
pub struct NtInstance {
    handle: NT_Inst,
    droppable: bool,
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

    /// Getes an entry by name, if it exists
    pub fn get_entry(&self, name: &str) -> Option<NtEntry> {
        let entry = unsafe {
            let (name_ptr, name_len) = str_pair(name);
            ntcore_sys::NTCoreRS_GetEntry(self.handle, name_ptr, name_len)
        };
        if entry != 0 {
            Some(NtEntry(entry))
        } else {
            None
        }
    }

    pub fn get_topic(&self, name: &str) -> Option<NtTopic> {
        let topic = unsafe {
            let (name_ptr, name_len) = str_pair(name);
            ntcore_sys::NTCoreRS_GetTopic(self.handle, name_ptr, name_len)
        };
        if topic != 0 {
            Some(NtTopic(topic))
        } else {
            None
        }
    }

    /// Gets topics matching the name prefix and type set
    pub fn get_topics(&self, prefix: &str, types: NtTypeSet) -> Vec<NtTopic> {
        unsafe {
            let (prefix_ptr, prefix_len) = str_pair(prefix);
            let res: Vec<NT_Handle> = ntcore_sys::NTCoreRS_GetTopics(
                self.handle,
                prefix_ptr,
                prefix_len,
                types.bits(),
                VEC_ALLOC,
            )
            .into();
            // SAFETY: NT_Handle and NtTopic are the same repr.
            core::mem::transmute(res)
        }
    }

    /// Gets topics matching the name prefix and type strings
    pub fn get_topics_str(
        &self,
        prefix: &str,
        types: impl IntoIterator<Item: AsRef<str>>,
    ) -> Vec<NtTopic> {
        let types_vec: Vec<WPI_String> = types
            .into_iter()
            .map(|s| {
                let (str_, len) = str_pair(s.as_ref());
                WPI_String { str_, len }
            })
            .collect();

        unsafe {
            let (prefix_ptr, prefix_len) = str_pair(prefix);
            let res: Vec<NT_Handle> = ntcore_sys::NTCoreRS_GetTopicsStr(
                self.handle,
                prefix_ptr,
                prefix_len,
                types_vec.as_ptr(),
                types_vec.len(),
                VEC_ALLOC,
            )
            .into();
            // SAFETY: NT_Handle and NtTopic are the same repr.
            core::mem::transmute(res)
        }
    }

    /// Gets topic info matching the name prefix and type set
    ///
    /// This double-allocates but honestly it doesn't matter given how much the function overall allocates
    pub fn get_topic_infos(&self, prefix: &str, types: NtTypeSet) -> Vec<NtTopicInfo> {
        unsafe {
            let (prefix_ptr, prefix_len) = str_pair(prefix);
            let res: Vec<ntcore_sys::NTCoreRS_TopicInfo> = ntcore_sys::NTCoreRS_GetTopicInfos(
                self.handle,
                prefix_ptr,
                prefix_len,
                types.bits(),
                VEC_ALLOC,
                Some(ntcore_rs_insttopicinfo_construct),
            )
            .into();
            res.into_iter().map(Into::into).collect()
        }
    }

    /// Gets topic info matching the name prefix and type strings
    ///
    /// This double-allocates but honestly it doesn't matter given how much the function overall allocates
    pub fn get_topic_infos_str(
        &self,
        prefix: &str,
        types: impl IntoIterator<Item: AsRef<str>>,
    ) -> Vec<NtTopicInfo> {
        let types_vec: Vec<WPI_String> = types
            .into_iter()
            .map(|s| {
                let (str_, len) = str_pair(s.as_ref());
                WPI_String { str_, len }
            })
            .collect();
        unsafe {
            let (prefix_ptr, prefix_len) = str_pair(prefix);
            let res: Vec<ntcore_sys::NTCoreRS_TopicInfo> = ntcore_sys::NTCoreRS_GetTopicInfosStr(
                self.handle,
                prefix_ptr,
                prefix_len,
                types_vec.as_ptr(),
                types_vec.len(),
                VEC_ALLOC,
                Some(ntcore_rs_insttopicinfo_construct),
            )
            .into();
            res.into_iter().map(Into::into).collect()
        }
    }
}

impl Drop for NtInstance {
    fn drop(&mut self) {
        if self.droppable {
            unsafe {
                NT_DestroyInstance(self.handle);
            }
        }
    }
}

pub struct NtTopicInfo {
    /// topic handle
    pub topic: NtTopic,
    /// name
    pub name: String,
    /// data type
    pub data_type: NtType,
    /// type string
    pub type_str: String,
    /// properties (json)
    pub properties: String,
}

impl From<ntcore_sys::NTCoreRS_TopicInfo> for NtTopicInfo {
    fn from(value: ntcore_sys::NTCoreRS_TopicInfo) -> Self {
        unsafe {
            Self {
                topic: NtTopic(value.topic),
                name: String::from_raw_parts(
                    value.name.data as *mut u8,
                    value.name.len,
                    value.name.capacity,
                ),
                data_type: value.type_,
                type_str: String::from_raw_parts(
                    value.type_str.data as *mut u8,
                    value.type_str.len,
                    value.type_str.capacity,
                ),
                properties: String::from_raw_parts(
                    value.properties.data as *mut u8,
                    value.properties.len,
                    value.properties.capacity,
                ),
            }
        }
    }
}

#[repr(transparent)]
pub struct NtTopic(NT_Topic);

impl NtTopic {
    pub fn get_topic_info(&self) -> NtTopicInfo {
        unsafe { ntcore_sys::NTCoreRS_GetTopicInfo(self.0, VEC_ALLOC).into() }
    }

    pub fn name(&self) -> String {
        unsafe { ntcore_sys::NTCoreRS_GetTopicName(self.0, VEC_ALLOC).into() }
    }

    pub fn nt_type(&self) -> NtType {
        unsafe { ntcore_sys::NT_GetTopicType(self.0) }
    }

    pub fn nt_type_string(&self) -> String {
        unsafe { ntcore_sys::NT_GetTopicTypeString(self.0, VEC_ALLOC).into() }
    }

    pub fn persistant(&self) -> bool {
        unsafe { ntcore_sys::NT_GetTopicPersistent(self.0) != 0 }
    }

    pub fn retained(&self) -> bool {
        unsafe { ntcore_sys::NT_GetTopicRetained(self.0) != 0 }
    }

    pub fn cached(&self) -> bool {
        unsafe { ntcore_sys::NT_GetTopicCached(self.0) != 0 }
    }

    pub fn set_persistant(&self, v: bool) {
        unsafe {
            ntcore_sys::NT_SetTopicPersistent(self.0, v as i32);
        }
    }

    pub fn set_retained(&self, v: bool) {
        unsafe {
            ntcore_sys::NT_SetTopicRetained(self.0, v as i32);
        }
    }

    pub fn set_cached(&self, v: bool) {
        unsafe {
            ntcore_sys::NT_SetTopicCached(self.0, v as i32);
        }
    }

    pub fn exists(&self) -> bool {
        unsafe { ntcore_sys::NT_GetTopicExists(self.0) != 0 }
    }
    // get, set, delete, property
    // get all properties, set all properties
    // need to also version buildlibs so it doesn't stomp on multiple versions of same crate
}
