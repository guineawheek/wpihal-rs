#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(unused)]

pub mod usage_reporting;

include!(concat!(env!("OUT_DIR"), "/hal_bindings.rs"));