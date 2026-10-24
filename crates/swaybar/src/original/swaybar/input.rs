extern "C" {
    pub type wl_proxy;
    pub type wl_display;
    pub type wl_buffer;
    pub type wl_compositor;
    pub type wl_output;
    pub type wl_pointer;
    pub type wl_seat;
    pub type wl_shm;
    pub type wl_surface;
    pub type wl_touch;
    pub type wl_cursor_theme;
    pub type _PangoContext;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type swaybar_tray;
    pub type r#loop;
    pub type status_line;
    pub type wp_cursor_shape_manager_v1;
    pub type zxdg_output_manager_v1;
    pub type zwlr_layer_shell_v1;
    pub type zwlr_layer_surface_v1;
    pub type zxdg_output_v1;
    pub type wp_cursor_shape_device_v1;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    static wl_surface_interface: wl_interface;
    static wl_pointer_interface: wl_interface;
    static wl_touch_interface: wl_interface;
    fn ipc_send_workspace_command(bar: *mut swaybar, ws: *const ::core::ffi::c_char);
    fn ipc_execute_binding(bar: *mut swaybar, bind: *mut swaybar_binding);
    static wp_cursor_shape_device_v1_interface: wl_interface;
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_pointer_axis(pub ::core::ffi::c_uint);
impl wl_pointer_axis {
    pub const WL_POINTER_AXIS_VERTICAL_SCROLL: Self = Self(0);
    pub const WL_POINTER_AXIS_HORIZONTAL_SCROLL: Self = Self(1);
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
pub struct wl_touch_listener {
    pub down: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_touch,
            uint32_t,
            uint32_t,
            *mut wl_surface,
            int32_t,
            wl_fixed_t,
            wl_fixed_t,
        ) -> (),
    >,
    pub up: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_touch,
            uint32_t,
            uint32_t,
            int32_t,
        ) -> (),
    >,
    pub motion: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_touch,
            uint32_t,
            int32_t,
            wl_fixed_t,
            wl_fixed_t,
        ) -> (),
    >,
    pub frame: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_touch) -> (),
    >,
    pub cancel: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_touch) -> (),
    >,
    pub shape: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_touch,
            int32_t,
            wl_fixed_t,
            wl_fixed_t,
        ) -> (),
    >,
    pub orientation: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut wl_touch,
            int32_t,
            wl_fixed_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
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
pub type ssize_t = isize;
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
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 81] = unsafe {
    ::core::mem::transmute::<
        [u8; 81],
        [::core::ffi::c_char; 81],
    >(
        *b"void seat_handle_capabilities(void *, struct wl_seat *, enum wl_seat_capability)\0",
    )
};
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_fixed_to_double(mut f: wl_fixed_t) -> ::core::ffi::c_double {
    return f as ::core::ffi::c_double / 256.0f64;
}
pub const WL_MARSHAL_FLAG_DESTROY: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
    << 0 as ::core::ffi::c_int;
pub const WL_COMPOSITOR_CREATE_SURFACE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const WL_SEAT_GET_POINTER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WL_SEAT_GET_TOUCH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
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
unsafe extern "C" fn wl_seat_get_touch(mut wl_seat: *mut wl_seat) -> *mut wl_touch {
    let mut id: *mut wl_proxy = ::core::ptr::null_mut::<wl_proxy>();
    id = wl_proxy_marshal_flags(
        wl_seat as *mut wl_proxy,
        WL_SEAT_GET_TOUCH as uint32_t,
        &raw const wl_touch_interface,
        wl_proxy_get_version(wl_seat as *mut wl_proxy),
        0 as uint32_t,
        NULL,
    );
    return id as *mut wl_touch;
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
pub const WL_POINTER_RELEASE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
unsafe extern "C" fn wl_pointer_release(mut wl_pointer: *mut wl_pointer) {
    wl_proxy_marshal_flags(
        wl_pointer as *mut wl_proxy,
        WL_POINTER_RELEASE as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_pointer as *mut wl_proxy),
        WL_MARSHAL_FLAG_DESTROY as uint32_t,
    );
}
#[inline]
unsafe extern "C" fn wl_touch_add_listener(
    mut wl_touch: *mut wl_touch,
    mut listener: *const wl_touch_listener,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return wl_proxy_add_listener(
        wl_touch as *mut wl_proxy,
        listener as *mut Option<unsafe extern "C" fn() -> ()>,
        data,
    );
}
pub const WL_TOUCH_RELEASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_touch_release(mut wl_touch: *mut wl_touch) {
    wl_proxy_marshal_flags(
        wl_touch as *mut wl_proxy,
        WL_TOUCH_RELEASE as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_touch as *mut wl_proxy),
        WL_MARSHAL_FLAG_DESTROY as uint32_t,
    );
}
pub const SWAY_SCROLL_UP: ::core::ffi::c_int = KEY_MAX + 1 as ::core::ffi::c_int;
pub const SWAY_SCROLL_DOWN: ::core::ffi::c_int = KEY_MAX + 2 as ::core::ffi::c_int;
pub const SWAY_SCROLL_LEFT: ::core::ffi::c_int = KEY_MAX + 3 as ::core::ffi::c_int;
pub const SWAY_SCROLL_RIGHT: ::core::ffi::c_int = KEY_MAX + 4 as ::core::ffi::c_int;
pub const SWAY_CONTINUOUS_SCROLL_TIMEOUT: ::core::ffi::c_int = 1000
    as ::core::ffi::c_int;
