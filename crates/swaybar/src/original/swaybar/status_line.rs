use ::c2rust_bitfields;
extern "C" {
    pub type json_object;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type r#loop;
    pub type wl_display;
    pub type wl_compositor;
    pub type wl_shm;
    pub type _PangoFontDescription;
    pub type swaybar_tray;
    pub type wp_cursor_shape_manager_v1;
    pub type zxdg_output_manager_v1;
    pub type zwlr_layer_shell_v1;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn fcntl(
        __fd: ::core::ffi::c_int,
        __cmd: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn ioctl(
        __fd: ::core::ffi::c_int,
        __request: ::core::ffi::c_ulong,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_object_get_ex(
        obj: *const json_object,
        key: *const ::core::ffi::c_char,
        value: *mut *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_get_boolean(obj: *const json_object) -> json_bool;
    fn json_object_get_int(obj: *const json_object) -> int32_t;
    fn json_tokener_new() -> *mut json_tokener;
    fn json_tokener_free(tok: *mut json_tokener);
    fn json_tokener_parse(str: *const ::core::ffi::c_char) -> *mut json_object;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn fdopen(
        __fd: ::core::ffi::c_int,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn getline(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn clearerr(__stream: *mut FILE);
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execvp(
        __file: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> ::core::ffi::c_int;
    fn fork() -> __pid_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn loop_remove_fd(r#loop: *mut r#loop, fd: ::core::ffi::c_int) -> bool;
    fn wl_list_init(list: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn i3bar_block_unref(block: *mut i3bar_block);
    fn i3bar_handle_readable(status: *mut status_line) -> bool;
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __ssize_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
pub type size_t = usize;
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
pub type ssize_t = isize;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 45] = unsafe {
    ::core::mem::transmute::<
        [u8; 45],
        [::core::ffi::c_char; 45],
    >(*b"struct status_line *status_line_init(char *)\0")
};
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGSTOP: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SIGCONT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn status_line_close_fds(mut status: *mut status_line) {
    if (*status).read_fd != -1 as ::core::ffi::c_int {
        loop_remove_fd((*(*status).bar).eventloop, (*status).read_fd);
        close((*status).read_fd);
        (*status).read_fd = -1 as ::core::ffi::c_int;
    }
    if (*status).write_fd != -1 as ::core::ffi::c_int {
        close((*status).write_fd);
        (*status).write_fd = -1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn status_error(
    mut status: *mut status_line,
    mut text: *const ::core::ffi::c_char,
) {
    status_line_close_fds(status);
    (*status).protocol = status_protocol::PROTOCOL_ERROR;
    (*status).text = text;
}
#[no_mangle]
pub unsafe extern "C" fn status_handle_readable(mut status: *mut status_line) -> bool {
    let mut read_bytes: ssize_t = 1 as ssize_t;
    let mut available_bytes: ::core::ffi::c_int = 0;
    let mut newline: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut header: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut version: *mut json_object = ::core::ptr::null_mut::<json_object>();
    match (*status).protocol {
        status_protocol::PROTOCOL_UNDEF => {
            *__errno_location() = 0 as ::core::ffi::c_int;
            available_bytes = 0;
            if ioctl(
                (*status).read_fd,
                FIONREAD as ::core::ffi::c_ulong,
                &raw mut available_bytes,
            ) == -1 as ::core::ffi::c_int
            {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] Unable to read status command output size\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaybar/status_line.c\0".as_ptr() as *const ::core::ffi::c_char,
                    43 as ::core::ffi::c_int,
                );
                status_error(
                    status,
                    b"[error reading from status command]\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return r#true != 0;
            }
            if (available_bytes as size_t).wrapping_add(1 as size_t)
                > (*status).buffer_size
            {
                (*status).buffer_size = (available_bytes + 1 as ::core::ffi::c_int)
                    as size_t;
                (*status).buffer = realloc(
                    (*status).buffer as *mut ::core::ffi::c_void,
                    (*status).buffer_size,
                ) as *mut ::core::ffi::c_char;
            }
            if (*status).buffer.is_null() {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] Unable to read status line: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaybar/status_line.c\0".as_ptr() as *const ::core::ffi::c_char,
                    54 as ::core::ffi::c_int,
                    strerror(*__errno_location()),
                );
                status_error(
                    status,
                    b"[error reading from status command]\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return r#true != 0;
            }
            read_bytes = read(
                (*status).read_fd,
                (*status).buffer as *mut ::core::ffi::c_void,
                available_bytes as size_t,
            );
            if read_bytes != available_bytes as ssize_t {
                status_error(
                    status,
                    b"[error reading from status command]\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return r#true != 0;
            }
            *(*status).buffer.offset(available_bytes as isize) = 0
                as ::core::ffi::c_char;
            newline = strchr((*status).buffer, '\n' as ::core::ffi::c_int);
            header = ::core::ptr::null_mut::<json_object>();
            version = ::core::ptr::null_mut::<json_object>();
            if !newline.is_null()
                && {
                    header = json_tokener_parse((*status).buffer) as *mut json_object;
                    !header.is_null()
                }
                && json_object_object_get_ex(
                    header,
                    b"version\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut version,
                ) != 0 && json_object_get_int(version) == 1 as int32_t
            {
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Using i3bar protocol.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaybar/status_line.c\0".as_ptr() as *const ::core::ffi::c_char,
                    73 as ::core::ffi::c_int,
                );
                (*status).protocol = status_protocol::PROTOCOL_I3BAR;
                let mut click_events: *mut json_object = ::core::ptr::null_mut::<
                    json_object,
                >();
                if json_object_object_get_ex(
                    header,
                    b"click_events\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut click_events,
                ) != 0 && json_object_get_boolean(click_events) != 0
                {
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Enabling click events.\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"swaybar/status_line.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        79 as ::core::ffi::c_int,
                    );
                    (*status).click_events = r#true != 0;
                    if write(
                        (*status).write_fd,
                        b"[\n\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        2 as size_t,
                    ) != 2 as ssize_t
                    {
                        status_error(
                            status,
                            b"[failed to write to status command]\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        json_object_put(header);
                        return r#true != 0;
                    }
                }
                let mut float_event_coords: *mut json_object = ::core::ptr::null_mut::<
                    json_object,
                >();
                if json_object_object_get_ex(
                    header,
                    b"float_event_coords\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut float_event_coords,
                ) != 0 && json_object_get_boolean(float_event_coords) != 0
                {
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Enabling floating-point coordinates.\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"swaybar/status_line.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        91 as ::core::ffi::c_int,
                    );
                    (*status).float_event_coords = r#true != 0;
                }
                let mut signal: *mut json_object = ::core::ptr::null_mut::<
                    json_object,
                >();
                if json_object_object_get_ex(
                    header,
                    b"stop_signal\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut signal,
                ) != 0
                {
                    (*status).stop_signal = json_object_get_int(signal)
                        as ::core::ffi::c_int;
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Setting stop signal to %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"swaybar/status_line.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        98 as ::core::ffi::c_int,
                        (*status).stop_signal,
                    );
                }
                if json_object_object_get_ex(
                    header,
                    b"cont_signal\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut signal,
                ) != 0
                {
                    (*status).cont_signal = json_object_get_int(signal)
                        as ::core::ffi::c_int;
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Setting cont signal to %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"swaybar/status_line.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        102 as ::core::ffi::c_int,
                        (*status).cont_signal,
                    );
                }
                json_object_put(header);
                wl_list_init(&raw mut (*status).blocks);
                (*status).tokener = json_tokener_new() as *mut json_tokener;
                (*status).buffer_index = strlen(
                    newline.offset(1 as ::core::ffi::c_int as isize),
                );
                memmove(
                    (*status).buffer as *mut ::core::ffi::c_void,
                    newline.offset(1 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    (*status).buffer_index.wrapping_add(1 as size_t),
                );
                return i3bar_handle_readable(status);
            }
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Using text protocol.\0".as_ptr() as *const ::core::ffi::c_char,
                b"swaybar/status_line.c\0".as_ptr() as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_int,
            );
            (*status).protocol = status_protocol::PROTOCOL_TEXT;
            (*status).text = (*status).buffer;
        }
        status_protocol::PROTOCOL_TEXT => {}
        status_protocol::PROTOCOL_I3BAR => return i3bar_handle_readable(status),
        _ => return r#false != 0,
    }
    loop {
        if *(*status).buffer.offset(read_bytes - 1 as ssize_t) as ::core::ffi::c_int
            == '\n' as ::core::ffi::c_int
        {
            *(*status).buffer.offset(read_bytes - 1 as ssize_t) = '\0'
                as ::core::ffi::c_char;
        }
        *__errno_location() = 0 as ::core::ffi::c_int;
        read_bytes = getline(
            &raw mut (*status).buffer,
            &raw mut (*status).buffer_size,
            (*status).read,
        ) as ssize_t;
        if *__errno_location() == EAGAIN {
            clearerr((*status).read);
            return r#true != 0;
        } else if *__errno_location() != 0 {
            status_error(
                status,
                b"[error reading from status command]\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#true != 0;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn status_line_init(
    mut cmd: *mut ::core::ffi::c_char,
) -> *mut status_line {
    let mut status: *mut status_line = calloc(
        1 as size_t,
        ::core::mem::size_of::<status_line>(),
    ) as *mut status_line;
    (*status).stop_signal = SIGSTOP;
    (*status).cont_signal = SIGCONT;
    (*status).buffer_size = 8192 as size_t;
    (*status).buffer = malloc((*status).buffer_size) as *mut ::core::ffi::c_char;
    let mut pipe_read_fd: [::core::ffi::c_int; 2] = [0; 2];
    let mut pipe_write_fd: [::core::ffi::c_int; 2] = [0; 2];
    if pipe(&raw mut pipe_read_fd as *mut ::core::ffi::c_int) != 0 as ::core::ffi::c_int
        || pipe(&raw mut pipe_write_fd as *mut ::core::ffi::c_int)
            != 0 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to create pipes for status_command fork\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/status_line.c\0".as_ptr() as *const ::core::ffi::c_char,
            152 as ::core::ffi::c_int,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if getenv(b"WAYLAND_SOCKET\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        && !(b"display must be initialized before  starting `status-command`; WAYLAND_SOCKET should not be set\0"
            .as_ptr() as *const ::core::ffi::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"!getenv(\"WAYLAND_SOCKET\") && \"display must be initialized before \" \" starting `status-command`; WAYLAND_SOCKET should not be set\"\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/status_line.c\0".as_ptr() as *const ::core::ffi::c_char,
            157 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    (*status).pid = fork() as pid_t;
    if (*status).pid < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] fork failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/status_line.c\0".as_ptr() as *const ::core::ffi::c_char,
            160 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    } else if (*status).pid == 0 as ::core::ffi::c_int {
        setpgid(0 as __pid_t, 0 as __pid_t);
        dup2(pipe_read_fd[1usize], STDOUT_FILENO);
        close(pipe_read_fd[0usize]);
        close(pipe_read_fd[1usize]);
        dup2(pipe_write_fd[0usize], STDIN_FILENO);
        close(pipe_write_fd[0usize]);
        close(pipe_write_fd[1usize]);
        let _cmd: [*mut ::core::ffi::c_char; 4] = [
            b"sh\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"-c\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            cmd,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        execvp(_cmd[0usize], &raw const _cmd as *const *mut ::core::ffi::c_char);
        exit(1 as ::core::ffi::c_int);
    }
    close(pipe_read_fd[1usize]);
    (*status).read_fd = pipe_read_fd[0usize];
    fcntl((*status).read_fd, F_SETFL, O_NONBLOCK);
    close(pipe_write_fd[0usize]);
    (*status).write_fd = pipe_write_fd[1usize];
    fcntl((*status).write_fd, F_SETFL, O_NONBLOCK);
    (*status).read = fdopen(
        (*status).read_fd,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*status).write = fdopen(
        (*status).write_fd,
        b"w\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn status_line_free(mut status: *mut status_line) {
    status_line_close_fds(status);
    kill(-(*status).pid, (*status).cont_signal);
    kill(-(*status).pid, SIGTERM);
    waitpid(
        (*status).pid,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        0 as ::core::ffi::c_int,
    );
    if (*status).protocol.0 == status_protocol::PROTOCOL_I3BAR.0 {
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
        json_tokener_free((*status).tokener);
    }
    free((*status).buffer as *mut ::core::ffi::c_void);
    free(status as *mut ::core::ffi::c_void);
}
pub const FIONREAD: ::core::ffi::c_int = 0x541b as ::core::ffi::c_int;
