use ::c2rust_bitfields;
extern "C" {
    pub type wl_proxy;
    pub type wl_display;
    pub type wl_buffer;
    pub type wl_compositor;
    pub type wl_output;
    pub type wl_pointer;
    pub type wl_registry;
    pub type wl_seat;
    pub type wl_shm;
    pub type wl_surface;
    pub type wl_cursor_theme;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _PangoContext;
    pub type _PangoFontDescription;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type wp_cursor_shape_device_v1;
    pub type wp_cursor_shape_manager_v1;
    pub type zwlr_layer_surface_v1;
    pub type zwlr_layer_shell_v1;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn mkstemp(__template: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn chmod(__file: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    fn execlp(
        __file: *const ::core::ffi::c_char,
        __arg: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn fork() -> __pid_t;
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn wl_proxy_marshal_flags(
        proxy: *mut wl_proxy,
        opcode: uint32_t,
        interface: *const wl_interface,
        version: uint32_t,
        flags: uint32_t,
        ...
    ) -> *mut wl_proxy;
    fn wl_proxy_destroy(proxy: *mut wl_proxy);
    fn wl_proxy_add_listener(
        proxy: *mut wl_proxy,
        implementation: *mut Option<unsafe extern "C" fn() -> ()>,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn wl_proxy_get_version(proxy: *mut wl_proxy) -> uint32_t;
    fn wl_display_connect(name: *const ::core::ffi::c_char) -> *mut wl_display;
    fn wl_display_disconnect(display: *mut wl_display);
    fn wl_display_dispatch(display: *mut wl_display) -> ::core::ffi::c_int;
    fn wl_display_roundtrip(display: *mut wl_display) -> ::core::ffi::c_int;
    static wl_registry_interface: wl_interface;
    static wl_compositor_interface: wl_interface;
    static wl_shm_interface: wl_interface;
    static wl_surface_interface: wl_interface;
    static wl_seat_interface: wl_interface;
    static wl_pointer_interface: wl_interface;
    static wl_output_interface: wl_interface;
    fn wl_cursor_theme_load(
        name: *const ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        shm: *mut wl_shm,
    ) -> *mut wl_cursor_theme;
    fn wl_cursor_theme_destroy(theme: *mut wl_cursor_theme);
    fn wl_cursor_theme_get_cursor(
        theme: *mut wl_cursor_theme,
        name: *const ::core::ffi::c_char,
    ) -> *mut wl_cursor;
    fn wl_cursor_image_get_buffer(image: *mut wl_cursor_image) -> *mut wl_buffer;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn list_free(list: *mut list_t);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_abort(filename: *const ::core::ffi::c_char, ...);
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fdopen(
        __fd: ::core::ffi::c_int,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    static wp_cursor_shape_manager_v1_interface: wl_interface;
    static wp_cursor_shape_device_v1_interface: wl_interface;
    fn destroy_buffer(buffer: *mut pool_buffer);
    fn swaynag_type_free(r#type: *mut swaynag_type);
    fn render_frame(swaynag: *mut swaynag);
    static zwlr_layer_shell_v1_interface: wl_interface;
    static zwlr_layer_surface_v1_interface: wl_interface;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __mode_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type int32_t = i32;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_message {
    pub name: *const ::core::ffi::c_char,
    pub signature: *const ::core::ffi::c_char,
    pub types: *mut *const wl_interface,
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
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
pub type wl_fixed_t = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_registry_listener {
    pub global: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_registry,
            uint32_t,
            *const ::core::ffi::c_char,
            uint32_t,
        ) -> (),
    >,
    pub global_remove: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_registry, uint32_t) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_surface_listener {
    pub enter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_surface,
            *mut wl_output,
        ) -> (),
    >,
    pub leave: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_surface,
            *mut wl_output,
        ) -> (),
    >,
    pub preferred_buffer_scale: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_surface, int32_t) -> (),
    >,
    pub preferred_buffer_transform: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_surface, uint32_t) -> (),
    >,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_seat_capability(pub ::core::ffi::c_uint);
impl wl_seat_capability {
    pub const WL_SEAT_CAPABILITY_POINTER: Self = Self(1);
    pub const WL_SEAT_CAPABILITY_KEYBOARD: Self = Self(2);
    pub const WL_SEAT_CAPABILITY_TOUCH: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_seat_listener {
    pub capabilities: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_seat, uint32_t) -> (),
    >,
    pub name: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_seat,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_pointer_button_state(pub ::core::ffi::c_uint);
impl wl_pointer_button_state {
    pub const WL_POINTER_BUTTON_STATE_RELEASED: Self = Self(0);
    pub const WL_POINTER_BUTTON_STATE_PRESSED: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_pointer_listener {
    pub enter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_pointer,
            uint32_t,
            *mut wl_surface,
            wl_fixed_t,
            wl_fixed_t,
        ) -> (),
    >,
    pub leave: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_pointer,
            uint32_t,
            *mut wl_surface,
        ) -> (),
    >,
    pub motion: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_pointer,
            uint32_t,
            wl_fixed_t,
            wl_fixed_t,
        ) -> (),
    >,
    pub button: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_pointer,
            uint32_t,
            uint32_t,
            uint32_t,
            uint32_t,
        ) -> (),
    >,
    pub axis: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_pointer,
            uint32_t,
            uint32_t,
            wl_fixed_t,
        ) -> (),
    >,
    pub frame: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_pointer) -> (),
    >,
    pub axis_source: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_pointer, uint32_t) -> (),
    >,
    pub axis_stop: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_pointer,
            uint32_t,
            uint32_t,
        ) -> (),
    >,
    pub axis_discrete: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_pointer,
            uint32_t,
            int32_t,
        ) -> (),
    >,
    pub axis_value120: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_pointer,
            uint32_t,
            int32_t,
        ) -> (),
    >,
    pub axis_relative_direction: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_pointer,
            uint32_t,
            uint32_t,
        ) -> (),
    >,
    pub warp: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_pointer,
            wl_fixed_t,
            wl_fixed_t,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_output_listener {
    pub geometry: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_output,
            int32_t,
            int32_t,
            int32_t,
            int32_t,
            int32_t,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            int32_t,
        ) -> (),
    >,
    pub mode: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_output,
            uint32_t,
            int32_t,
            int32_t,
            int32_t,
        ) -> (),
    >,
    pub done: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_output) -> (),
    >,
    pub scale: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_output, int32_t) -> (),
    >,
    pub name: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_output,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub description: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_output,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_cursor_image {
    pub width: uint32_t,
    pub height: uint32_t,
    pub hotspot_x: uint32_t,
    pub hotspot_y: uint32_t,
    pub delay: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_cursor {
    pub image_count: ::core::ffi::c_uint,
    pub images: *mut *mut wl_cursor_image,
    pub name: *mut ::core::ffi::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type PangoContext = _PangoContext;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wp_cursor_shape_device_v1_shape(pub ::core::ffi::c_uint);
impl wp_cursor_shape_device_v1_shape {
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_DEFAULT: Self = Self(1);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_CONTEXT_MENU: Self = Self(2);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_HELP: Self = Self(3);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_POINTER: Self = Self(4);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_PROGRESS: Self = Self(5);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_WAIT: Self = Self(6);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_CELL: Self = Self(7);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_CROSSHAIR: Self = Self(8);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_TEXT: Self = Self(9);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_VERTICAL_TEXT: Self = Self(10);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_ALIAS: Self = Self(11);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_COPY: Self = Self(12);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_MOVE: Self = Self(13);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_NO_DROP: Self = Self(14);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_NOT_ALLOWED: Self = Self(15);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_GRAB: Self = Self(16);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_GRABBING: Self = Self(17);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_E_RESIZE: Self = Self(18);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_N_RESIZE: Self = Self(19);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_NE_RESIZE: Self = Self(20);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_NW_RESIZE: Self = Self(21);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_S_RESIZE: Self = Self(22);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_SE_RESIZE: Self = Self(23);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_SW_RESIZE: Self = Self(24);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_W_RESIZE: Self = Self(25);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_EW_RESIZE: Self = Self(26);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_NS_RESIZE: Self = Self(27);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_NESW_RESIZE: Self = Self(28);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_NWSE_RESIZE: Self = Self(29);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_COL_RESIZE: Self = Self(30);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_ROW_RESIZE: Self = Self(31);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_ALL_SCROLL: Self = Self(32);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_ZOOM_IN: Self = Self(33);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_ZOOM_OUT: Self = Self(34);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_DND_ASK: Self = Self(35);
    pub const WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_ALL_RESIZE: Self = Self(36);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag_pointer {
    pub pointer: *mut wl_pointer,
    pub serial: uint32_t,
    pub cursor_theme: *mut wl_cursor_theme,
    pub cursor_image: *mut wl_cursor_image,
    pub cursor_surface: *mut wl_surface,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag_seat {
    pub wl_seat: *mut wl_seat,
    pub wl_name: uint32_t,
    pub swaynag: *mut swaynag,
    pub pointer: swaynag_pointer,
    pub link: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zwlr_layer_surface_v1_listener {
    pub configure: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut zwlr_layer_surface_v1,
            uint32_t,
            uint32_t,
            uint32_t,
        ) -> (),
    >,
    pub closed: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut zwlr_layer_surface_v1) -> (),
    >,
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const S_IRUSR: ::core::ffi::c_int = __S_IREAD;
pub const S_IWUSR: ::core::ffi::c_int = __S_IWRITE;
pub const S_IXUSR: ::core::ffi::c_int = __S_IEXEC;
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_fixed_to_int(mut f: wl_fixed_t) -> ::core::ffi::c_int {
    return f as ::core::ffi::c_int / 256 as ::core::ffi::c_int;
}
pub const WL_MARSHAL_FLAG_DESTROY: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
    << 0 as ::core::ffi::c_int;
pub const WL_DISPLAY_GET_REGISTRY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_display_get_registry(
    mut wl_display: *mut wl_display,
) -> *mut wl_registry {
    let mut registry: *mut wl_proxy = ::core::ptr::null_mut::<wl_proxy>();
    registry = wl_proxy_marshal_flags(
        wl_display as *mut wl_proxy,
        WL_DISPLAY_GET_REGISTRY as uint32_t,
        &raw const wl_registry_interface,
        wl_proxy_get_version(wl_display as *mut wl_proxy),
        0 as uint32_t,
        NULL,
    );
    return registry as *mut wl_registry;
}
#[inline]
unsafe extern "C" fn wl_registry_add_listener(
    mut wl_registry: *mut wl_registry,
    mut listener: *const wl_registry_listener,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return wl_proxy_add_listener(
        wl_registry as *mut wl_proxy,
        listener as *mut Option<unsafe extern "C" fn() -> ()>,
        data,
    );
}
pub const WL_REGISTRY_BIND: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_registry_destroy(mut wl_registry: *mut wl_registry) {
    wl_proxy_destroy(wl_registry as *mut wl_proxy);
}
#[inline]
unsafe extern "C" fn wl_registry_bind(
    mut wl_registry: *mut wl_registry,
    mut name: uint32_t,
    mut interface: *const wl_interface,
    mut version: uint32_t,
) -> *mut ::core::ffi::c_void {
    let mut id: *mut wl_proxy = ::core::ptr::null_mut::<wl_proxy>();
    id = wl_proxy_marshal_flags(
        wl_registry as *mut wl_proxy,
        WL_REGISTRY_BIND as uint32_t,
        interface,
        version,
        0 as uint32_t,
        name,
        (*interface).name,
        version,
        NULL,
    );
    return id as *mut ::core::ffi::c_void;
}
pub const WL_COMPOSITOR_CREATE_SURFACE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_compositor_destroy(mut wl_compositor: *mut wl_compositor) {
    wl_proxy_destroy(wl_compositor as *mut wl_proxy);
}
#[inline]
unsafe extern "C" fn wl_compositor_create_surface(
    mut wl_compositor: *mut wl_compositor,
) -> *mut wl_surface {
    let mut id: *mut wl_proxy = ::core::ptr::null_mut::<wl_proxy>();
    id = wl_proxy_marshal_flags(
        wl_compositor as *mut wl_proxy,
        WL_COMPOSITOR_CREATE_SURFACE as uint32_t,
        &raw const wl_surface_interface,
        wl_proxy_get_version(wl_compositor as *mut wl_proxy),
        0 as uint32_t,
        NULL,
    );
    return id as *mut wl_surface;
}
#[inline]
unsafe extern "C" fn wl_shm_destroy(mut wl_shm: *mut wl_shm) {
    wl_proxy_destroy(wl_shm as *mut wl_proxy);
}
#[inline]
unsafe extern "C" fn wl_surface_add_listener(
    mut wl_surface: *mut wl_surface,
    mut listener: *const wl_surface_listener,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return wl_proxy_add_listener(
        wl_surface as *mut wl_proxy,
        listener as *mut Option<unsafe extern "C" fn() -> ()>,
        data,
    );
}
pub const WL_SURFACE_DESTROY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WL_SURFACE_ATTACH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WL_SURFACE_COMMIT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const WL_SURFACE_SET_BUFFER_SCALE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const WL_SURFACE_DAMAGE_BUFFER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_surface_destroy(mut wl_surface: *mut wl_surface) {
    wl_proxy_marshal_flags(
        wl_surface as *mut wl_proxy,
        WL_SURFACE_DESTROY as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_surface as *mut wl_proxy),
        WL_MARSHAL_FLAG_DESTROY as uint32_t,
    );
}
#[inline]
unsafe extern "C" fn wl_surface_attach(
    mut wl_surface: *mut wl_surface,
    mut buffer: *mut wl_buffer,
    mut x: int32_t,
    mut y: int32_t,
) {
    wl_proxy_marshal_flags(
        wl_surface as *mut wl_proxy,
        WL_SURFACE_ATTACH as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_surface as *mut wl_proxy),
        0 as uint32_t,
        buffer,
        x,
        y,
    );
}
#[inline]
unsafe extern "C" fn wl_surface_commit(mut wl_surface: *mut wl_surface) {
    wl_proxy_marshal_flags(
        wl_surface as *mut wl_proxy,
        WL_SURFACE_COMMIT as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_surface as *mut wl_proxy),
        0 as uint32_t,
    );
}
#[inline]
unsafe extern "C" fn wl_surface_set_buffer_scale(
    mut wl_surface: *mut wl_surface,
    mut scale: int32_t,
) {
    wl_proxy_marshal_flags(
        wl_surface as *mut wl_proxy,
        WL_SURFACE_SET_BUFFER_SCALE as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_surface as *mut wl_proxy),
        0 as uint32_t,
        scale,
    );
}
#[inline]
unsafe extern "C" fn wl_surface_damage_buffer(
    mut wl_surface: *mut wl_surface,
    mut x: int32_t,
    mut y: int32_t,
    mut width: int32_t,
    mut height: int32_t,
) {
    wl_proxy_marshal_flags(
        wl_surface as *mut wl_proxy,
        WL_SURFACE_DAMAGE_BUFFER as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_surface as *mut wl_proxy),
        0 as uint32_t,
        x,
        y,
        width,
        height,
    );
}
#[inline]
unsafe extern "C" fn wl_seat_add_listener(
    mut wl_seat: *mut wl_seat,
    mut listener: *const wl_seat_listener,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return wl_proxy_add_listener(
        wl_seat as *mut wl_proxy,
        listener as *mut Option<unsafe extern "C" fn() -> ()>,
        data,
    );
}
pub const WL_SEAT_GET_POINTER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_seat_destroy(mut wl_seat: *mut wl_seat) {
    wl_proxy_destroy(wl_seat as *mut wl_proxy);
}
#[inline]
unsafe extern "C" fn wl_seat_get_pointer(mut wl_seat: *mut wl_seat) -> *mut wl_pointer {
    let mut id: *mut wl_proxy = ::core::ptr::null_mut::<wl_proxy>();
    id = wl_proxy_marshal_flags(
        wl_seat as *mut wl_proxy,
        WL_SEAT_GET_POINTER as uint32_t,
        &raw const wl_pointer_interface,
        wl_proxy_get_version(wl_seat as *mut wl_proxy),
        0 as uint32_t,
        NULL,
    );
    return id as *mut wl_pointer;
}
#[inline]
unsafe extern "C" fn wl_pointer_add_listener(
    mut wl_pointer: *mut wl_pointer,
    mut listener: *const wl_pointer_listener,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return wl_proxy_add_listener(
        wl_pointer as *mut wl_proxy,
        listener as *mut Option<unsafe extern "C" fn() -> ()>,
        data,
    );
}
pub const WL_POINTER_SET_CURSOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_pointer_destroy(mut wl_pointer: *mut wl_pointer) {
    wl_proxy_destroy(wl_pointer as *mut wl_proxy);
}
#[inline]
unsafe extern "C" fn wl_pointer_set_cursor(
    mut wl_pointer: *mut wl_pointer,
    mut serial: uint32_t,
    mut surface: *mut wl_surface,
    mut hotspot_x: int32_t,
    mut hotspot_y: int32_t,
) {
    wl_proxy_marshal_flags(
        wl_pointer as *mut wl_proxy,
        WL_POINTER_SET_CURSOR as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_pointer as *mut wl_proxy),
        0 as uint32_t,
        serial,
        surface,
        hotspot_x,
        hotspot_y,
    );
}
#[inline]
unsafe extern "C" fn wl_output_add_listener(
    mut wl_output: *mut wl_output,
    mut listener: *const wl_output_listener,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return wl_proxy_add_listener(
        wl_output as *mut wl_proxy,
        listener as *mut Option<unsafe extern "C" fn() -> ()>,
        data,
    );
}
#[inline]
unsafe extern "C" fn wl_output_destroy(mut wl_output: *mut wl_output) {
    wl_proxy_destroy(wl_output as *mut wl_proxy);
}
pub const WP_CURSOR_SHAPE_MANAGER_V1_GET_POINTER: ::core::ffi::c_int = 1
    as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wp_cursor_shape_manager_v1_get_pointer(
    mut wp_cursor_shape_manager_v1: *mut wp_cursor_shape_manager_v1,
    mut pointer: *mut wl_pointer,
) -> *mut wp_cursor_shape_device_v1 {
    let mut cursor_shape_device: *mut wl_proxy = ::core::ptr::null_mut::<wl_proxy>();
    cursor_shape_device = wl_proxy_marshal_flags(
        wp_cursor_shape_manager_v1 as *mut wl_proxy,
        WP_CURSOR_SHAPE_MANAGER_V1_GET_POINTER as uint32_t,
        &raw const wp_cursor_shape_device_v1_interface,
        wl_proxy_get_version(wp_cursor_shape_manager_v1 as *mut wl_proxy),
        0 as uint32_t,
        NULL_0,
        pointer,
    );
    return cursor_shape_device as *mut wp_cursor_shape_device_v1;
}
pub const WP_CURSOR_SHAPE_DEVICE_V1_DESTROY: ::core::ffi::c_int = 0
    as ::core::ffi::c_int;
