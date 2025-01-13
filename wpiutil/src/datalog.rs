use std::{marker::PhantomData, path::Path};

use wpiutil_sys::{WPI_DataLog, WPI_DataLog_AppendBoolean, WPI_DataLog_AppendBooleanArrayByte, WPI_DataLog_AppendDouble, WPI_DataLog_AppendDoubleArray, WPI_DataLog_AppendFloat, WPI_DataLog_AppendFloatArray, WPI_DataLog_AppendInteger, WPI_DataLog_AppendIntegerArray, WPI_DataLog_AppendRaw, WPI_DataLog_AppendString, WPI_DataLog_AppendStringArray, WPI_DataLog_CreateBackgroundWriter, WPI_DataLog_CreateWriter, WPI_DataLog_Finish, WPI_DataLog_Flush, WPI_DataLog_Pause, WPI_DataLog_Release, WPI_DataLog_SetBackgroundWriterFilename, WPI_DataLog_SetMetadata, WPI_DataLog_Start, WPI_DataLog_Stop};

use crate::wpistring::WPIString;


#[derive(PartialEq, Eq, Debug)]
pub struct DataLog<T: DataLogKind> {
    p: *mut WPI_DataLog,
    _kind: PhantomData<T>
}

fn check_utf8<'a>(s: Option<&'a str>) -> Result<&'a str, std::io::Error> {
    match s {
        Some(s) => Ok(s),
        None => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Filename or directory is not UTF-8"))
    }
}

pub struct Writer;
impl DataLogKind for Writer {}
pub struct BackgroundWriter;
impl DataLogKind for BackgroundWriter {}
pub trait DataLogKind {}


impl<T: DataLogKind> DataLog<T> {
    pub fn new(filename: &Path, extra_header: Option<&str>) -> Result<DataLog<Writer>, std::io::Error> {
        let filename_wpi_str = WPIString::from_str(check_utf8(filename.to_str())?);
        let extra_header = WPIString::from_str(extra_header.unwrap_or(""));

        unsafe {
            let mut error_code = 0i32;
            let writer_ptr = WPI_DataLog_CreateWriter(
                filename_wpi_str.as_raw(),
                &mut error_code,
                extra_header.as_raw()
            );
            Ok(DataLog {
                p: writer_ptr,
                _kind: PhantomData,
            })
        }
    }

    pub fn new_background(directory: &Path, filename: Option<&Path>, period: f64, extra_header: Option<&str>) -> Result<DataLog<BackgroundWriter>, std::io::Error> {
        let filename_wpi_str = WPIString::from_str(
            check_utf8(filename.map_or_else(|| Some(""), |m| m.to_str()))?
        );
        let directory_wpi_str= WPIString::from_str(check_utf8(directory.to_str())?);
        let extra_header = WPIString::from_str(extra_header.unwrap_or(""));

        unsafe {
            let writer_ptr = WPI_DataLog_CreateBackgroundWriter(
                directory_wpi_str.as_raw(),
                filename_wpi_str.as_raw(),
                period,
                extra_header.as_raw()
            );
            Ok(DataLog {
                p: writer_ptr,
                _kind: PhantomData,
            })
        }
    }

    pub fn flush(&self) {
        unsafe { WPI_DataLog_Flush(self.p); }
    }

    pub fn pause(&self) {
        unsafe { WPI_DataLog_Pause(self.p); }
    }

    pub fn resume(&self) {
        unsafe { WPI_DataLog_Pause(self.p); }
    }

    pub fn stop(&self) {
        unsafe { WPI_DataLog_Stop(self.p); }
    }

    pub fn start<'a>(&'a self, name: &str, data_type: &str, metadata: &str, timestamp: Option<i64>) -> DataLogEntry<'a, T> {
        let index = unsafe {
            WPI_DataLog_Start(
                self.p,
                WPIString::from_str(name).as_raw(), 
                WPIString::from_str(data_type).as_raw(), 
                WPIString::from_str(metadata).as_raw(), 
                timestamp.unwrap_or(0)
            )
        };
        DataLogEntry { index, parent: self }
    }


}

impl DataLog<BackgroundWriter> {
    pub fn set_filename(&self, filename: &str) {
        unsafe {
            WPI_DataLog_SetBackgroundWriterFilename(self.p, WPIString::from_str(filename).as_raw());
        }
    }
}

impl<T: DataLogKind> Drop for DataLog<T> {
    fn drop(&mut self) {
        unsafe {
            WPI_DataLog_Release(self.p);
        }
    }
}

