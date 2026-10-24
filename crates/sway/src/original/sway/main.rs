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
use ::sway as c2rust_ffi;
extern "C" {
    pub type _PangoFontDescription;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _PangoCairoFontMap;
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_buffer_impl;
    pub type wlr_renderer_impl;
    pub type wlr_texture_impl;
    pub type wlr_allocator;
    pub type wlr_backend_impl;
    pub type wlr_linux_dmabuf_feedback_v1_compiled;
    pub type wlr_color_manager_v1;
    pub type wlr_gamma_control_manager_v1;
    pub type sway_workspace;
    pub type sway_text_node;
    pub type sway_view;
    pub type sway_transaction_instruction;
    pub type sway_output;
    pub type sway_seat;
    pub type wlr_xwm;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_keyboard_group;
    pub type wlr_keyboard_impl;
    pub type wlr_primary_selection_source;
    pub type wlr_xwayland_shell_v1;
    pub type wlr_xcursor_manager;
    pub type sway_transaction;
    pub type wlr_session_lock_v1;
    pub type wlr_ext_workspace_manager_v1;
    pub type wlr_tearing_control_manager_v1;
    pub type wlr_xdg_activation_v1;
    pub type wlr_ext_foreign_toplevel_image_capture_source_manager_v1;
    pub type wlr_security_context_manager_v1;
    pub type wlr_export_dmabuf_manager_v1;
    pub type wlr_ext_image_copy_capture_manager_v1;
    pub type wlr_screencopy_manager_v1;
    pub type wlr_ext_data_control_manager_v1;
    pub type wlr_data_control_manager_v1;
    pub type wlr_content_type_manager_v1;
    pub type wlr_foreign_toplevel_manager_v1;
    pub type wlr_ext_foreign_toplevel_list_v1;
    pub type wlr_text_input_manager_v3;
    pub type wlr_input_method_manager_v2;
    pub type wlr_output_power_manager_v1;
    pub type wlr_session_lock_manager_v1;
    pub type wlr_output_manager_v1;
    pub type wlr_xdg_output_manager_v1;
    pub type wlr_pointer_constraints_v1;
    pub type wlr_drm_lease_v1_manager;
    pub type wlr_xdg_decoration_manager_v1;
    pub type wlr_server_decoration_manager;
    pub type wlr_relative_pointer_manager_v1;
    pub type wlr_tablet_manager_v2;
    pub type wlr_xdg_shell;
    pub type wlr_idle_notifier_v1;
    pub type wlr_pointer_gestures_v1;
    pub type wlr_data_device_manager;
    pub type wlr_session;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigprocmask(
        __how: ::core::ffi::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> ::core::ffi::c_int;
    fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn pthread_atfork(
        __prepare: Option<unsafe extern "C" fn() -> ()>,
        __parent: Option<unsafe extern "C" fn() -> ()>,
        __child: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn getline(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn pclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn popen(
        __command: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn pango_cairo_font_map_set_default(fontmap: *mut PangoCairoFontMap);
    fn getrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *mut rlimit,
    ) -> ::core::ffi::c_int;
    fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn wlr_log_init(verbosity: wlr_log_importance, callback: wlr_log_func_t);
    fn wl_event_loop_add_signal(
        r#loop: *mut wl_event_loop,
        signal_number: ::core::ffi::c_int,
        func: wl_event_loop_signal_func_t,
        data: *mut ::core::ffi::c_void,
    ) -> *mut wl_event_source;
    fn wl_event_loop_dispatch(
        r#loop: *mut wl_event_loop,
        timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn wl_display_terminate(display: *mut wl_display);
    fn wl_client_destroy(client: *mut wl_client);
    fn join_args(
        argv: *mut *mut ::core::ffi::c_char,
        argc: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn has_prefix(
        str: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    ) -> bool;
    fn swaynag_log(
        swaynag_command: *const ::core::ffi::c_char,
        swaynag: *mut swaynag_instance,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn swaynag_show(swaynag: *mut swaynag_instance);
    static mut root: *mut sway_root;
    fn root_destroy(root_0: *mut sway_root);
    fn load_main_config(
        path: *const ::core::ffi::c_char,
        is_active: bool,
        validating: bool,
    ) -> bool;
    fn run_deferred_commands();
    fn run_deferred_bindings();
    fn free_config(config_0: *mut sway_config);
    fn force_modeset();
    fn load_swaybars();
    static mut config: *mut sway_config;
    static mut unsupported_gpu_detected: bool;
    fn server_init(server_0: *mut sway_server) -> bool;
    fn server_fini(server_0: *mut sway_server);
    fn server_start(server_0: *mut sway_server) -> bool;
    fn server_run(server_0: *mut sway_server);
    fn set_rr_scheduling();
    fn ipc_init(server_0: *mut sway_server);
    fn ipc_event_shutdown(reason: *const ::core::ffi::c_char);
    fn transaction_commit_dirty();
    fn wlr_scene_set_linux_dmabuf_v1(
        scene: *mut wlr_scene,
        linux_dmabuf_v1: *mut wlr_linux_dmabuf_v1,
    );
    fn ipc_open_socket(socket_path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn ipc_single_command(
        socketfd: ::core::ffi::c_int,
        r#type: uint32_t,
        payload: *const ::core::ffi::c_char,
        len: *mut uint32_t,
    ) -> *mut ::core::ffi::c_char;
    fn sway_log_init(verbosity: sway_log_importance_t, terminate: terminate_callback_t);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_vlog(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        args: ::core::ffi::VaList,
    );
    fn parse_boolean(boolean: *const ::core::ffi::c_char, current: bool) -> bool;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut ::core::ffi::c_void,
    pub __gr_top: *mut ::core::ffi::c_void,
    pub __vr_top: *mut ::core::ffi::c_void,
    pub __gr_offs: ::core::ffi::c_int,
    pub __vr_offs: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __clock_t = ::core::ffi::c_long;
pub type __rlim64_t = ::core::ffi::c_ulong;
pub type __time_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
pub type pid_t = __pid_t;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub __pad0: ::core::ffi::c_int,
    pub _sifields: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub _pad: [::core::ffi::c_int; 28],
    pub _kill: C2Rust_Unnamed_8,
    pub _timer: C2Rust_Unnamed_7,
    pub _rt: C2Rust_Unnamed_6,
    pub _sigchld: C2Rust_Unnamed_5,
    pub _sigfault: C2Rust_Unnamed_2,
    pub _sigpoll: C2Rust_Unnamed_1,
    pub _sigsys: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub si_band: ::core::ffi::c_long,
    pub si_fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub si_addr: *mut ::core::ffi::c_void,
    pub si_addr_lsb: ::core::ffi::c_short,
    pub _bounds: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub _addr_bnd: C2Rust_Unnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: ::core::ffi::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub si_tid: ::core::ffi::c_int,
    pub si_overrun: ::core::ffi::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2Rust_Unnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::core::ffi::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *mut siginfo_t,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
}
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type ssize_t = isize;
pub type PangoFontDescription = _PangoFontDescription;
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
pub type PangoCairoFontMap = _PangoCairoFontMap;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct __rlimit_resource(pub ::core::ffi::c_uint);
impl __rlimit_resource {
    pub const RLIMIT_CPU: Self = Self(0);
    pub const RLIMIT_FSIZE: Self = Self(1);
    pub const RLIMIT_DATA: Self = Self(2);
    pub const RLIMIT_STACK: Self = Self(3);
    pub const RLIMIT_CORE: Self = Self(4);
    pub const __RLIMIT_RSS: Self = Self(5);
    pub const RLIMIT_NOFILE: Self = Self(7);
    pub const __RLIMIT_OFILE: Self = Self(7);
    pub const RLIMIT_AS: Self = Self(9);
    pub const __RLIMIT_NPROC: Self = Self(6);
    pub const __RLIMIT_MEMLOCK: Self = Self(8);
    pub const __RLIMIT_LOCKS: Self = Self(10);
    pub const __RLIMIT_SIGPENDING: Self = Self(11);
    pub const __RLIMIT_MSGQUEUE: Self = Self(12);
    pub const __RLIMIT_NICE: Self = Self(13);
    pub const __RLIMIT_RTPRIO: Self = Self(14);
    pub const __RLIMIT_RTTIME: Self = Self(15);
    pub const __RLIMIT_NLIMITS: Self = Self(16);
    pub const __RLIM_NLIMITS: Self = Self(16);
}
pub type rlim_t = __rlim64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_log_importance(pub ::core::ffi::c_uint);
impl wlr_log_importance {
    pub const WLR_SILENT: Self = Self(0);
    pub const WLR_ERROR: Self = Self(1);
    pub const WLR_INFO: Self = Self(2);
    pub const WLR_DEBUG: Self = Self(3);
    pub const WLR_LOG_IMPORTANCE_LAST: Self = Self(4);
}
pub type wlr_log_func_t = Option<
    unsafe extern "C" fn(
        wlr_log_importance,
        *const ::core::ffi::c_char,
        ::core::ffi::VaList,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_object {
    pub interface: *const wl_interface,
    pub implementation: *const ::core::ffi::c_void,
    pub id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_interface {
    pub name: *const ::core::ffi::c_char,
    pub version: ::core::ffi::c_int,
    pub method_count: ::core::ffi::c_int,
    pub methods: *const wl_message,
    pub event_count: ::core::ffi::c_int,
    pub events: *const wl_message,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_message {
    pub name: *const ::core::ffi::c_char,
    pub signature: *const ::core::ffi::c_char,
    pub types: *mut *const wl_interface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_array {
    pub size: size_t,
    pub alloc: size_t,
    pub data: *mut ::core::ffi::c_void,
}
pub type wl_event_loop_signal_func_t = Option<
    unsafe extern "C" fn(
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_listener {
    pub link: wl_list,
    pub notify: wl_notify_func_t,
}
pub type wl_notify_func_t = Option<
    unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_resource {
    pub object: wl_object,
    pub destroy: wl_resource_destroy_func_t,
    pub link: wl_list,
    pub destroy_signal: wl_signal,
    pub client: *mut wl_client,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
pub type wl_resource_destroy_func_t = Option<
    unsafe extern "C" fn(*mut wl_resource) -> (),
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_input_device_type(pub ::core::ffi::c_uint);
impl wlr_input_device_type {
    pub const WLR_INPUT_DEVICE_KEYBOARD: Self = Self(0);
    pub const WLR_INPUT_DEVICE_POINTER: Self = Self(1);
    pub const WLR_INPUT_DEVICE_TOUCH: Self = Self(2);
    pub const WLR_INPUT_DEVICE_TABLET: Self = Self(3);
    pub const WLR_INPUT_DEVICE_TABLET_PAD: Self = Self(4);
    pub const WLR_INPUT_DEVICE_SWITCH: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_device {
    pub r#type: wlr_input_device_type,
    pub name: *mut ::core::ffi::c_char,
    pub events: C2Rust_Unnamed_10,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub destroy: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_pointer_button_state(pub ::core::ffi::c_uint);
impl wl_pointer_button_state {
    pub const WL_POINTER_BUTTON_STATE_RELEASED: Self = Self(0);
    pub const WL_POINTER_BUTTON_STATE_PRESSED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_pointer_axis(pub ::core::ffi::c_uint);
impl wl_pointer_axis {
    pub const WL_POINTER_AXIS_VERTICAL_SCROLL: Self = Self(0);
    pub const WL_POINTER_AXIS_HORIZONTAL_SCROLL: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_pointer_axis_source(pub ::core::ffi::c_uint);
impl wl_pointer_axis_source {
    pub const WL_POINTER_AXIS_SOURCE_WHEEL: Self = Self(0);
    pub const WL_POINTER_AXIS_SOURCE_FINGER: Self = Self(1);
    pub const WL_POINTER_AXIS_SOURCE_CONTINUOUS: Self = Self(2);
    pub const WL_POINTER_AXIS_SOURCE_WHEEL_TILT: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_pointer_axis_relative_direction(pub ::core::ffi::c_uint);
impl wl_pointer_axis_relative_direction {
    pub const WL_POINTER_AXIS_RELATIVE_DIRECTION_IDENTICAL: Self = Self(0);
    pub const WL_POINTER_AXIS_RELATIVE_DIRECTION_INVERTED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_output_subpixel(pub ::core::ffi::c_uint);
impl wl_output_subpixel {
    pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: Self = Self(0);
    pub const WL_OUTPUT_SUBPIXEL_NONE: Self = Self(1);
    pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: Self = Self(2);
    pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: Self = Self(3);
    pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: Self = Self(4);
    pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_output_transform(pub ::core::ffi::c_uint);
impl wl_output_transform {
    pub const WL_OUTPUT_TRANSFORM_NORMAL: Self = Self(0);
    pub const WL_OUTPUT_TRANSFORM_90: Self = Self(1);
    pub const WL_OUTPUT_TRANSFORM_180: Self = Self(2);
    pub const WL_OUTPUT_TRANSFORM_270: Self = Self(3);
    pub const WL_OUTPUT_TRANSFORM_FLIPPED: Self = Self(4);
    pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: Self = Self(5);
    pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: Self = Self(6);
    pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_box {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_fbox {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon_set {
    pub WLR_PRIVATE: C2Rust_Unnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub addons: wl_list,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_color_transform_type(pub ::core::ffi::c_uint);
impl wlr_color_transform_type {
    pub const WLR_COLOR_TRANSFORM_INVERSE_EOTF: Self = Self(0);
    pub const WLR_COLOR_TRANSFORM_LCMS2: Self = Self(1);
    pub const WLR_COLOR_TRANSFORM_LUT_3X1D: Self = Self(2);
    pub const WLR_COLOR_TRANSFORM_MATRIX: Self = Self(3);
    pub const WLR_COLOR_TRANSFORM_PIPELINE: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_color_transform {
    pub r#type: wlr_color_transform_type,
    pub WLR_PRIVATE: C2Rust_Unnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
    pub ref_count: ::core::ffi::c_int,
    pub addons: wlr_addon_set,
}
pub type xkb_layout_index_t = uint32_t;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmModeModeInfo {
    pub clock: uint32_t,
    pub hdisplay: uint16_t,
    pub hsync_start: uint16_t,
    pub hsync_end: uint16_t,
    pub htotal: uint16_t,
    pub hskew: uint16_t,
    pub vdisplay: uint16_t,
    pub vsync_start: uint16_t,
    pub vsync_end: uint16_t,
    pub vtotal: uint16_t,
    pub vscan: uint16_t,
    pub vrefresh: uint32_t,
    pub flags: uint32_t,
    pub r#type: uint32_t,
    pub name: [::core::ffi::c_char; 32],
}
pub type drmModeModeInfo = _drmModeModeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag_instance {
    pub client: *mut wl_client,
    pub client_destroy: wl_listener,
    pub args: *const ::core::ffi::c_char,
    pub fd: [::core::ffi::c_int; 2],
    pub detailed: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: ::core::ffi::c_long,
    pub numRects: ::core::ffi::c_long,
}
pub type pixman_region32_data_t = pixman_region32_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
pub type pixman_box32_t = pixman_box32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_t = pixman_region32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_surface {
    pub resource: *mut wl_resource,
    pub compositor: *mut wlr_compositor,
    pub buffer: *mut wlr_client_buffer,
    pub buffer_damage: pixman_region32_t,
    pub opaque_region: pixman_region32_t,
    pub input_region: pixman_region32_t,
    pub current: wlr_surface_state,
    pub pending: wlr_surface_state,
    pub cached: wl_list,
    pub mapped: bool,
    pub role: *const wlr_surface_role,
    pub role_resource: *mut wl_resource,
    pub events: C2Rust_Unnamed_15,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_14,
    pub unmap_commit: bool,
    pub opaque: bool,
    pub handling_commit: bool,
    pub pending_rejected: bool,
    pub preferred_buffer_scale: int32_t,
    pub preferred_buffer_transform_sent: bool,
    pub preferred_buffer_transform: wl_output_transform,
    pub synced: wl_list,
    pub synced_len: size_t,
    pub pending_buffer_resource: *mut wl_resource,
    pub pending_buffer_resource_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub client_commit: wl_signal,
    pub commit: wl_signal,
    pub map: wl_signal,
    pub unmap: wl_signal,
    pub new_subsurface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_surface_role {
    pub name: *const ::core::ffi::c_char,
    pub no_object: bool,
    pub client_commit: Option<unsafe extern "C" fn(*mut wlr_surface) -> ()>,
    pub commit: Option<unsafe extern "C" fn(*mut wlr_surface) -> ()>,
    pub map: Option<unsafe extern "C" fn(*mut wlr_surface) -> ()>,
    pub unmap: Option<unsafe extern "C" fn(*mut wlr_surface) -> ()>,
    pub destroy: Option<unsafe extern "C" fn(*mut wlr_surface) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_surface_state {
    pub committed: uint32_t,
    pub seq: uint32_t,
    pub buffer: *mut wlr_buffer,
    pub dx: int32_t,
    pub dy: int32_t,
    pub surface_damage: pixman_region32_t,
    pub buffer_damage: pixman_region32_t,
    pub opaque: pixman_region32_t,
    pub input: pixman_region32_t,
    pub transform: wl_output_transform,
    pub scale: int32_t,
    pub frame_callback_list: wl_list,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
    pub subsurfaces_below: wl_list,
    pub subsurfaces_above: wl_list,
    pub viewport: C2Rust_Unnamed_16,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub has_src: bool,
    pub has_dst: bool,
    pub src: wlr_fbox,
    pub dst_width: ::core::ffi::c_int,
    pub dst_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_buffer {
    pub r#impl: *const wlr_buffer_impl,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub dropped: bool,
    pub n_locks: size_t,
    pub accessing_data_ptr: bool,
    pub events: C2Rust_Unnamed_17,
    pub addons: wlr_addon_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub destroy: wl_signal,
    pub release: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_client_buffer {
    pub base: wlr_buffer,
    pub texture: *mut wlr_texture,
    pub source: *mut wlr_buffer,
    pub WLR_PRIVATE: C2Rust_Unnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub source_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
    pub n_ignore_locks: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_texture {
    pub r#impl: *const wlr_texture_impl,
    pub width: uint32_t,
    pub height: uint32_t,
    pub renderer: *mut wlr_renderer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_renderer {
    pub render_buffer_caps: uint32_t,
    pub color_encodings: uint32_t,
    pub events: C2Rust_Unnamed_21,
    pub features: C2Rust_Unnamed_20,
    pub WLR_PRIVATE: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub r#impl: *const wlr_renderer_impl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub input_color_transform: bool,
    pub output_color_transform: bool,
    pub timeline: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub destroy: wl_signal,
    pub lost: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_compositor {
    pub global: *mut wl_global,
    pub renderer: *mut wlr_renderer,
    pub events: C2Rust_Unnamed_23,
    pub WLR_PRIVATE: C2Rust_Unnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_backend {
    pub r#impl: *const wlr_backend_impl,
    pub buffer_caps: uint32_t,
    pub features: C2Rust_Unnamed_25,
    pub events: C2Rust_Unnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub timeline: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_drm_format_set {
    pub len: size_t,
    pub capacity: size_t,
    pub formats: *mut wlr_drm_format,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_drm_format {
    pub format: uint32_t,
    pub len: size_t,
    pub capacity: size_t,
    pub modifiers: *mut uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_dmabuf_attributes {
    pub width: int32_t,
    pub height: int32_t,
    pub format: uint32_t,
    pub modifier: uint64_t,
    pub n_planes: ::core::ffi::c_int,
    pub offset: [uint32_t; 4],
    pub stride: [uint32_t; 4],
    pub fd: [::core::ffi::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_27,
    pub WLR_PRIVATE: C2Rust_Unnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub default_feedback: *mut wlr_linux_dmabuf_feedback_v1_compiled,
    pub default_formats: wlr_drm_format_set,
    pub surfaces: wl_list,
    pub main_device_fd: ::core::ffi::c_int,
    pub display_destroy: wl_listener,
    pub check_dmabuf_callback: Option<
        unsafe extern "C" fn(
            *mut wlr_dmabuf_attributes,
            *mut ::core::ffi::c_void,
        ) -> bool,
    >,
    pub check_dmabuf_callback_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_layout {
    pub outputs: wl_list,
    pub display: *mut wl_display,
    pub events: C2Rust_Unnamed_29,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_31,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_node {
    pub r#type: wlr_scene_node_type,
    pub parent: *mut wlr_scene_tree,
    pub link: wl_list,
    pub enabled: bool,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub events: C2Rust_Unnamed_33,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
    pub bounding_box: wlr_box,
    pub bounding_box_valid: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_scene_node_type(pub ::core::ffi::c_uint);
impl wlr_scene_node_type {
    pub const WLR_SCENE_NODE_TREE: Self = Self(0);
    pub const WLR_SCENE_NODE_RECT: Self = Self(1);
    pub const WLR_SCENE_NODE_BUFFER: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene {
    pub tree: wlr_scene_tree,
    pub outputs: wl_list,
    pub linux_dmabuf_v1: *mut wlr_linux_dmabuf_v1,
    pub gamma_control_manager_v1: *mut wlr_gamma_control_manager_v1,
    pub color_manager_v1: *mut wlr_color_manager_v1,
    pub restack_xwayland_surfaces: bool,
    pub WLR_PRIVATE: C2Rust_Unnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
    pub linux_dmabuf_v1_destroy: wl_listener,
    pub gamma_control_manager_v1_destroy: wl_listener,
    pub gamma_control_manager_v1_set_gamma: wl_listener,
    pub color_manager_v1_destroy: wl_listener,
    pub debug_damage_option: wlr_scene_debug_damage_option,
    pub direct_scanout: bool,
    pub calculate_visibility: bool,
    pub highlight_transparent_region: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_scene_debug_damage_option(pub ::core::ffi::c_uint);
impl wlr_scene_debug_damage_option {
    pub const WLR_SCENE_DEBUG_DAMAGE_NONE: Self = Self(0);
    pub const WLR_SCENE_DEBUG_DAMAGE_RERENDER: Self = Self(1);
    pub const WLR_SCENE_DEBUG_DAMAGE_HIGHLIGHT: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_rect {
    pub node: wlr_scene_node,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub color: [::core::ffi::c_float; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_root {
    pub node: sway_node,
    pub output_layout: *mut wlr_output_layout,
    pub root_scene: *mut wlr_scene,
    pub staging: *mut wlr_scene_tree,
    pub layer_tree: *mut wlr_scene_tree,
    pub layers: C2Rust_Unnamed_42,
    pub all_outputs: wl_list,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
    pub outputs: *mut list_t,
    pub non_desktop_outputs: *mut list_t,
    pub scratchpad: *mut list_t,
    pub fallback_output: *mut sway_output,
    pub fullscreen_global: *mut sway_container,
    pub events: C2Rust_Unnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_39,
    pub border: C2Rust_Unnamed_38,
    pub content_tree: *mut wlr_scene_tree,
    pub current: sway_container_state,
    pub pending: sway_container_state,
    pub title: *mut ::core::ffi::c_char,
    pub title_width: ::core::ffi::c_int,
    pub title_format: *mut ::core::ffi::c_char,
    pub prev_split_layout: sway_container_layout,
    pub is_sticky: bool,
    pub saved_x: ::core::ffi::c_double,
    pub saved_y: ::core::ffi::c_double,
    pub saved_width: ::core::ffi::c_double,
    pub saved_height: ::core::ffi::c_double,
    pub saved_border: sway_container_border,
    pub width_fraction: ::core::ffi::c_double,
    pub height_fraction: ::core::ffi::c_double,
    pub child_total_width: ::core::ffi::c_double,
    pub child_total_height: ::core::ffi::c_double,
    pub scratchpad: bool,
    pub transform: wlr_box,
    pub alpha: ::core::ffi::c_float,
    pub marks: *mut list_t,
    pub events: C2Rust_Unnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_37 {
    pub destroy: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_container_border(pub ::core::ffi::c_uint);
impl sway_container_border {
    pub const B_NONE: Self = Self(0);
    pub const B_PIXEL: Self = Self(1);
    pub const B_NORMAL: Self = Self(2);
    pub const B_CSD: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_container_layout(pub ::core::ffi::c_uint);
impl sway_container_layout {
    pub const L_NONE: Self = Self(0);
    pub const L_HORIZ: Self = Self(1);
    pub const L_VERT: Self = Self(2);
    pub const L_STACKED: Self = Self(3);
    pub const L_TABBED: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container_state {
    pub layout: sway_container_layout,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
    pub fullscreen_mode: sway_fullscreen_mode,
    pub workspace: *mut sway_workspace,
    pub parent: *mut sway_container,
    pub children: *mut list_t,
    pub focused_inactive_child: *mut sway_container,
    pub focused: bool,
    pub border: sway_container_border,
    pub border_thickness: ::core::ffi::c_int,
    pub border_top: bool,
    pub border_bottom: bool,
    pub border_left: bool,
    pub border_right: bool,
    pub content_x: ::core::ffi::c_double,
    pub content_y: ::core::ffi::c_double,
    pub content_width: ::core::ffi::c_double,
    pub content_height: ::core::ffi::c_double,
    pub formatted_title: *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_fullscreen_mode(pub ::core::ffi::c_uint);
impl sway_fullscreen_mode {
    pub const FULLSCREEN_NONE: Self = Self(0);
    pub const FULLSCREEN_WORKSPACE: Self = Self(1);
    pub const FULLSCREEN_GLOBAL: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
    pub tree: *mut wlr_scene_tree,
    pub border: *mut wlr_scene_tree,
    pub background: *mut wlr_scene_tree,
    pub title_text: *mut sway_text_node,
    pub marks_text: *mut sway_text_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_node {
    pub r#type: sway_node_type,
    pub c2rust_unnamed: C2Rust_Unnamed_41,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_40 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_41 {
    pub sway_root: *mut sway_root,
    pub sway_output: *mut sway_output,
    pub sway_workspace: *mut sway_workspace,
    pub sway_container: *mut sway_container,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_node_type(pub ::core::ffi::c_uint);
impl sway_node_type {
    pub const N_ROOT: Self = Self(0);
    pub const N_OUTPUT: Self = Self(1);
    pub const N_WORKSPACE: Self = Self(2);
    pub const N_CONTAINER: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
    pub shell_background: *mut wlr_scene_tree,
    pub shell_bottom: *mut wlr_scene_tree,
    pub tiling: *mut wlr_scene_tree,
    pub floating: *mut wlr_scene_tree,
    pub shell_top: *mut wlr_scene_tree,
    pub fullscreen: *mut wlr_scene_tree,
    pub fullscreen_global: *mut wlr_scene_tree,
    pub unmanaged: *mut wlr_scene_tree,
    pub shell_overlay: *mut wlr_scene_tree,
    pub popup: *mut wlr_scene_tree,
    pub seat: *mut wlr_scene_tree,
    pub session_lock: *mut wlr_scene_tree,
}
pub type sway_container_layout_0 = sway_container_layout;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_fowa(pub ::core::ffi::c_uint);
impl sway_fowa {
    pub const FOWA_SMART: Self = Self(0);
    pub const FOWA_URGENT: Self = Self(1);
    pub const FOWA_FOCUS: Self = Self(2);
    pub const FOWA_NONE: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_mode {
    pub name: *mut ::core::ffi::c_char,
    pub keysym_bindings: *mut list_t,
    pub keycode_bindings: *mut list_t,
    pub mouse_bindings: *mut list_t,
    pub switch_bindings: *mut list_t,
    pub gesture_bindings: *mut list_t,
    pub pango: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_config_mapped_from_region {
    pub x1: ::core::ffi::c_double,
    pub y1: ::core::ffi::c_double,
    pub x2: ::core::ffi::c_double,
    pub y2: ::core::ffi::c_double,
    pub mm: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct calibration_matrix {
    pub configured: bool,
    pub matrix: [::core::ffi::c_float; 6],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct input_config_mapped_to(pub ::core::ffi::c_uint);
impl input_config_mapped_to {
    pub const MAPPED_TO_DEFAULT: Self = Self(0);
    pub const MAPPED_TO_OUTPUT: Self = Self(1);
    pub const MAPPED_TO_REGION: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_config {
    pub identifier: *mut ::core::ffi::c_char,
    pub input_type: *const ::core::ffi::c_char,
    pub accel_profile: ::core::ffi::c_int,
    pub calibration_matrix: calibration_matrix,
    pub click_method: ::core::ffi::c_int,
    pub clickfinger_button_map: ::core::ffi::c_int,
    pub drag_3fg: ::core::ffi::c_int,
    pub drag: ::core::ffi::c_int,
    pub drag_lock: ::core::ffi::c_int,
    pub dwt: ::core::ffi::c_int,
    pub dwtp: ::core::ffi::c_int,
    pub left_handed: ::core::ffi::c_int,
    pub middle_emulation: ::core::ffi::c_int,
    pub natural_scroll: ::core::ffi::c_int,
    pub pointer_accel: ::core::ffi::c_float,
    pub rotation_angle: ::core::ffi::c_float,
    pub scroll_factor: ::core::ffi::c_float,
    pub repeat_delay: ::core::ffi::c_int,
    pub repeat_rate: ::core::ffi::c_int,
    pub scroll_button: ::core::ffi::c_int,
    pub scroll_button_lock: ::core::ffi::c_int,
    pub scroll_method: ::core::ffi::c_int,
    pub send_events: ::core::ffi::c_int,
    pub tap: ::core::ffi::c_int,
    pub tap_button_map: ::core::ffi::c_int,
    pub xkb_layout: *mut ::core::ffi::c_char,
    pub xkb_model: *mut ::core::ffi::c_char,
    pub xkb_options: *mut ::core::ffi::c_char,
    pub xkb_rules: *mut ::core::ffi::c_char,
    pub xkb_variant: *mut ::core::ffi::c_char,
    pub xkb_file: *mut ::core::ffi::c_char,
    pub xkb_file_is_set: bool,
    pub xkb_numlock: ::core::ffi::c_int,
    pub xkb_capslock: ::core::ffi::c_int,
    pub mapped_from_region: *mut input_config_mapped_from_region,
    pub mapped_to: input_config_mapped_to,
    pub mapped_to_output: *mut ::core::ffi::c_char,
    pub mapped_to_region: *mut wlr_box,
    pub tools: *mut list_t,
    pub capturable: bool,
    pub region: wlr_box,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct seat_config_hide_cursor_when_typing(pub ::core::ffi::c_uint);
impl seat_config_hide_cursor_when_typing {
    pub const HIDE_WHEN_TYPING_DEFAULT: Self = Self(0);
    pub const HIDE_WHEN_TYPING_ENABLE: Self = Self(1);
    pub const HIDE_WHEN_TYPING_DISABLE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct seat_config_allow_constrain(pub ::core::ffi::c_uint);
impl seat_config_allow_constrain {
    pub const CONSTRAIN_DEFAULT: Self = Self(0);
    pub const CONSTRAIN_ENABLE: Self = Self(1);
    pub const CONSTRAIN_DISABLE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct seat_config_shortcuts_inhibit(pub ::core::ffi::c_uint);
impl seat_config_shortcuts_inhibit {
    pub const SHORTCUTS_INHIBIT_DEFAULT: Self = Self(0);
    pub const SHORTCUTS_INHIBIT_ENABLE: Self = Self(1);
    pub const SHORTCUTS_INHIBIT_DISABLE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct seat_keyboard_grouping(pub ::core::ffi::c_uint);
impl seat_keyboard_grouping {
    pub const KEYBOARD_GROUP_DEFAULT: Self = Self(0);
    pub const KEYBOARD_GROUP_NONE: Self = Self(1);
    pub const KEYBOARD_GROUP_SMART: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seat_config {
    pub name: *mut ::core::ffi::c_char,
    pub fallback: ::core::ffi::c_int,
    pub attachments: *mut list_t,
    pub hide_cursor_timeout: ::core::ffi::c_int,
    pub hide_cursor_when_typing: seat_config_hide_cursor_when_typing,
    pub allow_constrain: seat_config_allow_constrain,
    pub shortcuts_inhibit: seat_config_shortcuts_inhibit,
    pub keyboard_grouping: seat_keyboard_grouping,
    pub idle_inhibit_sources: uint32_t,
    pub idle_wake_sources: uint32_t,
    pub xcursor_theme: C2Rust_Unnamed_43,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
    pub name: *mut ::core::ffi::c_char,
    pub size: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct scale_filter_mode(pub ::core::ffi::c_uint);
impl scale_filter_mode {
    pub const SCALE_FILTER_DEFAULT: Self = Self(0);
    pub const SCALE_FILTER_LINEAR: Self = Self(1);
    pub const SCALE_FILTER_NEAREST: Self = Self(2);
    pub const SCALE_FILTER_SMART: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct render_bit_depth(pub ::core::ffi::c_uint);
impl render_bit_depth {
    pub const RENDER_BIT_DEPTH_DEFAULT: Self = Self(0);
    pub const RENDER_BIT_DEPTH_6: Self = Self(1);
    pub const RENDER_BIT_DEPTH_8: Self = Self(2);
    pub const RENDER_BIT_DEPTH_10: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct color_profile(pub ::core::ffi::c_uint);
impl color_profile {
    pub const COLOR_PROFILE_DEFAULT: Self = Self(0);
    pub const COLOR_PROFILE_TRANSFORM: Self = Self(1);
    pub const COLOR_PROFILE_TRANSFORM_WITH_DEVICE_PRIMARIES: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_config {
    pub name: *mut ::core::ffi::c_char,
    pub enabled: ::core::ffi::c_int,
    pub power: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub refresh_rate: ::core::ffi::c_float,
    pub custom_mode: ::core::ffi::c_int,
    pub drm_mode: drmModeModeInfo,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub scale: ::core::ffi::c_float,
    pub scale_filter: scale_filter_mode,
    pub transform: int32_t,
    pub subpixel: wl_output_subpixel,
    pub max_render_time: ::core::ffi::c_int,
    pub adaptive_sync: ::core::ffi::c_int,
    pub render_bit_depth: render_bit_depth,
    pub color_profile: color_profile,
    pub color_transform: *mut wlr_color_transform,
    pub allow_tearing: ::core::ffi::c_int,
    pub hdr: ::core::ffi::c_int,
    pub background: *mut ::core::ffi::c_char,
    pub background_option: *mut ::core::ffi::c_char,
    pub background_fallback: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct side_gaps {
    pub top: ::core::ffi::c_int,
    pub right: ::core::ffi::c_int,
    pub bottom: ::core::ffi::c_int,
    pub left: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct smart_gaps_mode(pub ::core::ffi::c_uint);
impl smart_gaps_mode {
    pub const SMART_GAPS_OFF: Self = Self(0);
    pub const SMART_GAPS_ON: Self = Self(1);
    pub const SMART_GAPS_INVERSE_OUTER: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct pango_markup_config(pub ::core::ffi::c_uint);
impl pango_markup_config {
    pub const PANGO_MARKUP_DISABLED: Self = Self(0);
    pub const PANGO_MARKUP_ENABLED: Self = Self(1);
    pub const PANGO_MARKUP_DEFAULT: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bar_config {
    pub swaybar_command: *mut ::core::ffi::c_char,
    pub client: *mut wl_client,
    pub client_destroy: wl_listener,
    pub mode: *mut ::core::ffi::c_char,
    pub hidden_state: *mut ::core::ffi::c_char,
    pub visible_by_modifier: bool,
    pub id: *mut ::core::ffi::c_char,
    pub modifier: uint32_t,
    pub outputs: *mut list_t,
    pub position: *mut ::core::ffi::c_char,
    pub bindings: *mut list_t,
    pub status_command: *mut ::core::ffi::c_char,
    pub pango_markup: pango_markup_config,
    pub font: *mut ::core::ffi::c_char,
    pub height: ::core::ffi::c_int,
    pub workspace_buttons: bool,
    pub wrap_scroll: bool,
    pub separator_symbol: *mut ::core::ffi::c_char,
    pub strip_workspace_numbers: bool,
    pub strip_workspace_name: bool,
    pub binding_mode_indicator: bool,
    pub verbose: bool,
    pub gaps: side_gaps,
    pub status_padding: ::core::ffi::c_int,
    pub status_edge_padding: ::core::ffi::c_int,
    pub workspace_min_width: uint32_t,
    pub colors: C2Rust_Unnamed_44,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_44 {
    pub background: *mut ::core::ffi::c_char,
    pub statusline: *mut ::core::ffi::c_char,
    pub separator: *mut ::core::ffi::c_char,
    pub focused_background: *mut ::core::ffi::c_char,
    pub focused_statusline: *mut ::core::ffi::c_char,
    pub focused_separator: *mut ::core::ffi::c_char,
    pub focused_workspace_border: *mut ::core::ffi::c_char,
    pub focused_workspace_bg: *mut ::core::ffi::c_char,
    pub focused_workspace_text: *mut ::core::ffi::c_char,
    pub active_workspace_border: *mut ::core::ffi::c_char,
    pub active_workspace_bg: *mut ::core::ffi::c_char,
    pub active_workspace_text: *mut ::core::ffi::c_char,
    pub inactive_workspace_border: *mut ::core::ffi::c_char,
    pub inactive_workspace_bg: *mut ::core::ffi::c_char,
    pub inactive_workspace_text: *mut ::core::ffi::c_char,
    pub urgent_workspace_border: *mut ::core::ffi::c_char,
    pub urgent_workspace_bg: *mut ::core::ffi::c_char,
    pub urgent_workspace_text: *mut ::core::ffi::c_char,
    pub binding_mode_border: *mut ::core::ffi::c_char,
    pub binding_mode_bg: *mut ::core::ffi::c_char,
    pub binding_mode_text: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct border_colors {
    pub border: [::core::ffi::c_float; 4],
    pub background: [::core::ffi::c_float; 4],
    pub text: [::core::ffi::c_float; 4],
    pub indicator: [::core::ffi::c_float; 4],
    pub child_border: [::core::ffi::c_float; 4],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct edge_border_types(pub ::core::ffi::c_uint);
impl edge_border_types {
    pub const E_NONE: Self = Self(0);
    pub const E_VERTICAL: Self = Self(1);
    pub const E_HORIZONTAL: Self = Self(2);
    pub const E_BOTH: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct edge_border_smart_types(pub ::core::ffi::c_uint);
impl edge_border_smart_types {
    pub const ESMART_OFF: Self = Self(0);
    pub const ESMART_ON: Self = Self(1);
    pub const ESMART_NO_GAPS: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_popup_during_fullscreen(pub ::core::ffi::c_uint);
impl sway_popup_during_fullscreen {
    pub const POPUP_SMART: Self = Self(0);
    pub const POPUP_IGNORE: Self = Self(1);
    pub const POPUP_LEAVE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct focus_follows_mouse_mode(pub ::core::ffi::c_uint);
impl focus_follows_mouse_mode {
    pub const FOLLOWS_NO: Self = Self(0);
    pub const FOLLOWS_YES: Self = Self(1);
    pub const FOLLOWS_ALWAYS: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct focus_wrapping_mode(pub ::core::ffi::c_uint);
impl focus_wrapping_mode {
    pub const WRAP_NO: Self = Self(0);
    pub const WRAP_YES: Self = Self(1);
    pub const WRAP_FORCE: Self = Self(2);
    pub const WRAP_WORKSPACE: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mouse_warping_mode(pub ::core::ffi::c_uint);
impl mouse_warping_mode {
    pub const WARP_NO: Self = Self(0);
    pub const WARP_OUTPUT: Self = Self(1);
    pub const WARP_CONTAINER: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct alignment(pub ::core::ffi::c_uint);
impl alignment {
    pub const ALIGN_LEFT: Self = Self(0);
    pub const ALIGN_CENTER: Self = Self(1);
    pub const ALIGN_RIGHT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xwayland_mode(pub ::core::ffi::c_uint);
impl xwayland_mode {
    pub const XWAYLAND_MODE_DISABLED: Self = Self(0);
    pub const XWAYLAND_MODE_LAZY: Self = Self(1);
    pub const XWAYLAND_MODE_IMMEDIATE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_config {
    pub swaynag_command: *mut ::core::ffi::c_char,
    pub swaynag_config_errors: swaynag_instance,
    pub symbols: *mut list_t,
    pub modes: *mut list_t,
    pub bars: *mut list_t,
    pub cmd_queue: *mut list_t,
    pub workspace_configs: *mut list_t,
    pub output_configs: *mut list_t,
    pub input_configs: *mut list_t,
    pub input_type_configs: *mut list_t,
    pub seat_configs: *mut list_t,
    pub criteria: *mut list_t,
    pub no_focus: *mut list_t,
    pub active_bar_modifiers: *mut list_t,
    pub current_mode: *mut sway_mode,
    pub current_bar: *mut bar_config,
    pub floating_mod: uint32_t,
    pub floating_mod_inverse: bool,
    pub dragging_key: uint32_t,
    pub resizing_key: uint32_t,
    pub floating_scroll_up_cmd: *mut ::core::ffi::c_char,
    pub floating_scroll_down_cmd: *mut ::core::ffi::c_char,
    pub floating_scroll_left_cmd: *mut ::core::ffi::c_char,
    pub floating_scroll_right_cmd: *mut ::core::ffi::c_char,
    pub default_orientation: sway_container_layout,
    pub default_layout: sway_container_layout,
    pub font: *mut ::core::ffi::c_char,
    pub font_description: *mut PangoFontDescription,
    pub font_height: ::core::ffi::c_int,
    pub font_baseline: ::core::ffi::c_int,
    pub pango_markup: bool,
    pub titlebar_border_thickness: ::core::ffi::c_int,
    pub titlebar_h_padding: ::core::ffi::c_int,
    pub titlebar_v_padding: ::core::ffi::c_int,
    pub urgent_timeout: size_t,
    pub focus_on_window_activation: sway_fowa,
    pub popup_during_fullscreen: sway_popup_during_fullscreen,
    pub xwayland: xwayland_mode,
    pub swaybg_command: *mut ::core::ffi::c_char,
    pub swaybg_client: *mut wl_client,
    pub swaybg_client_destroy: wl_listener,
    pub focus_follows_mouse: focus_follows_mouse_mode,
    pub mouse_warping: mouse_warping_mode,
    pub focus_wrapping: focus_wrapping_mode,
    pub active: bool,
    pub failed: bool,
    pub reloading: bool,
    pub reading: bool,
    pub validating: bool,
    pub auto_back_and_forth: bool,
    pub show_marks: bool,
    pub title_align: alignment,
    pub primary_selection: bool,
    pub tiling_drag: bool,
    pub tiling_drag_threshold: ::core::ffi::c_int,
    pub smart_gaps: smart_gaps_mode,
    pub gaps_inner: ::core::ffi::c_int,
    pub gaps_outer: side_gaps,
    pub config_chain: *mut list_t,
    pub user_config_path: bool,
    pub current_config_path: *const ::core::ffi::c_char,
    pub current_config: *const ::core::ffi::c_char,
    pub current_config_line_number: ::core::ffi::c_int,
    pub current_config_line: *mut ::core::ffi::c_char,
    pub border: sway_container_border,
    pub floating_border: sway_container_border,
    pub border_thickness: ::core::ffi::c_int,
    pub floating_border_thickness: ::core::ffi::c_int,
    pub hide_edge_borders: edge_border_types,
    pub hide_edge_borders_smart: edge_border_smart_types,
    pub hide_lone_tab: bool,
    pub border_colors: C2Rust_Unnamed_47,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_45,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_45 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_46,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
    pub focused: border_colors,
    pub focused_inactive: border_colors,
    pub focused_tab_title: border_colors,
    pub unfocused: border_colors,
    pub urgent: border_colors,
    pub placeholder: border_colors,
    pub background: [::core::ffi::c_float; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_idle_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_49,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_idle_inhibit_manager_v1 {
    pub wlr_manager: *mut wlr_idle_inhibit_manager_v1,
    pub new_idle_inhibitor_v1: wl_listener,
    pub manager_destroy: wl_listener,
    pub inhibitors: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xwayland_server_options {
    pub lazy: bool,
    pub enable_wm: bool,
    pub no_touch_pointer_emulation: bool,
    pub force_xrandr_emulation: bool,
    pub terminate_delay: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xwayland_server {
    pub pid: pid_t,
    pub client: *mut wl_client,
    pub pipe_source: *mut wl_event_source,
    pub wm_fd: [::core::ffi::c_int; 2],
    pub wl_fd: [::core::ffi::c_int; 2],
    pub ready: bool,
    pub server_start: time_t,
    pub display: ::core::ffi::c_int,
    pub display_name: [::core::ffi::c_char; 16],
    pub x_fd: [::core::ffi::c_int; 2],
    pub x_fd_read_event: [*mut wl_event_source; 2],
    pub options: wlr_xwayland_server_options,
    pub wl_display: *mut wl_display,
    pub idle_source: *mut wl_event_source,
    pub events: C2Rust_Unnamed_51,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_50,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
    pub start: wl_signal,
    pub ready: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_generic_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub pad: [uint32_t; 7],
    pub full_sequence: uint32_t,
}
pub type xcb_atom_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xwayland {
    pub server: *mut wlr_xwayland_server,
    pub own_server: bool,
    pub xwm: *mut wlr_xwm,
    pub shell_v1: *mut wlr_xwayland_shell_v1,
    pub display_name: *const ::core::ffi::c_char,
    pub wl_display: *mut wl_display,
    pub compositor: *mut wlr_compositor,
    pub seat: *mut wlr_seat,
    pub events: C2Rust_Unnamed_54,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_52,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_52 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_53,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_53 {
    pub x: int32_t,
    pub y: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_54 {
    pub destroy: wl_signal,
    pub ready: wl_signal,
    pub new_surface: wl_signal,
    pub remove_startup_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat {
    pub global: *mut wl_global,
    pub display: *mut wl_display,
    pub clients: wl_list,
    pub name: *mut ::core::ffi::c_char,
    pub capabilities: uint32_t,
    pub accumulated_capabilities: uint32_t,
    pub selection_source: *mut wlr_data_source,
    pub selection_serial: uint32_t,
    pub selection_offers: wl_list,
    pub primary_selection_source: *mut wlr_primary_selection_source,
    pub primary_selection_serial: uint32_t,
    pub drag: *mut wlr_drag,
    pub drag_source: *mut wlr_data_source,
    pub drag_serial: uint32_t,
    pub drag_offers: wl_list,
    pub pointer_state: wlr_seat_pointer_state,
    pub keyboard_state: wlr_seat_keyboard_state,
    pub touch_state: wlr_seat_touch_state,
    pub events: C2Rust_Unnamed_56,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_55,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_56 {
    pub pointer_grab_begin: wl_signal,
    pub pointer_grab_end: wl_signal,
    pub keyboard_grab_begin: wl_signal,
    pub keyboard_grab_end: wl_signal,
    pub touch_grab_begin: wl_signal,
    pub touch_grab_end: wl_signal,
    pub request_set_cursor: wl_signal,
    pub request_set_selection: wl_signal,
    pub set_selection: wl_signal,
    pub request_set_primary_selection: wl_signal,
    pub set_primary_selection: wl_signal,
    pub request_start_drag: wl_signal,
    pub start_drag: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_touch_state {
    pub seat: *mut wlr_seat,
    pub touch_points: wl_list,
    pub grab_serial: uint32_t,
    pub grab_id: uint32_t,
    pub grab: *mut wlr_seat_touch_grab,
    pub default_grab: *mut wlr_seat_touch_grab,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_touch_grab {
    pub interface: *const wlr_touch_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_touch_grab_interface {
    pub down: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_touch_grab,
            uint32_t,
            *mut wlr_touch_point,
        ) -> uint32_t,
    >,
    pub up: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_touch_grab,
            uint32_t,
            *mut wlr_touch_point,
        ) -> uint32_t,
    >,
    pub motion: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_touch_grab,
            uint32_t,
            *mut wlr_touch_point,
        ) -> (),
    >,
    pub enter: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_touch_grab,
            uint32_t,
            *mut wlr_touch_point,
        ) -> (),
    >,
    pub frame: Option<unsafe extern "C" fn(*mut wlr_seat_touch_grab) -> ()>,
    pub cancel: Option<unsafe extern "C" fn(*mut wlr_seat_touch_grab) -> ()>,
    pub wl_cancel: Option<
        unsafe extern "C" fn(*mut wlr_seat_touch_grab, *mut wlr_seat_client) -> (),
    >,
    pub clear_focus: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_touch_grab,
            uint32_t,
            *mut wlr_touch_point,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_touch_point {
    pub touch_id: int32_t,
    pub surface: *mut wlr_surface,
    pub client: *mut wlr_seat_client,
    pub focus_surface: *mut wlr_surface,
    pub focus_client: *mut wlr_seat_client,
    pub sx: ::core::ffi::c_double,
    pub sy: ::core::ffi::c_double,
    pub events: C2Rust_Unnamed_58,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_57,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_57 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_58 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_client {
    pub client: *mut wl_client,
    pub seat: *mut wlr_seat,
    pub link: wl_list,
    pub resources: wl_list,
    pub pointers: wl_list,
    pub keyboards: wl_list,
    pub touches: wl_list,
    pub data_devices: wl_list,
    pub events: C2Rust_Unnamed_60,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_59,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_59 {
    pub acc_discrete: [int32_t; 2],
    pub last_discrete: [int32_t; 2],
    pub acc_axis: [::core::ffi::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_serial_ringset {
    pub data: [wlr_serial_range; 128],
    pub end: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_serial_range {
    pub min_incl: uint32_t,
    pub max_incl: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_60 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_keyboard_state {
    pub seat: *mut wlr_seat,
    pub keyboard: *mut wlr_keyboard,
    pub focused_client: *mut wlr_seat_client,
    pub focused_surface: *mut wlr_surface,
    pub grab: *mut wlr_seat_keyboard_grab,
    pub default_grab: *mut wlr_seat_keyboard_grab,
    pub events: C2Rust_Unnamed_62,
    pub WLR_PRIVATE: C2Rust_Unnamed_61,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_61 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
    pub focus_change: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_keyboard_grab {
    pub interface: *const wlr_keyboard_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_grab_interface {
    pub enter: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_keyboard_grab,
            *mut wlr_surface,
            *const uint32_t,
            size_t,
            *const wlr_keyboard_modifiers,
        ) -> (),
    >,
    pub clear_focus: Option<unsafe extern "C" fn(*mut wlr_seat_keyboard_grab) -> ()>,
    pub key: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_keyboard_grab,
            uint32_t,
            uint32_t,
            uint32_t,
        ) -> (),
    >,
    pub modifiers: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_keyboard_grab,
            *const wlr_keyboard_modifiers,
        ) -> (),
    >,
    pub cancel: Option<unsafe extern "C" fn(*mut wlr_seat_keyboard_grab) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_modifiers {
    pub depressed: xkb_mod_mask_t,
    pub latched: xkb_mod_mask_t,
    pub locked: xkb_mod_mask_t,
    pub group: xkb_layout_index_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard {
    pub base: wlr_input_device,
    pub r#impl: *const wlr_keyboard_impl,
    pub group: *mut wlr_keyboard_group,
    pub keymap_string: *mut ::core::ffi::c_char,
    pub keymap_size: size_t,
    pub keymap_fd: ::core::ffi::c_int,
    pub keymap: *mut xkb_keymap,
    pub xkb_state: *mut xkb_state,
    pub led_indexes: [xkb_led_index_t; 5],
    pub mod_indexes: [xkb_mod_index_t; 8],
    pub leds: uint32_t,
    pub keycodes: [uint32_t; 32],
    pub num_keycodes: size_t,
    pub modifiers: wlr_keyboard_modifiers,
    pub repeat_info: C2Rust_Unnamed_64,
    pub events: C2Rust_Unnamed_63,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
    pub rate: int32_t,
    pub delay: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_pointer_state {
    pub seat: *mut wlr_seat,
    pub focused_client: *mut wlr_seat_client,
    pub focused_surface: *mut wlr_surface,
    pub sx: ::core::ffi::c_double,
    pub sy: ::core::ffi::c_double,
    pub grab: *mut wlr_seat_pointer_grab,
    pub default_grab: *mut wlr_seat_pointer_grab,
    pub sent_axis_source: bool,
    pub cached_axis_source: wl_pointer_axis_source,
    pub buttons: [wlr_seat_pointer_button; 16],
    pub button_count: size_t,
    pub grab_button: uint32_t,
    pub grab_serial: uint32_t,
    pub grab_time: uint32_t,
    pub events: C2Rust_Unnamed_66,
    pub WLR_PRIVATE: C2Rust_Unnamed_65,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_65 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
    pub focus_change: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_pointer_button {
    pub button: uint32_t,
    pub n_pressed: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_pointer_grab {
    pub interface: *const wlr_pointer_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_grab_interface {
    pub enter: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_pointer_grab,
            *mut wlr_surface,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub clear_focus: Option<unsafe extern "C" fn(*mut wlr_seat_pointer_grab) -> ()>,
    pub motion: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_pointer_grab,
            uint32_t,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub button: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_pointer_grab,
            uint32_t,
            uint32_t,
            wl_pointer_button_state,
        ) -> uint32_t,
    >,
    pub axis: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_pointer_grab,
            uint32_t,
            wl_pointer_axis,
            ::core::ffi::c_double,
            int32_t,
            wl_pointer_axis_source,
            wl_pointer_axis_relative_direction,
        ) -> (),
    >,
    pub frame: Option<unsafe extern "C" fn(*mut wlr_seat_pointer_grab) -> ()>,
    pub cancel: Option<unsafe extern "C" fn(*mut wlr_seat_pointer_grab) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_xwayland {
    pub wlr_xwayland: *mut wlr_xwayland,
    pub xcursor_manager: *mut wlr_xcursor_manager,
    pub atoms: [xcb_atom_t; 11],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_session_lock {
    pub lock: *mut wlr_session_lock_v1,
    pub focused: *mut wlr_surface,
    pub abandoned: bool,
    pub outputs: wl_list,
    pub new_surface: wl_listener,
    pub unlock: wl_listener,
    pub destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_server {
    pub wl_display: *mut wl_display,
    pub wl_event_loop: *mut wl_event_loop,
    pub socket: *mut ::core::ffi::c_char,
    pub backend: *mut wlr_backend,
    pub session: *mut wlr_session,
    pub headless_backend: *mut wlr_backend,
    pub renderer: *mut wlr_renderer,
    pub allocator: *mut wlr_allocator,
    pub compositor: *mut wlr_compositor,
    pub linux_dmabuf_v1: *mut wlr_linux_dmabuf_v1,
    pub data_device_manager: *mut wlr_data_device_manager,
    pub input: *mut sway_input_manager,
    pub new_output: wl_listener,
    pub renderer_lost: wl_listener,
    pub recreating_renderer: *mut wl_event_source,
    pub idle_notifier_v1: *mut wlr_idle_notifier_v1,
    pub idle_inhibit_manager_v1: sway_idle_inhibit_manager_v1,
    pub layer_shell: *mut wlr_layer_shell_v1,
    pub layer_shell_surface: wl_listener,
    pub xdg_shell: *mut wlr_xdg_shell,
    pub xdg_shell_toplevel: wl_listener,
    pub tablet_v2: *mut wlr_tablet_manager_v2,
    pub xwayland: sway_xwayland,
    pub xwayland_surface: wl_listener,
    pub xwayland_ready: wl_listener,
    pub relative_pointer_manager: *mut wlr_relative_pointer_manager_v1,
    pub server_decoration_manager: *mut wlr_server_decoration_manager,
    pub server_decoration: wl_listener,
    pub decorations: wl_list,
    pub xdg_decoration_manager: *mut wlr_xdg_decoration_manager_v1,
    pub xdg_decoration: wl_listener,
    pub xdg_decorations: wl_list,
    pub drm_lease_manager: *mut wlr_drm_lease_v1_manager,
    pub drm_lease_request: wl_listener,
    pub pointer_constraints: *mut wlr_pointer_constraints_v1,
    pub pointer_constraint: wl_listener,
    pub xdg_output_manager_v1: *mut wlr_xdg_output_manager_v1,
    pub output_manager_v1: *mut wlr_output_manager_v1,
    pub output_manager_apply: wl_listener,
    pub output_manager_test: wl_listener,
    pub gamma_control_manager_v1: *mut wlr_gamma_control_manager_v1,
    pub gamma_control_set_gamma: wl_listener,
    pub session_lock: C2Rust_Unnamed_67,
    pub output_power_manager_v1: *mut wlr_output_power_manager_v1,
    pub output_power_manager_set_mode: wl_listener,
    pub input_method: *mut wlr_input_method_manager_v2,
    pub text_input: *mut wlr_text_input_manager_v3,
    pub foreign_toplevel_list: *mut wlr_ext_foreign_toplevel_list_v1,
    pub foreign_toplevel_manager: *mut wlr_foreign_toplevel_manager_v1,
    pub content_type_manager_v1: *mut wlr_content_type_manager_v1,
    pub wlr_data_control_manager_v1: *mut wlr_data_control_manager_v1,
    pub ext_data_control_manager_v1: *mut wlr_ext_data_control_manager_v1,
    pub screencopy_manager_v1: *mut wlr_screencopy_manager_v1,
    pub ext_image_copy_capture_manager_v1: *mut wlr_ext_image_copy_capture_manager_v1,
    pub export_dmabuf_manager_v1: *mut wlr_export_dmabuf_manager_v1,
    pub security_context_manager_v1: *mut wlr_security_context_manager_v1,
    pub ext_foreign_toplevel_image_capture_source_manager_v1: *mut wlr_ext_foreign_toplevel_image_capture_source_manager_v1,
    pub new_foreign_toplevel_capture_request: wl_listener,
    pub xdg_activation_v1: *mut wlr_xdg_activation_v1,
    pub xdg_activation_v1_request_activate: wl_listener,
    pub xdg_activation_v1_new_token: wl_listener,
    pub xdg_toplevel_tag_manager_v1_set_tag: wl_listener,
    pub request_set_cursor_shape: wl_listener,
    pub tearing_control_v1: *mut wlr_tearing_control_manager_v1,
    pub tearing_control_new_object: wl_listener,
    pub tearing_controllers: wl_list,
    pub workspace_manager_v1: *mut wlr_ext_workspace_manager_v1,
    pub workspace_manager_v1_commit: wl_listener,
    pub pending_launcher_ctxs: wl_list,
    pub txn_timeout_ms: size_t,
    pub queued_transaction: *mut sway_transaction,
    pub pending_transaction: *mut sway_transaction,
    pub dirty_nodes: *mut list_t,
    pub delayed_modeset: *mut wl_event_source,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
    pub lock: *mut sway_session_lock,
    pub manager: *mut wlr_session_lock_manager_v1,
    pub new_lock: wl_listener,
    pub manager_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_input_manager {
    pub devices: wl_list,
    pub seats: wl_list,
    pub keyboard_shortcuts_inhibit: *mut wlr_keyboard_shortcuts_inhibit_manager_v1,
    pub virtual_keyboard: *mut wlr_virtual_keyboard_manager_v1,
    pub virtual_pointer: *mut wlr_virtual_pointer_manager_v1,
    pub pointer_gestures: *mut wlr_pointer_gestures_v1,
    pub transient_seat_manager: *mut wlr_transient_seat_manager_v1,
    pub new_input: wl_listener,
    pub inhibit_activate: wl_listener,
    pub inhibit_deactivate: wl_listener,
    pub keyboard_shortcuts_inhibit_new_inhibitor: wl_listener,
    pub virtual_keyboard_new: wl_listener,
    pub virtual_pointer_new: wl_listener,
    pub transient_seat_create: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_transient_seat_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_69,
    pub WLR_PRIVATE: C2Rust_Unnamed_68,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
    pub destroy: wl_signal,
    pub create_seat: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_pointers: wl_list,
    pub events: C2Rust_Unnamed_71,
    pub WLR_PRIVATE: C2Rust_Unnamed_70,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_71 {
    pub new_virtual_pointer: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_keyboards: wl_list,
    pub events: C2Rust_Unnamed_73,
    pub WLR_PRIVATE: C2Rust_Unnamed_72,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_73 {
    pub new_virtual_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_75,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_74,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_74 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_75 {
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_debug {
    pub noatomic: bool,
    pub txn_timings: bool,
    pub txn_wait: bool,
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SIG_DFL: __sighandler_t = None;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const SIG_SETMASK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut exit_value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut original_nofile_rlimit: rlimit = rlimit {
    rlim_cur: 0 as rlim_t,
    rlim_max: 0,
};
#[no_mangle]
pub static mut server: sway_server = sway_server {
    wl_display: ::core::ptr::null_mut::<wl_display>(),
    wl_event_loop: ::core::ptr::null_mut::<wl_event_loop>(),
    socket: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    backend: ::core::ptr::null_mut::<wlr_backend>(),
    session: ::core::ptr::null_mut::<wlr_session>(),
    headless_backend: ::core::ptr::null_mut::<wlr_backend>(),
    renderer: ::core::ptr::null_mut::<wlr_renderer>(),
    allocator: ::core::ptr::null_mut::<wlr_allocator>(),
    compositor: ::core::ptr::null_mut::<wlr_compositor>(),
    linux_dmabuf_v1: ::core::ptr::null_mut::<wlr_linux_dmabuf_v1>(),
    data_device_manager: ::core::ptr::null_mut::<wlr_data_device_manager>(),
    input: ::core::ptr::null_mut::<sway_input_manager>(),
    new_output: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    renderer_lost: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    recreating_renderer: ::core::ptr::null_mut::<wl_event_source>(),
    idle_notifier_v1: ::core::ptr::null_mut::<wlr_idle_notifier_v1>(),
    idle_inhibit_manager_v1: sway_idle_inhibit_manager_v1 {
        wlr_manager: ::core::ptr::null_mut::<wlr_idle_inhibit_manager_v1>(),
        new_idle_inhibitor_v1: wl_listener {
            link: wl_list {
                prev: ::core::ptr::null_mut::<wl_list>(),
                next: ::core::ptr::null_mut::<wl_list>(),
            },
            notify: None,
        },
        manager_destroy: wl_listener {
            link: wl_list {
                prev: ::core::ptr::null_mut::<wl_list>(),
                next: ::core::ptr::null_mut::<wl_list>(),
            },
            notify: None,
        },
        inhibitors: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
    },
    layer_shell: ::core::ptr::null_mut::<wlr_layer_shell_v1>(),
    layer_shell_surface: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    xdg_shell: ::core::ptr::null_mut::<wlr_xdg_shell>(),
    xdg_shell_toplevel: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    tablet_v2: ::core::ptr::null_mut::<wlr_tablet_manager_v2>(),
    xwayland: sway_xwayland {
        wlr_xwayland: ::core::ptr::null_mut::<wlr_xwayland>(),
        xcursor_manager: ::core::ptr::null_mut::<wlr_xcursor_manager>(),
        atoms: [0; 11],
    },
    xwayland_surface: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    xwayland_ready: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    relative_pointer_manager: ::core::ptr::null_mut::<wlr_relative_pointer_manager_v1>(),
    server_decoration_manager: ::core::ptr::null_mut::<wlr_server_decoration_manager>(),
    server_decoration: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    decorations: wl_list {
        prev: ::core::ptr::null_mut::<wl_list>(),
        next: ::core::ptr::null_mut::<wl_list>(),
    },
    xdg_decoration_manager: ::core::ptr::null_mut::<wlr_xdg_decoration_manager_v1>(),
    xdg_decoration: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    xdg_decorations: wl_list {
        prev: ::core::ptr::null_mut::<wl_list>(),
        next: ::core::ptr::null_mut::<wl_list>(),
    },
    drm_lease_manager: ::core::ptr::null_mut::<wlr_drm_lease_v1_manager>(),
    drm_lease_request: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    pointer_constraints: ::core::ptr::null_mut::<wlr_pointer_constraints_v1>(),
    pointer_constraint: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    xdg_output_manager_v1: ::core::ptr::null_mut::<wlr_xdg_output_manager_v1>(),
    output_manager_v1: ::core::ptr::null_mut::<wlr_output_manager_v1>(),
    output_manager_apply: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    output_manager_test: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    gamma_control_manager_v1: ::core::ptr::null_mut::<wlr_gamma_control_manager_v1>(),
    gamma_control_set_gamma: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    session_lock: C2Rust_Unnamed_67 {
        lock: ::core::ptr::null_mut::<sway_session_lock>(),
        manager: ::core::ptr::null_mut::<wlr_session_lock_manager_v1>(),
        new_lock: wl_listener {
            link: wl_list {
                prev: ::core::ptr::null_mut::<wl_list>(),
                next: ::core::ptr::null_mut::<wl_list>(),
            },
            notify: None,
        },
        manager_destroy: wl_listener {
            link: wl_list {
                prev: ::core::ptr::null_mut::<wl_list>(),
                next: ::core::ptr::null_mut::<wl_list>(),
            },
            notify: None,
        },
    },
    output_power_manager_v1: ::core::ptr::null_mut::<wlr_output_power_manager_v1>(),
    output_power_manager_set_mode: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    input_method: ::core::ptr::null_mut::<wlr_input_method_manager_v2>(),
    text_input: ::core::ptr::null_mut::<wlr_text_input_manager_v3>(),
    foreign_toplevel_list: ::core::ptr::null_mut::<wlr_ext_foreign_toplevel_list_v1>(),
    foreign_toplevel_manager: ::core::ptr::null_mut::<wlr_foreign_toplevel_manager_v1>(),
    content_type_manager_v1: ::core::ptr::null_mut::<wlr_content_type_manager_v1>(),
    wlr_data_control_manager_v1: ::core::ptr::null_mut::<wlr_data_control_manager_v1>(),
    ext_data_control_manager_v1: ::core::ptr::null_mut::<
        wlr_ext_data_control_manager_v1,
    >(),
    screencopy_manager_v1: ::core::ptr::null_mut::<wlr_screencopy_manager_v1>(),
    ext_image_copy_capture_manager_v1: ::core::ptr::null_mut::<
        wlr_ext_image_copy_capture_manager_v1,
    >(),
    export_dmabuf_manager_v1: ::core::ptr::null_mut::<wlr_export_dmabuf_manager_v1>(),
    security_context_manager_v1: ::core::ptr::null_mut::<
        wlr_security_context_manager_v1,
    >(),
    ext_foreign_toplevel_image_capture_source_manager_v1: ::core::ptr::null_mut::<
        wlr_ext_foreign_toplevel_image_capture_source_manager_v1,
    >(),
    new_foreign_toplevel_capture_request: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    xdg_activation_v1: ::core::ptr::null_mut::<wlr_xdg_activation_v1>(),
    xdg_activation_v1_request_activate: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    xdg_activation_v1_new_token: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    xdg_toplevel_tag_manager_v1_set_tag: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    request_set_cursor_shape: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    tearing_control_v1: ::core::ptr::null_mut::<wlr_tearing_control_manager_v1>(),
    tearing_control_new_object: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    tearing_controllers: wl_list {
        prev: ::core::ptr::null_mut::<wl_list>(),
        next: ::core::ptr::null_mut::<wl_list>(),
    },
    workspace_manager_v1: ::core::ptr::null_mut::<wlr_ext_workspace_manager_v1>(),
    workspace_manager_v1_commit: wl_listener {
        link: wl_list {
            prev: ::core::ptr::null_mut::<wl_list>(),
            next: ::core::ptr::null_mut::<wl_list>(),
        },
        notify: None,
    },
    pending_launcher_ctxs: wl_list {
        prev: ::core::ptr::null_mut::<wl_list>(),
        next: ::core::ptr::null_mut::<wl_list>(),
    },
    txn_timeout_ms: 0,
    queued_transaction: ::core::ptr::null_mut::<sway_transaction>(),
    pending_transaction: ::core::ptr::null_mut::<sway_transaction>(),
    dirty_nodes: ::core::ptr::null_mut::<list_t>(),
    delayed_modeset: ::core::ptr::null_mut::<wl_event_source>(),
};
#[no_mangle]
pub static mut debug: sway_debug = sway_debug {
    noatomic: false,
    txn_timings: false,
    txn_wait: false,
};
#[no_mangle]
pub unsafe extern "C" fn sway_terminate(mut exit_code: ::core::ffi::c_int) {
    if server.wl_display.is_null() {
        exit(exit_code);
    } else {
        exit_value = exit_code;
        ipc_event_shutdown(b"exit\0".as_ptr() as *const ::core::ffi::c_char);
        wl_event_loop_dispatch(server.wl_event_loop, 0 as ::core::ffi::c_int);
        wl_display_terminate(server.wl_display);
    };
}
#[no_mangle]
pub unsafe extern "C" fn run_as_ipc_client(
    mut command: *mut ::core::ffi::c_char,
    mut socket_path: *mut ::core::ffi::c_char,
) {
    let mut socketfd: ::core::ffi::c_int = ipc_open_socket(socket_path);
    let mut len: uint32_t = strlen(command) as uint32_t;
    let mut resp: *mut ::core::ffi::c_char = ipc_single_command(
        socketfd,
        ipc_command_type::IPC_COMMAND.0 as uint32_t,
        command,
        &raw mut len,
    );
    printf(b"%s\n\0".as_ptr() as *const ::core::ffi::c_char, resp);
    free(resp as *mut ::core::ffi::c_void);
    close(socketfd);
}
unsafe extern "C" fn log_env() {
    let mut log_vars: [*const ::core::ffi::c_char; 4] = [
        b"LD_LIBRARY_PATH\0".as_ptr() as *const ::core::ffi::c_char,
        b"LD_PRELOAD\0".as_ptr() as *const ::core::ffi::c_char,
        b"PATH\0".as_ptr() as *const ::core::ffi::c_char,
        b"SWAYSOCK\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>()
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>())
    {
        let mut value: *mut ::core::ffi::c_char = getenv(log_vars[i]);
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] %s=%s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            65 as ::core::ffi::c_int,
            log_vars[i],
            if !value.is_null() {
                value
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn log_file(mut f: *mut FILE) {
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut line_size: size_t = 0 as size_t;
    let mut nread: ssize_t = 0;
    loop {
        nread = getline(&raw mut line, &raw mut line_size, f) as ssize_t;
        if nread == -1 as ssize_t {
            break;
        }
        if *line.offset(nread - 1 as ssize_t) as ::core::ffi::c_int
            == '\n' as ::core::ffi::c_int
        {
            *line.offset(nread - 1 as ssize_t) = '\0' as ::core::ffi::c_char;
        }
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            77 as ::core::ffi::c_int,
            line,
        );
    }
    free(line as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn log_distro() {
    let mut paths: [*const ::core::ffi::c_char; 5] = [
        b"/etc/lsb-release\0".as_ptr() as *const ::core::ffi::c_char,
        b"/etc/os-release\0".as_ptr() as *const ::core::ffi::c_char,
        b"/etc/debian_version\0".as_ptr() as *const ::core::ffi::c_char,
        b"/etc/redhat-release\0".as_ptr() as *const ::core::ffi::c_char,
        b"/etc/gentoo-release\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 5]>()
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>())
    {
        let mut f: *mut FILE = fopen(
            paths[i],
            b"r\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !f.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_INFO,
                b"[%s:%d] Contents of %s:\0".as_ptr() as *const ::core::ffi::c_char,
                b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                93 as ::core::ffi::c_int,
                paths[i],
            );
            log_file(f);
            fclose(f);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn log_kernel() {
    let mut f: *mut FILE = popen(
        b"uname -a\0".as_ptr() as *const ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if f.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Unable to determine kernel version\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            103 as ::core::ffi::c_int,
        );
        return;
    }
    log_file(f);
    pclose(f);
}
unsafe extern "C" fn restore_nofile_limit() {
    if original_nofile_rlimit.rlim_cur == 0 as rlim_t {
        return;
    }
    if setrlimit(
        __rlimit_resource::RLIMIT_NOFILE.0 as __rlimit_resource_t,
        &raw mut original_nofile_rlimit,
    ) != 0 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to restore max open files limit: setrlimit(NOFILE) failed: %s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            115 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
    }
}
unsafe extern "C" fn increase_nofile_limit() {
    if getrlimit(
        __rlimit_resource::RLIMIT_NOFILE.0 as __rlimit_resource_t,
        &raw mut original_nofile_rlimit,
    ) != 0 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to bump max open files limit: getrlimit(NOFILE) failed: %s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            122 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        return;
    }
    let mut new_rlimit: rlimit = original_nofile_rlimit;
    new_rlimit.rlim_cur = new_rlimit.rlim_max;
    if setrlimit(
        __rlimit_resource::RLIMIT_NOFILE.0 as __rlimit_resource_t,
        &raw mut new_rlimit,
    ) != 0 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to bump max open files limit: setrlimit(NOFILE) failed: %s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            130 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Running with %d max open files\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            132 as ::core::ffi::c_int,
            original_nofile_rlimit.rlim_cur as ::core::ffi::c_int,
        );
        return;
    }
    pthread_atfork(
        None,
        None,
        Some(restore_nofile_limit as unsafe extern "C" fn() -> ()),
    );
}
unsafe extern "C" fn term_signal(
    mut signal: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    sway_terminate(EXIT_SUCCESS);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn restore_signals() {
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&raw mut set);
    sigprocmask(SIG_SETMASK, &raw mut set, ::core::ptr::null_mut::<sigset_t>());
    let mut sa_dfl: sigaction = sigaction {
        __sigaction_handler: C2Rust_Unnamed_9 {
            sa_handler: SIG_DFL,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigaction(SIGCHLD, &raw mut sa_dfl, ::core::ptr::null_mut::<sigaction>());
    sigaction(SIGPIPE, &raw mut sa_dfl, ::core::ptr::null_mut::<sigaction>());
}
unsafe extern "C" fn init_signals() {
    wl_event_loop_add_signal(
        server.wl_event_loop,
        SIGTERM,
        Some(
            term_signal
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        NULL_0,
    );
    wl_event_loop_add_signal(
        server.wl_event_loop,
        SIGINT,
        Some(
            term_signal
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        NULL_0,
    );
    let mut sa_ign: sigaction = sigaction {
        __sigaction_handler: C2Rust_Unnamed_9 {
            sa_handler: ::core::mem::transmute::<
                ::libc::intptr_t,
                __sighandler_t,
            >(1 as ::core::ffi::c_int as ::libc::intptr_t),
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigaction(SIGCHLD, &raw mut sa_ign, ::core::ptr::null_mut::<sigaction>());
    sigaction(SIGPIPE, &raw mut sa_ign, ::core::ptr::null_mut::<sigaction>());
    pthread_atfork(None, None, Some(restore_signals as unsafe extern "C" fn() -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn enable_debug_flag(mut flag: *const ::core::ffi::c_char) {
    if strcmp(flag, b"noatomic\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        debug.noatomic = r#true != 0;
    } else if strcmp(flag, b"txn-wait\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        debug.txn_wait = r#true != 0;
    } else if strcmp(flag, b"txn-timings\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        debug.txn_timings = r#true != 0;
    } else if has_prefix(
        flag,
        b"txn-timeout=\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        server.txn_timeout_ms = atoi(
            flag
                .offset(
                    strlen(b"txn-timeout=\0".as_ptr() as *const ::core::ffi::c_char)
                        as isize,
                ),
        ) as size_t;
    } else {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unknown debug flag: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            178 as ::core::ffi::c_int,
            flag,
        );
    };
}
unsafe extern "C" fn convert_wlr_log_importance(
    mut importance: wlr_log_importance,
) -> sway_log_importance_t {
    match importance {
        wlr_log_importance::WLR_ERROR => return sway_log_importance_t::SWAY_ERROR,
        wlr_log_importance::WLR_INFO => return sway_log_importance_t::SWAY_INFO,
        _ => return sway_log_importance_t::SWAY_DEBUG,
    };
}
unsafe extern "C" fn handle_wlr_log(
    mut importance: wlr_log_importance,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
) {
    static mut sway_fmt: [::core::ffi::c_char; 1024] = [0; 1024];
    snprintf(
        &raw mut sway_fmt as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"[wlr] %s\0".as_ptr() as *const ::core::ffi::c_char,
        fmt,
    );
    _sway_vlog(
        convert_wlr_log_importance(importance),
        &raw mut sway_fmt as *mut ::core::ffi::c_char,
        args.clone(),
    );
}
static mut long_options: [option; 9] = [
    option {
        name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'h' as ::core::ffi::c_int,
    },
    option {
        name: b"config\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'c' as ::core::ffi::c_int,
    },
    option {
        name: b"validate\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'C' as ::core::ffi::c_int,
    },
    option {
        name: b"debug\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'd' as ::core::ffi::c_int,
    },
    option {
        name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'v' as ::core::ffi::c_int,
    },
    option {
        name: b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'V' as ::core::ffi::c_int,
    },
    option {
        name: b"get-socketpath\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'p' as ::core::ffi::c_int,
    },
    option {
        name: b"unsupported-gpu\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'u' as ::core::ffi::c_int,
    },
    option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 0 as ::core::ffi::c_int,
    },
];
static mut usage: [::core::ffi::c_char; 478] = unsafe {
    ::core::mem::transmute::<
        [u8; 478],
        [::core::ffi::c_char; 478],
    >(
        *b"Usage: sway [options] [command]\n\n  -h, --help             Show help message and quit.\n  -c, --config <config>  Specify a config file.\n  -C, --validate         Check the validity of the config file, then exit.\n  -d, --debug            Enables full logging, including debug information.\n  -v, --version          Show the version number and quit.\n  -V, --verbose          Enables more verbose logging.\n      --get-socketpath   Gets the IPC socket path and prints it, then exits.\n\n\0",
    )
};
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut verbose: bool = r#false != 0;
    let mut debug_0: bool = r#false != 0;
    let mut validate: bool = r#false != 0;
    let mut allow_unsupported_gpu: bool = r#false != 0;
    let mut config_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut c: ::core::ffi::c_int = 0;
    loop {
        let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        c = getopt_long(
            argc,
            argv,
            b"hCdD:vVc:\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const long_options as *const option,
            &raw mut option_index,
        );
        if c == -1 as ::core::ffi::c_int {
            break;
        }
        match c {
            104 => {
                printf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const usage as *const ::core::ffi::c_char,
                );
                exit(EXIT_SUCCESS);
            }
            99 => {
                free(config_path as *mut ::core::ffi::c_void);
                config_path = strdup(optarg);
            }
            67 => {
                validate = r#true != 0;
            }
            100 => {
                debug_0 = r#true != 0;
            }
            68 => {
                enable_debug_flag(optarg);
            }
            117 => {
                allow_unsupported_gpu = r#true != 0;
            }
            118 => {
                printf(
                    b"sway version 1.13-dev\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                exit(EXIT_SUCCESS);
            }
            86 => {
                verbose = r#true != 0;
            }
            112 => {
                if !getenv(b"SWAYSOCK\0".as_ptr() as *const ::core::ffi::c_char)
                    .is_null()
                {
                    printf(
                        b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        getenv(b"SWAYSOCK\0".as_ptr() as *const ::core::ffi::c_char),
                    );
                    exit(EXIT_SUCCESS);
                } else {
                    fprintf(
                        stderr,
                        b"sway socket not detected.\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    exit(EXIT_FAILURE);
                }
            }
            _ => {
                fprintf(
                    stderr,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const usage as *const ::core::ffi::c_char,
                );
                exit(EXIT_FAILURE);
            }
        }
    }
    if getenv(b"XDG_RUNTIME_DIR\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        && optind == argc
    {
        fprintf(
            stderr,
            b"XDG_RUNTIME_DIR is not set in the environment. Aborting.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    let mut unsupported_gpu_env: *mut ::core::ffi::c_char = getenv(
        b"SWAY_UNSUPPORTED_GPU\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !allow_unsupported_gpu && !unsupported_gpu_env.is_null() {
        allow_unsupported_gpu = parse_boolean(unsupported_gpu_env, r#false != 0);
    }
    if debug_0 {
        sway_log_init(
            sway_log_importance_t::SWAY_DEBUG,
            Some(sway_terminate as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        );
        wlr_log_init(
            wlr_log_importance::WLR_DEBUG,
            Some(
                handle_wlr_log
                    as unsafe extern "C" fn(
                        wlr_log_importance,
                        *const ::core::ffi::c_char,
                        ::core::ffi::VaList,
                    ) -> (),
            ),
        );
    } else if verbose {
        sway_log_init(
            sway_log_importance_t::SWAY_INFO,
            Some(sway_terminate as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        );
        wlr_log_init(
            wlr_log_importance::WLR_INFO,
            Some(
                handle_wlr_log
                    as unsafe extern "C" fn(
                        wlr_log_importance,
                        *const ::core::ffi::c_char,
                        ::core::ffi::VaList,
                    ) -> (),
            ),
        );
    } else {
        sway_log_init(
            sway_log_importance_t::SWAY_ERROR,
            Some(sway_terminate as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        );
        wlr_log_init(
            wlr_log_importance::WLR_ERROR,
            Some(
                handle_wlr_log
                    as unsafe extern "C" fn(
                        wlr_log_importance,
                        *const ::core::ffi::c_char,
                        ::core::ffi::VaList,
                    ) -> (),
            ),
        );
    }
    _sway_log(
        sway_log_importance_t::SWAY_INFO,
        b"[%s:%d] Sway version 1.13-dev\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
        307 as ::core::ffi::c_int,
    );
    _sway_log(
        sway_log_importance_t::SWAY_INFO,
        b"[%s:%d] wlroots version 0.21.0-dev\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
        308 as ::core::ffi::c_int,
    );
    log_kernel();
    log_distro();
    log_env();
    if optind < argc {
        if optind != 1 as ::core::ffi::c_int {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Detected both options and positional arguments. If you are trying to use the IPC client, options are not supported. Otherwise, check the provided arguments for issues. See `man 1 sway` or `sway -h` for usage. If you are trying to generate a debug log, use `sway -d 2>sway.log`.\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                315 as ::core::ffi::c_int,
            );
            exit(EXIT_FAILURE);
        }
        let mut socket_path: *mut ::core::ffi::c_char = getenv(
            b"SWAYSOCK\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if socket_path.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Unable to retrieve socket path\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                326 as ::core::ffi::c_int,
            );
            exit(EXIT_FAILURE);
        }
        let mut command: *mut ::core::ffi::c_char = join_args(
            argv.offset(optind as isize),
            argc - optind,
        );
        run_as_ipc_client(command, socket_path);
        free(command as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    increase_nofile_limit();
    _sway_log(
        sway_log_importance_t::SWAY_INFO,
        b"[%s:%d] Starting sway version 1.13-dev\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
        337 as ::core::ffi::c_int,
    );
    if !server_init(&raw mut server) {
        return 1 as ::core::ffi::c_int;
    }
    init_signals();
    if !server.linux_dmabuf_v1.is_null() {
        wlr_scene_set_linux_dmabuf_v1((*root).root_scene, server.linux_dmabuf_v1);
    }
    if validate {
        let mut valid: bool = load_main_config(config_path, r#false != 0, r#true != 0);
        free(config_path as *mut ::core::ffi::c_void);
        return if valid as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
    }
    ipc_init(&raw mut server);
    let mut nag_gpu: swaynag_instance = swaynag_instance {
        client: ::core::ptr::null_mut::<wl_client>(),
        client_destroy: wl_listener {
            link: wl_list {
                prev: ::core::ptr::null_mut::<wl_list>(),
                next: ::core::ptr::null_mut::<wl_list>(),
            },
            notify: None,
        },
        args: b"--type error --message 'Proprietary GPU drivers are not supported by sway. Do not report issues.' --detailed-message\0"
            .as_ptr() as *const ::core::ffi::c_char,
        fd: [0; 2],
        detailed: r#true != 0,
    };
    setenv(
        b"WAYLAND_DISPLAY\0".as_ptr() as *const ::core::ffi::c_char,
        server.socket,
        r#true,
    );
    if !load_main_config(config_path, r#false != 0, r#false != 0) {
        sway_terminate(EXIT_FAILURE);
    } else {
        set_rr_scheduling();
        if !server_start(&raw mut server) {
            sway_terminate(EXIT_FAILURE);
        } else {
            (*config).active = r#true != 0;
            force_modeset();
            load_swaybars();
            run_deferred_commands();
            run_deferred_bindings();
            transaction_commit_dirty();
            if !(*config).swaynag_config_errors.client.is_null() {
                swaynag_show(&raw mut (*config).swaynag_config_errors);
            }
            if unsupported_gpu_detected as ::core::ffi::c_int != 0
                && !allow_unsupported_gpu
            {
                swaynag_log(
                    (*config).swaynag_command,
                    &raw mut nag_gpu,
                    b"To remove this message, launch sway with --unsupported-gpu or set the environment variable SWAY_UNSUPPORTED_GPU=true.\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                swaynag_show(&raw mut nag_gpu);
            }
            server_run(&raw mut server);
        }
    }
    _sway_log(
        sway_log_importance_t::SWAY_INFO,
        b"[%s:%d] Shutting down sway\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/main.c\0".as_ptr() as *const ::core::ffi::c_char,
        398 as ::core::ffi::c_int,
    );
    server_fini(&raw mut server);
    root_destroy(root);
    root = ::core::ptr::null_mut::<sway_root>();
    free(config_path as *mut ::core::ffi::c_void);
    free_config(config);
    if !nag_gpu.client.is_null() {
        wl_client_destroy(nag_gpu.client);
    }
    pango_cairo_font_map_set_default(::core::ptr::null_mut::<PangoCairoFontMap>());
    return exit_value;
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
