use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_surface;
    pub type _cairo_font_options;
    pub type wl_proxy;
    pub type wl_display;
    pub type wl_buffer;
    pub type wl_callback;
    pub type wl_compositor;
    pub type wl_output;
    pub type wl_region;
    pub type wl_shm;
    pub type wl_surface;
    pub type _PangoContext;
    pub type _PangoFontDescription;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    fn cairo_destroy(cr: *mut cairo_t);
    fn cairo_save(cr: *mut cairo_t);
    fn cairo_restore(cr: *mut cairo_t);
    fn cairo_set_operator(cr: *mut cairo_t, op: cairo_operator_t);
    fn cairo_set_source_surface(
        cr: *mut cairo_t,
        surface: *mut cairo_surface_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    );
    fn cairo_set_antialias(cr: *mut cairo_t, antialias: cairo_antialias_t);
    fn cairo_set_line_width(cr: *mut cairo_t, width: ::core::ffi::c_double);
    fn cairo_scale(
        cr: *mut cairo_t,
        sx: ::core::ffi::c_double,
        sy: ::core::ffi::c_double,
    );
    fn cairo_move_to(
        cr: *mut cairo_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    );
    fn cairo_line_to(
        cr: *mut cairo_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    );
    fn cairo_rectangle(
        cr: *mut cairo_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        width: ::core::ffi::c_double,
        height: ::core::ffi::c_double,
    );
    fn cairo_paint(cr: *mut cairo_t);
    fn cairo_stroke(cr: *mut cairo_t);
    fn cairo_fill(cr: *mut cairo_t);
    fn cairo_font_options_create() -> *mut cairo_font_options_t;
    fn cairo_font_options_destroy(options: *mut cairo_font_options_t);
    fn cairo_font_options_set_antialias(
        options: *mut cairo_font_options_t,
        antialias: cairo_antialias_t,
    );
    fn cairo_font_options_set_subpixel_order(
        options: *mut cairo_font_options_t,
        subpixel_order: cairo_subpixel_order_t,
    );
    fn cairo_set_font_options(cr: *mut cairo_t, options: *const cairo_font_options_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_recording_surface_create(
        content: cairo_content_t,
        extents: *const cairo_rectangle_t,
    ) -> *mut cairo_surface_t;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
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
    fn cairo_set_source_u32(cairo: *mut cairo_t, color: uint32_t);
    fn to_cairo_subpixel_order(subpixel: wl_output_subpixel) -> cairo_subpixel_order_t;
    fn has_prefix(
        str: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    ) -> bool;
    fn get_text_size(
        cairo: *mut cairo_t,
        desc: *const PangoFontDescription,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
        baseline: *mut ::core::ffi::c_int,
        scale: ::core::ffi::c_double,
        markup: bool,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn render_text(
        cairo: *mut cairo_t,
        desc: *mut PangoFontDescription,
        scale: ::core::ffi::c_double,
        markup: bool,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn get_next_buffer(
        shm: *mut wl_shm,
        pool: *mut pool_buffer,
        width: uint32_t,
        height: uint32_t,
    ) -> *mut pool_buffer;
    static wl_callback_interface: wl_interface;
    static wl_region_interface: wl_interface;
    fn free_hotspots(list: *mut wl_list);
    fn i3bar_block_unref(block: *mut i3bar_block);
    fn i3bar_block_send_click(
        status: *mut status_line,
        block: *mut i3bar_block,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        rx: ::core::ffi::c_double,
        ry: ::core::ffi::c_double,
        w: ::core::ffi::c_double,
        h: ::core::ffi::c_double,
        scale: ::core::ffi::c_int,
        button: uint32_t,
        released: bool,
    ) -> hotspot_event_handling;
    fn ipc_send_workspace_command(bar: *mut swaybar, ws: *const ::core::ffi::c_char);
    fn render_tray(
        cairo: *mut cairo_t,
        output: *mut swaybar_output,
        x: *mut ::core::ffi::c_double,
    ) -> uint32_t;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct _cairo_content(pub ::core::ffi::c_uint);
impl _cairo_content {
    pub const CAIRO_CONTENT_COLOR: Self = Self(4096);
    pub const CAIRO_CONTENT_ALPHA: Self = Self(8192);
    pub const CAIRO_CONTENT_COLOR_ALPHA: Self = Self(12288);
}
pub type cairo_content_t = _cairo_content;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct _cairo_operator(pub ::core::ffi::c_uint);
impl _cairo_operator {
    pub const CAIRO_OPERATOR_CLEAR: Self = Self(0);
    pub const CAIRO_OPERATOR_SOURCE: Self = Self(1);
    pub const CAIRO_OPERATOR_OVER: Self = Self(2);
    pub const CAIRO_OPERATOR_IN: Self = Self(3);
    pub const CAIRO_OPERATOR_OUT: Self = Self(4);
    pub const CAIRO_OPERATOR_ATOP: Self = Self(5);
    pub const CAIRO_OPERATOR_DEST: Self = Self(6);
    pub const CAIRO_OPERATOR_DEST_OVER: Self = Self(7);
    pub const CAIRO_OPERATOR_DEST_IN: Self = Self(8);
    pub const CAIRO_OPERATOR_DEST_OUT: Self = Self(9);
    pub const CAIRO_OPERATOR_DEST_ATOP: Self = Self(10);
    pub const CAIRO_OPERATOR_XOR: Self = Self(11);
    pub const CAIRO_OPERATOR_ADD: Self = Self(12);
    pub const CAIRO_OPERATOR_SATURATE: Self = Self(13);
    pub const CAIRO_OPERATOR_MULTIPLY: Self = Self(14);
    pub const CAIRO_OPERATOR_SCREEN: Self = Self(15);
    pub const CAIRO_OPERATOR_OVERLAY: Self = Self(16);
    pub const CAIRO_OPERATOR_DARKEN: Self = Self(17);
    pub const CAIRO_OPERATOR_LIGHTEN: Self = Self(18);
    pub const CAIRO_OPERATOR_COLOR_DODGE: Self = Self(19);
    pub const CAIRO_OPERATOR_COLOR_BURN: Self = Self(20);
    pub const CAIRO_OPERATOR_HARD_LIGHT: Self = Self(21);
    pub const CAIRO_OPERATOR_SOFT_LIGHT: Self = Self(22);
    pub const CAIRO_OPERATOR_DIFFERENCE: Self = Self(23);
    pub const CAIRO_OPERATOR_EXCLUSION: Self = Self(24);
    pub const CAIRO_OPERATOR_HSL_HUE: Self = Self(25);
    pub const CAIRO_OPERATOR_HSL_SATURATION: Self = Self(26);
    pub const CAIRO_OPERATOR_HSL_COLOR: Self = Self(27);
    pub const CAIRO_OPERATOR_HSL_LUMINOSITY: Self = Self(28);
}
pub type cairo_operator_t = _cairo_operator;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct _cairo_antialias(pub ::core::ffi::c_uint);
impl _cairo_antialias {
    pub const CAIRO_ANTIALIAS_DEFAULT: Self = Self(0);
    pub const CAIRO_ANTIALIAS_NONE: Self = Self(1);
    pub const CAIRO_ANTIALIAS_GRAY: Self = Self(2);
    pub const CAIRO_ANTIALIAS_SUBPIXEL: Self = Self(3);
    pub const CAIRO_ANTIALIAS_FAST: Self = Self(4);
    pub const CAIRO_ANTIALIAS_GOOD: Self = Self(5);
    pub const CAIRO_ANTIALIAS_BEST: Self = Self(6);
}
pub type cairo_antialias_t = _cairo_antialias;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
}
pub type cairo_rectangle_t = _cairo_rectangle;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct _cairo_subpixel_order(pub ::core::ffi::c_uint);
impl _cairo_subpixel_order {
    pub const CAIRO_SUBPIXEL_ORDER_DEFAULT: Self = Self(0);
    pub const CAIRO_SUBPIXEL_ORDER_RGB: Self = Self(1);
    pub const CAIRO_SUBPIXEL_ORDER_BGR: Self = Self(2);
    pub const CAIRO_SUBPIXEL_ORDER_VRGB: Self = Self(3);
    pub const CAIRO_SUBPIXEL_ORDER_VBGR: Self = Self(4);
}
pub type cairo_subpixel_order_t = _cairo_subpixel_order;
pub type cairo_font_options_t = _cairo_font_options;
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
pub struct wl_callback_listener {
    pub done: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_callback, uint32_t) -> (),
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
pub type pid_t = __pid_t;
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
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
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
pub struct box_colors {
    pub border: uint32_t,
    pub background: uint32_t,
    pub text: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct box_size {
    pub width: uint32_t,
    pub height: uint32_t,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct hotspot_event_handling(pub ::core::ffi::c_uint);
impl hotspot_event_handling {
    pub const HOTSPOT_IGNORE: Self = Self(0);
    pub const HOTSPOT_PROCESS: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_hotspot {
    pub link: wl_list,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub callback: Option<
        unsafe extern "C" fn(
            *mut swaybar_output,
            *mut swaybar_hotspot,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            uint32_t,
            bool,
            *mut ::core::ffi::c_void,
        ) -> hotspot_event_handling,
    >,
    pub destroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub data: *mut ::core::ffi::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i3bar_block {
    pub link: wl_list,
    pub ref_count: ::core::ffi::c_int,
    pub full_text: *mut ::core::ffi::c_char,
    pub short_text: *mut ::core::ffi::c_char,
    pub align: *mut ::core::ffi::c_char,
    pub min_width_str: *mut ::core::ffi::c_char,
    pub urgent: bool,
    pub color: uint32_t,
    pub color_set: bool,
    pub min_width: ::core::ffi::c_int,
    pub name: *mut ::core::ffi::c_char,
    pub instance: *mut ::core::ffi::c_char,
    pub separator: bool,
    pub separator_block_width: ::core::ffi::c_int,
    pub markup: bool,
    pub background: uint32_t,
    pub border: uint32_t,
    pub border_set: bool,
    pub border_top: ::core::ffi::c_int,
    pub border_bottom: ::core::ffi::c_int,
    pub border_left: ::core::ffi::c_int,
    pub border_right: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct render_context {
    pub cairo: *mut cairo_t,
    pub output: *mut swaybar_output,
    pub textaa_sharp: *mut cairo_font_options_t,
    pub textaa_safe: *mut cairo_font_options_t,
    pub background_color: uint32_t,
    pub has_transparency: bool,
}
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 43] = unsafe {
    ::core::mem::transmute::<
        [u8; 43],
        [::core::ffi::c_char; 43],
    >(*b"void render_frame(struct swaybar_output *)\0")
};
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const WL_MARSHAL_FLAG_DESTROY: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
    << 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_callback_add_listener(
    mut wl_callback: *mut wl_callback,
    mut listener: *const wl_callback_listener,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return wl_proxy_add_listener(
        wl_callback as *mut wl_proxy,
        listener as *mut Option<unsafe extern "C" fn() -> ()>,
        data,
    );
}
#[inline]
unsafe extern "C" fn wl_callback_destroy(mut wl_callback: *mut wl_callback) {
    wl_proxy_destroy(wl_callback as *mut wl_proxy);
}
pub const WL_COMPOSITOR_CREATE_REGION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
pub const WL_SURFACE_ATTACH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WL_SURFACE_DAMAGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WL_SURFACE_FRAME: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const WL_SURFACE_SET_OPAQUE_REGION: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const WL_SURFACE_COMMIT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const WL_SURFACE_SET_BUFFER_SCALE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
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
unsafe extern "C" fn wl_surface_damage(
    mut wl_surface: *mut wl_surface,
    mut x: int32_t,
    mut y: int32_t,
    mut width: int32_t,
    mut height: int32_t,
) {
    wl_proxy_marshal_flags(
        wl_surface as *mut wl_proxy,
        WL_SURFACE_DAMAGE as uint32_t,
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
unsafe extern "C" fn wl_surface_frame(
    mut wl_surface: *mut wl_surface,
) -> *mut wl_callback {
    let mut callback: *mut wl_proxy = ::core::ptr::null_mut::<wl_proxy>();
    callback = wl_proxy_marshal_flags(
        wl_surface as *mut wl_proxy,
        WL_SURFACE_FRAME as uint32_t,
        &raw const wl_callback_interface,
        wl_proxy_get_version(wl_surface as *mut wl_proxy),
        0 as uint32_t,
        NULL,
    );
    return callback as *mut wl_callback;
}
#[inline]
unsafe extern "C" fn wl_surface_set_opaque_region(
    mut wl_surface: *mut wl_surface,
    mut region: *mut wl_region,
) {
    wl_proxy_marshal_flags(
        wl_surface as *mut wl_proxy,
        WL_SURFACE_SET_OPAQUE_REGION as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_surface as *mut wl_proxy),
        0 as uint32_t,
        region,
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
pub const WL_REGION_DESTROY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WL_REGION_ADD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn wl_region_add(
    mut wl_region: *mut wl_region,
    mut x: int32_t,
    mut y: int32_t,
    mut width: int32_t,
    mut height: int32_t,
) {
    wl_proxy_marshal_flags(
        wl_region as *mut wl_proxy,
        WL_REGION_ADD as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_region as *mut wl_proxy),
        0 as uint32_t,
        x,
        y,
        width,
        height,
    );
}
pub const ZWLR_LAYER_SURFACE_V1_SET_SIZE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ZWLR_LAYER_SURFACE_V1_SET_EXCLUSIVE_ZONE: ::core::ffi::c_int = 2
    as ::core::ffi::c_int;
pub const ZWLR_LAYER_SURFACE_V1_SET_MARGIN: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn zwlr_layer_surface_v1_set_size(
    mut zwlr_layer_surface_v1: *mut zwlr_layer_surface_v1,
    mut width: uint32_t,
    mut height: uint32_t,
) {
    wl_proxy_marshal_flags(
        zwlr_layer_surface_v1 as *mut wl_proxy,
        ZWLR_LAYER_SURFACE_V1_SET_SIZE as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(zwlr_layer_surface_v1 as *mut wl_proxy),
        0 as uint32_t,
        width,
        height,
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
unsafe extern "C" fn zwlr_layer_surface_v1_set_margin(
    mut zwlr_layer_surface_v1: *mut zwlr_layer_surface_v1,
    mut top: int32_t,
    mut right: int32_t,
    mut bottom: int32_t,
    mut left: int32_t,
) {
    wl_proxy_marshal_flags(
        zwlr_layer_surface_v1 as *mut wl_proxy,
        ZWLR_LAYER_SURFACE_V1_SET_MARGIN as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(zwlr_layer_surface_v1 as *mut wl_proxy),
        0 as uint32_t,
        top,
        right,
        bottom,
        left,
    );
}
static mut WS_HORIZONTAL_PADDING: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
static mut WS_VERTICAL_PADDING: ::core::ffi::c_double = 1.5f64;
static mut BORDER_WIDTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn choose_text_aa_mode(
    mut ctx: *mut render_context,
    mut fontcolor: uint32_t,
) {
    let mut salpha: uint32_t = fontcolor & 0xff as uint32_t;
    let mut balpha: uint32_t = (*ctx).background_color & 0xff as uint32_t;
    let mut fo: *mut cairo_font_options_t = if salpha == balpha {
        (*ctx).textaa_sharp
    } else {
        (*ctx).textaa_safe
    };
    cairo_set_font_options((*ctx).cairo, fo);
    let mut op: cairo_operator_t = _cairo_operator(
        (if salpha == 0xff as uint32_t {
            _cairo_operator::CAIRO_OPERATOR_OVER.0 as ::core::ffi::c_int
        } else {
            _cairo_operator::CAIRO_OPERATOR_SOURCE.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    cairo_set_operator((*ctx).cairo, op);
}
unsafe extern "C" fn render_status_line_error(
    mut ctx: *mut render_context,
    mut x: *mut ::core::ffi::c_double,
) -> uint32_t {
    let mut output: *mut swaybar_output = (*ctx).output;
    let mut error: *const ::core::ffi::c_char = (*(*(*output).bar).status).text;
    if error.is_null() {
        return 0 as uint32_t;
    }
    let mut height: uint32_t = (*output).height;
    let mut cairo: *mut cairo_t = (*ctx).cairo;
    cairo_set_source_u32(cairo, 0xff0000ff as uint32_t);
    let mut margin: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    let mut ws_vertical_padding: ::core::ffi::c_double = (*(*(*output).bar).config)
        .status_padding as ::core::ffi::c_double;
    let mut font: *mut PangoFontDescription = (*(*(*output).bar).config)
        .font_description;
    let mut text_width: ::core::ffi::c_int = 0;
    let mut text_height: ::core::ffi::c_int = 0;
    get_text_size(
        cairo,
        font,
        &raw mut text_width,
        &raw mut text_height,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        r#false != 0,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        error,
    );
    let mut ideal_height: uint32_t = (text_height as ::core::ffi::c_double
        + ws_vertical_padding * 2 as ::core::ffi::c_int as ::core::ffi::c_double)
        as uint32_t;
    let mut ideal_surface_height: uint32_t = ideal_height;
    if (*(*(*output).bar).config).height == 0 && (*output).height < ideal_surface_height
    {
        return ideal_surface_height;
    }
    *x -= (text_width + margin) as ::core::ffi::c_double;
    let mut text_y: ::core::ffi::c_double = height as ::core::ffi::c_double / 2.0f64
        - text_height as ::core::ffi::c_double / 2.0f64;
    cairo_move_to(
        cairo,
        *x,
        floor(text_y) as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    choose_text_aa_mode(ctx, 0xff0000ff as uint32_t);
    render_text(
        cairo,
        font,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        r#false != 0,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        error,
    );
    *x -= margin as ::core::ffi::c_double;
    return (*output).height;
}
unsafe extern "C" fn render_status_line_text(
    mut ctx: *mut render_context,
    mut x: *mut ::core::ffi::c_double,
) -> uint32_t {
    let mut output: *mut swaybar_output = (*ctx).output;
    let mut text: *const ::core::ffi::c_char = (*(*(*output).bar).status).text;
    if text.is_null() {
        return 0 as uint32_t;
    }
    let mut cairo: *mut cairo_t = (*ctx).cairo;
    let mut config: *mut swaybar_config = (*(*output).bar).config;
    let mut fontcolor: uint32_t = if (*output).focused as ::core::ffi::c_int != 0 {
        (*config).colors.focused_statusline
    } else {
        (*config).colors.statusline
    };
    cairo_set_source_u32(cairo, fontcolor);
    let mut text_width: ::core::ffi::c_int = 0;
    let mut text_height: ::core::ffi::c_int = 0;
    get_text_size(
        cairo,
        (*config).font_description,
        &raw mut text_width,
        &raw mut text_height,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*config).pango_markup,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        text,
    );
    let mut ws_vertical_padding: ::core::ffi::c_double = (*config).status_padding
        as ::core::ffi::c_double;
    let mut margin: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    let mut ideal_height: uint32_t = (text_height as ::core::ffi::c_double
        + ws_vertical_padding * 2 as ::core::ffi::c_int as ::core::ffi::c_double)
        as uint32_t;
    let mut ideal_surface_height: uint32_t = ideal_height;
    if (*(*(*output).bar).config).height == 0 && (*output).height < ideal_surface_height
    {
        return ideal_surface_height;
    }
    *x -= (text_width + margin) as ::core::ffi::c_double;
    let mut height: uint32_t = (*output).height;
    let mut text_y: ::core::ffi::c_double = height as ::core::ffi::c_double / 2.0f64
        - text_height as ::core::ffi::c_double / 2.0f64;
    cairo_move_to(
        cairo,
        *x,
        floor(text_y) as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    choose_text_aa_mode(ctx, fontcolor);
    render_text(
        cairo,
        (*config).font_description,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*config).pango_markup,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        text,
    );
    *x -= margin as ::core::ffi::c_double;
    return (*output).height;
}
unsafe extern "C" fn render_sharp_rectangle(
    mut cairo: *mut cairo_t,
    mut color: uint32_t,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut width: ::core::ffi::c_double,
    mut height: ::core::ffi::c_double,
) {
    cairo_save(cairo);
    cairo_set_operator(cairo, _cairo_operator::CAIRO_OPERATOR_SOURCE);
    cairo_set_source_u32(cairo, color);
    cairo_set_antialias(cairo, _cairo_antialias::CAIRO_ANTIALIAS_NONE);
    cairo_rectangle(cairo, x, y, width, height);
    cairo_fill(cairo);
    cairo_restore(cairo);
}
unsafe extern "C" fn render_sharp_line(
    mut cairo: *mut cairo_t,
    mut color: uint32_t,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut width: ::core::ffi::c_double,
    mut height: ::core::ffi::c_double,
) {
    if width > 1 as ::core::ffi::c_int as ::core::ffi::c_double
        && height > 1 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        render_sharp_rectangle(cairo, color, x, y, width, height);
    } else {
        cairo_save(cairo);
        cairo_set_operator(cairo, _cairo_operator::CAIRO_OPERATOR_SOURCE);
        cairo_set_source_u32(cairo, color);
        cairo_set_antialias(cairo, _cairo_antialias::CAIRO_ANTIALIAS_NONE);
        if width == 1 as ::core::ffi::c_int as ::core::ffi::c_double {
            x += 0.5f64;
            height += y;
            width = x;
        }
        if height == 1 as ::core::ffi::c_int as ::core::ffi::c_double {
            y += 0.5f64;
            width += x;
            height = y;
        }
        cairo_move_to(cairo, x, y);
        cairo_set_line_width(cairo, 1.0f64);
        cairo_line_to(cairo, width, height);
        cairo_stroke(cairo);
        cairo_restore(cairo);
    };
}
unsafe extern "C" fn block_hotspot_callback(
    mut output: *mut swaybar_output,
    mut hotspot: *mut swaybar_hotspot,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut button: uint32_t,
    mut released: bool,
    mut data: *mut ::core::ffi::c_void,
) -> hotspot_event_handling {
    let mut block: *mut i3bar_block = data as *mut i3bar_block;
    let mut status: *mut status_line = (*(*output).bar).status;
    return i3bar_block_send_click(
        status,
        block,
        x,
        y,
        x - (*hotspot).x as ::core::ffi::c_double,
        y - (*hotspot).y as ::core::ffi::c_double,
        (*hotspot).width as ::core::ffi::c_double,
        (*hotspot).height as ::core::ffi::c_double,
        (*output).scale as ::core::ffi::c_int,
        button,
        released,
    );
}
unsafe extern "C" fn i3bar_block_unref_callback(mut data: *mut ::core::ffi::c_void) {
    i3bar_block_unref(data as *mut i3bar_block);
}
unsafe extern "C" fn render_status_block(
    mut ctx: *mut render_context,
    mut block: *mut i3bar_block,
    mut x: *mut ::core::ffi::c_double,
    mut edge: bool,
    mut use_short_text: bool,
) -> uint32_t {
    if (*block).full_text.is_null() || *(*block).full_text == 0 {
        return 0 as uint32_t;
    }
    let mut text: *mut ::core::ffi::c_char = (*block).full_text;
    if use_short_text as ::core::ffi::c_int != 0 && !(*block).short_text.is_null()
        && *(*block).short_text as ::core::ffi::c_int != 0
    {
        text = (*block).short_text;
    }
    let mut cairo: *mut cairo_t = (*ctx).cairo;
    let mut output: *mut swaybar_output = (*ctx).output;
    let mut config: *mut swaybar_config = (*(*output).bar).config;
    let mut text_width: ::core::ffi::c_int = 0;
    let mut text_height: ::core::ffi::c_int = 0;
    get_text_size(
        cairo,
        (*config).font_description,
        &raw mut text_width,
        &raw mut text_height,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*block).markup,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        text,
    );
    let mut margin: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    let mut ws_vertical_padding: ::core::ffi::c_double = (*config).status_padding
        as ::core::ffi::c_double;
    let mut width: ::core::ffi::c_int = text_width;
    if !(*block).min_width_str.is_null() {
        let mut w: ::core::ffi::c_int = 0;
        get_text_size(
            cairo,
            (*config).font_description,
            &raw mut w,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            1 as ::core::ffi::c_int as ::core::ffi::c_double,
            (*block).markup,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            (*block).min_width_str,
        );
        (*block).min_width = w;
    }
    if width < (*block).min_width {
        width = (*block).min_width;
    }
    let mut block_width: ::core::ffi::c_double = width as ::core::ffi::c_double;
    let mut ideal_height: uint32_t = (text_height as ::core::ffi::c_double
        + ws_vertical_padding * 2 as ::core::ffi::c_int as ::core::ffi::c_double)
        as uint32_t;
    let mut ideal_surface_height: uint32_t = ideal_height;
    if (*(*(*output).bar).config).height == 0 && (*output).height < ideal_surface_height
    {
        return ideal_surface_height;
    }
    *x -= width as ::core::ffi::c_double;
    if ((*block).border_set as ::core::ffi::c_int != 0
        || (*block).urgent as ::core::ffi::c_int != 0)
        && (*block).border_left > 0 as ::core::ffi::c_int
    {
        *x -= ((*block).border_left + margin) as ::core::ffi::c_double;
        block_width += ((*block).border_left + margin) as ::core::ffi::c_double;
    }
    if ((*block).border_set as ::core::ffi::c_int != 0
        || (*block).urgent as ::core::ffi::c_int != 0)
        && (*block).border_right > 0 as ::core::ffi::c_int
    {
        *x -= ((*block).border_right + margin) as ::core::ffi::c_double;
        block_width += ((*block).border_right + margin) as ::core::ffi::c_double;
    }
    let mut sep_width: ::core::ffi::c_int = 0;
    let mut sep_height: ::core::ffi::c_int = 0;
    let mut sep_block_width: ::core::ffi::c_int = (*block).separator_block_width;
    if !edge {
        if !(*config).sep_symbol.is_null() {
            get_text_size(
                cairo,
                (*config).font_description,
                &raw mut sep_width,
                &raw mut sep_height,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                1 as ::core::ffi::c_int as ::core::ffi::c_double,
                r#false != 0,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                (*config).sep_symbol,
            );
            let mut _ideal_height: uint32_t = (sep_height as ::core::ffi::c_double
                + ws_vertical_padding * 2 as ::core::ffi::c_int as ::core::ffi::c_double)
                as uint32_t;
            let mut _ideal_surface_height: uint32_t = _ideal_height;
            if (*(*(*output).bar).config).height == 0
                && (*output).height < _ideal_surface_height
            {
                return _ideal_surface_height;
            }
            if (*block).separator as ::core::ffi::c_int != 0
                && sep_width > sep_block_width
            {
                sep_block_width = sep_width + margin * 2 as ::core::ffi::c_int;
            }
        }
        *x -= sep_block_width as ::core::ffi::c_double;
    } else if (*config).status_edge_padding != 0 {
        *x -= (*config).status_edge_padding as ::core::ffi::c_double;
    }
    let mut height: uint32_t = (*output).height;
    if (*(*(*output).bar).status).click_events {
        let mut hotspot: *mut swaybar_hotspot = calloc(
            1 as size_t,
            ::core::mem::size_of::<swaybar_hotspot>(),
        ) as *mut swaybar_hotspot;
        (*hotspot).x = *x as ::core::ffi::c_int;
        (*hotspot).y = 0 as ::core::ffi::c_int;
        (*hotspot).width = width;
        (*hotspot).height = height as ::core::ffi::c_int;
        (*hotspot).callback = Some(
            block_hotspot_callback
                as unsafe extern "C" fn(
                    *mut swaybar_output,
                    *mut swaybar_hotspot,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    uint32_t,
                    bool,
                    *mut ::core::ffi::c_void,
                ) -> hotspot_event_handling,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut swaybar_output,
                    *mut swaybar_hotspot,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    uint32_t,
                    bool,
                    *mut ::core::ffi::c_void,
                ) -> hotspot_event_handling,
            >;
        (*hotspot).destroy = Some(
            i3bar_block_unref_callback
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
        (*hotspot).data = block as *mut ::core::ffi::c_void;
        (*block).ref_count += 1;
        wl_list_insert(&raw mut (*output).hotspots, &raw mut (*hotspot).link);
    }
    let mut x_pos: ::core::ffi::c_double = *x;
    let mut y_pos: ::core::ffi::c_double = ws_vertical_padding;
    let mut render_height: ::core::ffi::c_double = height as ::core::ffi::c_double
        - ws_vertical_padding * 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut bg_color: uint32_t = if (*block).urgent as ::core::ffi::c_int != 0 {
        (*config).colors.urgent_workspace.background
    } else {
        (*block).background
    };
    (*ctx).has_transparency = (*ctx).has_transparency as ::core::ffi::c_int
        | (bg_color & 0xff as uint32_t != 0xff as uint32_t) as ::core::ffi::c_int != 0;
    if bg_color != 0 {
        render_sharp_rectangle(
            cairo,
            bg_color,
            x_pos,
            y_pos,
            block_width,
            render_height,
        );
        (*ctx).background_color = bg_color;
    }
    let mut border_color: uint32_t = if (*block).urgent as ::core::ffi::c_int != 0 {
        (*config).colors.urgent_workspace.border
    } else {
        (*block).border
    };
    if (*block).border_set as ::core::ffi::c_int != 0
        || (*block).urgent as ::core::ffi::c_int != 0
    {
        if (*block).border_top > 0 as ::core::ffi::c_int {
            render_sharp_line(
                cairo,
                border_color,
                x_pos,
                y_pos,
                block_width,
                (*block).border_top as ::core::ffi::c_double,
            );
        }
        if (*block).border_bottom > 0 as ::core::ffi::c_int {
            render_sharp_line(
                cairo,
                border_color,
                x_pos,
                y_pos + render_height - (*block).border_bottom as ::core::ffi::c_double,
                block_width,
                (*block).border_bottom as ::core::ffi::c_double,
            );
        }
        if (*block).border_left > 0 as ::core::ffi::c_int {
            render_sharp_line(
                cairo,
                border_color,
                x_pos,
                y_pos,
                (*block).border_left as ::core::ffi::c_double,
                render_height,
            );
        }
        x_pos += ((*block).border_left + margin) as ::core::ffi::c_double;
    }
    let mut offset: ::core::ffi::c_double = 0 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    if has_prefix((*block).align, b"left\0".as_ptr() as *const ::core::ffi::c_char) {
        offset = x_pos;
    } else if has_prefix(
        (*block).align,
        b"right\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        offset = x_pos + width as ::core::ffi::c_double
            - text_width as ::core::ffi::c_double;
    } else if has_prefix(
        (*block).align,
        b"center\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        offset = x_pos
            + ((width - text_width) / 2 as ::core::ffi::c_int) as ::core::ffi::c_double;
    }
    let mut text_y: ::core::ffi::c_double = height as ::core::ffi::c_double / 2.0f64
        - text_height as ::core::ffi::c_double / 2.0f64;
    cairo_move_to(
        cairo,
        offset,
        floor(text_y) as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    let mut color: uint32_t = if (*output).focused as ::core::ffi::c_int != 0 {
        (*config).colors.focused_statusline
    } else {
        (*config).colors.statusline
    };
    color = if (*block).color_set as ::core::ffi::c_int != 0 {
        (*block).color
    } else {
        color
    };
    color = if (*block).urgent as ::core::ffi::c_int != 0 {
        (*config).colors.urgent_workspace.text
    } else {
        color
    };
    cairo_set_source_u32(cairo, color);
    choose_text_aa_mode(ctx, color);
    render_text(
        cairo,
        (*config).font_description,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*block).markup,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        text,
    );
    x_pos += width as ::core::ffi::c_double;
    if (*block).border_set as ::core::ffi::c_int != 0
        || (*block).urgent as ::core::ffi::c_int != 0
    {
        x_pos += margin as ::core::ffi::c_double;
        if (*block).border_right > 0 as ::core::ffi::c_int {
            render_sharp_line(
                cairo,
                border_color,
                x_pos,
                y_pos,
                (*block).border_right as ::core::ffi::c_double,
                render_height,
            );
        }
        x_pos += (*block).border_right as ::core::ffi::c_double;
    }
    if !edge && (*block).separator as ::core::ffi::c_int != 0 {
        if (*output).focused {
            color = (*config).colors.focused_separator;
        } else {
            color = (*config).colors.separator;
        }
        cairo_set_source_u32(cairo, color);
        if !(*config).sep_symbol.is_null() {
            offset = x_pos
                + ((sep_block_width - sep_width) / 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_double;
            let mut sep_y: ::core::ffi::c_double = height as ::core::ffi::c_double
                / 2.0f64 - sep_height as ::core::ffi::c_double / 2.0f64;
            cairo_move_to(
                cairo,
                offset,
                floor(sep_y) as ::core::ffi::c_int as ::core::ffi::c_double,
            );
            choose_text_aa_mode(ctx, color);
            render_text(
                cairo,
                (*config).font_description,
                1 as ::core::ffi::c_int as ::core::ffi::c_double,
                r#false != 0,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                (*config).sep_symbol,
            );
        } else {
            cairo_set_operator(cairo, _cairo_operator::CAIRO_OPERATOR_SOURCE);
            cairo_set_line_width(
                cairo,
                1 as ::core::ffi::c_int as ::core::ffi::c_double,
            );
            cairo_move_to(
                cairo,
                x_pos
                    + (sep_block_width / 2 as ::core::ffi::c_int)
                        as ::core::ffi::c_double,
                margin as ::core::ffi::c_double,
            );
            cairo_line_to(
                cairo,
                x_pos
                    + (sep_block_width / 2 as ::core::ffi::c_int)
                        as ::core::ffi::c_double,
                height.wrapping_sub(margin as uint32_t) as ::core::ffi::c_double,
            );
            cairo_stroke(cairo);
        }
    }
    return (*output).height;
}
unsafe extern "C" fn predict_status_block_pos(
    mut cairo: *mut cairo_t,
    mut output: *mut swaybar_output,
    mut block: *mut i3bar_block,
    mut x: *mut ::core::ffi::c_double,
    mut edge: bool,
) {
    if (*block).full_text.is_null() || *(*block).full_text == 0 {
        return;
    }
    let mut config: *mut swaybar_config = (*(*output).bar).config;
    let mut text_width: ::core::ffi::c_int = 0;
    let mut text_height: ::core::ffi::c_int = 0;
    get_text_size(
        cairo,
        (*config).font_description,
        &raw mut text_width,
        &raw mut text_height,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*block).markup,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*block).full_text,
    );
    let mut margin: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    let mut ws_vertical_padding: ::core::ffi::c_double = (*config).status_padding
        as ::core::ffi::c_double;
    let mut width: ::core::ffi::c_int = text_width;
    if !(*block).min_width_str.is_null() {
        let mut w: ::core::ffi::c_int = 0;
        get_text_size(
            cairo,
            (*config).font_description,
            &raw mut w,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            1 as ::core::ffi::c_int as ::core::ffi::c_double,
            (*block).markup,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            (*block).min_width_str,
        );
        (*block).min_width = w;
    }
    if width < (*block).min_width {
        width = (*block).min_width;
    }
    let mut ideal_height: uint32_t = (text_height as ::core::ffi::c_double
        + ws_vertical_padding * 2 as ::core::ffi::c_int as ::core::ffi::c_double)
        as uint32_t;
    let mut ideal_surface_height: uint32_t = ideal_height;
    if (*(*(*output).bar).config).height == 0 && (*output).height < ideal_surface_height
    {
        return;
    }
    *x -= width as ::core::ffi::c_double;
    if ((*block).border_set as ::core::ffi::c_int != 0
        || (*block).urgent as ::core::ffi::c_int != 0)
        && (*block).border_left > 0 as ::core::ffi::c_int
    {
        *x -= ((*block).border_left + margin) as ::core::ffi::c_double;
    }
    if ((*block).border_set as ::core::ffi::c_int != 0
        || (*block).urgent as ::core::ffi::c_int != 0)
        && (*block).border_right > 0 as ::core::ffi::c_int
    {
        *x -= ((*block).border_right + margin) as ::core::ffi::c_double;
    }
    let mut sep_width: ::core::ffi::c_int = 0;
    let mut sep_height: ::core::ffi::c_int = 0;
    let mut sep_block_width: ::core::ffi::c_int = (*block).separator_block_width;
    if !edge {
        if !(*config).sep_symbol.is_null() {
            get_text_size(
                cairo,
                (*config).font_description,
                &raw mut sep_width,
                &raw mut sep_height,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                1 as ::core::ffi::c_int as ::core::ffi::c_double,
                r#false != 0,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                (*config).sep_symbol,
            );
            let mut _ideal_height: uint32_t = (sep_height as ::core::ffi::c_double
                + ws_vertical_padding * 2 as ::core::ffi::c_int as ::core::ffi::c_double)
                as uint32_t;
            let mut _ideal_surface_height: uint32_t = _ideal_height;
            if (*(*(*output).bar).config).height == 0
                && (*output).height < _ideal_surface_height
            {
                return;
            }
            if sep_width > sep_block_width {
                sep_block_width = sep_width + margin * 2 as ::core::ffi::c_int;
            }
        }
        *x -= sep_block_width as ::core::ffi::c_double;
    } else if (*config).status_edge_padding != 0 {
        *x -= (*config).status_edge_padding as ::core::ffi::c_double;
    }
}
unsafe extern "C" fn predict_status_line_pos(
    mut cairo: *mut cairo_t,
    mut output: *mut swaybar_output,
    mut x: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut edge: bool = x == (*output).width as ::core::ffi::c_double;
    let mut block: *mut i3bar_block = ::core::ptr::null_mut::<i3bar_block>();
    block = ((*(*(*output).bar).status).blocks.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut i3bar_block
        as *mut i3bar_block;
    while &raw mut (*block).link != &raw mut (*(*(*output).bar).status).blocks {
        predict_status_block_pos(cairo, output, block, &raw mut x, edge);
        edge = r#false != 0;
        block = ((*block).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut i3bar_block
            as *mut i3bar_block;
    }
    return x;
}
unsafe extern "C" fn predict_workspace_button_length(
    mut cairo: *mut cairo_t,
    mut output: *mut swaybar_output,
    mut ws: *mut swaybar_workspace,
) -> uint32_t {
    let mut config: *mut swaybar_config = (*(*output).bar).config;
    let mut text_width: ::core::ffi::c_int = 0;
    let mut text_height: ::core::ffi::c_int = 0;
    get_text_size(
        cairo,
        (*config).font_description,
        &raw mut text_width,
        &raw mut text_height,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*config).pango_markup,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*ws).label,
    );
    let mut ws_vertical_padding: ::core::ffi::c_int = WS_VERTICAL_PADDING
        as ::core::ffi::c_int;
    let mut ws_horizontal_padding: ::core::ffi::c_int = WS_HORIZONTAL_PADDING;
    let mut border_width: ::core::ffi::c_int = BORDER_WIDTH;
    let mut ideal_height: uint32_t = (ws_vertical_padding * 2 as ::core::ffi::c_int
        + text_height + border_width * 2 as ::core::ffi::c_int) as uint32_t;
    let mut ideal_surface_height: uint32_t = ideal_height;
    if (*(*(*output).bar).config).height == 0 && (*output).height < ideal_surface_height
    {
        return 0 as uint32_t;
    }
    let mut width: uint32_t = (text_width
        + ws_horizontal_padding * 2 as ::core::ffi::c_int
        + border_width * 2 as ::core::ffi::c_int) as uint32_t;
    if width < (*config).workspace_min_width {
        width = (*config).workspace_min_width;
    }
    return width;
}
unsafe extern "C" fn predict_workspace_buttons_length(
    mut cairo: *mut cairo_t,
    mut output: *mut swaybar_output,
) -> uint32_t {
    let mut width: uint32_t = 0 as uint32_t;
    if (*(*(*output).bar).config).workspace_buttons {
        let mut ws: *mut swaybar_workspace = ::core::ptr::null_mut::<
            swaybar_workspace,
        >();
        ws = ((*output).workspaces.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_workspace
            as *mut swaybar_workspace;
        while &raw mut (*ws).link != &raw mut (*output).workspaces {
            width = width
                .wrapping_add(predict_workspace_button_length(cairo, output, ws));
            ws = ((*ws).link.next as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_workspace
                as *mut swaybar_workspace;
        }
    }
    return width;
}
unsafe extern "C" fn predict_binding_mode_indicator_length(
    mut cairo: *mut cairo_t,
    mut output: *mut swaybar_output,
) -> uint32_t {
    let mut mode: *const ::core::ffi::c_char = (*(*output).bar).mode;
    if mode.is_null() {
        return 0 as uint32_t;
    }
    let mut config: *mut swaybar_config = (*(*output).bar).config;
    if !(*config).binding_mode_indicator {
        return 0 as uint32_t;
    }
    let mut text_width: ::core::ffi::c_int = 0;
    let mut text_height: ::core::ffi::c_int = 0;
    get_text_size(
        cairo,
        (*config).font_description,
        &raw mut text_width,
        &raw mut text_height,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*(*output).bar).mode_pango_markup,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        mode,
    );
    let mut ws_vertical_padding: ::core::ffi::c_int = WS_VERTICAL_PADDING
        as ::core::ffi::c_int;
    let mut ws_horizontal_padding: ::core::ffi::c_int = WS_HORIZONTAL_PADDING;
    let mut border_width: ::core::ffi::c_int = BORDER_WIDTH;
    let mut ideal_height: uint32_t = (text_height
        + ws_vertical_padding * 2 as ::core::ffi::c_int
        + border_width * 2 as ::core::ffi::c_int) as uint32_t;
    let mut ideal_surface_height: uint32_t = ideal_height;
    if (*(*(*output).bar).config).height == 0 && (*output).height < ideal_surface_height
    {
        return 0 as uint32_t;
    }
    let mut width: uint32_t = (text_width
        + ws_horizontal_padding * 2 as ::core::ffi::c_int
        + border_width * 2 as ::core::ffi::c_int) as uint32_t;
    if width < (*config).workspace_min_width {
        width = (*config).workspace_min_width;
    }
    return width;
}
unsafe extern "C" fn render_status_line_i3bar(
    mut ctx: *mut render_context,
    mut x: *mut ::core::ffi::c_double,
) -> uint32_t {
    let mut output: *mut swaybar_output = (*ctx).output;
    let mut max_height: uint32_t = 0 as uint32_t;
    let mut edge: bool = *x == (*output).width as ::core::ffi::c_double;
    let mut block: *mut i3bar_block = ::core::ptr::null_mut::<i3bar_block>();
    let mut use_short_text: bool = r#false != 0;
    let mut cairo: *mut cairo_t = (*ctx).cairo;
    let mut reserved_width: ::core::ffi::c_double = predict_workspace_buttons_length(
            cairo,
            output,
        )
        .wrapping_add(predict_binding_mode_indicator_length(cairo, output))
        .wrapping_add(3 as uint32_t) as ::core::ffi::c_double;
    let mut predicted_full_pos: ::core::ffi::c_double = predict_status_line_pos(
        cairo,
        output,
        *x,
    );
    if predicted_full_pos < reserved_width {
        use_short_text = r#true != 0;
    }
    block = ((*(*(*output).bar).status).blocks.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut i3bar_block
        as *mut i3bar_block;
    while &raw mut (*block).link != &raw mut (*(*(*output).bar).status).blocks {
        let mut h: uint32_t = render_status_block(ctx, block, x, edge, use_short_text);
        max_height = if h > max_height { h } else { max_height };
        edge = r#false != 0;
        block = ((*block).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut i3bar_block
            as *mut i3bar_block;
    }
    return max_height;
}
unsafe extern "C" fn render_status_line(
    mut ctx: *mut render_context,
    mut x: *mut ::core::ffi::c_double,
) -> uint32_t {
    let mut status: *mut status_line = (*(*(*ctx).output).bar).status;
    match (*status).protocol {
        status_protocol::PROTOCOL_ERROR => return render_status_line_error(ctx, x),
        status_protocol::PROTOCOL_TEXT => return render_status_line_text(ctx, x),
        status_protocol::PROTOCOL_I3BAR => return render_status_line_i3bar(ctx, x),
        status_protocol::PROTOCOL_UNDEF => return 0 as uint32_t,
        _ => {}
    }
    return 0 as uint32_t;
}
unsafe extern "C" fn render_box(
    mut ctx: *mut render_context,
    mut x: ::core::ffi::c_double,
    mut colors: box_colors,
    mut label: *const ::core::ffi::c_char,
    mut pango_markup: bool,
) -> box_size {
    let mut output: *mut swaybar_output = (*ctx).output;
    let mut config: *mut swaybar_config = (*(*output).bar).config;
    let mut cairo: *mut cairo_t = (*ctx).cairo;
    let mut text_width: ::core::ffi::c_int = 0;
    let mut text_height: ::core::ffi::c_int = 0;
    get_text_size(
        cairo,
        (*config).font_description,
        &raw mut text_width,
        &raw mut text_height,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        pango_markup,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        label,
    );
    let mut width: uint32_t = (text_width
        + WS_HORIZONTAL_PADDING * 2 as ::core::ffi::c_int
        + BORDER_WIDTH * 2 as ::core::ffi::c_int) as uint32_t;
    if width < (*config).workspace_min_width {
        width = (*config).workspace_min_width;
    }
    let mut ideal_height: uint32_t = (text_height as ::core::ffi::c_double
        + WS_VERTICAL_PADDING * 2 as ::core::ffi::c_int as ::core::ffi::c_double
        + (BORDER_WIDTH * 2 as ::core::ffi::c_int) as ::core::ffi::c_double) as uint32_t;
    let mut ideal_surface_height: uint32_t = ideal_height;
    if (*(*(*output).bar).config).height == 0 && (*output).height < ideal_surface_height
    {
        return box_size {
            width: width,
            height: ideal_surface_height,
        };
    }
    let mut height: uint32_t = (*output).height;
    cairo_set_operator(cairo, _cairo_operator::CAIRO_OPERATOR_SOURCE);
    cairo_set_source_u32(cairo, colors.background);
    (*ctx).background_color = colors.background;
    (*ctx).has_transparency = (*ctx).has_transparency as ::core::ffi::c_int
        | (colors.background & 0xff as uint32_t != 0xff as uint32_t)
            as ::core::ffi::c_int != 0;
    cairo_rectangle(
        cairo,
        x,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        width as ::core::ffi::c_double,
        height as ::core::ffi::c_double,
    );
    cairo_fill(cairo);
    cairo_set_source_u32(cairo, colors.border);
    cairo_rectangle(
        cairo,
        x,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        width as ::core::ffi::c_double,
        BORDER_WIDTH as ::core::ffi::c_double,
    );
    cairo_fill(cairo);
    cairo_rectangle(
        cairo,
        x,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        BORDER_WIDTH as ::core::ffi::c_double,
        height as ::core::ffi::c_double,
    );
    cairo_fill(cairo);
    cairo_rectangle(
        cairo,
        x + width as ::core::ffi::c_double - BORDER_WIDTH as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        BORDER_WIDTH as ::core::ffi::c_double,
        height as ::core::ffi::c_double,
    );
    cairo_fill(cairo);
    cairo_rectangle(
        cairo,
        x,
        height.wrapping_sub(BORDER_WIDTH as uint32_t) as ::core::ffi::c_double,
        width as ::core::ffi::c_double,
        BORDER_WIDTH as ::core::ffi::c_double,
    );
    cairo_fill(cairo);
    let mut text_y: ::core::ffi::c_double = height as ::core::ffi::c_double / 2.0f64
        - text_height as ::core::ffi::c_double / 2.0f64;
    cairo_set_source_u32(cairo, colors.text);
    cairo_move_to(
        cairo,
        x + width.wrapping_div(2 as uint32_t) as ::core::ffi::c_double
            - (text_width / 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
        floor(text_y) as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    choose_text_aa_mode(ctx, colors.text);
    render_text(
        cairo,
        (*config).font_description,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        pango_markup,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        label,
    );
    return box_size {
        width: width,
        height: (*output).height,
    };
}
unsafe extern "C" fn render_binding_mode_indicator(
    mut ctx: *mut render_context,
    mut x: ::core::ffi::c_double,
) -> uint32_t {
    let mut output: *mut swaybar_output = (*ctx).output;
    let mut mode: *const ::core::ffi::c_char = (*(*output).bar).mode;
    if mode.is_null() {
        return 0 as uint32_t;
    }
    let mut size: box_size = render_box(
        ctx,
        x,
        (*(*(*output).bar).config).colors.binding_mode,
        mode,
        (*(*output).bar).mode_pango_markup,
    );
    return size.height;
}
unsafe extern "C" fn workspace_hotspot_callback(
    mut output: *mut swaybar_output,
    mut hotspot: *mut swaybar_hotspot,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut button: uint32_t,
    mut released: bool,
    mut data: *mut ::core::ffi::c_void,
) -> hotspot_event_handling {
    if button != BTN_LEFT as uint32_t {
        return hotspot_event_handling::HOTSPOT_PROCESS;
    }
    if released {
        return hotspot_event_handling::HOTSPOT_IGNORE;
    }
    ipc_send_workspace_command((*output).bar, data as *const ::core::ffi::c_char);
    return hotspot_event_handling::HOTSPOT_IGNORE;
}
unsafe extern "C" fn render_workspace_button(
    mut ctx: *mut render_context,
    mut ws: *mut swaybar_workspace,
    mut x: *mut ::core::ffi::c_double,
) -> uint32_t {
    let mut output: *mut swaybar_output = (*ctx).output;
    let mut config: *mut swaybar_config = (*(*output).bar).config;
    let mut box_colors: box_colors = box_colors {
        border: 0,
        background: 0,
        text: 0,
    };
    if (*ws).urgent {
        box_colors = (*config).colors.urgent_workspace;
    } else if (*ws).focused {
        box_colors = (*config).colors.focused_workspace;
    } else if (*ws).visible {
        box_colors = (*config).colors.active_workspace;
    } else {
        box_colors = (*config).colors.inactive_workspace;
    }
    let mut size: box_size = render_box(
        ctx,
        *x,
        box_colors,
        (*ws).label,
        (*config).pango_markup,
    );
    let mut hotspot: *mut swaybar_hotspot = calloc(
        1 as size_t,
        ::core::mem::size_of::<swaybar_hotspot>(),
    ) as *mut swaybar_hotspot;
    (*hotspot).x = *x as ::core::ffi::c_int;
    (*hotspot).y = 0 as ::core::ffi::c_int;
    (*hotspot).width = size.width as ::core::ffi::c_int;
    (*hotspot).height = size.height as ::core::ffi::c_int;
    (*hotspot).callback = Some(
        workspace_hotspot_callback
            as unsafe extern "C" fn(
                *mut swaybar_output,
                *mut swaybar_hotspot,
                ::core::ffi::c_double,
                ::core::ffi::c_double,
                uint32_t,
                bool,
                *mut ::core::ffi::c_void,
            ) -> hotspot_event_handling,
    )
        as Option<
            unsafe extern "C" fn(
                *mut swaybar_output,
                *mut swaybar_hotspot,
                ::core::ffi::c_double,
                ::core::ffi::c_double,
                uint32_t,
                bool,
                *mut ::core::ffi::c_void,
            ) -> hotspot_event_handling,
        >;
    (*hotspot).destroy = Some(
        free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
    ) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    (*hotspot).data = strdup((*ws).name) as *mut ::core::ffi::c_void;
    wl_list_insert(&raw mut (*output).hotspots, &raw mut (*hotspot).link);
    *x += size.width as ::core::ffi::c_double;
    return size.height;
}
unsafe extern "C" fn render_to_cairo(mut ctx: *mut render_context) -> uint32_t {
    let mut cairo: *mut cairo_t = (*ctx).cairo;
    let mut output: *mut swaybar_output = (*ctx).output;
    let mut bar: *mut swaybar = (*output).bar;
    let mut config: *mut swaybar_config = (*bar).config;
    let mut th: ::core::ffi::c_int = 0;
    get_text_size(
        cairo,
        (*config).font_description,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut th,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        r#false != 0,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut max_height: uint32_t = (th as ::core::ffi::c_double
        + WS_VERTICAL_PADDING * 4 as ::core::ffi::c_int as ::core::ffi::c_double)
        as uint32_t;
    let mut x: ::core::ffi::c_double = (*output).width as ::core::ffi::c_double;
    if !(*bar).tray.is_null() {
        let mut h: uint32_t = render_tray(cairo, output, &raw mut x);
        max_height = if h > max_height { h } else { max_height };
    }
    if !(*bar).status.is_null() {
        let mut h_0: uint32_t = render_status_line(ctx, &raw mut x);
        max_height = if h_0 > max_height { h_0 } else { max_height };
    }
    x = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    if (*config).workspace_buttons {
        let mut ws: *mut swaybar_workspace = ::core::ptr::null_mut::<
            swaybar_workspace,
        >();
        ws = ((*output).workspaces.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_workspace
            as *mut swaybar_workspace;
        while &raw mut (*ws).link != &raw mut (*output).workspaces {
            let mut h_1: uint32_t = render_workspace_button(ctx, ws, &raw mut x);
            max_height = if h_1 > max_height { h_1 } else { max_height };
            ws = ((*ws).link.next as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_workspace
                as *mut swaybar_workspace;
        }
    }
    if (*config).binding_mode_indicator {
        let mut h_2: uint32_t = render_binding_mode_indicator(ctx, x);
        max_height = if h_2 > max_height { h_2 } else { max_height };
    }
    return if max_height > (*output).height { max_height } else { (*output).height };
}
unsafe extern "C" fn output_frame_handle_done(
    mut data: *mut ::core::ffi::c_void,
    mut callback: *mut wl_callback,
    mut time: uint32_t,
) {
    wl_callback_destroy(callback);
    let mut output: *mut swaybar_output = data as *mut swaybar_output;
    (*output).frame_scheduled = r#false != 0;
    if (*output).dirty {
        render_frame(output);
        (*output).dirty = r#false != 0;
    }
}
static mut output_frame_listener: wl_callback_listener = wl_callback_listener {
    done: Some(
        output_frame_handle_done
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_callback,
                uint32_t,
            ) -> (),
    ),
};
#[no_mangle]
pub unsafe extern "C" fn render_frame(mut output: *mut swaybar_output) {
    if !(*output).surface.is_null() {} else {
        __assert_fail(
            b"output->surface != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/render.c\0".as_ptr() as *const ::core::ffi::c_char,
            720 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    if (*output).layer_surface.is_null() {
        return;
    }
    free_hotspots(&raw mut (*output).hotspots);
    let mut background_color: uint32_t = 0;
    if (*output).focused {
        background_color = (*(*(*output).bar).config).colors.focused_background;
    } else {
        background_color = (*(*(*output).bar).config).colors.background;
    }
    let mut ctx: render_context = render_context {
        cairo: ::core::ptr::null_mut::<cairo_t>(),
        output: output,
        textaa_sharp: ::core::ptr::null_mut::<cairo_font_options_t>(),
        textaa_safe: ::core::ptr::null_mut::<cairo_font_options_t>(),
        background_color: background_color,
        has_transparency: background_color & 0xff as uint32_t != 0xff as uint32_t,
    };
    let mut recorder: *mut cairo_surface_t = cairo_recording_surface_create(
        _cairo_content::CAIRO_CONTENT_COLOR_ALPHA,
        ::core::ptr::null::<cairo_rectangle_t>(),
    );
    let mut cairo: *mut cairo_t = cairo_create(recorder);
    cairo_scale(
        cairo,
        (*output).scale as ::core::ffi::c_double,
        (*output).scale as ::core::ffi::c_double,
    );
    cairo_set_antialias(cairo, _cairo_antialias::CAIRO_ANTIALIAS_BEST);
    ctx.cairo = cairo;
    let mut fo: *mut cairo_font_options_t = cairo_font_options_create();
    cairo_font_options_set_antialias(fo, _cairo_antialias::CAIRO_ANTIALIAS_GRAY);
    ctx.textaa_safe = fo;
    if (*output).subpixel.0 == wl_output_subpixel::WL_OUTPUT_SUBPIXEL_NONE.0 {
        ctx.textaa_sharp = ctx.textaa_safe;
    } else {
        fo = cairo_font_options_create();
        cairo_font_options_set_antialias(fo, _cairo_antialias::CAIRO_ANTIALIAS_SUBPIXEL);
        cairo_font_options_set_subpixel_order(
            fo,
            to_cairo_subpixel_order((*output).subpixel),
        );
        ctx.textaa_sharp = fo;
    }
    cairo_set_operator(cairo, _cairo_operator::CAIRO_OPERATOR_SOURCE);
    cairo_set_source_u32(cairo, background_color);
    cairo_paint(cairo);
    let mut height: uint32_t = render_to_cairo(&raw mut ctx);
    let mut config_height: ::core::ffi::c_int = (*(*(*output).bar).config).height;
    if config_height > 0 as ::core::ffi::c_int {
        height = config_height as uint32_t;
    }
    if height != (*output).height || (*output).width == 0 as uint32_t {
        zwlr_layer_surface_v1_set_size((*output).layer_surface, 0 as uint32_t, height);
        zwlr_layer_surface_v1_set_margin(
            (*output).layer_surface,
            (*(*(*output).bar).config).gaps.top as int32_t,
            (*(*(*output).bar).config).gaps.right as int32_t,
            (*(*(*output).bar).config).gaps.bottom as int32_t,
            (*(*(*output).bar).config).gaps.left as int32_t,
        );
        if strcmp(
            (*(*(*output).bar).config).mode,
            b"dock\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            zwlr_layer_surface_v1_set_exclusive_zone(
                (*output).layer_surface,
                height as int32_t,
            );
        }
        wl_surface_commit((*output).surface);
    } else if height > 0 as uint32_t {
        (*output).current_buffer = get_next_buffer(
            (*(*output).bar).shm,
            &raw mut (*output).buffers as *mut pool_buffer,
            (*output).width.wrapping_mul((*output).scale as uint32_t),
            (*output).height.wrapping_mul((*output).scale as uint32_t),
        );
        if !(*output).current_buffer.is_null() {
            let mut shm: *mut cairo_t = (*(*output).current_buffer).cairo;
            cairo_save(shm);
            cairo_set_operator(shm, _cairo_operator::CAIRO_OPERATOR_CLEAR);
            cairo_paint(shm);
            cairo_restore(shm);
            cairo_set_source_surface(shm, recorder, 0.0f64, 0.0f64);
            cairo_paint(shm);
            wl_surface_set_buffer_scale((*output).surface, (*output).scale);
            wl_surface_attach(
                (*output).surface,
                (*(*output).current_buffer).buffer,
                0 as int32_t,
                0 as int32_t,
            );
            wl_surface_damage(
                (*output).surface,
                0 as int32_t,
                0 as int32_t,
                (*output).width as int32_t,
                (*output).height as int32_t,
            );
            if !ctx.has_transparency {
                let mut region: *mut wl_region = wl_compositor_create_region(
                    (*(*output).bar).compositor,
                );
                wl_region_add(
                    region,
                    0 as int32_t,
                    0 as int32_t,
                    INT32_MAX as int32_t,
                    INT32_MAX as int32_t,
                );
                wl_surface_set_opaque_region((*output).surface, region);
                wl_region_destroy(region);
            } else {
                wl_surface_set_opaque_region(
                    (*output).surface,
                    ::core::ptr::null_mut::<wl_region>(),
                );
            }
            let mut frame_callback: *mut wl_callback = wl_surface_frame(
                (*output).surface,
            );
            wl_callback_add_listener(
                frame_callback,
                &raw const output_frame_listener,
                output as *mut ::core::ffi::c_void,
            );
            (*output).frame_scheduled = r#true != 0;
            wl_surface_commit((*output).surface);
        }
    }
    if ctx.textaa_sharp != ctx.textaa_safe {
        cairo_font_options_destroy(ctx.textaa_sharp);
    }
    cairo_font_options_destroy(ctx.textaa_safe);
    cairo_surface_destroy(recorder);
    cairo_destroy(cairo);
}
pub const BTN_LEFT: ::core::ffi::c_int = 0x110 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
