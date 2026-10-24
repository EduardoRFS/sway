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
pub mod build {
pub mod reference {
pub mod protocols {
pub mod cursor_shape_v1_protocol;
pub mod tablet_v2_protocol;
pub mod wlr_layer_shell_unstable_v1_protocol;
pub mod wlr_output_power_management_unstable_v1_protocol;
pub mod xdg_output_unstable_v1_protocol;
pub mod xdg_shell_protocol;
} // mod protocols
} // mod reference
} // mod build
pub mod original {
pub mod client {
pub mod pool_buffer;
} // mod client
pub mod common {
pub mod cairo;
pub mod gesture;
pub mod ipc_client;
pub mod list;
pub mod log;
pub mod pango;
pub mod r#loop;
pub mod stringop;
pub mod util;
} // mod common
pub mod swaynag {
pub mod config;
pub mod render;
pub mod swaynag;
pub mod types;
} // mod swaynag
} // mod original
} // mod src
