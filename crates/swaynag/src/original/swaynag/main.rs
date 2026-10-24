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
#[allow(unused_imports)]
use ::swaynag as c2rust_ffi;
extern "C" {
    pub type _cairo;
    pub type _cairo_surface;
    pub type _PangoContext;
    pub type _PangoFontDescription;
    pub type wl_display;
    pub type wl_buffer;
    pub type wl_compositor;
    pub type wl_output;
    pub type wl_seat;
    pub type wl_shm;
    pub type wl_surface;
    pub type wp_cursor_shape_manager_v1;
    pub type zwlr_layer_surface_v1;
    pub type zwlr_layer_shell_v1;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sway_log_init(verbosity: sway_log_importance_t, terminate: terminate_callback_t);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn create_list() -> *mut list_t;
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn pango_font_description_to_string(
        desc: *const PangoFontDescription,
    ) -> *mut ::core::ffi::c_char;
    fn wl_list_init(list: *mut wl_list);
    fn swaynag_type_new(name: *const ::core::ffi::c_char) -> *mut swaynag_type;
    fn swaynag_types_add_default(types: *mut list_t);
    fn swaynag_type_get(
        types: *mut list_t,
        name: *mut ::core::ffi::c_char,
    ) -> *mut swaynag_type;
    fn swaynag_type_merge(dest: *mut swaynag_type, src: *mut swaynag_type);
    fn swaynag_types_free(types: *mut list_t);
    fn swaynag_setup(swaynag_0: *mut swaynag);
    fn swaynag_run(swaynag_0: *mut swaynag);
    fn swaynag_destroy(swaynag_0: *mut swaynag);
    fn swaynag_parse_options(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
        swaynag_0: *mut swaynag,
        types: *mut list_t,
        r#type: *mut swaynag_type,
        config: *mut *mut ::core::ffi::c_char,
        debug: *mut bool,
    ) -> ::core::ffi::c_int;
    fn swaynag_get_config_path() -> *mut ::core::ffi::c_char;
    fn swaynag_load_config(
        path: *mut ::core::ffi::c_char,
        swaynag_0: *mut swaynag,
        types: *mut list_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __uid_t = ::core::ffi::c_uint;
pub type __pid_t = ::core::ffi::c_int;
pub type __clock_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
pub type __sigval_t = sigval;
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
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
pub type int32_t = i32;
pub type uint32_t = u32;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type ssize_t = isize;
pub type PangoContext = _PangoContext;
pub type PangoFontDescription = _PangoFontDescription;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool_buffer {
    pub buffer: *mut wl_buffer,
    pub surface: *mut cairo_surface_t,
    pub cairo: *mut cairo_t,
    pub pango: *mut PangoContext,
    pub width: uint32_t,
    pub height: uint32_t,
    pub data: *mut ::core::ffi::c_void,
    pub size: size_t,
    pub busy: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag_type {
    pub name: *mut ::core::ffi::c_char,
    pub font_description: *mut PangoFontDescription,
    pub output: *mut ::core::ffi::c_char,
    pub anchors: uint32_t,
    pub layer: int32_t,
    pub button_text: uint32_t,
    pub button_background: uint32_t,
    pub details_background: uint32_t,
    pub background: uint32_t,
    pub text: uint32_t,
    pub border: uint32_t,
    pub border_bottom: uint32_t,
    pub bar_border_thickness: ssize_t,
    pub message_padding: ssize_t,
    pub details_border_thickness: ssize_t,
    pub button_border_thickness: ssize_t,
    pub button_gap: ssize_t,
    pub button_gap_close: ssize_t,
    pub button_margin_right: ssize_t,
    pub button_padding: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag {
    pub run_display: bool,
    pub display: *mut wl_display,
    pub compositor: *mut wl_compositor,
    pub seat: *mut wl_seat,
    pub shm: *mut wl_shm,
    pub outputs: wl_list,
    pub seats: wl_list,
    pub output: *mut swaynag_output,
    pub layer_shell: *mut zwlr_layer_shell_v1,
    pub layer_surface: *mut zwlr_layer_surface_v1,
    pub cursor_shape_manager: *mut wp_cursor_shape_manager_v1,
    pub surface: *mut wl_surface,
    pub width: uint32_t,
    pub height: uint32_t,
    pub scale: int32_t,
    pub buffers: [pool_buffer; 2],
    pub current_buffer: *mut pool_buffer,
    pub r#type: *mut swaynag_type,
    pub message: *mut ::core::ffi::c_char,
    pub buttons: *mut list_t,
    pub details: swaynag_details,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag_details {
    pub visible: bool,
    pub message: *mut ::core::ffi::c_char,
    pub details_text: *mut ::core::ffi::c_char,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub offset: ::core::ffi::c_int,
    pub visible_lines: ::core::ffi::c_int,
    pub total_lines: ::core::ffi::c_int,
    pub button_details: *mut swaynag_button,
    pub button_up: swaynag_button,
    pub button_down: swaynag_button,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag_button {
    pub text: *mut ::core::ffi::c_char,
    pub r#type: swaynag_action_type,
    pub action: *mut ::core::ffi::c_char,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub terminal: bool,
    pub dismiss: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct swaynag_action_type(pub ::core::ffi::c_uint);
impl swaynag_action_type {
    pub const SWAYNAG_ACTION_DISMISS: Self = Self(0);
    pub const SWAYNAG_ACTION_EXPAND: Self = Self(1);
    pub const SWAYNAG_ACTION_COMMAND: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag_output {
    pub name: *mut ::core::ffi::c_char,
    pub wl_output: *mut wl_output,
    pub wl_name: uint32_t,
    pub scale: uint32_t,
    pub swaynag: *mut swaynag,
    pub link: wl_list,
}
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut swaynag: swaynag = swaynag {
    run_display: false,
    display: ::core::ptr::null_mut::<wl_display>(),
    compositor: ::core::ptr::null_mut::<wl_compositor>(),
    seat: ::core::ptr::null_mut::<wl_seat>(),
    shm: ::core::ptr::null_mut::<wl_shm>(),
    outputs: wl_list {
        prev: ::core::ptr::null_mut::<wl_list>(),
        next: ::core::ptr::null_mut::<wl_list>(),
    },
    seats: wl_list {
        prev: ::core::ptr::null_mut::<wl_list>(),
        next: ::core::ptr::null_mut::<wl_list>(),
    },
    output: ::core::ptr::null_mut::<swaynag_output>(),
    layer_shell: ::core::ptr::null_mut::<zwlr_layer_shell_v1>(),
    layer_surface: ::core::ptr::null_mut::<zwlr_layer_surface_v1>(),
    cursor_shape_manager: ::core::ptr::null_mut::<wp_cursor_shape_manager_v1>(),
    surface: ::core::ptr::null_mut::<wl_surface>(),
    width: 0,
    height: 0,
    scale: 0,
    buffers: [pool_buffer {
        buffer: ::core::ptr::null_mut::<wl_buffer>(),
        surface: ::core::ptr::null_mut::<cairo_surface_t>(),
        cairo: ::core::ptr::null_mut::<cairo_t>(),
        pango: ::core::ptr::null_mut::<PangoContext>(),
        width: 0,
        height: 0,
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
        busy: false,
    }; 2],
    current_buffer: ::core::ptr::null_mut::<pool_buffer>(),
    r#type: ::core::ptr::null_mut::<swaynag_type>(),
    message: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    buttons: ::core::ptr::null_mut::<list_t>(),
    details: swaynag_details {
        visible: false,
        message: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        details_text: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        offset: 0,
        visible_lines: 0,
        total_lines: 0,
        button_details: ::core::ptr::null_mut::<swaynag_button>(),
        button_up: swaynag_button {
            text: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            r#type: swaynag_action_type::SWAYNAG_ACTION_DISMISS,
            action: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            terminal: false,
            dismiss: false,
        },
        button_down: swaynag_button {
            text: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            r#type: swaynag_action_type::SWAYNAG_ACTION_DISMISS,
            action: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            terminal: false,
            dismiss: false,
        },
    },
};
#[no_mangle]
pub unsafe extern "C" fn sig_handler(mut signal: ::core::ffi::c_int) {
    swaynag_destroy(&raw mut swaynag);
    exit(EXIT_FAILURE);
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut r#type: *mut swaynag_type = ::core::ptr::null_mut::<swaynag_type>();
    let mut font: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2Rust_Unnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut status: ::core::ffi::c_int = EXIT_SUCCESS;
    let mut types: *mut list_t = create_list();
    swaynag_types_add_default(types);
    swaynag.buttons = create_list();
    wl_list_init(&raw mut swaynag.outputs);
    wl_list_init(&raw mut swaynag.seats);
    let mut button_close: *mut swaynag_button = calloc(
        1 as size_t,
        ::core::mem::size_of::<swaynag_button>(),
    ) as *mut swaynag_button;
    (*button_close).text = strdup(b"X\0".as_ptr() as *const ::core::ffi::c_char);
    (*button_close).r#type = swaynag_action_type::SWAYNAG_ACTION_DISMISS;
    list_add(swaynag.buttons, button_close as *mut ::core::ffi::c_void);
    swaynag.details.details_text = strdup(
        b"Toggle details\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut config_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut debug: bool = r#false != 0;
    status = swaynag_parse_options(
        argc,
        argv,
        ::core::ptr::null_mut::<swaynag>(),
        ::core::ptr::null_mut::<list_t>(),
        ::core::ptr::null_mut::<swaynag_type>(),
        &raw mut config_path,
        &raw mut debug,
    );
    '_cleanup: {
        if status == 0 as ::core::ffi::c_int {
            sway_log_init(
                sway_log_importance_t(
                    (if debug as ::core::ffi::c_int != 0 {
                        sway_log_importance_t::SWAY_DEBUG.0 as ::core::ffi::c_int
                    } else {
                        sway_log_importance_t::SWAY_ERROR.0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                ),
                None,
            );
            if config_path.is_null() {
                config_path = swaynag_get_config_path();
            }
            if !config_path.is_null() {
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Loading config file: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaynag/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                    46 as ::core::ffi::c_int,
                    config_path,
                );
                status = swaynag_load_config(config_path, &raw mut swaynag, types);
                free(config_path as *mut ::core::ffi::c_void);
                if status != 0 as ::core::ffi::c_int {
                    break '_cleanup;
                }
            }
            if argc > 1 as ::core::ffi::c_int {
                let mut type_args: *mut swaynag_type = swaynag_type_new(
                    b"<args>\0".as_ptr() as *const ::core::ffi::c_char,
                );
                list_add(types, type_args as *mut ::core::ffi::c_void);
                status = swaynag_parse_options(
                    argc,
                    argv,
                    &raw mut swaynag,
                    types,
                    type_args,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    ::core::ptr::null_mut::<bool>(),
                );
                if status != 0 as ::core::ffi::c_int {
                    break '_cleanup;
                }
            }
            if swaynag.message.is_null() {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] No message passed. Please provide --message/-m\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaynag/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                    67 as ::core::ffi::c_int,
                );
                status = EXIT_FAILURE;
            } else {
                if swaynag.r#type.is_null() {
                    swaynag.r#type = swaynag_type_get(
                        types,
                        b"error\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
                r#type = swaynag_type_new((*swaynag.r#type).name);
                swaynag_type_merge(
                    r#type,
                    swaynag_type_get(
                        types,
                        b"<defaults>\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    ),
                );
                swaynag_type_merge(
                    r#type,
                    swaynag_type_get(
                        types,
                        b"<config>\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    ),
                );
                swaynag_type_merge(r#type, swaynag.r#type);
                swaynag_type_merge(
                    r#type,
                    swaynag_type_get(
                        types,
                        b"<args>\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    ),
                );
                swaynag.r#type = r#type;
                if !swaynag.details.message.is_null() {
                    swaynag.details.button_details = calloc(
                        1 as size_t,
                        ::core::mem::size_of::<swaynag_button>(),
                    ) as *mut swaynag_button;
                    (*swaynag.details.button_details).text = strdup(
                        swaynag.details.details_text,
                    );
                    (*swaynag.details.button_details).r#type = swaynag_action_type::SWAYNAG_ACTION_EXPAND;
                    list_add(
                        swaynag.buttons,
                        swaynag.details.button_details as *mut ::core::ffi::c_void,
                    );
                }
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Output: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"swaynag/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                    93 as ::core::ffi::c_int,
                    (*swaynag.r#type).output,
                );
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Anchors: %u\0".as_ptr() as *const ::core::ffi::c_char,
                    b"swaynag/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                    94 as ::core::ffi::c_int,
                    (*swaynag.r#type).anchors,
                );
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Type: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"swaynag/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                    95 as ::core::ffi::c_int,
                    (*swaynag.r#type).name,
                );
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Message: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"swaynag/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                    96 as ::core::ffi::c_int,
                    swaynag.message,
                );
                font = pango_font_description_to_string(
                    (*swaynag.r#type).font_description,
                );
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Font: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"swaynag/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                    98 as ::core::ffi::c_int,
                    font,
                );
                free(font as *mut ::core::ffi::c_void);
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Buttons\0".as_ptr() as *const ::core::ffi::c_char,
                    b"swaynag/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                    100 as ::core::ffi::c_int,
                );
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < (*swaynag.buttons).length {
                    let mut button: *mut swaynag_button = *(*swaynag.buttons)
                        .items
                        .offset(i as isize) as *mut swaynag_button;
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] \t[%s] `%s`\0".as_ptr() as *const ::core::ffi::c_char,
                        b"swaynag/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                        103 as ::core::ffi::c_int,
                        (*button).text,
                        (*button).action,
                    );
                    i += 1;
                }
                sa = sigaction {
                    __sigaction_handler: C2Rust_Unnamed_9 {
                        sa_handler: Some(
                            sig_handler as unsafe extern "C" fn(::core::ffi::c_int) -> (),
                        ),
                    },
                    sa_mask: __sigset_t { __val: [0; 16] },
                    sa_flags: 0,
                    sa_restorer: None,
                };
                sigaction(SIGTERM, &raw mut sa, ::core::ptr::null_mut::<sigaction>());
                swaynag_setup(&raw mut swaynag);
                swaynag_run(&raw mut swaynag);
            }
        }
    }
    swaynag_types_free(types);
    swaynag_destroy(&raw mut swaynag);
    return status;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
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
