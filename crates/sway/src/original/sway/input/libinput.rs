extern "C" {
    pub type udev_device;
    pub type libinput_device;
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_renderer_impl;
    pub type wlr_buffer_impl;
    pub type wlr_drm_syncobj_timeline;
    pub type wlr_texture_impl;
    pub type wlr_backend_impl;
    pub type wlr_swapchain;
    pub type wlr_allocator;
    pub type wlr_output_layer_state;
    pub type wlr_linux_dmabuf_feedback_v1_compiled;
    pub type wlr_output_layer_feedback_event;
    pub type wlr_gamma_control_v1;
    pub type wlr_color_manager_v1;
    pub type wlr_gamma_control_manager_v1;
    pub type sway_workspace;
    pub type sway_text_node;
    pub type wlr_xwm;
    pub type wlr_xdg_toplevel;
    pub type wlr_foreign_toplevel_handle_v1;
    pub type wlr_ext_foreign_toplevel_handle_v1;
    pub type launcher_ctx;
    pub type sway_xdg_decoration;
    pub type wlr_ext_image_capture_source_v1;
    pub type sway_transaction_instruction;
    pub type wlr_ext_workspace_group_handle_v1;
    pub type sway_server;
    fn udev_device_unref(p: *mut udev_device) -> *mut udev_device;
    fn udev_device_get_property_value(
        udev_device: *mut udev_device,
        key: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn libinput_device_get_udev_device(device: *mut libinput_device) -> *mut udev_device;
    fn libinput_config_status_to_str(
        status: libinput_config_status,
    ) -> *const ::core::ffi::c_char;
    fn libinput_device_config_tap_get_finger_count(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_tap_set_enabled(
        device: *mut libinput_device,
        enable: libinput_config_tap_state,
    ) -> libinput_config_status;
    fn libinput_device_config_tap_get_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_tap_state;
    fn libinput_device_config_tap_get_default_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_tap_state;
    fn libinput_device_config_tap_set_button_map(
        device: *mut libinput_device,
        map: libinput_config_tap_button_map,
    ) -> libinput_config_status;
    fn libinput_device_config_tap_get_button_map(
        device: *mut libinput_device,
    ) -> libinput_config_tap_button_map;
    fn libinput_device_config_tap_get_default_button_map(
        device: *mut libinput_device,
    ) -> libinput_config_tap_button_map;
    fn libinput_device_config_tap_set_drag_enabled(
        device: *mut libinput_device,
        enable: libinput_config_drag_state,
    ) -> libinput_config_status;
    fn libinput_device_config_tap_get_drag_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_drag_state;
    fn libinput_device_config_tap_get_default_drag_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_drag_state;
    fn libinput_device_config_tap_set_drag_lock_enabled(
        device: *mut libinput_device,
        enable: libinput_config_drag_lock_state,
    ) -> libinput_config_status;
    fn libinput_device_config_tap_get_drag_lock_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_drag_lock_state;
    fn libinput_device_config_tap_get_default_drag_lock_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_drag_lock_state;
    fn libinput_device_config_3fg_drag_set_enabled(
        device: *mut libinput_device,
        enable: libinput_config_3fg_drag_state,
    ) -> libinput_config_status;
    fn libinput_device_config_3fg_drag_get_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_3fg_drag_state;
    fn libinput_device_config_3fg_drag_get_default_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_3fg_drag_state;
    fn libinput_device_config_calibration_has_matrix(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_calibration_set_matrix(
        device: *mut libinput_device,
        matrix: *const ::core::ffi::c_float,
    ) -> libinput_config_status;
    fn libinput_device_config_calibration_get_matrix(
        device: *mut libinput_device,
        matrix: *mut ::core::ffi::c_float,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_calibration_get_default_matrix(
        device: *mut libinput_device,
        matrix: *mut ::core::ffi::c_float,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_send_events_set_mode(
        device: *mut libinput_device,
        mode: uint32_t,
    ) -> libinput_config_status;
    fn libinput_device_config_send_events_get_mode(
        device: *mut libinput_device,
    ) -> uint32_t;
    fn libinput_device_config_send_events_get_default_mode(
        device: *mut libinput_device,
    ) -> uint32_t;
    fn libinput_device_config_accel_is_available(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_accel_set_speed(
        device: *mut libinput_device,
        speed: ::core::ffi::c_double,
    ) -> libinput_config_status;
    fn libinput_device_config_accel_get_speed(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_double;
    fn libinput_device_config_accel_get_default_speed(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_double;
    fn libinput_device_config_accel_set_profile(
        device: *mut libinput_device,
        profile: libinput_config_accel_profile,
    ) -> libinput_config_status;
    fn libinput_device_config_accel_get_profile(
        device: *mut libinput_device,
    ) -> libinput_config_accel_profile;
    fn libinput_device_config_accel_get_default_profile(
        device: *mut libinput_device,
    ) -> libinput_config_accel_profile;
    fn libinput_device_config_scroll_has_natural_scroll(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_scroll_set_natural_scroll_enabled(
        device: *mut libinput_device,
        enable: ::core::ffi::c_int,
    ) -> libinput_config_status;
    fn libinput_device_config_scroll_get_natural_scroll_enabled(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_scroll_get_default_natural_scroll_enabled(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_left_handed_is_available(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_left_handed_set(
        device: *mut libinput_device,
        left_handed: ::core::ffi::c_int,
    ) -> libinput_config_status;
    fn libinput_device_config_left_handed_get(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_left_handed_get_default(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_click_get_methods(
        device: *mut libinput_device,
    ) -> uint32_t;
    fn libinput_device_config_click_set_method(
        device: *mut libinput_device,
        method: libinput_config_click_method,
    ) -> libinput_config_status;
    fn libinput_device_config_click_get_method(
        device: *mut libinput_device,
    ) -> libinput_config_click_method;
    fn libinput_device_config_click_get_default_method(
        device: *mut libinput_device,
    ) -> libinput_config_click_method;
    fn libinput_device_config_click_set_clickfinger_button_map(
        device: *mut libinput_device,
        map: libinput_config_clickfinger_button_map,
    ) -> libinput_config_status;
    fn libinput_device_config_click_get_clickfinger_button_map(
        device: *mut libinput_device,
    ) -> libinput_config_clickfinger_button_map;
    fn libinput_device_config_click_get_default_clickfinger_button_map(
        device: *mut libinput_device,
    ) -> libinput_config_clickfinger_button_map;
    fn libinput_device_config_middle_emulation_is_available(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_middle_emulation_set_enabled(
        device: *mut libinput_device,
        enable: libinput_config_middle_emulation_state,
    ) -> libinput_config_status;
    fn libinput_device_config_middle_emulation_get_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_middle_emulation_state;
    fn libinput_device_config_middle_emulation_get_default_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_middle_emulation_state;
    fn libinput_device_config_scroll_get_methods(
        device: *mut libinput_device,
    ) -> uint32_t;
    fn libinput_device_config_scroll_set_method(
        device: *mut libinput_device,
        method: libinput_config_scroll_method,
    ) -> libinput_config_status;
    fn libinput_device_config_scroll_get_method(
        device: *mut libinput_device,
    ) -> libinput_config_scroll_method;
    fn libinput_device_config_scroll_get_default_method(
        device: *mut libinput_device,
    ) -> libinput_config_scroll_method;
    fn libinput_device_config_scroll_set_button(
        device: *mut libinput_device,
        button: uint32_t,
    ) -> libinput_config_status;
    fn libinput_device_config_scroll_get_button(
        device: *mut libinput_device,
    ) -> uint32_t;
    fn libinput_device_config_scroll_get_default_button(
        device: *mut libinput_device,
    ) -> uint32_t;
    fn libinput_device_config_scroll_set_button_lock(
        device: *mut libinput_device,
        state: libinput_config_scroll_button_lock_state,
    ) -> libinput_config_status;
    fn libinput_device_config_scroll_get_button_lock(
        device: *mut libinput_device,
    ) -> libinput_config_scroll_button_lock_state;
    fn libinput_device_config_dwt_is_available(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_dwt_set_enabled(
        device: *mut libinput_device,
        enable: libinput_config_dwt_state,
    ) -> libinput_config_status;
    fn libinput_device_config_dwt_get_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_dwt_state;
    fn libinput_device_config_dwt_get_default_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_dwt_state;
    fn libinput_device_config_dwtp_is_available(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_dwtp_set_enabled(
        device: *mut libinput_device,
        enable: libinput_config_dwtp_state,
    ) -> libinput_config_status;
    fn libinput_device_config_dwtp_get_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_dwtp_state;
    fn libinput_device_config_dwtp_get_default_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_dwtp_state;
    fn libinput_device_config_rotation_is_available(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_rotation_set_angle(
        device: *mut libinput_device,
        degrees_cw: ::core::ffi::c_uint,
    ) -> libinput_config_status;
    fn libinput_device_config_rotation_get_angle(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_uint;
    fn libinput_device_config_rotation_get_default_angle(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_uint;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn has_prefix(
        str: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    ) -> bool;
    fn output_by_name_or_id(name_or_id: *const ::core::ffi::c_char) -> *mut sway_output;
    fn input_device_get_config(device: *mut sway_input_device) -> *mut input_config;
    fn wlr_libinput_get_device_handle(
        dev: *mut wlr_input_device,
    ) -> *mut libinput_device;
    fn wlr_input_device_is_libinput(device: *mut wlr_input_device) -> bool;
    fn ipc_event_input(
        change: *const ::core::ffi::c_char,
        device: *mut sway_input_device,
    );
}
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
pub type size_t = usize;
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_status(pub ::core::ffi::c_uint);
impl libinput_config_status {
    pub const LIBINPUT_CONFIG_STATUS_SUCCESS: Self = Self(0);
    pub const LIBINPUT_CONFIG_STATUS_UNSUPPORTED: Self = Self(1);
    pub const LIBINPUT_CONFIG_STATUS_INVALID: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_tap_state(pub ::core::ffi::c_uint);
impl libinput_config_tap_state {
    pub const LIBINPUT_CONFIG_TAP_DISABLED: Self = Self(0);
    pub const LIBINPUT_CONFIG_TAP_ENABLED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_tap_button_map(pub ::core::ffi::c_uint);
impl libinput_config_tap_button_map {
    pub const LIBINPUT_CONFIG_TAP_MAP_LRM: Self = Self(0);
    pub const LIBINPUT_CONFIG_TAP_MAP_LMR: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_clickfinger_button_map(pub ::core::ffi::c_uint);
impl libinput_config_clickfinger_button_map {
    pub const LIBINPUT_CONFIG_CLICKFINGER_MAP_LRM: Self = Self(0);
    pub const LIBINPUT_CONFIG_CLICKFINGER_MAP_LMR: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_drag_state(pub ::core::ffi::c_uint);
impl libinput_config_drag_state {
    pub const LIBINPUT_CONFIG_DRAG_DISABLED: Self = Self(0);
    pub const LIBINPUT_CONFIG_DRAG_ENABLED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_drag_lock_state(pub ::core::ffi::c_uint);
impl libinput_config_drag_lock_state {
    pub const LIBINPUT_CONFIG_DRAG_LOCK_DISABLED: Self = Self(0);
    pub const LIBINPUT_CONFIG_DRAG_LOCK_ENABLED_TIMEOUT: Self = Self(1);
    pub const LIBINPUT_CONFIG_DRAG_LOCK_ENABLED: Self = Self(1);
    pub const LIBINPUT_CONFIG_DRAG_LOCK_ENABLED_STICKY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_3fg_drag_state(pub ::core::ffi::c_uint);
impl libinput_config_3fg_drag_state {
    pub const LIBINPUT_CONFIG_3FG_DRAG_DISABLED: Self = Self(0);
    pub const LIBINPUT_CONFIG_3FG_DRAG_ENABLED_3FG: Self = Self(1);
    pub const LIBINPUT_CONFIG_3FG_DRAG_ENABLED_4FG: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_send_events_mode(pub ::core::ffi::c_uint);
impl libinput_config_send_events_mode {
    pub const LIBINPUT_CONFIG_SEND_EVENTS_ENABLED: Self = Self(0);
    pub const LIBINPUT_CONFIG_SEND_EVENTS_DISABLED: Self = Self(1);
    pub const LIBINPUT_CONFIG_SEND_EVENTS_DISABLED_ON_EXTERNAL_MOUSE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_accel_profile(pub ::core::ffi::c_uint);
impl libinput_config_accel_profile {
    pub const LIBINPUT_CONFIG_ACCEL_PROFILE_NONE: Self = Self(0);
    pub const LIBINPUT_CONFIG_ACCEL_PROFILE_FLAT: Self = Self(1);
    pub const LIBINPUT_CONFIG_ACCEL_PROFILE_ADAPTIVE: Self = Self(2);
    pub const LIBINPUT_CONFIG_ACCEL_PROFILE_CUSTOM: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_click_method(pub ::core::ffi::c_uint);
impl libinput_config_click_method {
    pub const LIBINPUT_CONFIG_CLICK_METHOD_NONE: Self = Self(0);
    pub const LIBINPUT_CONFIG_CLICK_METHOD_BUTTON_AREAS: Self = Self(1);
    pub const LIBINPUT_CONFIG_CLICK_METHOD_CLICKFINGER: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_middle_emulation_state(pub ::core::ffi::c_uint);
impl libinput_config_middle_emulation_state {
    pub const LIBINPUT_CONFIG_MIDDLE_EMULATION_DISABLED: Self = Self(0);
    pub const LIBINPUT_CONFIG_MIDDLE_EMULATION_ENABLED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_scroll_method(pub ::core::ffi::c_uint);
impl libinput_config_scroll_method {
    pub const LIBINPUT_CONFIG_SCROLL_NO_SCROLL: Self = Self(0);
    pub const LIBINPUT_CONFIG_SCROLL_2FG: Self = Self(1);
    pub const LIBINPUT_CONFIG_SCROLL_EDGE: Self = Self(2);
    pub const LIBINPUT_CONFIG_SCROLL_ON_BUTTON_DOWN: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_scroll_button_lock_state(pub ::core::ffi::c_uint);
impl libinput_config_scroll_button_lock_state {
    pub const LIBINPUT_CONFIG_SCROLL_BUTTON_LOCK_DISABLED: Self = Self(0);
    pub const LIBINPUT_CONFIG_SCROLL_BUTTON_LOCK_ENABLED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_dwt_state(pub ::core::ffi::c_uint);
impl libinput_config_dwt_state {
    pub const LIBINPUT_CONFIG_DWT_DISABLED: Self = Self(0);
    pub const LIBINPUT_CONFIG_DWT_ENABLED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct libinput_config_dwtp_state(pub ::core::ffi::c_uint);
impl libinput_config_dwtp_state {
    pub const LIBINPUT_CONFIG_DWTP_DISABLED: Self = Self(0);
    pub const LIBINPUT_CONFIG_DWTP_ENABLED: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_object {
    pub interface: *const wl_interface,
    pub implementation: *const ::core::ffi::c_void,
    pub id: uint32_t,
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
pub struct wl_array {
    pub size: size_t,
    pub alloc: size_t,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_listener {
    pub link: wl_list,
    pub notify: wl_notify_func_t,
}
pub type wl_notify_func_t = Option<
    unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_resource {
    pub object: wl_object,
    pub destroy: wl_resource_destroy_func_t,
    pub link: wl_list,
    pub destroy_signal: wl_signal,
    pub client: *mut wl_client,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
pub type wl_resource_destroy_func_t = Option<
    unsafe extern "C" fn(*mut wl_resource) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: ::core::ffi::c_long,
    pub numRects: ::core::ffi::c_long,
}
pub type pixman_region32_data_t = pixman_region32_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
pub type pixman_box32_t = pixman_box32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_t = pixman_region32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_output_transform(pub ::core::ffi::c_uint);
impl wl_output_transform {
    pub const WL_OUTPUT_TRANSFORM_NORMAL: Self = Self(0);
    pub const WL_OUTPUT_TRANSFORM_90: Self = Self(1);
    pub const WL_OUTPUT_TRANSFORM_180: Self = Self(2);
    pub const WL_OUTPUT_TRANSFORM_270: Self = Self(3);
    pub const WL_OUTPUT_TRANSFORM_FLIPPED: Self = Self(4);
    pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: Self = Self(5);
    pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: Self = Self(6);
    pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon_set {
    pub WLR_PRIVATE: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub addons: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon {
    pub r#impl: *const wlr_addon_interface,
    pub WLR_PRIVATE: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub owner: *const ::core::ffi::c_void,
    pub link: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon_interface {
    pub name: *const ::core::ffi::c_char,
    pub destroy: Option<unsafe extern "C" fn(*mut wlr_addon) -> ()>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_color_named_primaries(pub ::core::ffi::c_uint);
impl wlr_color_named_primaries {
    pub const WLR_COLOR_NAMED_PRIMARIES_SRGB: Self = Self(1);
    pub const WLR_COLOR_NAMED_PRIMARIES_BT2020: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_color_transfer_function(pub ::core::ffi::c_uint);
impl wlr_color_transfer_function {
    pub const WLR_COLOR_TRANSFER_FUNCTION_SRGB: Self = Self(1);
    pub const WLR_COLOR_TRANSFER_FUNCTION_ST2084_PQ: Self = Self(2);
    pub const WLR_COLOR_TRANSFER_FUNCTION_EXT_LINEAR: Self = Self(4);
    pub const WLR_COLOR_TRANSFER_FUNCTION_GAMMA22: Self = Self(8);
    pub const WLR_COLOR_TRANSFER_FUNCTION_BT1886: Self = Self(16);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_color_encoding(pub ::core::ffi::c_uint);
impl wlr_color_encoding {
    pub const WLR_COLOR_ENCODING_NONE: Self = Self(0);
    pub const WLR_COLOR_ENCODING_IDENTITY: Self = Self(1);
    pub const WLR_COLOR_ENCODING_BT709: Self = Self(2);
    pub const WLR_COLOR_ENCODING_FCC: Self = Self(4);
    pub const WLR_COLOR_ENCODING_BT601: Self = Self(8);
    pub const WLR_COLOR_ENCODING_SMPTE240: Self = Self(16);
    pub const WLR_COLOR_ENCODING_BT2020: Self = Self(32);
    pub const WLR_COLOR_ENCODING_BT2020_CL: Self = Self(64);
    pub const WLR_COLOR_ENCODING_ICTCP: Self = Self(128);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_color_range(pub ::core::ffi::c_uint);
impl wlr_color_range {
    pub const WLR_COLOR_RANGE_NONE: Self = Self(0);
    pub const WLR_COLOR_RANGE_LIMITED: Self = Self(1);
    pub const WLR_COLOR_RANGE_FULL: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_color_cie1931_xy {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_color_primaries {
    pub red: wlr_color_cie1931_xy,
    pub green: wlr_color_cie1931_xy,
    pub blue: wlr_color_cie1931_xy,
    pub white: wlr_color_cie1931_xy,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_color_transform_type(pub ::core::ffi::c_uint);
impl wlr_color_transform_type {
    pub const WLR_COLOR_TRANSFORM_INVERSE_EOTF: Self = Self(0);
    pub const WLR_COLOR_TRANSFORM_LCMS2: Self = Self(1);
    pub const WLR_COLOR_TRANSFORM_LUT_3X1D: Self = Self(2);
    pub const WLR_COLOR_TRANSFORM_MATRIX: Self = Self(3);
    pub const WLR_COLOR_TRANSFORM_PIPELINE: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_color_transform {
    pub r#type: wlr_color_transform_type,
    pub WLR_PRIVATE: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub ref_count: ::core::ffi::c_int,
    pub addons: wlr_addon_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_box {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_fbox {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_renderer {
    pub render_buffer_caps: uint32_t,
    pub color_encodings: uint32_t,
    pub events: C2Rust_Unnamed_4,
    pub features: C2Rust_Unnamed_3,
    pub WLR_PRIVATE: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub r#impl: *const wlr_renderer_impl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub input_color_transform: bool,
    pub output_color_transform: bool,
    pub timeline: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub destroy: wl_signal,
    pub lost: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_buffer {
    pub r#impl: *const wlr_buffer_impl,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub dropped: bool,
    pub n_locks: size_t,
    pub accessing_data_ptr: bool,
    pub events: C2Rust_Unnamed_5,
    pub addons: wlr_addon_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub destroy: wl_signal,
    pub release: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_scale_filter_mode(pub ::core::ffi::c_uint);
impl wlr_scale_filter_mode {
    pub const WLR_SCALE_FILTER_BILINEAR: Self = Self(0);
    pub const WLR_SCALE_FILTER_NEAREST: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_texture {
    pub r#impl: *const wlr_texture_impl,
    pub width: uint32_t,
    pub height: uint32_t,
    pub renderer: *mut wlr_renderer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_dmabuf_attributes {
    pub width: int32_t,
    pub height: int32_t,
    pub format: uint32_t,
    pub modifier: uint64_t,
    pub n_planes: ::core::ffi::c_int,
    pub offset: [uint32_t; 4],
    pub stride: [uint32_t; 4],
    pub fd: [::core::ffi::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_backend {
    pub r#impl: *const wlr_backend_impl,
    pub buffer_caps: uint32_t,
    pub features: C2Rust_Unnamed_7,
    pub events: C2Rust_Unnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub timeline: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_drm_format_set {
    pub len: size_t,
    pub capacity: size_t,
    pub formats: *mut wlr_drm_format,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_drm_format {
    pub format: uint32_t,
    pub len: size_t,
    pub capacity: size_t,
    pub modifiers: *mut uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_client_buffer {
    pub base: wlr_buffer,
    pub texture: *mut wlr_texture,
    pub source: *mut wlr_buffer,
    pub WLR_PRIVATE: C2Rust_Unnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub source_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
    pub n_ignore_locks: size_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_output_mode_aspect_ratio(pub ::core::ffi::c_uint);
impl wlr_output_mode_aspect_ratio {
    pub const WLR_OUTPUT_MODE_ASPECT_RATIO_NONE: Self = Self(0);
    pub const WLR_OUTPUT_MODE_ASPECT_RATIO_4_3: Self = Self(1);
    pub const WLR_OUTPUT_MODE_ASPECT_RATIO_16_9: Self = Self(2);
    pub const WLR_OUTPUT_MODE_ASPECT_RATIO_64_27: Self = Self(3);
    pub const WLR_OUTPUT_MODE_ASPECT_RATIO_256_135: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_mode {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub preferred: bool,
    pub picture_aspect_ratio: wlr_output_mode_aspect_ratio,
    pub link: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_cursor {
    pub output: *mut wlr_output,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub enabled: bool,
    pub visible: bool,
    pub width: uint32_t,
    pub height: uint32_t,
    pub src_box: wlr_fbox,
    pub transform: wl_output_transform,
    pub hotspot_x: int32_t,
    pub hotspot_y: int32_t,
    pub texture: *mut wlr_texture,
    pub own_texture: bool,
    pub wait_timeline: *mut wlr_drm_syncobj_timeline,
    pub wait_point: uint64_t,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub renderer_destroy: wl_listener,
    pub color_transform: *mut wlr_color_transform,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output {
    pub r#impl: *const wlr_output_impl,
    pub backend: *mut wlr_backend,
    pub event_loop: *mut wl_event_loop,
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub name: *mut ::core::ffi::c_char,
    pub description: *mut ::core::ffi::c_char,
    pub make: *mut ::core::ffi::c_char,
    pub model: *mut ::core::ffi::c_char,
    pub serial: *mut ::core::ffi::c_char,
    pub phys_width: int32_t,
    pub phys_height: int32_t,
    pub default_primaries: *const wlr_color_primaries,
    pub modes: wl_list,
    pub current_mode: *mut wlr_output_mode,
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
    pub supported_primaries: uint32_t,
    pub supported_transfer_functions: uint32_t,
    pub enabled: bool,
    pub scale: ::core::ffi::c_float,
    pub subpixel: wl_output_subpixel,
    pub transform: wl_output_transform,
    pub adaptive_sync_status: wlr_output_adaptive_sync_status,
    pub render_format: uint32_t,
    pub color_encoding: wlr_color_encoding,
    pub color_range: wlr_color_range,
    pub image_description: *const wlr_output_image_description,
    pub adaptive_sync_supported: bool,
    pub needs_frame: bool,
    pub frame_pending: bool,
    pub non_desktop: bool,
    pub commit_seq: uint32_t,
    pub events: C2Rust_Unnamed_12,
    pub idle_frame: *mut wl_event_source,
    pub idle_done: *mut wl_event_source,
    pub attach_render_locks: ::core::ffi::c_int,
    pub cursors: wl_list,
    pub hardware_cursor: *mut wlr_output_cursor,
    pub cursor_swapchain: *mut wlr_swapchain,
    pub cursor_front_buffer: *mut wlr_buffer,
    pub software_cursor_locks: ::core::ffi::c_int,
    pub layers: wl_list,
    pub allocator: *mut wlr_allocator,
    pub renderer: *mut wlr_renderer,
    pub swapchain: *mut wlr_swapchain,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub display_destroy: wl_listener,
    pub image_description_value: wlr_output_image_description,
    pub color_transform: *mut wlr_color_transform,
    pub default_primaries_value: wlr_color_primaries,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_image_description {
    pub primaries: wlr_color_named_primaries,
    pub transfer_function: wlr_color_transfer_function,
    pub mastering_display_primaries: wlr_color_primaries,
    pub mastering_luminance: C2Rust_Unnamed_11,
    pub max_cll: ::core::ffi::c_double,
    pub max_fall: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub min: ::core::ffi::c_double,
    pub max: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
    pub frame: wl_signal,
    pub damage: wl_signal,
    pub needs_frame: wl_signal,
    pub precommit: wl_signal,
    pub commit: wl_signal,
    pub present: wl_signal,
    pub bind: wl_signal,
    pub description: wl_signal,
    pub request_state: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_output_adaptive_sync_status(pub ::core::ffi::c_uint);
impl wlr_output_adaptive_sync_status {
    pub const WLR_OUTPUT_ADAPTIVE_SYNC_DISABLED: Self = Self(0);
    pub const WLR_OUTPUT_ADAPTIVE_SYNC_ENABLED: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_impl {
    pub set_cursor: Option<
        unsafe extern "C" fn(
            *mut wlr_output,
            *mut wlr_buffer,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> bool,
    >,
    pub move_cursor: Option<
        unsafe extern "C" fn(
            *mut wlr_output,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> bool,
    >,
    pub destroy: Option<unsafe extern "C" fn(*mut wlr_output) -> ()>,
    pub test: Option<
        unsafe extern "C" fn(*mut wlr_output, *const wlr_output_state) -> bool,
    >,
    pub commit: Option<
        unsafe extern "C" fn(*mut wlr_output, *const wlr_output_state) -> bool,
    >,
    pub get_gamma_size: Option<unsafe extern "C" fn(*mut wlr_output) -> size_t>,
    pub get_cursor_formats: Option<
        unsafe extern "C" fn(*mut wlr_output, uint32_t) -> *const wlr_drm_format_set,
    >,
    pub get_cursor_sizes: Option<
        unsafe extern "C" fn(
            *mut wlr_output,
            *mut size_t,
        ) -> *const wlr_output_cursor_size,
    >,
    pub get_primary_formats: Option<
        unsafe extern "C" fn(*mut wlr_output, uint32_t) -> *const wlr_drm_format_set,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_cursor_size {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_state {
    pub committed: uint32_t,
    pub allow_reconfiguration: bool,
    pub damage: pixman_region32_t,
    pub enabled: bool,
    pub scale: ::core::ffi::c_float,
    pub transform: wl_output_transform,
    pub adaptive_sync_enabled: bool,
    pub render_format: uint32_t,
    pub subpixel: wl_output_subpixel,
    pub buffer: *mut wlr_buffer,
    pub buffer_src_box: wlr_fbox,
    pub buffer_dst_box: wlr_box,
    pub tearing_page_flip: bool,
    pub color_encoding: wlr_color_encoding,
    pub color_range: wlr_color_range,
    pub mode_type: wlr_output_state_mode_type,
    pub mode: *mut wlr_output_mode,
    pub custom_mode: C2Rust_Unnamed_13,
    pub layers: *mut wlr_output_layer_state,
    pub layers_len: size_t,
    pub wait_timeline: *mut wlr_drm_syncobj_timeline,
    pub wait_point: uint64_t,
    pub signal_timeline: *mut wlr_drm_syncobj_timeline,
    pub signal_point: uint64_t,
    pub color_transform: *mut wlr_color_transform,
    pub image_description: *mut wlr_output_image_description,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub width: int32_t,
    pub height: int32_t,
    pub refresh: int32_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_output_state_mode_type(pub ::core::ffi::c_uint);
impl wlr_output_state_mode_type {
    pub const WLR_OUTPUT_STATE_MODE_FIXED: Self = Self(0);
    pub const WLR_OUTPUT_STATE_MODE_CUSTOM: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_device {
    pub r#type: wlr_input_device_type,
    pub name: *mut ::core::ffi::c_char,
    pub events: C2Rust_Unnamed_14,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub destroy: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_input_device_type(pub ::core::ffi::c_uint);
impl wlr_input_device_type {
    pub const WLR_INPUT_DEVICE_KEYBOARD: Self = Self(0);
    pub const WLR_INPUT_DEVICE_POINTER: Self = Self(1);
    pub const WLR_INPUT_DEVICE_TOUCH: Self = Self(2);
    pub const WLR_INPUT_DEVICE_TABLET: Self = Self(3);
    pub const WLR_INPUT_DEVICE_TABLET_PAD: Self = Self(4);
    pub const WLR_INPUT_DEVICE_SWITCH: Self = Self(5);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_surface {
    pub resource: *mut wl_resource,
    pub compositor: *mut wlr_compositor,
    pub buffer: *mut wlr_client_buffer,
    pub buffer_damage: pixman_region32_t,
    pub opaque_region: pixman_region32_t,
    pub input_region: pixman_region32_t,
    pub current: wlr_surface_state,
    pub pending: wlr_surface_state,
    pub cached: wl_list,
    pub mapped: bool,
    pub role: *const wlr_surface_role,
    pub role_resource: *mut wl_resource,
    pub events: C2Rust_Unnamed_17,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_16,
    pub unmap_commit: bool,
    pub opaque: bool,
    pub handling_commit: bool,
    pub pending_rejected: bool,
    pub preferred_buffer_scale: int32_t,
    pub preferred_buffer_transform_sent: bool,
    pub preferred_buffer_transform: wl_output_transform,
    pub synced: wl_list,
    pub synced_len: size_t,
    pub pending_buffer_resource: *mut wl_resource,
    pub pending_buffer_resource_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub client_commit: wl_signal,
    pub commit: wl_signal,
    pub map: wl_signal,
    pub unmap: wl_signal,
    pub new_subsurface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_surface_role {
    pub name: *const ::core::ffi::c_char,
    pub no_object: bool,
    pub client_commit: Option<unsafe extern "C" fn(*mut wlr_surface) -> ()>,
    pub commit: Option<unsafe extern "C" fn(*mut wlr_surface) -> ()>,
    pub map: Option<unsafe extern "C" fn(*mut wlr_surface) -> ()>,
    pub unmap: Option<unsafe extern "C" fn(*mut wlr_surface) -> ()>,
    pub destroy: Option<unsafe extern "C" fn(*mut wlr_surface) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_surface_state {
    pub committed: uint32_t,
    pub seq: uint32_t,
    pub buffer: *mut wlr_buffer,
    pub dx: int32_t,
    pub dy: int32_t,
    pub surface_damage: pixman_region32_t,
    pub buffer_damage: pixman_region32_t,
    pub opaque: pixman_region32_t,
    pub input: pixman_region32_t,
    pub transform: wl_output_transform,
    pub scale: int32_t,
    pub frame_callback_list: wl_list,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
    pub subsurfaces_below: wl_list,
    pub subsurfaces_above: wl_list,
    pub viewport: C2Rust_Unnamed_18,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub has_src: bool,
    pub has_dst: bool,
    pub src: wlr_fbox,
    pub dst_width: ::core::ffi::c_int,
    pub dst_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_compositor {
    pub global: *mut wl_global,
    pub renderer: *mut wlr_renderer,
    pub events: C2Rust_Unnamed_20,
    pub WLR_PRIVATE: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_damage_ring {
    pub current: pixman_region32_t,
    pub WLR_PRIVATE: C2Rust_Unnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_23,
    pub WLR_PRIVATE: C2Rust_Unnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub default_feedback: *mut wlr_linux_dmabuf_feedback_v1_compiled,
    pub default_formats: wlr_drm_format_set,
    pub surfaces: wl_list,
    pub main_device_fd: ::core::ffi::c_int,
    pub display_destroy: wl_listener,
    pub check_dmabuf_callback: Option<
        unsafe extern "C" fn(
            *mut wlr_dmabuf_attributes,
            *mut ::core::ffi::c_void,
        ) -> bool,
    >,
    pub check_dmabuf_callback_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_feedback_v1_init_options {
    pub main_renderer: *mut wlr_renderer,
    pub scanout_primary_output: *mut wlr_output,
    pub output_layer_feedback_event: *const wlr_output_layer_feedback_event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_layout {
    pub outputs: wl_list,
    pub display: *mut wl_display,
    pub events: C2Rust_Unnamed_25,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_node {
    pub r#type: wlr_scene_node_type,
    pub parent: *mut wlr_scene_tree,
    pub link: wl_list,
    pub enabled: bool,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub events: C2Rust_Unnamed_27,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub bounding_box: wlr_box,
    pub bounding_box_valid: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_scene_node_type(pub ::core::ffi::c_uint);
impl wlr_scene_node_type {
    pub const WLR_SCENE_NODE_TREE: Self = Self(0);
    pub const WLR_SCENE_NODE_RECT: Self = Self(1);
    pub const WLR_SCENE_NODE_BUFFER: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_buffer {
    pub node: wlr_scene_node,
    pub buffer: *mut wlr_buffer,
    pub events: C2Rust_Unnamed_33,
    pub point_accepts_input: wlr_scene_buffer_point_accepts_input_func_t,
    pub primary_output: *mut wlr_scene_output,
    pub opacity: ::core::ffi::c_float,
    pub filter_mode: wlr_scale_filter_mode,
    pub src_box: wlr_fbox,
    pub dst_width: ::core::ffi::c_int,
    pub dst_height: ::core::ffi::c_int,
    pub transform: wl_output_transform,
    pub opaque_region: pixman_region32_t,
    pub transfer_function: wlr_color_transfer_function,
    pub primaries: wlr_color_named_primaries,
    pub color_encoding: wlr_color_encoding,
    pub color_range: wlr_color_range,
    pub WLR_PRIVATE: C2Rust_Unnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
    pub active_outputs: uint64_t,
    pub texture: *mut wlr_texture,
    pub prev_feedback_options: wlr_linux_dmabuf_feedback_v1_init_options,
    pub own_buffer: bool,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
    pub buffer_is_opaque: bool,
    pub wait_timeline: *mut wlr_drm_syncobj_timeline,
    pub wait_point: uint64_t,
    pub buffer_release: wl_listener,
    pub renderer_destroy: wl_listener,
    pub is_single_pixel_buffer: bool,
    pub single_pixel_buffer_color: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_output {
    pub output: *mut wlr_output,
    pub link: wl_list,
    pub scene: *mut wlr_scene,
    pub addon: wlr_addon,
    pub damage_ring: wlr_damage_ring,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub events: C2Rust_Unnamed_31,
    pub WLR_PRIVATE: C2Rust_Unnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
    pub pending_commit_damage: pixman_region32_t,
    pub index: uint8_t,
    pub dmabuf_feedback_debounce: uint8_t,
    pub prev_scanout: bool,
    pub gamma_lut_changed: bool,
    pub gamma_lut: *mut wlr_gamma_control_v1,
    pub gamma_lut_color_transform: *mut wlr_color_transform,
    pub prev_gamma_lut_color_transform: *mut wlr_color_transform,
    pub prev_supplied_color_transform: *mut wlr_color_transform,
    pub combined_color_transform: *mut wlr_color_transform,
    pub output_commit: wl_listener,
    pub output_damage: wl_listener,
    pub output_needs_frame: wl_listener,
    pub damage_highlight_regions: wl_list,
    pub render_list: wl_array,
    pub in_timeline: *mut wlr_drm_syncobj_timeline,
    pub in_point: uint64_t,
    pub out_timeline: *mut wlr_drm_syncobj_timeline,
    pub out_point: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene {
    pub tree: wlr_scene_tree,
    pub outputs: wl_list,
    pub linux_dmabuf_v1: *mut wlr_linux_dmabuf_v1,
    pub gamma_control_manager_v1: *mut wlr_gamma_control_manager_v1,
    pub color_manager_v1: *mut wlr_color_manager_v1,
    pub restack_xwayland_surfaces: bool,
    pub WLR_PRIVATE: C2Rust_Unnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
    pub linux_dmabuf_v1_destroy: wl_listener,
    pub gamma_control_manager_v1_destroy: wl_listener,
    pub gamma_control_manager_v1_set_gamma: wl_listener,
    pub color_manager_v1_destroy: wl_listener,
    pub debug_damage_option: wlr_scene_debug_damage_option,
    pub direct_scanout: bool,
    pub calculate_visibility: bool,
    pub highlight_transparent_region: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_scene_debug_damage_option(pub ::core::ffi::c_uint);
impl wlr_scene_debug_damage_option {
    pub const WLR_SCENE_DEBUG_DAMAGE_NONE: Self = Self(0);
    pub const WLR_SCENE_DEBUG_DAMAGE_RERENDER: Self = Self(1);
    pub const WLR_SCENE_DEBUG_DAMAGE_HIGHLIGHT: Self = Self(2);
}
pub type wlr_scene_buffer_point_accepts_input_func_t = Option<
    unsafe extern "C" fn(
        *mut wlr_scene_buffer,
        *mut ::core::ffi::c_double,
        *mut ::core::ffi::c_double,
    ) -> bool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
    pub outputs_update: wl_signal,
    pub output_sample: wl_signal,
    pub frame_done: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_rect {
    pub node: wlr_scene_node,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub color: [::core::ffi::c_float; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_root {
    pub node: sway_node,
    pub output_layout: *mut wlr_output_layout,
    pub root_scene: *mut wlr_scene,
    pub staging: *mut wlr_scene_tree,
    pub layer_tree: *mut wlr_scene_tree,
    pub layers: C2Rust_Unnamed_45,
    pub all_outputs: wl_list,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
    pub outputs: *mut list_t,
    pub non_desktop_outputs: *mut list_t,
    pub scratchpad: *mut list_t,
    pub fallback_output: *mut sway_output,
    pub fullscreen_global: *mut sway_container,
    pub events: C2Rust_Unnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_37,
    pub border: C2Rust_Unnamed_36,
    pub content_tree: *mut wlr_scene_tree,
    pub current: sway_container_state,
    pub pending: sway_container_state,
    pub title: *mut ::core::ffi::c_char,
    pub title_width: ::core::ffi::c_int,
    pub title_format: *mut ::core::ffi::c_char,
    pub prev_split_layout: sway_container_layout,
    pub is_sticky: bool,
    pub saved_x: ::core::ffi::c_double,
    pub saved_y: ::core::ffi::c_double,
    pub saved_width: ::core::ffi::c_double,
    pub saved_height: ::core::ffi::c_double,
    pub saved_border: sway_container_border,
    pub width_fraction: ::core::ffi::c_double,
    pub height_fraction: ::core::ffi::c_double,
    pub child_total_width: ::core::ffi::c_double,
    pub child_total_height: ::core::ffi::c_double,
    pub scratchpad: bool,
    pub transform: wlr_box,
    pub alpha: ::core::ffi::c_float,
    pub marks: *mut list_t,
    pub events: C2Rust_Unnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
    pub destroy: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_container_border(pub ::core::ffi::c_uint);
impl sway_container_border {
    pub const B_NONE: Self = Self(0);
    pub const B_PIXEL: Self = Self(1);
    pub const B_NORMAL: Self = Self(2);
    pub const B_CSD: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_container_layout(pub ::core::ffi::c_uint);
impl sway_container_layout {
    pub const L_NONE: Self = Self(0);
    pub const L_HORIZ: Self = Self(1);
    pub const L_VERT: Self = Self(2);
    pub const L_STACKED: Self = Self(3);
    pub const L_TABBED: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container_state {
    pub layout: sway_container_layout,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
    pub fullscreen_mode: sway_fullscreen_mode,
    pub workspace: *mut sway_workspace,
    pub parent: *mut sway_container,
    pub children: *mut list_t,
    pub focused_inactive_child: *mut sway_container,
    pub focused: bool,
    pub border: sway_container_border,
    pub border_thickness: ::core::ffi::c_int,
    pub border_top: bool,
    pub border_bottom: bool,
    pub border_left: bool,
    pub border_right: bool,
    pub content_x: ::core::ffi::c_double,
    pub content_y: ::core::ffi::c_double,
    pub content_width: ::core::ffi::c_double,
    pub content_height: ::core::ffi::c_double,
    pub formatted_title: *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_fullscreen_mode(pub ::core::ffi::c_uint);
impl sway_fullscreen_mode {
    pub const FULLSCREEN_NONE: Self = Self(0);
    pub const FULLSCREEN_WORKSPACE: Self = Self(1);
    pub const FULLSCREEN_GLOBAL: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_37 {
    pub tree: *mut wlr_scene_tree,
    pub border: *mut wlr_scene_tree,
    pub background: *mut wlr_scene_tree,
    pub title_text: *mut sway_text_node,
    pub marks_text: *mut sway_text_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_view {
    pub r#type: sway_view_type,
    pub r#impl: *const sway_view_impl,
    pub scene_tree: *mut wlr_scene_tree,
    pub content_tree: *mut wlr_scene_tree,
    pub saved_surface_tree: *mut wlr_scene_tree,
    pub output_handler: *mut wlr_scene_buffer,
    pub outputs_update: wl_listener,
    pub image_capture_scene: *mut wlr_scene,
    pub image_capture_source: *mut wlr_ext_image_capture_source_v1,
    pub container: *mut sway_container,
    pub surface: *mut wlr_surface,
    pub xdg_decoration: *mut sway_xdg_decoration,
    pub pid: pid_t,
    pub ctx: *mut launcher_ctx,
    pub natural_width: ::core::ffi::c_int,
    pub natural_height: ::core::ffi::c_int,
    pub using_csd: bool,
    pub urgent: timespec,
    pub allow_request_urgent: bool,
    pub urgent_timer: *mut wl_event_source,
    pub geometry: wlr_box,
    pub ext_foreign_toplevel: *mut wlr_ext_foreign_toplevel_handle_v1,
    pub foreign_toplevel: *mut wlr_foreign_toplevel_handle_v1,
    pub foreign_activate_request: wl_listener,
    pub foreign_fullscreen_request: wl_listener,
    pub foreign_close_request: wl_listener,
    pub foreign_destroy: wl_listener,
    pub destroying: bool,
    pub executed_criteria: *mut list_t,
    pub c2rust_unnamed: C2Rust_Unnamed_39,
    pub events: C2Rust_Unnamed_38,
    pub max_render_time: ::core::ffi::c_int,
    pub shortcuts_inhibit: seat_config_shortcuts_inhibit,
    pub tearing_mode: sway_view_tearing_mode,
    pub tearing_hint: wp_tearing_control_v1_presentation_hint,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wp_tearing_control_v1_presentation_hint(pub ::core::ffi::c_uint);
impl wp_tearing_control_v1_presentation_hint {
    pub const WP_TEARING_CONTROL_V1_PRESENTATION_HINT_VSYNC: Self = Self(0);
    pub const WP_TEARING_CONTROL_V1_PRESENTATION_HINT_ASYNC: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_view_tearing_mode(pub ::core::ffi::c_uint);
impl sway_view_tearing_mode {
    pub const TEARING_OVERRIDE_FALSE: Self = Self(0);
    pub const TEARING_OVERRIDE_TRUE: Self = Self(1);
    pub const TEARING_WINDOW_HINT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct seat_config_shortcuts_inhibit(pub ::core::ffi::c_uint);
impl seat_config_shortcuts_inhibit {
    pub const SHORTCUTS_INHIBIT_DEFAULT: Self = Self(0);
    pub const SHORTCUTS_INHIBIT_ENABLE: Self = Self(1);
    pub const SHORTCUTS_INHIBIT_DISABLE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_39 {
    pub wlr_xdg_toplevel: *mut wlr_xdg_toplevel,
    pub wlr_xwayland_surface: *mut wlr_xwayland_surface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xwayland_surface {
    pub window_id: xcb_window_t,
    pub xwm: *mut wlr_xwm,
    pub surface_id: uint32_t,
    pub serial: uint64_t,
    pub link: wl_list,
    pub stack_link: wl_list,
    pub unpaired_link: wl_list,
    pub surface: *mut wlr_surface,
    pub surface_addon: wlr_addon,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub override_redirect: bool,
    pub opacity: ::core::ffi::c_float,
    pub title: *mut ::core::ffi::c_char,
    pub class: *mut ::core::ffi::c_char,
    pub instance: *mut ::core::ffi::c_char,
    pub role: *mut ::core::ffi::c_char,
    pub startup_id: *mut ::core::ffi::c_char,
    pub pid: pid_t,
    pub children: wl_list,
    pub parent: *mut wlr_xwayland_surface,
    pub parent_link: wl_list,
    pub window_type: *mut xcb_atom_t,
    pub window_type_len: size_t,
    pub protocols: *mut xcb_atom_t,
    pub protocols_len: size_t,
    pub decorations: uint32_t,
    pub hints: *mut xcb_icccm_wm_hints_t,
    pub size_hints: *mut xcb_size_hints_t,
    pub strut_partial: *mut xcb_ewmh_wm_strut_partial_t,
    pub pinging: bool,
    pub ping_timer: *mut wl_event_source,
    pub modal: bool,
    pub fullscreen: bool,
    pub maximized_vert: bool,
    pub maximized_horz: bool,
    pub minimized: bool,
    pub withdrawn: bool,
    pub sticky: bool,
    pub shaded: bool,
    pub skip_taskbar: bool,
    pub skip_pager: bool,
    pub above: bool,
    pub below: bool,
    pub demands_attention: bool,
    pub has_alpha: bool,
    pub has_input_shape: bool,
    pub input_shape: pixman_region32_t,
    pub events: C2Rust_Unnamed_41,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_40 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_41 {
    pub destroy: wl_signal,
    pub request_configure: wl_signal,
    pub request_move: wl_signal,
    pub request_resize: wl_signal,
    pub request_minimize: wl_signal,
    pub request_maximize: wl_signal,
    pub request_fullscreen: wl_signal,
    pub request_activate: wl_signal,
    pub request_close: wl_signal,
    pub request_sticky: wl_signal,
    pub request_shaded: wl_signal,
    pub request_skip_taskbar: wl_signal,
    pub request_skip_pager: wl_signal,
    pub request_above: wl_signal,
    pub request_below: wl_signal,
    pub request_demands_attention: wl_signal,
    pub associate: wl_signal,
    pub dissociate: wl_signal,
    pub set_title: wl_signal,
    pub set_class: wl_signal,
    pub set_role: wl_signal,
    pub set_parent: wl_signal,
    pub set_startup_id: wl_signal,
    pub set_window_type: wl_signal,
    pub set_hints: wl_signal,
    pub set_size_hints: wl_signal,
    pub set_decorations: wl_signal,
    pub set_strut_partial: wl_signal,
    pub set_override_redirect: wl_signal,
    pub set_geometry: wl_signal,
    pub set_opacity: wl_signal,
    pub set_icon: wl_signal,
    pub set_input_shape: wl_signal,
    pub focus_in: wl_signal,
    pub grab_focus: wl_signal,
    pub map_request: wl_signal,
    pub ping_timeout: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_ewmh_wm_strut_partial_t {
    pub left: uint32_t,
    pub right: uint32_t,
    pub top: uint32_t,
    pub bottom: uint32_t,
    pub left_start_y: uint32_t,
    pub left_end_y: uint32_t,
    pub right_start_y: uint32_t,
    pub right_end_y: uint32_t,
    pub top_start_x: uint32_t,
    pub top_end_x: uint32_t,
    pub bottom_start_x: uint32_t,
    pub bottom_end_x: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_size_hints_t {
    pub flags: uint32_t,
    pub x: int32_t,
    pub y: int32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub min_width: int32_t,
    pub min_height: int32_t,
    pub max_width: int32_t,
    pub max_height: int32_t,
    pub width_inc: int32_t,
    pub height_inc: int32_t,
    pub min_aspect_num: int32_t,
    pub min_aspect_den: int32_t,
    pub max_aspect_num: int32_t,
    pub max_aspect_den: int32_t,
    pub base_width: int32_t,
    pub base_height: int32_t,
    pub win_gravity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_icccm_wm_hints_t {
    pub flags: int32_t,
    pub input: uint32_t,
    pub initial_state: int32_t,
    pub icon_pixmap: xcb_pixmap_t,
    pub icon_window: xcb_window_t,
    pub icon_x: int32_t,
    pub icon_y: int32_t,
    pub icon_mask: xcb_pixmap_t,
    pub window_group: xcb_window_t,
}
pub type xcb_window_t = uint32_t;
pub type xcb_pixmap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_view_impl {
    pub get_constraints: Option<
        unsafe extern "C" fn(
            *mut sway_view,
            *mut ::core::ffi::c_double,
            *mut ::core::ffi::c_double,
            *mut ::core::ffi::c_double,
            *mut ::core::ffi::c_double,
        ) -> (),
    >,
    pub get_string_prop: Option<
        unsafe extern "C" fn(
            *mut sway_view,
            sway_view_prop,
        ) -> *const ::core::ffi::c_char,
    >,
    pub get_int_prop: Option<
        unsafe extern "C" fn(*mut sway_view, sway_view_prop) -> uint32_t,
    >,
    pub configure: Option<
        unsafe extern "C" fn(
            *mut sway_view,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> uint32_t,
    >,
    pub set_activated: Option<unsafe extern "C" fn(*mut sway_view, bool) -> ()>,
    pub set_tiled: Option<unsafe extern "C" fn(*mut sway_view, bool) -> ()>,
    pub set_fullscreen: Option<unsafe extern "C" fn(*mut sway_view, bool) -> ()>,
    pub set_resizing: Option<unsafe extern "C" fn(*mut sway_view, bool) -> ()>,
    pub wants_floating: Option<unsafe extern "C" fn(*mut sway_view) -> bool>,
    pub is_transient_for: Option<
        unsafe extern "C" fn(*mut sway_view, *mut sway_view) -> bool,
    >,
    pub close: Option<unsafe extern "C" fn(*mut sway_view) -> ()>,
    pub close_popups: Option<unsafe extern "C" fn(*mut sway_view) -> ()>,
    pub destroy: Option<unsafe extern "C" fn(*mut sway_view) -> ()>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_view_prop(pub ::core::ffi::c_uint);
impl sway_view_prop {
    pub const VIEW_PROP_TITLE: Self = Self(0);
    pub const VIEW_PROP_APP_ID: Self = Self(1);
    pub const VIEW_PROP_TAG: Self = Self(2);
    pub const VIEW_PROP_CLASS: Self = Self(3);
    pub const VIEW_PROP_INSTANCE: Self = Self(4);
    pub const VIEW_PROP_WINDOW_TYPE: Self = Self(5);
    pub const VIEW_PROP_WINDOW_ROLE: Self = Self(6);
    pub const VIEW_PROP_X11_WINDOW_ID: Self = Self(7);
    pub const VIEW_PROP_X11_PARENT_ID: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_view_type(pub ::core::ffi::c_uint);
impl sway_view_type {
    pub const SWAY_VIEW_XDG_SHELL: Self = Self(0);
    pub const SWAY_VIEW_XWAYLAND: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_node {
    pub r#type: sway_node_type,
    pub c2rust_unnamed: C2Rust_Unnamed_43,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_42,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_43 {
    pub sway_root: *mut sway_root,
    pub sway_output: *mut sway_output,
    pub sway_workspace: *mut sway_workspace,
    pub sway_container: *mut sway_container,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_output {
    pub node: sway_node,
    pub layers: C2Rust_Unnamed_44,
    pub fullscreen_background: *mut wlr_scene_rect,
    pub wlr_output: *mut wlr_output,
    pub scene_output: *mut wlr_scene_output,
    pub server: *mut sway_server,
    pub link: wl_list,
    pub usable_area: wlr_box,
    pub lx: ::core::ffi::c_int,
    pub ly: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub detected_subpixel: wl_output_subpixel,
    pub scale_filter: scale_filter_mode,
    pub enabled: bool,
    pub workspaces: *mut list_t,
    pub layer_surfaces: wl_list,
    pub current: sway_output_state,
    pub layout_destroy: wl_listener,
    pub destroy: wl_listener,
    pub present: wl_listener,
    pub frame: wl_listener,
    pub request_state: wl_listener,
    pub color_transform: *mut wlr_color_transform,
    pub ext_workspace_group: *mut wlr_ext_workspace_group_handle_v1,
    pub last_presentation: timespec,
    pub refresh_nsec: uint32_t,
    pub max_render_time: ::core::ffi::c_int,
    pub repaint_timer: *mut wl_event_source,
    pub allow_tearing: bool,
    pub hdr: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_output_state {
    pub workspaces: *mut list_t,
    pub active_workspace: *mut sway_workspace,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct scale_filter_mode(pub ::core::ffi::c_uint);
impl scale_filter_mode {
    pub const SCALE_FILTER_DEFAULT: Self = Self(0);
    pub const SCALE_FILTER_LINEAR: Self = Self(1);
    pub const SCALE_FILTER_NEAREST: Self = Self(2);
    pub const SCALE_FILTER_SMART: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_44 {
    pub shell_background: *mut wlr_scene_tree,
    pub shell_bottom: *mut wlr_scene_tree,
    pub tiling: *mut wlr_scene_tree,
    pub fullscreen: *mut wlr_scene_tree,
    pub shell_top: *mut wlr_scene_tree,
    pub shell_overlay: *mut wlr_scene_tree,
    pub session_lock: *mut wlr_scene_tree,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_node_type(pub ::core::ffi::c_uint);
impl sway_node_type {
    pub const N_ROOT: Self = Self(0);
    pub const N_OUTPUT: Self = Self(1);
    pub const N_WORKSPACE: Self = Self(2);
    pub const N_CONTAINER: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_45 {
    pub shell_background: *mut wlr_scene_tree,
    pub shell_bottom: *mut wlr_scene_tree,
    pub tiling: *mut wlr_scene_tree,
    pub floating: *mut wlr_scene_tree,
    pub shell_top: *mut wlr_scene_tree,
    pub fullscreen: *mut wlr_scene_tree,
    pub fullscreen_global: *mut wlr_scene_tree,
    pub unmanaged: *mut wlr_scene_tree,
    pub shell_overlay: *mut wlr_scene_tree,
    pub popup: *mut wlr_scene_tree,
    pub seat: *mut wlr_scene_tree,
    pub session_lock: *mut wlr_scene_tree,
}
pub type sway_container_layout_0 = sway_container_layout;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_input_device {
    pub identifier: *mut ::core::ffi::c_char,
    pub wlr_device: *mut wlr_input_device,
    pub link: wl_list,
    pub device_destroy: wl_listener,
    pub is_virtual: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_config_mapped_from_region {
    pub x1: ::core::ffi::c_double,
    pub y1: ::core::ffi::c_double,
    pub x2: ::core::ffi::c_double,
    pub y2: ::core::ffi::c_double,
    pub mm: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct calibration_matrix {
    pub configured: bool,
    pub matrix: [::core::ffi::c_float; 6],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct input_config_mapped_to(pub ::core::ffi::c_uint);
impl input_config_mapped_to {
    pub const MAPPED_TO_DEFAULT: Self = Self(0);
    pub const MAPPED_TO_OUTPUT: Self = Self(1);
    pub const MAPPED_TO_REGION: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_config {
    pub identifier: *mut ::core::ffi::c_char,
    pub input_type: *const ::core::ffi::c_char,
    pub accel_profile: ::core::ffi::c_int,
    pub calibration_matrix: calibration_matrix,
    pub click_method: ::core::ffi::c_int,
    pub clickfinger_button_map: ::core::ffi::c_int,
    pub drag_3fg: ::core::ffi::c_int,
    pub drag: ::core::ffi::c_int,
    pub drag_lock: ::core::ffi::c_int,
    pub dwt: ::core::ffi::c_int,
    pub dwtp: ::core::ffi::c_int,
    pub left_handed: ::core::ffi::c_int,
    pub middle_emulation: ::core::ffi::c_int,
    pub natural_scroll: ::core::ffi::c_int,
    pub pointer_accel: ::core::ffi::c_float,
    pub rotation_angle: ::core::ffi::c_float,
    pub scroll_factor: ::core::ffi::c_float,
    pub repeat_delay: ::core::ffi::c_int,
    pub repeat_rate: ::core::ffi::c_int,
    pub scroll_button: ::core::ffi::c_int,
    pub scroll_button_lock: ::core::ffi::c_int,
    pub scroll_method: ::core::ffi::c_int,
    pub send_events: ::core::ffi::c_int,
    pub tap: ::core::ffi::c_int,
    pub tap_button_map: ::core::ffi::c_int,
    pub xkb_layout: *mut ::core::ffi::c_char,
    pub xkb_model: *mut ::core::ffi::c_char,
    pub xkb_options: *mut ::core::ffi::c_char,
    pub xkb_rules: *mut ::core::ffi::c_char,
    pub xkb_variant: *mut ::core::ffi::c_char,
    pub xkb_file: *mut ::core::ffi::c_char,
    pub xkb_file_is_set: bool,
    pub xkb_numlock: ::core::ffi::c_int,
    pub xkb_capslock: ::core::ffi::c_int,
    pub mapped_from_region: *mut input_config_mapped_from_region,
    pub mapped_to: input_config_mapped_to,
    pub mapped_to_output: *mut ::core::ffi::c_char,
    pub mapped_to_region: *mut wlr_box,
    pub tools: *mut list_t,
    pub capturable: bool,
    pub region: wlr_box,
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn log_status(mut status: libinput_config_status) {
    if status.0 != libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS.0 {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to apply libinput config: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
            14 as ::core::ffi::c_int,
            libinput_config_status_to_str(status),
        );
    }
}
unsafe extern "C" fn set_send_events(
    mut device: *mut libinput_device,
    mut mode: uint32_t,
) -> bool {
    if libinput_device_config_send_events_get_mode(device) == mode {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] send_events_set_mode(%u)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        23 as ::core::ffi::c_int,
        mode,
    );
    log_status(libinput_device_config_send_events_set_mode(device, mode));
    return r#true != 0;
}
unsafe extern "C" fn set_tap(
    mut device: *mut libinput_device,
    mut tap: libinput_config_tap_state,
) -> bool {
    if libinput_device_config_tap_get_finger_count(device) <= 0 as ::core::ffi::c_int
        || libinput_device_config_tap_get_enabled(device).0 == tap.0
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] tap_set_enabled(%d)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        34 as ::core::ffi::c_int,
        tap.0,
    );
    log_status(libinput_device_config_tap_set_enabled(device, tap));
    return r#true != 0;
}
unsafe extern "C" fn set_tap_button_map(
    mut device: *mut libinput_device,
    mut map: libinput_config_tap_button_map,
) -> bool {
    if libinput_device_config_tap_get_finger_count(device) <= 0 as ::core::ffi::c_int
        || libinput_device_config_tap_get_button_map(device).0 == map.0
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] tap_set_button_map(%d)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        45 as ::core::ffi::c_int,
        map.0,
    );
    log_status(libinput_device_config_tap_set_button_map(device, map));
    return r#true != 0;
}
unsafe extern "C" fn set_3fg_drag(
    mut device: *mut libinput_device,
    mut drag_3fg: libinput_config_3fg_drag_state,
) -> bool {
    if libinput_device_config_3fg_drag_get_enabled(device).0 == drag_3fg.0 {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] 3fg_drag_set_enabled(%d)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        55 as ::core::ffi::c_int,
        drag_3fg.0,
    );
    log_status(libinput_device_config_3fg_drag_set_enabled(device, drag_3fg));
    return r#true != 0;
}
unsafe extern "C" fn set_tap_drag(
    mut device: *mut libinput_device,
    mut drag: libinput_config_drag_state,
) -> bool {
    if libinput_device_config_tap_get_finger_count(device) <= 0 as ::core::ffi::c_int
        || libinput_device_config_tap_get_drag_enabled(device).0 == drag.0
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] tap_set_drag_enabled(%d)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        66 as ::core::ffi::c_int,
        drag.0,
    );
    log_status(libinput_device_config_tap_set_drag_enabled(device, drag));
    return r#true != 0;
}
unsafe extern "C" fn set_tap_drag_lock(
    mut device: *mut libinput_device,
    mut lock: libinput_config_drag_lock_state,
) -> bool {
    if libinput_device_config_tap_get_finger_count(device) <= 0 as ::core::ffi::c_int
        || libinput_device_config_tap_get_drag_lock_enabled(device).0 == lock.0
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] tap_set_drag_lock_enabled(%d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        77 as ::core::ffi::c_int,
        lock.0,
    );
    log_status(libinput_device_config_tap_set_drag_lock_enabled(device, lock));
    return r#true != 0;
}
unsafe extern "C" fn set_accel_speed(
    mut device: *mut libinput_device,
    mut speed: ::core::ffi::c_double,
) -> bool {
    if libinput_device_config_accel_is_available(device) == 0
        || libinput_device_config_accel_get_speed(device) == speed
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] accel_set_speed(%f)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        87 as ::core::ffi::c_int,
        speed,
    );
    log_status(libinput_device_config_accel_set_speed(device, speed));
    return r#true != 0;
}
unsafe extern "C" fn set_rotation_angle(
    mut device: *mut libinput_device,
    mut angle: ::core::ffi::c_double,
) -> bool {
    if libinput_device_config_rotation_is_available(device) == 0
        || libinput_device_config_rotation_get_angle(device) as ::core::ffi::c_double
            == angle
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] rotation_set_angle(%f)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        97 as ::core::ffi::c_int,
        angle,
    );
    log_status(
        libinput_device_config_rotation_set_angle(device, angle as ::core::ffi::c_uint),
    );
    return r#true != 0;
}
unsafe extern "C" fn set_accel_profile(
    mut device: *mut libinput_device,
    mut profile: libinput_config_accel_profile,
) -> bool {
    if libinput_device_config_accel_is_available(device) == 0
        || libinput_device_config_accel_get_profile(device).0 == profile.0
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] accel_set_profile(%d)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        108 as ::core::ffi::c_int,
        profile.0,
    );
    log_status(libinput_device_config_accel_set_profile(device, profile));
    return r#true != 0;
}
unsafe extern "C" fn set_natural_scroll(
    mut d: *mut libinput_device,
    mut n: bool,
) -> bool {
    if libinput_device_config_scroll_has_natural_scroll(d) == 0
        || libinput_device_config_scroll_get_natural_scroll_enabled(d)
            == n as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] scroll_set_natural_scroll(%d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        118 as ::core::ffi::c_int,
        n as ::core::ffi::c_int,
    );
    log_status(
        libinput_device_config_scroll_set_natural_scroll_enabled(
            d,
            n as ::core::ffi::c_int,
        ),
    );
    return r#true != 0;
}
unsafe extern "C" fn set_left_handed(
    mut device: *mut libinput_device,
    mut left: bool,
) -> bool {
    if libinput_device_config_left_handed_is_available(device) == 0
        || libinput_device_config_left_handed_get(device) == left as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] left_handed_set(%d)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        128 as ::core::ffi::c_int,
        left as ::core::ffi::c_int,
    );
    log_status(
        libinput_device_config_left_handed_set(device, left as ::core::ffi::c_int),
    );
    return r#true != 0;
}
unsafe extern "C" fn set_click_method(
    mut device: *mut libinput_device,
    mut method: libinput_config_click_method,
) -> bool {
    let mut click: uint32_t = libinput_device_config_click_get_methods(device);
    if click
        & !(libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_NONE.0
            as ::core::ffi::c_int) as uint32_t == 0 as uint32_t
        || libinput_device_config_click_get_method(device).0 == method.0
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] click_set_method(%d)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        140 as ::core::ffi::c_int,
        method.0,
    );
    log_status(libinput_device_config_click_set_method(device, method));
    return r#true != 0;
}
unsafe extern "C" fn set_clickfinger_button_map(
    mut device: *mut libinput_device,
    mut map: libinput_config_clickfinger_button_map,
) -> bool {
    if libinput_device_config_click_get_clickfinger_button_map(device).0 == map.0 {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] clickfinger_set_button_map(%d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        150 as ::core::ffi::c_int,
        map.0,
    );
    log_status(libinput_device_config_click_set_clickfinger_button_map(device, map));
    return r#true != 0;
}
unsafe extern "C" fn set_middle_emulation(
    mut dev: *mut libinput_device,
    mut mid: libinput_config_middle_emulation_state,
) -> bool {
    if libinput_device_config_middle_emulation_is_available(dev) == 0
        || libinput_device_config_middle_emulation_get_enabled(dev).0 == mid.0
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] middle_emulation_set_enabled(%d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        161 as ::core::ffi::c_int,
        mid.0,
    );
    log_status(libinput_device_config_middle_emulation_set_enabled(dev, mid));
    return r#true != 0;
}
unsafe extern "C" fn set_scroll_method(
    mut device: *mut libinput_device,
    mut method: libinput_config_scroll_method,
) -> bool {
    let mut scroll: uint32_t = libinput_device_config_scroll_get_methods(device);
    if scroll
        & !(libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_NO_SCROLL.0
            as ::core::ffi::c_int) as uint32_t == 0 as uint32_t
        || libinput_device_config_scroll_get_method(device).0 == method.0
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] scroll_set_method(%d)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        173 as ::core::ffi::c_int,
        method.0,
    );
    log_status(libinput_device_config_scroll_set_method(device, method));
    return r#true != 0;
}
unsafe extern "C" fn set_scroll_button(
    mut dev: *mut libinput_device,
    mut button: uint32_t,
) -> bool {
    let mut scroll: uint32_t = libinput_device_config_scroll_get_methods(dev);
    if scroll
        & !(libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_NO_SCROLL.0
            as ::core::ffi::c_int) as uint32_t == 0 as uint32_t
        || libinput_device_config_scroll_get_button(dev) == button
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] scroll_set_button(%u)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        184 as ::core::ffi::c_int,
        button,
    );
    log_status(libinput_device_config_scroll_set_button(dev, button));
    return r#true != 0;
}
unsafe extern "C" fn set_scroll_button_lock(
    mut dev: *mut libinput_device,
    mut lock: libinput_config_scroll_button_lock_state,
) -> bool {
    let mut scroll: uint32_t = libinput_device_config_scroll_get_methods(dev);
    if scroll
        & !(libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_NO_SCROLL.0
            as ::core::ffi::c_int) as uint32_t == 0 as uint32_t
        || libinput_device_config_scroll_get_button_lock(dev).0 == lock.0
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] scroll_set_button_lock(%u)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        196 as ::core::ffi::c_int,
        lock.0,
    );
    log_status(libinput_device_config_scroll_set_button_lock(dev, lock));
    return r#true != 0;
}
unsafe extern "C" fn set_dwt(mut device: *mut libinput_device, mut dwt: bool) -> bool {
    if libinput_device_config_dwt_is_available(device) == 0
        || libinput_device_config_dwt_get_enabled(device).0 == dwt as ::core::ffi::c_uint
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] dwt_set_enabled(%d)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        206 as ::core::ffi::c_int,
        dwt as ::core::ffi::c_int,
    );
    log_status(
        libinput_device_config_dwt_set_enabled(
            device,
            libinput_config_dwt_state(dwt as ::core::ffi::c_uint),
        ),
    );
    return r#true != 0;
}
unsafe extern "C" fn set_dwtp(mut device: *mut libinput_device, mut dwtp: bool) -> bool {
    if libinput_device_config_dwtp_is_available(device) == 0
        || libinput_device_config_dwtp_get_enabled(device).0
            == dwtp as ::core::ffi::c_uint
    {
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] dwtp_set_enabled(%d)\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        216 as ::core::ffi::c_int,
        dwtp as ::core::ffi::c_int,
    );
    log_status(
        libinput_device_config_dwtp_set_enabled(
            device,
            libinput_config_dwtp_state(dwtp as ::core::ffi::c_uint),
        ),
    );
    return r#true != 0;
}
unsafe extern "C" fn set_calibration_matrix(
    mut dev: *mut libinput_device,
    mut mat: *mut ::core::ffi::c_float,
) -> bool {
    if libinput_device_config_calibration_has_matrix(dev) == 0 {
        return r#false != 0;
    }
    let mut changed: bool = r#false != 0;
    let mut current: [::core::ffi::c_float; 6] = [0.; 6];
    libinput_device_config_calibration_get_matrix(
        dev,
        &raw mut current as *mut ::core::ffi::c_float,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int {
        if current[i as usize] != *mat.offset(i as isize) {
            changed = r#true != 0;
            break;
        } else {
            i += 1;
        }
    }
    if changed {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] calibration_set_matrix(%f, %f, %f, %f, %f, %f)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
            235 as ::core::ffi::c_int,
            *mat.offset(0isize) as ::core::ffi::c_double,
            *mat.offset(1isize) as ::core::ffi::c_double,
            *mat.offset(2isize) as ::core::ffi::c_double,
            *mat.offset(3isize) as ::core::ffi::c_double,
            *mat.offset(4isize) as ::core::ffi::c_double,
            *mat.offset(5isize) as ::core::ffi::c_double,
        );
        log_status(libinput_device_config_calibration_set_matrix(dev, mat));
    }
    return changed;
}
unsafe extern "C" fn configure_send_events(
    mut device: *mut libinput_device,
    mut ic: *mut input_config,
) -> bool {
    if !(*ic).mapped_to_output.is_null()
        && strcmp(b"*\0".as_ptr() as *const ::core::ffi::c_char, (*ic).mapped_to_output)
            != 0 as ::core::ffi::c_int
        && output_by_name_or_id((*ic).mapped_to_output).is_null()
    {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] %s '%s' is mapped to offline output '%s'; disabling input\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
            247 as ::core::ffi::c_int,
            (*ic).input_type,
            (*ic).identifier,
            (*ic).mapped_to_output,
        );
        return set_send_events(
            device,
            libinput_config_send_events_mode::LIBINPUT_CONFIG_SEND_EVENTS_DISABLED.0
                as ::core::ffi::c_int as uint32_t,
        );
    } else if (*ic).send_events != INT_MIN {
        return set_send_events(device, (*ic).send_events as uint32_t)
    } else {
        return set_send_events(
            device,
            libinput_device_config_send_events_get_default_mode(device),
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn sway_input_configure_libinput_device(
    mut input_device: *mut sway_input_device,
) -> bool {
    let mut ic: *mut input_config = input_device_get_config(input_device);
    if ic.is_null() || !wlr_input_device_is_libinput((*input_device).wlr_device) {
        return r#false != 0;
    }
    let mut device: *mut libinput_device = wlr_libinput_get_device_handle(
        (*input_device).wlr_device,
    );
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] sway_input_configure_libinput_device('%s' on '%s')\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        270 as ::core::ffi::c_int,
        (*ic).identifier,
        (*input_device).identifier,
    );
    let mut changed: bool = configure_send_events(device, ic);
    if (*ic).tap != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_tap(
                device,
                libinput_config_tap_state((*ic).tap as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int != 0;
    }
    if (*ic).tap_button_map != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_tap_button_map(
                device,
                libinput_config_tap_button_map(
                    (*ic).tap_button_map as ::core::ffi::c_uint,
                ),
            ) as ::core::ffi::c_int != 0;
    }
    if (*ic).drag_3fg != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_3fg_drag(
                device,
                libinput_config_3fg_drag_state((*ic).drag_3fg as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int != 0;
    }
    if (*ic).drag != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_tap_drag(
                device,
                libinput_config_drag_state((*ic).drag as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int != 0;
    }
    if (*ic).drag_lock != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_tap_drag_lock(
                device,
                libinput_config_drag_lock_state((*ic).drag_lock as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int != 0;
    }
    if (*ic).pointer_accel != FLT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_accel_speed(device, (*ic).pointer_accel as ::core::ffi::c_double)
                as ::core::ffi::c_int != 0;
    }
    if (*ic).rotation_angle != FLT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_rotation_angle(device, (*ic).rotation_angle as ::core::ffi::c_double)
                as ::core::ffi::c_int != 0;
    }
    if (*ic).accel_profile != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_accel_profile(
                device,
                libinput_config_accel_profile((*ic).accel_profile as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int != 0;
    }
    if (*ic).natural_scroll != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_natural_scroll(device, (*ic).natural_scroll != 0) as ::core::ffi::c_int
            != 0;
    }
    if (*ic).left_handed != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_left_handed(device, (*ic).left_handed != 0) as ::core::ffi::c_int != 0;
    }
    if (*ic).click_method != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_click_method(
                device,
                libinput_config_click_method((*ic).click_method as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int != 0;
    }
    if (*ic).clickfinger_button_map != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_clickfinger_button_map(
                device,
                libinput_config_clickfinger_button_map(
                    (*ic).clickfinger_button_map as ::core::ffi::c_uint,
                ),
            ) as ::core::ffi::c_int != 0;
    }
    if (*ic).middle_emulation != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_middle_emulation(
                device,
                libinput_config_middle_emulation_state(
                    (*ic).middle_emulation as ::core::ffi::c_uint,
                ),
            ) as ::core::ffi::c_int != 0;
    }
    if (*ic).scroll_method != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_scroll_method(
                device,
                libinput_config_scroll_method((*ic).scroll_method as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int != 0;
    }
    if (*ic).scroll_button != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_scroll_button(device, (*ic).scroll_button as uint32_t)
                as ::core::ffi::c_int != 0;
    }
    if (*ic).scroll_button_lock != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_scroll_button_lock(
                device,
                libinput_config_scroll_button_lock_state(
                    (*ic).scroll_button_lock as ::core::ffi::c_uint,
                ),
            ) as ::core::ffi::c_int != 0;
    }
    if (*ic).dwt != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_dwt(device, (*ic).dwt != 0) as ::core::ffi::c_int != 0;
    }
    if (*ic).dwtp != INT_MIN {
        changed = changed as ::core::ffi::c_int
            | set_dwtp(device, (*ic).dwtp != 0) as ::core::ffi::c_int != 0;
    }
    if (*ic).calibration_matrix.configured {
        changed = changed as ::core::ffi::c_int
            | set_calibration_matrix(
                device,
                &raw mut (*ic).calibration_matrix.matrix as *mut ::core::ffi::c_float,
            ) as ::core::ffi::c_int != 0;
    }
    return changed;
}
#[no_mangle]
pub unsafe extern "C" fn sway_input_configure_libinput_device_send_events(
    mut input_device: *mut sway_input_device,
) {
    let mut ic: *mut input_config = input_device_get_config(input_device);
    if ic.is_null() || !wlr_input_device_is_libinput((*input_device).wlr_device) {
        return;
    }
    let mut device: *mut libinput_device = wlr_libinput_get_device_handle(
        (*input_device).wlr_device,
    );
    let mut changed: bool = configure_send_events(device, ic);
    if changed {
        ipc_event_input(
            b"libinput_config\0".as_ptr() as *const ::core::ffi::c_char,
            input_device,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sway_input_reset_libinput_device(
    mut input_device: *mut sway_input_device,
) {
    if !wlr_input_device_is_libinput((*input_device).wlr_device) {
        return;
    }
    let mut device: *mut libinput_device = wlr_libinput_get_device_handle(
        (*input_device).wlr_device,
    );
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] sway_input_reset_libinput_device(%s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/libinput.c\0".as_ptr() as *const ::core::ffi::c_char,
        358 as ::core::ffi::c_int,
        (*input_device).identifier,
    );
    let mut changed: bool = r#false != 0;
    changed = changed as ::core::ffi::c_int
        | set_send_events(
            device,
            libinput_device_config_send_events_get_default_mode(device),
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_tap(device, libinput_device_config_tap_get_default_enabled(device))
            as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_tap_button_map(
            device,
            libinput_device_config_tap_get_default_button_map(device),
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_3fg_drag(
            device,
            libinput_device_config_3fg_drag_get_default_enabled(device),
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_tap_drag(
            device,
            libinput_device_config_tap_get_default_drag_enabled(device),
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_tap_drag_lock(
            device,
            libinput_device_config_tap_get_default_drag_lock_enabled(device),
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_accel_speed(device, libinput_device_config_accel_get_default_speed(device))
            as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_rotation_angle(
            device,
            libinput_device_config_rotation_get_default_angle(device)
                as ::core::ffi::c_double,
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_accel_profile(
            device,
            libinput_device_config_accel_get_default_profile(device),
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_natural_scroll(
            device,
            libinput_device_config_scroll_get_default_natural_scroll_enabled(device) != 0,
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_left_handed(
            device,
            libinput_device_config_left_handed_get_default(device) != 0,
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_click_method(
            device,
            libinput_device_config_click_get_default_method(device),
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_clickfinger_button_map(
            device,
            libinput_device_config_click_get_default_clickfinger_button_map(device),
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_middle_emulation(
            device,
            libinput_device_config_middle_emulation_get_default_enabled(device),
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_scroll_method(
            device,
            libinput_device_config_scroll_get_default_method(device),
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_scroll_button(
            device,
            libinput_device_config_scroll_get_default_button(device),
        ) as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_dwt(device, libinput_device_config_dwt_get_default_enabled(device).0 != 0)
            as ::core::ffi::c_int != 0;
    changed = changed as ::core::ffi::c_int
        | set_dwtp(
            device,
            libinput_device_config_dwtp_get_default_enabled(device).0 != 0,
        ) as ::core::ffi::c_int != 0;
    let mut matrix: [::core::ffi::c_float; 6] = [0.; 6];
    libinput_device_config_calibration_get_default_matrix(
        device,
        &raw mut matrix as *mut ::core::ffi::c_float,
    );
    changed = changed as ::core::ffi::c_int
        | set_calibration_matrix(device, &raw mut matrix as *mut ::core::ffi::c_float)
            as ::core::ffi::c_int != 0;
    if changed {
        ipc_event_input(
            b"libinput_config\0".as_ptr() as *const ::core::ffi::c_char,
            input_device,
        );
    }
}
unsafe extern "C" fn sway_udev_device_is_builtin(
    mut udev_device: *mut udev_device,
) -> bool {
    let mut id_path: *const ::core::ffi::c_char = udev_device_get_property_value(
        udev_device,
        b"ID_PATH\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if id_path.is_null() {
        return r#false != 0;
    }
    if has_prefix(id_path, b"platform-\0".as_ptr() as *const ::core::ffi::c_char) {
        return r#true != 0;
    }
    return has_prefix(id_path, b"pci-\0".as_ptr() as *const ::core::ffi::c_char)
        as ::core::ffi::c_int != 0
        && !strstr(id_path, b"-platform-\0".as_ptr() as *const ::core::ffi::c_char)
            .is_null();
}
#[no_mangle]
pub unsafe extern "C" fn sway_libinput_device_is_builtin(
    mut sway_device: *mut sway_input_device,
) -> bool {
    if !wlr_input_device_is_libinput((*sway_device).wlr_device) {
        return r#false != 0;
    }
    let mut device: *mut libinput_device = wlr_libinput_get_device_handle(
        (*sway_device).wlr_device,
    );
    let mut udev_device: *mut udev_device = libinput_device_get_udev_device(device);
    if udev_device.is_null() {
        return r#false != 0;
    }
    let mut is_builtin: bool = sway_udev_device_is_builtin(udev_device);
    udev_device_unref(udev_device);
    return is_builtin;
}
pub const FLT_MIN: ::core::ffi::c_float = __FLT_MIN__;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __FLT_MIN__: ::core::ffi::c_float = 1.17549435e-38f32;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
