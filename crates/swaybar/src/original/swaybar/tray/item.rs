extern "C" {
    pub type _cairo;
    pub type _cairo_surface;
    pub type _cairo_pattern;
    pub type wl_display;
    pub type wl_buffer;
    pub type wl_compositor;
    pub type wl_output;
    pub type wl_shm;
    pub type wl_surface;
    pub type _PangoContext;
    pub type _PangoFontDescription;
    pub type swaybar_watcher;
    pub type sd_bus;
    pub type r#loop;
    pub type status_line;
    pub type wp_cursor_shape_manager_v1;
    pub type zxdg_output_manager_v1;
    pub type zwlr_layer_shell_v1;
    pub type zwlr_layer_surface_v1;
    pub type zxdg_output_v1;
    pub type sd_bus_message;
    pub type sd_bus_slot;
    pub type sd_bus_creds;
    fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    fn cairo_destroy(cr: *mut cairo_t);
    fn cairo_set_operator(cr: *mut cairo_t, op: cairo_operator_t);
    fn cairo_set_source(cr: *mut cairo_t, source: *mut cairo_pattern_t);
    fn cairo_set_line_width(cr: *mut cairo_t, width: ::core::ffi::c_double);
    fn cairo_translate(
        cr: *mut cairo_t,
        tx: ::core::ffi::c_double,
        ty: ::core::ffi::c_double,
    );
    fn cairo_scale(
        cr: *mut cairo_t,
        sx: ::core::ffi::c_double,
        sy: ::core::ffi::c_double,
    );
    fn cairo_arc(
        cr: *mut cairo_t,
        xc: ::core::ffi::c_double,
        yc: ::core::ffi::c_double,
        radius: ::core::ffi::c_double,
        angle1: ::core::ffi::c_double,
        angle2: ::core::ffi::c_double,
    );
    fn cairo_rectangle(
        cr: *mut cairo_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        width: ::core::ffi::c_double,
        height: ::core::ffi::c_double,
    );
    fn cairo_stroke(cr: *mut cairo_t);
    fn cairo_fill(cr: *mut cairo_t);
    fn cairo_get_operator(cr: *mut cairo_t) -> cairo_operator_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_image_surface_create(
        format: cairo_format_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_format_stride_for_width(
        format: cairo_format_t,
        width: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn cairo_image_surface_create_for_data(
        data: *mut ::core::ffi::c_uchar,
        format: cairo_format_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        stride: ::core::ffi::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_image_surface_get_height(
        surface: *mut cairo_surface_t,
    ) -> ::core::ffi::c_int;
    fn cairo_pattern_create_for_surface(
        surface: *mut cairo_surface_t,
    ) -> *mut cairo_pattern_t;
    fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);
    fn cairo_pattern_set_matrix(
        pattern: *mut cairo_pattern_t,
        matrix: *const cairo_matrix_t,
    );
    fn cairo_matrix_init_scale(
        matrix: *mut cairo_matrix_t,
        sx: ::core::ffi::c_double,
        sy: ::core::ffi::c_double,
    );
    fn cairo_matrix_translate(
        matrix: *mut cairo_matrix_t,
        tx: ::core::ffi::c_double,
        ty: ::core::ffi::c_double,
    );
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strndup(
        __string: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn wl_list_init(list: *mut wl_list);
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn create_list() -> *mut list_t;
    fn list_free(list: *mut list_t);
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn list_cat(list: *mut list_t, source: *mut list_t);
    fn list_seq_find(
        list: *mut list_t,
        compare: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        cmp_to: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn list_free_items_and_destroy(list: *mut list_t);
    fn set_bar_dirty(bar: *mut swaybar);
    fn event_to_x11_button(event: uint32_t) -> uint32_t;
    fn find_icon(
        themes: *mut list_t,
        basedirs: *mut list_t,
        name: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        theme: *mut ::core::ffi::c_char,
        min_size: *mut ::core::ffi::c_int,
        max_size: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn sd_bus_slot_unref(p: *mut sd_bus_slot) -> *mut sd_bus_slot;
    fn sd_bus_message_get_error(m: *mut sd_bus_message) -> *const sd_bus_error;
    fn sd_bus_message_get_errno(m: *mut sd_bus_message) -> ::core::ffi::c_int;
    fn sd_bus_message_get_creds(m: *mut sd_bus_message) -> *mut sd_bus_creds;
    fn sd_bus_message_is_method_error(
        m: *mut sd_bus_message,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sd_bus_message_read(
        m: *mut sd_bus_message,
        types: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sd_bus_message_read_array(
        m: *mut sd_bus_message,
        r#type: ::core::ffi::c_char,
        ret_ptr: *mut *const ::core::ffi::c_void,
        ret_size: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn sd_bus_message_enter_container(
        m: *mut sd_bus_message,
        r#type: ::core::ffi::c_char,
        contents: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sd_bus_message_exit_container(m: *mut sd_bus_message) -> ::core::ffi::c_int;
    fn sd_bus_message_at_end(
        m: *mut sd_bus_message,
        complete: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sd_bus_call_method_async(
        bus: *mut sd_bus,
        ret_slot: *mut *mut sd_bus_slot,
        destination: *const ::core::ffi::c_char,
        path: *const ::core::ffi::c_char,
        interface: *const ::core::ffi::c_char,
        member: *const ::core::ffi::c_char,
        callback: sd_bus_message_handler_t,
        userdata: *mut ::core::ffi::c_void,
        types: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sd_bus_match_signal_async(
        bus: *mut sd_bus,
        ret: *mut *mut sd_bus_slot,
        sender: *const ::core::ffi::c_char,
        path: *const ::core::ffi::c_char,
        interface: *const ::core::ffi::c_char,
        member: *const ::core::ffi::c_char,
        match_callback: sd_bus_message_handler_t,
        install_callback: sd_bus_message_handler_t,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sd_bus_creds_get_mask(c: *const sd_bus_creds) -> uint64_t;
    fn cairo_set_source_u32(cairo: *mut cairo_t, color: uint32_t);
    fn cairo_image_surface_scale(
        image: *mut cairo_surface_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> *mut cairo_surface_t;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn load_image(path: *const ::core::ffi::c_char) -> *mut cairo_surface_t;
    fn has_prefix(
        str: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    ) -> bool;
}
pub type __uint32_t = u32;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;
pub type int32_t = i32;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_matrix {
    pub xx: ::core::ffi::c_double,
    pub yx: ::core::ffi::c_double,
    pub xy: ::core::ffi::c_double,
    pub yy: ::core::ffi::c_double,
    pub x0: ::core::ffi::c_double,
    pub y0: ::core::ffi::c_double,
}
pub type cairo_matrix_t = _cairo_matrix;
pub type cairo_pattern_t = _cairo_pattern;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct _cairo_format(pub ::core::ffi::c_int);
impl _cairo_format {
    pub const CAIRO_FORMAT_INVALID: Self = Self(-1);
    pub const CAIRO_FORMAT_ARGB32: Self = Self(0);
    pub const CAIRO_FORMAT_RGB24: Self = Self(1);
    pub const CAIRO_FORMAT_A8: Self = Self(2);
    pub const CAIRO_FORMAT_A1: Self = Self(3);
    pub const CAIRO_FORMAT_RGB16_565: Self = Self(4);
    pub const CAIRO_FORMAT_RGB30: Self = Self(5);
    pub const CAIRO_FORMAT_RGB96F: Self = Self(6);
    pub const CAIRO_FORMAT_RGBA128F: Self = Self(7);
}
pub type cairo_format_t = _cairo_format;
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
pub type PangoContext = _PangoContext;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct zwlr_layer_surface_v1_anchor(pub ::core::ffi::c_uint);
impl zwlr_layer_surface_v1_anchor {
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_TOP: Self = Self(1);
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_BOTTOM: Self = Self(2);
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_LEFT: Self = Self(4);
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_RIGHT: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sd_bus_error {
    pub name: *const ::core::ffi::c_char,
    pub message: *const ::core::ffi::c_char,
    pub _need_free: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_ulong);
impl C2Rust_Unnamed_1 {
    pub const SD_BUS_CREDS_PID: Self = Self(1);
    pub const SD_BUS_CREDS_TID: Self = Self(2);
    pub const SD_BUS_CREDS_PPID: Self = Self(4);
    pub const SD_BUS_CREDS_UID: Self = Self(8);
    pub const SD_BUS_CREDS_EUID: Self = Self(16);
    pub const SD_BUS_CREDS_SUID: Self = Self(32);
    pub const SD_BUS_CREDS_FSUID: Self = Self(64);
    pub const SD_BUS_CREDS_GID: Self = Self(128);
    pub const SD_BUS_CREDS_EGID: Self = Self(256);
    pub const SD_BUS_CREDS_SGID: Self = Self(512);
    pub const SD_BUS_CREDS_FSGID: Self = Self(1024);
    pub const SD_BUS_CREDS_SUPPLEMENTARY_GIDS: Self = Self(2048);
    pub const SD_BUS_CREDS_COMM: Self = Self(4096);
    pub const SD_BUS_CREDS_TID_COMM: Self = Self(8192);
    pub const SD_BUS_CREDS_EXE: Self = Self(16384);
    pub const SD_BUS_CREDS_CMDLINE: Self = Self(32768);
    pub const SD_BUS_CREDS_CGROUP: Self = Self(65536);
    pub const SD_BUS_CREDS_UNIT: Self = Self(131072);
    pub const SD_BUS_CREDS_SLICE: Self = Self(262144);
    pub const SD_BUS_CREDS_USER_UNIT: Self = Self(524288);
    pub const SD_BUS_CREDS_USER_SLICE: Self = Self(1048576);
    pub const SD_BUS_CREDS_SESSION: Self = Self(2097152);
    pub const SD_BUS_CREDS_OWNER_UID: Self = Self(4194304);
    pub const SD_BUS_CREDS_EFFECTIVE_CAPS: Self = Self(8388608);
    pub const SD_BUS_CREDS_PERMITTED_CAPS: Self = Self(16777216);
    pub const SD_BUS_CREDS_INHERITABLE_CAPS: Self = Self(33554432);
    pub const SD_BUS_CREDS_BOUNDING_CAPS: Self = Self(67108864);
    pub const SD_BUS_CREDS_SELINUX_CONTEXT: Self = Self(134217728);
    pub const SD_BUS_CREDS_AUDIT_SESSION_ID: Self = Self(268435456);
    pub const SD_BUS_CREDS_AUDIT_LOGIN_UID: Self = Self(536870912);
    pub const SD_BUS_CREDS_TTY: Self = Self(1073741824);
    pub const SD_BUS_CREDS_UNIQUE_NAME: Self = Self(2147483648);
    pub const SD_BUS_CREDS_WELL_KNOWN_NAMES: Self = Self(4294967296);
    pub const SD_BUS_CREDS_DESCRIPTION: Self = Self(8589934592);
    pub const SD_BUS_CREDS_PIDFD: Self = Self(17179869184);
    pub const SD_BUS_CREDS_AUGMENT: Self = Self(9223372036854775808);
    pub const _SD_BUS_CREDS_ALL: Self = Self(34359738367);
}
pub type sd_bus_message_handler_t = Option<
    unsafe extern "C" fn(
        *mut sd_bus_message,
        *mut ::core::ffi::c_void,
        *mut sd_bus_error,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_pixmap {
    pub size: ::core::ffi::c_int,
    pub pixels: [::core::ffi::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_sni_slot {
    pub link: wl_list,
    pub sni: *mut swaybar_sni,
    pub prop: *const ::core::ffi::c_char,
    pub r#type: *const ::core::ffi::c_char,
    pub dest: *mut ::core::ffi::c_void,
    pub slot: *mut sd_bus_slot,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_sni {
    pub tray: *mut swaybar_tray,
    pub icon: *mut cairo_surface_t,
    pub min_size: ::core::ffi::c_int,
    pub max_size: ::core::ffi::c_int,
    pub target_size: ::core::ffi::c_int,
    pub watcher_id: *mut ::core::ffi::c_char,
    pub service: *mut ::core::ffi::c_char,
    pub path: *mut ::core::ffi::c_char,
    pub interface: *mut ::core::ffi::c_char,
    pub status: *mut ::core::ffi::c_char,
    pub icon_name: *mut ::core::ffi::c_char,
    pub icon_pixmap: *mut list_t,
    pub attention_icon_name: *mut ::core::ffi::c_char,
    pub attention_icon_pixmap: *mut list_t,
    pub item_is_menu: bool,
    pub menu: *mut ::core::ffi::c_char,
    pub icon_theme_path: *mut ::core::ffi::c_char,
    pub slots: wl_list,
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
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as __uint32_t) >> 24 as ::core::ffi::c_int
        | (__bsx & 0xff0000 as __uint32_t) >> 8 as ::core::ffi::c_int
        | (__bsx & 0xff00 as __uint32_t) << 8 as ::core::ffi::c_int
        | (__bsx & 0xff as __uint32_t) << 24 as ::core::ffi::c_int;
}
pub const SD_BUS_ERROR_UNKNOWN_PROPERTY: [::core::ffi::c_char; 43] = unsafe {
    ::core::mem::transmute::<
        [u8; 43],
        [::core::ffi::c_char; 43],
    >(*b"org.freedesktop.DBus.Error.UnknownProperty\0")
};
unsafe extern "C" fn sni_ready(mut sni: *mut swaybar_sni) -> bool {
    return !(*sni).status.is_null()
        && if *(*sni).status.offset(0isize) as ::core::ffi::c_int
            == 'N' as ::core::ffi::c_int
        {
            (!(*sni).attention_icon_name.is_null()
                || !(*sni).attention_icon_pixmap.is_null()) as ::core::ffi::c_int
        } else {
            (!(*sni).icon_name.is_null() || !(*sni).icon_pixmap.is_null())
                as ::core::ffi::c_int
        } != 0;
}
unsafe extern "C" fn set_sni_dirty(mut sni: *mut swaybar_sni) {
    if sni_ready(sni) {
        (*sni).max_size = 0 as ::core::ffi::c_int;
        (*sni).min_size = (*sni).max_size;
        (*sni).target_size = (*sni).min_size;
        set_bar_dirty((*(*sni).tray).bar);
    }
}
unsafe extern "C" fn read_pixmap(
    mut msg: *mut sd_bus_message,
    mut sni: *mut swaybar_sni,
    mut prop: *const ::core::ffi::c_char,
    mut dest: *mut *mut list_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = sd_bus_message_enter_container(
        msg,
        'a' as ::core::ffi::c_char,
        b"(iiay)\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] %s %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
            40 as ::core::ffi::c_int,
            (*sni).watcher_id,
            prop,
            strerror(-ret),
        );
        return ret;
    }
    if sd_bus_message_at_end(msg, 0 as ::core::ffi::c_int) != 0 {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] %s %s no. of icons = 0\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
            45 as ::core::ffi::c_int,
            (*sni).watcher_id,
            prop,
        );
        return ret;
    }
    let mut pixmaps: *mut list_t = create_list();
    if pixmaps.is_null() {
        return -12 as ::core::ffi::c_int;
    }
    '_error: {
        while sd_bus_message_at_end(msg, 0 as ::core::ffi::c_int) == 0 {
            ret = sd_bus_message_enter_container(
                msg,
                'r' as ::core::ffi::c_char,
                b"iiay\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if ret < 0 as ::core::ffi::c_int {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] %s %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
                    57 as ::core::ffi::c_int,
                    (*sni).watcher_id,
                    prop,
                    strerror(-ret),
                );
                break '_error;
            } else {
                let mut width: ::core::ffi::c_int = 0;
                let mut height: ::core::ffi::c_int = 0;
                ret = sd_bus_message_read(
                    msg,
                    b"ii\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut width,
                    &raw mut height,
                );
                if ret < 0 as ::core::ffi::c_int {
                    _sway_log(
                        sway_log_importance_t::SWAY_ERROR,
                        b"[%s:%d] %s %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
                        64 as ::core::ffi::c_int,
                        (*sni).watcher_id,
                        prop,
                        strerror(-ret),
                    );
                    break '_error;
                } else {
                    let mut pixels: *const ::core::ffi::c_void = ::core::ptr::null::<
                        ::core::ffi::c_void,
                    >();
                    let mut pixel_data_size: size_t = 0;
                    ret = sd_bus_message_read_array(
                        msg,
                        'y' as ::core::ffi::c_char,
                        &raw mut pixels,
                        &raw mut pixel_data_size,
                    );
                    if ret < 0 as ::core::ffi::c_int {
                        _sway_log(
                            sway_log_importance_t::SWAY_ERROR,
                            b"[%s:%d] %s %s: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"swaybar/tray/item.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            72 as ::core::ffi::c_int,
                            (*sni).watcher_id,
                            prop,
                            strerror(-ret),
                        );
                        break '_error;
                    } else {
                        if height > 0 as ::core::ffi::c_int && width == height
                            && (width as size_t).wrapping_mul(height as size_t)
                                <= pixel_data_size.wrapping_div(4 as size_t)
                        {
                            _sway_log(
                                sway_log_importance_t::SWAY_DEBUG,
                                b"[%s:%d] %s %s: found icon w:%d h:%d\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"swaybar/tray/item.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                77 as ::core::ffi::c_int,
                                (*sni).watcher_id,
                                prop,
                                width,
                                height,
                            );
                            let mut pixmap: *mut swaybar_pixmap = malloc(
                                ::core::mem::size_of::<swaybar_pixmap>()
                                    .wrapping_add(pixel_data_size),
                            ) as *mut swaybar_pixmap;
                            (*pixmap).size = height;
                            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i < height * width {
                                *(&raw mut (*pixmap).pixels as *mut ::core::ffi::c_uchar
                                    as *mut uint32_t)
                                    .offset(i as isize) = __bswap_32(
                                    *(pixels as *mut uint32_t).offset(i as isize),
                                ) as uint32_t;
                                i += 1;
                            }
                            list_add(pixmaps, pixmap as *mut ::core::ffi::c_void);
                        } else {
                            _sway_log(
                                sway_log_importance_t::SWAY_DEBUG,
                                b"[%s:%d] %s %s: discard invalid icon w:%d h:%d\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"swaybar/tray/item.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                89 as ::core::ffi::c_int,
                                (*sni).watcher_id,
                                prop,
                                width,
                                height,
                            );
                        }
                        sd_bus_message_exit_container(msg);
                    }
                }
            }
        }
        if (*pixmaps).length < 1 as ::core::ffi::c_int {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] %s %s no. of icons = 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int,
                (*sni).watcher_id,
                prop,
            );
        } else {
            list_free_items_and_destroy(*dest);
            *dest = pixmaps;
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] %s %s no. of icons = %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_int,
                (*sni).watcher_id,
                prop,
                (*pixmaps).length,
            );
            return ret;
        }
    }
    list_free_items_and_destroy(pixmaps);
    return ret;
}
unsafe extern "C" fn get_property_callback(
    mut msg: *mut sd_bus_message,
    mut data: *mut ::core::ffi::c_void,
    mut error: *mut sd_bus_error,
) -> ::core::ffi::c_int {
    let mut d: *mut swaybar_sni_slot = data as *mut swaybar_sni_slot;
    let mut sni: *mut swaybar_sni = (*d).sni;
    let mut prop: *const ::core::ffi::c_char = (*d).prop;
    let mut r#type: *const ::core::ffi::c_char = (*d).r#type;
    let mut dest: *mut ::core::ffi::c_void = (*d).dest;
    let mut ret: ::core::ffi::c_int = 0;
    '_cleanup: {
        if sd_bus_message_is_method_error(
            msg,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) != 0
        {
            let mut err: *const sd_bus_error = sd_bus_message_get_error(msg);
            let mut log_lv: sway_log_importance_t = sway_log_importance_t::SWAY_ERROR;
            if strcmp(prop, b"IconThemePath\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 && strcmp((*err).name, SD_BUS_ERROR_UNKNOWN_PROPERTY.as_ptr()) == 0
            {
                log_lv = sway_log_importance_t::SWAY_DEBUG;
            }
            _sway_log(
                log_lv,
                b"[%s:%d] %s %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
                127 as ::core::ffi::c_int,
                (*sni).watcher_id,
                prop,
                (*err).message,
            );
            ret = sd_bus_message_get_errno(msg);
        } else {
            ret = sd_bus_message_enter_container(
                msg,
                'v' as ::core::ffi::c_char,
                r#type,
            );
            if ret < 0 as ::core::ffi::c_int {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] %s %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
                    134 as ::core::ffi::c_int,
                    (*sni).watcher_id,
                    prop,
                    strerror(-ret),
                );
            } else {
                if r#type.is_null() {
                    ret = read_pixmap(msg, sni, prop, dest as *mut *mut list_t);
                    if ret < 0 as ::core::ffi::c_int {
                        break '_cleanup;
                    }
                } else {
                    if *r#type as ::core::ffi::c_int == 's' as ::core::ffi::c_int
                        || *r#type as ::core::ffi::c_int == 'o' as ::core::ffi::c_int
                    {
                        free(
                            *(dest as *mut *mut ::core::ffi::c_char)
                                as *mut ::core::ffi::c_void,
                        );
                    }
                    ret = sd_bus_message_read(msg, r#type, dest);
                    if ret < 0 as ::core::ffi::c_int {
                        _sway_log(
                            sway_log_importance_t::SWAY_ERROR,
                            b"[%s:%d] %s %s: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"swaybar/tray/item.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            150 as ::core::ffi::c_int,
                            (*sni).watcher_id,
                            prop,
                            strerror(-ret),
                        );
                        break '_cleanup;
                    } else if *r#type as ::core::ffi::c_int == 's' as ::core::ffi::c_int
                        || *r#type as ::core::ffi::c_int == 'o' as ::core::ffi::c_int
                    {
                        let mut str: *mut *mut ::core::ffi::c_char = dest
                            as *mut *mut ::core::ffi::c_char;
                        *str = strdup(*str);
                        _sway_log(
                            sway_log_importance_t::SWAY_DEBUG,
                            b"[%s:%d] %s %s = '%s'\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"swaybar/tray/item.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            157 as ::core::ffi::c_int,
                            (*sni).watcher_id,
                            prop,
                            *str,
                        );
                    } else if *r#type as ::core::ffi::c_int == 'b' as ::core::ffi::c_int
                    {
                        _sway_log(
                            sway_log_importance_t::SWAY_DEBUG,
                            b"[%s:%d] %s %s = %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"swaybar/tray/item.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            159 as ::core::ffi::c_int,
                            (*sni).watcher_id,
                            prop,
                            if *(dest as *mut bool) as ::core::ffi::c_int != 0 {
                                b"true\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"false\0".as_ptr() as *const ::core::ffi::c_char
                            },
                        );
                    }
                }
                if strcmp(prop, b"Status\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || !(*sni).status.is_null()
                        && if *(*sni).status.offset(0isize) as ::core::ffi::c_int
                            == 'N' as ::core::ffi::c_int
                        {
                            (*prop.offset(0isize) as ::core::ffi::c_int
                                == 'A' as ::core::ffi::c_int) as ::core::ffi::c_int
                        } else {
                            has_prefix(
                                prop,
                                b"Icon\0".as_ptr() as *const ::core::ffi::c_char,
                            ) as ::core::ffi::c_int
                        } != 0
                {
                    set_sni_dirty(sni);
                }
            }
        }
    }
    wl_list_remove(&raw mut (*d).link);
    free(data);
    return ret;
}
unsafe extern "C" fn sni_get_property_async(
    mut sni: *mut swaybar_sni,
    mut prop: *const ::core::ffi::c_char,
    mut r#type: *const ::core::ffi::c_char,
    mut dest: *mut ::core::ffi::c_void,
) {
    let mut data: *mut swaybar_sni_slot = calloc(
        1 as size_t,
        ::core::mem::size_of::<swaybar_sni_slot>(),
    ) as *mut swaybar_sni_slot;
    (*data).sni = sni as *mut swaybar_sni;
    (*data).prop = prop;
    (*data).r#type = r#type;
    (*data).dest = dest;
    let mut ret: ::core::ffi::c_int = sd_bus_call_method_async(
        (*(*sni).tray).bus,
        &raw mut (*data).slot,
        (*sni).service,
        (*sni).path,
        b"org.freedesktop.DBus.Properties\0".as_ptr() as *const ::core::ffi::c_char,
        b"Get\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            get_property_callback
                as unsafe extern "C" fn(
                    *mut sd_bus_message,
                    *mut ::core::ffi::c_void,
                    *mut sd_bus_error,
                ) -> ::core::ffi::c_int,
        ),
        data as *mut ::core::ffi::c_void,
        b"ss\0".as_ptr() as *const ::core::ffi::c_char,
        (*sni).interface,
        prop,
    );
    if ret >= 0 as ::core::ffi::c_int {
        wl_list_insert(&raw mut (*sni).slots, &raw mut (*data).link);
    } else {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] %s %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
            187 as ::core::ffi::c_int,
            (*sni).watcher_id,
            prop,
            strerror(-ret),
        );
        free(data as *mut ::core::ffi::c_void);
    };
}
unsafe extern "C" fn sni_check_msg_sender(
    mut sni: *mut swaybar_sni,
    mut msg: *mut sd_bus_message,
    mut signal: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut has_well_known_names: bool = sd_bus_creds_get_mask(
        sd_bus_message_get_creds(msg),
    ) & C2Rust_Unnamed_1::SD_BUS_CREDS_WELL_KNOWN_NAMES.0 as uint64_t != 0;
    if *(*sni).service.offset(0isize) as ::core::ffi::c_int == ':' as ::core::ffi::c_int
        || has_well_known_names as ::core::ffi::c_int != 0
    {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] %s has new %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
            211 as ::core::ffi::c_int,
            (*sni).watcher_id,
            signal,
        );
        return 1 as ::core::ffi::c_int;
    } else {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] %s may have new %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
            214 as ::core::ffi::c_int,
            (*sni).watcher_id,
            signal,
        );
        return 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn handle_new_icon(
    mut msg: *mut sd_bus_message,
    mut data: *mut ::core::ffi::c_void,
    mut error: *mut sd_bus_error,
) -> ::core::ffi::c_int {
    let mut sni: *mut swaybar_sni = data as *mut swaybar_sni;
    sni_get_property_async(
        sni,
        b"IconName\0".as_ptr() as *const ::core::ffi::c_char,
        b"s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sni).icon_name as *mut ::core::ffi::c_void,
    );
    sni_get_property_async(
        sni,
        b"IconPixmap\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut (*sni).icon_pixmap as *mut ::core::ffi::c_void,
    );
    if strcmp(
        (*sni).interface,
        b"org.kde.StatusNotifierItem\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        sni_get_property_async(
            sni,
            b"IconThemePath\0".as_ptr() as *const ::core::ffi::c_char,
            b"s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*sni).icon_theme_path as *mut ::core::ffi::c_void,
        );
    }
    return sni_check_msg_sender(
        sni,
        msg,
        b"icon\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn handle_new_attention_icon(
    mut msg: *mut sd_bus_message,
    mut data: *mut ::core::ffi::c_void,
    mut error: *mut sd_bus_error,
) -> ::core::ffi::c_int {
    let mut sni: *mut swaybar_sni = data as *mut swaybar_sni;
    sni_get_property_async(
        sni,
        b"AttentionIconName\0".as_ptr() as *const ::core::ffi::c_char,
        b"s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sni).attention_icon_name as *mut ::core::ffi::c_void,
    );
    sni_get_property_async(
        sni,
        b"AttentionIconPixmap\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut (*sni).attention_icon_pixmap as *mut ::core::ffi::c_void,
    );
    return sni_check_msg_sender(
        sni,
        msg,
        b"attention icon\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn handle_new_status(
    mut msg: *mut sd_bus_message,
    mut data: *mut ::core::ffi::c_void,
    mut error: *mut sd_bus_error,
) -> ::core::ffi::c_int {
    let mut sni: *mut swaybar_sni = data as *mut swaybar_sni;
    let mut ret: ::core::ffi::c_int = sni_check_msg_sender(
        sni,
        msg,
        b"status\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if ret == 1 as ::core::ffi::c_int {
        let mut status: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut r: ::core::ffi::c_int = sd_bus_message_read(
            msg,
            b"s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut status,
        );
        if r < 0 as ::core::ffi::c_int {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] %s new status error: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
                244 as ::core::ffi::c_int,
                (*sni).watcher_id,
                strerror(-ret),
            );
            ret = r;
        } else {
            free((*sni).status as *mut ::core::ffi::c_void);
            (*sni).status = strdup(status);
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] %s has new status = '%s'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
                249 as ::core::ffi::c_int,
                (*sni).watcher_id,
                status,
            );
            set_sni_dirty(sni);
        }
    } else {
        sni_get_property_async(
            sni,
            b"Status\0".as_ptr() as *const ::core::ffi::c_char,
            b"s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*sni).status as *mut ::core::ffi::c_void,
        );
    }
    return ret;
}
unsafe extern "C" fn sni_match_signal_async(
    mut sni: *mut swaybar_sni,
    mut signal: *mut ::core::ffi::c_char,
    mut callback: sd_bus_message_handler_t,
) {
    let mut slot: *mut swaybar_sni_slot = calloc(
        1 as size_t,
        ::core::mem::size_of::<swaybar_sni_slot>(),
    ) as *mut swaybar_sni_slot;
    let mut ret: ::core::ffi::c_int = sd_bus_match_signal_async(
        (*(*sni).tray).bus,
        &raw mut (*slot).slot,
        (*sni).service,
        (*sni).path,
        (*sni).interface,
        signal,
        callback,
        None,
        sni as *mut ::core::ffi::c_void,
    );
    if ret >= 0 as ::core::ffi::c_int {
        wl_list_insert(&raw mut (*sni).slots, &raw mut (*slot).link);
    } else {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] %s failed to subscribe to signal %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
            267 as ::core::ffi::c_int,
            (*sni).service,
            signal,
            strerror(-ret),
        );
        free(slot as *mut ::core::ffi::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn create_sni(
    mut id: *mut ::core::ffi::c_char,
    mut tray: *mut swaybar_tray,
) -> *mut swaybar_sni {
    let mut sni: *mut swaybar_sni = calloc(
        1 as size_t,
        ::core::mem::size_of::<swaybar_sni>(),
    ) as *mut swaybar_sni;
    if sni.is_null() {
        return ::core::ptr::null_mut::<swaybar_sni>();
    }
    (*sni).tray = tray;
    wl_list_init(&raw mut (*sni).slots);
    (*sni).watcher_id = strdup(id);
    let mut path_ptr: *mut ::core::ffi::c_char = strchr(id, '/' as ::core::ffi::c_int);
    if path_ptr.is_null() {
        (*sni).service = strdup(id);
        (*sni).path = strdup(
            b"/StatusNotifierItem\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*sni).interface = b"org.freedesktop.StatusNotifierItem\0".as_ptr()
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    } else {
        (*sni).service = strndup(id, path_ptr.offset_from(id) as size_t);
        (*sni).path = strdup(path_ptr);
        (*sni).interface = b"org.kde.StatusNotifierItem\0".as_ptr()
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        sni_get_property_async(
            sni,
            b"IconThemePath\0".as_ptr() as *const ::core::ffi::c_char,
            b"s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*sni).icon_theme_path as *mut ::core::ffi::c_void,
        );
    }
    sni_get_property_async(
        sni,
        b"Status\0".as_ptr() as *const ::core::ffi::c_char,
        b"s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sni).status as *mut ::core::ffi::c_void,
    );
    sni_get_property_async(
        sni,
        b"IconName\0".as_ptr() as *const ::core::ffi::c_char,
        b"s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sni).icon_name as *mut ::core::ffi::c_void,
    );
    sni_get_property_async(
        sni,
        b"IconPixmap\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut (*sni).icon_pixmap as *mut ::core::ffi::c_void,
    );
    sni_get_property_async(
        sni,
        b"AttentionIconName\0".as_ptr() as *const ::core::ffi::c_char,
        b"s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sni).attention_icon_name as *mut ::core::ffi::c_void,
    );
    sni_get_property_async(
        sni,
        b"AttentionIconPixmap\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut (*sni).attention_icon_pixmap as *mut ::core::ffi::c_void,
    );
    sni_get_property_async(
        sni,
        b"ItemIsMenu\0".as_ptr() as *const ::core::ffi::c_char,
        b"b\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sni).item_is_menu as *mut ::core::ffi::c_void,
    );
    sni_get_property_async(
        sni,
        b"Menu\0".as_ptr() as *const ::core::ffi::c_char,
        b"o\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sni).menu as *mut ::core::ffi::c_void,
    );
    sni_match_signal_async(
        sni,
        b"NewIcon\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        Some(
            handle_new_icon
                as unsafe extern "C" fn(
                    *mut sd_bus_message,
                    *mut ::core::ffi::c_void,
                    *mut sd_bus_error,
                ) -> ::core::ffi::c_int,
        ),
    );
    sni_match_signal_async(
        sni,
        b"NewAttentionIcon\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        Some(
            handle_new_attention_icon
                as unsafe extern "C" fn(
                    *mut sd_bus_message,
                    *mut ::core::ffi::c_void,
                    *mut sd_bus_error,
                ) -> ::core::ffi::c_int,
        ),
    );
    sni_match_signal_async(
        sni,
        b"NewStatus\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        Some(
            handle_new_status
                as unsafe extern "C" fn(
                    *mut sd_bus_message,
                    *mut ::core::ffi::c_void,
                    *mut sd_bus_error,
                ) -> ::core::ffi::c_int,
        ),
    );
    return sni;
}
#[no_mangle]
pub unsafe extern "C" fn destroy_sni(mut sni: *mut swaybar_sni) {
    if sni.is_null() {
        return;
    }
    cairo_surface_destroy((*sni).icon);
    free((*sni).watcher_id as *mut ::core::ffi::c_void);
    free((*sni).service as *mut ::core::ffi::c_void);
    free((*sni).path as *mut ::core::ffi::c_void);
    free((*sni).status as *mut ::core::ffi::c_void);
    free((*sni).icon_name as *mut ::core::ffi::c_void);
    list_free_items_and_destroy((*sni).icon_pixmap);
    free((*sni).attention_icon_name as *mut ::core::ffi::c_void);
    list_free_items_and_destroy((*sni).attention_icon_pixmap);
    free((*sni).menu as *mut ::core::ffi::c_void);
    free((*sni).icon_theme_path as *mut ::core::ffi::c_void);
    let mut slot: *mut swaybar_sni_slot = ::core::ptr::null_mut::<swaybar_sni_slot>();
    let mut slot_tmp: *mut swaybar_sni_slot = ::core::ptr::null_mut::<
        swaybar_sni_slot,
    >();
    slot = ((*sni).slots.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_sni_slot
        as *mut swaybar_sni_slot;
    slot_tmp = ((*slot).link.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_sni_slot
        as *mut swaybar_sni_slot;
    while &raw mut (*slot).link != &raw mut (*sni).slots {
        sd_bus_slot_unref((*slot).slot);
        free(slot as *mut ::core::ffi::c_void);
        slot = slot_tmp;
        slot_tmp = ((*slot).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_sni_slot
            as *mut swaybar_sni_slot;
    }
    free(sni as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn handle_click(
    mut sni: *mut swaybar_sni,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut button: uint32_t,
    mut delta: ::core::ffi::c_int,
) {
    let mut method: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut binding: *mut tray_binding = ::core::ptr::null_mut::<tray_binding>();
    binding = ((*(*(*(*sni).tray).bar).config).tray_bindings.next
        as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut tray_binding
        as *mut tray_binding;
    while &raw mut (*binding).link
        != &raw mut (*(*(*(*sni).tray).bar).config).tray_bindings
    {
        if (*binding).button == button {
            method = (*binding).command;
            break;
        } else {
            binding = ((*binding).link.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut tray_binding
                as *mut tray_binding;
        }
    }
    if method.is_null() {
        static mut default_bindings: [*const ::core::ffi::c_char; 10] = [
            b"nop\0".as_ptr() as *const ::core::ffi::c_char,
            b"Activate\0".as_ptr() as *const ::core::ffi::c_char,
            b"SecondaryActivate\0".as_ptr() as *const ::core::ffi::c_char,
            b"ContextMenu\0".as_ptr() as *const ::core::ffi::c_char,
            b"ScrollUp\0".as_ptr() as *const ::core::ffi::c_char,
            b"ScrollDown\0".as_ptr() as *const ::core::ffi::c_char,
            b"ScrollLeft\0".as_ptr() as *const ::core::ffi::c_char,
            b"ScrollRight\0".as_ptr() as *const ::core::ffi::c_char,
            b"nop\0".as_ptr() as *const ::core::ffi::c_char,
            b"nop\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        method = default_bindings[event_to_x11_button(button) as usize];
    }
    if strcmp(method, b"nop\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return;
    }
    if (*sni).item_is_menu as ::core::ffi::c_int != 0
        && strcmp(method, b"Activate\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        method = b"ContextMenu\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if has_prefix(method, b"Scroll\0".as_ptr() as *const ::core::ffi::c_char) {
        let mut dir: ::core::ffi::c_char = *method
            .offset(strlen(b"Scroll\0".as_ptr() as *const ::core::ffi::c_char) as isize);
        let mut orientation: *mut ::core::ffi::c_char = (if dir as ::core::ffi::c_int
            == 'U' as ::core::ffi::c_int
            || dir as ::core::ffi::c_int == 'D' as ::core::ffi::c_int
        {
            b"vertical\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"horizontal\0".as_ptr() as *const ::core::ffi::c_char
        }) as *mut ::core::ffi::c_char;
        let mut sign: ::core::ffi::c_int = if dir as ::core::ffi::c_int
            == 'U' as ::core::ffi::c_int
            || dir as ::core::ffi::c_int == 'L' as ::core::ffi::c_int
        {
            -1 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        sd_bus_call_method_async(
            (*(*sni).tray).bus,
            ::core::ptr::null_mut::<*mut sd_bus_slot>(),
            (*sni).service,
            (*sni).path,
            (*sni).interface,
            b"Scroll\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            b"is\0".as_ptr() as *const ::core::ffi::c_char,
            delta * sign,
            orientation,
        );
    } else {
        sd_bus_call_method_async(
            (*(*sni).tray).bus,
            ::core::ptr::null_mut::<*mut sd_bus_slot>(),
            (*sni).service,
            (*sni).path,
            (*sni).interface,
            method,
            None,
            NULL,
            b"ii\0".as_ptr() as *const ::core::ffi::c_char,
            x,
            y,
        );
    };
}
unsafe extern "C" fn cmp_sni_id(
    mut item: *const ::core::ffi::c_void,
    mut cmp_to: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut sni: *const swaybar_sni = item as *const swaybar_sni;
    return strcmp((*sni).watcher_id, cmp_to as *const ::core::ffi::c_char);
}
unsafe extern "C" fn icon_hotspot_callback(
    mut output: *mut swaybar_output,
    mut hotspot: *mut swaybar_hotspot,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut button: uint32_t,
    mut released: bool,
    mut data: *mut ::core::ffi::c_void,
) -> hotspot_event_handling {
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Clicked on %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
        389 as ::core::ffi::c_int,
        data as *mut ::core::ffi::c_char,
    );
    let mut tray: *mut swaybar_tray = (*(*output).bar).tray;
    let mut idx: ::core::ffi::c_int = list_seq_find(
        (*tray).items,
        Some(
            cmp_sni_id
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        data,
    );
    if idx != -1 as ::core::ffi::c_int {
        if released {
            return hotspot_event_handling::HOTSPOT_IGNORE;
        }
        let mut sni: *mut swaybar_sni = *(*(*tray).items).items.offset(idx as isize)
            as *mut swaybar_sni;
        let mut config: *mut swaybar_config = (*(*tray).bar).config;
        let mut global_x: ::core::ffi::c_int = ((*output)
            .output_x
            .wrapping_add((*config).gaps.left as uint32_t) as ::core::ffi::c_double + x)
            as ::core::ffi::c_int;
        let mut top_bar: bool = (*config).position
            & zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_TOP.0
                as ::core::ffi::c_int as uint32_t != 0;
        let mut global_y: ::core::ffi::c_int = ((*output).output_y
            as ::core::ffi::c_double
            + if top_bar as ::core::ffi::c_int != 0 {
                (*config).gaps.top as ::core::ffi::c_double + y
            } else {
                ((*output).output_height as ::core::ffi::c_int - (*config).gaps.bottom)
                    as ::core::ffi::c_double - y
            }) as ::core::ffi::c_int;
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Guessing click position at (%d, %d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
            408 as ::core::ffi::c_int,
            global_x,
            global_y,
        );
        handle_click(sni, global_x, global_y, button, 1 as ::core::ffi::c_int);
        return hotspot_event_handling::HOTSPOT_IGNORE;
    } else {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] but it doesn't exist\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/tray/item.c\0".as_ptr() as *const ::core::ffi::c_char,
            412 as ::core::ffi::c_int,
        );
    }
    return hotspot_event_handling::HOTSPOT_PROCESS;
}
unsafe extern "C" fn reload_sni(
    mut sni: *mut swaybar_sni,
    mut icon_theme: *mut ::core::ffi::c_char,
    mut target_size: ::core::ffi::c_int,
) {
    let mut icon_name: *mut ::core::ffi::c_char = if *(*sni).status.offset(0isize)
        as ::core::ffi::c_int == 'N' as ::core::ffi::c_int
    {
        (*sni).attention_icon_name
    } else {
        (*sni).icon_name
    };
    if !icon_name.is_null() {
        let mut icon_search_paths: *mut list_t = create_list();
        list_cat(icon_search_paths, (*(*sni).tray).basedirs);
        if !(*sni).icon_theme_path.is_null() {
            list_add(
                icon_search_paths,
                (*sni).icon_theme_path as *mut ::core::ffi::c_void,
            );
        }
        let mut icon_path: *mut ::core::ffi::c_char = find_icon(
            (*(*sni).tray).themes,
            icon_search_paths,
            icon_name,
            target_size,
            icon_theme,
            &raw mut (*sni).min_size,
            &raw mut (*sni).max_size,
        );
        list_free(icon_search_paths);
        if !icon_path.is_null() {
            cairo_surface_destroy((*sni).icon);
            (*sni).icon = load_image(icon_path);
            free(icon_path as *mut ::core::ffi::c_void);
            return;
        }
    }
    let mut pixmaps: *mut list_t = if *(*sni).status.offset(0isize) as ::core::ffi::c_int
        == 'N' as ::core::ffi::c_int
    {
        (*sni).attention_icon_pixmap
    } else {
        (*sni).icon_pixmap
    };
    if !pixmaps.is_null() {
        let mut pixmap: *mut swaybar_pixmap = ::core::ptr::null_mut::<swaybar_pixmap>();
        let mut min_error: ::core::ffi::c_int = INT_MAX;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*pixmaps).length {
            let mut p: *mut swaybar_pixmap = *(*pixmaps).items.offset(i as isize)
                as *mut swaybar_pixmap;
            let mut e: ::core::ffi::c_int = abs(target_size - (*p).size);
            if e < min_error {
                pixmap = p;
                min_error = e;
            }
            i += 1;
        }
        cairo_surface_destroy((*sni).icon);
        (*sni).icon = cairo_image_surface_create_for_data(
            &raw mut (*pixmap).pixels as *mut ::core::ffi::c_uchar,
            _cairo_format::CAIRO_FORMAT_ARGB32,
            (*pixmap).size,
            (*pixmap).size,
            cairo_format_stride_for_width(
                _cairo_format::CAIRO_FORMAT_ARGB32,
                (*pixmap).size,
            ),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn render_sni(
    mut cairo: *mut cairo_t,
    mut output: *mut swaybar_output,
    mut x: *mut ::core::ffi::c_double,
    mut sni: *mut swaybar_sni,
) -> uint32_t {
    let mut height: uint32_t = (*output)
        .height
        .wrapping_mul((*output).scale as uint32_t);
    let mut padding: ::core::ffi::c_int = (*(*(*output).bar).config).tray_padding;
    let mut target_size: ::core::ffi::c_int = height
        .wrapping_sub((2 as ::core::ffi::c_int * padding) as uint32_t)
        as ::core::ffi::c_int;
    if target_size != (*sni).target_size && sni_ready(sni) as ::core::ffi::c_int != 0 {
        if target_size < (*sni).min_size || target_size > (*sni).max_size {
            reload_sni(sni, (*(*(*output).bar).config).icon_theme, target_size);
        }
        (*sni).target_size = target_size;
    }
    if !(*sni).status.is_null()
        && *(*sni).status.offset(0isize) as ::core::ffi::c_int
            == 'P' as ::core::ffi::c_int
    {
        return 0 as uint32_t;
    }
    let mut icon_size: ::core::ffi::c_int = 0;
    let mut icon: *mut cairo_surface_t = ::core::ptr::null_mut::<cairo_surface_t>();
    if !(*sni).icon.is_null() {
        let mut actual_size: ::core::ffi::c_int = cairo_image_surface_get_height(
            (*sni).icon,
        );
        icon_size = if actual_size < target_size {
            actual_size * (target_size / actual_size)
        } else {
            target_size
        };
        icon = cairo_image_surface_scale((*sni).icon, icon_size, icon_size);
    } else {
        icon_size = (target_size as ::core::ffi::c_double * 0.8f64)
            as ::core::ffi::c_int;
        icon = cairo_image_surface_create(
            _cairo_format::CAIRO_FORMAT_ARGB32,
            icon_size,
            icon_size,
        );
        let mut cairo_icon: *mut cairo_t = cairo_create(icon);
        cairo_set_source_u32(cairo_icon, 0xff0000ff as uint32_t);
        cairo_translate(
            cairo_icon,
            (icon_size / 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
            (icon_size / 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
        );
        cairo_scale(
            cairo_icon,
            (icon_size / 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
            (icon_size / 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
        );
        cairo_arc(
            cairo_icon,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            1 as ::core::ffi::c_int as ::core::ffi::c_double,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            7 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
        cairo_fill(cairo_icon);
        cairo_set_operator(cairo_icon, _cairo_operator::CAIRO_OPERATOR_CLEAR);
        cairo_arc(
            cairo_icon,
            0.35f64,
            -0.3f64,
            0.1f64,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            7 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
        cairo_fill(cairo_icon);
        cairo_arc(
            cairo_icon,
            -0.35f64,
            -0.3f64,
            0.1f64,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            7 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
        cairo_fill(cairo_icon);
        cairo_arc(
            cairo_icon,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            0.75f64,
            0.5f64,
            3.71238898038469f64,
            5.71238898038469f64,
        );
        cairo_set_line_width(cairo_icon, 0.1f64);
        cairo_stroke(cairo_icon);
        cairo_destroy(cairo_icon);
    }
    let mut descaled_padding: ::core::ffi::c_double = padding as ::core::ffi::c_double
        / (*output).scale as ::core::ffi::c_double;
    let mut descaled_icon_size: ::core::ffi::c_double = icon_size
        as ::core::ffi::c_double / (*output).scale as ::core::ffi::c_double;
    let mut size: ::core::ffi::c_int = (descaled_icon_size
        + 2 as ::core::ffi::c_int as ::core::ffi::c_double * descaled_padding)
        as ::core::ffi::c_int;
    *x -= size as ::core::ffi::c_double;
    let mut icon_y: ::core::ffi::c_int = floor(
        (*output).height.wrapping_sub(size as uint32_t) as ::core::ffi::c_double / 2.0f64,
    ) as ::core::ffi::c_int;
    let mut op: cairo_operator_t = cairo_get_operator(cairo);
    cairo_set_operator(cairo, _cairo_operator::CAIRO_OPERATOR_OVER);
    let mut scale_matrix: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut icon_pattern: *mut cairo_pattern_t = cairo_pattern_create_for_surface(icon);
    cairo_matrix_init_scale(
        &raw mut scale_matrix,
        (*output).scale as ::core::ffi::c_double,
        (*output).scale as ::core::ffi::c_double,
    );
    cairo_matrix_translate(
        &raw mut scale_matrix,
        -(*x + descaled_padding),
        -(icon_y as ::core::ffi::c_double + descaled_padding),
    );
    cairo_pattern_set_matrix(icon_pattern, &raw mut scale_matrix);
    cairo_set_source(cairo, icon_pattern);
    cairo_rectangle(
        cairo,
        *x,
        icon_y as ::core::ffi::c_double,
        size as ::core::ffi::c_double,
        size as ::core::ffi::c_double,
    );
    cairo_fill(cairo);
    cairo_set_operator(cairo, op);
    cairo_pattern_destroy(icon_pattern);
    cairo_surface_destroy(icon);
    let mut hotspot: *mut swaybar_hotspot = calloc(
        1 as size_t,
        ::core::mem::size_of::<swaybar_hotspot>(),
    ) as *mut swaybar_hotspot;
    (*hotspot).x = *x as ::core::ffi::c_int;
    (*hotspot).y = 0 as ::core::ffi::c_int;
    (*hotspot).width = size;
    (*hotspot).height = (*output).height as ::core::ffi::c_int;
    (*hotspot).callback = Some(
        icon_hotspot_callback
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
    (*hotspot).data = strdup((*sni).watcher_id) as *mut ::core::ffi::c_void;
    wl_list_insert(&raw mut (*output).hotspots, &raw mut (*hotspot).link);
    return (*output).height;
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
