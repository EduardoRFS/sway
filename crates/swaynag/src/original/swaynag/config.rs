use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
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
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn wordexp(
        __words: *const ::core::ffi::c_char,
        __pwordexp: *mut wordexp_t,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn wordfree(__wordexp: *mut wordexp_t);
    fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn pango_font_description_free(desc: *mut PangoFontDescription);
    fn pango_font_description_from_string(
        str: *const ::core::ffi::c_char,
    ) -> *mut PangoFontDescription;
    fn swaynag_type_new(name: *const ::core::ffi::c_char) -> *mut swaynag_type;
    fn swaynag_type_get(
        types: *mut list_t,
        name: *mut ::core::ffi::c_char,
    ) -> *mut swaynag_type;
    fn parse_color(color: *const ::core::ffi::c_char, result: *mut uint32_t) -> bool;
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
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordexp_t {
    pub we_wordc: size_t,
    pub we_wordv: *mut *mut ::core::ffi::c_char,
    pub we_offs: size_t,
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct zwlr_layer_shell_v1_layer(pub ::core::ffi::c_uint);
impl zwlr_layer_shell_v1_layer {
    pub const ZWLR_LAYER_SHELL_V1_LAYER_BACKGROUND: Self = Self(0);
    pub const ZWLR_LAYER_SHELL_V1_LAYER_BOTTOM: Self = Self(1);
    pub const ZWLR_LAYER_SHELL_V1_LAYER_TOP: Self = Self(2);
    pub const ZWLR_LAYER_SHELL_V1_LAYER_OVERLAY: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct zwlr_layer_surface_v1_anchor(pub ::core::ffi::c_uint);
impl zwlr_layer_surface_v1_anchor {
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_TOP: Self = Self(1);
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_BOTTOM: Self = Self(2);
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_LEFT: Self = Self(4);
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_RIGHT: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct type_options(pub ::core::ffi::c_uint);
impl type_options {
    pub const TO_COLOR_BACKGROUND: Self = Self(256);
    pub const TO_COLOR_BORDER: Self = Self(257);
    pub const TO_COLOR_BORDER_BOTTOM: Self = Self(258);
    pub const TO_COLOR_BUTTON: Self = Self(259);
    pub const TO_COLOR_DETAILS: Self = Self(260);
    pub const TO_COLOR_TEXT: Self = Self(261);
    pub const TO_COLOR_BUTTON_TEXT: Self = Self(262);
    pub const TO_THICK_BAR_BORDER: Self = Self(263);
    pub const TO_PADDING_MESSAGE: Self = Self(264);
    pub const TO_THICK_DET_BORDER: Self = Self(265);
    pub const TO_THICK_BTN_BORDER: Self = Self(266);
    pub const TO_GAP_BTN: Self = Self(267);
    pub const TO_GAP_BTN_DISMISS: Self = Self(268);
    pub const TO_MARGIN_BTN_RIGHT: Self = Self(269);
    pub const TO_PADDING_BTN: Self = Self(270);
}
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn read_and_trim_stdin() -> *mut ::core::ffi::c_char {
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut buffer_len: size_t = 0 as size_t;
    let mut line_size: size_t = 0 as size_t;
    '_freebuf: {
        '_freeline: {
            loop {
                let mut nread: ssize_t = getline(
                    &raw mut line,
                    &raw mut line_size,
                    stdin,
                ) as ssize_t;
                if nread == -1 as ssize_t {
                    if feof(stdin) != 0 {
                        break;
                    }
                    perror(b"getline\0".as_ptr() as *const ::core::ffi::c_char);
                    break '_freeline;
                } else {
                    buffer = realloc(
                        buffer as *mut ::core::ffi::c_void,
                        buffer_len
                            .wrapping_add(nread as size_t)
                            .wrapping_add(1 as size_t),
                    ) as *mut ::core::ffi::c_char;
                    if buffer.is_null() {
                        perror(b"realloc\0".as_ptr() as *const ::core::ffi::c_char);
                        break '_freebuf;
                    } else {
                        memcpy(
                            buffer.offset(buffer_len as isize)
                                as *mut ::core::ffi::c_void,
                            line as *const ::core::ffi::c_void,
                            (nread + 1 as ssize_t) as size_t,
                        );
                        buffer_len = buffer_len.wrapping_add(nread as size_t);
                    }
                }
            }
            free(line as *mut ::core::ffi::c_void);
            while buffer_len != 0
                && *buffer.offset(buffer_len.wrapping_sub(1 as size_t) as isize)
                    as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                buffer_len = buffer_len.wrapping_sub(1);
                *buffer.offset(buffer_len as isize) = '\0' as ::core::ffi::c_char;
            }
            return buffer;
        }
        free(line as *mut ::core::ffi::c_void);
    }
    free(buffer as *mut ::core::ffi::c_void);
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn swaynag_parse_options(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut swaynag: *mut swaynag,
    mut types: *mut list_t,
    mut r#type: *mut swaynag_type,
    mut config: *mut *mut ::core::ffi::c_char,
    mut debug: *mut bool,
) -> ::core::ffi::c_int {
    static mut opts: [option; 33] = [
        option {
            name: b"button\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'b' as ::core::ffi::c_int,
        },
        option {
            name: b"button-no-terminal\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'B' as ::core::ffi::c_int,
        },
        option {
            name: b"button-dismiss\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'z' as ::core::ffi::c_int,
        },
        option {
            name: b"button-dismiss-no-terminal\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'Z' as ::core::ffi::c_int,
        },
        option {
            name: b"config\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'c' as ::core::ffi::c_int,
        },
        option {
            name: b"debug\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'd' as ::core::ffi::c_int,
        },
        option {
            name: b"edge\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'e' as ::core::ffi::c_int,
        },
        option {
            name: b"layer\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'y' as ::core::ffi::c_int,
        },
        option {
            name: b"font\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'f' as ::core::ffi::c_int,
        },
        option {
            name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'h' as ::core::ffi::c_int,
        },
        option {
            name: b"detailed-message\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'l' as ::core::ffi::c_int,
        },
        option {
            name: b"detailed-button\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'L' as ::core::ffi::c_int,
        },
        option {
            name: b"message\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'm' as ::core::ffi::c_int,
        },
        option {
            name: b"output\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'o' as ::core::ffi::c_int,
        },
        option {
            name: b"dismiss-button\0".as_ptr() as *const ::core::ffi::c_char,
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
            name: b"background\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_COLOR_BACKGROUND.0 as ::core::ffi::c_int,
        },
        option {
            name: b"border\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_COLOR_BORDER.0 as ::core::ffi::c_int,
        },
        option {
            name: b"border-bottom\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_COLOR_BORDER_BOTTOM.0 as ::core::ffi::c_int,
        },
        option {
            name: b"button-background\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_COLOR_BUTTON.0 as ::core::ffi::c_int,
        },
        option {
            name: b"text\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_COLOR_TEXT.0 as ::core::ffi::c_int,
        },
        option {
            name: b"button-text\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_COLOR_BUTTON_TEXT.0 as ::core::ffi::c_int,
        },
        option {
            name: b"border-bottom-size\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_THICK_BAR_BORDER.0 as ::core::ffi::c_int,
        },
        option {
            name: b"message-padding\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_PADDING_MESSAGE.0 as ::core::ffi::c_int,
        },
        option {
            name: b"details-border-size\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_THICK_DET_BORDER.0 as ::core::ffi::c_int,
        },
        option {
            name: b"details-background\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_COLOR_DETAILS.0 as ::core::ffi::c_int,
        },
        option {
            name: b"button-border-size\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_THICK_BTN_BORDER.0 as ::core::ffi::c_int,
        },
        option {
            name: b"button-gap\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_GAP_BTN.0 as ::core::ffi::c_int,
        },
        option {
            name: b"button-dismiss-gap\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_GAP_BTN_DISMISS.0 as ::core::ffi::c_int,
        },
        option {
            name: b"button-margin-right\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_MARGIN_BTN_RIGHT.0 as ::core::ffi::c_int,
        },
        option {
            name: b"button-padding\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: type_options::TO_PADDING_BTN.0 as ::core::ffi::c_int,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
    ];
    let mut usage: *const ::core::ffi::c_char = b"Usage: swaynag [options...]\n\n  -b, --button <text> <action>  Create a button with text that executes action in a terminal when pressed. Multiple buttons can be defined.\n  -B, --button-no-terminal <text> <action>  Like --button, but doesnot run the action in a terminal.\n  -z, --button-dismiss <text> <action>  Create a button with text that dismisses swaynag, and executes action in a terminal when pressed. Multiple buttons can be defined.\n  -Z, --button-dismiss-no-terminal <text> <action>  Like --button-dismiss, but does not run the action in a terminal.\n  -c, --config <path>             Path to config file.\n  -d, --debug                     Enable debugging.\n  -e, --edge top|bottom           Set the edge to use.\n  -y, --layer overlay|top|bottom|background\n                                  Set the layer to use.\n  -f, --font <font>               Set the font to use.\n  -h, --help                      Show help message and quit.\n  -l, --detailed-message          Read a detailed message from stdin.\n  -L, --detailed-button <text>    Set the text of the detail button.\n  -m, --message <msg>             Set the message text.\n  -o, --output <output>           Set the output to use.\n  -s, --dismiss-button <text>     Set the dismiss button text.\n  -t, --type <type>               Set the message type.\n  -v, --version                   Show the version number and quit.\n\nThe following appearance options can also be given:\n  --background RRGGBB[AA]         Background color.\n  --border RRGGBB[AA]             Border color.\n  --border-bottom RRGGBB[AA]      Bottom border color.\n  --button-background RRGGBB[AA]  Button background color.\n  --text RRGGBB[AA]               Text color.\n  --button-text RRGGBB[AA]        Button text color.\n  --border-bottom-size size       Thickness of the bar border.\n  --message-padding padding       Padding for the message.\n  --details-border-size size      Thickness for the details border.\n  --details-background RRGGBB[AA] Details background color.\n  --button-border-size size       Thickness for the button border.\n  --button-gap gap                Size of the gap between buttons\n  --button-dismiss-gap gap        Size of the gap for dismiss button.\n  --button-margin-right margin    Margin from dismiss button to edge.\n  --button-padding padding        Padding for the button text.\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    optind = 1 as ::core::ffi::c_int;
    loop {
        let mut c: ::core::ffi::c_int = getopt_long(
            argc,
            argv,
            b"b:B:z:Z:c:de:y:f:hlL:m:o:s:t:v\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const opts as *const option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if c == -1 as ::core::ffi::c_int {
            break;
        }
        's_663: {
            'c_48380: {
                match c {
                    66 => {}
                    122 => {}
                    98 | 90 => {
                        break 'c_48380;
                    }
                    99 => {
                        if !config.is_null() {
                            *config = strdup(optarg);
                        }
                        break 's_663;
                    }
                    100 => {
                        if !debug.is_null() {
                            *debug = r#true != 0;
                        }
                        break 's_663;
                    }
                    101 => {
                        if !r#type.is_null() {
                            if strcmp(
                                optarg,
                                b"top\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                            {
                                (*r#type).anchors = (zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_TOP
                                    .0 as ::core::ffi::c_int
                                    | zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_LEFT
                                        .0 as ::core::ffi::c_int
                                    | zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_RIGHT
                                        .0 as ::core::ffi::c_int) as uint32_t;
                            } else if strcmp(
                                optarg,
                                b"bottom\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                            {
                                (*r#type).anchors = (zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_BOTTOM
                                    .0 as ::core::ffi::c_int
                                    | zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_LEFT
                                        .0 as ::core::ffi::c_int
                                    | zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_RIGHT
                                        .0 as ::core::ffi::c_int) as uint32_t;
                            } else {
                                fprintf(
                                    stderr,
                                    b"Invalid edge: %s\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    optarg,
                                );
                                return EXIT_FAILURE;
                            }
                        }
                        break 's_663;
                    }
                    121 => {
                        if !r#type.is_null() {
                            if strcmp(
                                optarg,
                                b"background\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                            {
                                (*r#type).layer = zwlr_layer_shell_v1_layer::ZWLR_LAYER_SHELL_V1_LAYER_BACKGROUND
                                    .0 as ::core::ffi::c_int as int32_t;
                            } else if strcmp(
                                optarg,
                                b"bottom\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                            {
                                (*r#type).layer = zwlr_layer_shell_v1_layer::ZWLR_LAYER_SHELL_V1_LAYER_BOTTOM
                                    .0 as ::core::ffi::c_int as int32_t;
                            } else if strcmp(
                                optarg,
                                b"top\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                            {
                                (*r#type).layer = zwlr_layer_shell_v1_layer::ZWLR_LAYER_SHELL_V1_LAYER_TOP
                                    .0 as ::core::ffi::c_int as int32_t;
                            } else if strcmp(
                                optarg,
                                b"overlay\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                            {
                                (*r#type).layer = zwlr_layer_shell_v1_layer::ZWLR_LAYER_SHELL_V1_LAYER_OVERLAY
                                    .0 as ::core::ffi::c_int as int32_t;
                            } else {
                                fprintf(
                                    stderr,
                                    b"Invalid layer: %s\nUsage: --layer overlay|top|bottom|background\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    optarg,
                                );
                                return EXIT_FAILURE;
                            }
                        }
                        break 's_663;
                    }
                    102 => {
                        if !r#type.is_null() {
                            pango_font_description_free((*r#type).font_description);
                            (*r#type).font_description = pango_font_description_from_string(
                                optarg,
                            );
                        }
                        break 's_663;
                    }
                    108 => {
                        if !swaynag.is_null() {
                            free((*swaynag).details.message as *mut ::core::ffi::c_void);
                            (*swaynag).details.message = read_and_trim_stdin();
                            if (*swaynag).details.message.is_null() {
                                return EXIT_FAILURE;
                            }
                            (*swaynag).details.button_up.text = strdup(
                                b"\xE2\x96\xB2\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            (*swaynag).details.button_down.text = strdup(
                                b"\xE2\x96\xBC\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                        break 's_663;
                    }
                    76 => {
                        if !swaynag.is_null() {
                            free(
                                (*swaynag).details.details_text as *mut ::core::ffi::c_void,
                            );
                            (*swaynag).details.details_text = strdup(optarg);
                        }
                        break 's_663;
                    }
                    109 => {
                        if !swaynag.is_null() {
                            free((*swaynag).message as *mut ::core::ffi::c_void);
                            (*swaynag).message = strdup(optarg);
                        }
                        break 's_663;
                    }
                    111 => {
                        if !r#type.is_null() {
                            free((*r#type).output as *mut ::core::ffi::c_void);
                            (*r#type).output = strdup(optarg);
                        }
                        break 's_663;
                    }
                    115 => {
                        if !swaynag.is_null() {
                            let mut button_close: *mut swaynag_button = *(*(*swaynag)
                                .buttons)
                                .items
                                .offset(0isize) as *mut swaynag_button;
                            free((*button_close).text as *mut ::core::ffi::c_void);
                            (*button_close).text = strdup(optarg);
                        }
                        break 's_663;
                    }
                    116 => {
                        if !swaynag.is_null() {
                            (*swaynag).r#type = swaynag_type_get(types, optarg);
                            if (*swaynag).r#type.is_null() {
                                fprintf(
                                    stderr,
                                    b"Unknown type %s\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    optarg,
                                );
                                return EXIT_FAILURE;
                            }
                        }
                        break 's_663;
                    }
                    118 => {
                        printf(
                            b"swaynag version 1.13-dev\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    256 => {
                        if !r#type.is_null()
                            && !parse_color(optarg, &raw mut (*r#type).background)
                        {
                            fprintf(
                                stderr,
                                b"Invalid background color: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                optarg,
                            );
                        }
                        break 's_663;
                    }
                    257 => {
                        if !r#type.is_null()
                            && !parse_color(optarg, &raw mut (*r#type).border)
                        {
                            fprintf(
                                stderr,
                                b"Invalid border color: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                optarg,
                            );
                        }
                        break 's_663;
                    }
                    258 => {
                        if !r#type.is_null()
                            && !parse_color(optarg, &raw mut (*r#type).border_bottom)
                        {
                            fprintf(
                                stderr,
                                b"Invalid border bottom color: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                optarg,
                            );
                        }
                        break 's_663;
                    }
                    259 => {
                        if !r#type.is_null()
                            && !parse_color(optarg, &raw mut (*r#type).button_background)
                        {
                            fprintf(
                                stderr,
                                b"Invalid button background color: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                optarg,
                            );
                        }
                        break 's_663;
                    }
                    260 => {
                        if !r#type.is_null()
                            && !parse_color(
                                optarg,
                                &raw mut (*r#type).details_background,
                            )
                        {
                            fprintf(
                                stderr,
                                b"Invalid details background color: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                optarg,
                            );
                        }
                        break 's_663;
                    }
                    261 => {
                        if !r#type.is_null()
                            && !parse_color(optarg, &raw mut (*r#type).text)
                        {
                            fprintf(
                                stderr,
                                b"Invalid text color: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                optarg,
                            );
                        }
                        break 's_663;
                    }
                    262 => {
                        if !r#type.is_null()
                            && !parse_color(optarg, &raw mut (*r#type).button_text)
                        {
                            fprintf(
                                stderr,
                                b"Invalid button text color: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                optarg,
                            );
                        }
                        break 's_663;
                    }
                    263 => {
                        if !r#type.is_null() {
                            (*r#type).bar_border_thickness = strtol(
                                optarg,
                                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                0 as ::core::ffi::c_int,
                            ) as ssize_t;
                        }
                        break 's_663;
                    }
                    264 => {
                        if !r#type.is_null() {
                            (*r#type).message_padding = strtol(
                                optarg,
                                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                0 as ::core::ffi::c_int,
                            ) as ssize_t;
                        }
                        break 's_663;
                    }
                    265 => {
                        if !r#type.is_null() {
                            (*r#type).details_border_thickness = strtol(
                                optarg,
                                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                0 as ::core::ffi::c_int,
                            ) as ssize_t;
                        }
                        break 's_663;
                    }
                    266 => {
                        if !r#type.is_null() {
                            (*r#type).button_border_thickness = strtol(
                                optarg,
                                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                0 as ::core::ffi::c_int,
                            ) as ssize_t;
                        }
                        break 's_663;
                    }
                    267 => {
                        if !r#type.is_null() {
                            (*r#type).button_gap = strtol(
                                optarg,
                                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                0 as ::core::ffi::c_int,
                            ) as ssize_t;
                        }
                        break 's_663;
                    }
                    268 => {
                        if !r#type.is_null() {
                            (*r#type).button_gap_close = strtol(
                                optarg,
                                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                0 as ::core::ffi::c_int,
                            ) as ssize_t;
                        }
                        break 's_663;
                    }
                    269 => {
                        if !r#type.is_null() {
                            (*r#type).button_margin_right = strtol(
                                optarg,
                                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                0 as ::core::ffi::c_int,
                            ) as ssize_t;
                        }
                        break 's_663;
                    }
                    270 => {
                        if !r#type.is_null() {
                            (*r#type).button_padding = strtol(
                                optarg,
                                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                0 as ::core::ffi::c_int,
                            ) as ssize_t;
                        }
                        break 's_663;
                    }
                    _ => {
                        fprintf(
                            if c == 'h' as ::core::ffi::c_int { stdout } else { stderr },
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            usage,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                }
            }
            if !swaynag.is_null() {
                if optind >= argc {
                    fprintf(
                        stderr,
                        b"Missing action for button %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        optarg,
                    );
                    return EXIT_FAILURE;
                }
                let mut button: *mut swaynag_button = calloc(
                    1 as size_t,
                    ::core::mem::size_of::<swaynag_button>(),
                ) as *mut swaynag_button;
                if button.is_null() {
                    perror(b"calloc\0".as_ptr() as *const ::core::ffi::c_char);
                    return EXIT_FAILURE;
                }
                (*button).text = strdup(optarg);
                (*button).r#type = swaynag_action_type::SWAYNAG_ACTION_COMMAND;
                (*button).action = strdup(*argv.offset(optind as isize));
                (*button).terminal = c == 'b' as ::core::ffi::c_int;
                (*button).dismiss = c == 'z' as ::core::ffi::c_int
                    || c == 'Z' as ::core::ffi::c_int;
                list_add((*swaynag).buttons, button as *mut ::core::ffi::c_void);
            }
            optind += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn file_exists(mut path: *const ::core::ffi::c_char) -> bool {
    return !path.is_null() && access(path, R_OK) != -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn swaynag_get_config_path() -> *mut ::core::ffi::c_char {
    static mut config_paths: [*const ::core::ffi::c_char; 3] = [
        b"$HOME/.swaynag/config\0".as_ptr() as *const ::core::ffi::c_char,
        b"$XDG_CONFIG_HOME/swaynag/config\0".as_ptr() as *const ::core::ffi::c_char,
        concat!(env!("SWAY_SYSCONFDIR"), "/swaynag/config\0").as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut config_home: *mut ::core::ffi::c_char = getenv(
        b"XDG_CONFIG_HOME\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if config_home.is_null()
        || *config_home.offset(0isize) as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int
    {
        config_paths[1usize] = b"$HOME/.config/swaynag/config\0".as_ptr()
            as *const ::core::ffi::c_char;
    }
    let mut p: wordexp_t = wordexp_t {
        we_wordc: 0,
        we_wordv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        we_offs: 0,
    };
    let mut i: size_t = 0 as size_t;
    while i
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>())
    {
        if wordexp(config_paths[i], &raw mut p, 0 as ::core::ffi::c_int)
            == 0 as ::core::ffi::c_int
        {
            let mut path: *mut ::core::ffi::c_char = strdup(*p.we_wordv.offset(0isize));
            wordfree(&raw mut p);
            if file_exists(path) {
                return path;
            }
            free(path as *mut ::core::ffi::c_void);
        }
        i = i.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn swaynag_load_config(
    mut path: *mut ::core::ffi::c_char,
    mut swaynag: *mut swaynag,
    mut types: *mut list_t,
) -> ::core::ffi::c_int {
    let mut config: *mut FILE = fopen(
        path,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if config.is_null() {
        fprintf(
            stderr,
            b"Failed to read config. Running without it.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut r#type: *mut swaynag_type = swaynag_type_new(
        b"<config>\0".as_ptr() as *const ::core::ffi::c_char,
    );
    list_add(types, r#type as *mut ::core::ffi::c_void);
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut line_size: size_t = 0 as size_t;
    let mut nread: ssize_t = 0;
    let mut line_number: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        nread = getline(&raw mut line, &raw mut line_size, config) as ssize_t;
        if nread == -1 as ssize_t {
            break;
        }
        line_number += 1;
        if *line == 0
            || *line.offset(0isize) as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            || *line.offset(0isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int
        {
            continue;
        }
        if *line.offset(nread - 1 as ssize_t) as ::core::ffi::c_int
            == '\n' as ::core::ffi::c_int
        {
            *line.offset(nread - 1 as ssize_t) = '\0' as ::core::ffi::c_char;
        }
        if *line.offset(0isize) as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
            let mut close: *mut ::core::ffi::c_char = strchr(
                line,
                ']' as ::core::ffi::c_int,
            );
            if close.is_null() || close != line.offset(nread - 2 as ssize_t)
                || nread <= 3 as ssize_t
            {
                fprintf(
                    stderr,
                    b"Line %d is malformed\n\0".as_ptr() as *const ::core::ffi::c_char,
                    line_number,
                );
                result = 1 as ::core::ffi::c_int;
                break;
            } else {
                *close = '\0' as ::core::ffi::c_char;
                r#type = swaynag_type_get(types, line.offset(1isize));
                if r#type.is_null() {
                    r#type = swaynag_type_new(line.offset(1isize));
                    list_add(types, r#type as *mut ::core::ffi::c_void);
                }
            }
        } else {
            let mut flag: *mut ::core::ffi::c_char = malloc(
                (nread + 3 as ssize_t) as size_t,
            ) as *mut ::core::ffi::c_char;
            if flag.is_null() {
                perror(b"calloc\0".as_ptr() as *const ::core::ffi::c_char);
                return EXIT_FAILURE;
            }
            snprintf(
                flag,
                (nread + 3 as ssize_t) as size_t,
                b"--%s\0".as_ptr() as *const ::core::ffi::c_char,
                line,
            );
            let mut argv: [*mut ::core::ffi::c_char; 2] = [
                b"swaynag\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                flag,
            ];
            result = swaynag_parse_options(
                2 as ::core::ffi::c_int,
                &raw mut argv as *mut *mut ::core::ffi::c_char,
                swaynag,
                types,
                r#type,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                ::core::ptr::null_mut::<bool>(),
            );
            free(flag as *mut ::core::ffi::c_void);
            if result != 0 as ::core::ffi::c_int {
                break;
            }
        }
    }
    free(line as *mut ::core::ffi::c_void);
    fclose(config);
    return result;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
