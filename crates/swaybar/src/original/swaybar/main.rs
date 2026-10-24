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
use ::swaybar as c2rust_ffi;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type wl_display;
    pub type wl_compositor;
    pub type wl_shm;
    pub type _PangoFontDescription;
    pub type swaybar_tray;
    pub type r#loop;
    pub type status_line;
    pub type wp_cursor_shape_manager_v1;
    pub type zxdg_output_manager_v1;
    pub type zwlr_layer_shell_v1;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut optarg: *mut ::core::ffi::c_char;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    fn bar_setup(bar: *mut swaybar, socket_path: *const ::core::ffi::c_char) -> bool;
    fn bar_run(bar: *mut swaybar);
    fn bar_teardown(bar: *mut swaybar);
    fn get_socketpath() -> *mut ::core::ffi::c_char;
    fn sway_log_init(verbosity: sway_log_importance_t, terminate: terminate_callback_t);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
}
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __clock_t = ::core::ffi::c_long;
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
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
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
pub type PangoFontDescription = _PangoFontDescription;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct box_colors {
    pub border: uint32_t,
    pub background: uint32_t,
    pub text: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_config {
    pub status_command: *mut ::core::ffi::c_char,
    pub pango_markup: bool,
    pub position: uint32_t,
    pub font_description: *mut PangoFontDescription,
    pub sep_symbol: *mut ::core::ffi::c_char,
    pub mode: *mut ::core::ffi::c_char,
    pub hidden_state: *mut ::core::ffi::c_char,
    pub modifier: *mut ::core::ffi::c_char,
    pub strip_workspace_numbers: bool,
    pub strip_workspace_name: bool,
    pub binding_mode_indicator: bool,
    pub wrap_scroll: bool,
    pub workspace_buttons: bool,
    pub workspace_min_width: uint32_t,
    pub bindings: *mut list_t,
    pub outputs: wl_list,
    pub height: ::core::ffi::c_int,
    pub status_padding: ::core::ffi::c_int,
    pub status_edge_padding: ::core::ffi::c_int,
    pub gaps: C2Rust_Unnamed_11,
    pub colors: C2Rust_Unnamed_10,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_hidden: bool,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub background: uint32_t,
    pub statusline: uint32_t,
    pub separator: uint32_t,
    pub focused_background: uint32_t,
    pub focused_statusline: uint32_t,
    pub focused_separator: uint32_t,
    pub focused_workspace: box_colors,
    pub active_workspace: box_colors,
    pub inactive_workspace: box_colors,
    pub urgent_workspace: box_colors,
    pub binding_mode: box_colors,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub top: ::core::ffi::c_int,
    pub right: ::core::ffi::c_int,
    pub bottom: ::core::ffi::c_int,
    pub left: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar {
    pub id: *mut ::core::ffi::c_char,
    pub mode: *mut ::core::ffi::c_char,
    pub mode_pango_markup: bool,
    pub visible_by_modifier: bool,
    pub visible_by_urgency: bool,
    pub visible_by_mode: bool,
    pub visible: bool,
    pub display: *mut wl_display,
    pub compositor: *mut wl_compositor,
    pub layer_shell: *mut zwlr_layer_shell_v1,
    pub xdg_output_manager: *mut zxdg_output_manager_v1,
    pub cursor_shape_manager: *mut wp_cursor_shape_manager_v1,
    pub shm: *mut wl_shm,
    pub config: *mut swaybar_config,
    pub status: *mut status_line,
    pub eventloop: *mut r#loop,
    pub ipc_event_socketfd: ::core::ffi::c_int,
    pub ipc_socketfd: ::core::ffi::c_int,
    pub outputs: wl_list,
    pub unused_outputs: wl_list,
    pub seats: wl_list,
    pub tray: *mut swaybar_tray,
    pub running: bool,
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
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
static mut swaybar: swaybar = swaybar {
    id: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    mode: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    mode_pango_markup: false,
    visible_by_modifier: false,
    visible_by_urgency: false,
    visible_by_mode: false,
    visible: false,
    display: ::core::ptr::null_mut::<wl_display>(),
    compositor: ::core::ptr::null_mut::<wl_compositor>(),
    layer_shell: ::core::ptr::null_mut::<zwlr_layer_shell_v1>(),
    xdg_output_manager: ::core::ptr::null_mut::<zxdg_output_manager_v1>(),
    cursor_shape_manager: ::core::ptr::null_mut::<wp_cursor_shape_manager_v1>(),
    shm: ::core::ptr::null_mut::<wl_shm>(),
    config: ::core::ptr::null_mut::<swaybar_config>(),
    status: ::core::ptr::null_mut::<status_line>(),
    eventloop: ::core::ptr::null_mut::<r#loop>(),
    ipc_event_socketfd: 0,
    ipc_socketfd: 0,
    outputs: wl_list {
        prev: ::core::ptr::null_mut::<wl_list>(),
        next: ::core::ptr::null_mut::<wl_list>(),
    },
    unused_outputs: wl_list {
        prev: ::core::ptr::null_mut::<wl_list>(),
        next: ::core::ptr::null_mut::<wl_list>(),
    },
    seats: wl_list {
        prev: ::core::ptr::null_mut::<wl_list>(),
        next: ::core::ptr::null_mut::<wl_list>(),
    },
    tray: ::core::ptr::null_mut::<swaybar_tray>(),
    running: false,
};
#[no_mangle]
pub unsafe extern "C" fn sig_handler(mut signal: ::core::ffi::c_int) {
    swaybar.running = r#false != 0;
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut socket_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut debug: bool = r#false != 0;
    static mut long_options: [option; 6] = [
        option {
            name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'h' as ::core::ffi::c_int,
        },
        option {
            name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'v' as ::core::ffi::c_int,
        },
        option {
            name: b"socket\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 's' as ::core::ffi::c_int,
        },
        option {
            name: b"bar_id\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'b' as ::core::ffi::c_int,
        },
        option {
            name: b"debug\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'd' as ::core::ffi::c_int,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
    ];
    let mut usage: *const ::core::ffi::c_char = b"Usage: swaybar [options...]\n\n  -h, --help             Show help message and quit.\n  -v, --version          Show the version number and quit.\n  -s, --socket <socket>  Connect to sway via socket.\n  -b, --bar_id <id>      Bar ID for which to get the configuration.\n  -d, --debug            Enable debugging.\n\n PLEASE NOTE that swaybar will be automatically started by sway as\n soon as there is a 'bar' configuration block in your config file.\n You should never need to start it manually.\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut c: ::core::ffi::c_int = 0;
    loop {
        let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        c = getopt_long(
            argc,
            argv,
            b"hvs:b:d\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const long_options as *const option,
            &raw mut option_index,
        );
        if c == -1 as ::core::ffi::c_int {
            break;
        }
        match c {
            115 => {
                socket_path = strdup(optarg);
            }
            98 => {
                swaybar.id = strdup(optarg);
            }
            118 => {
                printf(
                    b"swaybar version 1.13-dev\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                exit(EXIT_SUCCESS);
            }
            100 => {
                debug = r#true != 0;
            }
            _ => {
                fprintf(stderr, b"%s\0".as_ptr() as *const ::core::ffi::c_char, usage);
                exit(EXIT_FAILURE);
            }
        }
    }
    if debug {
        sway_log_init(sway_log_importance_t::SWAY_DEBUG, None);
    } else {
        sway_log_init(sway_log_importance_t::SWAY_INFO, None);
    }
    if socket_path.is_null() {
        socket_path = get_socketpath();
        if socket_path.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Unable to retrieve socket path\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaybar/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                78 as ::core::ffi::c_int,
            );
            return 1 as ::core::ffi::c_int;
        }
    }
    if !bar_setup(&raw mut swaybar, socket_path) {
        free(socket_path as *mut ::core::ffi::c_void);
        return 1 as ::core::ffi::c_int;
    }
    free(socket_path as *mut ::core::ffi::c_void);
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2Rust_Unnamed_9 {
            sa_handler: Some(
                sig_handler as unsafe extern "C" fn(::core::ffi::c_int) -> (),
            ),
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigaction(SIGINT, &raw mut sa, ::core::ptr::null_mut::<sigaction>());
    sigaction(SIGTERM, &raw mut sa, ::core::ptr::null_mut::<sigaction>());
    swaybar.running = r#true != 0;
    bar_run(&raw mut swaybar);
    bar_teardown(&raw mut swaybar);
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