pub const WP_CURSOR_SHAPE_DEVICE_V1_SET_SHAPE: ::core::ffi::c_int = 1
    as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wp_cursor_shape_device_v1_destroy(
    mut wp_cursor_shape_device_v1: *mut wp_cursor_shape_device_v1,
) {
    wl_proxy_marshal_flags(
        wp_cursor_shape_device_v1 as *mut wl_proxy,
        WP_CURSOR_SHAPE_DEVICE_V1_DESTROY as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wp_cursor_shape_device_v1 as *mut wl_proxy),
        WL_MARSHAL_FLAG_DESTROY as uint32_t,
    );
}
#[inline]
unsafe extern "C" fn wp_cursor_shape_device_v1_set_shape(
    mut wp_cursor_shape_device_v1: *mut wp_cursor_shape_device_v1,
    mut serial: uint32_t,
    mut shape: uint32_t,
) {
    wl_proxy_marshal_flags(
        wp_cursor_shape_device_v1 as *mut wl_proxy,
        WP_CURSOR_SHAPE_DEVICE_V1_SET_SHAPE as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wp_cursor_shape_device_v1 as *mut wl_proxy),
        0 as uint32_t,
        serial,
        shape,
    );
}
unsafe extern "C" fn nop() {}
unsafe extern "C" fn terminal_execute(
    mut terminal: *mut ::core::ffi::c_char,
    mut command: *mut ::core::ffi::c_char,
) -> bool {
    let mut fname: [::core::ffi::c_char; 19] = ::core::mem::transmute::<
        [u8; 19],
        [::core::ffi::c_char; 19],
    >(*b"/tmp/swaynagXXXXXX\0");
    let mut tmp: *mut FILE = fdopen(
        mkstemp(&raw mut fname as *mut ::core::ffi::c_char),
        b"w\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if tmp.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create temp script\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
            23 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Created temp script: %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
        26 as ::core::ffi::c_int,
        &raw mut fname as *mut ::core::ffi::c_char,
    );
    fprintf(
        tmp,
        b"#!/bin/sh\nrm %s\n%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut fname as *mut ::core::ffi::c_char,
        command,
    );
    fclose(tmp);
    chmod(
        &raw mut fname as *mut ::core::ffi::c_char,
        (S_IRUSR | S_IWUSR | S_IXUSR) as __mode_t,
    );
    let mut cmd_size: size_t = strlen(terminal)
        .wrapping_add(strlen(b" -e \0".as_ptr() as *const ::core::ffi::c_char))
        .wrapping_add(strlen(&raw mut fname as *mut ::core::ffi::c_char))
        .wrapping_add(1 as size_t);
    let mut cmd: *mut ::core::ffi::c_char = malloc(cmd_size) as *mut ::core::ffi::c_char;
    if cmd.is_null() {
        perror(b"malloc\0".as_ptr() as *const ::core::ffi::c_char);
        return r#false != 0;
    }
    snprintf(
        cmd,
        cmd_size,
        b"%s -e %s\0".as_ptr() as *const ::core::ffi::c_char,
        terminal,
        &raw mut fname as *mut ::core::ffi::c_char,
    );
    execlp(
        b"sh\0".as_ptr() as *const ::core::ffi::c_char,
        b"sh\0".as_ptr() as *const ::core::ffi::c_char,
        b"-c\0".as_ptr() as *const ::core::ffi::c_char,
        cmd,
        NULL_0,
    );
    _sway_log(
        sway_log_importance_t::SWAY_ERROR,
        b"[%s:%d] Failed to run command, execlp() returned.: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
        38 as ::core::ffi::c_int,
        strerror(*__errno_location()),
    );
    free(cmd as *mut ::core::ffi::c_void);
    return r#false != 0;
}
unsafe extern "C" fn swaynag_button_execute(
    mut swaynag: *mut swaynag,
    mut button: *mut swaynag_button,
) {
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Executing [%s]: %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
        45 as ::core::ffi::c_int,
        (*button).text,
        (*button).action,
    );
    if (*button).r#type.0 == swaynag_action_type::SWAYNAG_ACTION_DISMISS.0 {
        (*swaynag).run_display = r#false != 0;
    } else if (*button).r#type.0 == swaynag_action_type::SWAYNAG_ACTION_EXPAND.0 {
        (*swaynag).details.visible = !(*swaynag).details.visible;
        render_frame(swaynag);
    } else {
        let mut pid: pid_t = fork();
        if pid < 0 as ::core::ffi::c_int {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Failed to fork: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
                54 as ::core::ffi::c_int,
                strerror(*__errno_location()),
            );
            return;
        } else if pid == 0 as ::core::ffi::c_int {
            pid = fork() as pid_t;
            if pid < 0 as ::core::ffi::c_int {
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Failed to fork: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
                    60 as ::core::ffi::c_int,
                    strerror(*__errno_location()),
                );
                return;
            } else if pid == 0 as ::core::ffi::c_int {
                let mut terminal: *mut ::core::ffi::c_char = getenv(
                    b"TERMINAL\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if (*button).terminal as ::core::ffi::c_int != 0 && !terminal.is_null()
                    && *terminal as ::core::ffi::c_int != 0
                {
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Found $TERMINAL: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
                        66 as ::core::ffi::c_int,
                        terminal,
                    );
                    if !terminal_execute(terminal, (*button).action) {
                        swaynag_destroy(swaynag);
                        _exit(EXIT_FAILURE);
                    }
                } else {
                    if (*button).terminal {
                        _sway_log(
                            sway_log_importance_t::SWAY_DEBUG,
                            b"[%s:%d] $TERMINAL not found. Running directly\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"swaynag/swaynag.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            73 as ::core::ffi::c_int,
                        );
                    }
                    execlp(
                        b"sh\0".as_ptr() as *const ::core::ffi::c_char,
                        b"sh\0".as_ptr() as *const ::core::ffi::c_char,
                        b"-c\0".as_ptr() as *const ::core::ffi::c_char,
                        (*button).action,
                        NULL_0,
                    );
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] execlp failed: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
                        77 as ::core::ffi::c_int,
                        strerror(*__errno_location()),
                    );
                    _exit(EXIT_FAILURE);
                }
            }
            _exit(EXIT_SUCCESS);
        }
        if (*button).dismiss {
            (*swaynag).run_display = r#false != 0;
        }
        if waitpid(
            pid,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] waitpid failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_int,
                strerror(*__errno_location()),
            );
        }
    };
}
unsafe extern "C" fn layer_surface_configure(
    mut data: *mut ::core::ffi::c_void,
    mut surface: *mut zwlr_layer_surface_v1,
    mut serial: uint32_t,
    mut width: uint32_t,
    mut height: uint32_t,
) {
    let mut swaynag: *mut swaynag = data as *mut swaynag;
    (*swaynag).width = width;
    (*swaynag).height = height;
    zwlr_layer_surface_v1_ack_configure(surface, serial);
    render_frame(swaynag);
}
unsafe extern "C" fn layer_surface_closed(
    mut data: *mut ::core::ffi::c_void,
    mut surface: *mut zwlr_layer_surface_v1,
) {
    let mut swaynag: *mut swaynag = data as *mut swaynag;
    swaynag_destroy(swaynag);
}
static mut layer_surface_listener: zwlr_layer_surface_v1_listener = zwlr_layer_surface_v1_listener {
    configure: Some(
        layer_surface_configure
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut zwlr_layer_surface_v1,
                uint32_t,
                uint32_t,
                uint32_t,
            ) -> (),
    ),
    closed: Some(
        layer_surface_closed
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut zwlr_layer_surface_v1,
            ) -> (),
    ),
};
unsafe extern "C" fn surface_enter(
    mut data: *mut ::core::ffi::c_void,
    mut surface: *mut wl_surface,
    mut output: *mut wl_output,
) {
    let mut swaynag: *mut swaynag = data as *mut swaynag;
    let mut swaynag_output: *mut swaynag_output = ::core::ptr::null_mut::<
        swaynag_output,
    >();
    swaynag_output = ((*swaynag).outputs.next as *mut ::core::ffi::c_char)
        .offset(-(32 as ::core::ffi::c_ulong as isize)) as *mut swaynag_output
        as *mut swaynag_output;
    while &raw mut (*swaynag_output).link != &raw mut (*swaynag).outputs {
        if (*swaynag_output).wl_output == output {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Surface enter on output %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_int,
                (*swaynag_output).name,
            );
            (*swaynag).output = swaynag_output;
            (*swaynag).scale = (*(*swaynag).output).scale as int32_t;
            render_frame(swaynag);
            break;
        } else {
            swaynag_output = ((*swaynag_output).link.next as *mut ::core::ffi::c_char)
                .offset(-(32 as ::core::ffi::c_ulong as isize)) as *mut swaynag_output
                as *mut swaynag_output;
        }
    }
}
static mut surface_listener: wl_surface_listener = unsafe {
    wl_surface_listener {
        enter: Some(
            surface_enter
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_surface,
                    *mut wl_output,
                ) -> (),
        ),
        leave: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_surface,
                    *mut wl_output,
                ) -> (),
            >,
        >(Some(nop)),
        preferred_buffer_scale: None,
        preferred_buffer_transform: None,
    }
};
unsafe extern "C" fn update_cursor(mut seat: *mut swaynag_seat) {
    let mut pointer: *mut swaynag_pointer = &raw mut (*seat).pointer;
    let mut swaynag: *mut swaynag = (*seat).swaynag;
    if !(*pointer).cursor_theme.is_null() {
        wl_cursor_theme_destroy((*pointer).cursor_theme);
    }
    let mut cursor_theme: *const ::core::ffi::c_char = getenv(
        b"XCURSOR_THEME\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut cursor_size: ::core::ffi::c_uint = 24 as ::core::ffi::c_uint;
    let mut env_cursor_size: *const ::core::ffi::c_char = getenv(
        b"XCURSOR_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !env_cursor_size.is_null() && *env_cursor_size as ::core::ffi::c_int != 0 {
        *__errno_location() = 0 as ::core::ffi::c_int;
        let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut size: ::core::ffi::c_uint = strtoul(
            env_cursor_size,
            &raw mut end,
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint;
        if *end == 0 && *__errno_location() == 0 as ::core::ffi::c_int {
            cursor_size = size;
        }
    }
    (*pointer).cursor_theme = wl_cursor_theme_load(
        cursor_theme,
        cursor_size.wrapping_mul((*swaynag).scale as ::core::ffi::c_uint)
            as ::core::ffi::c_int,
        (*swaynag).shm,
    );
    if (*pointer).cursor_theme.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to load cursor theme\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
            156 as ::core::ffi::c_int,
        );
        return;
    }
    let mut cursor: *mut wl_cursor = wl_cursor_theme_get_cursor(
        (*pointer).cursor_theme,
        b"default\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cursor.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to get default cursor from theme\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
            161 as ::core::ffi::c_int,
        );
        return;
    }
    (*pointer).cursor_image = *(*cursor).images.offset(0isize);
    wl_surface_set_buffer_scale((*pointer).cursor_surface, (*swaynag).scale);
    wl_surface_attach(
        (*pointer).cursor_surface,
        wl_cursor_image_get_buffer((*pointer).cursor_image),
        0 as int32_t,
        0 as int32_t,
    );
    wl_pointer_set_cursor(
        (*pointer).pointer,
        (*pointer).serial,
        (*pointer).cursor_surface,
        (*(*pointer).cursor_image).hotspot_x.wrapping_div((*swaynag).scale as uint32_t)
            as int32_t,
        (*(*pointer).cursor_image).hotspot_y.wrapping_div((*swaynag).scale as uint32_t)
            as int32_t,
    );
    wl_surface_damage_buffer(
        (*pointer).cursor_surface,
        0 as int32_t,
        0 as int32_t,
        INT32_MAX as int32_t,
        INT32_MAX as int32_t,
    );
    wl_surface_commit((*pointer).cursor_surface);
}
#[no_mangle]
pub unsafe extern "C" fn update_all_cursors(mut swaynag: *mut swaynag) {
    let mut seat: *mut swaynag_seat = ::core::ptr::null_mut::<swaynag_seat>();
    seat = ((*swaynag).seats.next as *mut ::core::ffi::c_char)
        .offset(-(72 as ::core::ffi::c_ulong as isize)) as *mut swaynag_seat
        as *mut swaynag_seat;
    while &raw mut (*seat).link != &raw mut (*swaynag).seats {
        if !(*seat).pointer.pointer.is_null() {
            update_cursor(seat);
        }
        seat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(72 as ::core::ffi::c_ulong as isize)) as *mut swaynag_seat
            as *mut swaynag_seat;
    }
}
unsafe extern "C" fn wl_pointer_enter(
    mut data: *mut ::core::ffi::c_void,
    mut wl_pointer: *mut wl_pointer,
    mut serial: uint32_t,
    mut surface: *mut wl_surface,
    mut surface_x: wl_fixed_t,
    mut surface_y: wl_fixed_t,
) {
    let mut seat: *mut swaynag_seat = data as *mut swaynag_seat;
    let mut pointer: *mut swaynag_pointer = &raw mut (*seat).pointer;
    (*pointer).x = wl_fixed_to_int(surface_x);
    (*pointer).y = wl_fixed_to_int(surface_y);
    if !(*(*seat).swaynag).cursor_shape_manager.is_null() {
        let mut device: *mut wp_cursor_shape_device_v1 = wp_cursor_shape_manager_v1_get_pointer(
            (*(*seat).swaynag).cursor_shape_manager,
            wl_pointer,
        );
        wp_cursor_shape_device_v1_set_shape(
            device,
            serial,
            wp_cursor_shape_device_v1_shape::WP_CURSOR_SHAPE_DEVICE_V1_SHAPE_DEFAULT.0
                as ::core::ffi::c_int as uint32_t,
        );
        wp_cursor_shape_device_v1_destroy(device);
    } else {
        (*pointer).serial = serial;
        update_cursor(seat);
    };
}
unsafe extern "C" fn wl_pointer_motion(
    mut data: *mut ::core::ffi::c_void,
    mut wl_pointer: *mut wl_pointer,
    mut time: uint32_t,
    mut surface_x: wl_fixed_t,
    mut surface_y: wl_fixed_t,
) {
    let mut seat: *mut swaynag_seat = data as *mut swaynag_seat;
    (*seat).pointer.x = wl_fixed_to_int(surface_x);
    (*seat).pointer.y = wl_fixed_to_int(surface_y);
}
unsafe extern "C" fn wl_pointer_button(
    mut data: *mut ::core::ffi::c_void,
    mut wl_pointer: *mut wl_pointer,
    mut serial: uint32_t,
    mut time: uint32_t,
    mut button: uint32_t,
    mut state: uint32_t,
) {
    let mut seat: *mut swaynag_seat = data as *mut swaynag_seat;
    let mut swaynag: *mut swaynag = (*seat).swaynag;
    if state
        != wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0
            as ::core::ffi::c_int as uint32_t
    {
        return;
    }
    let mut x: ::core::ffi::c_double = (*seat).pointer.x as ::core::ffi::c_double;
    let mut y: ::core::ffi::c_double = (*seat).pointer.y as ::core::ffi::c_double;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*swaynag).buttons).length {
        let mut nagbutton: *mut swaynag_button = *(*(*swaynag).buttons)
            .items
            .offset(i as isize) as *mut swaynag_button;
        if x >= (*nagbutton).x as ::core::ffi::c_double
            && y >= (*nagbutton).y as ::core::ffi::c_double
            && x < ((*nagbutton).x + (*nagbutton).width) as ::core::ffi::c_double
            && y < ((*nagbutton).y + (*nagbutton).height) as ::core::ffi::c_double
        {
            swaynag_button_execute(swaynag, nagbutton);
            return;
        }
        i += 1;
    }
    if (*swaynag).details.visible as ::core::ffi::c_int != 0
        && (*swaynag).details.total_lines != (*swaynag).details.visible_lines
    {
        let mut button_up: swaynag_button = (*swaynag).details.button_up;
        if x >= button_up.x as ::core::ffi::c_double
            && y >= button_up.y as ::core::ffi::c_double
            && x < (button_up.x + button_up.width) as ::core::ffi::c_double
            && y < (button_up.y + button_up.height) as ::core::ffi::c_double
            && (*swaynag).details.offset > 0 as ::core::ffi::c_int
        {
            (*swaynag).details.offset -= 1;
            render_frame(swaynag);
            return;
        }
        let mut button_down: swaynag_button = (*swaynag).details.button_down;
        let mut bot: ::core::ffi::c_int = (*swaynag).details.total_lines;
        bot -= (*swaynag).details.visible_lines;
        if x >= button_down.x as ::core::ffi::c_double
            && y >= button_down.y as ::core::ffi::c_double
            && x < (button_down.x + button_down.width) as ::core::ffi::c_double
            && y < (button_down.y + button_down.height) as ::core::ffi::c_double
            && (*swaynag).details.offset < bot
        {
            (*swaynag).details.offset += 1;
            render_frame(swaynag);
            return;
        }
    }
}
unsafe extern "C" fn wl_pointer_axis(
    mut data: *mut ::core::ffi::c_void,
    mut wl_pointer: *mut wl_pointer,
    mut time: uint32_t,
    mut axis: uint32_t,
    mut value: wl_fixed_t,
) {
    let mut seat: *mut swaynag_seat = data as *mut swaynag_seat;
    let mut swaynag: *mut swaynag = (*seat).swaynag;
    if !(*swaynag).details.visible || (*seat).pointer.x < (*swaynag).details.x
        || (*seat).pointer.y < (*swaynag).details.y
        || (*seat).pointer.x >= (*swaynag).details.x + (*swaynag).details.width
        || (*seat).pointer.y >= (*swaynag).details.y + (*swaynag).details.height
        || (*swaynag).details.total_lines == (*swaynag).details.visible_lines
    {
        return;
    }
    let mut direction: ::core::ffi::c_int = wl_fixed_to_int(value);
    let mut bot: ::core::ffi::c_int = (*swaynag).details.total_lines
        - (*swaynag).details.visible_lines;
    if direction < 0 as ::core::ffi::c_int
        && (*swaynag).details.offset > 0 as ::core::ffi::c_int
    {
        (*swaynag).details.offset -= 1;
    } else if direction > 0 as ::core::ffi::c_int && (*swaynag).details.offset < bot {
        (*swaynag).details.offset += 1;
    }
    render_frame(swaynag);
}
static mut pointer_listener: wl_pointer_listener = unsafe {
    wl_pointer_listener {
        enter: Some(
            wl_pointer_enter
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_pointer,
                    uint32_t,
                    *mut wl_surface,
                    wl_fixed_t,
                    wl_fixed_t,
                ) -> (),
        ),
        leave: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_pointer,
                    uint32_t,
                    *mut wl_surface,
                ) -> (),
            >,
        >(Some(nop)),
        motion: Some(
            wl_pointer_motion
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_pointer,
                    uint32_t,
                    wl_fixed_t,
                    wl_fixed_t,
                ) -> (),
        ),
        button: Some(
            wl_pointer_button
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_pointer,
                    uint32_t,
                    uint32_t,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        axis: Some(
            wl_pointer_axis
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_pointer,
                    uint32_t,
                    uint32_t,
                    wl_fixed_t,
                ) -> (),
        ),
        frame: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_pointer) -> ()>,
        >(Some(nop)),
        axis_source: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_pointer,
                    uint32_t,
                ) -> (),
            >,
        >(Some(nop)),
        axis_stop: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_pointer,
                    uint32_t,
                    uint32_t,
                ) -> (),
            >,
        >(Some(nop)),
        axis_discrete: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_pointer,
                    uint32_t,
                    int32_t,
                ) -> (),
            >,
        >(Some(nop)),
        axis_value120: None,
        axis_relative_direction: None,
        warp: None,
    }
};
unsafe extern "C" fn seat_handle_capabilities(
    mut data: *mut ::core::ffi::c_void,
    mut wl_seat: *mut wl_seat,
    mut caps: wl_seat_capability,
) {
    let mut seat: *mut swaynag_seat = data as *mut swaynag_seat;
    let mut cap_pointer: bool = caps.0 & wl_seat_capability::WL_SEAT_CAPABILITY_POINTER.0
        != 0;
    if cap_pointer as ::core::ffi::c_int != 0 && (*seat).pointer.pointer.is_null() {
        (*seat).pointer.pointer = wl_seat_get_pointer(wl_seat);
        wl_pointer_add_listener(
            (*seat).pointer.pointer,
            &raw const pointer_listener,
            seat as *mut ::core::ffi::c_void,
        );
    } else if !cap_pointer && !(*seat).pointer.pointer.is_null() {
        wl_pointer_destroy((*seat).pointer.pointer);
        (*seat).pointer.pointer = ::core::ptr::null_mut::<wl_pointer>();
    }
}
static mut seat_listener: wl_seat_listener = unsafe {
    wl_seat_listener {
        capabilities: ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_seat,
                    wl_seat_capability,
                ) -> (),
            >,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_seat,
                    uint32_t,
                ) -> (),
            >,
        >(
            Some(
                seat_handle_capabilities
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut wl_seat,
                        wl_seat_capability,
                    ) -> (),
            ),
        ),
        name: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_seat,
                    *const ::core::ffi::c_char,
                ) -> (),
            >,
        >(Some(nop)),
    }
};
unsafe extern "C" fn output_scale(
    mut data: *mut ::core::ffi::c_void,
    mut output: *mut wl_output,
    mut factor: int32_t,
) {
    let mut swaynag_output: *mut swaynag_output = data as *mut swaynag_output;
    (*swaynag_output).scale = factor as uint32_t;
    if (*(*swaynag_output).swaynag).output == swaynag_output {
        (*(*swaynag_output).swaynag).scale = (*swaynag_output).scale as int32_t;
        if (*(*swaynag_output).swaynag).cursor_shape_manager.is_null() {
            update_all_cursors((*swaynag_output).swaynag);
        }
        render_frame((*swaynag_output).swaynag);
    }
}
unsafe extern "C" fn output_name(
    mut data: *mut ::core::ffi::c_void,
    mut output: *mut wl_output,
    mut name: *const ::core::ffi::c_char,
) {
    let mut swaynag_output: *mut swaynag_output = data as *mut swaynag_output;
    (*swaynag_output).name = strdup(name);
    let mut outname: *const ::core::ffi::c_char = (*(*(*swaynag_output).swaynag).r#type)
        .output;
    if (*(*swaynag_output).swaynag).output.is_null() && !outname.is_null()
        && strcmp(outname, name) == 0 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Using output %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
            342 as ::core::ffi::c_int,
            name,
        );
        (*(*swaynag_output).swaynag).output = swaynag_output;
    }
}
static mut output_listener: wl_output_listener = unsafe {
    wl_output_listener {
        geometry: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_output,
                    int32_t,
                    int32_t,
                    int32_t,
                    int32_t,
                    int32_t,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    int32_t,
                ) -> (),
            >,
        >(Some(nop)),
        mode: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_output,
                    uint32_t,
                    int32_t,
                    int32_t,
                    int32_t,
                ) -> (),
            >,
        >(Some(nop)),
        done: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_output) -> ()>,
        >(Some(nop)),
        scale: Some(
            output_scale
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_output,
                    int32_t,
                ) -> (),
        ),
        name: Some(
            output_name
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_output,
                    *const ::core::ffi::c_char,
                ) -> (),
        ),
        description: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_output,
                    *const ::core::ffi::c_char,
                ) -> (),
            >,
        >(Some(nop)),
    }
};
unsafe extern "C" fn handle_global(
    mut data: *mut ::core::ffi::c_void,
    mut registry: *mut wl_registry,
    mut name: uint32_t,
    mut interface: *const ::core::ffi::c_char,
    mut version: uint32_t,
) {
    let mut swaynag: *mut swaynag = data as *mut swaynag;
    if strcmp(interface, wl_compositor_interface.name) == 0 as ::core::ffi::c_int {
        (*swaynag).compositor = wl_registry_bind(
            registry,
            name,
            &raw const wl_compositor_interface,
            4 as uint32_t,
        ) as *mut wl_compositor;
    } else if strcmp(interface, wl_seat_interface.name) == 0 as ::core::ffi::c_int {
        let mut seat: *mut swaynag_seat = calloc(
            1 as size_t,
            ::core::mem::size_of::<swaynag_seat>(),
        ) as *mut swaynag_seat;
        if seat.is_null() {
            perror(b"calloc\0".as_ptr() as *const ::core::ffi::c_char);
            return;
        }
        (*seat).swaynag = swaynag;
        (*seat).wl_name = name;
        (*seat).wl_seat = wl_registry_bind(
            registry,
            name,
            &raw const wl_seat_interface,
            1 as uint32_t,
        ) as *mut wl_seat;
        wl_seat_add_listener(
            (*seat).wl_seat,
            &raw const seat_listener,
            seat as *mut ::core::ffi::c_void,
        );
        wl_list_insert(&raw mut (*swaynag).seats, &raw mut (*seat).link);
    } else if strcmp(interface, wl_shm_interface.name) == 0 as ::core::ffi::c_int {
        (*swaynag).shm = wl_registry_bind(
            registry,
            name,
            &raw const wl_shm_interface,
            1 as uint32_t,
        ) as *mut wl_shm;
    } else if strcmp(interface, wl_output_interface.name) == 0 as ::core::ffi::c_int {
        if (*swaynag).output.is_null() {
            let mut output: *mut swaynag_output = calloc(
                1 as size_t,
                ::core::mem::size_of::<swaynag_output>(),
            ) as *mut swaynag_output;
            if output.is_null() {
                perror(b"calloc\0".as_ptr() as *const ::core::ffi::c_char);
                return;
            }
            (*output).wl_output = wl_registry_bind(
                registry,
                name,
                &raw const wl_output_interface,
                4 as uint32_t,
            ) as *mut wl_output;
            (*output).wl_name = name;
            (*output).scale = 1 as uint32_t;
            (*output).swaynag = swaynag;
            wl_list_insert(&raw mut (*swaynag).outputs, &raw mut (*output).link);
            wl_output_add_listener(
                (*output).wl_output,
                &raw const output_listener,
                output as *mut ::core::ffi::c_void,
            );
        }
    } else if strcmp(interface, zwlr_layer_shell_v1_interface.name)
        == 0 as ::core::ffi::c_int
    {
        (*swaynag).layer_shell = wl_registry_bind(
            registry,
            name,
            &raw const zwlr_layer_shell_v1_interface,
            1 as uint32_t,
        ) as *mut zwlr_layer_shell_v1;
    } else if strcmp(interface, wp_cursor_shape_manager_v1_interface.name)
        == 0 as ::core::ffi::c_int
    {
        (*swaynag).cursor_shape_manager = wl_registry_bind(
            registry,
            name,
            &raw const wp_cursor_shape_manager_v1_interface,
            1 as uint32_t,
        ) as *mut wp_cursor_shape_manager_v1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn swaynag_seat_destroy(mut seat: *mut swaynag_seat) {
    if !(*seat).pointer.cursor_theme.is_null() {
        wl_cursor_theme_destroy((*seat).pointer.cursor_theme);
    }
    if !(*seat).pointer.pointer.is_null() {
        wl_pointer_destroy((*seat).pointer.pointer);
    }
    wl_seat_destroy((*seat).wl_seat);
    wl_list_remove(&raw mut (*seat).link);
    free(seat as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn handle_global_remove(
    mut data: *mut ::core::ffi::c_void,
    mut registry: *mut wl_registry,
    mut name: uint32_t,
) {
    let mut swaynag: *mut swaynag = data as *mut swaynag;
    if (*(*swaynag).output).wl_name == name {
        (*swaynag).run_display = r#false != 0;
    }
    let mut seat: *mut swaynag_seat = ::core::ptr::null_mut::<swaynag_seat>();
    let mut tmpseat: *mut swaynag_seat = ::core::ptr::null_mut::<swaynag_seat>();
    seat = ((*swaynag).seats.next as *mut ::core::ffi::c_char)
        .offset(-(72 as ::core::ffi::c_ulong as isize)) as *mut swaynag_seat
        as *mut swaynag_seat;
    tmpseat = ((*seat).link.next as *mut ::core::ffi::c_char)
        .offset(-(72 as ::core::ffi::c_ulong as isize)) as *mut swaynag_seat
        as *mut swaynag_seat;
    while &raw mut (*seat).link != &raw mut (*swaynag).seats {
        if (*seat).wl_name == name {
            swaynag_seat_destroy(seat);
        }
        seat = tmpseat;
        tmpseat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(72 as ::core::ffi::c_ulong as isize)) as *mut swaynag_seat
            as *mut swaynag_seat;
    }
}
static mut registry_listener: wl_registry_listener = wl_registry_listener {
    global: Some(
        handle_global
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_registry,
                uint32_t,
                *const ::core::ffi::c_char,
                uint32_t,
            ) -> (),
    ),
    global_remove: Some(
        handle_global_remove
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_registry,
                uint32_t,
            ) -> (),
    ),
};
#[no_mangle]
pub unsafe extern "C" fn swaynag_setup_cursors(mut swaynag: *mut swaynag) {
    let mut seat: *mut swaynag_seat = ::core::ptr::null_mut::<swaynag_seat>();
    seat = ((*swaynag).seats.next as *mut ::core::ffi::c_char)
        .offset(-(72 as ::core::ffi::c_ulong as isize)) as *mut swaynag_seat
        as *mut swaynag_seat;
    while &raw mut (*seat).link != &raw mut (*swaynag).seats {
        let mut p: *mut swaynag_pointer = &raw mut (*seat).pointer;
        (*p).cursor_surface = wl_compositor_create_surface((*swaynag).compositor);
        if !(*p).cursor_surface.is_null() {} else {
            __assert_fail(
                b"p->cursor_surface\0".as_ptr() as *const ::core::ffi::c_char,
                b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
                446 as ::core::ffi::c_uint,
                b"void swaynag_setup_cursors(struct swaynag *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        seat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(72 as ::core::ffi::c_ulong as isize)) as *mut swaynag_seat
            as *mut swaynag_seat;
    }
}
#[no_mangle]
pub unsafe extern "C" fn swaynag_setup(mut swaynag: *mut swaynag) {
    (*swaynag).display = wl_display_connect(::core::ptr::null::<::core::ffi::c_char>());
    if (*swaynag).display.is_null() {
        _sway_abort(
            b"[%s:%d] Unable to connect to the compositor. If your compositor is running, check or set the WAYLAND_DISPLAY environment variable.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
            453 as ::core::ffi::c_int,
        );
    }
    (*swaynag).scale = 1 as ::core::ffi::c_int as int32_t;
    let mut registry: *mut wl_registry = wl_display_get_registry((*swaynag).display);
    wl_registry_add_listener(
        registry,
        &raw const registry_listener,
        swaynag as *mut ::core::ffi::c_void,
    );
    if wl_display_roundtrip((*swaynag).display) < 0 as ::core::ffi::c_int {
        _sway_abort(
            b"[%s:%d] failed to register with the wayland display\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
            463 as ::core::ffi::c_int,
        );
    }
    if !(*swaynag).compositor.is_null() && !(*swaynag).layer_shell.is_null()
        && !(*swaynag).shm.is_null()
    {} else {
        __assert_fail(
            b"swaynag->compositor && swaynag->layer_shell && swaynag->shm\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
            466 as ::core::ffi::c_uint,
            b"void swaynag_setup(struct swaynag *)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if wl_display_roundtrip((*swaynag).display) < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Error during outputs init.\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
            470 as ::core::ffi::c_int,
        );
        swaynag_destroy(swaynag);
        exit(EXIT_FAILURE);
    }
    if (*swaynag).output.is_null() && !(*(*swaynag).r#type).output.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Output '%s' not found\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
            476 as ::core::ffi::c_int,
            (*(*swaynag).r#type).output,
        );
        swaynag_destroy(swaynag);
        exit(EXIT_FAILURE);
    }
    if (*swaynag).cursor_shape_manager.is_null() {
        swaynag_setup_cursors(swaynag);
    }
    (*swaynag).surface = wl_compositor_create_surface((*swaynag).compositor);
    if !(*swaynag).surface.is_null() {} else {
        __assert_fail(
            b"swaynag->surface\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
            486 as ::core::ffi::c_uint,
            b"void swaynag_setup(struct swaynag *)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    wl_surface_add_listener(
        (*swaynag).surface,
        &raw const surface_listener,
        swaynag as *mut ::core::ffi::c_void,
    );
    (*swaynag).layer_surface = zwlr_layer_shell_v1_get_layer_surface(
        (*swaynag).layer_shell,
        (*swaynag).surface,
        if !(*swaynag).output.is_null() {
            (*(*swaynag).output).wl_output
        } else {
            ::core::ptr::null_mut::<wl_output>()
        },
        (*(*swaynag).r#type).layer as uint32_t,
        b"swaynag\0".as_ptr() as *const ::core::ffi::c_char,
    ) as *mut zwlr_layer_surface_v1;
    if !(*swaynag).layer_surface.is_null() {} else {
        __assert_fail(
            b"swaynag->layer_surface\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaynag/swaynag.c\0".as_ptr() as *const ::core::ffi::c_char,
            494 as ::core::ffi::c_uint,
            b"void swaynag_setup(struct swaynag *)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    zwlr_layer_surface_v1_add_listener(
        (*swaynag).layer_surface,
        &raw const layer_surface_listener,
        swaynag as *mut ::core::ffi::c_void,
    );
    zwlr_layer_surface_v1_set_anchor(
        (*swaynag).layer_surface,
        (*(*swaynag).r#type).anchors,
    );
    wl_registry_destroy(registry);
}
#[no_mangle]
pub unsafe extern "C" fn swaynag_run(mut swaynag: *mut swaynag) {
    (*swaynag).run_display = r#true != 0;
    render_frame(swaynag);
    while (*swaynag).run_display as ::core::ffi::c_int != 0
        && wl_display_dispatch((*swaynag).display) != -1 as ::core::ffi::c_int
    {}
}
#[no_mangle]
pub unsafe extern "C" fn swaynag_destroy(mut swaynag: *mut swaynag) {
    (*swaynag).run_display = r#false != 0;
    free((*swaynag).message as *mut ::core::ffi::c_void);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*swaynag).buttons).length {
        let mut button: *mut swaynag_button = *(*(*swaynag).buttons)
            .items
            .offset(i as isize) as *mut swaynag_button;
        free((*button).text as *mut ::core::ffi::c_void);
        free((*button).action as *mut ::core::ffi::c_void);
        free(button as *mut ::core::ffi::c_void);
        i += 1;
    }
    list_free((*swaynag).buttons);
    free((*swaynag).details.message as *mut ::core::ffi::c_void);
    free((*swaynag).details.details_text as *mut ::core::ffi::c_void);
    free((*swaynag).details.button_up.text as *mut ::core::ffi::c_void);
    free((*swaynag).details.button_down.text as *mut ::core::ffi::c_void);
    if !(*swaynag).r#type.is_null() {
        swaynag_type_free((*swaynag).r#type);
    }
    if !(*swaynag).layer_surface.is_null() {
        zwlr_layer_surface_v1_destroy((*swaynag).layer_surface);
    }
    if !(*swaynag).surface.is_null() {
        wl_surface_destroy((*swaynag).surface);
    }
    let mut seat: *mut swaynag_seat = ::core::ptr::null_mut::<swaynag_seat>();
    let mut tmpseat: *mut swaynag_seat = ::core::ptr::null_mut::<swaynag_seat>();
    seat = ((*swaynag).seats.next as *mut ::core::ffi::c_char)
        .offset(-(72 as ::core::ffi::c_ulong as isize)) as *mut swaynag_seat
        as *mut swaynag_seat;
    tmpseat = ((*seat).link.next as *mut ::core::ffi::c_char)
        .offset(-(72 as ::core::ffi::c_ulong as isize)) as *mut swaynag_seat
        as *mut swaynag_seat;
    while &raw mut (*seat).link != &raw mut (*swaynag).seats {
        swaynag_seat_destroy(seat);
        seat = tmpseat;
        tmpseat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(72 as ::core::ffi::c_ulong as isize)) as *mut swaynag_seat
            as *mut swaynag_seat;
    }
    destroy_buffer((&raw mut (*swaynag).buffers as *mut pool_buffer).offset(0isize));
    destroy_buffer((&raw mut (*swaynag).buffers as *mut pool_buffer).offset(1isize));
    if !(*swaynag).outputs.prev.is_null() || !(*swaynag).outputs.next.is_null() {
        let mut output: *mut swaynag_output = ::core::ptr::null_mut::<swaynag_output>();
        let mut temp: *mut swaynag_output = ::core::ptr::null_mut::<swaynag_output>();
        output = ((*swaynag).outputs.next as *mut ::core::ffi::c_char)
            .offset(-(32 as ::core::ffi::c_ulong as isize)) as *mut swaynag_output
            as *mut swaynag_output;
        temp = ((*output).link.next as *mut ::core::ffi::c_char)
            .offset(-(32 as ::core::ffi::c_ulong as isize)) as *mut swaynag_output
            as *mut swaynag_output;
        while &raw mut (*output).link != &raw mut (*swaynag).outputs {
            wl_output_destroy((*output).wl_output);
            free((*output).name as *mut ::core::ffi::c_void);
            wl_list_remove(&raw mut (*output).link);
            free(output as *mut ::core::ffi::c_void);
            output = temp;
            temp = ((*output).link.next as *mut ::core::ffi::c_char)
                .offset(-(32 as ::core::ffi::c_ulong as isize)) as *mut swaynag_output
                as *mut swaynag_output;
        }
    }
    if !(*swaynag).compositor.is_null() {
        wl_compositor_destroy((*swaynag).compositor);
    }
    if !(*swaynag).shm.is_null() {
        wl_shm_destroy((*swaynag).shm);
    }
    if !(*swaynag).display.is_null() {
        wl_display_disconnect((*swaynag).display);
    }
}
pub const ZWLR_LAYER_SHELL_V1_GET_LAYER_SURFACE: ::core::ffi::c_int = 0
    as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn zwlr_layer_shell_v1_get_layer_surface(
    mut zwlr_layer_shell_v1: *mut zwlr_layer_shell_v1,
    mut surface: *mut wl_surface,
    mut output: *mut wl_output,
    mut layer: uint32_t,
    mut namespace: *const ::core::ffi::c_char,
) -> *mut zwlr_layer_surface_v1 {
    let mut id: *mut wl_proxy = ::core::ptr::null_mut::<wl_proxy>();
    id = wl_proxy_marshal_flags(
        zwlr_layer_shell_v1 as *mut wl_proxy,
        ZWLR_LAYER_SHELL_V1_GET_LAYER_SURFACE as uint32_t,
        &raw const zwlr_layer_surface_v1_interface,
        wl_proxy_get_version(zwlr_layer_shell_v1 as *mut wl_proxy),
        0 as uint32_t,
        NULL_0,
        surface,
        output,
        layer,
        namespace,
    );
    return id as *mut zwlr_layer_surface_v1;
}
#[inline]
unsafe extern "C" fn zwlr_layer_surface_v1_add_listener(
    mut zwlr_layer_surface_v1: *mut zwlr_layer_surface_v1,
    mut listener: *const zwlr_layer_surface_v1_listener,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return wl_proxy_add_listener(
        zwlr_layer_surface_v1 as *mut wl_proxy,
        listener as *mut Option<unsafe extern "C" fn() -> ()>,
        data,
    );
}
pub const ZWLR_LAYER_SURFACE_V1_SET_ANCHOR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ZWLR_LAYER_SURFACE_V1_ACK_CONFIGURE: ::core::ffi::c_int = 6
    as ::core::ffi::c_int;
pub const ZWLR_LAYER_SURFACE_V1_DESTROY: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn zwlr_layer_surface_v1_set_anchor(
    mut zwlr_layer_surface_v1: *mut zwlr_layer_surface_v1,
    mut anchor: uint32_t,
) {
    wl_proxy_marshal_flags(
        zwlr_layer_surface_v1 as *mut wl_proxy,
        ZWLR_LAYER_SURFACE_V1_SET_ANCHOR as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(zwlr_layer_surface_v1 as *mut wl_proxy),
        0 as uint32_t,
        anchor,
    );
}
#[inline]
unsafe extern "C" fn zwlr_layer_surface_v1_ack_configure(
    mut zwlr_layer_surface_v1: *mut zwlr_layer_surface_v1,
    mut serial: uint32_t,
) {
    wl_proxy_marshal_flags(
        zwlr_layer_surface_v1 as *mut wl_proxy,
        ZWLR_LAYER_SURFACE_V1_ACK_CONFIGURE as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(zwlr_layer_surface_v1 as *mut wl_proxy),
        0 as uint32_t,
        serial,
    );
}
#[inline]
unsafe extern "C" fn zwlr_layer_surface_v1_destroy(
    mut zwlr_layer_surface_v1: *mut zwlr_layer_surface_v1,
) {
    wl_proxy_marshal_flags(
        zwlr_layer_surface_v1 as *mut wl_proxy,
        ZWLR_LAYER_SURFACE_V1_DESTROY as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(zwlr_layer_surface_v1 as *mut wl_proxy),
        WL_MARSHAL_FLAG_DESTROY as uint32_t,
    );
}
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __S_IEXEC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
