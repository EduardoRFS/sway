//! Shared runtime and compatibility interfaces for the translated programs.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_imports, unused_assignments, unused_mut)]
#![allow(stable_features, deprecated)]
#![feature(extern_types)]

pub mod gesture;
pub mod ipc;
pub mod socket_path;

pub mod ffi {
    mod allocation;
    // Unmodified C2Rust output; style lints apply to the maintained modules.
    #[allow(clippy::all)]
    pub mod cairo;
    pub mod gesture;
    pub mod ipc_client;
    // Unmodified C2Rust output; style lints apply to the maintained modules.
    #[allow(clippy::all)]
    pub mod list;
    // Unmodified C2Rust output; style lints apply to the maintained modules.
    #[allow(clippy::all)]
    pub mod log;
    // Unmodified C2Rust output; style lints apply to the maintained modules.
    #[allow(clippy::all)]
    pub mod pango;
    pub mod r#loop;
    // Unmodified C2Rust output; style lints apply to the maintained modules.
    #[allow(clippy::all)]
    pub mod stringop;
    // Unmodified C2Rust output; style lints apply to the maintained modules.
    #[allow(clippy::all)]
    pub mod util;
}