pub const SWAY_CONTINUOUS_SCROLL_THRESHOLD: ::core::ffi::c_int = 10000
    as ::core::ffi::c_int;
pub const BTN_LEFT: ::core::ffi::c_int = 0x110 as ::core::ffi::c_int;
pub const BTN_RIGHT: uint32_t = 273 as uint32_t;
pub const BTN_MIDDLE: uint32_t = 274 as uint32_t;
pub const BTN_SIDE: uint32_t = 275 as uint32_t;
pub const BTN_EXTRA: uint32_t = 276 as uint32_t;
pub const KEY_MAX: ::core::ffi::c_int = 0x2ff as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn free_hotspots(mut list: *mut wl_list) {
    let mut hotspot: *mut swaybar_hotspot = ::core::ptr::null_mut::<swaybar_hotspot>();
    let mut tmp: *mut swaybar_hotspot = ::core::ptr::null_mut::<swaybar_hotspot>();
    hotspot = ((*list).next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_hotspot
        as *mut swaybar_hotspot;
    tmp = ((*hotspot).link.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_hotspot
        as *mut swaybar_hotspot;
    while &raw mut (*hotspot).link != list {
        wl_list_remove(&raw mut (*hotspot).link);
        if (*hotspot).destroy.is_some() {
            (*hotspot).destroy.expect("non-null function pointer")((*hotspot).data);
        }
        free(hotspot as *mut ::core::ffi::c_void);
        hotspot = tmp;
        tmp = ((*hotspot).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_hotspot
            as *mut swaybar_hotspot;
    }
}
#[no_mangle]
pub unsafe extern "C" fn event_to_x11_button(mut event: uint32_t) -> uint32_t {
    match event {
        0x110 => return 1 as uint32_t,
        BTN_MIDDLE => return 2 as uint32_t,
        BTN_RIGHT => return 3 as uint32_t,
        768 => return 4 as uint32_t,
        769 => return 5 as uint32_t,
        770 => return 6 as uint32_t,
        771 => return 7 as uint32_t,
        BTN_SIDE => return 8 as uint32_t,
        BTN_EXTRA => return 9 as uint32_t,
        _ => return 0 as uint32_t,
    };
}
unsafe extern "C" fn wl_axis_to_button(
    mut axis: uint32_t,
    mut value: wl_fixed_t,
) -> uint32_t {
    let mut negative: bool = wl_fixed_to_double(value)
        < 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    match axis {
        0 => {
            return (if negative as ::core::ffi::c_int != 0 {
                SWAY_SCROLL_UP
            } else {
                SWAY_SCROLL_DOWN
            }) as uint32_t;
        }
        1 => {
            return (if negative as ::core::ffi::c_int != 0 {
                SWAY_SCROLL_LEFT
            } else {
                SWAY_SCROLL_RIGHT
            }) as uint32_t;
        }
        _ => {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Unexpected axis value on mouse scroll\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaybar/input.c\0".as_ptr() as *const ::core::ffi::c_char,
                57 as ::core::ffi::c_int,
            );
            return 0 as uint32_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn update_cursor(mut seat: *mut swaybar_seat) {
    let mut pointer: *mut swaybar_pointer = &raw mut (*seat).pointer;
    if pointer.is_null() || (*pointer).cursor_surface.is_null() {
        return;
    }
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
    if !env_cursor_size.is_null() && strlen(env_cursor_size) > 0 as size_t {
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
    let mut scale: ::core::ffi::c_int = if !(*pointer).current.is_null() {
        (*(*pointer).current).scale as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    (*pointer).cursor_theme = wl_cursor_theme_load(
        cursor_theme,
        cursor_size.wrapping_mul(scale as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (*(*seat).bar).shm,
    );
    if (*pointer).cursor_theme.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to load cursor theme\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/input.c\0".as_ptr() as *const ::core::ffi::c_char,
            85 as ::core::ffi::c_int,
        );
        return;
    }
    let mut cursor: *mut wl_cursor = ::core::ptr::null_mut::<wl_cursor>();
    cursor = wl_cursor_theme_get_cursor(
        (*pointer).cursor_theme,
        b"default\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cursor.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to get default cursor from theme\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/input.c\0".as_ptr() as *const ::core::ffi::c_char,
            91 as ::core::ffi::c_int,
        );
        return;
    }
    (*pointer).cursor_image = *(*cursor).images.offset(0isize);
    wl_surface_set_buffer_scale((*pointer).cursor_surface, scale as int32_t);
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
        (*(*pointer).cursor_image).hotspot_x.wrapping_div(scale as uint32_t) as int32_t,
        (*(*pointer).cursor_image).hotspot_y.wrapping_div(scale as uint32_t) as int32_t,
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
unsafe extern "C" fn wl_pointer_enter(
    mut data: *mut ::core::ffi::c_void,
    mut wl_pointer: *mut wl_pointer,
    mut serial: uint32_t,
    mut surface: *mut wl_surface,
    mut surface_x: wl_fixed_t,
    mut surface_y: wl_fixed_t,
) {
    let mut seat: *mut swaybar_seat = data as *mut swaybar_seat;
    let mut pointer: *mut swaybar_pointer = &raw mut (*seat).pointer;
    (*seat).pointer.x = wl_fixed_to_double(surface_x);
    (*seat).pointer.y = wl_fixed_to_double(surface_y);
    let mut output: *mut swaybar_output = ::core::ptr::null_mut::<swaybar_output>();
    output = ((*(*seat).bar).outputs.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    while &raw mut (*output).link != &raw mut (*(*seat).bar).outputs {
        if (*output).surface == surface {
            (*pointer).current = output;
            break;
        } else {
            output = ((*output).link.next as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
                as *mut swaybar_output;
        }
    }
    if !(*(*seat).bar).cursor_shape_manager.is_null() {
        let mut device: *mut wp_cursor_shape_device_v1 = wp_cursor_shape_manager_v1_get_pointer(
            (*(*seat).bar).cursor_shape_manager,
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
unsafe extern "C" fn wl_pointer_leave(
    mut data: *mut ::core::ffi::c_void,
    mut wl_pointer: *mut wl_pointer,
    mut serial: uint32_t,
    mut surface: *mut wl_surface,
) {
    let mut seat: *mut swaybar_seat = data as *mut swaybar_seat;
    (*seat).pointer.current = ::core::ptr::null_mut::<swaybar_output>();
}
unsafe extern "C" fn wl_pointer_motion(
    mut data: *mut ::core::ffi::c_void,
    mut wl_pointer: *mut wl_pointer,
    mut time: uint32_t,
    mut surface_x: wl_fixed_t,
    mut surface_y: wl_fixed_t,
) {
    let mut seat: *mut swaybar_seat = data as *mut swaybar_seat;
    (*seat).pointer.x = wl_fixed_to_double(surface_x);
    (*seat).pointer.y = wl_fixed_to_double(surface_y);
}
unsafe extern "C" fn check_bindings(
    mut bar: *mut swaybar,
    mut button: uint32_t,
    mut state: uint32_t,
) -> bool {
    let mut released: bool = state
        == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_RELEASED.0
            as ::core::ffi::c_int as uint32_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*(*bar).config).bindings).length {
        let mut binding: *mut swaybar_binding = *(*(*(*bar).config).bindings)
            .items
            .offset(i as isize) as *mut swaybar_binding;
        if (*binding).button == button
            && (*binding).release as ::core::ffi::c_int == released as ::core::ffi::c_int
        {
            ipc_execute_binding(bar, binding);
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn process_hotspots(
    mut output: *mut swaybar_output,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut button: uint32_t,
    mut state: uint32_t,
) -> bool {
    let mut released: bool = state
        == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_RELEASED.0
            as ::core::ffi::c_int as uint32_t;
    let mut hotspot: *mut swaybar_hotspot = ::core::ptr::null_mut::<swaybar_hotspot>();
    hotspot = ((*output).hotspots.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_hotspot
        as *mut swaybar_hotspot;
    while &raw mut (*hotspot).link != &raw mut (*output).hotspots {
        if x >= (*hotspot).x as ::core::ffi::c_double
            && y >= (*hotspot).y as ::core::ffi::c_double
            && x < ((*hotspot).x + (*hotspot).width) as ::core::ffi::c_double
            && y < ((*hotspot).y + (*hotspot).height) as ::core::ffi::c_double
        {
            if hotspot_event_handling::HOTSPOT_IGNORE.0
                == (*hotspot)
                    .callback
                    .expect(
                        "non-null function pointer",
                    )(output, hotspot, x, y, button, released, (*hotspot).data)
                    .0
            {
                return r#true != 0;
            }
        }
        hotspot = ((*hotspot).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_hotspot
            as *mut swaybar_hotspot;
    }
    return r#false != 0;
}
unsafe extern "C" fn wl_pointer_button(
    mut data: *mut ::core::ffi::c_void,
    mut wl_pointer: *mut wl_pointer,
    mut serial: uint32_t,
    mut time: uint32_t,
    mut button: uint32_t,
    mut state: uint32_t,
) {
    let mut seat: *mut swaybar_seat = data as *mut swaybar_seat;
    let mut pointer: *mut swaybar_pointer = &raw mut (*seat).pointer;
    let mut output: *mut swaybar_output = (*pointer).current;
    if !_sway_assert(
        !output.is_null(),
        b"[%s:%d] %s:button with no active output\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"swaybar/input.c\0".as_ptr() as *const ::core::ffi::c_char,
        185 as ::core::ffi::c_int,
        b"void wl_pointer_button(void *, struct wl_pointer *, uint32_t, uint32_t, uint32_t, uint32_t)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if process_hotspots(output, (*pointer).x, (*pointer).y, button, state) {
        return;
    }
    check_bindings((*seat).bar, button, state);
}
unsafe extern "C" fn workspace_next(
    mut bar: *mut swaybar,
    mut output: *mut swaybar_output,
    mut left: bool,
) {
    let mut config: *mut swaybar_config = (*bar).config;
    let mut first: *mut swaybar_workspace = ((*output).workspaces.next
        as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_workspace;
    let mut last: *mut swaybar_workspace = ((*output).workspaces.prev
        as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_workspace;
    let mut active: *mut swaybar_workspace = ::core::ptr::null_mut::<
        swaybar_workspace,
    >();
    active = ((*output).workspaces.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_workspace
        as *mut swaybar_workspace;
    while &raw mut (*active).link != &raw mut (*output).workspaces {
        if (*active).visible {
            break;
        }
        active = ((*active).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_workspace
            as *mut swaybar_workspace;
    }
    if !_sway_assert(
        (*active).visible,
        b"[%s:%d] %s:axis with null workspace\0".as_ptr() as *const ::core::ffi::c_char,
        b"swaybar/input.c\0".as_ptr() as *const ::core::ffi::c_char,
        210 as ::core::ffi::c_int,
        b"void workspace_next(struct swaybar *, struct swaybar_output *, _Bool)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    let mut new: *mut swaybar_workspace = ::core::ptr::null_mut::<swaybar_workspace>();
    if left {
        if active == first {
            new = if (*config).wrap_scroll as ::core::ffi::c_int != 0 {
                last
            } else {
                ::core::ptr::null_mut::<swaybar_workspace>()
            };
        } else {
            new = ((*active).link.prev as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_workspace
                as *mut swaybar_workspace;
        }
    } else if active == last {
        new = if (*config).wrap_scroll as ::core::ffi::c_int != 0 {
            first
        } else {
            ::core::ptr::null_mut::<swaybar_workspace>()
        };
    } else {
        new = ((*active).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_workspace
            as *mut swaybar_workspace;
    }
    if !new.is_null() && new != active {
        ipc_send_workspace_command(bar, (*new).name);
        (*new).visible = r#true != 0;
        (*active).visible = r#false != 0;
    }
}
unsafe extern "C" fn process_discrete_scroll(
    mut seat: *mut swaybar_seat,
    mut output: *mut swaybar_output,
    mut pointer: *mut swaybar_pointer,
    mut axis: uint32_t,
    mut value: wl_fixed_t,
) {
    let mut button: uint32_t = wl_axis_to_button(axis, value);
    if process_hotspots(
        output,
        (*pointer).x,
        (*pointer).y,
        button,
        wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0 as ::core::ffi::c_int
            as uint32_t,
    ) {
        return;
    }
    if check_bindings(
        (*seat).bar,
        button,
        wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0 as ::core::ffi::c_int
            as uint32_t,
    ) {
        check_bindings(
            (*seat).bar,
            button,
            wl_pointer_button_state::WL_POINTER_BUTTON_STATE_RELEASED.0
                as ::core::ffi::c_int as uint32_t,
        );
        return;
    }
    let mut config: *mut swaybar_config = (*(*seat).bar).config;
    let mut amt: ::core::ffi::c_double = wl_fixed_to_double(value);
    if amt == 0.0f64 || !(*config).workspace_buttons {
        check_bindings(
            (*seat).bar,
            button,
            wl_pointer_button_state::WL_POINTER_BUTTON_STATE_RELEASED.0
                as ::core::ffi::c_int as uint32_t,
        );
        return;
    }
    if !_sway_assert(
        wl_list_empty(&raw mut (*output).workspaces) == 0,
        b"[%s:%d] %s:axis with no workspaces\0".as_ptr() as *const ::core::ffi::c_char,
        b"swaybar/input.c\0".as_ptr() as *const ::core::ffi::c_char,
        261 as ::core::ffi::c_int,
        b"void process_discrete_scroll(struct swaybar_seat *, struct swaybar_output *, struct swaybar_pointer *, uint32_t, wl_fixed_t)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    workspace_next((*seat).bar, output, amt < 0.0f64);
    check_bindings(
        (*seat).bar,
        button,
        wl_pointer_button_state::WL_POINTER_BUTTON_STATE_RELEASED.0 as ::core::ffi::c_int
            as uint32_t,
    );
}
unsafe extern "C" fn process_continuous_scroll(
    mut seat: *mut swaybar_seat,
    mut output: *mut swaybar_output,
    mut pointer: *mut swaybar_pointer,
    mut axis: uint32_t,
) {
    while abs((*seat).axis[axis as usize].value as ::core::ffi::c_int)
        > SWAY_CONTINUOUS_SCROLL_THRESHOLD
    {
        if (*seat).axis[axis as usize].value > 0 as wl_fixed_t {
            process_discrete_scroll(
                seat,
                output,
                pointer,
                axis,
                SWAY_CONTINUOUS_SCROLL_THRESHOLD as wl_fixed_t,
            );
            (*seat).axis[axis as usize].value = ((*seat).axis[axis as usize].value
                as ::core::ffi::c_int - SWAY_CONTINUOUS_SCROLL_THRESHOLD) as wl_fixed_t;
        } else {
            process_discrete_scroll(
                seat,
                output,
                pointer,
                axis,
                -(SWAY_CONTINUOUS_SCROLL_THRESHOLD as wl_fixed_t),
            );
            (*seat).axis[axis as usize].value = ((*seat).axis[axis as usize].value
                as ::core::ffi::c_int + SWAY_CONTINUOUS_SCROLL_THRESHOLD) as wl_fixed_t;
        }
    }
}
unsafe extern "C" fn wl_pointer_axis_0(
    mut data: *mut ::core::ffi::c_void,
    mut wl_pointer: *mut wl_pointer,
    mut time: uint32_t,
    mut axis: uint32_t,
    mut value: wl_fixed_t,
) {
    let mut seat: *mut swaybar_seat = data as *mut swaybar_seat;
    let mut pointer: *mut swaybar_pointer = &raw mut (*seat).pointer;
    let mut output: *mut swaybar_output = (*pointer).current;
    if !_sway_assert(
        !output.is_null(),
        b"[%s:%d] %s:axis with no active output\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"swaybar/input.c\0".as_ptr() as *const ::core::ffi::c_char,
        292 as ::core::ffi::c_int,
        b"void wl_pointer_axis(void *, struct wl_pointer *, uint32_t, uint32_t, wl_fixed_t)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if !_sway_assert(
        axis < 2 as uint32_t,
        b"[%s:%d] %s:axis out of range\0".as_ptr() as *const ::core::ffi::c_char,
        b"swaybar/input.c\0".as_ptr() as *const ::core::ffi::c_char,
        296 as ::core::ffi::c_int,
        b"void wl_pointer_axis(void *, struct wl_pointer *, uint32_t, uint32_t, wl_fixed_t)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if (*seat).axis[axis as usize].discrete_steps == 0 as uint32_t
        && time.wrapping_sub((*seat).axis[axis as usize].update_time)
            > SWAY_CONTINUOUS_SCROLL_TIMEOUT as uint32_t
    {
        (*seat).axis[axis as usize].value = 0 as ::core::ffi::c_int as wl_fixed_t;
    }
    (*seat).axis[axis as usize].value += value;
    (*seat).axis[axis as usize].update_time = time;
}
unsafe extern "C" fn wl_pointer_frame(
    mut data: *mut ::core::ffi::c_void,
    mut wl_pointer: *mut wl_pointer,
) {
    let mut seat: *mut swaybar_seat = data as *mut swaybar_seat;
    let mut pointer: *mut swaybar_pointer = &raw mut (*seat).pointer;
    let mut output: *mut swaybar_output = (*pointer).current;
    if output.is_null() {
        return;
    }
    let mut axis: uint32_t = 0 as uint32_t;
    while axis < 2 as uint32_t {
        if (*seat).axis[axis as usize].discrete_steps != 0 {
            let mut step: uint32_t = 0 as uint32_t;
            while step < (*seat).axis[axis as usize].discrete_steps {
                process_discrete_scroll(
                    seat,
                    output,
                    pointer,
                    axis,
                    (*seat).axis[axis as usize].value,
                );
                step = step.wrapping_add(1);
            }
            (*seat).axis[axis as usize].value = 0 as ::core::ffi::c_int as wl_fixed_t;
            (*seat).axis[axis as usize].discrete_steps = 0 as uint32_t;
        } else {
            process_continuous_scroll(seat, output, pointer, axis);
        }
        axis = axis.wrapping_add(1);
    }
}
unsafe extern "C" fn wl_pointer_axis_source(
    mut data: *mut ::core::ffi::c_void,
    mut wl_pointer: *mut wl_pointer,
    mut axis_source: uint32_t,
) {}
unsafe extern "C" fn wl_pointer_axis_stop(
    mut data: *mut ::core::ffi::c_void,
    mut wl_pointer: *mut wl_pointer,
    mut time: uint32_t,
    mut axis: uint32_t,
) {}
unsafe extern "C" fn wl_pointer_axis_discrete(
    mut data: *mut ::core::ffi::c_void,
    mut wl_pointer: *mut wl_pointer,
    mut axis: uint32_t,
    mut discrete: int32_t,
) {
    let mut seat: *mut swaybar_seat = data as *mut swaybar_seat;
    if !_sway_assert(
        axis < 2 as uint32_t,
        b"[%s:%d] %s:axis out of range\0".as_ptr() as *const ::core::ffi::c_char,
        b"swaybar/input.c\0".as_ptr() as *const ::core::ffi::c_char,
        352 as ::core::ffi::c_int,
        b"void wl_pointer_axis_discrete(void *, struct wl_pointer *, uint32_t, int32_t)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    (*seat).axis[axis as usize].discrete_steps = (*seat)
        .axis[axis as usize]
        .discrete_steps
        .wrapping_add(abs(discrete as ::core::ffi::c_int) as uint32_t);
}
static mut pointer_listener: wl_pointer_listener = wl_pointer_listener {
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
    leave: Some(
        wl_pointer_leave
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_pointer,
                uint32_t,
                *mut wl_surface,
            ) -> (),
    ),
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
        wl_pointer_axis_0
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_pointer,
                uint32_t,
                uint32_t,
                wl_fixed_t,
            ) -> (),
    ),
    frame: Some(
        wl_pointer_frame
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_pointer) -> (),
    ),
    axis_source: Some(
        wl_pointer_axis_source
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_pointer,
                uint32_t,
            ) -> (),
    ),
    axis_stop: Some(
        wl_pointer_axis_stop
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_pointer,
                uint32_t,
                uint32_t,
            ) -> (),
    ),
    axis_discrete: Some(
        wl_pointer_axis_discrete
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_pointer,
                uint32_t,
                int32_t,
            ) -> (),
    ),
    axis_value120: None,
    axis_relative_direction: None,
    warp: None,
};
unsafe extern "C" fn get_touch_slot(
    mut touch: *mut swaybar_touch,
    mut id: int32_t,
) -> *mut touch_slot {
    let mut next: ssize_t = -1 as ssize_t;
    let mut i: size_t = 0 as size_t;
    while i
        < ::core::mem::size_of::<[touch_slot; 16]>()
            .wrapping_div(::core::mem::size_of::<touch_slot>())
    {
        if (*touch).slots[i].id == id {
            return (&raw mut (*touch).slots as *mut touch_slot).offset(i as isize);
        }
        if next == -1 as ssize_t && (*touch).slots[i].output.is_null() {
            next = i as ssize_t;
        }
        i = i.wrapping_add(1);
    }
    if next == -1 as ssize_t {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Ran out of touch slots\0".as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/input.c\0".as_ptr() as *const ::core::ffi::c_char,
            382 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<touch_slot>();
    }
    return (&raw mut (*touch).slots as *mut touch_slot).offset(next);
}
unsafe extern "C" fn wl_touch_down(
    mut data: *mut ::core::ffi::c_void,
    mut wl_touch: *mut wl_touch,
    mut serial: uint32_t,
    mut time: uint32_t,
    mut surface: *mut wl_surface,
    mut id: int32_t,
    mut _x: wl_fixed_t,
    mut _y: wl_fixed_t,
) {
    let mut seat: *mut swaybar_seat = data as *mut swaybar_seat;
    let mut _output: *mut swaybar_output = ::core::ptr::null_mut::<swaybar_output>();
    let mut output: *mut swaybar_output = ::core::ptr::null_mut::<swaybar_output>();
    _output = ((*(*seat).bar).outputs.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
        as *mut swaybar_output;
    while &raw mut (*_output).link != &raw mut (*(*seat).bar).outputs {
        if (*_output).surface == surface {
            output = _output;
            break;
        } else {
            _output = ((*_output).link.next as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut swaybar_output
                as *mut swaybar_output;
        }
    }
    if output.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Got touch event for unknown surface\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/input.c\0".as_ptr() as *const ::core::ffi::c_char,
            400 as ::core::ffi::c_int,
        );
        return;
    }
    let mut slot: *mut touch_slot = get_touch_slot(&raw mut (*seat).touch, id);
    if slot.is_null() {
        return;
    }
    (*slot).id = id;
    (*slot).output = output;
    (*slot).start_x = wl_fixed_to_double(_x);
    (*slot).x = (*slot).start_x;
    (*slot).start_y = wl_fixed_to_double(_y);
    (*slot).y = (*slot).start_y;
    (*slot).time = time;
}
unsafe extern "C" fn wl_touch_up(
    mut data: *mut ::core::ffi::c_void,
    mut wl_touch: *mut wl_touch,
    mut serial: uint32_t,
    mut time: uint32_t,
    mut id: int32_t,
) {
    let mut seat: *mut swaybar_seat = data as *mut swaybar_seat;
    let mut slot: *mut touch_slot = get_touch_slot(&raw mut (*seat).touch, id);
    if slot.is_null() {
        return;
    }
    if time.wrapping_sub((*slot).time) < 500 as uint32_t {
        process_hotspots(
            (*slot).output,
            (*slot).x,
            (*slot).y,
            BTN_LEFT as uint32_t,
            wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0
                as ::core::ffi::c_int as uint32_t,
        );
    }
    (*slot).output = ::core::ptr::null_mut::<swaybar_output>();
}
unsafe extern "C" fn wl_touch_motion(
    mut data: *mut ::core::ffi::c_void,
    mut wl_touch: *mut wl_touch,
    mut time: uint32_t,
    mut id: int32_t,
    mut x: wl_fixed_t,
    mut y: wl_fixed_t,
) {
    let mut seat: *mut swaybar_seat = data as *mut swaybar_seat;
    let mut slot: *mut touch_slot = get_touch_slot(&raw mut (*seat).touch, id);
    if slot.is_null() {
        return;
    }
    let mut prev_progress: ::core::ffi::c_int = (((*slot).x - (*slot).start_x)
        / (*(*slot).output).width as ::core::ffi::c_double
        * 100 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int;
    (*slot).x = wl_fixed_to_double(x);
    (*slot).y = wl_fixed_to_double(y);
    let mut progress: ::core::ffi::c_int = (((*slot).x - (*slot).start_x)
        / (*(*slot).output).width as ::core::ffi::c_double
        * 100 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int;
    if abs(progress) / 20 as ::core::ffi::c_int
        != abs(prev_progress) / 20 as ::core::ffi::c_int
    {
        workspace_next(
            (*seat).bar,
            (*slot).output,
            progress - prev_progress < 0 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn wl_touch_frame(
    mut data: *mut ::core::ffi::c_void,
    mut wl_touch: *mut wl_touch,
) {}
unsafe extern "C" fn wl_touch_cancel(
    mut data: *mut ::core::ffi::c_void,
    mut wl_touch: *mut wl_touch,
) {
    let mut seat: *mut swaybar_seat = data as *mut swaybar_seat;
    let mut touch: *mut swaybar_touch = &raw mut (*seat).touch;
    let mut i: size_t = 0 as size_t;
    while i
        < ::core::mem::size_of::<[touch_slot; 16]>()
            .wrapping_div(::core::mem::size_of::<touch_slot>())
    {
        (*touch).slots[i].output = ::core::ptr::null_mut::<swaybar_output>();
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn wl_touch_shape(
    mut data: *mut ::core::ffi::c_void,
    mut wl_touch: *mut wl_touch,
    mut id: int32_t,
    mut major: wl_fixed_t,
    mut minor: wl_fixed_t,
) {}
unsafe extern "C" fn wl_touch_orientation(
    mut data: *mut ::core::ffi::c_void,
    mut wl_touch: *mut wl_touch,
    mut id: int32_t,
    mut orientation: wl_fixed_t,
) {}
static mut touch_listener: wl_touch_listener = wl_touch_listener {
    down: Some(
        wl_touch_down
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_touch,
                uint32_t,
                uint32_t,
                *mut wl_surface,
                int32_t,
                wl_fixed_t,
                wl_fixed_t,
            ) -> (),
    ),
    up: Some(
        wl_touch_up
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_touch,
                uint32_t,
                uint32_t,
                int32_t,
            ) -> (),
    ),
    motion: Some(
        wl_touch_motion
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_touch,
                uint32_t,
                int32_t,
                wl_fixed_t,
                wl_fixed_t,
            ) -> (),
    ),
    frame: Some(
        wl_touch_frame
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_touch) -> (),
    ),
    cancel: Some(
        wl_touch_cancel
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_touch) -> (),
    ),
    shape: Some(
        wl_touch_shape
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_touch,
                int32_t,
                wl_fixed_t,
                wl_fixed_t,
            ) -> (),
    ),
    orientation: Some(
        wl_touch_orientation
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut wl_touch,
                int32_t,
                wl_fixed_t,
            ) -> (),
    ),
};
unsafe extern "C" fn seat_handle_capabilities(
    mut data: *mut ::core::ffi::c_void,
    mut wl_seat: *mut wl_seat,
    mut caps: wl_seat_capability,
) {
    let mut seat: *mut swaybar_seat = data as *mut swaybar_seat;
    let mut have_pointer: bool = caps.0
        & wl_seat_capability::WL_SEAT_CAPABILITY_POINTER.0 != 0;
    let mut have_touch: bool = caps.0 & wl_seat_capability::WL_SEAT_CAPABILITY_TOUCH.0
        != 0;
    if !have_pointer && !(*seat).pointer.pointer.is_null() {
        wl_pointer_release((*seat).pointer.pointer);
        (*seat).pointer.pointer = ::core::ptr::null_mut::<wl_pointer>();
    } else if have_pointer as ::core::ffi::c_int != 0
        && (*seat).pointer.pointer.is_null()
    {
        (*seat).pointer.pointer = wl_seat_get_pointer(wl_seat);
        if (*(*seat).bar).running as ::core::ffi::c_int != 0
            && (*seat).pointer.cursor_surface.is_null()
        {
            (*seat).pointer.cursor_surface = wl_compositor_create_surface(
                (*(*seat).bar).compositor,
            );
            if !(*seat).pointer.cursor_surface.is_null() {} else {
                __assert_fail(
                    b"seat->pointer.cursor_surface\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaybar/input.c\0".as_ptr() as *const ::core::ffi::c_char,
                    497 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            };
        }
        wl_pointer_add_listener(
            (*seat).pointer.pointer,
            &raw const pointer_listener,
            seat as *mut ::core::ffi::c_void,
        );
    }
    if !have_touch && !(*seat).touch.touch.is_null() {
        wl_touch_release((*seat).touch.touch);
        (*seat).touch.touch = ::core::ptr::null_mut::<wl_touch>();
    } else if have_touch as ::core::ffi::c_int != 0 && (*seat).touch.touch.is_null() {
        (*seat).touch.touch = wl_seat_get_touch(wl_seat);
        wl_touch_add_listener(
            (*seat).touch.touch,
            &raw const touch_listener,
            seat as *mut ::core::ffi::c_void,
        );
    }
}
unsafe extern "C" fn seat_handle_name(
    mut data: *mut ::core::ffi::c_void,
    mut wl_seat: *mut wl_seat,
    mut name: *const ::core::ffi::c_char,
) {}
#[no_mangle]
pub static mut seat_listener: wl_seat_listener = unsafe {
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
        name: Some(
            seat_handle_name
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut wl_seat,
                    *const ::core::ffi::c_char,
                ) -> (),
        ),
    }
};
#[no_mangle]
pub unsafe extern "C" fn swaybar_seat_free(mut seat: *mut swaybar_seat) {
    if seat.is_null() {
        return;
    }
    if !(*seat).pointer.pointer.is_null() {
        wl_pointer_release((*seat).pointer.pointer);
    }
    if !(*seat).pointer.cursor_theme.is_null() {
        wl_cursor_theme_destroy((*seat).pointer.cursor_theme);
    }
    if !(*seat).pointer.cursor_surface.is_null() {
        wl_surface_destroy((*seat).pointer.cursor_surface);
    }
    if !(*seat).touch.touch.is_null() {
        wl_touch_release((*seat).touch.touch);
    }
    wl_seat_destroy((*seat).wl_seat);
    wl_list_remove(&raw mut (*seat).link);
    free(seat as *mut ::core::ffi::c_void);
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
