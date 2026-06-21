#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(unused)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl WPI_String {
    /// # Safety
    /// The buffer the [`WPI_String`] points to must stay alive for the lifetime of the instance.
    pub const unsafe fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(core::slice::from_raw_parts(
                self.str_ as *const u8,
                self.len,
            ))
        }
    }
}
