use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type json_object;
    pub type wl_display;
    pub type wl_buffer;
    pub type wl_compositor;
    pub type wl_output;
    pub type wl_shm;
    pub type wl_surface;
    pub type _PangoContext;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type swaybar_watcher;
    pub type sd_bus;
    pub type r#loop;
    pub type wp_cursor_shape_manager_v1;
    pub type zxdg_output_manager_v1;
    pub type zwlr_layer_shell_v1;
    pub type zwlr_layer_surface_v1;
    pub type zxdg_output_v1;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_is_type(
        obj: *const json_object,
        r#type: json_type,
    ) -> ::core::ffi::c_int;
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
    fn json_object_get_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_tokener_error_desc(jerr: json_tokener_error) -> *const ::core::ffi::c_char;
    fn json_tokener_get_error(tok: *mut json_tokener) -> json_tokener_error;
    fn json_tokener_new_ex(depth: ::core::ffi::c_int) -> *mut json_tokener;
    fn json_tokener_free(tok: *mut json_tokener);
    fn json_tokener_parse(str: *const ::core::ffi::c_char) -> *mut json_object;
    fn json_tokener_parse_ex(
        tok: *mut json_tokener,
        str: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> *mut json_object;
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn wl_list_empty(list: *const wl_list) -> ::core::ffi::c_int;
    fn create_list() -> *mut list_t;
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn list_del(list: *mut list_t, index: ::core::ffi::c_int);
    fn list_free_items_and_destroy(list: *mut list_t);
    fn parse_color(color: *const ::core::ffi::c_char, result: *mut uint32_t) -> bool;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn pango_font_description_equal(
        desc1: *const PangoFontDescription,
        desc2: *const PangoFontDescription,
    ) -> gboolean;
    fn pango_font_description_free(desc: *mut PangoFontDescription);
    fn pango_font_description_from_string(
        str: *const ::core::ffi::c_char,
    ) -> *mut PangoFontDescription;
    fn free_tray_binding(binding: *mut tray_binding);
    fn init_config() -> *mut swaybar_config;
    fn free_config(config: *mut swaybar_config);
    fn parse_position(position: *const ::core::ffi::c_char) -> uint32_t;
    fn free_binding(binding: *mut swaybar_binding);
    fn determine_bar_visibility(bar: *mut swaybar, moving_layer: bool) -> bool;
    fn free_workspaces(list: *mut wl_list);
    fn status_in(
        fd: ::core::ffi::c_int,
        mask: ::core::ffi::c_short,
        data: *mut ::core::ffi::c_void,
    );
    fn destroy_layer_surface(output: *mut swaybar_output);
    fn status_line_init(cmd: *mut ::core::ffi::c_char) -> *mut status_line;
    fn status_line_free(status: *mut status_line);
    fn create_tray(bar: *mut swaybar) -> *mut swaybar_tray;
    fn destroy_tray(tray: *mut swaybar_tray);
    fn tray_in(
        fd: ::core::ffi::c_int,
        mask: ::core::ffi::c_short,
        data: *mut ::core::ffi::c_void,
    );
    fn ipc_single_command(
        socketfd: ::core::ffi::c_int,
        r#type: uint32_t,
        payload: *const ::core::ffi::c_char,
        len: *mut uint32_t,
    ) -> *mut ::core::ffi::c_char;
    fn ipc_recv_response(socketfd: ::core::ffi::c_int) -> *mut ipc_response;
    fn free_ipc_response(response: *mut ipc_response);
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
    fn has_prefix(
        str: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    ) -> bool;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
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
pub type int32_t = i32;
pub type uint32_t = u32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
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
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
pub type pid_t = __pid_t;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type PangoContext = _PangoContext;
pub type PangoFontDescription = _PangoFontDescription;
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
pub struct swaybar_binding {
    pub button: uint32_t,
    pub command: *mut ::core::ffi::c_char,
    pub release: bool,
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
pub struct tray_binding {
    pub button: uint32_t,
    pub command: *mut ::core::ffi::c_char,
    pub link: wl_list,
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
pub struct C2Rust_Unnamed_1 {
    pub name: *const ::core::ffi::c_char,
    pub color: *mut uint32_t,
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
pub struct ipc_response {
    pub size: uint32_t,
    pub r#type: uint32_t,
    pub payload: *mut ::core::ffi::c_char,
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ipc_send_workspace_command(
    mut bar: *mut swaybar,
    mut ws: *const ::core::ffi::c_char,
) {
    let mut size: uint32_t = strlen(
            b"workspace \"\"\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .wrapping_add(strlen(ws)) as uint32_t;
    let mut i: size_t = 0 as size_t;
    while i < strlen(ws) {
        if *ws.offset(i as isize) as ::core::ffi::c_int == '"' as ::core::ffi::c_int
            || *ws.offset(i as isize) as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
        {
            size = size.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    let mut command: *mut ::core::ffi::c_char = malloc(
        size.wrapping_add(1 as uint32_t) as size_t,
    ) as *mut ::core::ffi::c_char;
    if command.is_null() {
        return;
    }
    strcpy(command, b"workspace \"\0".as_ptr() as *const ::core::ffi::c_char);
    strcpy(
        command.offset(size.wrapping_sub(1 as uint32_t) as isize),
        b"\"\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i_0: size_t = 0 as size_t;
    let mut d: size_t = strlen(b"workspace \"\0".as_ptr() as *const ::core::ffi::c_char);
    while i_0 < strlen(ws) {
        if *ws.offset(i_0 as isize) as ::core::ffi::c_int == '"' as ::core::ffi::c_int
            || *ws.offset(i_0 as isize) as ::core::ffi::c_int
                == '\\' as ::core::ffi::c_int
        {
            let c2rust_fresh0 = d;
            d = d.wrapping_add(1);
            *command.offset(c2rust_fresh0 as isize) = '\\' as ::core::ffi::c_char;
        }
        let c2rust_fresh1 = d;
        d = d.wrapping_add(1);
        *command.offset(c2rust_fresh1 as isize) = *ws.offset(i_0 as isize);
        i_0 = i_0.wrapping_add(1);
    }
    free(
        ipc_single_command(
            (*bar).ipc_socketfd,
            ipc_command_type::IPC_COMMAND.0 as uint32_t,
            command,
            &raw mut size,
        ) as *mut ::core::ffi::c_void,
    );
    free(command as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn parse_font(
    mut font: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut new_font: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if has_prefix(font, b"pango:\0".as_ptr() as *const ::core::ffi::c_char) {
        font = font
            .offset(strlen(b"pango:\0".as_ptr() as *const ::core::ffi::c_char) as isize);
    }
    new_font = strdup(font);
    return new_font;
}
unsafe extern "C" fn ipc_parse_colors(
    mut config: *mut swaybar_config,
    mut colors: *mut json_object,
) {
    let mut properties: [C2Rust_Unnamed_1; 21] = [
        C2Rust_Unnamed_1 {
            name: b"background\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.background,
        },
        C2Rust_Unnamed_1 {
            name: b"statusline\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.statusline,
        },
        C2Rust_Unnamed_1 {
            name: b"separator\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.separator,
        },
        C2Rust_Unnamed_1 {
            name: b"focused_background\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.focused_background,
        },
        C2Rust_Unnamed_1 {
            name: b"focused_statusline\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.focused_statusline,
        },
        C2Rust_Unnamed_1 {
            name: b"focused_separator\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.focused_separator,
        },
        C2Rust_Unnamed_1 {
            name: b"focused_workspace_border\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.focused_workspace.border,
        },
        C2Rust_Unnamed_1 {
            name: b"focused_workspace_bg\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.focused_workspace.background,
        },
        C2Rust_Unnamed_1 {
            name: b"focused_workspace_text\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.focused_workspace.text,
        },
        C2Rust_Unnamed_1 {
            name: b"active_workspace_border\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.active_workspace.border,
        },
        C2Rust_Unnamed_1 {
            name: b"active_workspace_bg\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.active_workspace.background,
        },
        C2Rust_Unnamed_1 {
            name: b"active_workspace_text\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.active_workspace.text,
        },
        C2Rust_Unnamed_1 {
            name: b"inactive_workspace_border\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.inactive_workspace.border,
        },
        C2Rust_Unnamed_1 {
            name: b"inactive_workspace_bg\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.inactive_workspace.background,
        },
        C2Rust_Unnamed_1 {
            name: b"inactive_workspace_text\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.inactive_workspace.text,
        },
        C2Rust_Unnamed_1 {
            name: b"urgent_workspace_border\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.urgent_workspace.border,
        },
        C2Rust_Unnamed_1 {
            name: b"urgent_workspace_bg\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.urgent_workspace.background,
        },
        C2Rust_Unnamed_1 {
            name: b"urgent_workspace_text\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.urgent_workspace.text,
        },
        C2Rust_Unnamed_1 {
            name: b"binding_mode_border\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.binding_mode.border,
        },
        C2Rust_Unnamed_1 {
            name: b"binding_mode_bg\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.binding_mode.background,
        },
        C2Rust_Unnamed_1 {
            name: b"binding_mode_text\0".as_ptr() as *const ::core::ffi::c_char,
            color: &raw mut (*config).colors.binding_mode.text,
        },
    ];
    let mut i: size_t = 0 as size_t;
    while i
        < ::core::mem::size_of::<[C2Rust_Unnamed_1; 21]>()
            .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_1>())
    {
        let mut object: *mut json_object = ::core::ptr::null_mut::<json_object>();
        if json_object_object_get_ex(colors, properties[i].name, &raw mut object) != 0 {
            let mut hexstring: *const ::core::ffi::c_char = json_object_get_string(
                object,
            );
            if !parse_color(hexstring, properties[i].color) {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] Ignoring invalid %s: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaybar/ipc.c\0".as_ptr() as *const ::core::ffi::c_char,
                    90 as ::core::ffi::c_int,
                    properties[i].name,
                    hexstring,
                );
            }
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn ipc_parse_config(
    mut config: *mut swaybar_config,
    mut payload: *const ::core::ffi::c_char,
) -> bool {
    let mut bar_config: *mut json_object = json_tokener_parse(payload);
    let mut success: *mut json_object = ::core::ptr::null_mut::<json_object>();
    if json_object_object_get_ex(
        bar_config,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut success,
    ) != 0 && json_object_get_boolean(success) == 0
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] No bar with that ID. Use 'swaymsg -t get_bar_config' to get the available bar configs.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/ipc.c\0".as_ptr() as *const ::core::ffi::c_char,
            103 as ::core::ffi::c_int,
        );
        json_object_put(bar_config);
        return r#false != 0;
    }
    let mut bar_height: *mut json_object = json_object_object_get(
        bar_config,
        b"bar_height\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !bar_height.is_null() {
        (*config).height = json_object_get_int(bar_height) as ::core::ffi::c_int;
    }
    let mut binding_mode_indicator: *mut json_object = json_object_object_get(
        bar_config,
        b"binding_mode_indicator\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !binding_mode_indicator.is_null() {
        (*config).binding_mode_indicator = json_object_get_boolean(
            binding_mode_indicator,
        ) != 0;
    }
    let mut bindings: *mut json_object = json_object_object_get(
        bar_config,
        b"bindings\0".as_ptr() as *const ::core::ffi::c_char,
    );
    while (*(*config).bindings).length != 0 {
        let mut binding: *mut swaybar_binding = *(*(*config).bindings)
            .items
            .offset(0isize) as *mut swaybar_binding;
        list_del((*config).bindings, 0 as ::core::ffi::c_int);
        free_binding(binding);
    }
    if !bindings.is_null() {
        let mut length: ::core::ffi::c_int = json_object_array_length(bindings)
            as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < length {
            let mut bindobj: *mut json_object = json_object_array_get_idx(
                bindings,
                i as size_t,
            );
            let mut binding_0: *mut swaybar_binding = calloc(
                1 as size_t,
                ::core::mem::size_of::<swaybar_binding>(),
            ) as *mut swaybar_binding;
            (*binding_0).button = json_object_get_int(
                json_object_object_get(
                    bindobj,
                    b"event_code\0".as_ptr() as *const ::core::ffi::c_char,
                ),
            ) as uint32_t;
            (*binding_0).command = strdup(
                json_object_get_string(
                    json_object_object_get(
                        bindobj,
                        b"command\0".as_ptr() as *const ::core::ffi::c_char,
                    ),
                ),
            );
            (*binding_0).release = json_object_get_boolean(
                json_object_object_get(
                    bindobj,
                    b"release\0".as_ptr() as *const ::core::ffi::c_char,
                ),
            ) != 0;
            list_add((*config).bindings, binding_0 as *mut ::core::ffi::c_void);
            i += 1;
        }
    }
    let mut colors: *mut json_object = json_object_object_get(
        bar_config,
        b"colors\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !colors.is_null() {
        ipc_parse_colors(config, colors);
    }
    let mut font: *mut json_object = json_object_object_get(
        bar_config,
        b"font\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !font.is_null() {
        pango_font_description_free((*config).font_description);
        let mut font_value: *mut ::core::ffi::c_char = parse_font(
            json_object_get_string(font),
        );
        (*config).font_description = pango_font_description_from_string(font_value);
        free(font_value as *mut ::core::ffi::c_void);
    }
    let mut gaps: *mut json_object = json_object_object_get(
        bar_config,
        b"gaps\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !gaps.is_null() {
        let mut top: *mut json_object = json_object_object_get(
            gaps,
            b"top\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !top.is_null() {
            (*config).gaps.top = json_object_get_int(top) as ::core::ffi::c_int;
        }
        let mut right: *mut json_object = json_object_object_get(
            gaps,
            b"right\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !right.is_null() {
            (*config).gaps.right = json_object_get_int(right) as ::core::ffi::c_int;
        }
        let mut bottom: *mut json_object = json_object_object_get(
            gaps,
            b"bottom\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !bottom.is_null() {
            (*config).gaps.bottom = json_object_get_int(bottom) as ::core::ffi::c_int;
        }
        let mut left: *mut json_object = json_object_object_get(
            gaps,
            b"left\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !left.is_null() {
            (*config).gaps.left = json_object_get_int(left) as ::core::ffi::c_int;
        }
    }
    let mut hidden_state: *mut json_object = json_object_object_get(
        bar_config,
        b"hidden_state\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !hidden_state.is_null() {
        free((*config).hidden_state as *mut ::core::ffi::c_void);
        (*config).hidden_state = strdup(json_object_get_string(hidden_state));
    }
    let mut mode: *mut json_object = json_object_object_get(
        bar_config,
        b"mode\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mode.is_null() {
        free((*config).mode as *mut ::core::ffi::c_void);
        (*config).mode = strdup(json_object_get_string(mode));
    }
    let mut outputs: *mut json_object = json_object_object_get(
        bar_config,
        b"outputs\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut output: *mut config_output = ::core::ptr::null_mut::<config_output>();
    let mut tmp: *mut config_output = ::core::ptr::null_mut::<config_output>();
    output = ((*config).outputs.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
        as *mut config_output;
    tmp = ((*output).link.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
        as *mut config_output;
    while &raw mut (*output).link != &raw mut (*config).outputs {
        wl_list_remove(&raw mut (*output).link);
        free((*output).name as *mut ::core::ffi::c_void);
        free(output as *mut ::core::ffi::c_void);
        output = tmp;
        tmp = ((*output).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
            as *mut config_output;
    }
    if !outputs.is_null() {
        let mut length_0: ::core::ffi::c_int = json_object_array_length(outputs)
            as ::core::ffi::c_int;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < length_0 {
            let mut output_0: *mut json_object = json_object_array_get_idx(
                outputs,
                i_0 as size_t,
            );
            let mut name: *const ::core::ffi::c_char = json_object_get_string(output_0);
            if strcmp(b"*\0".as_ptr() as *const ::core::ffi::c_char, name)
                == 0 as ::core::ffi::c_int
            {
                let mut coutput: *mut config_output = ::core::ptr::null_mut::<
                    config_output,
                >();
                let mut tmp_0: *mut config_output = ::core::ptr::null_mut::<
                    config_output,
                >();
                coutput = ((*config).outputs.next as *mut ::core::ffi::c_char)
                    .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
                    as *mut config_output;
                tmp_0 = ((*coutput).link.next as *mut ::core::ffi::c_char)
                    .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
                    as *mut config_output;
                while &raw mut (*coutput).link != &raw mut (*config).outputs {
                    wl_list_remove(&raw mut (*coutput).link);
                    free((*coutput).name as *mut ::core::ffi::c_void);
                    free(coutput as *mut ::core::ffi::c_void);
                    coutput = tmp_0;
                    tmp_0 = ((*coutput).link.next as *mut ::core::ffi::c_char)
                        .offset(-(0 as ::core::ffi::c_ulong as isize))
                        as *mut config_output as *mut config_output;
                }
                break;
            } else {
                let mut coutput_0: *mut config_output = calloc(
                    1 as size_t,
                    ::core::mem::size_of::<config_output>(),
                ) as *mut config_output;
                (*coutput_0).name = strdup(name);
                wl_list_insert(&raw mut (*config).outputs, &raw mut (*coutput_0).link);
                i_0 += 1;
            }
        }
    }
    let mut pango_markup: *mut json_object = json_object_object_get(
        bar_config,
        b"pango_markup\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !pango_markup.is_null() {
        (*config).pango_markup = json_object_get_boolean(pango_markup) != 0;
    }
    let mut position: *mut json_object = json_object_object_get(
        bar_config,
        b"position\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !position.is_null() {
        (*config).position = parse_position(json_object_get_string(position));
    }
    let mut separator_symbol: *mut json_object = json_object_object_get(
        bar_config,
        b"separator_symbol\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !separator_symbol.is_null() {
        free((*config).sep_symbol as *mut ::core::ffi::c_void);
        (*config).sep_symbol = strdup(json_object_get_string(separator_symbol));
    }
    let mut status_command: *mut json_object = json_object_object_get(
        bar_config,
        b"status_command\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !status_command.is_null() {
        let mut command: *const ::core::ffi::c_char = json_object_get_string(
            status_command,
        );
        free((*config).status_command as *mut ::core::ffi::c_void);
        (*config).status_command = strdup(command);
    }
    let mut status_edge_padding: *mut json_object = json_object_object_get(
        bar_config,
        b"status_edge_padding\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !status_edge_padding.is_null() {
        (*config).status_edge_padding = json_object_get_int(status_edge_padding)
            as ::core::ffi::c_int;
    }
    let mut status_padding: *mut json_object = json_object_object_get(
        bar_config,
        b"status_padding\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !status_padding.is_null() {
        (*config).status_padding = json_object_get_int(status_padding)
            as ::core::ffi::c_int;
    }
    let mut strip_workspace_name: *mut json_object = json_object_object_get(
        bar_config,
        b"strip_workspace_name\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !strip_workspace_name.is_null() {
        (*config).strip_workspace_name = json_object_get_boolean(strip_workspace_name)
            != 0;
    }
    let mut strip_workspace_numbers: *mut json_object = json_object_object_get(
        bar_config,
        b"strip_workspace_numbers\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !strip_workspace_numbers.is_null() {
        (*config).strip_workspace_numbers = json_object_get_boolean(
            strip_workspace_numbers,
        ) != 0;
    }
    let mut workspace_buttons: *mut json_object = json_object_object_get(
        bar_config,
        b"workspace_buttons\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !workspace_buttons.is_null() {
        (*config).workspace_buttons = json_object_get_boolean(workspace_buttons) != 0;
    }
    let mut workspace_min_width: *mut json_object = json_object_object_get(
        bar_config,
        b"workspace_min_width\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !workspace_min_width.is_null() {
        (*config).workspace_min_width = json_object_get_int(workspace_min_width)
            as uint32_t;
    }
    let mut wrap_scroll: *mut json_object = json_object_object_get(
        bar_config,
        b"wrap_scroll\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !wrap_scroll.is_null() {
        (*config).wrap_scroll = json_object_get_boolean(wrap_scroll) != 0;
    }
    let mut tray_outputs: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut tray_padding: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut tray_bindings: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut icon_theme: *mut json_object = ::core::ptr::null_mut::<json_object>();
    if !(*config).tray_outputs.is_null() && (*(*config).tray_outputs).length != 0 {
        list_free_items_and_destroy((*config).tray_outputs);
    }
    if json_object_object_get_ex(
        bar_config,
        b"tray_outputs\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut tray_outputs,
    ) != 0
    {
        (*config).tray_outputs = create_list();
        let mut length_1: ::core::ffi::c_int = json_object_array_length(tray_outputs)
            as ::core::ffi::c_int;
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < length_1 {
            let mut output_1: *mut json_object = json_object_array_get_idx(
                tray_outputs,
                i_1 as size_t,
            );
            let mut name_0: *const ::core::ffi::c_char = json_object_get_string(
                output_1,
            );
            if strcmp(name_0, b"none\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                (*config).tray_hidden = r#true != 0;
                list_free_items_and_destroy((*config).tray_outputs);
                (*config).tray_outputs = create_list();
                break;
            } else {
                list_add(
                    (*config).tray_outputs,
                    strdup(name_0) as *mut ::core::ffi::c_void,
                );
                i_1 += 1;
            }
        }
    }
    if json_object_object_get_ex(
        bar_config,
        b"tray_padding\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut tray_padding,
    ) != 0
    {
        (*config).tray_padding = json_object_get_int(tray_padding) as ::core::ffi::c_int;
    }
    let mut tray_bind: *mut tray_binding = ::core::ptr::null_mut::<tray_binding>();
    let mut tmp_tray_bind: *mut tray_binding = ::core::ptr::null_mut::<tray_binding>();
    tray_bind = ((*config).tray_bindings.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut tray_binding
        as *mut tray_binding;
    tmp_tray_bind = ((*tray_bind).link.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut tray_binding
        as *mut tray_binding;
    while &raw mut (*tray_bind).link != &raw mut (*config).tray_bindings {
        wl_list_remove(&raw mut (*tray_bind).link);
        free_tray_binding(tray_bind);
        tray_bind = tmp_tray_bind;
        tmp_tray_bind = ((*tray_bind).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut tray_binding
            as *mut tray_binding;
    }
    if json_object_object_get_ex(
        bar_config,
        b"tray_bindings\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut tray_bindings,
    ) != 0
    {
        let mut length_2: ::core::ffi::c_int = json_object_array_length(tray_bindings)
            as ::core::ffi::c_int;
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 < length_2 {
            let mut bind: *mut json_object = json_object_array_get_idx(
                tray_bindings,
                i_2 as size_t,
            );
            let mut binding_1: *mut tray_binding = calloc(
                1 as size_t,
                ::core::mem::size_of::<tray_binding>(),
            ) as *mut tray_binding;
            (*binding_1).button = json_object_get_int(
                json_object_object_get(
                    bind,
                    b"event_code\0".as_ptr() as *const ::core::ffi::c_char,
                ),
            ) as uint32_t;
            (*binding_1).command = strdup(
                json_object_get_string(
                    json_object_object_get(
                        bind,
                        b"command\0".as_ptr() as *const ::core::ffi::c_char,
                    ),
                ),
            );
            wl_list_insert(&raw mut (*config).tray_bindings, &raw mut (*binding_1).link);
            i_2 += 1;
        }
    }
    if json_object_object_get_ex(
        bar_config,
        b"icon_theme\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut icon_theme,
    ) != 0
    {
        (*config).icon_theme = strdup(json_object_get_string(icon_theme));
    }
    json_object_put(bar_config);
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_get_workspaces(mut bar: *mut swaybar) -> bool {
    let mut output: *mut swaybar_output = ::core::ptr::null_mut::<swaybar_output>();
    output = ((*bar).outputs.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    while &raw mut (*output).link != &raw mut (*bar).outputs {
        free_workspaces(&raw mut (*output).workspaces);
        (*output).focused = r#false != 0;
        output = ((*output).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
            as *mut swaybar_output;
    }
    let mut len: uint32_t = 0 as uint32_t;
    let mut res: *mut ::core::ffi::c_char = ipc_single_command(
        (*bar).ipc_socketfd,
        ipc_command_type::IPC_GET_WORKSPACES.0 as uint32_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut len,
    );
    let mut results: *mut json_object = json_tokener_parse(res);
    if results.is_null() {
        free(res as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    (*bar).visible_by_urgency = r#false != 0;
    let mut length: size_t = json_object_array_length(results);
    let mut ws_json: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut num: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut name: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut visible: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut focused: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut out: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut urgent: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut i: size_t = 0 as size_t;
    while i < length {
        ws_json = json_object_array_get_idx(results, i) as *mut json_object;
        json_object_object_get_ex(
            ws_json,
            b"num\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut num,
        );
        json_object_object_get_ex(
            ws_json,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut name,
        );
        json_object_object_get_ex(
            ws_json,
            b"visible\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut visible,
        );
        json_object_object_get_ex(
            ws_json,
            b"focused\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut focused,
        );
        json_object_object_get_ex(
            ws_json,
            b"output\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut out,
        );
        json_object_object_get_ex(
            ws_json,
            b"urgent\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut urgent,
        );
        output = ((*bar).outputs.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
            as *mut swaybar_output;
        while &raw mut (*output).link != &raw mut (*bar).outputs {
            let mut ws_output: *const ::core::ffi::c_char = json_object_get_string(out);
            if !ws_output.is_null()
                && strcmp(ws_output, (*output).name) == 0 as ::core::ffi::c_int
            {
                let mut ws: *mut swaybar_workspace = calloc(
                    1 as size_t,
                    ::core::mem::size_of::<swaybar_workspace>(),
                ) as *mut swaybar_workspace;
                (*ws).num = json_object_get_int(num) as ::core::ffi::c_int;
                (*ws).name = strdup(json_object_get_string(name));
                (*ws).label = strdup((*ws).name);
                if (*ws).num != -1 as ::core::ffi::c_int {
                    let mut len_offset: size_t = snprintf(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        0 as size_t,
                        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                        (*ws).num,
                    ) as size_t;
                    if (*(*bar).config).strip_workspace_name {
                        free((*ws).label as *mut ::core::ffi::c_void);
                        (*ws).label = malloc(len_offset.wrapping_add(1 as size_t))
                            as *mut ::core::ffi::c_char;
                        snprintf(
                            (*ws).label,
                            len_offset.wrapping_add(1 as size_t),
                            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                            (*ws).num,
                        );
                    } else if (*(*bar).config).strip_workspace_numbers {
                        len_offset = len_offset
                            .wrapping_add(
                                (*(*ws).label.offset(len_offset as isize)
                                    as ::core::ffi::c_int == ':' as ::core::ffi::c_int)
                                    as ::core::ffi::c_int as size_t,
                            );
                        if *(*ws).name.offset(len_offset as isize) as ::core::ffi::c_int
                            != '\0' as ::core::ffi::c_int
                        {
                            free((*ws).label as *mut ::core::ffi::c_void);
                            (*ws).label = strdup((*ws).name.offset(len_offset as isize));
                        }
                    }
                }
                (*ws).visible = json_object_get_boolean(visible) != 0;
                (*ws).focused = json_object_get_boolean(focused) != 0;
                if (*ws).focused {
                    (*output).focused = r#true != 0;
                }
                (*ws).urgent = json_object_get_boolean(urgent) != 0;
                if (*ws).urgent {
                    (*bar).visible_by_urgency = r#true != 0;
                }
                wl_list_insert((*output).workspaces.prev, &raw mut (*ws).link);
            }
            output = ((*output).link.next as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
                as *mut swaybar_output;
        }
        i = i.wrapping_add(1);
    }
    json_object_put(results);
    free(res as *mut ::core::ffi::c_void);
    return determine_bar_visibility(bar, r#false != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_execute_binding(
    mut bar: *mut swaybar,
    mut bind: *mut swaybar_binding,
) {
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Executing binding for button %u (release=%d): `%s`\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"swaybar/ipc.c\0".as_ptr() as *const ::core::ffi::c_char,
        412 as ::core::ffi::c_int,
        (*bind).button,
        (*bind).release as ::core::ffi::c_int,
        (*bind).command,
    );
    let mut len: uint32_t = strlen((*bind).command) as uint32_t;
    free(
        ipc_single_command(
            (*bar).ipc_socketfd,
            ipc_command_type::IPC_COMMAND.0 as uint32_t,
            (*bind).command,
            &raw mut len,
        ) as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ipc_initialize(mut bar: *mut swaybar) -> bool {
    if (*bar).id.is_null() {
        let mut len: uint32_t = 0 as uint32_t;
        let mut res: *mut ::core::ffi::c_char = ipc_single_command(
            (*bar).ipc_socketfd,
            ipc_command_type::IPC_GET_BAR_CONFIG.0 as uint32_t,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut len,
        );
        let mut bars: *mut json_object = json_tokener_parse(res);
        if json_object_is_type(bars, json_type::json_type_array) == 0
            || json_object_array_length(bars) == 0 as size_t
        {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] No bar configuration found, please configure a bar block in your sway config file.\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"swaybar/ipc.c\0".as_ptr() as *const ::core::ffi::c_char,
                427 as ::core::ffi::c_int,
            );
            json_object_put(bars);
            free(res as *mut ::core::ffi::c_void);
            return r#false != 0;
        }
        let mut first: *mut json_object = json_object_array_get_idx(bars, 0 as size_t);
        (*bar).id = strdup(json_object_get_string(first));
        json_object_put(bars);
        free(res as *mut ::core::ffi::c_void);
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Using first bar config: %s. Use --bar_id to manually select a different bar configuration.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/ipc.c\0".as_ptr() as *const ::core::ffi::c_char,
            437 as ::core::ffi::c_int,
            (*bar).id,
        );
    }
    let mut len_0: uint32_t = strlen((*bar).id) as uint32_t;
    let mut res_0: *mut ::core::ffi::c_char = ipc_single_command(
        (*bar).ipc_socketfd,
        ipc_command_type::IPC_GET_BAR_CONFIG.0 as uint32_t,
        (*bar).id,
        &raw mut len_0,
    );
    if !ipc_parse_config((*bar).config, res_0) {
        free(res_0 as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    free(res_0 as *mut ::core::ffi::c_void);
    let mut subscribe: *mut ::core::ffi::c_char = b"[ \"barconfig_update\", \"bar_state_update\", \"mode\", \"workspace\" ]\0"
        .as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    len_0 = strlen(subscribe) as uint32_t;
    free(
        ipc_single_command(
            (*bar).ipc_event_socketfd,
            ipc_command_type::IPC_SUBSCRIBE.0 as uint32_t,
            subscribe,
            &raw mut len_0,
        ) as *mut ::core::ffi::c_void,
    );
    return r#true != 0;
}
unsafe extern "C" fn handle_bar_state_update(
    mut bar: *mut swaybar,
    mut event: *mut json_object,
) -> bool {
    let mut json_id: *mut json_object = ::core::ptr::null_mut::<json_object>();
    json_object_object_get_ex(
        event,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut json_id,
    );
    let mut id: *const ::core::ffi::c_char = json_object_get_string(json_id);
    if strcmp(id, (*bar).id) != 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut visible_by_modifier: *mut json_object = ::core::ptr::null_mut::<
        json_object,
    >();
    json_object_object_get_ex(
        event,
        b"visible_by_modifier\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut visible_by_modifier,
    );
    (*bar).visible_by_modifier = json_object_get_boolean(visible_by_modifier) != 0;
    if (*bar).visible_by_modifier {
        (*bar).visible_by_mode = r#false != 0;
        (*bar).visible_by_urgency = r#false != 0;
    }
    return determine_bar_visibility(bar, r#false != 0);
}
unsafe extern "C" fn handle_barconfig_update(
    mut bar: *mut swaybar,
    mut payload: *const ::core::ffi::c_char,
    mut json_config: *mut json_object,
) -> bool {
    let mut json_id: *mut json_object = json_object_object_get(
        json_config,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut id: *const ::core::ffi::c_char = json_object_get_string(json_id);
    if strcmp(id, (*bar).id) != 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut newcfg: *mut swaybar_config = init_config();
    ipc_parse_config(newcfg, payload);
    let mut oldcfg: *mut swaybar_config = (*bar).config;
    (*bar).config = newcfg;
    let mut output: *mut swaybar_output = ::core::ptr::null_mut::<swaybar_output>();
    let mut tmp_output: *mut swaybar_output = ::core::ptr::null_mut::<swaybar_output>();
    output = ((*bar).outputs.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    tmp_output = ((*output).link.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    while &raw mut (*output).link != &raw mut (*bar).outputs {
        let mut found: bool = wl_list_empty(&raw mut (*newcfg).outputs) != 0;
        let mut coutput: *mut config_output = ::core::ptr::null_mut::<config_output>();
        coutput = ((*newcfg).outputs.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
            as *mut config_output;
        while &raw mut (*coutput).link != &raw mut (*newcfg).outputs {
            if strcmp((*coutput).name, (*output).name) == 0 as ::core::ffi::c_int
                || strcmp((*coutput).name, (*output).identifier)
                    == 0 as ::core::ffi::c_int
            {
                found = r#true != 0;
                break;
            } else {
                coutput = ((*coutput).link.next as *mut ::core::ffi::c_char)
                    .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
                    as *mut config_output;
            }
        }
        if !found {
            destroy_layer_surface(output);
            wl_list_remove(&raw mut (*output).link);
            wl_list_insert(&raw mut (*bar).unused_outputs, &raw mut (*output).link);
        } else if pango_font_description_equal(
            (*oldcfg).font_description,
            (*newcfg).font_description,
        ) == 0
        {
            (*output).height = 0 as uint32_t;
        }
        output = tmp_output;
        tmp_output = ((*output).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
            as *mut swaybar_output;
    }
    output = ((*bar).unused_outputs.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    tmp_output = ((*output).link.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    while &raw mut (*output).link != &raw mut (*bar).unused_outputs {
        let mut found_0: bool = wl_list_empty(&raw mut (*newcfg).outputs) != 0;
        let mut coutput_0: *mut config_output = ::core::ptr::null_mut::<config_output>();
        coutput_0 = ((*newcfg).outputs.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
            as *mut config_output;
        while &raw mut (*coutput_0).link != &raw mut (*newcfg).outputs {
            if strcmp((*coutput_0).name, (*output).name) == 0 as ::core::ffi::c_int
                || strcmp((*coutput_0).name, (*output).identifier)
                    == 0 as ::core::ffi::c_int
            {
                found_0 = r#true != 0;
                break;
            } else {
                coutput_0 = ((*coutput_0).link.next as *mut ::core::ffi::c_char)
                    .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
                    as *mut config_output;
            }
        }
        if found_0 {
            wl_list_remove(&raw mut (*output).link);
            wl_list_insert(&raw mut (*bar).outputs, &raw mut (*output).link);
        }
        output = tmp_output;
        tmp_output = ((*output).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
            as *mut swaybar_output;
    }
    if !(*bar).status.is_null()
        && ((*newcfg).status_command.is_null()
            || strcmp((*newcfg).status_command, (*oldcfg).status_command)
                != 0 as ::core::ffi::c_int)
    {
        status_line_free((*bar).status);
        (*bar).status = ::core::ptr::null_mut::<status_line>();
    }
    if (*bar).status.is_null() && !(*newcfg).status_command.is_null() {
        (*bar).status = status_line_init((*newcfg).status_command) as *mut status_line;
        (*(*bar).status).bar = bar;
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
    if (*oldcfg).tray_hidden as ::core::ffi::c_int != 0 && !(*newcfg).tray_hidden {
        (*bar).tray = create_tray(bar);
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
    } else if !(*bar).tray.is_null() && (*newcfg).tray_hidden as ::core::ffi::c_int != 0
    {
        loop_remove_fd((*bar).eventloop, (*(*bar).tray).fd);
        destroy_tray((*bar).tray);
        (*bar).tray = ::core::ptr::null_mut::<swaybar_tray>();
    }
    if (*newcfg).workspace_buttons {
        ipc_get_workspaces(bar);
    }
    let mut moving_layer: bool = strcmp((*oldcfg).mode, (*newcfg).mode)
        != 0 as ::core::ffi::c_int;
    free_config(oldcfg);
    determine_bar_visibility(bar, moving_layer);
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn handle_ipc_readable(mut bar: *mut swaybar) -> bool {
    let mut resp: *mut ipc_response = ipc_recv_response((*bar).ipc_event_socketfd);
    if resp.is_null() {
        return r#false != 0;
    }
    let mut tok: *mut json_tokener = json_tokener_new_ex(JSON_MAX_DEPTH);
    if tok.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] failed to create tokener: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/ipc.c\0".as_ptr() as *const ::core::ffi::c_char,
            574 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        free_ipc_response(resp);
        return r#false != 0;
    }
    let mut result: *mut json_object = json_tokener_parse_ex(
        tok,
        (*resp).payload,
        -1 as ::core::ffi::c_int,
    );
    let mut err: json_tokener_error = json_tokener_get_error(tok);
    json_tokener_free(tok);
    if err.0 != json_tokener_error::json_tokener_success.0 {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] failed to parse payload as json: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/ipc.c\0".as_ptr() as *const ::core::ffi::c_char,
            584 as ::core::ffi::c_int,
            json_tokener_error_desc(err),
        );
        free_ipc_response(resp);
        return r#false != 0;
    }
    let mut bar_is_dirty: bool = r#true != 0;
    match (*resp).r#type {
        2147483648 => {
            bar_is_dirty = ipc_get_workspaces(bar);
        }
        2147483650 => {
            let mut json_change: *mut json_object = ::core::ptr::null_mut::<
                json_object,
            >();
            let mut json_pango_markup: *mut json_object = ::core::ptr::null_mut::<
                json_object,
            >();
            if json_object_object_get_ex(
                result,
                b"change\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut json_change,
            ) != 0
            {
                let mut change: *const ::core::ffi::c_char = json_object_get_string(
                    json_change,
                );
                free((*bar).mode as *mut ::core::ffi::c_void);
                (*bar).mode = if strcmp(
                    change,
                    b"default\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                {
                    strdup(change)
                } else {
                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                };
                (*bar).visible_by_mode = !(*bar).mode.is_null();
                determine_bar_visibility(bar, r#false != 0);
                if json_object_object_get_ex(
                    result,
                    b"pango_markup\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut json_pango_markup,
                ) != 0
                {
                    (*bar).mode_pango_markup = json_object_get_boolean(json_pango_markup)
                        != 0;
                }
            } else {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] failed to parse response\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaybar/ipc.c\0".as_ptr() as *const ::core::ffi::c_char,
                    604 as ::core::ffi::c_int,
                );
                bar_is_dirty = r#false != 0;
            }
        }
        2147483652 => {
            bar_is_dirty = handle_barconfig_update(bar, (*resp).payload, result);
        }
        2147483668 => {
            bar_is_dirty = handle_bar_state_update(bar, result);
        }
        _ => {
            bar_is_dirty = r#false != 0;
        }
    }
    json_object_put(result);
    free_ipc_response(resp);
    return bar_is_dirty;
}
pub const JSON_MAX_DEPTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
