use ::c2rust_bitfields;
extern "C" {
    pub type json_object;
    pub type wl_display;
    pub type wl_compositor;
    pub type wl_shm;
    pub type _PangoFontDescription;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type swaybar_tray;
    pub type r#loop;
    pub type wp_cursor_shape_manager_v1;
    pub type zxdg_output_manager_v1;
    pub type zwlr_layer_shell_v1;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_get_type(obj: *const json_object) -> json_type;
    fn json_object_to_json_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const ::core::ffi::c_char,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
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
    fn json_object_new_int(i: int32_t) -> *mut json_object;
    fn json_object_get_int(obj: *const json_object) -> int32_t;
    fn json_object_new_double(d: ::core::ffi::c_double) -> *mut json_object;
    fn json_object_new_string(s: *const ::core::ffi::c_char) -> *mut json_object;
    fn json_object_get_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_tokener_error_desc(jerr: json_tokener_error) -> *const ::core::ffi::c_char;
    fn json_tokener_get_error(tok: *mut json_tokener) -> json_tokener_error;
    fn json_tokener_reset(tok: *mut json_tokener);
    fn json_tokener_parse_ex(
        tok: *mut json_tokener,
        str: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> *mut json_object;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strspn(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn parse_color(color: *const ::core::ffi::c_char, result: *mut uint32_t) -> bool;
    fn dprintf(
        __fd: ::core::ffi::c_int,
        __fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn event_to_x11_button(event: uint32_t) -> uint32_t;
    fn status_error(status: *mut status_line, text: *const ::core::ffi::c_char);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type int32_t = i32;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub bpos: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
}
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const _ISupper: Self = Self(256);
    pub const _ISlower: Self = Self(512);
    pub const _ISalpha: Self = Self(1024);
    pub const _ISdigit: Self = Self(2048);
    pub const _ISxdigit: Self = Self(4096);
    pub const _ISspace: Self = Self(8192);
    pub const _ISprint: Self = Self(16384);
    pub const _ISgraph: Self = Self(32768);
    pub const _ISblank: Self = Self(1);
    pub const _IScntrl: Self = Self(2);
    pub const _ISpunct: Self = Self(4);
    pub const _ISalnum: Self = Self(8);
}
pub type ssize_t = isize;
pub type pid_t = __pid_t;
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
    pub gaps: C2Rust_Unnamed_1,
    pub colors: C2Rust_Unnamed_0,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_hidden: bool,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
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
pub struct C2Rust_Unnamed_1 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct hotspot_event_handling(pub ::core::ffi::c_uint);
impl hotspot_event_handling {
    pub const HOTSPOT_IGNORE: Self = Self(0);
    pub const HOTSPOT_PROCESS: Self = Self(1);
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
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn i3bar_block_unref(mut block: *mut i3bar_block) {
    if block.is_null() {
        return;
    }
    (*block).ref_count -= 1;
    if (*block).ref_count == 0 as ::core::ffi::c_int {
        free((*block).full_text as *mut ::core::ffi::c_void);
        free((*block).short_text as *mut ::core::ffi::c_void);
        free((*block).align as *mut ::core::ffi::c_void);
        free((*block).min_width_str as *mut ::core::ffi::c_void);
        free((*block).name as *mut ::core::ffi::c_void);
        free((*block).instance as *mut ::core::ffi::c_void);
        free(block as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn i3bar_parse_json_color(
    mut json: *mut json_object,
    mut color: *mut uint32_t,
) -> bool {
    if json.is_null() {
        return r#false != 0;
    }
    let mut hexstring: *const ::core::ffi::c_char = json_object_get_string(json);
    let mut color_set: bool = parse_color(hexstring, color);
    if !color_set {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Ignoring invalid block hexadecimal color string: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/i3bar.c\0".as_ptr() as *const ::core::ffi::c_char,
            38 as ::core::ffi::c_int,
            hexstring,
        );
    }
    return color_set;
}
unsafe extern "C" fn i3bar_parse_json(
    mut status: *mut status_line,
    mut json_array: *mut json_object,
) {
    let mut block: *mut i3bar_block = ::core::ptr::null_mut::<i3bar_block>();
    let mut tmp: *mut i3bar_block = ::core::ptr::null_mut::<i3bar_block>();
    block = ((*status).blocks.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut i3bar_block
        as *mut i3bar_block;
    tmp = ((*block).link.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut i3bar_block
        as *mut i3bar_block;
    while &raw mut (*block).link != &raw mut (*status).blocks {
        wl_list_remove(&raw mut (*block).link);
        i3bar_block_unref(block);
        block = tmp;
        tmp = ((*block).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut i3bar_block
            as *mut i3bar_block;
    }
    let mut i: size_t = 0 as size_t;
    while i < json_object_array_length(json_array) {
        let mut full_text: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut short_text: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut color: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut min_width: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut align: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut urgent: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut name: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut instance: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut separator: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut separator_block_width: *mut json_object = ::core::ptr::null_mut::<
            json_object,
        >();
        let mut background: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut border: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut border_top: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut border_bottom: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut border_left: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut border_right: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut markup: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut json: *mut json_object = json_object_array_get_idx(json_array, i);
        if !json.is_null() {
            json_object_object_get_ex(
                json,
                b"full_text\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut full_text,
            );
            json_object_object_get_ex(
                json,
                b"short_text\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut short_text,
            );
            json_object_object_get_ex(
                json,
                b"color\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut color,
            );
            json_object_object_get_ex(
                json,
                b"min_width\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut min_width,
            );
            json_object_object_get_ex(
                json,
                b"align\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut align,
            );
            json_object_object_get_ex(
                json,
                b"urgent\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut urgent,
            );
            json_object_object_get_ex(
                json,
                b"name\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut name,
            );
            json_object_object_get_ex(
                json,
                b"instance\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut instance,
            );
            json_object_object_get_ex(
                json,
                b"markup\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut markup,
            );
            json_object_object_get_ex(
                json,
                b"separator\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut separator,
            );
            json_object_object_get_ex(
                json,
                b"separator_block_width\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut separator_block_width,
            );
            json_object_object_get_ex(
                json,
                b"background\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut background,
            );
            json_object_object_get_ex(
                json,
                b"border\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut border,
            );
            json_object_object_get_ex(
                json,
                b"border_top\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut border_top,
            );
            json_object_object_get_ex(
                json,
                b"border_bottom\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut border_bottom,
            );
            json_object_object_get_ex(
                json,
                b"border_left\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut border_left,
            );
            json_object_object_get_ex(
                json,
                b"border_right\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut border_right,
            );
            let mut block_0: *mut i3bar_block = calloc(
                1 as size_t,
                ::core::mem::size_of::<i3bar_block>(),
            ) as *mut i3bar_block;
            (*block_0).ref_count = 1 as ::core::ffi::c_int;
            (*block_0).full_text = if !full_text.is_null() {
                strdup(json_object_get_string(full_text))
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
            (*block_0).short_text = if !short_text.is_null() {
                strdup(json_object_get_string(short_text))
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
            (*block_0).color_set = i3bar_parse_json_color(
                color,
                &raw mut (*block_0).color,
            );
            if !min_width.is_null() {
                let mut r#type: json_type = json_object_get_type(min_width);
                if r#type.0 == json_type::json_type_int.0 {
                    (*block_0).min_width = json_object_get_int(min_width)
                        as ::core::ffi::c_int;
                } else if r#type.0 == json_type::json_type_string.0 {
                    (*block_0).min_width_str = strdup(json_object_get_string(min_width));
                }
            }
            (*block_0).align = strdup(
                if !align.is_null() {
                    json_object_get_string(align)
                } else {
                    b"left\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            (*block_0).urgent = if !urgent.is_null() {
                json_object_get_int(urgent)
            } else {
                r#false as int32_t
            } != 0;
            (*block_0).name = if !name.is_null() {
                strdup(json_object_get_string(name))
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
            (*block_0).instance = if !instance.is_null() {
                strdup(json_object_get_string(instance))
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
            if !markup.is_null() {
                (*block_0).markup = r#false != 0;
                let mut markup_str: *const ::core::ffi::c_char = json_object_get_string(
                    markup,
                );
                if strcmp(markup_str, b"pango\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    (*block_0).markup = r#true != 0;
                }
            }
            (*block_0).separator = if !separator.is_null() {
                json_object_get_int(separator)
            } else {
                r#true as int32_t
            } != 0;
            (*block_0).separator_block_width = (if !separator_block_width.is_null() {
                json_object_get_int(separator_block_width)
            } else {
                9 as int32_t
            }) as ::core::ffi::c_int;
            i3bar_parse_json_color(background, &raw mut (*block_0).background);
            (*block_0).border_set = i3bar_parse_json_color(
                border,
                &raw mut (*block_0).border,
            );
            (*block_0).border_top = (if !border_top.is_null() {
                json_object_get_int(border_top)
            } else {
                1 as int32_t
            }) as ::core::ffi::c_int;
            (*block_0).border_bottom = (if !border_bottom.is_null() {
                json_object_get_int(border_bottom)
            } else {
                1 as int32_t
            }) as ::core::ffi::c_int;
            (*block_0).border_left = (if !border_left.is_null() {
                json_object_get_int(border_left)
            } else {
                1 as int32_t
            }) as ::core::ffi::c_int;
            (*block_0).border_right = (if !border_right.is_null() {
                json_object_get_int(border_right)
            } else {
                1 as int32_t
            }) as ::core::ffi::c_int;
            wl_list_insert(&raw mut (*status).blocks, &raw mut (*block_0).link);
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn i3bar_handle_readable(mut status: *mut status_line) -> bool {
    while !(*status).started {
        let mut c: size_t = 0 as size_t;
        while c < (*status).buffer_index {
            if *(*status).buffer.offset(c as isize) as ::core::ffi::c_int
                == '[' as ::core::ffi::c_int
            {
                (*status).started = r#true != 0;
                c = c.wrapping_add(1);
                (*status).buffer_index = (*status).buffer_index.wrapping_sub(c);
                memmove(
                    (*status).buffer as *mut ::core::ffi::c_void,
                    (*status).buffer.offset(c as isize) as *const ::core::ffi::c_void,
                    (*status).buffer_index,
                );
                break;
            } else {
                if *(*__ctype_b_loc())
                    .offset(
                        *(*status).buffer.offset(c as isize) as ::core::ffi::c_int
                            as isize,
                    ) as ::core::ffi::c_int
                    & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int
                        as ::core::ffi::c_ushort as ::core::ffi::c_int == 0
                {
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Invalid i3bar json: expected '[' but encountered '%c'\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"swaybar/i3bar.c\0".as_ptr() as *const ::core::ffi::c_char,
                        130 as ::core::ffi::c_int,
                        *(*status).buffer.offset(c as isize) as ::core::ffi::c_int,
                    );
                    status_error(
                        status,
                        b"[invalid i3bar json]\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return r#true != 0;
                }
                c = c.wrapping_add(1);
            }
        }
        if (*status).started {
            break;
        }
        *__errno_location() = 0 as ::core::ffi::c_int;
        let mut read_bytes: ssize_t = read(
            (*status).read_fd,
            (*status).buffer as *mut ::core::ffi::c_void,
            (*status).buffer_size,
        );
        if read_bytes > -1 as ssize_t {
            (*status).buffer_index = read_bytes as size_t;
        } else if *__errno_location() == EAGAIN {
            return r#false != 0
        } else {
            status_error(
                status,
                b"[error reading from status command]\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#true != 0;
        }
    }
    let mut last_object: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut test_object: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut buffer_pos: size_t = 0 as size_t;
    loop {
        if (*status).expecting_comma {
            while buffer_pos < (*status).buffer_index {
                if *(*status).buffer.offset(buffer_pos as isize) as ::core::ffi::c_int
                    == ',' as ::core::ffi::c_int
                {
                    (*status).expecting_comma = r#false != 0;
                    buffer_pos = buffer_pos.wrapping_add(1);
                    break;
                } else {
                    if *(*__ctype_b_loc())
                        .offset(
                            *(*status).buffer.offset(buffer_pos as isize)
                                as ::core::ffi::c_int as isize,
                        ) as ::core::ffi::c_int
                        & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int
                            as ::core::ffi::c_ushort as ::core::ffi::c_int == 0
                    {
                        _sway_log(
                            sway_log_importance_t::SWAY_DEBUG,
                            b"[%s:%d] Invalid i3bar json: expected ',' but encountered '%c'\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"swaybar/i3bar.c\0".as_ptr() as *const ::core::ffi::c_char,
                            170 as ::core::ffi::c_int,
                            *(*status).buffer.offset(buffer_pos as isize)
                                as ::core::ffi::c_int,
                        );
                        status_error(
                            status,
                            b"[invalid i3bar json]\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return r#true != 0;
                    }
                    buffer_pos = buffer_pos.wrapping_add(1);
                }
            }
            if buffer_pos < (*status).buffer_index {
                continue;
            } else {
                (*status).buffer_index = 0 as size_t;
                buffer_pos = (*status).buffer_index;
            }
        } else {
            test_object = json_tokener_parse_ex(
                (*status).tokener,
                (*status).buffer.offset(buffer_pos as isize),
                (*status).buffer_index.wrapping_sub(buffer_pos) as ::core::ffi::c_int,
            );
            let mut err: json_tokener_error = json_tokener_get_error((*status).tokener);
            if err.0 == json_tokener_error::json_tokener_success.0 {
                if json_object_get_type(test_object).0 == json_type::json_type_array.0 {
                    if !last_object.is_null() {
                        json_object_put(last_object);
                    }
                    last_object = test_object;
                } else {
                    json_object_put(test_object);
                }
                let mut last_char_pos: *mut ::core::ffi::c_char = (*status)
                    .buffer
                    .offset(
                        buffer_pos
                            .wrapping_add((*(*status).tokener).char_offset as size_t)
                            .wrapping_sub(1 as size_t) as isize,
                    );
                let mut last_char: ::core::ffi::c_char = *last_char_pos;
                while *(*__ctype_b_loc())
                    .offset(last_char as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int
                        as ::core::ffi::c_ushort as ::core::ffi::c_int != 0
                {
                    last_char_pos = last_char_pos.offset(-1);
                    last_char = *last_char_pos;
                }
                *last_char_pos = '\0' as ::core::ffi::c_char;
                let mut offset: size_t = strspn(
                    (*status).buffer.offset(buffer_pos as isize),
                    b" \x0C\n\r\t\x0B\0".as_ptr() as *const ::core::ffi::c_char,
                ) as size_t;
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Received i3bar json: '%s%c'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaybar/i3bar.c\0".as_ptr() as *const ::core::ffi::c_char,
                    205 as ::core::ffi::c_int,
                    (*status).buffer.offset(buffer_pos.wrapping_add(offset) as isize),
                    last_char as ::core::ffi::c_int,
                );
                *last_char_pos = last_char;
                buffer_pos = buffer_pos
                    .wrapping_add((*(*status).tokener).char_offset as size_t);
                (*status).expecting_comma = r#true != 0;
                if buffer_pos < (*status).buffer_index {
                    continue;
                } else {
                    (*status).buffer_index = 0 as size_t;
                    buffer_pos = (*status).buffer_index;
                }
            } else if err.0 == json_tokener_error::json_tokener_continue.0 {
                json_tokener_reset((*status).tokener);
                if (*status).buffer_index < (*status).buffer_size {
                    (*status).buffer_index = (*status)
                        .buffer_index
                        .wrapping_sub(buffer_pos);
                    memmove(
                        (*status).buffer as *mut ::core::ffi::c_void,
                        (*status).buffer.offset(buffer_pos as isize)
                            as *const ::core::ffi::c_void,
                        (*status).buffer_index,
                    );
                    buffer_pos = 0 as size_t;
                } else {
                    (*status).buffer_size = (*status)
                        .buffer_size
                        .wrapping_mul(2 as size_t);
                    let mut new_buffer: *mut ::core::ffi::c_char = realloc(
                        (*status).buffer as *mut ::core::ffi::c_void,
                        (*status).buffer_size,
                    ) as *mut ::core::ffi::c_char;
                    if !new_buffer.is_null() {
                        (*status).buffer = new_buffer;
                    } else {
                        free((*status).buffer as *mut ::core::ffi::c_void);
                        status_error(
                            status,
                            b"[failed to allocate buffer]\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return r#true != 0;
                    }
                }
            } else {
                let mut last_char_0: ::core::ffi::c_char = *(*status)
                    .buffer
                    .offset((*status).buffer_index.wrapping_sub(1 as size_t) as isize);
                *(*status)
                    .buffer
                    .offset((*status).buffer_index.wrapping_sub(1 as size_t) as isize) = '\0'
                    as ::core::ffi::c_char;
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Failed to parse i3bar json - %s: '%s%c'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaybar/i3bar.c\0".as_ptr() as *const ::core::ffi::c_char,
                    239 as ::core::ffi::c_int,
                    json_tokener_error_desc(err),
                    (*status).buffer.offset(buffer_pos as isize),
                    last_char_0 as ::core::ffi::c_int,
                );
                status_error(
                    status,
                    b"[failed to parse i3bar json]\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return r#true != 0;
            }
        }
        *__errno_location() = 0 as ::core::ffi::c_int;
        let mut read_bytes_0: ssize_t = read(
            (*status).read_fd,
            (*status).buffer.offset((*status).buffer_index as isize)
                as *mut ::core::ffi::c_void,
            (*status).buffer_size.wrapping_sub((*status).buffer_index),
        );
        if read_bytes_0 > -1 as ssize_t {
            (*status).buffer_index = (*status)
                .buffer_index
                .wrapping_add(read_bytes_0 as size_t);
        } else {
            if *__errno_location() == EAGAIN {
                break;
            }
            status_error(
                status,
                b"[error reading from status command]\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#true != 0;
        }
    }
    if !last_object.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Rendering last received json\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/i3bar.c\0".as_ptr() as *const ::core::ffi::c_char,
            260 as ::core::ffi::c_int,
        );
        i3bar_parse_json(status, last_object);
        json_object_put(last_object);
        return r#true != 0;
    } else {
        return r#false != 0
    };
}
#[no_mangle]
pub unsafe extern "C" fn i3bar_block_send_click(
    mut status: *mut status_line,
    mut block: *mut i3bar_block,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut rx: ::core::ffi::c_double,
    mut ry: ::core::ffi::c_double,
    mut w: ::core::ffi::c_double,
    mut h: ::core::ffi::c_double,
    mut scale: ::core::ffi::c_int,
    mut button: uint32_t,
    mut released: bool,
) -> hotspot_event_handling {
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] block %s clicked\0".as_ptr() as *const ::core::ffi::c_char,
        b"swaybar/i3bar.c\0".as_ptr() as *const ::core::ffi::c_char,
        272 as ::core::ffi::c_int,
        (*block).name,
    );
    if (*block).name.is_null() || !(*status).click_events {
        return hotspot_event_handling::HOTSPOT_PROCESS;
    }
    if released {
        return hotspot_event_handling::HOTSPOT_IGNORE;
    }
    let mut event_json: *mut json_object = json_object_new_object();
    json_object_object_add(
        event_json,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*block).name),
    );
    if !(*block).instance.is_null() {
        json_object_object_add(
            event_json,
            b"instance\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*block).instance),
        );
    }
    json_object_object_add(
        event_json,
        b"button\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(event_to_x11_button(button) as int32_t),
    );
    json_object_object_add(
        event_json,
        b"event\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(button as int32_t),
    );
    if (*status).float_event_coords {
        json_object_object_add(
            event_json,
            b"x\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_double(x),
        );
        json_object_object_add(
            event_json,
            b"y\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_double(y),
        );
        json_object_object_add(
            event_json,
            b"relative_x\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_double(rx),
        );
        json_object_object_add(
            event_json,
            b"relative_y\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_double(ry),
        );
        json_object_object_add(
            event_json,
            b"width\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_double(w),
        );
        json_object_object_add(
            event_json,
            b"height\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_double(h),
        );
    } else {
        json_object_object_add(
            event_json,
            b"x\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int(x as int32_t),
        );
        json_object_object_add(
            event_json,
            b"y\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int(y as int32_t),
        );
        json_object_object_add(
            event_json,
            b"relative_x\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int(rx as int32_t),
        );
        json_object_object_add(
            event_json,
            b"relative_y\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int(ry as int32_t),
        );
        json_object_object_add(
            event_json,
            b"width\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int(w as int32_t),
        );
        json_object_object_add(
            event_json,
            b"height\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int(h as int32_t),
        );
    }
    json_object_object_add(
        event_json,
        b"scale\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(scale as int32_t),
    );
    if dprintf(
        (*status).write_fd,
        b"%s%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*status).clicked as ::core::ffi::c_int != 0 {
            b",\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        json_object_to_json_string(event_json),
    ) < 0 as ::core::ffi::c_int
    {
        status_error(
            status,
            b"[failed to write click event]\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    (*status).clicked = r#true != 0;
    json_object_put(event_json);
    return hotspot_event_handling::HOTSPOT_IGNORE;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