// SAFETY: WPILib headers explicitly say so.
unsafe impl<T: DataLogKind> Sync for DataLog<T> {}
unsafe impl<T: DataLogKind> Send for DataLog<T> {}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataLogEntry<'a, T: DataLogKind> {
    index: std::ffi::c_int,
    parent: &'a DataLog<T>
}

impl<'a, T: DataLogKind> DataLogEntry<'a, T> {
    pub fn finish(self, timestamp: Option<i64>) {
        unsafe {
            WPI_DataLog_Finish(self.parent.p, self.index, timestamp.unwrap_or(0));
        }
    }

    pub fn set_metadata(&self, metadata: &str, timestamp: Option<i64>) {
        unsafe {
            WPI_DataLog_SetMetadata(self.parent.p, self.index, WPIString::from_str(metadata).as_raw(), timestamp.unwrap_or(0));
        }
    }

    pub fn append_raw(&self, data: &[u8], timestamp: Option<i64>) {
        unsafe {
            WPI_DataLog_AppendRaw(self.parent.p, self.index, data.as_ptr(), data.len(), timestamp.unwrap_or(0));
        }
    }

    pub fn append_boolean(&self, value: bool, timestamp: Option<i64>) {
        unsafe {
            WPI_DataLog_AppendBoolean(self.parent.p, self.index, value as std::ffi::c_int, timestamp.unwrap_or(0));
        }
    }

    pub fn append_integer(&self, value: i64, timestamp: Option<i64>) {
        unsafe {
            WPI_DataLog_AppendInteger(self.parent.p, self.index, value, timestamp.unwrap_or(0));
        }
    }

    pub fn append_float(&self, value: f32, timestamp: Option<i64>) {
        unsafe {
            WPI_DataLog_AppendFloat(self.parent.p, self.index, value, timestamp.unwrap_or(0));
        }
    }

    pub fn append_double(&self, value: f64, timestamp: Option<i64>) {
        unsafe {
            WPI_DataLog_AppendDouble(self.parent.p, self.index, value, timestamp.unwrap_or(0));
        }
    }

    pub fn append_string(&self, value: &str, timestamp: Option<i64>) {
        unsafe {
            WPI_DataLog_AppendString(self.parent.p, self.index, WPIString::from_str(value).as_raw(), timestamp.unwrap_or(0));
        }
    }

    pub fn append_boolean_array(&self, value: &[bool], timestamp: Option<i64>) {
        unsafe {
            WPI_DataLog_AppendBooleanArrayByte(self.parent.p, self.index, value.as_ptr() as *const u8, value.len(), timestamp.unwrap_or(0));
        }
    }

    pub fn append_integer_array(&self, value: &[i64], timestamp: Option<i64>) {
        unsafe {
            WPI_DataLog_AppendIntegerArray(self.parent.p, self.index, value.as_ptr(), value.len(), timestamp.unwrap_or(0));
        }
    }

    pub fn append_float_array(&self, value: &[f32], timestamp: Option<i64>) {
        unsafe {
            WPI_DataLog_AppendFloatArray(self.parent.p, self.index, value.as_ptr(), value.len(), timestamp.unwrap_or(0));
        }
    }

    pub fn append_double_array(&self, value: &[f64], timestamp: Option<i64>) {
        unsafe {
            WPI_DataLog_AppendDoubleArray(self.parent.p, self.index, value.as_ptr(), value.len(), timestamp.unwrap_or(0));
        }
    }

    pub fn append_str_array(&self, value: &[&str], timestamp: Option<i64>) {
        let v: Vec<wpiutil_sys::WPI_String> = value.iter().map(|s| wpiutil_sys::WPI_String { 
            str_: s.as_ptr() as *const std::os::raw::c_char,
            len: s.as_bytes().len()
        }).collect();

        unsafe {
            WPI_DataLog_AppendStringArray(self.parent.p, self.index, v.as_ptr(), value.len(), timestamp.unwrap_or(0));
        }
    }

    pub fn append_string_array(&self, value: &[&String], timestamp: Option<i64>) {
        let v: Vec<wpiutil_sys::WPI_String> = value.iter().map(|s| wpiutil_sys::WPI_String { 
            str_: s.as_ptr() as *const std::os::raw::c_char,
            len: s.as_bytes().len()
        }).collect();

        unsafe {
            WPI_DataLog_AppendStringArray(self.parent.p, self.index, v.as_ptr(), value.len(), timestamp.unwrap_or(0));
        }
    }
}