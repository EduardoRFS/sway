#![allow(unused_imports, unused_unsafe, stable_features, deprecated)]
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(raw_ref_op)]

#[macro_use]
extern crate c2rust_bitfields;

pub mod src {
pub mod common {
    pub use sway_common::ffi::*;
} // mod common
} // mod src
