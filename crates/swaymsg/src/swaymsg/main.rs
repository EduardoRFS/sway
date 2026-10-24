#![allow(unused_imports, unused_unsafe, stable_features, deprecated)]
#![allow(
    clippy::missing_safety_doc,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, raw_ref_op)]
#[macro_use]
extern crate c2rust_bitfields;
#[allow(unused_imports)]
use ::swaymsg as c2rust_ffi;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type json_object;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_is_type(
        obj: *const json_object,
        r#type: json_type,
    ) -> ::core::ffi::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_to_json_string_ext(
        obj: *mut json_object,
        flags: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn json_object_object_get(
        obj: *const json_object,
        key: *const ::core::ffi::c_char,
    ) -> *mut json_object;
    fn json_object_object_get_ex(
        obj: *const json_object,
        key: *const ::core::ffi::c_char,
        value: *mut *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_array_length(obj: *const json_object) -> size_t;
    fn json_object_array_get_idx(
        obj: *const json_object,
        idx: size_t,
    ) -> *mut json_object;
    fn json_object_get_boolean(obj: *const json_object) -> json_bool;
    fn json_object_get_int(obj: *const json_object) -> int32_t;
    fn json_object_get_double(obj: *const json_object) -> ::core::ffi::c_double;
    fn json_object_get_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_tokener_error_desc(jerr: json_tokener_error) -> *const ::core::ffi::c_char;
    fn json_tokener_get_error(tok: *mut json_tokener) -> json_tokener_error;
    fn json_tokener_new_ex(depth: ::core::ffi::c_int) -> *mut json_tokener;
    fn json_tokener_free(tok: *mut json_tokener);
    fn json_tokener_parse_ex(
        tok: *mut json_tokener,
        str: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> *mut json_object;
    fn join_args(
        argv: *mut *mut ::core::ffi::c_char,
        argc: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn get_socketpath() -> *mut ::core::ffi::c_char;
    fn ipc_open_socket(socket_path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn ipc_single_command(
        socketfd: ::core::ffi::c_int,
        r#type: uint32_t,
        payload: *const ::core::ffi::c_char,
        len: *mut uint32_t,
    ) -> *mut ::core::ffi::c_char;
    fn ipc_recv_response(socketfd: ::core::ffi::c_int) -> *mut ipc_response;
    fn free_ipc_response(response: *mut ipc_response);
    fn ipc_set_recv_timeout(socketfd: ::core::ffi::c_int, tv: timeval) -> bool;
    fn sway_log_init(verbosity: sway_log_importance_t, terminate: terminate_callback_t);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_abort(filename: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type int32_t = i32;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub bpos: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
}
pub type json_bool = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct json_type(pub ::core::ffi::c_uint);
impl json_type {
    pub const json_type_null: Self = Self(0);
    pub const json_type_boolean: Self = Self(1);
    pub const json_type_double: Self = Self(2);
    pub const json_type_int: Self = Self(3);
    pub const json_type_object: Self = Self(4);
    pub const json_type_array: Self = Self(5);
    pub const json_type_string: Self = Self(6);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct json_tokener_error(pub ::core::ffi::c_uint);
impl json_tokener_error {
    pub const json_tokener_success: Self = Self(0);
    pub const json_tokener_continue: Self = Self(1);
    pub const json_tokener_error_depth: Self = Self(2);
    pub const json_tokener_error_parse_eof: Self = Self(3);
    pub const json_tokener_error_parse_unexpected: Self = Self(4);
    pub const json_tokener_error_parse_null: Self = Self(5);
    pub const json_tokener_error_parse_boolean: Self = Self(6);
    pub const json_tokener_error_parse_number: Self = Self(7);
    pub const json_tokener_error_parse_array: Self = Self(8);
    pub const json_tokener_error_parse_object_key_name: Self = Self(9);
    pub const json_tokener_error_parse_object_key_sep: Self = Self(10);
    pub const json_tokener_error_parse_object_value_sep: Self = Self(11);
    pub const json_tokener_error_parse_string: Self = Self(12);
    pub const json_tokener_error_parse_comment: Self = Self(13);
    pub const json_tokener_error_parse_utf8_string: Self = Self(14);
    pub const json_tokener_error_size: Self = Self(15);
    pub const json_tokener_error_memory: Self = Self(16);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct json_tokener_state(pub ::core::ffi::c_uint);
impl json_tokener_state {
    pub const json_tokener_state_eatws: Self = Self(0);
    pub const json_tokener_state_start: Self = Self(1);
    pub const json_tokener_state_finish: Self = Self(2);
    pub const json_tokener_state_null: Self = Self(3);
    pub const json_tokener_state_comment_start: Self = Self(4);
    pub const json_tokener_state_comment: Self = Self(5);
    pub const json_tokener_state_comment_eol: Self = Self(6);
    pub const json_tokener_state_comment_end: Self = Self(7);
    pub const json_tokener_state_string: Self = Self(8);
    pub const json_tokener_state_string_escape: Self = Self(9);
    pub const json_tokener_state_escape_unicode: Self = Self(10);
    pub const json_tokener_state_escape_unicode_need_escape: Self = Self(11);
    pub const json_tokener_state_escape_unicode_need_u: Self = Self(12);
    pub const json_tokener_state_boolean: Self = Self(13);
    pub const json_tokener_state_number: Self = Self(14);
    pub const json_tokener_state_array: Self = Self(15);
    pub const json_tokener_state_array_add: Self = Self(16);
    pub const json_tokener_state_array_sep: Self = Self(17);
    pub const json_tokener_state_object_field_start: Self = Self(18);
    pub const json_tokener_state_object_field: Self = Self(19);
    pub const json_tokener_state_object_field_end: Self = Self(20);
    pub const json_tokener_state_object_value: Self = Self(21);
    pub const json_tokener_state_object_value_add: Self = Self(22);
    pub const json_tokener_state_object_sep: Self = Self(23);
    pub const json_tokener_state_array_after_sep: Self = Self(24);
    pub const json_tokener_state_object_field_start_after_sep: Self = Self(25);
    pub const json_tokener_state_inf: Self = Self(26);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_tokener_srec {
    pub state: json_tokener_state,
    pub saved_state: json_tokener_state,
    pub obj: *mut json_object,
    pub current: *mut json_object,
    pub obj_field_name: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_tokener {
    pub str: *mut ::core::ffi::c_char,
    pub pb: *mut printbuf,
    pub max_depth: ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
    pub is_double: ::core::ffi::c_int,
    pub st_pos: ::core::ffi::c_int,
    pub char_offset: ::core::ffi::c_int,
    pub err: json_tokener_error,
    pub ucs_char: ::core::ffi::c_uint,
    pub high_surrogate: ::core::ffi::c_uint,
    pub quote_char: ::core::ffi::c_char,
    pub stack: *mut json_tokener_srec,
    pub flags: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ipc_command_type(pub ::core::ffi::c_int);
impl ipc_command_type {
    pub const IPC_COMMAND: Self = Self(0);
    pub const IPC_GET_WORKSPACES: Self = Self(1);
    pub const IPC_SUBSCRIBE: Self = Self(2);
    pub const IPC_GET_OUTPUTS: Self = Self(3);
    pub const IPC_GET_TREE: Self = Self(4);
    pub const IPC_GET_MARKS: Self = Self(5);
    pub const IPC_GET_BAR_CONFIG: Self = Self(6);
    pub const IPC_GET_VERSION: Self = Self(7);
    pub const IPC_GET_BINDING_MODES: Self = Self(8);
    pub const IPC_GET_CONFIG: Self = Self(9);
    pub const IPC_SEND_TICK: Self = Self(10);
    pub const IPC_SYNC: Self = Self(11);
    pub const IPC_GET_BINDING_STATE: Self = Self(12);
    pub const IPC_GET_INPUTS: Self = Self(100);
    pub const IPC_GET_SEATS: Self = Self(101);
    pub const IPC_EVENT_WORKSPACE: Self = Self(-2147483648);
    pub const IPC_EVENT_OUTPUT: Self = Self(-2147483647);
    pub const IPC_EVENT_MODE: Self = Self(-2147483646);
    pub const IPC_EVENT_WINDOW: Self = Self(-2147483645);
    pub const IPC_EVENT_BARCONFIG_UPDATE: Self = Self(-2147483644);
    pub const IPC_EVENT_BINDING: Self = Self(-2147483643);
    pub const IPC_EVENT_SHUTDOWN: Self = Self(-2147483642);
    pub const IPC_EVENT_TICK: Self = Self(-2147483641);
    pub const IPC_EVENT_BAR_STATE_UPDATE: Self = Self(-2147483628);
    pub const IPC_EVENT_INPUT: Self = Self(-2147483627);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_response {
    pub size: uint32_t,
    pub r#type: uint32_t,
    pub payload: *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_log_importance_t(pub ::core::ffi::c_uint);
impl sway_log_importance_t {
    pub const SWAY_SILENT: Self = Self(0);
    pub const SWAY_ERROR: Self = Self(1);
    pub const SWAY_INFO: Self = Self(2);
    pub const SWAY_DEBUG: Self = Self(3);
    pub const SWAY_LOG_IMPORTANCE_LAST: Self = Self(4);
}
pub type terminate_callback_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub a: *const ::core::ffi::c_char,
    pub b: *const ::core::ffi::c_char,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_SPACED: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
    << 0 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_PRETTY: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
    << 1 as ::core::ffi::c_int;
pub const JSON_MAX_DEPTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
unsafe extern "C" fn success_object(mut result: *mut json_object) -> bool {
    let mut success_0: *mut json_object = ::core::ptr::null_mut::<json_object>();
    if json_object_object_get_ex(
        result,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut success_0,
    ) == 0
    {
        return r#true != 0;
    }
    return json_object_get_boolean(success_0) != 0;
}
unsafe extern "C" fn success(mut r: *mut json_object, mut fallback: bool) -> bool {
    if json_object_is_type(r, json_type::json_type_array) == 0 {
        if json_object_is_type(r, json_type::json_type_object) != 0 {
            return success_object(r);
        }
        return fallback;
    }
    let mut results_len: size_t = json_object_array_length(r);
    if results_len == 0 {
        return fallback;
    }
    let mut i: size_t = 0 as size_t;
    while i < results_len {
        let mut result: *mut json_object = json_object_array_get_idx(r, i);
        if !success_object(result) {
            return r#false != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn pretty_print_cmd(mut r: *mut json_object) {
    if !success_object(r) {
        let mut error: *mut json_object = ::core::ptr::null_mut::<json_object>();
        if json_object_object_get_ex(
            r,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut error,
        ) == 0
        {
            printf(
                b"An unknown error occurred\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            printf(
                b"Error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_get_string(error),
            );
        }
    }
}
unsafe extern "C" fn pretty_print_workspace(mut w: *mut json_object) {
    let mut name: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut rect: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut visible: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut output: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut urgent: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut layout: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut representation: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut focused: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json_object_object_get_ex(
        w,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut name,
    );
    json_object_object_get_ex(
        w,
        b"rect\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut rect,
    );
    json_object_object_get_ex(
        w,
        b"visible\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut visible,
    );
    json_object_object_get_ex(
        w,
        b"output\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut output,
    );
    json_object_object_get_ex(
        w,
        b"urgent\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut urgent,
    );
    json_object_object_get_ex(
        w,
        b"layout\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut layout,
    );
    json_object_object_get_ex(
        w,
        b"representation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut representation,
    );
    json_object_object_get_ex(
        w,
        b"focused\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut focused,
    );
    printf(
        b"Workspace %s%s%s%s\n  Output: %s\n  Layout: %s\n  Representation: %s\n\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        json_object_get_string(name),
        if json_object_get_boolean(focused) != 0 {
            b" (focused)\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if json_object_get_boolean(visible) == 0 {
            b" (off-screen)\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if json_object_get_boolean(urgent) != 0 {
            b" (urgent)\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        json_object_get_string(output),
        json_object_get_string(layout),
        json_object_get_string(representation),
    );
}
unsafe extern "C" fn pretty_type_name(
    mut name: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut type_names: [C2Rust_Unnamed; 7] = [
        C2Rust_Unnamed {
            a: b"keyboard\0".as_ptr() as *const ::core::ffi::c_char,
            b: b"Keyboard\0".as_ptr() as *const ::core::ffi::c_char,
        },
        C2Rust_Unnamed {
            a: b"pointer\0".as_ptr() as *const ::core::ffi::c_char,
            b: b"Pointer\0".as_ptr() as *const ::core::ffi::c_char,
        },
        C2Rust_Unnamed {
            a: b"touchpad\0".as_ptr() as *const ::core::ffi::c_char,
            b: b"Touchpad\0".as_ptr() as *const ::core::ffi::c_char,
        },
        C2Rust_Unnamed {
            a: b"tablet_pad\0".as_ptr() as *const ::core::ffi::c_char,
            b: b"Tablet pad\0".as_ptr() as *const ::core::ffi::c_char,
        },
        C2Rust_Unnamed {
            a: b"tablet_tool\0".as_ptr() as *const ::core::ffi::c_char,
            b: b"Tablet tool\0".as_ptr() as *const ::core::ffi::c_char,
        },
        C2Rust_Unnamed {
            a: b"touch\0".as_ptr() as *const ::core::ffi::c_char,
            b: b"Touch\0".as_ptr() as *const ::core::ffi::c_char,
        },
        C2Rust_Unnamed {
            a: b"switch\0".as_ptr() as *const ::core::ffi::c_char,
            b: b"Switch\0".as_ptr() as *const ::core::ffi::c_char,
        },
    ];
    let mut i: size_t = 0 as size_t;
    while i
        < ::core::mem::size_of::<[C2Rust_Unnamed; 7]>()
            .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed>())
    {
        if strcmp(type_names[i].a, name) == 0 as ::core::ffi::c_int {
            return type_names[i].b;
        }
        i = i.wrapping_add(1);
    }
    return name;
}
unsafe extern "C" fn pretty_print_input(mut i: *mut json_object) {
    let mut id: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut name: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut r#type: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut product: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut vendor: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut kbdlayout: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut libinput: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json_object_object_get_ex(
        i,
        b"identifier\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut id,
    );
    json_object_object_get_ex(
        i,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut name,
    );
    json_object_object_get_ex(
        i,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut r#type,
    );
    json_object_object_get_ex(
        i,
        b"product\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut product,
    );
    json_object_object_get_ex(
        i,
        b"vendor\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut vendor,
    );
    let mut fmt: *const ::core::ffi::c_char = b"Input device: %s\n  Type: %s\n  Identifier: %s\n  Product ID: %d\n  Vendor ID: %d\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    printf(
        fmt,
        json_object_get_string(name),
        pretty_type_name(json_object_get_string(r#type)),
        json_object_get_string(id),
        json_object_get_int(product),
        json_object_get_int(vendor),
    );
    if json_object_object_get_ex(
        i,
        b"xkb_active_layout_name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut kbdlayout,
    ) != 0
    {
        let mut layout: *const ::core::ffi::c_char = json_object_get_string(kbdlayout);
        printf(
            b"  Active Keyboard Layout: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            if !layout.is_null() {
                layout
            } else {
                b"(unnamed)\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    if json_object_object_get_ex(
        i,
        b"libinput\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut libinput,
    ) != 0
    {
        let mut events: *mut json_object = ::core::ptr::null_mut::<json_object>();
        if json_object_object_get_ex(
            libinput,
            b"send_events\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut events,
        ) != 0
        {
            printf(
                b"  Libinput Send Events: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_get_string(events),
            );
        }
    }
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn pretty_print_seat(mut i: *mut json_object) {
    let mut name: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut capabilities: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut devices: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json_object_object_get_ex(
        i,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut name,
    );
    json_object_object_get_ex(
        i,
        b"capabilities\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut capabilities,
    );
    json_object_object_get_ex(
        i,
        b"devices\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut devices,
    );
    let mut fmt: *const ::core::ffi::c_char = b"Seat: %s\n  Capabilities: %d\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    printf(fmt, json_object_get_string(name), json_object_get_int(capabilities));
    let mut devices_len: size_t = json_object_array_length(devices);
    if devices_len > 0 as size_t {
        printf(b"  Devices:\n\0".as_ptr() as *const ::core::ffi::c_char);
        let mut i_0: size_t = 0 as size_t;
        while i_0 < devices_len {
            let mut device: *mut json_object = json_object_array_get_idx(devices, i_0);
            let mut device_name: *mut json_object = ::core::ptr::null_mut::<
                json_object,
            >();
            json_object_object_get_ex(
                device,
                b"name\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut device_name,
            );
            printf(
                b"    %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_get_string(device_name),
            );
            i_0 = i_0.wrapping_add(1);
        }
    }
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn pretty_print_output(mut o: *mut json_object) {
    let mut name: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut rect: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut focused: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut active: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut power: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut ws: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut current_mode: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut non_desktop: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json_object_object_get_ex(
        o,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut name,
    );
    json_object_object_get_ex(
        o,
        b"rect\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut rect,
    );
    json_object_object_get_ex(
        o,
        b"focused\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut focused,
    );
    json_object_object_get_ex(
        o,
        b"active\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut active,
    );
    json_object_object_get_ex(
        o,
        b"power\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut power,
    );
    json_object_object_get_ex(
        o,
        b"current_workspace\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut ws,
    );
    json_object_object_get_ex(
        o,
        b"non_desktop\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut non_desktop,
    );
    let mut make: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut model: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut serial: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut scale: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut scale_filter: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut subpixel: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut transform: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut max_render_time: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut adaptive_sync_status: *mut json_object = ::core::ptr::null_mut::<
        json_object,
    >();
    let mut allow_tearing: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut hdr: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json_object_object_get_ex(
        o,
        b"make\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut make,
    );
    json_object_object_get_ex(
        o,
        b"model\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut model,
    );
    json_object_object_get_ex(
        o,
        b"serial\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut serial,
    );
    json_object_object_get_ex(
        o,
        b"scale\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut scale,
    );
    json_object_object_get_ex(
        o,
        b"scale_filter\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut scale_filter,
    );
    json_object_object_get_ex(
        o,
        b"subpixel_hinting\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut subpixel,
    );
    json_object_object_get_ex(
        o,
        b"transform\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut transform,
    );
    json_object_object_get_ex(
        o,
        b"max_render_time\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut max_render_time,
    );
    json_object_object_get_ex(
        o,
        b"adaptive_sync_status\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut adaptive_sync_status,
    );
    json_object_object_get_ex(
        o,
        b"allow_tearing\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut allow_tearing,
    );
    json_object_object_get_ex(
        o,
        b"hdr\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut hdr,
    );
    let mut x: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut y: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json_object_object_get_ex(
        rect,
        b"x\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut x,
    );
    json_object_object_get_ex(
        rect,
        b"y\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut y,
    );
    let mut modes: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json_object_object_get_ex(
        o,
        b"modes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut modes,
    );
    let mut width: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut height: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut refresh: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json_object_object_get_ex(
        o,
        b"current_mode\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut current_mode,
    );
    json_object_object_get_ex(
        current_mode,
        b"width\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut width,
    );
    json_object_object_get_ex(
        current_mode,
        b"height\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut height,
    );
    json_object_object_get_ex(
        current_mode,
        b"refresh\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut refresh,
    );
    let mut features: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut features_adaptive_sync: *mut json_object = ::core::ptr::null_mut::<
        json_object,
    >();
    let mut features_hdr: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json_object_object_get_ex(
        o,
        b"features\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut features,
    );
    json_object_object_get_ex(
        features,
        b"adaptive_sync\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut features_adaptive_sync,
    );
    json_object_object_get_ex(
        features,
        b"hdr\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut features_hdr,
    );
    if json_object_get_boolean(non_desktop) != 0 {
        printf(
            b"Output %s '%s %s %s' (non-desktop)\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            json_object_get_string(name),
            json_object_get_string(make),
            json_object_get_string(model),
            json_object_get_string(serial),
        );
    } else if json_object_get_boolean(active) != 0 {
        printf(
            b"Output %s '%s %s %s'%s\n  Current mode: %dx%d @ %.3f Hz\n  Power: %s\n  Position: %d,%d\n  Scale factor: %f\n  Scale filter: %s\n  Subpixel hinting: %s\n  Transform: %s\n  Workspace: %s\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            json_object_get_string(name),
            json_object_get_string(make),
            json_object_get_string(model),
            json_object_get_string(serial),
            if json_object_get_boolean(focused) != 0 {
                b" (focused)\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            json_object_get_int(width),
            json_object_get_int(height),
            json_object_get_int(refresh) as ::core::ffi::c_double
                / 1000 as ::core::ffi::c_int as ::core::ffi::c_double,
            if json_object_get_boolean(power) != 0 {
                b"on\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"off\0".as_ptr() as *const ::core::ffi::c_char
            },
            json_object_get_int(x),
            json_object_get_int(y),
            json_object_get_double(scale),
            json_object_get_string(scale_filter),
            json_object_get_string(subpixel),
            json_object_get_string(transform),
            json_object_get_string(ws),
        );
        let mut max_render_time_int: ::core::ffi::c_int = json_object_get_int(
            max_render_time,
        ) as ::core::ffi::c_int;
        printf(b"  Max render time: \0".as_ptr() as *const ::core::ffi::c_char);
        printf(
            if max_render_time_int == 0 as ::core::ffi::c_int {
                b"off\n\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"%d ms\n\0".as_ptr() as *const ::core::ffi::c_char
            },
            max_render_time_int,
        );
        printf(
            b"  Adaptive sync: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            if json_object_get_boolean(features_adaptive_sync) != 0 {
                json_object_get_string(adaptive_sync_status)
            } else {
                b"unsupported\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        printf(
            b"  Allow tearing: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            if json_object_get_boolean(allow_tearing) != 0 {
                b"yes\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"no\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        let mut hdr_str: *const ::core::ffi::c_char = b"unsupported\0".as_ptr()
            as *const ::core::ffi::c_char;
        if json_object_get_boolean(features_hdr) != 0 {
            hdr_str = if json_object_get_boolean(hdr) != 0 {
                b"on\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"off\0".as_ptr() as *const ::core::ffi::c_char
            };
        }
        printf(b"  HDR: %s\n\0".as_ptr() as *const ::core::ffi::c_char, hdr_str);
    } else {
        printf(
            b"Output %s '%s %s %s' (disabled)\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            json_object_get_string(name),
            json_object_get_string(make),
            json_object_get_string(model),
            json_object_get_string(serial),
        );
    }
    let mut modes_len: size_t = if json_object_is_type(modes, json_type::json_type_array)
        != 0
    {
        json_object_array_length(modes)
    } else {
        0 as size_t
    };
    if modes_len > 0 as size_t {
        printf(b"  Available modes:\n\0".as_ptr() as *const ::core::ffi::c_char);
        let mut i: size_t = 0 as size_t;
        while i < modes_len {
            let mut mode: *mut json_object = json_object_array_get_idx(modes, i);
            let mut mode_width: *mut json_object = ::core::ptr::null_mut::<
                json_object,
            >();
            let mut mode_height: *mut json_object = ::core::ptr::null_mut::<
                json_object,
            >();
            let mut mode_refresh: *mut json_object = ::core::ptr::null_mut::<
                json_object,
            >();
            let mut mode_picture_aspect_ratio: *mut json_object = ::core::ptr::null_mut::<
                json_object,
            >();
            json_object_object_get_ex(
                mode,
                b"width\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut mode_width,
            );
            json_object_object_get_ex(
                mode,
                b"height\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut mode_height,
            );
            json_object_object_get_ex(
                mode,
                b"refresh\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut mode_refresh,
            );
            json_object_object_get_ex(
                mode,
                b"picture_aspect_ratio\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut mode_picture_aspect_ratio,
            );
            printf(
                b"    %dx%d @ %.3f Hz\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_get_int(mode_width),
                json_object_get_int(mode_height),
                json_object_get_int(mode_refresh) as ::core::ffi::c_double
                    / 1000 as ::core::ffi::c_int as ::core::ffi::c_double,
            );
            if !mode_picture_aspect_ratio.is_null()
                && strcmp(
                    b"none\0".as_ptr() as *const ::core::ffi::c_char,
                    json_object_get_string(mode_picture_aspect_ratio),
                ) != 0 as ::core::ffi::c_int
            {
                printf(
                    b" (%s)\0".as_ptr() as *const ::core::ffi::c_char,
                    json_object_get_string(mode_picture_aspect_ratio),
                );
            }
            printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
            i = i.wrapping_add(1);
        }
    }
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn pretty_print_version(mut v: *mut json_object) {
    let mut ver: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json_object_object_get_ex(
        v,
        b"human_readable\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut ver,
    );
    printf(
        b"sway version %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_get_string(ver),
    );
}
unsafe extern "C" fn pretty_print_config(mut c: *mut json_object) {
    let mut config: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json_object_object_get_ex(
        c,
        b"config\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config,
    );
    printf(
        b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_get_string(config),
    );
}
unsafe extern "C" fn pretty_print_tree(
    mut obj: *mut json_object,
    mut indent: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < indent {
        printf(b"  \0".as_ptr() as *const ::core::ffi::c_char);
        i += 1;
    }
    let mut id: ::core::ffi::c_int = json_object_get_int(
        json_object_object_get(obj, b"id\0".as_ptr() as *const ::core::ffi::c_char),
    ) as ::core::ffi::c_int;
    let mut name: *const ::core::ffi::c_char = json_object_get_string(
        json_object_object_get(obj, b"name\0".as_ptr() as *const ::core::ffi::c_char),
    );
    let mut r#type: *const ::core::ffi::c_char = json_object_get_string(
        json_object_object_get(obj, b"type\0".as_ptr() as *const ::core::ffi::c_char),
    );
    let mut shell: *const ::core::ffi::c_char = json_object_get_string(
        json_object_object_get(obj, b"shell\0".as_ptr() as *const ::core::ffi::c_char),
    );
    printf(b"#%d: %s \"%s\"\0".as_ptr() as *const ::core::ffi::c_char, id, r#type, name);
    if !shell.is_null() {
        let mut pid: ::core::ffi::c_int = json_object_get_int(
            json_object_object_get(obj, b"pid\0".as_ptr() as *const ::core::ffi::c_char),
        ) as ::core::ffi::c_int;
        let mut app_id: *const ::core::ffi::c_char = json_object_get_string(
            json_object_object_get(
                obj,
                b"app_id\0".as_ptr() as *const ::core::ffi::c_char,
            ),
        );
        let mut window_props_obj: *mut json_object = json_object_object_get(
            obj,
            b"window_properties\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut instance: *const ::core::ffi::c_char = json_object_get_string(
            json_object_object_get(
                window_props_obj,
                b"instance\0".as_ptr() as *const ::core::ffi::c_char,
            ),
        );
        let mut class: *const ::core::ffi::c_char = json_object_get_string(
            json_object_object_get(
                window_props_obj,
                b"class\0".as_ptr() as *const ::core::ffi::c_char,
            ),
        );
        let mut x11_id: ::core::ffi::c_int = json_object_get_int(
            json_object_object_get(
                obj,
                b"window\0".as_ptr() as *const ::core::ffi::c_char,
            ),
        ) as ::core::ffi::c_int;
        let mut foreign_toplevel_id: *const ::core::ffi::c_char = json_object_get_string(
            json_object_object_get(
                obj,
                b"foreign_toplevel_identifier\0".as_ptr() as *const ::core::ffi::c_char,
            ),
        );
        let mut sandbox_engine: *const ::core::ffi::c_char = json_object_get_string(
            json_object_object_get(
                obj,
                b"sandbox_engine\0".as_ptr() as *const ::core::ffi::c_char,
            ),
        );
        let mut sandbox_app_id: *const ::core::ffi::c_char = json_object_get_string(
            json_object_object_get(
                obj,
                b"sandbox_app_id\0".as_ptr() as *const ::core::ffi::c_char,
            ),
        );
        let mut sandbox_instance_id: *const ::core::ffi::c_char = json_object_get_string(
            json_object_object_get(
                obj,
                b"sandbox_instance_id\0".as_ptr() as *const ::core::ffi::c_char,
            ),
        );
        printf(b" (%s, pid: %d\0".as_ptr() as *const ::core::ffi::c_char, shell, pid);
        if !app_id.is_null() {
            printf(b", app_id: \"%s\"\0".as_ptr() as *const ::core::ffi::c_char, app_id);
        }
        if !instance.is_null() {
            printf(
                b", instance: \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                instance,
            );
        }
        if !class.is_null() {
            printf(b", class: \"%s\"\0".as_ptr() as *const ::core::ffi::c_char, class);
        }
        if x11_id != 0 as ::core::ffi::c_int {
            printf(
                b", X11 window: 0x%X\0".as_ptr() as *const ::core::ffi::c_char,
                x11_id,
            );
        }
        if !foreign_toplevel_id.is_null() {
            printf(
                b", foreign_toplevel_id: \"%s\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                foreign_toplevel_id,
            );
        }
        if !sandbox_engine.is_null() {
            printf(
                b", sandbox_engine: \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                sandbox_engine,
            );
        }
        if !sandbox_app_id.is_null() {
            printf(
                b", sandbox_app_id: \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                sandbox_app_id,
            );
        }
        if !sandbox_instance_id.is_null() {
            printf(
                b", sandbox_instance_id: \"%s\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sandbox_instance_id,
            );
        }
        printf(b")\0".as_ptr() as *const ::core::ffi::c_char);
    }
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    let mut nodes_obj: *mut json_object = json_object_object_get(
        obj,
        b"nodes\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut len: size_t = json_object_array_length(nodes_obj);
    let mut i_0: size_t = 0 as size_t;
    while i_0 < len {
        pretty_print_tree(
            json_object_array_get_idx(nodes_obj, i_0),
            indent + 1 as ::core::ffi::c_int,
        );
        i_0 = i_0.wrapping_add(1);
    }
    let mut floating_nodes_obj: *mut json_object = ::core::ptr::null_mut::<
        json_object,
    >();
    let mut floating_nodes: json_bool = json_object_object_get_ex(
        obj,
        b"floating_nodes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut floating_nodes_obj,
    );
    if floating_nodes != 0 {
        let mut len_0: size_t = json_object_array_length(floating_nodes_obj);
        let mut i_1: size_t = 0 as size_t;
        while i_1 < len_0 {
            pretty_print_tree(
                json_object_array_get_idx(floating_nodes_obj, i_1),
                indent + 1 as ::core::ffi::c_int,
            );
            i_1 = i_1.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn pretty_print(
    mut r#type: ::core::ffi::c_int,
    mut resp: *mut json_object,
) {
    match r#type {
        10 => return,
        7 => {
            pretty_print_version(resp);
            return;
        }
        9 => {
            pretty_print_config(resp);
            return;
        }
        4 => {
            pretty_print_tree(resp, 0 as ::core::ffi::c_int);
            return;
        }
        0 | 1 | 100 | 3 | 101 => {}
        _ => {
            printf(
                b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_to_json_string_ext(
                    resp,
                    JSON_C_TO_STRING_PRETTY | JSON_C_TO_STRING_SPACED,
                ),
            );
            return;
        }
    }
    let mut obj: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut len: size_t = json_object_array_length(resp);
    let mut i: size_t = 0 as size_t;
    while i < len {
        obj = json_object_array_get_idx(resp, i) as *mut json_object;
        match r#type {
            0 => {
                pretty_print_cmd(obj);
            }
            1 => {
                pretty_print_workspace(obj);
            }
            100 => {
                pretty_print_input(obj);
            }
            3 => {
                pretty_print_output(obj);
            }
            101 => {
                pretty_print_seat(obj);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut quiet: bool = r#false != 0;
    static mut raw: bool = r#false != 0;
    static mut monitor: bool = r#false != 0;
    let mut socket_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut cmdtype: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    sway_log_init(sway_log_importance_t::SWAY_INFO, None);
    static mut long_options: [option; 9] = [
        option {
            name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'h' as ::core::ffi::c_int,
        },
        option {
            name: b"monitor\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'm' as ::core::ffi::c_int,
        },
        option {
            name: b"pretty\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'p' as ::core::ffi::c_int,
        },
        option {
            name: b"quiet\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'q' as ::core::ffi::c_int,
        },
        option {
            name: b"raw\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'r' as ::core::ffi::c_int,
        },
        option {
            name: b"socket\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 's' as ::core::ffi::c_int,
        },
        option {
            name: b"type\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 't' as ::core::ffi::c_int,
        },
        option {
            name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'v' as ::core::ffi::c_int,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
    ];
    let mut usage: *const ::core::ffi::c_char = b"Usage: swaymsg [options] [message]\n\n  -h, --help             Show help message and quit.\n  -m, --monitor          Monitor until killed (-t SUBSCRIBE only)\n  -p, --pretty           Use pretty output even when not using a tty\n  -q, --quiet            Be quiet.\n  -r, --raw              Use raw output even if using a tty\n  -s, --socket <socket>  Use the specified socket.\n  -t, --type <type>      Specify the message type.\n  -v, --version          Show the version number and quit.\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    raw = isatty(STDOUT_FILENO) == 0;
    let mut c: ::core::ffi::c_int = 0;
    loop {
        let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        c = getopt_long(
            argc,
            argv,
            b"hmpqrs:t:v\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const long_options as *const option,
            &raw mut option_index,
        );
        if c == -1 as ::core::ffi::c_int {
            break;
        }
        match c {
            109 => {
                monitor = r#true != 0;
            }
            112 => {
                raw = r#false != 0;
            }
            113 => {
                quiet = r#true != 0;
            }
            114 => {
                raw = r#true != 0;
            }
            115 => {
                socket_path = strdup(optarg);
            }
            116 => {
                cmdtype = strdup(optarg);
            }
            118 => {
                printf(
                    b"swaymsg version 1.13-dev\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                exit(EXIT_SUCCESS);
            }
            _ => {
                fprintf(stderr, b"%s\0".as_ptr() as *const ::core::ffi::c_char, usage);
                exit(EXIT_FAILURE);
            }
        }
    }
    if cmdtype.is_null() {
        cmdtype = strdup(b"command\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if socket_path.is_null() {
        socket_path = get_socketpath();
        if socket_path.is_null() {
            if quiet {
                exit(EXIT_FAILURE);
            }
            _sway_abort(
                b"[%s:%d] Unable to retrieve socket path\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaymsg/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                523 as ::core::ffi::c_int,
            );
        }
    }
    let mut r#type: uint32_t = ipc_command_type::IPC_COMMAND.0 as uint32_t;
    if strcasecmp(cmdtype, b"command\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_COMMAND.0 as uint32_t;
    } else if strcasecmp(
        cmdtype,
        b"get_workspaces\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_GET_WORKSPACES.0 as uint32_t;
    } else if strcasecmp(cmdtype, b"get_seats\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_GET_SEATS.0 as uint32_t;
    } else if strcasecmp(cmdtype, b"get_inputs\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_GET_INPUTS.0 as uint32_t;
    } else if strcasecmp(
        cmdtype,
        b"get_outputs\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_GET_OUTPUTS.0 as uint32_t;
    } else if strcasecmp(cmdtype, b"get_tree\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_GET_TREE.0 as uint32_t;
    } else if strcasecmp(cmdtype, b"get_marks\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_GET_MARKS.0 as uint32_t;
    } else if strcasecmp(
        cmdtype,
        b"get_bar_config\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_GET_BAR_CONFIG.0 as uint32_t;
    } else if strcasecmp(
        cmdtype,
        b"get_version\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_GET_VERSION.0 as uint32_t;
    } else if strcasecmp(
        cmdtype,
        b"get_binding_modes\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_GET_BINDING_MODES.0 as uint32_t;
    } else if strcasecmp(
        cmdtype,
        b"get_binding_state\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_GET_BINDING_STATE.0 as uint32_t;
    } else if strcasecmp(cmdtype, b"get_config\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_GET_CONFIG.0 as uint32_t;
    } else if strcasecmp(cmdtype, b"send_tick\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_SEND_TICK.0 as uint32_t;
    } else if strcasecmp(cmdtype, b"subscribe\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        r#type = ipc_command_type::IPC_SUBSCRIBE.0 as uint32_t;
    } else {
        if quiet {
            exit(EXIT_FAILURE);
        }
        _sway_abort(
            b"[%s:%d] Unknown message type %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaymsg/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            561 as ::core::ffi::c_int,
            cmdtype,
        );
    }
    free(cmdtype as *mut ::core::ffi::c_void);
    if monitor as ::core::ffi::c_int != 0
        && r#type != ipc_command_type::IPC_SUBSCRIBE.0 as uint32_t
    {
        if !quiet {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Monitor can only be used with -t SUBSCRIBE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaymsg/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                568 as ::core::ffi::c_int,
            );
        }
        free(socket_path as *mut ::core::ffi::c_void);
        return 1 as ::core::ffi::c_int;
    }
    let mut command: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if optind < argc {
        command = join_args(argv.offset(optind as isize), argc - optind);
    } else {
        command = strdup(b"\0".as_ptr() as *const ::core::ffi::c_char);
    }
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut socketfd: ::core::ffi::c_int = ipc_open_socket(socket_path);
    let mut timeout: timeval = timeval {
        tv_sec: 3 as __time_t,
        tv_usec: 0 as __suseconds_t,
    };
    ipc_set_recv_timeout(socketfd, timeout);
    let mut len: uint32_t = strlen(command) as uint32_t;
    let mut resp: *mut ::core::ffi::c_char = ipc_single_command(
        socketfd,
        r#type,
        command,
        &raw mut len,
    );
    let mut tok: *mut json_tokener = json_tokener_new_ex(JSON_MAX_DEPTH);
    if tok.is_null() {
        if quiet {
            exit(EXIT_FAILURE);
        }
        _sway_abort(
            b"[%s:%d] failed allocating json_tokener\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaymsg/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            594 as ::core::ffi::c_int,
        );
    }
    let mut obj: *mut json_object = json_tokener_parse_ex(
        tok,
        resp,
        -1 as ::core::ffi::c_int,
    );
    let mut err: json_tokener_error = json_tokener_get_error(tok);
    json_tokener_free(tok);
    if obj.is_null() || err.0 != json_tokener_error::json_tokener_success.0 {
        if !quiet {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] failed to parse payload as json: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaymsg/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                601 as ::core::ffi::c_int,
                json_tokener_error_desc(err),
            );
        }
        ret = 1 as ::core::ffi::c_int;
    } else {
        if !success(obj, r#true != 0) {
            ret = 2 as ::core::ffi::c_int;
        }
        if !quiet
            && (r#type != ipc_command_type::IPC_SUBSCRIBE.0 as uint32_t
                || ret != 0 as ::core::ffi::c_int)
        {
            if raw {
                printf(
                    b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    json_object_to_json_string_ext(
                        obj,
                        JSON_C_TO_STRING_PRETTY | JSON_C_TO_STRING_SPACED,
                    ),
                );
            } else {
                pretty_print(r#type as ::core::ffi::c_int, obj);
            }
        }
        json_object_put(obj);
    }
    free(command as *mut ::core::ffi::c_void);
    free(resp as *mut ::core::ffi::c_void);
    if r#type == ipc_command_type::IPC_SUBSCRIBE.0 as uint32_t
        && ret == 0 as ::core::ffi::c_int
    {
        timeout.tv_sec = 0 as __time_t;
        timeout.tv_usec = 0 as __suseconds_t;
        ipc_set_recv_timeout(socketfd, timeout);
        loop {
            let mut reply: *mut ipc_response = ipc_recv_response(socketfd);
            if reply.is_null() {
                break;
            }
            let mut tok_0: *mut json_tokener = json_tokener_new_ex(JSON_MAX_DEPTH);
            if tok_0.is_null() {
                if quiet {
                    exit(EXIT_FAILURE);
                }
                _sway_abort(
                    b"[%s:%d] failed allocating json_tokener\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaymsg/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                    639 as ::core::ffi::c_int,
                );
            }
            let mut obj_0: *mut json_object = json_tokener_parse_ex(
                tok_0,
                (*reply).payload,
                -1 as ::core::ffi::c_int,
            );
            let mut err_0: json_tokener_error = json_tokener_get_error(tok_0);
            json_tokener_free(tok_0);
            if obj_0.is_null() || err_0.0 != json_tokener_error::json_tokener_success.0 {
                if !quiet {
                    _sway_log(
                        sway_log_importance_t::SWAY_ERROR,
                        b"[%s:%d] failed to parse payload as json: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"swaymsg/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                        646 as ::core::ffi::c_int,
                        json_tokener_error_desc(err_0),
                    );
                }
                ret = 1 as ::core::ffi::c_int;
                break;
            } else {
                if quiet {
                    json_object_put(obj_0);
                } else {
                    if raw {
                        printf(
                            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            json_object_to_json_string(obj_0),
                        );
                    } else {
                        printf(
                            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            json_object_to_json_string_ext(
                                obj_0,
                                JSON_C_TO_STRING_PRETTY | JSON_C_TO_STRING_SPACED,
                            ),
                        );
                    }
                    fflush(stdout);
                    json_object_put(obj_0);
                }
                free_ipc_response(reply);
                if !monitor {
                    break;
                }
            }
        }
    }
    close(socketfd);
    free(socket_path as *mut ::core::ffi::c_void);
    return ret;
}
pub fn main() {
    // Match the C runtime's default SIGPIPE disposition.
    unsafe {
        extern "C" { fn signal(number: i32, handler: usize) -> usize; }
        signal(13, 0);
    }
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args_os()
        .map(|arg| {
            ::std::ffi::CString::new(::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()))
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(
            main_0(
                (args_ptrs.len() - 1) as ::core::ffi::c_int,
                args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
            ) as i32,
        )
    }
}
