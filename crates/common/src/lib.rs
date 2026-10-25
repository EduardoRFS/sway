//! Shared runtime and compatibility interfaces for the translated programs.
#![feature(extern_types)]

pub mod gesture;
pub mod ipc;
pub mod socket_path;

pub mod ffi {
    mod allocation;
    // Generated compatibility code; maintained modules use normal Rust lints.
    #[allow(
        clippy::all,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code,
        unused_imports,
        unused_assignments,
        unused_mut,
        deprecated
    )]
    pub mod cairo;
    pub mod gesture;
    pub mod ipc_client;
    // Generated compatibility code; maintained modules use normal Rust lints.
    #[allow(
        clippy::all,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code,
        unused_imports,
        unused_assignments,
        unused_mut,
        deprecated
    )]
    pub mod list;
    // Generated compatibility code; maintained modules use normal Rust lints.
    #[allow(
        clippy::all,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code,
        unused_imports,
        unused_assignments,
        unused_mut,
        deprecated
    )]
    pub mod log;
    pub mod r#loop;
    // Generated compatibility code; maintained modules use normal Rust lints.
    #[allow(
        clippy::all,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code,
        unused_imports,
        unused_assignments,
        unused_mut,
        deprecated
    )]
    pub mod pango;
    // Generated compatibility code; maintained modules use normal Rust lints.
    #[allow(
        clippy::all,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code,
        unused_imports,
        unused_assignments,
        unused_mut,
        deprecated
    )]
    pub mod stringop;
    // Generated compatibility code; maintained modules use normal Rust lints.
    #[allow(
        clippy::all,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code,
        unused_imports,
        unused_assignments,
        unused_mut,
        deprecated
    )]
    pub mod util;
}
