use ::c2rust_bitfields;
extern "C" {
    pub type wl_proxy;
    pub type wl_display;
    pub type wl_buffer;
    pub type wl_compositor;
    pub type wl_output;
    pub type wl_pointer;
    pub type wl_region;
    pub type wl_registry;
    pub type wl_seat;
    pub type wl_shm;
    pub type wl_surface;
    pub type wl_touch;
    pub type wl_cursor_theme;
    pub type _PangoContext;
    pub type _PangoFontDescription;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _cairo;
    pub type _cairo_surface;
    pub type swaybar_watcher;
    pub type sd_bus;
    pub type r#loop;
    pub type json_object;
    pub type wp_cursor_shape_manager_v1;
    pub type zxdg_output_manager_v1;
    pub type zwlr_layer_shell_v1;
    pub type zwlr_layer_surface_v1;
    pub type zxdg_output_v1;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn wl_list_init(list: *mut wl_list);
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn wl_list_empty(list: *const wl_list) -> ::core::ffi::c_int;
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
    fn wl_display_get_fd(display: *mut wl_display) -> ::core::ffi::c_int;
    fn wl_display_dispatch(display: *mut wl_display) -> ::core::ffi::c_int;
    fn wl_display_flush(display: *mut wl_display) -> ::core::ffi::c_int;
    fn wl_display_roundtrip(display: *mut wl_display) -> ::core::ffi::c_int;
    static wl_registry_interface: wl_interface;
    static wl_compositor_interface: wl_interface;
    static wl_shm_interface: wl_interface;
    static wl_surface_interface: wl_interface;
    static wl_seat_interface: wl_interface;
    static wl_output_interface: wl_interface;
    static wl_region_interface: wl_interface;
    fn init_config() -> *mut swaybar_config;
    fn free_config(config: *mut swaybar_config);
    static seat_listener: wl_seat_listener;
    fn update_cursor(seat: *mut swaybar_seat);
    fn free_hotspots(list: *mut wl_list);
    fn swaybar_seat_free(seat: *mut swaybar_seat);
    static zwlr_layer_shell_v1_interface: wl_interface;
    static zwlr_layer_surface_v1_interface: wl_interface;
    static zxdg_output_manager_v1_interface: wl_interface;
    static zxdg_output_v1_interface: wl_interface;
    fn status_line_init(cmd: *mut ::core::ffi::c_char) -> *mut status_line;
    fn status_error(status: *mut status_line, text: *const ::core::ffi::c_char);
    fn status_handle_readable(status: *mut status_line) -> bool;
    fn status_line_free(status: *mut status_line);
    static wp_cursor_shape_manager_v1_interface: wl_interface;
    fn destroy_buffer(buffer: *mut pool_buffer);
    fn ipc_initialize(bar: *mut swaybar) -> bool;
    fn handle_ipc_readable(bar: *mut swaybar) -> bool;
    fn ipc_get_workspaces(bar: *mut swaybar) -> bool;
    fn render_frame(output: *mut swaybar_output);
    fn create_tray(bar: *mut swaybar) -> *mut swaybar_tray;
    fn destroy_tray(tray: *mut swaybar_tray);
    fn tray_in(
        fd: ::core::ffi::c_int,
        mask: ::core::ffi::c_short,
        data: *mut ::core::ffi::c_void,
    );
    fn ipc_open_socket(socket_path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn loop_create() -> *mut r#loop;
    fn loop_poll(r#loop: *mut r#loop);
    fn loop_add_fd(
        r#loop: *mut r#loop,
        fd: ::core::ffi::c_int,
        mask: ::core::ffi::c_short,
        func: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                ::core::ffi::c_short,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        data: *mut ::core::ffi::c_void,
    );
    fn loop_remove_fd(r#loop: *mut r#loop, fd: ::core::ffi::c_int) -> bool;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_abort(filename: *const ::core::ffi::c_char, ...);
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type pid_t = __pid_t;
pub type size_t = usize;
pub type int32_t = i32;
pub type uint32_t = u32;
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
pub struct wl_output_subpixel(pub ::core::ffi::c_uint);
impl wl_output_subpixel {
    pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: Self = Self(0);
    pub const WL_OUTPUT_SUBPIXEL_NONE: Self = Self(1);
    pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: Self = Self(2);
    pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: Self = Self(3);
    pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: Self = Self(4);
    pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: Self = Self(5);
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
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
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
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct box_colors {
    pub border: uint32_t,
    pub background: uint32_t,
    pub text: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_output {
    pub link: wl_list,
    pub name: *mut ::core::ffi::c_char,
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
    pub gaps: C2Rust_Unnamed_0,
    pub colors: C2Rust_Unnamed,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_hidden: bool,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
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
pub struct C2Rust_Unnamed_0 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_tray {
    pub bar: *mut swaybar,
    pub fd: ::core::ffi::c_int,
    pub bus: *mut sd_bus,
    pub host_xdg: swaybar_host,
    pub host_kde: swaybar_host,
    pub items: *mut list_t,
    pub watcher_xdg: *mut swaybar_watcher,
    pub watcher_kde: *mut swaybar_watcher,
    pub basedirs: *mut list_t,
    pub themes: *mut list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_host {
    pub tray: *mut swaybar_tray,
    pub service: *mut ::core::ffi::c_char,
    pub watcher_interface: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_line {
    pub bar: *mut swaybar,
    pub pid: pid_t,
    pub read_fd: ::core::ffi::c_int,
    pub write_fd: ::core::ffi::c_int,
    pub read: *mut FILE,
    pub write: *mut FILE,
    pub protocol: status_protocol,
    pub text: *const ::core::ffi::c_char,
    pub blocks: wl_list,
    pub stop_signal: ::core::ffi::c_int,
    pub cont_signal: ::core::ffi::c_int,
    pub click_events: bool,
    pub float_event_coords: bool,
    pub clicked: bool,
    pub buffer: *mut ::core::ffi::c_char,
    pub buffer_size: size_t,
    pub buffer_index: size_t,
    pub started: bool,
    pub expecting_comma: bool,
    pub tokener: *mut json_tokener,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_tokener_srec {
    pub state: json_tokener_state,
    pub saved_state: json_tokener_state,
    pub obj: *mut json_object,
    pub current: *mut json_object,
    pub obj_field_name: *mut ::core::ffi::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub bpos: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct status_protocol(pub ::core::ffi::c_uint);
impl status_protocol {
    pub const PROTOCOL_UNDEF: Self = Self(0);
    pub const PROTOCOL_ERROR: Self = Self(1);
    pub const PROTOCOL_TEXT: Self = Self(2);
    pub const PROTOCOL_I3BAR: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_output {
    pub link: wl_list,
    pub bar: *mut swaybar,
    pub output: *mut wl_output,
    pub xdg_output: *mut zxdg_output_v1,
    pub surface: *mut wl_surface,
    pub layer_surface: *mut zwlr_layer_surface_v1,
    pub wl_name: uint32_t,
    pub workspaces: wl_list,
    pub hotspots: wl_list,
    pub name: *mut ::core::ffi::c_char,
    pub identifier: *mut ::core::ffi::c_char,
    pub focused: bool,
    pub width: uint32_t,
    pub height: uint32_t,
    pub scale: int32_t,
    pub subpixel: wl_output_subpixel,
    pub buffers: [pool_buffer; 2],
    pub current_buffer: *mut pool_buffer,
    pub dirty: bool,
    pub frame_scheduled: bool,
    pub output_height: uint32_t,
    pub output_width: uint32_t,
    pub output_x: uint32_t,
    pub output_y: uint32_t,
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
pub struct swaybar_pointer {
    pub pointer: *mut wl_pointer,
    pub cursor_theme: *mut wl_cursor_theme,
    pub cursor_image: *mut wl_cursor_image,
    pub cursor_surface: *mut wl_surface,
    pub current: *mut swaybar_output,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub serial: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct touch_slot {
    pub id: int32_t,
    pub time: uint32_t,
    pub output: *mut swaybar_output,
    pub start_x: ::core::ffi::c_double,
    pub start_y: ::core::ffi::c_double,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_touch {
    pub touch: *mut wl_touch,
    pub slots: [touch_slot; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_scroll_axis {
    pub value: wl_fixed_t,
    pub discrete_steps: uint32_t,
    pub update_time: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_seat {
    pub bar: *mut swaybar,
    pub wl_name: uint32_t,
    pub wl_seat: *mut wl_seat,
    pub pointer: swaybar_pointer,
    pub touch: swaybar_touch,
    pub link: wl_list,
    pub axis: [swaybar_scroll_axis; 2],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct zwlr_layer_shell_v1_layer(pub ::core::ffi::c_uint);
impl zwlr_layer_shell_v1_layer {
    pub const ZWLR_LAYER_SHELL_V1_LAYER_BACKGROUND: Self = Self(0);
    pub const ZWLR_LAYER_SHELL_V1_LAYER_BOTTOM: Self = Self(1);
    pub const ZWLR_LAYER_SHELL_V1_LAYER_TOP: Self = Self(2);
    pub const ZWLR_LAYER_SHELL_V1_LAYER_OVERLAY: Self = Self(3);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zxdg_output_v1_listener {
    pub logical_position: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut zxdg_output_v1,
            int32_t,
            int32_t,
        ) -> (),
    >,
    pub logical_size: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut zxdg_output_v1,
            int32_t,
            int32_t,
        ) -> (),
    >,
    pub done: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut zxdg_output_v1) -> (),
    >,
    pub name: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut zxdg_output_v1,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub description: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut zxdg_output_v1,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_workspace {
    pub link: wl_list,
    pub num: ::core::ffi::c_int,
    pub name: *mut ::core::ffi::c_char,
    pub label: *mut ::core::ffi::c_char,
    pub focused: bool,
    pub visible: bool,
    pub urgent: bool,
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
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
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
pub const WL_COMPOSITOR_CREATE_REGION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
unsafe extern "C" fn wl_compositor_create_region(
    mut wl_compositor: *mut wl_compositor,
) -> *mut wl_region {
    let mut id: *mut wl_proxy = ::core::ptr::null_mut::<wl_proxy>();
    id = wl_proxy_marshal_flags(
        wl_compositor as *mut wl_proxy,
        WL_COMPOSITOR_CREATE_REGION as uint32_t,
        &raw const wl_region_interface,
        wl_proxy_get_version(wl_compositor as *mut wl_proxy),
        0 as uint32_t,
        NULL,
    );
    return id as *mut wl_region;
}
pub const WL_SURFACE_DESTROY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WL_SURFACE_ATTACH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WL_SURFACE_SET_INPUT_REGION: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
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
unsafe extern "C" fn wl_surface_set_input_region(
    mut wl_surface: *mut wl_surface,
    mut region: *mut wl_region,
) {
    wl_proxy_marshal_flags(
        wl_surface as *mut wl_proxy,
        WL_SURFACE_SET_INPUT_REGION as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_surface as *mut wl_proxy),
        0 as uint32_t,
        region,
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
pub const WL_REGION_DESTROY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_region_destroy(mut wl_region: *mut wl_region) {
    wl_proxy_marshal_flags(
        wl_region as *mut wl_proxy,
        WL_REGION_DESTROY as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_region as *mut wl_proxy),
        WL_MARSHAL_FLAG_DESTROY as uint32_t,
    );
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
pub const ZWLR_LAYER_SURFACE_V1_SET_EXCLUSIVE_ZONE: ::core::ffi::c_int = 2
    as ::core::ffi::c_int;
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
unsafe extern "C" fn zwlr_layer_surface_v1_set_exclusive_zone(
    mut zwlr_layer_surface_v1: *mut zwlr_layer_surface_v1,
    mut zone: int32_t,
) {
    wl_proxy_marshal_flags(
        zwlr_layer_surface_v1 as *mut wl_proxy,
        ZWLR_LAYER_SURFACE_V1_SET_EXCLUSIVE_ZONE as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(zwlr_layer_surface_v1 as *mut wl_proxy),
        0 as uint32_t,
        zone,
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
pub const ZXDG_OUTPUT_MANAGER_V1_GET_XDG_OUTPUT: ::core::ffi::c_int = 1
    as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn zxdg_output_manager_v1_get_xdg_output(
    mut zxdg_output_manager_v1: *mut zxdg_output_manager_v1,
    mut output: *mut wl_output,
) -> *mut zxdg_output_v1 {
    let mut id: *mut wl_proxy = ::core::ptr::null_mut::<wl_proxy>();
    id = wl_proxy_marshal_flags(
        zxdg_output_manager_v1 as *mut wl_proxy,
        ZXDG_OUTPUT_MANAGER_V1_GET_XDG_OUTPUT as uint32_t,
        &raw const zxdg_output_v1_interface,
        wl_proxy_get_version(zxdg_output_manager_v1 as *mut wl_proxy),
        0 as uint32_t,
        NULL_0,
        output,
    );
    return id as *mut zxdg_output_v1;
}
#[inline]
unsafe extern "C" fn zxdg_output_v1_add_listener(
    mut zxdg_output_v1: *mut zxdg_output_v1,
    mut listener: *const zxdg_output_v1_listener,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return wl_proxy_add_listener(
        zxdg_output_v1 as *mut wl_proxy,
        listener as *mut Option<unsafe extern "C" fn() -> ()>,
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn free_workspaces(mut list: *mut wl_list) {
    let mut ws: *mut swaybar_workspace = ::core::ptr::null_mut::<swaybar_workspace>();
    let mut tmp: *mut swaybar_workspace = ::core::ptr::null_mut::<swaybar_workspace>();
    ws = ((*list).next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_workspace
        as *mut swaybar_workspace;
    tmp = ((*ws).link.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_workspace
        as *mut swaybar_workspace;
    while &raw mut (*ws).link != list {
        wl_list_remove(&raw mut (*ws).link);
        free((*ws).name as *mut ::core::ffi::c_void);
        free((*ws).label as *mut ::core::ffi::c_void);
        free(ws as *mut ::core::ffi::c_void);
        ws = tmp;
        tmp = ((*ws).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_workspace
            as *mut swaybar_workspace;
    }
}
unsafe extern "C" fn swaybar_output_free(mut output: *mut swaybar_output) {
    if output.is_null() {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Removing output %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
        46 as ::core::ffi::c_int,
        (*output).name,
    );
    if !(*output).layer_surface.is_null() {
        zwlr_layer_surface_v1_destroy((*output).layer_surface);
    }
    if !(*output).surface.is_null() {
        wl_surface_destroy((*output).surface);
    }
    wl_output_destroy((*output).output);
    destroy_buffer((&raw mut (*output).buffers as *mut pool_buffer).offset(0isize));
    destroy_buffer((&raw mut (*output).buffers as *mut pool_buffer).offset(1isize));
    free_hotspots(&raw mut (*output).hotspots);
    free_workspaces(&raw mut (*output).workspaces);
    wl_list_remove(&raw mut (*output).link);
    free((*output).name as *mut ::core::ffi::c_void);
    free((*output).identifier as *mut ::core::ffi::c_void);
    free(output as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn set_output_dirty(mut output: *mut swaybar_output) {
    if (*output).frame_scheduled {
        (*output).dirty = r#true != 0;
    } else if !(*output).surface.is_null() {
        render_frame(output);
    }
}
unsafe extern "C" fn layer_surface_configure(
    mut data: *mut ::core::ffi::c_void,
    mut surface: *mut zwlr_layer_surface_v1,
    mut serial: uint32_t,
    mut width: uint32_t,
    mut height: uint32_t,
) {
    let mut output: *mut swaybar_output = data as *mut swaybar_output;
    (*output).width = width;
    (*output).height = height;
    zwlr_layer_surface_v1_ack_configure(surface, serial);
    set_output_dirty(output);
}
unsafe extern "C" fn layer_surface_closed(
    mut _output: *mut ::core::ffi::c_void,
    mut surface: *mut zwlr_layer_surface_v1,
) {
    let mut output: *mut swaybar_output = _output as *mut swaybar_output;
    swaybar_output_free(output);
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
unsafe extern "C" fn add_layer_surface(mut output: *mut swaybar_output) {
    if !(*output).layer_surface.is_null() {
        return;
    }
    let mut bar: *mut swaybar = (*output).bar;
    let mut config: *mut swaybar_config = (*bar).config;
    let mut hidden: bool = strcmp(
        (*config).mode,
        b"hide\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    let mut overlay: bool = !hidden
        && strcmp((*config).mode, b"overlay\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int;
    (*output).layer_surface = zwlr_layer_shell_v1_get_layer_surface(
        (*bar).layer_shell,
        (*output).surface,
        (*output).output,
        (if hidden as ::core::ffi::c_int != 0 || overlay as ::core::ffi::c_int != 0 {
            zwlr_layer_shell_v1_layer::ZWLR_LAYER_SHELL_V1_LAYER_OVERLAY.0
                as ::core::ffi::c_int
        } else {
            zwlr_layer_shell_v1_layer::ZWLR_LAYER_SHELL_V1_LAYER_BOTTOM.0
                as ::core::ffi::c_int
        }) as uint32_t,
        b"panel\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !(*output).layer_surface.is_null() {} else {
        __assert_fail(
            b"output->layer_surface\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
            106 as ::core::ffi::c_uint,
            b"void add_layer_surface(struct swaybar_output *)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    zwlr_layer_surface_v1_add_listener(
        (*output).layer_surface,
        &raw const layer_surface_listener,
        output as *mut ::core::ffi::c_void,
    );
    if overlay {
        let mut region: *mut wl_region = wl_compositor_create_region((*bar).compositor);
        wl_surface_set_input_region((*output).surface, region);
        wl_region_destroy(region);
    }
    zwlr_layer_surface_v1_set_anchor((*output).layer_surface, (*config).position);
    if hidden as ::core::ffi::c_int != 0 || overlay as ::core::ffi::c_int != 0 {
        zwlr_layer_surface_v1_set_exclusive_zone((*output).layer_surface, -1 as int32_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn destroy_layer_surface(mut output: *mut swaybar_output) {
    if (*output).layer_surface.is_null() {
        return;
    }
    zwlr_layer_surface_v1_destroy((*output).layer_surface);
    wl_surface_attach(
        (*output).surface,
        ::core::ptr::null_mut::<wl_buffer>(),
        0 as int32_t,
        0 as int32_t,
    );
    (*output).layer_surface = ::core::ptr::null_mut::<zwlr_layer_surface_v1>();
    (*output).width = 0 as uint32_t;
    (*output).frame_scheduled = r#false != 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_bar_dirty(mut bar: *mut swaybar) {
    let mut output: *mut swaybar_output = ::core::ptr::null_mut::<swaybar_output>();
    output = ((*bar).outputs.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    while &raw mut (*output).link != &raw mut (*bar).outputs {
        set_output_dirty(output);
        output = ((*output).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
            as *mut swaybar_output;
    }
}
#[no_mangle]
pub unsafe extern "C" fn determine_bar_visibility(
    mut bar: *mut swaybar,
    mut moving_layer: bool,
) -> bool {
    let mut config: *mut swaybar_config = (*bar).config;
    let mut visible: bool = !(strcmp(
        (*config).mode,
        b"invisible\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp((*config).mode, (*config).hidden_state) == 0 as ::core::ffi::c_int
            && !(*bar).visible_by_modifier && !(*bar).visible_by_urgency
            && !(*bar).visible_by_mode);
    let mut output: *mut swaybar_output = ::core::ptr::null_mut::<swaybar_output>();
    output = ((*bar).outputs.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    while &raw mut (*output).link != &raw mut (*bar).outputs {
        if !visible || moving_layer as ::core::ffi::c_int != 0 {
            destroy_layer_surface(output);
        }
        if visible {
            add_layer_surface(output);
        }
        output = ((*output).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
            as *mut swaybar_output;
    }
    set_bar_dirty(bar);
    if visible as ::core::ffi::c_int != (*bar).visible as ::core::ffi::c_int {
        (*bar).visible = visible;
        if !(*bar).status.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Sending %s signal to status command\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
                167 as ::core::ffi::c_int,
                if visible as ::core::ffi::c_int != 0 {
                    b"cont\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"stop\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            kill(
                -(*(*bar).status).pid,
                if visible as ::core::ffi::c_int != 0 {
                    (*(*bar).status).cont_signal
                } else {
                    (*(*bar).status).stop_signal
                },
            );
        }
    }
    return visible;
}
unsafe extern "C" fn bar_uses_output(mut output: *mut swaybar_output) -> bool {
    if wl_list_empty(&raw mut (*(*(*output).bar).config).outputs) != 0 {
        return r#true != 0;
    }
    let mut identifier: *mut ::core::ffi::c_char = (*output).identifier;
    let mut coutput: *mut config_output = ::core::ptr::null_mut::<config_output>();
    coutput = ((*(*(*output).bar).config).outputs.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
        as *mut config_output;
    while &raw mut (*coutput).link != &raw mut (*(*(*output).bar).config).outputs {
        if strcmp((*coutput).name, (*output).name) == 0 as ::core::ffi::c_int
            || !identifier.is_null()
                && strcmp((*coutput).name, identifier) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        coutput = ((*coutput).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
            as *mut config_output;
    }
    return r#false != 0;
}
unsafe extern "C" fn output_geometry(
    mut data: *mut ::core::ffi::c_void,
    mut wl_output: *mut wl_output,
    mut x: int32_t,
    mut y: int32_t,
    mut width_mm: int32_t,
    mut height_mm: int32_t,
    mut subpixel: int32_t,
    mut make: *const ::core::ffi::c_char,
    mut model: *const ::core::ffi::c_char,
    mut transform: int32_t,
) {
    let mut output: *mut swaybar_output = data as *mut swaybar_output;
    (*output).subpixel = wl_output_subpixel(subpixel as ::core::ffi::c_uint);
}
unsafe extern "C" fn output_mode(
    mut data: *mut ::core::ffi::c_void,
    mut wl_output: *mut wl_output,
    mut flags: uint32_t,
    mut width: int32_t,
    mut height: int32_t,
    mut refresh: int32_t,
) {}
unsafe extern "C" fn output_done(
    mut data: *mut ::core::ffi::c_void,
    mut wl_output: *mut wl_output,
) {
    let mut output: *mut swaybar_output = data as *mut swaybar_output;
    set_output_dirty(output);
}
unsafe extern "C" fn output_scale(
    mut data: *mut ::core::ffi::c_void,
    mut wl_output: *mut wl_output,
    mut factor: int32_t,
) {
    let mut output: *mut swaybar_output = data as *mut swaybar_output;
    (*output).scale = factor;
    let mut render: bool = r#false != 0;
    let mut seat: *mut swaybar_seat = ::core::ptr::null_mut::<swaybar_seat>();
    seat = ((*(*output).bar).seats.next as *mut ::core::ffi::c_char)
        .offset(-(864 as ::core::ffi::c_ulong as isize)) as *mut swaybar_seat
        as *mut swaybar_seat;
    while &raw mut (*seat).link != &raw mut (*(*output).bar).seats {
        if output == (*seat).pointer.current {
            update_cursor(seat);
            render = r#true != 0;
        }
        seat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(864 as ::core::ffi::c_ulong as isize)) as *mut swaybar_seat
            as *mut swaybar_seat;
    }
    if render {
        render_frame(output);
    }
}
static mut output_listener: wl_output_listener = wl_output_listener {
    geometry: Some(
        output_geometry
            as unsafe extern "C" fn(
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
    ),
    mode: Some(
        output_mode
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_output,
                uint32_t,
                int32_t,
                int32_t,
                int32_t,
            ) -> (),
    ),
    done: Some(
        output_done
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_output) -> (),
    ),
    scale: Some(
        output_scale
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_output,
                int32_t,
            ) -> (),
    ),
    name: None,
    description: None,
};
unsafe extern "C" fn xdg_output_handle_logical_position(
    mut data: *mut ::core::ffi::c_void,
    mut xdg_output: *mut zxdg_output_v1,
    mut x: int32_t,
    mut y: int32_t,
) {
    let mut output: *mut swaybar_output = data as *mut swaybar_output;
    (*output).output_x = x as uint32_t;
    (*output).output_y = y as uint32_t;
}
unsafe extern "C" fn xdg_output_handle_logical_size(
    mut data: *mut ::core::ffi::c_void,
    mut xdg_output: *mut zxdg_output_v1,
    mut width: int32_t,
    mut height: int32_t,
) {
    let mut output: *mut swaybar_output = data as *mut swaybar_output;
    (*output).output_height = height as uint32_t;
    (*output).output_width = width as uint32_t;
}
unsafe extern "C" fn xdg_output_handle_done(
    mut data: *mut ::core::ffi::c_void,
    mut xdg_output: *mut zxdg_output_v1,
) {
    let mut output: *mut swaybar_output = data as *mut swaybar_output;
    let mut bar: *mut swaybar = (*output).bar;
    if wl_list_empty(&raw mut (*output).link) == 0 {
        return;
    }
    if !(*output).name.is_null() {} else {
        __assert_fail(
            b"output->name != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
            257 as ::core::ffi::c_uint,
            b"void xdg_output_handle_done(void *, struct zxdg_output_v1 *)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if !bar_uses_output(output) {
        wl_list_remove(&raw mut (*output).link);
        wl_list_insert(&raw mut (*bar).unused_outputs, &raw mut (*output).link);
        return;
    }
    wl_list_remove(&raw mut (*output).link);
    wl_list_insert(&raw mut (*bar).outputs, &raw mut (*output).link);
    (*output).surface = wl_compositor_create_surface((*bar).compositor);
    if !(*output).surface.is_null() {} else {
        __assert_fail(
            b"output->surface\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
            268 as ::core::ffi::c_uint,
            b"void xdg_output_handle_done(void *, struct zxdg_output_v1 *)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    determine_bar_visibility(bar, r#false != 0);
    if (*bar).running as ::core::ffi::c_int != 0
        && (*(*bar).config).workspace_buttons as ::core::ffi::c_int != 0
    {
        ipc_get_workspaces(bar);
    }
}
unsafe extern "C" fn xdg_output_handle_name(
    mut data: *mut ::core::ffi::c_void,
    mut xdg_output: *mut zxdg_output_v1,
    mut name: *const ::core::ffi::c_char,
) {
    let mut output: *mut swaybar_output = data as *mut swaybar_output;
    free((*output).name as *mut ::core::ffi::c_void);
    (*output).name = strdup(name);
}
unsafe extern "C" fn xdg_output_handle_description(
    mut data: *mut ::core::ffi::c_void,
    mut xdg_output: *mut zxdg_output_v1,
    mut description: *const ::core::ffi::c_char,
) {
    let mut output: *mut swaybar_output = data as *mut swaybar_output;
    free((*output).identifier as *mut ::core::ffi::c_void);
    (*output).identifier = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut paren: *mut ::core::ffi::c_char = strrchr(
        description,
        '(' as ::core::ffi::c_int,
    );
    if !paren.is_null() {
        let mut length: size_t = paren.offset_from(description) as size_t;
        (*output).identifier = malloc(length) as *mut ::core::ffi::c_char;
        if (*output).identifier.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Failed to allocate output identifier\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
                296 as ::core::ffi::c_int,
            );
            return;
        }
        strncpy((*output).identifier, description, length);
        *(*output).identifier.offset(length.wrapping_sub(1 as size_t) as isize) = '\0'
            as ::core::ffi::c_char;
    }
}
static mut xdg_output_listener: zxdg_output_v1_listener = zxdg_output_v1_listener {
    logical_position: Some(
        xdg_output_handle_logical_position
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut zxdg_output_v1,
                int32_t,
                int32_t,
            ) -> (),
    ),
    logical_size: Some(
        xdg_output_handle_logical_size
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut zxdg_output_v1,
                int32_t,
                int32_t,
            ) -> (),
    ),
    done: Some(
        xdg_output_handle_done
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut zxdg_output_v1) -> (),
    ),
    name: Some(
        xdg_output_handle_name
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut zxdg_output_v1,
                *const ::core::ffi::c_char,
            ) -> (),
    ),
    description: Some(
        xdg_output_handle_description
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut zxdg_output_v1,
                *const ::core::ffi::c_char,
            ) -> (),
    ),
};
unsafe extern "C" fn add_xdg_output(mut output: *mut swaybar_output) {
    if !(*output).xdg_output.is_null() {
        return;
    }
    if !(*(*output).bar).xdg_output_manager.is_null() {} else {
        __assert_fail(
            b"output->bar->xdg_output_manager != NULL\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
            316 as ::core::ffi::c_uint,
            b"void add_xdg_output(struct swaybar_output *)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    (*output).xdg_output = zxdg_output_manager_v1_get_xdg_output(
        (*(*output).bar).xdg_output_manager,
        (*output).output,
    );
    zxdg_output_v1_add_listener(
        (*output).xdg_output,
        &raw const xdg_output_listener,
        output as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn handle_global(
    mut data: *mut ::core::ffi::c_void,
    mut registry: *mut wl_registry,
    mut name: uint32_t,
    mut interface: *const ::core::ffi::c_char,
    mut version: uint32_t,
) {
    let mut bar: *mut swaybar = data as *mut swaybar;
    if strcmp(interface, wl_compositor_interface.name) == 0 as ::core::ffi::c_int {
        (*bar).compositor = wl_registry_bind(
            registry,
            name,
            &raw const wl_compositor_interface,
            4 as uint32_t,
        ) as *mut wl_compositor;
    } else if strcmp(interface, wl_seat_interface.name) == 0 as ::core::ffi::c_int {
        let mut seat: *mut swaybar_seat = calloc(
            1 as size_t,
            ::core::mem::size_of::<swaybar_seat>(),
        ) as *mut swaybar_seat;
        if seat.is_null() {
            _sway_abort(
                b"[%s:%d] Failed to allocate swaybar_seat\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
                332 as ::core::ffi::c_int,
            );
            return;
        }
        (*seat).bar = bar;
        (*seat).wl_name = name;
        (*seat).wl_seat = wl_registry_bind(
            registry,
            name,
            &raw const wl_seat_interface,
            5 as uint32_t,
        ) as *mut wl_seat;
        wl_seat_add_listener(
            (*seat).wl_seat,
            &raw const seat_listener,
            seat as *mut ::core::ffi::c_void,
        );
        wl_list_insert(&raw mut (*bar).seats, &raw mut (*seat).link);
    } else if strcmp(interface, wl_shm_interface.name) == 0 as ::core::ffi::c_int {
        (*bar).shm = wl_registry_bind(
            registry,
            name,
            &raw const wl_shm_interface,
            1 as uint32_t,
        ) as *mut wl_shm;
    } else if strcmp(interface, wl_output_interface.name) == 0 as ::core::ffi::c_int {
        let mut output: *mut swaybar_output = calloc(
            1 as size_t,
            ::core::mem::size_of::<swaybar_output>(),
        ) as *mut swaybar_output;
        (*output).bar = bar;
        (*output).output = wl_registry_bind(
            registry,
            name,
            &raw const wl_output_interface,
            3 as uint32_t,
        ) as *mut wl_output;
        wl_output_add_listener(
            (*output).output,
            &raw const output_listener,
            output as *mut ::core::ffi::c_void,
        );
        (*output).scale = 1 as ::core::ffi::c_int as int32_t;
        (*output).wl_name = name;
        wl_list_init(&raw mut (*output).workspaces);
        wl_list_init(&raw mut (*output).hotspots);
        wl_list_init(&raw mut (*output).link);
        if !(*bar).xdg_output_manager.is_null() {
            add_xdg_output(output);
        }
    } else if strcmp(interface, zwlr_layer_shell_v1_interface.name)
        == 0 as ::core::ffi::c_int
    {
        (*bar).layer_shell = wl_registry_bind(
            registry,
            name,
            &raw const zwlr_layer_shell_v1_interface,
            1 as uint32_t,
        ) as *mut zwlr_layer_shell_v1;
    } else if strcmp(interface, zxdg_output_manager_v1_interface.name)
        == 0 as ::core::ffi::c_int
    {
        (*bar).xdg_output_manager = wl_registry_bind(
            registry,
            name,
            &raw const zxdg_output_manager_v1_interface,
            2 as uint32_t,
        ) as *mut zxdg_output_manager_v1;
    } else if strcmp(interface, wp_cursor_shape_manager_v1_interface.name)
        == 0 as ::core::ffi::c_int
    {
        (*bar).cursor_shape_manager = wl_registry_bind(
            registry,
            name,
            &raw const wp_cursor_shape_manager_v1_interface,
            1 as uint32_t,
        ) as *mut wp_cursor_shape_manager_v1;
    }
}
unsafe extern "C" fn handle_global_remove(
    mut data: *mut ::core::ffi::c_void,
    mut registry: *mut wl_registry,
    mut name: uint32_t,
) {
    let mut bar: *mut swaybar = data as *mut swaybar;
    let mut output: *mut swaybar_output = ::core::ptr::null_mut::<swaybar_output>();
    let mut tmp: *mut swaybar_output = ::core::ptr::null_mut::<swaybar_output>();
    output = ((*bar).outputs.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    tmp = ((*output).link.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    while &raw mut (*output).link != &raw mut (*bar).outputs {
        if (*output).wl_name == name {
            swaybar_output_free(output);
            return;
        }
        output = tmp;
        tmp = ((*output).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
            as *mut swaybar_output;
    }
    output = ((*bar).unused_outputs.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    tmp = ((*output).link.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    while &raw mut (*output).link != &raw mut (*bar).unused_outputs {
        if (*output).wl_name == name {
            swaybar_output_free(output);
            return;
        }
        output = tmp;
        tmp = ((*output).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
            as *mut swaybar_output;
    }
    let mut seat: *mut swaybar_seat = ::core::ptr::null_mut::<swaybar_seat>();
    let mut tmp_seat: *mut swaybar_seat = ::core::ptr::null_mut::<swaybar_seat>();
    seat = ((*bar).seats.next as *mut ::core::ffi::c_char)
        .offset(-(864 as ::core::ffi::c_ulong as isize)) as *mut swaybar_seat
        as *mut swaybar_seat;
    tmp_seat = ((*seat).link.next as *mut ::core::ffi::c_char)
        .offset(-(864 as ::core::ffi::c_ulong as isize)) as *mut swaybar_seat
        as *mut swaybar_seat;
    while &raw mut (*seat).link != &raw mut (*bar).seats {
        if (*seat).wl_name == name {
            swaybar_seat_free(seat);
            return;
        }
        seat = tmp_seat;
        tmp_seat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(864 as ::core::ffi::c_ulong as isize)) as *mut swaybar_seat
            as *mut swaybar_seat;
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
pub unsafe extern "C" fn bar_setup(
    mut bar: *mut swaybar,
    mut socket_path: *const ::core::ffi::c_char,
) -> bool {
    (*bar).visible = r#true != 0;
    (*bar).config = init_config();
    wl_list_init(&raw mut (*bar).outputs);
    wl_list_init(&raw mut (*bar).unused_outputs);
    wl_list_init(&raw mut (*bar).seats);
    (*bar).eventloop = loop_create();
    (*bar).ipc_socketfd = ipc_open_socket(socket_path);
    (*bar).ipc_event_socketfd = ipc_open_socket(socket_path);
    if !ipc_initialize(bar) {
        return r#false != 0;
    }
    (*bar).display = wl_display_connect(::core::ptr::null::<::core::ffi::c_char>());
    if (*bar).display.is_null() {
        _sway_abort(
            b"[%s:%d] Unable to connect to the compositor. If your compositor is running, check or set the WAYLAND_DISPLAY environment variable.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
            416 as ::core::ffi::c_int,
        );
    }
    let mut registry: *mut wl_registry = wl_display_get_registry((*bar).display);
    wl_registry_add_listener(
        registry,
        &raw const registry_listener,
        bar as *mut ::core::ffi::c_void,
    );
    wl_display_roundtrip((*bar).display);
    if !(*bar).compositor.is_null() && !(*bar).layer_shell.is_null()
        && !(*bar).shm.is_null() && !(*bar).xdg_output_manager.is_null()
    {} else {
        __assert_fail(
            b"bar->compositor && bar->layer_shell && bar->shm && bar->xdg_output_manager\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
            425 as ::core::ffi::c_uint,
            b"_Bool bar_setup(struct swaybar *, const char *)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    wl_display_roundtrip((*bar).display);
    if (*bar).cursor_shape_manager.is_null() {
        let mut seat: *mut swaybar_seat = ::core::ptr::null_mut::<swaybar_seat>();
        seat = ((*bar).seats.next as *mut ::core::ffi::c_char)
            .offset(-(864 as ::core::ffi::c_ulong as isize)) as *mut swaybar_seat
            as *mut swaybar_seat;
        while &raw mut (*seat).link != &raw mut (*bar).seats {
            let mut pointer: *mut swaybar_pointer = &raw mut (*seat).pointer;
            if !pointer.is_null() {
                (*pointer).cursor_surface = wl_compositor_create_surface(
                    (*bar).compositor,
                );
                if !(*pointer).cursor_surface.is_null() {} else {
                    __assert_fail(
                        b"pointer->cursor_surface\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
                        439 as ::core::ffi::c_uint,
                        b"_Bool bar_setup(struct swaybar *, const char *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
            }
            seat = ((*seat).link.next as *mut ::core::ffi::c_char)
                .offset(-(864 as ::core::ffi::c_ulong as isize)) as *mut swaybar_seat
                as *mut swaybar_seat;
        }
    }
    if !(*(*bar).config).status_command.is_null() {
        (*bar).status = status_line_init((*(*bar).config).status_command)
            as *mut status_line;
        (*(*bar).status).bar = bar;
    }
    if !(*(*bar).config).tray_hidden {
        (*bar).tray = create_tray(bar);
    }
    if (*(*bar).config).workspace_buttons {
        ipc_get_workspaces(bar);
    }
    determine_bar_visibility(bar, r#false != 0);
    return r#true != 0;
}
unsafe extern "C" fn display_in(
    mut fd: ::core::ffi::c_int,
    mut mask: ::core::ffi::c_short,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut bar: *mut swaybar = data as *mut swaybar;
    if mask as ::core::ffi::c_int & (POLLHUP | POLLERR) != 0 {
        if mask as ::core::ffi::c_int & POLLERR != 0 {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Wayland display poll error\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
                465 as ::core::ffi::c_int,
            );
        }
        (*bar).running = r#false != 0;
        return;
    }
    if wl_display_dispatch((*bar).display) == -1 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] wl_display_dispatch failed\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
            471 as ::core::ffi::c_int,
        );
        (*bar).running = r#false != 0;
    }
}
unsafe extern "C" fn ipc_in(
    mut fd: ::core::ffi::c_int,
    mut mask: ::core::ffi::c_short,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut bar: *mut swaybar = data as *mut swaybar;
    if mask as ::core::ffi::c_int & (POLLHUP | POLLERR) != 0 {
        if mask as ::core::ffi::c_int & POLLERR != 0 {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] IPC poll error\0".as_ptr() as *const ::core::ffi::c_char,
                b"swaybar/bar.c\0".as_ptr() as *const ::core::ffi::c_char,
                480 as ::core::ffi::c_int,
            );
        }
        (*bar).running = r#false != 0;
        return;
    }
    if handle_ipc_readable(bar) {
        set_bar_dirty(bar);
    }
}
#[no_mangle]
pub unsafe extern "C" fn status_in(
    mut fd: ::core::ffi::c_int,
    mut mask: ::core::ffi::c_short,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut bar: *mut swaybar = data as *mut swaybar;
    if mask as ::core::ffi::c_int & (POLLHUP | POLLERR) != 0 {
        status_error(
            (*bar).status,
            b"[error reading from status command]\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        set_bar_dirty(bar);
        loop_remove_fd((*bar).eventloop, fd);
    } else if status_handle_readable((*bar).status) {
        set_bar_dirty(bar);
    }
}
#[no_mangle]
pub unsafe extern "C" fn bar_run(mut bar: *mut swaybar) {
    loop_add_fd(
        (*bar).eventloop,
        wl_display_get_fd((*bar).display),
        POLLIN as ::core::ffi::c_short,
        Some(
            display_in
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        bar as *mut ::core::ffi::c_void,
    );
    loop_add_fd(
        (*bar).eventloop,
        (*bar).ipc_event_socketfd,
        POLLIN as ::core::ffi::c_short,
        Some(
            ipc_in
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        bar as *mut ::core::ffi::c_void,
    );
    if !(*bar).status.is_null() {
        loop_add_fd(
            (*bar).eventloop,
            (*(*bar).status).read_fd,
            POLLIN as ::core::ffi::c_short,
            Some(
                status_in
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_short,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            bar as *mut ::core::ffi::c_void,
        );
    }
    if !(*bar).tray.is_null() {
        loop_add_fd(
            (*bar).eventloop,
            (*(*bar).tray).fd,
            POLLIN as ::core::ffi::c_short,
            Some(
                tray_in
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_short,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            bar as *mut ::core::ffi::c_void,
        );
    }
    while (*bar).running {
        *__errno_location() = 0 as ::core::ffi::c_int;
        if wl_display_flush((*bar).display) == -1 as ::core::ffi::c_int
            && *__errno_location() != EAGAIN
        {
            break;
        }
        loop_poll((*bar).eventloop);
    }
}
unsafe extern "C" fn free_outputs(mut list: *mut wl_list) {
    let mut output: *mut swaybar_output = ::core::ptr::null_mut::<swaybar_output>();
    let mut tmp: *mut swaybar_output = ::core::ptr::null_mut::<swaybar_output>();
    output = ((*list).next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    tmp = ((*output).link.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    while &raw mut (*output).link != list {
        swaybar_output_free(output);
        output = tmp;
        tmp = ((*output).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
            as *mut swaybar_output;
    }
}
unsafe extern "C" fn free_seats(mut list: *mut wl_list) {
    let mut seat: *mut swaybar_seat = ::core::ptr::null_mut::<swaybar_seat>();
    let mut tmp: *mut swaybar_seat = ::core::ptr::null_mut::<swaybar_seat>();
    seat = ((*list).next as *mut ::core::ffi::c_char)
        .offset(-(864 as ::core::ffi::c_ulong as isize)) as *mut swaybar_seat
        as *mut swaybar_seat;
    tmp = ((*seat).link.next as *mut ::core::ffi::c_char)
        .offset(-(864 as ::core::ffi::c_ulong as isize)) as *mut swaybar_seat
        as *mut swaybar_seat;
    while &raw mut (*seat).link != list {
        swaybar_seat_free(seat);
        seat = tmp;
        tmp = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(864 as ::core::ffi::c_ulong as isize)) as *mut swaybar_seat
            as *mut swaybar_seat;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bar_teardown(mut bar: *mut swaybar) {
    destroy_tray((*bar).tray);
    free_outputs(&raw mut (*bar).outputs);
    free_outputs(&raw mut (*bar).unused_outputs);
    free_seats(&raw mut (*bar).seats);
    if !(*bar).config.is_null() {
        free_config((*bar).config);
    }
    close((*bar).ipc_event_socketfd);
    close((*bar).ipc_socketfd);
    if !(*bar).status.is_null() {
        status_line_free((*bar).status);
    }
    free((*bar).id as *mut ::core::ffi::c_void);
    free((*bar).mode as *mut ::core::ffi::c_void);
}
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
