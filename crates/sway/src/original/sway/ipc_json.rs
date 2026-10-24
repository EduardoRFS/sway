extern "C" {
    pub type json_object;
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_buffer_impl;
    pub type wlr_renderer_impl;
    pub type wlr_texture_impl;
    pub type wlr_drm_syncobj_timeline;
    pub type wlr_backend_impl;
    pub type wlr_swapchain;
    pub type wlr_allocator;
    pub type wlr_output_layer_state;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type udev;
    pub type udev_monitor;
    pub type libinput_device;
    pub type wlr_linux_dmabuf_feedback_v1_compiled;
    pub type wlr_output_layer_feedback_event;
    pub type wlr_gamma_control_v1;
    pub type wlr_color_manager_v1;
    pub type wlr_gamma_control_manager_v1;
    pub type sway_transaction;
    pub type wlr_xdg_activation_v1;
    pub type wlr_ext_foreign_toplevel_image_capture_source_manager_v1;
    pub type wlr_security_context_manager_v1;
    pub type wlr_export_dmabuf_manager_v1;
    pub type wlr_ext_image_copy_capture_manager_v1;
    pub type wlr_screencopy_manager_v1;
    pub type wlr_ext_data_control_manager_v1;
    pub type wlr_data_control_manager_v1;
    pub type wlr_foreign_toplevel_manager_v1;
    pub type wlr_output_power_manager_v1;
    pub type wlr_session_lock_manager_v1;
    pub type wlr_session_lock_v1;
    pub type wlr_output_manager_v1;
    pub type wlr_xdg_output_manager_v1;
    pub type wlr_drm_lease_v1_manager;
    pub type wlr_xdg_decoration_manager_v1;
    pub type wlr_server_decoration_manager;
    pub type wlr_relative_pointer_manager_v1;
    pub type wlr_xcursor_manager;
    pub type wlr_keyboard_group;
    pub type wlr_keyboard_impl;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_primary_selection_source;
    pub type wlr_xwayland_shell_v1;
    pub type wlr_xwm;
    pub type wlr_tablet_manager_v2;
    pub type wlr_xdg_shell;
    pub type wlr_idle_notifier_v1;
    pub type wlr_data_device_manager;
    pub type libseat;
    pub type sway_transaction_instruction;
    pub type sway_text_node;
    pub type wlr_xdg_toplevel;
    pub type wlr_foreign_toplevel_handle_v1;
    pub type launcher_ctx;
    pub type sway_xdg_decoration;
    pub type wlr_ext_image_capture_source_v1;
    pub type wlr_tablet_v2_tablet_tool;
    pub type wlr_tablet_v2_tablet;
    pub type wlr_tablet_v2_tablet_pad;
    pub type wlr_tablet_pad;
    pub type sway_switch;
    pub type sway_keyboard;
    pub type wlr_touch_impl;
    pub type wlr_cursor;
    pub type _PangoFontDescription;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const ::core::ffi::c_char,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_new_array() -> *mut json_object;
    fn json_object_array_length(obj: *const json_object) -> size_t;
    fn json_object_array_add(
        obj: *mut json_object,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_array_get_idx(
        obj: *const json_object,
        idx: size_t,
    ) -> *mut json_object;
    fn json_object_new_boolean(b: json_bool) -> *mut json_object;
    fn json_object_new_int(i: int32_t) -> *mut json_object;
    fn json_object_get_int(obj: *const json_object) -> int32_t;
    fn json_object_new_double(d: ::core::ffi::c_double) -> *mut json_object;
    fn json_object_new_string(s: *const ::core::ffi::c_char) -> *mut json_object;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn wlr_surface_get_content_type_v1(
        manager: *mut wlr_content_type_manager_v1,
        surface: *mut wlr_surface,
    ) -> wp_content_type_v1_type;
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    fn libinput_device_get_id_product(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_uint;
    fn libinput_device_get_id_vendor(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_uint;
    fn libinput_device_config_tap_get_finger_count(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_tap_get_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_tap_state;
    fn libinput_device_config_tap_get_button_map(
        device: *mut libinput_device,
    ) -> libinput_config_tap_button_map;
    fn libinput_device_config_tap_get_drag_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_drag_state;
    fn libinput_device_config_tap_get_drag_lock_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_drag_lock_state;
    fn libinput_device_config_3fg_drag_get_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_3fg_drag_state;
    fn libinput_device_config_calibration_has_matrix(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_calibration_get_matrix(
        device: *mut libinput_device,
        matrix: *mut ::core::ffi::c_float,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_send_events_get_mode(
        device: *mut libinput_device,
    ) -> uint32_t;
    fn libinput_device_config_accel_is_available(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_accel_get_speed(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_double;
    fn libinput_device_config_accel_get_profile(
        device: *mut libinput_device,
    ) -> libinput_config_accel_profile;
    fn libinput_device_config_scroll_has_natural_scroll(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_scroll_get_natural_scroll_enabled(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_left_handed_is_available(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_left_handed_get(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_click_get_methods(
        device: *mut libinput_device,
    ) -> uint32_t;
    fn libinput_device_config_click_get_method(
        device: *mut libinput_device,
    ) -> libinput_config_click_method;
    fn libinput_device_config_click_get_clickfinger_button_map(
        device: *mut libinput_device,
    ) -> libinput_config_clickfinger_button_map;
    fn libinput_device_config_middle_emulation_is_available(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_middle_emulation_get_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_middle_emulation_state;
    fn libinput_device_config_scroll_get_methods(
        device: *mut libinput_device,
    ) -> uint32_t;
    fn libinput_device_config_scroll_get_method(
        device: *mut libinput_device,
    ) -> libinput_config_scroll_method;
    fn libinput_device_config_scroll_get_button(
        device: *mut libinput_device,
    ) -> uint32_t;
    fn libinput_device_config_scroll_get_button_lock(
        device: *mut libinput_device,
    ) -> libinput_config_scroll_button_lock_state;
    fn libinput_device_config_dwt_is_available(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_dwt_get_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_dwt_state;
    fn libinput_device_config_dwtp_is_available(
        device: *mut libinput_device,
    ) -> ::core::ffi::c_int;
    fn libinput_device_config_dwtp_get_enabled(
        device: *mut libinput_device,
    ) -> libinput_config_dwtp_state;
    fn xkb_keymap_num_layouts(keymap: *mut xkb_keymap) -> xkb_layout_index_t;
    fn xkb_keymap_layout_get_name(
        keymap: *mut xkb_keymap,
        idx: xkb_layout_index_t,
    ) -> *const ::core::ffi::c_char;
    fn xkb_state_layout_index_is_active(
        state: *mut xkb_state,
        idx: xkb_layout_index_t,
        r#type: xkb_state_component,
    ) -> ::core::ffi::c_int;
    static mut root: *mut sway_root;
    fn root_get_box(root_0: *mut sway_root, r#box: *mut wlr_box);
    fn input_manager_get_default_seat() -> *mut sway_seat;
    fn input_device_get_config(device: *mut sway_input_device) -> *mut input_config;
    fn input_device_get_type(
        device: *mut sway_input_device,
    ) -> *const ::core::ffi::c_char;
    fn container_titlebar_height() -> size_t;
    fn container_is_floating(container: *mut sway_container) -> bool;
    fn container_has_urgent_child(container: *mut sway_container) -> bool;
    fn container_parent_layout(con: *mut sway_container) -> sway_container_layout;
    fn container_get_siblings(container: *mut sway_container) -> *mut list_t;
    fn container_sibling_index(child: *mut sway_container) -> ::core::ffi::c_int;
    fn container_is_scratchpad_hidden(con: *mut sway_container) -> bool;
    fn output_get_active_workspace(output: *mut sway_output) -> *mut sway_workspace;
    fn output_supports_hdr(
        output: *mut wlr_output,
        unsupported_reason_ptr: *mut *const ::core::ffi::c_char,
    ) -> bool;
    fn node_get_name(node: *mut sway_node) -> *mut ::core::ffi::c_char;
    fn node_get_box(node: *mut sway_node, r#box: *mut wlr_box);
    fn node_get_output(node: *mut sway_node) -> *mut sway_output;
    fn node_get_parent(node: *mut sway_node) -> *mut sway_node;
    fn wlr_libinput_get_device_handle(
        dev: *mut wlr_input_device,
    ) -> *mut libinput_device;
    fn wlr_input_device_is_libinput(device: *mut wlr_input_device) -> bool;
    fn sway_idle_inhibit_v1_user_inhibitor_for_view(
        view: *mut sway_view,
    ) -> *mut sway_idle_inhibitor_v1;
    fn sway_idle_inhibit_v1_application_inhibitor_for_view(
        view: *mut sway_view,
    ) -> *mut sway_idle_inhibitor_v1;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn view_get_app_id(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_class(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_instance(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_x11_window_id(view: *mut sway_view) -> uint32_t;
    fn view_get_x11_parent_id(view: *mut sway_view) -> uint32_t;
    fn view_get_window_role(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_window_type(view: *mut sway_view) -> uint32_t;
    fn view_get_sandbox_engine(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_sandbox_app_id(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_sandbox_instance_id(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_tag(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_shell(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_inhibit_idle(view: *mut sway_view) -> bool;
    fn view_is_visible(view: *mut sway_view) -> bool;
    fn view_is_urgent(view: *mut sway_view) -> bool;
    fn view_can_tear(view: *mut sway_view) -> bool;
    fn sway_output_scale_filter_to_string(
        scale_filter: scale_filter_mode,
    ) -> *const ::core::ffi::c_char;
    static mut config: *mut sway_config;
    fn seat_get_focus(seat: *mut sway_seat) -> *mut sway_node;
    fn seat_for_each_node(
        seat: *mut sway_seat,
        f: Option<unsafe extern "C" fn(*mut sway_node, *mut ::core::ffi::c_void) -> ()>,
        data: *mut ::core::ffi::c_void,
    );
    static mut server: sway_server;
    fn wlr_keyboard_from_input_device(
        input_device: *mut wlr_input_device,
    ) -> *mut wlr_keyboard;
}
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
pub type size_t = usize;
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type json_bool = ::core::ffi::c_int;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pid_t = __pid_t;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wp_content_type_v1_type(pub ::core::ffi::c_uint);
impl wp_content_type_v1_type {
    pub const WP_CONTENT_TYPE_V1_TYPE_NONE: Self = Self(0);
    pub const WP_CONTENT_TYPE_V1_TYPE_PHOTO: Self = Self(1);
    pub const WP_CONTENT_TYPE_V1_TYPE_VIDEO: Self = Self(2);
    pub const WP_CONTENT_TYPE_V1_TYPE_GAME: Self = Self(3);
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
    pub events: C2Rust_Unnamed_3,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_1,
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
pub struct C2Rust_Unnamed_1 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon_set {
    pub WLR_PRIVATE: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub addons: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
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
    pub viewport: C2Rust_Unnamed_4,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub has_src: bool,
    pub has_dst: bool,
    pub src: wlr_fbox,
    pub dst_width: ::core::ffi::c_int,
    pub dst_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_fbox {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
}
pub type pixman_region32_t = pixman_region32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_data_t = pixman_region32_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: ::core::ffi::c_long,
    pub numRects: ::core::ffi::c_long,
}
pub type pixman_box32_t = pixman_box32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_client_buffer {
    pub base: wlr_buffer,
    pub texture: *mut wlr_texture,
    pub source: *mut wlr_buffer,
    pub WLR_PRIVATE: C2Rust_Unnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub source_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
    pub n_ignore_locks: size_t,
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
pub struct wlr_renderer {
    pub render_buffer_caps: uint32_t,
    pub color_encodings: uint32_t,
    pub events: C2Rust_Unnamed_9,
    pub features: C2Rust_Unnamed_8,
    pub WLR_PRIVATE: C2Rust_Unnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub r#impl: *const wlr_renderer_impl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub input_color_transform: bool,
    pub output_color_transform: bool,
    pub timeline: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub destroy: wl_signal,
    pub lost: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_compositor {
    pub global: *mut wl_global,
    pub renderer: *mut wlr_renderer,
    pub events: C2Rust_Unnamed_11,
    pub WLR_PRIVATE: C2Rust_Unnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_content_type_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_13,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub destroy: wl_signal,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_pointer_axis_source(pub ::core::ffi::c_uint);
impl wl_pointer_axis_source {
    pub const WL_POINTER_AXIS_SOURCE_WHEEL: Self = Self(0);
    pub const WL_POINTER_AXIS_SOURCE_FINGER: Self = Self(1);
    pub const WL_POINTER_AXIS_SOURCE_CONTINUOUS: Self = Self(2);
    pub const WL_POINTER_AXIS_SOURCE_WHEEL_TILT: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_pointer_axis_relative_direction(pub ::core::ffi::c_uint);
impl wl_pointer_axis_relative_direction {
    pub const WL_POINTER_AXIS_RELATIVE_DIRECTION_IDENTICAL: Self = Self(0);
    pub const WL_POINTER_AXIS_RELATIVE_DIRECTION_INVERTED: Self = Self(1);
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
pub struct wlr_addon {
    pub r#impl: *const wlr_addon_interface,
    pub WLR_PRIVATE: C2Rust_Unnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_scale_filter_mode(pub ::core::ffi::c_uint);
impl wlr_scale_filter_mode {
    pub const WLR_SCALE_FILTER_BILINEAR: Self = Self(0);
    pub const WLR_SCALE_FILTER_NEAREST: Self = Self(1);
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
    pub features: C2Rust_Unnamed_17,
    pub events: C2Rust_Unnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
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
    pub events: C2Rust_Unnamed_21,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
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
    pub mastering_luminance: C2Rust_Unnamed_20,
    pub max_cll: ::core::ffi::c_double,
    pub max_fall: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub min: ::core::ffi::c_double,
    pub max: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
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
    pub custom_mode: C2Rust_Unnamed_22,
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
pub struct C2Rust_Unnamed_22 {
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
pub struct wlr_ext_foreign_toplevel_list_v1 {
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub toplevels: wl_list,
    pub events: C2Rust_Unnamed_24,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_foreign_toplevel_handle_v1 {
    pub list: *mut wlr_ext_foreign_toplevel_list_v1,
    pub resources: wl_list,
    pub link: wl_list,
    pub title: *mut ::core::ffi::c_char,
    pub app_id: *mut ::core::ffi::c_char,
    pub identifier: *mut ::core::ffi::c_char,
    pub events: C2Rust_Unnamed_25,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub destroy: wl_signal,
}
pub type xkb_layout_index_t = uint32_t;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xkb_state_component(pub ::core::ffi::c_uint);
impl xkb_state_component {
    pub const XKB_STATE_MODS_DEPRESSED: Self = Self(1);
    pub const XKB_STATE_MODS_LATCHED: Self = Self(2);
    pub const XKB_STATE_MODS_LOCKED: Self = Self(4);
    pub const XKB_STATE_MODS_EFFECTIVE: Self = Self(8);
    pub const XKB_STATE_LAYOUT_DEPRESSED: Self = Self(16);
    pub const XKB_STATE_LAYOUT_LATCHED: Self = Self(32);
    pub const XKB_STATE_LAYOUT_LOCKED: Self = Self(64);
    pub const XKB_STATE_LAYOUT_EFFECTIVE: Self = Self(128);
    pub const XKB_STATE_LEDS: Self = Self(256);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_device {
    pub r#type: wlr_input_device_type,
    pub name: *mut ::core::ffi::c_char,
    pub events: C2Rust_Unnamed_26,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub destroy: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_tablet_tool_tip_state(pub ::core::ffi::c_uint);
impl wlr_tablet_tool_tip_state {
    pub const WLR_TABLET_TOOL_TIP_UP: Self = Self(0);
    pub const WLR_TABLET_TOOL_TIP_DOWN: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmModeModeInfo {
    pub clock: uint32_t,
    pub hdisplay: uint16_t,
    pub hsync_start: uint16_t,
    pub hsync_end: uint16_t,
    pub htotal: uint16_t,
    pub hskew: uint16_t,
    pub vdisplay: uint16_t,
    pub vsync_start: uint16_t,
    pub vsync_end: uint16_t,
    pub vtotal: uint16_t,
    pub vscan: uint16_t,
    pub vrefresh: uint32_t,
    pub flags: uint32_t,
    pub r#type: uint32_t,
    pub name: [::core::ffi::c_char; 32],
}
pub type drmModeModeInfo = _drmModeModeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag_instance {
    pub client: *mut wl_client,
    pub client_destroy: wl_listener,
    pub args: *const ::core::ffi::c_char,
    pub fd: [::core::ffi::c_int; 2],
    pub detailed: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_surface_synced {
    pub surface: *mut wlr_surface,
    pub r#impl: *const wlr_surface_synced_impl,
    pub link: wl_list,
    pub index: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_surface_synced_impl {
    pub state_size: size_t,
    pub init_state: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub finish_state: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub move_state: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
    >,
    pub commit: Option<unsafe extern "C" fn(*mut wlr_surface_synced) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_damage_ring {
    pub current: pixman_region32_t,
    pub WLR_PRIVATE: C2Rust_Unnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_29,
    pub WLR_PRIVATE: C2Rust_Unnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
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
pub struct C2Rust_Unnamed_29 {
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
    pub events: C2Rust_Unnamed_31,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1 {
    pub surface: *mut wlr_surface,
    pub output: *mut wlr_output,
    pub resource: *mut wl_resource,
    pub shell: *mut wlr_layer_shell_v1,
    pub popups: wl_list,
    pub namespace: *mut ::core::ffi::c_char,
    pub configured: bool,
    pub configure_list: wl_list,
    pub current: wlr_layer_surface_v1_state,
    pub pending: wlr_layer_surface_v1_state,
    pub initialized: bool,
    pub initial_commit: bool,
    pub events: C2Rust_Unnamed_33,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_34,
    pub keyboard_interactive: zwlr_layer_surface_v1_keyboard_interactivity,
    pub desired_width: uint32_t,
    pub desired_height: uint32_t,
    pub layer: zwlr_layer_shell_v1_layer,
    pub exclusive_edge: uint32_t,
    pub configure_serial: uint32_t,
    pub actual_width: uint32_t,
    pub actual_height: uint32_t,
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
pub struct zwlr_layer_surface_v1_keyboard_interactivity(pub ::core::ffi::c_uint);
impl zwlr_layer_surface_v1_keyboard_interactivity {
    pub const ZWLR_LAYER_SURFACE_V1_KEYBOARD_INTERACTIVITY_NONE: Self = Self(0);
    pub const ZWLR_LAYER_SURFACE_V1_KEYBOARD_INTERACTIVITY_EXCLUSIVE: Self = Self(1);
    pub const ZWLR_LAYER_SURFACE_V1_KEYBOARD_INTERACTIVITY_ON_DEMAND: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_36,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
    pub new_surface: wl_signal,
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
    pub events: C2Rust_Unnamed_38,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_37 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
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
    pub events: C2Rust_Unnamed_44,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_40 {
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
    pub events: C2Rust_Unnamed_42,
    pub WLR_PRIVATE: C2Rust_Unnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_41 {
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
pub struct C2Rust_Unnamed_42 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_43,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
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
pub struct C2Rust_Unnamed_44 {
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
    pub layers: C2Rust_Unnamed_103,
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
    pub events: C2Rust_Unnamed_45,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_45 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_98,
    pub border: C2Rust_Unnamed_97,
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
    pub events: C2Rust_Unnamed_46,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_workspace {
    pub node: sway_node,
    pub layers: C2Rust_Unnamed_96,
    pub fullscreen: *mut sway_container,
    pub name: *mut ::core::ffi::c_char,
    pub representation: *mut ::core::ffi::c_char,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub layout: sway_container_layout,
    pub prev_split_layout: sway_container_layout,
    pub current_gaps: side_gaps,
    pub gaps_inner: ::core::ffi::c_int,
    pub gaps_outer: side_gaps,
    pub output: *mut sway_output,
    pub floating: *mut list_t,
    pub tiling: *mut list_t,
    pub output_priority: *mut list_t,
    pub urgent: bool,
    pub current: sway_workspace_state,
    pub ext_workspace: *mut wlr_ext_workspace_handle_v1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_handle_v1 {
    pub manager: *mut wlr_ext_workspace_manager_v1,
    pub group: *mut wlr_ext_workspace_group_handle_v1,
    pub id: *mut ::core::ffi::c_char,
    pub name: *mut ::core::ffi::c_char,
    pub coordinates: wl_array,
    pub caps: uint32_t,
    pub state: uint32_t,
    pub events: C2Rust_Unnamed_48,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_47,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_group_handle_v1 {
    pub manager: *mut wlr_ext_workspace_manager_v1,
    pub caps: uint32_t,
    pub events: C2Rust_Unnamed_50,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_49,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
    pub outputs: wl_list,
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_manager_v1 {
    pub global: *mut wl_global,
    pub groups: wl_list,
    pub workspaces: wl_list,
    pub events: C2Rust_Unnamed_52,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_51,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
    pub resources: wl_list,
    pub idle_source: *mut wl_event_source,
    pub event_loop: *mut wl_event_loop,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_52 {
    pub commit: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_workspace_state {
    pub fullscreen: *mut sway_container,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub layout: sway_container_layout,
    pub output: *mut sway_output,
    pub floating: *mut list_t,
    pub tiling: *mut list_t,
    pub focused_inactive_child: *mut sway_container,
    pub focused: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_output {
    pub node: sway_node,
    pub layers: C2Rust_Unnamed_93,
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
pub struct sway_server {
    pub wl_display: *mut wl_display,
    pub wl_event_loop: *mut wl_event_loop,
    pub socket: *mut ::core::ffi::c_char,
    pub backend: *mut wlr_backend,
    pub session: *mut wlr_session,
    pub headless_backend: *mut wlr_backend,
    pub renderer: *mut wlr_renderer,
    pub allocator: *mut wlr_allocator,
    pub compositor: *mut wlr_compositor,
    pub linux_dmabuf_v1: *mut wlr_linux_dmabuf_v1,
    pub data_device_manager: *mut wlr_data_device_manager,
    pub input: *mut sway_input_manager,
    pub new_output: wl_listener,
    pub renderer_lost: wl_listener,
    pub recreating_renderer: *mut wl_event_source,
    pub idle_notifier_v1: *mut wlr_idle_notifier_v1,
    pub idle_inhibit_manager_v1: sway_idle_inhibit_manager_v1,
    pub layer_shell: *mut wlr_layer_shell_v1,
    pub layer_shell_surface: wl_listener,
    pub xdg_shell: *mut wlr_xdg_shell,
    pub xdg_shell_toplevel: wl_listener,
    pub tablet_v2: *mut wlr_tablet_manager_v2,
    pub xwayland: sway_xwayland,
    pub xwayland_surface: wl_listener,
    pub xwayland_ready: wl_listener,
    pub relative_pointer_manager: *mut wlr_relative_pointer_manager_v1,
    pub server_decoration_manager: *mut wlr_server_decoration_manager,
    pub server_decoration: wl_listener,
    pub decorations: wl_list,
    pub xdg_decoration_manager: *mut wlr_xdg_decoration_manager_v1,
    pub xdg_decoration: wl_listener,
    pub xdg_decorations: wl_list,
    pub drm_lease_manager: *mut wlr_drm_lease_v1_manager,
    pub drm_lease_request: wl_listener,
    pub pointer_constraints: *mut wlr_pointer_constraints_v1,
    pub pointer_constraint: wl_listener,
    pub xdg_output_manager_v1: *mut wlr_xdg_output_manager_v1,
    pub output_manager_v1: *mut wlr_output_manager_v1,
    pub output_manager_apply: wl_listener,
    pub output_manager_test: wl_listener,
    pub gamma_control_manager_v1: *mut wlr_gamma_control_manager_v1,
    pub gamma_control_set_gamma: wl_listener,
    pub session_lock: C2Rust_Unnamed_59,
    pub output_power_manager_v1: *mut wlr_output_power_manager_v1,
    pub output_power_manager_set_mode: wl_listener,
    pub input_method: *mut wlr_input_method_manager_v2,
    pub text_input: *mut wlr_text_input_manager_v3,
    pub foreign_toplevel_list: *mut wlr_ext_foreign_toplevel_list_v1,
    pub foreign_toplevel_manager: *mut wlr_foreign_toplevel_manager_v1,
    pub content_type_manager_v1: *mut wlr_content_type_manager_v1,
    pub wlr_data_control_manager_v1: *mut wlr_data_control_manager_v1,
    pub ext_data_control_manager_v1: *mut wlr_ext_data_control_manager_v1,
    pub screencopy_manager_v1: *mut wlr_screencopy_manager_v1,
    pub ext_image_copy_capture_manager_v1: *mut wlr_ext_image_copy_capture_manager_v1,
    pub export_dmabuf_manager_v1: *mut wlr_export_dmabuf_manager_v1,
    pub security_context_manager_v1: *mut wlr_security_context_manager_v1,
    pub ext_foreign_toplevel_image_capture_source_manager_v1: *mut wlr_ext_foreign_toplevel_image_capture_source_manager_v1,
    pub new_foreign_toplevel_capture_request: wl_listener,
    pub xdg_activation_v1: *mut wlr_xdg_activation_v1,
    pub xdg_activation_v1_request_activate: wl_listener,
    pub xdg_activation_v1_new_token: wl_listener,
    pub xdg_toplevel_tag_manager_v1_set_tag: wl_listener,
    pub request_set_cursor_shape: wl_listener,
    pub tearing_control_v1: *mut wlr_tearing_control_manager_v1,
    pub tearing_control_new_object: wl_listener,
    pub tearing_controllers: wl_list,
    pub workspace_manager_v1: *mut wlr_ext_workspace_manager_v1,
    pub workspace_manager_v1_commit: wl_listener,
    pub pending_launcher_ctxs: wl_list,
    pub txn_timeout_ms: size_t,
    pub queued_transaction: *mut sway_transaction,
    pub pending_transaction: *mut sway_transaction,
    pub dirty_nodes: *mut list_t,
    pub delayed_modeset: *mut wl_event_source,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tearing_control_manager_v1 {
    pub global: *mut wl_global,
    pub surface_hints: wl_list,
    pub events: C2Rust_Unnamed_54,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_53,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_53 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_54 {
    pub new_object: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_text_input_manager_v3 {
    pub global: *mut wl_global,
    pub text_inputs: wl_list,
    pub events: C2Rust_Unnamed_56,
    pub WLR_PRIVATE: C2Rust_Unnamed_55,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_56 {
    pub new_text_input: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_manager_v2 {
    pub global: *mut wl_global,
    pub input_methods: wl_list,
    pub events: C2Rust_Unnamed_58,
    pub WLR_PRIVATE: C2Rust_Unnamed_57,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_57 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_58 {
    pub new_input_method: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_59 {
    pub lock: *mut sway_session_lock,
    pub manager: *mut wlr_session_lock_manager_v1,
    pub new_lock: wl_listener,
    pub manager_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_session_lock {
    pub lock: *mut wlr_session_lock_v1,
    pub focused: *mut wlr_surface,
    pub abandoned: bool,
    pub outputs: wl_list,
    pub new_surface: wl_listener,
    pub unlock: wl_listener,
    pub destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_constraints_v1 {
    pub global: *mut wl_global,
    pub constraints: wl_list,
    pub events: C2Rust_Unnamed_61,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_60,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_60 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_61 {
    pub destroy: wl_signal,
    pub new_constraint: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_xwayland {
    pub wlr_xwayland: *mut wlr_xwayland,
    pub xcursor_manager: *mut wlr_xcursor_manager,
    pub atoms: [xcb_atom_t; 11],
}
pub type xcb_atom_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xwayland {
    pub server: *mut wlr_xwayland_server,
    pub own_server: bool,
    pub xwm: *mut wlr_xwm,
    pub shell_v1: *mut wlr_xwayland_shell_v1,
    pub display_name: *const ::core::ffi::c_char,
    pub wl_display: *mut wl_display,
    pub compositor: *mut wlr_compositor,
    pub seat: *mut wlr_seat,
    pub events: C2Rust_Unnamed_64,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_62,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_63,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
    pub x: int32_t,
    pub y: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_generic_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub pad: [uint32_t; 7],
    pub full_sequence: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
    pub destroy: wl_signal,
    pub ready: wl_signal,
    pub new_surface: wl_signal,
    pub remove_startup_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat {
    pub global: *mut wl_global,
    pub display: *mut wl_display,
    pub clients: wl_list,
    pub name: *mut ::core::ffi::c_char,
    pub capabilities: uint32_t,
    pub accumulated_capabilities: uint32_t,
    pub selection_source: *mut wlr_data_source,
    pub selection_serial: uint32_t,
    pub selection_offers: wl_list,
    pub primary_selection_source: *mut wlr_primary_selection_source,
    pub primary_selection_serial: uint32_t,
    pub drag: *mut wlr_drag,
    pub drag_source: *mut wlr_data_source,
    pub drag_serial: uint32_t,
    pub drag_offers: wl_list,
    pub pointer_state: wlr_seat_pointer_state,
    pub keyboard_state: wlr_seat_keyboard_state,
    pub touch_state: wlr_seat_touch_state,
    pub events: C2Rust_Unnamed_66,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_65,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_65 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
    pub pointer_grab_begin: wl_signal,
    pub pointer_grab_end: wl_signal,
    pub keyboard_grab_begin: wl_signal,
    pub keyboard_grab_end: wl_signal,
    pub touch_grab_begin: wl_signal,
    pub touch_grab_end: wl_signal,
    pub request_set_cursor: wl_signal,
    pub request_set_selection: wl_signal,
    pub set_selection: wl_signal,
    pub request_set_primary_selection: wl_signal,
    pub set_primary_selection: wl_signal,
    pub request_start_drag: wl_signal,
    pub start_drag: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_touch_state {
    pub seat: *mut wlr_seat,
    pub touch_points: wl_list,
    pub grab_serial: uint32_t,
    pub grab_id: uint32_t,
    pub grab: *mut wlr_seat_touch_grab,
    pub default_grab: *mut wlr_seat_touch_grab,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_touch_grab {
    pub interface: *const wlr_touch_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_touch_grab_interface {
    pub down: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_touch_grab,
            uint32_t,
            *mut wlr_touch_point,
        ) -> uint32_t,
    >,
    pub up: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_touch_grab,
            uint32_t,
            *mut wlr_touch_point,
        ) -> uint32_t,
    >,
    pub motion: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_touch_grab,
            uint32_t,
            *mut wlr_touch_point,
        ) -> (),
    >,
    pub enter: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_touch_grab,
            uint32_t,
            *mut wlr_touch_point,
        ) -> (),
    >,
    pub frame: Option<unsafe extern "C" fn(*mut wlr_seat_touch_grab) -> ()>,
    pub cancel: Option<unsafe extern "C" fn(*mut wlr_seat_touch_grab) -> ()>,
    pub wl_cancel: Option<
        unsafe extern "C" fn(*mut wlr_seat_touch_grab, *mut wlr_seat_client) -> (),
    >,
    pub clear_focus: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_touch_grab,
            uint32_t,
            *mut wlr_touch_point,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_touch_point {
    pub touch_id: int32_t,
    pub surface: *mut wlr_surface,
    pub client: *mut wlr_seat_client,
    pub focus_surface: *mut wlr_surface,
    pub focus_client: *mut wlr_seat_client,
    pub sx: ::core::ffi::c_double,
    pub sy: ::core::ffi::c_double,
    pub events: C2Rust_Unnamed_68,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_67,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_client {
    pub client: *mut wl_client,
    pub seat: *mut wlr_seat,
    pub link: wl_list,
    pub resources: wl_list,
    pub pointers: wl_list,
    pub keyboards: wl_list,
    pub touches: wl_list,
    pub data_devices: wl_list,
    pub events: C2Rust_Unnamed_70,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_69,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
    pub acc_discrete: [int32_t; 2],
    pub last_discrete: [int32_t; 2],
    pub acc_axis: [::core::ffi::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_serial_ringset {
    pub data: [wlr_serial_range; 128],
    pub end: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_serial_range {
    pub min_incl: uint32_t,
    pub max_incl: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_keyboard_state {
    pub seat: *mut wlr_seat,
    pub keyboard: *mut wlr_keyboard,
    pub focused_client: *mut wlr_seat_client,
    pub focused_surface: *mut wlr_surface,
    pub grab: *mut wlr_seat_keyboard_grab,
    pub default_grab: *mut wlr_seat_keyboard_grab,
    pub events: C2Rust_Unnamed_72,
    pub WLR_PRIVATE: C2Rust_Unnamed_71,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_71 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
    pub focus_change: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_keyboard_grab {
    pub interface: *const wlr_keyboard_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_grab_interface {
    pub enter: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_keyboard_grab,
            *mut wlr_surface,
            *const uint32_t,
            size_t,
            *const wlr_keyboard_modifiers,
        ) -> (),
    >,
    pub clear_focus: Option<unsafe extern "C" fn(*mut wlr_seat_keyboard_grab) -> ()>,
    pub key: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_keyboard_grab,
            uint32_t,
            uint32_t,
            uint32_t,
        ) -> (),
    >,
    pub modifiers: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_keyboard_grab,
            *const wlr_keyboard_modifiers,
        ) -> (),
    >,
    pub cancel: Option<unsafe extern "C" fn(*mut wlr_seat_keyboard_grab) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_modifiers {
    pub depressed: xkb_mod_mask_t,
    pub latched: xkb_mod_mask_t,
    pub locked: xkb_mod_mask_t,
    pub group: xkb_layout_index_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard {
    pub base: wlr_input_device,
    pub r#impl: *const wlr_keyboard_impl,
    pub group: *mut wlr_keyboard_group,
    pub keymap_string: *mut ::core::ffi::c_char,
    pub keymap_size: size_t,
    pub keymap_fd: ::core::ffi::c_int,
    pub keymap: *mut xkb_keymap,
    pub xkb_state: *mut xkb_state,
    pub led_indexes: [xkb_led_index_t; 5],
    pub mod_indexes: [xkb_mod_index_t; 8],
    pub leds: uint32_t,
    pub keycodes: [uint32_t; 32],
    pub num_keycodes: size_t,
    pub modifiers: wlr_keyboard_modifiers,
    pub repeat_info: C2Rust_Unnamed_74,
    pub events: C2Rust_Unnamed_73,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_73 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_74 {
    pub rate: int32_t,
    pub delay: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_pointer_state {
    pub seat: *mut wlr_seat,
    pub focused_client: *mut wlr_seat_client,
    pub focused_surface: *mut wlr_surface,
    pub sx: ::core::ffi::c_double,
    pub sy: ::core::ffi::c_double,
    pub grab: *mut wlr_seat_pointer_grab,
    pub default_grab: *mut wlr_seat_pointer_grab,
    pub sent_axis_source: bool,
    pub cached_axis_source: wl_pointer_axis_source,
    pub buttons: [wlr_seat_pointer_button; 16],
    pub button_count: size_t,
    pub grab_button: uint32_t,
    pub grab_serial: uint32_t,
    pub grab_time: uint32_t,
    pub events: C2Rust_Unnamed_76,
    pub WLR_PRIVATE: C2Rust_Unnamed_75,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_75 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_76 {
    pub focus_change: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_pointer_button {
    pub button: uint32_t,
    pub n_pressed: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_pointer_grab {
    pub interface: *const wlr_pointer_grab_interface,
    pub seat: *mut wlr_seat,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_grab_interface {
    pub enter: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_pointer_grab,
            *mut wlr_surface,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub clear_focus: Option<unsafe extern "C" fn(*mut wlr_seat_pointer_grab) -> ()>,
    pub motion: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_pointer_grab,
            uint32_t,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub button: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_pointer_grab,
            uint32_t,
            uint32_t,
            wl_pointer_button_state,
        ) -> uint32_t,
    >,
    pub axis: Option<
        unsafe extern "C" fn(
            *mut wlr_seat_pointer_grab,
            uint32_t,
            wl_pointer_axis,
            ::core::ffi::c_double,
            int32_t,
            wl_pointer_axis_source,
            wl_pointer_axis_relative_direction,
        ) -> (),
    >,
    pub frame: Option<unsafe extern "C" fn(*mut wlr_seat_pointer_grab) -> ()>,
    pub cancel: Option<unsafe extern "C" fn(*mut wlr_seat_pointer_grab) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xwayland_server {
    pub pid: pid_t,
    pub client: *mut wl_client,
    pub pipe_source: *mut wl_event_source,
    pub wm_fd: [::core::ffi::c_int; 2],
    pub wl_fd: [::core::ffi::c_int; 2],
    pub ready: bool,
    pub server_start: time_t,
    pub display: ::core::ffi::c_int,
    pub display_name: [::core::ffi::c_char; 16],
    pub x_fd: [::core::ffi::c_int; 2],
    pub x_fd_read_event: [*mut wl_event_source; 2],
    pub options: wlr_xwayland_server_options,
    pub wl_display: *mut wl_display,
    pub idle_source: *mut wl_event_source,
    pub events: C2Rust_Unnamed_78,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_77,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_77 {
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_78 {
    pub start: wl_signal,
    pub ready: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xwayland_server_options {
    pub lazy: bool,
    pub enable_wm: bool,
    pub no_touch_pointer_emulation: bool,
    pub force_xrandr_emulation: bool,
    pub terminate_delay: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_idle_inhibit_manager_v1 {
    pub wlr_manager: *mut wlr_idle_inhibit_manager_v1,
    pub new_idle_inhibitor_v1: wl_listener,
    pub manager_destroy: wl_listener,
    pub inhibitors: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_idle_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_80,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_79,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_79 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_80 {
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_input_manager {
    pub devices: wl_list,
    pub seats: wl_list,
    pub keyboard_shortcuts_inhibit: *mut wlr_keyboard_shortcuts_inhibit_manager_v1,
    pub virtual_keyboard: *mut wlr_virtual_keyboard_manager_v1,
    pub virtual_pointer: *mut wlr_virtual_pointer_manager_v1,
    pub pointer_gestures: *mut wlr_pointer_gestures_v1,
    pub transient_seat_manager: *mut wlr_transient_seat_manager_v1,
    pub new_input: wl_listener,
    pub inhibit_activate: wl_listener,
    pub inhibit_deactivate: wl_listener,
    pub keyboard_shortcuts_inhibit_new_inhibitor: wl_listener,
    pub virtual_keyboard_new: wl_listener,
    pub virtual_pointer_new: wl_listener,
    pub transient_seat_create: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_transient_seat_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_82,
    pub WLR_PRIVATE: C2Rust_Unnamed_81,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_81 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_82 {
    pub destroy: wl_signal,
    pub create_seat: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_gestures_v1 {
    pub global: *mut wl_global,
    pub swipes: wl_list,
    pub pinches: wl_list,
    pub holds: wl_list,
    pub events: C2Rust_Unnamed_84,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_83,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_83 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_84 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_pointers: wl_list,
    pub events: C2Rust_Unnamed_86,
    pub WLR_PRIVATE: C2Rust_Unnamed_85,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_85 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_86 {
    pub new_virtual_pointer: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_keyboards: wl_list,
    pub events: C2Rust_Unnamed_88,
    pub WLR_PRIVATE: C2Rust_Unnamed_87,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_87 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_88 {
    pub new_virtual_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_90,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_89,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_89 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_90 {
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_session {
    pub active: bool,
    pub seat: [::core::ffi::c_char; 256],
    pub udev: *mut udev,
    pub mon: *mut udev_monitor,
    pub udev_event: *mut wl_event_source,
    pub seat_handle: *mut libseat,
    pub libseat_event: *mut wl_event_source,
    pub devices: wl_list,
    pub event_loop: *mut wl_event_loop,
    pub events: C2Rust_Unnamed_92,
    pub WLR_PRIVATE: C2Rust_Unnamed_91,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_91 {
    pub event_loop_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_92 {
    pub active: wl_signal,
    pub add_drm_card: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_93 {
    pub shell_background: *mut wlr_scene_tree,
    pub shell_bottom: *mut wlr_scene_tree,
    pub tiling: *mut wlr_scene_tree,
    pub fullscreen: *mut wlr_scene_tree,
    pub shell_top: *mut wlr_scene_tree,
    pub shell_overlay: *mut wlr_scene_tree,
    pub session_lock: *mut wlr_scene_tree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_node {
    pub r#type: sway_node_type,
    pub c2rust_unnamed: C2Rust_Unnamed_95,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_94,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_94 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_95 {
    pub sway_root: *mut sway_root,
    pub sway_output: *mut sway_output,
    pub sway_workspace: *mut sway_workspace,
    pub sway_container: *mut sway_container,
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
pub struct side_gaps {
    pub top: ::core::ffi::c_int,
    pub right: ::core::ffi::c_int,
    pub bottom: ::core::ffi::c_int,
    pub left: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_96 {
    pub tiling: *mut wlr_scene_tree,
    pub fullscreen: *mut wlr_scene_tree,
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
pub struct C2Rust_Unnamed_97 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_98 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_100,
    pub events: C2Rust_Unnamed_99,
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
pub struct C2Rust_Unnamed_99 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_100 {
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
    pub events: C2Rust_Unnamed_102,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_101,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_101 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_102 {
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
pub struct C2Rust_Unnamed_103 {
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
pub struct sway_seat {
    pub wlr_seat: *mut wlr_seat,
    pub cursor: *mut sway_cursor,
    pub scene_tree: *mut wlr_scene_tree,
    pub drag_icons: *mut wlr_scene_tree,
    pub has_focus: bool,
    pub focus_stack: wl_list,
    pub workspace: *mut sway_workspace,
    pub prev_workspace_name: *mut ::core::ffi::c_char,
    pub focused_layer: *mut wlr_layer_surface_v1,
    pub has_exclusive_layer: bool,
    pub touch_id: int32_t,
    pub touch_x: ::core::ffi::c_double,
    pub touch_y: ::core::ffi::c_double,
    pub seatop_impl: *const sway_seatop_impl,
    pub seatop_data: *mut ::core::ffi::c_void,
    pub last_button_serial: uint32_t,
    pub idle_inhibit_sources: uint32_t,
    pub idle_wake_sources: uint32_t,
    pub deferred_bindings: *mut list_t,
    pub im_relay: sway_input_method_relay,
    pub focus_destroy: wl_listener,
    pub new_node: wl_listener,
    pub request_start_drag: wl_listener,
    pub start_drag: wl_listener,
    pub request_set_selection: wl_listener,
    pub request_set_primary_selection: wl_listener,
    pub destroy: wl_listener,
    pub devices: wl_list,
    pub keyboard_groups: wl_list,
    pub keyboard_shortcuts_inhibitors: wl_list,
    pub link: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_input_method_relay {
    pub seat: *mut sway_seat,
    pub text_inputs: wl_list,
    pub input_popups: wl_list,
    pub input_method: *mut wlr_input_method_v2,
    pub text_input_new: wl_listener,
    pub text_input_manager_destroy: wl_listener,
    pub input_method_new: wl_listener,
    pub input_method_manager_destroy: wl_listener,
    pub input_method_commit: wl_listener,
    pub input_method_new_popup_surface: wl_listener,
    pub input_method_grab_keyboard: wl_listener,
    pub input_method_destroy: wl_listener,
    pub input_method_keyboard_grab_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_v2 {
    pub resource: *mut wl_resource,
    pub seat: *mut wlr_seat,
    pub seat_client: *mut wlr_seat_client,
    pub pending: wlr_input_method_v2_state,
    pub current: wlr_input_method_v2_state,
    pub active: bool,
    pub client_active: bool,
    pub current_serial: uint32_t,
    pub popup_surfaces: wl_list,
    pub keyboard_grab: *mut wlr_input_method_keyboard_grab_v2,
    pub link: wl_list,
    pub events: C2Rust_Unnamed_105,
    pub WLR_PRIVATE: C2Rust_Unnamed_104,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_104 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_105 {
    pub commit: wl_signal,
    pub new_popup_surface: wl_signal,
    pub grab_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_keyboard_grab_v2 {
    pub resource: *mut wl_resource,
    pub input_method: *mut wlr_input_method_v2,
    pub keyboard: *mut wlr_keyboard,
    pub events: C2Rust_Unnamed_107,
    pub WLR_PRIVATE: C2Rust_Unnamed_106,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_106 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_107 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_v2_state {
    pub preedit: wlr_input_method_v2_preedit_string,
    pub commit_text: *mut ::core::ffi::c_char,
    pub delete: wlr_input_method_v2_delete_surrounding_text,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_v2_delete_surrounding_text {
    pub before_length: uint32_t,
    pub after_length: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_v2_preedit_string {
    pub text: *mut ::core::ffi::c_char,
    pub cursor_begin: int32_t,
    pub cursor_end: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_seatop_impl {
    pub button: Option<
        unsafe extern "C" fn(
            *mut sway_seat,
            uint32_t,
            *mut wlr_input_device,
            uint32_t,
            wl_pointer_button_state,
        ) -> (),
    >,
    pub pointer_motion: Option<unsafe extern "C" fn(*mut sway_seat, uint32_t) -> ()>,
    pub pointer_axis: Option<
        unsafe extern "C" fn(*mut sway_seat, *mut wlr_pointer_axis_event) -> (),
    >,
    pub hold_begin: Option<
        unsafe extern "C" fn(*mut sway_seat, *mut wlr_pointer_hold_begin_event) -> (),
    >,
    pub hold_end: Option<
        unsafe extern "C" fn(*mut sway_seat, *mut wlr_pointer_hold_end_event) -> (),
    >,
    pub pinch_begin: Option<
        unsafe extern "C" fn(*mut sway_seat, *mut wlr_pointer_pinch_begin_event) -> (),
    >,
    pub pinch_update: Option<
        unsafe extern "C" fn(*mut sway_seat, *mut wlr_pointer_pinch_update_event) -> (),
    >,
    pub pinch_end: Option<
        unsafe extern "C" fn(*mut sway_seat, *mut wlr_pointer_pinch_end_event) -> (),
    >,
    pub swipe_begin: Option<
        unsafe extern "C" fn(*mut sway_seat, *mut wlr_pointer_swipe_begin_event) -> (),
    >,
    pub swipe_update: Option<
        unsafe extern "C" fn(*mut sway_seat, *mut wlr_pointer_swipe_update_event) -> (),
    >,
    pub swipe_end: Option<
        unsafe extern "C" fn(*mut sway_seat, *mut wlr_pointer_swipe_end_event) -> (),
    >,
    pub rebase: Option<unsafe extern "C" fn(*mut sway_seat, uint32_t) -> ()>,
    pub touch_motion: Option<
        unsafe extern "C" fn(
            *mut sway_seat,
            *mut wlr_touch_motion_event,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub touch_up: Option<
        unsafe extern "C" fn(*mut sway_seat, *mut wlr_touch_up_event) -> (),
    >,
    pub touch_down: Option<
        unsafe extern "C" fn(
            *mut sway_seat,
            *mut wlr_touch_down_event,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub touch_cancel: Option<
        unsafe extern "C" fn(*mut sway_seat, *mut wlr_touch_cancel_event) -> (),
    >,
    pub tablet_tool_motion: Option<
        unsafe extern "C" fn(*mut sway_seat, *mut sway_tablet_tool, uint32_t) -> (),
    >,
    pub tablet_tool_tip: Option<
        unsafe extern "C" fn(
            *mut sway_seat,
            *mut sway_tablet_tool,
            uint32_t,
            wlr_tablet_tool_tip_state,
        ) -> (),
    >,
    pub end: Option<unsafe extern "C" fn(*mut sway_seat) -> ()>,
    pub unref: Option<unsafe extern "C" fn(*mut sway_seat, *mut sway_container) -> ()>,
    pub allow_set_cursor: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_tablet_tool {
    pub seat: *mut sway_seat,
    pub tablet: *mut sway_tablet,
    pub tablet_v2_tool: *mut wlr_tablet_v2_tablet_tool,
    pub mode: sway_tablet_tool_mode,
    pub tilt_x: ::core::ffi::c_double,
    pub tilt_y: ::core::ffi::c_double,
    pub set_cursor: wl_listener,
    pub tool_destroy: wl_listener,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_tablet_tool_mode(pub ::core::ffi::c_uint);
impl sway_tablet_tool_mode {
    pub const SWAY_TABLET_TOOL_MODE_ABSOLUTE: Self = Self(0);
    pub const SWAY_TABLET_TOOL_MODE_RELATIVE: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_tablet {
    pub link: wl_list,
    pub seat_device: *mut sway_seat_device,
    pub tablet_v2: *mut wlr_tablet_v2_tablet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_seat_device {
    pub sway_seat: *mut sway_seat,
    pub input_device: *mut sway_input_device,
    pub keyboard: *mut sway_keyboard,
    pub switch_device: *mut sway_switch,
    pub tablet: *mut sway_tablet,
    pub tablet_pad: *mut sway_tablet_pad,
    pub link: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_tablet_pad {
    pub link: wl_list,
    pub seat_device: *mut sway_seat_device,
    pub tablet: *mut sway_tablet,
    pub wlr: *mut wlr_tablet_pad,
    pub tablet_v2_pad: *mut wlr_tablet_v2_tablet_pad,
    pub attach: wl_listener,
    pub button: wl_listener,
    pub ring: wl_listener,
    pub strip: wl_listener,
    pub current_surface: *mut wlr_surface,
    pub surface_destroy: wl_listener,
    pub tablet_destroy: wl_listener,
}
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
pub struct wlr_touch_cancel_event {
    pub touch: *mut wlr_touch,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_touch {
    pub base: wlr_input_device,
    pub r#impl: *const wlr_touch_impl,
    pub output_name: *mut ::core::ffi::c_char,
    pub width_mm: ::core::ffi::c_double,
    pub height_mm: ::core::ffi::c_double,
    pub events: C2Rust_Unnamed_108,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_108 {
    pub down: wl_signal,
    pub up: wl_signal,
    pub motion: wl_signal,
    pub cancel: wl_signal,
    pub frame: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_touch_down_event {
    pub touch: *mut wlr_touch,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_touch_up_event {
    pub touch: *mut wlr_touch,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_touch_motion_event {
    pub touch: *mut wlr_touch,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_swipe_end_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub cancelled: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer {
    pub base: wlr_input_device,
    pub r#impl: *const wlr_pointer_impl,
    pub output_name: *mut ::core::ffi::c_char,
    pub buttons: [uint32_t; 16],
    pub button_count: size_t,
    pub events: C2Rust_Unnamed_109,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_109 {
    pub motion: wl_signal,
    pub motion_absolute: wl_signal,
    pub button: wl_signal,
    pub axis: wl_signal,
    pub frame: wl_signal,
    pub swipe_begin: wl_signal,
    pub swipe_update: wl_signal,
    pub swipe_end: wl_signal,
    pub pinch_begin: wl_signal,
    pub pinch_update: wl_signal,
    pub pinch_end: wl_signal,
    pub hold_begin: wl_signal,
    pub hold_end: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_impl {
    pub name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_swipe_update_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
    pub dx: ::core::ffi::c_double,
    pub dy: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_swipe_begin_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_pinch_end_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub cancelled: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_pinch_update_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
    pub dx: ::core::ffi::c_double,
    pub dy: ::core::ffi::c_double,
    pub scale: ::core::ffi::c_double,
    pub rotation: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_pinch_begin_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_hold_end_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub cancelled: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_hold_begin_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_axis_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub source: wl_pointer_axis_source,
    pub orientation: wl_pointer_axis,
    pub relative_direction: wl_pointer_axis_relative_direction,
    pub delta: ::core::ffi::c_double,
    pub delta_discrete: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_cursor {
    pub seat: *mut sway_seat,
    pub cursor: *mut wlr_cursor,
    pub previous: C2Rust_Unnamed_113,
    pub xcursor_manager: *mut wlr_xcursor_manager,
    pub tablets: wl_list,
    pub tablet_pads: wl_list,
    pub image: *const ::core::ffi::c_char,
    pub image_client: *mut wl_client,
    pub image_surface: *mut wlr_surface,
    pub hotspot_x: ::core::ffi::c_int,
    pub hotspot_y: ::core::ffi::c_int,
    pub active_constraint: *mut wlr_pointer_constraint_v1,
    pub confine: pixman_region32_t,
    pub active_confine_requires_warp: bool,
    pub hold_begin: wl_listener,
    pub hold_end: wl_listener,
    pub pinch_begin: wl_listener,
    pub pinch_update: wl_listener,
    pub pinch_end: wl_listener,
    pub swipe_begin: wl_listener,
    pub swipe_update: wl_listener,
    pub swipe_end: wl_listener,
    pub motion: wl_listener,
    pub motion_absolute: wl_listener,
    pub button: wl_listener,
    pub axis: wl_listener,
    pub frame: wl_listener,
    pub touch_down: wl_listener,
    pub touch_up: wl_listener,
    pub touch_cancel: wl_listener,
    pub touch_motion: wl_listener,
    pub touch_frame: wl_listener,
    pub simulating_pointer_from_touch: bool,
    pub pointer_touch_up: bool,
    pub pointer_touch_id: int32_t,
    pub tool_axis: wl_listener,
    pub tool_tip: wl_listener,
    pub tool_proximity: wl_listener,
    pub tool_button: wl_listener,
    pub simulating_pointer_from_tool_tip: bool,
    pub simulating_pointer_from_tool_button: bool,
    pub tool_buttons: uint32_t,
    pub request_set_cursor: wl_listener,
    pub image_surface_destroy: wl_listener,
    pub constraint_commit: wl_listener,
    pub hide_source: *mut wl_event_source,
    pub hidden: bool,
    pub hide_when_typing: seat_config_hide_cursor_when_typing,
    pub pressed_button_count: size_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct seat_config_hide_cursor_when_typing(pub ::core::ffi::c_uint);
impl seat_config_hide_cursor_when_typing {
    pub const HIDE_WHEN_TYPING_DEFAULT: Self = Self(0);
    pub const HIDE_WHEN_TYPING_ENABLE: Self = Self(1);
    pub const HIDE_WHEN_TYPING_DISABLE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_constraint_v1 {
    pub pointer_constraints: *mut wlr_pointer_constraints_v1,
    pub resource: *mut wl_resource,
    pub surface: *mut wlr_surface,
    pub seat: *mut wlr_seat,
    pub lifetime: zwp_pointer_constraints_v1_lifetime,
    pub r#type: wlr_pointer_constraint_v1_type,
    pub region: pixman_region32_t,
    pub current: wlr_pointer_constraint_v1_state,
    pub pending: wlr_pointer_constraint_v1_state,
    pub link: wl_list,
    pub events: C2Rust_Unnamed_111,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_110,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_110 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub synced: wlr_surface_synced,
    pub destroying: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_111 {
    pub set_region: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_constraint_v1_state {
    pub committed: uint32_t,
    pub region: pixman_region32_t,
    pub cursor_hint: C2Rust_Unnamed_112,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_112 {
    pub enabled: bool,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_pointer_constraint_v1_type(pub ::core::ffi::c_uint);
impl wlr_pointer_constraint_v1_type {
    pub const WLR_POINTER_CONSTRAINT_V1_LOCKED: Self = Self(0);
    pub const WLR_POINTER_CONSTRAINT_V1_CONFINED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct zwp_pointer_constraints_v1_lifetime(pub ::core::ffi::c_uint);
impl zwp_pointer_constraints_v1_lifetime {
    pub const ZWP_POINTER_CONSTRAINTS_V1_LIFETIME_ONESHOT: Self = Self(1);
    pub const ZWP_POINTER_CONSTRAINTS_V1_LIFETIME_PERSISTENT: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_113 {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub node: *mut sway_node,
}
pub type PangoFontDescription = _PangoFontDescription;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_fowa(pub ::core::ffi::c_uint);
impl sway_fowa {
    pub const FOWA_SMART: Self = Self(0);
    pub const FOWA_URGENT: Self = Self(1);
    pub const FOWA_FOCUS: Self = Self(2);
    pub const FOWA_NONE: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_mode {
    pub name: *mut ::core::ffi::c_char,
    pub keysym_bindings: *mut list_t,
    pub keycode_bindings: *mut list_t,
    pub mouse_bindings: *mut list_t,
    pub switch_bindings: *mut list_t,
    pub gesture_bindings: *mut list_t,
    pub pango: bool,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct seat_config_allow_constrain(pub ::core::ffi::c_uint);
impl seat_config_allow_constrain {
    pub const CONSTRAIN_DEFAULT: Self = Self(0);
    pub const CONSTRAIN_ENABLE: Self = Self(1);
    pub const CONSTRAIN_DISABLE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct seat_keyboard_grouping(pub ::core::ffi::c_uint);
impl seat_keyboard_grouping {
    pub const KEYBOARD_GROUP_DEFAULT: Self = Self(0);
    pub const KEYBOARD_GROUP_NONE: Self = Self(1);
    pub const KEYBOARD_GROUP_SMART: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seat_config {
    pub name: *mut ::core::ffi::c_char,
    pub fallback: ::core::ffi::c_int,
    pub attachments: *mut list_t,
    pub hide_cursor_timeout: ::core::ffi::c_int,
    pub hide_cursor_when_typing: seat_config_hide_cursor_when_typing,
    pub allow_constrain: seat_config_allow_constrain,
    pub shortcuts_inhibit: seat_config_shortcuts_inhibit,
    pub keyboard_grouping: seat_keyboard_grouping,
    pub idle_inhibit_sources: uint32_t,
    pub idle_wake_sources: uint32_t,
    pub xcursor_theme: C2Rust_Unnamed_114,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_114 {
    pub name: *mut ::core::ffi::c_char,
    pub size: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct render_bit_depth(pub ::core::ffi::c_uint);
impl render_bit_depth {
    pub const RENDER_BIT_DEPTH_DEFAULT: Self = Self(0);
    pub const RENDER_BIT_DEPTH_6: Self = Self(1);
    pub const RENDER_BIT_DEPTH_8: Self = Self(2);
    pub const RENDER_BIT_DEPTH_10: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct color_profile(pub ::core::ffi::c_uint);
impl color_profile {
    pub const COLOR_PROFILE_DEFAULT: Self = Self(0);
    pub const COLOR_PROFILE_TRANSFORM: Self = Self(1);
    pub const COLOR_PROFILE_TRANSFORM_WITH_DEVICE_PRIMARIES: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_config {
    pub name: *mut ::core::ffi::c_char,
    pub enabled: ::core::ffi::c_int,
    pub power: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub refresh_rate: ::core::ffi::c_float,
    pub custom_mode: ::core::ffi::c_int,
    pub drm_mode: drmModeModeInfo,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub scale: ::core::ffi::c_float,
    pub scale_filter: scale_filter_mode,
    pub transform: int32_t,
    pub subpixel: wl_output_subpixel,
    pub max_render_time: ::core::ffi::c_int,
    pub adaptive_sync: ::core::ffi::c_int,
    pub render_bit_depth: render_bit_depth,
    pub color_profile: color_profile,
    pub color_transform: *mut wlr_color_transform,
    pub allow_tearing: ::core::ffi::c_int,
    pub hdr: ::core::ffi::c_int,
    pub background: *mut ::core::ffi::c_char,
    pub background_option: *mut ::core::ffi::c_char,
    pub background_fallback: *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct smart_gaps_mode(pub ::core::ffi::c_uint);
impl smart_gaps_mode {
    pub const SMART_GAPS_OFF: Self = Self(0);
    pub const SMART_GAPS_ON: Self = Self(1);
    pub const SMART_GAPS_INVERSE_OUTER: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct pango_markup_config(pub ::core::ffi::c_uint);
impl pango_markup_config {
    pub const PANGO_MARKUP_DISABLED: Self = Self(0);
    pub const PANGO_MARKUP_ENABLED: Self = Self(1);
    pub const PANGO_MARKUP_DEFAULT: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bar_config {
    pub swaybar_command: *mut ::core::ffi::c_char,
    pub client: *mut wl_client,
    pub client_destroy: wl_listener,
    pub mode: *mut ::core::ffi::c_char,
    pub hidden_state: *mut ::core::ffi::c_char,
    pub visible_by_modifier: bool,
    pub id: *mut ::core::ffi::c_char,
    pub modifier: uint32_t,
    pub outputs: *mut list_t,
    pub position: *mut ::core::ffi::c_char,
    pub bindings: *mut list_t,
    pub status_command: *mut ::core::ffi::c_char,
    pub pango_markup: pango_markup_config,
    pub font: *mut ::core::ffi::c_char,
    pub height: ::core::ffi::c_int,
    pub workspace_buttons: bool,
    pub wrap_scroll: bool,
    pub separator_symbol: *mut ::core::ffi::c_char,
    pub strip_workspace_numbers: bool,
    pub strip_workspace_name: bool,
    pub binding_mode_indicator: bool,
    pub verbose: bool,
    pub gaps: side_gaps,
    pub status_padding: ::core::ffi::c_int,
    pub status_edge_padding: ::core::ffi::c_int,
    pub workspace_min_width: uint32_t,
    pub colors: C2Rust_Unnamed_115,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_115 {
    pub background: *mut ::core::ffi::c_char,
    pub statusline: *mut ::core::ffi::c_char,
    pub separator: *mut ::core::ffi::c_char,
    pub focused_background: *mut ::core::ffi::c_char,
    pub focused_statusline: *mut ::core::ffi::c_char,
    pub focused_separator: *mut ::core::ffi::c_char,
    pub focused_workspace_border: *mut ::core::ffi::c_char,
    pub focused_workspace_bg: *mut ::core::ffi::c_char,
    pub focused_workspace_text: *mut ::core::ffi::c_char,
    pub active_workspace_border: *mut ::core::ffi::c_char,
    pub active_workspace_bg: *mut ::core::ffi::c_char,
    pub active_workspace_text: *mut ::core::ffi::c_char,
    pub inactive_workspace_border: *mut ::core::ffi::c_char,
    pub inactive_workspace_bg: *mut ::core::ffi::c_char,
    pub inactive_workspace_text: *mut ::core::ffi::c_char,
    pub urgent_workspace_border: *mut ::core::ffi::c_char,
    pub urgent_workspace_bg: *mut ::core::ffi::c_char,
    pub urgent_workspace_text: *mut ::core::ffi::c_char,
    pub binding_mode_border: *mut ::core::ffi::c_char,
    pub binding_mode_bg: *mut ::core::ffi::c_char,
    pub binding_mode_text: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bar_binding {
    pub button: uint32_t,
    pub release: bool,
    pub command: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_binding {
    pub button: uint32_t,
    pub command: *const ::core::ffi::c_char,
    pub link: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct border_colors {
    pub border: [::core::ffi::c_float; 4],
    pub background: [::core::ffi::c_float; 4],
    pub text: [::core::ffi::c_float; 4],
    pub indicator: [::core::ffi::c_float; 4],
    pub child_border: [::core::ffi::c_float; 4],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct edge_border_types(pub ::core::ffi::c_uint);
impl edge_border_types {
    pub const E_NONE: Self = Self(0);
    pub const E_VERTICAL: Self = Self(1);
    pub const E_HORIZONTAL: Self = Self(2);
    pub const E_BOTH: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct edge_border_smart_types(pub ::core::ffi::c_uint);
impl edge_border_smart_types {
    pub const ESMART_OFF: Self = Self(0);
    pub const ESMART_ON: Self = Self(1);
    pub const ESMART_NO_GAPS: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_popup_during_fullscreen(pub ::core::ffi::c_uint);
impl sway_popup_during_fullscreen {
    pub const POPUP_SMART: Self = Self(0);
    pub const POPUP_IGNORE: Self = Self(1);
    pub const POPUP_LEAVE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct focus_follows_mouse_mode(pub ::core::ffi::c_uint);
impl focus_follows_mouse_mode {
    pub const FOLLOWS_NO: Self = Self(0);
    pub const FOLLOWS_YES: Self = Self(1);
    pub const FOLLOWS_ALWAYS: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct focus_wrapping_mode(pub ::core::ffi::c_uint);
impl focus_wrapping_mode {
    pub const WRAP_NO: Self = Self(0);
    pub const WRAP_YES: Self = Self(1);
    pub const WRAP_FORCE: Self = Self(2);
    pub const WRAP_WORKSPACE: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mouse_warping_mode(pub ::core::ffi::c_uint);
impl mouse_warping_mode {
    pub const WARP_NO: Self = Self(0);
    pub const WARP_OUTPUT: Self = Self(1);
    pub const WARP_CONTAINER: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct alignment(pub ::core::ffi::c_uint);
impl alignment {
    pub const ALIGN_LEFT: Self = Self(0);
    pub const ALIGN_CENTER: Self = Self(1);
    pub const ALIGN_RIGHT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xwayland_mode(pub ::core::ffi::c_uint);
impl xwayland_mode {
    pub const XWAYLAND_MODE_DISABLED: Self = Self(0);
    pub const XWAYLAND_MODE_LAZY: Self = Self(1);
    pub const XWAYLAND_MODE_IMMEDIATE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_config {
    pub swaynag_command: *mut ::core::ffi::c_char,
    pub swaynag_config_errors: swaynag_instance,
    pub symbols: *mut list_t,
    pub modes: *mut list_t,
    pub bars: *mut list_t,
    pub cmd_queue: *mut list_t,
    pub workspace_configs: *mut list_t,
    pub output_configs: *mut list_t,
    pub input_configs: *mut list_t,
    pub input_type_configs: *mut list_t,
    pub seat_configs: *mut list_t,
    pub criteria: *mut list_t,
    pub no_focus: *mut list_t,
    pub active_bar_modifiers: *mut list_t,
    pub current_mode: *mut sway_mode,
    pub current_bar: *mut bar_config,
    pub floating_mod: uint32_t,
    pub floating_mod_inverse: bool,
    pub dragging_key: uint32_t,
    pub resizing_key: uint32_t,
    pub floating_scroll_up_cmd: *mut ::core::ffi::c_char,
    pub floating_scroll_down_cmd: *mut ::core::ffi::c_char,
    pub floating_scroll_left_cmd: *mut ::core::ffi::c_char,
    pub floating_scroll_right_cmd: *mut ::core::ffi::c_char,
    pub default_orientation: sway_container_layout,
    pub default_layout: sway_container_layout,
    pub font: *mut ::core::ffi::c_char,
    pub font_description: *mut PangoFontDescription,
    pub font_height: ::core::ffi::c_int,
    pub font_baseline: ::core::ffi::c_int,
    pub pango_markup: bool,
    pub titlebar_border_thickness: ::core::ffi::c_int,
    pub titlebar_h_padding: ::core::ffi::c_int,
    pub titlebar_v_padding: ::core::ffi::c_int,
    pub urgent_timeout: size_t,
    pub focus_on_window_activation: sway_fowa,
    pub popup_during_fullscreen: sway_popup_during_fullscreen,
    pub xwayland: xwayland_mode,
    pub swaybg_command: *mut ::core::ffi::c_char,
    pub swaybg_client: *mut wl_client,
    pub swaybg_client_destroy: wl_listener,
    pub focus_follows_mouse: focus_follows_mouse_mode,
    pub mouse_warping: mouse_warping_mode,
    pub focus_wrapping: focus_wrapping_mode,
    pub active: bool,
    pub failed: bool,
    pub reloading: bool,
    pub reading: bool,
    pub validating: bool,
    pub auto_back_and_forth: bool,
    pub show_marks: bool,
    pub title_align: alignment,
    pub primary_selection: bool,
    pub tiling_drag: bool,
    pub tiling_drag_threshold: ::core::ffi::c_int,
    pub smart_gaps: smart_gaps_mode,
    pub gaps_inner: ::core::ffi::c_int,
    pub gaps_outer: side_gaps,
    pub config_chain: *mut list_t,
    pub user_config_path: bool,
    pub current_config_path: *const ::core::ffi::c_char,
    pub current_config: *const ::core::ffi::c_char,
    pub current_config_line_number: ::core::ffi::c_int,
    pub current_config_line: *mut ::core::ffi::c_char,
    pub border: sway_container_border,
    pub floating_border: sway_container_border,
    pub border_thickness: ::core::ffi::c_int,
    pub floating_border_thickness: ::core::ffi::c_int,
    pub hide_edge_borders: edge_border_types,
    pub hide_edge_borders_smart: edge_border_smart_types,
    pub hide_lone_tab: bool,
    pub border_colors: C2Rust_Unnamed_118,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_116,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_116 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_117,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_117 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_118 {
    pub focused: border_colors,
    pub focused_inactive: border_colors,
    pub focused_tab_title: border_colors,
    pub unfocused: border_colors,
    pub urgent: border_colors,
    pub placeholder: border_colors,
    pub background: [::core::ffi::c_float; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_output_non_desktop {
    pub wlr_output: *mut wlr_output,
    pub destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_idle_inhibitor_v1 {
    pub wlr_inhibitor: *mut wlr_idle_inhibitor_v1,
    pub view: *mut sway_view,
    pub mode: sway_idle_inhibit_mode,
    pub link: wl_list,
    pub destroy: wl_listener,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_idle_inhibit_mode(pub ::core::ffi::c_uint);
impl sway_idle_inhibit_mode {
    pub const INHIBIT_IDLE_APPLICATION: Self = Self(0);
    pub const INHIBIT_IDLE_FOCUS: Self = Self(1);
    pub const INHIBIT_IDLE_FULLSCREEN: Self = Self(2);
    pub const INHIBIT_IDLE_OPEN: Self = Self(3);
    pub const INHIBIT_IDLE_VISIBLE: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_idle_inhibitor_v1 {
    pub surface: *mut wlr_surface,
    pub resource: *mut wl_resource,
    pub link: wl_list,
    pub events: C2Rust_Unnamed_120,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_119,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_119 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_120 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct focus_inactive_data {
    pub node: *mut sway_node,
    pub object: *mut json_object,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct atom_name(pub ::core::ffi::c_uint);
impl atom_name {
    pub const NET_WM_WINDOW_TYPE_NORMAL: Self = Self(0);
    pub const NET_WM_WINDOW_TYPE_DIALOG: Self = Self(1);
    pub const NET_WM_WINDOW_TYPE_UTILITY: Self = Self(2);
    pub const NET_WM_WINDOW_TYPE_TOOLBAR: Self = Self(3);
    pub const NET_WM_WINDOW_TYPE_SPLASH: Self = Self(4);
    pub const NET_WM_WINDOW_TYPE_MENU: Self = Self(5);
    pub const NET_WM_WINDOW_TYPE_DROPDOWN_MENU: Self = Self(6);
    pub const NET_WM_WINDOW_TYPE_POPUP_MENU: Self = Self(7);
    pub const NET_WM_WINDOW_TYPE_TOOLTIP: Self = Self(8);
    pub const NET_WM_WINDOW_TYPE_NOTIFICATION: Self = Self(9);
    pub const NET_WM_STATE_MODAL: Self = Self(10);
    pub const ATOM_LAST: Self = Self(11);
}
pub const BTN_LEFT: uint32_t = 272 as uint32_t;
pub const BTN_RIGHT: uint32_t = 273 as uint32_t;
pub const BTN_MIDDLE: uint32_t = 274 as uint32_t;
pub const BTN_SIDE: uint32_t = 275 as uint32_t;
pub const BTN_EXTRA: uint32_t = 276 as uint32_t;
pub const KEY_MAX: ::core::ffi::c_int = 0x2ff as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
static mut i3_output_id: ::core::ffi::c_int = INT32_MAX;
static mut i3_scratch_id: ::core::ffi::c_int = INT32_MAX - 1 as ::core::ffi::c_int;
unsafe extern "C" fn ipc_json_node_type_description(
    mut node_type: sway_node_type,
) -> *const ::core::ffi::c_char {
    match node_type {
        sway_node_type::N_ROOT => return b"root\0".as_ptr() as *const ::core::ffi::c_char,
        sway_node_type::N_OUTPUT => {
            return b"output\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_node_type::N_WORKSPACE => {
            return b"workspace\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_node_type::N_CONTAINER => {
            return b"con\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"none\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn ipc_json_layout_description(
    mut l: sway_container_layout,
) -> *const ::core::ffi::c_char {
    match l {
        sway_container_layout(2) => {
            return b"splitv\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_container_layout(1) => {
            return b"splith\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_container_layout(4) => {
            return b"tabbed\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_container_layout(3) => {
            return b"stacked\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_container_layout(0) | _ => {}
    }
    return b"none\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn ipc_json_orientation_description(
    mut l: sway_container_layout,
) -> *const ::core::ffi::c_char {
    match l {
        sway_container_layout(2) => {
            return b"vertical\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_container_layout(1) => {
            return b"horizontal\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return b"none\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
unsafe extern "C" fn ipc_json_border_description(
    mut border: sway_container_border,
) -> *const ::core::ffi::c_char {
    match border {
        sway_container_border::B_NONE => {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_container_border::B_PIXEL => {
            return b"pixel\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_container_border::B_NORMAL => {
            return b"normal\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_container_border::B_CSD => {
            return b"csd\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn ipc_json_output_transform_description(
    mut transform: wl_output_transform,
) -> *const ::core::ffi::c_char {
    match transform {
        wl_output_transform::WL_OUTPUT_TRANSFORM_NORMAL => {
            return b"normal\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wl_output_transform::WL_OUTPUT_TRANSFORM_90 => {
            return b"270\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wl_output_transform::WL_OUTPUT_TRANSFORM_180 => {
            return b"180\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wl_output_transform::WL_OUTPUT_TRANSFORM_270 => {
            return b"90\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wl_output_transform::WL_OUTPUT_TRANSFORM_FLIPPED => {
            return b"flipped\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wl_output_transform::WL_OUTPUT_TRANSFORM_FLIPPED_90 => {
            return b"flipped-270\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wl_output_transform::WL_OUTPUT_TRANSFORM_FLIPPED_180 => {
            return b"flipped-180\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wl_output_transform::WL_OUTPUT_TRANSFORM_FLIPPED_270 => {
            return b"flipped-90\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn ipc_json_output_adaptive_sync_status_description(
    mut status: wlr_output_adaptive_sync_status,
) -> *const ::core::ffi::c_char {
    match status {
        wlr_output_adaptive_sync_status::WLR_OUTPUT_ADAPTIVE_SYNC_DISABLED => {
            return b"disabled\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wlr_output_adaptive_sync_status::WLR_OUTPUT_ADAPTIVE_SYNC_ENABLED => {
            return b"enabled\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn ipc_json_output_mode_aspect_ratio_description(
    mut aspect_ratio: wlr_output_mode_aspect_ratio,
) -> *const ::core::ffi::c_char {
    match aspect_ratio {
        wlr_output_mode_aspect_ratio::WLR_OUTPUT_MODE_ASPECT_RATIO_NONE => {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wlr_output_mode_aspect_ratio::WLR_OUTPUT_MODE_ASPECT_RATIO_4_3 => {
            return b"4:3\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wlr_output_mode_aspect_ratio::WLR_OUTPUT_MODE_ASPECT_RATIO_16_9 => {
            return b"16:9\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wlr_output_mode_aspect_ratio::WLR_OUTPUT_MODE_ASPECT_RATIO_64_27 => {
            return b"64:27\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wlr_output_mode_aspect_ratio::WLR_OUTPUT_MODE_ASPECT_RATIO_256_135 => {
            return b"256:135\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn ipc_json_output_mode_description(
    mut mode: *const wlr_output_mode,
) -> *mut json_object {
    let mut pic_ar: *const ::core::ffi::c_char = ipc_json_output_mode_aspect_ratio_description(
        (*mode).picture_aspect_ratio,
    );
    let mut mode_object: *mut json_object = json_object_new_object();
    json_object_object_add(
        mode_object,
        b"width\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*mode).width),
    );
    json_object_object_add(
        mode_object,
        b"height\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*mode).height),
    );
    json_object_object_add(
        mode_object,
        b"refresh\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*mode).refresh),
    );
    json_object_object_add(
        mode_object,
        b"picture_aspect_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(pic_ar),
    );
    return mode_object;
}
unsafe extern "C" fn ipc_json_xwindow_type_description(
    mut view: *mut sway_view,
) -> *const ::core::ffi::c_char {
    let mut surface: *mut wlr_xwayland_surface = (*view)
        .c2rust_unnamed
        .wlr_xwayland_surface;
    let mut xwayland: *mut sway_xwayland = &raw mut server.xwayland;
    let mut i: size_t = 0 as size_t;
    if i < (*surface).window_type_len {
        let mut r#type: xcb_atom_t = *(*surface).window_type.offset(i as isize);
        if r#type
            == (*xwayland)
                .atoms[atom_name::NET_WM_WINDOW_TYPE_NORMAL.0 as ::core::ffi::c_int
                as usize]
        {
            return b"normal\0".as_ptr() as *const ::core::ffi::c_char
        } else if r#type
            == (*xwayland)
                .atoms[atom_name::NET_WM_WINDOW_TYPE_DIALOG.0 as ::core::ffi::c_int
                as usize]
        {
            return b"dialog\0".as_ptr() as *const ::core::ffi::c_char
        } else if r#type
            == (*xwayland)
                .atoms[atom_name::NET_WM_WINDOW_TYPE_UTILITY.0 as ::core::ffi::c_int
                as usize]
        {
            return b"utility\0".as_ptr() as *const ::core::ffi::c_char
        } else if r#type
            == (*xwayland)
                .atoms[atom_name::NET_WM_WINDOW_TYPE_TOOLBAR.0 as ::core::ffi::c_int
                as usize]
        {
            return b"toolbar\0".as_ptr() as *const ::core::ffi::c_char
        } else if r#type
            == (*xwayland)
                .atoms[atom_name::NET_WM_WINDOW_TYPE_SPLASH.0 as ::core::ffi::c_int
                as usize]
        {
            return b"splash\0".as_ptr() as *const ::core::ffi::c_char
        } else if r#type
            == (*xwayland)
                .atoms[atom_name::NET_WM_WINDOW_TYPE_MENU.0 as ::core::ffi::c_int
                as usize]
        {
            return b"menu\0".as_ptr() as *const ::core::ffi::c_char
        } else if r#type
            == (*xwayland)
                .atoms[atom_name::NET_WM_WINDOW_TYPE_DROPDOWN_MENU.0
                as ::core::ffi::c_int as usize]
        {
            return b"dropdown_menu\0".as_ptr() as *const ::core::ffi::c_char
        } else if r#type
            == (*xwayland)
                .atoms[atom_name::NET_WM_WINDOW_TYPE_POPUP_MENU.0 as ::core::ffi::c_int
                as usize]
        {
            return b"popup_menu\0".as_ptr() as *const ::core::ffi::c_char
        } else if r#type
            == (*xwayland)
                .atoms[atom_name::NET_WM_WINDOW_TYPE_TOOLTIP.0 as ::core::ffi::c_int
                as usize]
        {
            return b"tooltip\0".as_ptr() as *const ::core::ffi::c_char
        } else if r#type
            == (*xwayland)
                .atoms[atom_name::NET_WM_WINDOW_TYPE_NOTIFICATION.0 as ::core::ffi::c_int
                as usize]
        {
            return b"notification\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            return b"unknown\0".as_ptr() as *const ::core::ffi::c_char
        }
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn ipc_json_user_idle_inhibitor_description(
    mut mode: sway_idle_inhibit_mode,
) -> *const ::core::ffi::c_char {
    match mode {
        sway_idle_inhibit_mode::INHIBIT_IDLE_FOCUS => {
            return b"focus\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_idle_inhibit_mode::INHIBIT_IDLE_FULLSCREEN => {
            return b"fullscreen\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_idle_inhibit_mode::INHIBIT_IDLE_OPEN => {
            return b"open\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_idle_inhibit_mode::INHIBIT_IDLE_VISIBLE => {
            return b"visible\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_idle_inhibit_mode::INHIBIT_IDLE_APPLICATION => {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        _ => {}
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn ipc_json_content_type_description(
    mut r#type: wp_content_type_v1_type,
) -> *const ::core::ffi::c_char {
    match r#type {
        wp_content_type_v1_type::WP_CONTENT_TYPE_V1_TYPE_NONE => {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wp_content_type_v1_type::WP_CONTENT_TYPE_V1_TYPE_PHOTO => {
            return b"photo\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wp_content_type_v1_type::WP_CONTENT_TYPE_V1_TYPE_VIDEO => {
            return b"video\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wp_content_type_v1_type::WP_CONTENT_TYPE_V1_TYPE_GAME => {
            return b"game\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn ipc_json_get_version() -> *mut json_object {
    let mut major: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut minor: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut patch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut version: *mut json_object = json_object_new_object();
    sscanf(
        SWAY_VERSION.as_ptr(),
        b"%d.%d.%d\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut major,
        &raw mut minor,
        &raw mut patch,
    );
    json_object_object_add(
        version,
        b"human_readable\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(SWAY_VERSION.as_ptr()),
    );
    json_object_object_add(
        version,
        b"variant\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(b"sway\0".as_ptr() as *const ::core::ffi::c_char),
    );
    json_object_object_add(
        version,
        b"major\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(major as int32_t),
    );
    json_object_object_add(
        version,
        b"minor\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(minor as int32_t),
    );
    json_object_object_add(
        version,
        b"patch\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(patch as int32_t),
    );
    json_object_object_add(
        version,
        b"loaded_config_file_name\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*config).current_config_path),
    );
    return version;
}
unsafe extern "C" fn ipc_json_create_rect(mut r#box: *mut wlr_box) -> *mut json_object {
    let mut rect: *mut json_object = json_object_new_object();
    json_object_object_add(
        rect,
        b"x\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*r#box).x as int32_t),
    );
    json_object_object_add(
        rect,
        b"y\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*r#box).y as int32_t),
    );
    json_object_object_add(
        rect,
        b"width\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*r#box).width as int32_t),
    );
    json_object_object_add(
        rect,
        b"height\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*r#box).height as int32_t),
    );
    return rect;
}
unsafe extern "C" fn ipc_json_create_empty_rect() -> *mut json_object {
    let mut empty: wlr_box = wlr_box {
        x: 0 as ::core::ffi::c_int,
        y: 0 as ::core::ffi::c_int,
        width: 0 as ::core::ffi::c_int,
        height: 0 as ::core::ffi::c_int,
    };
    return ipc_json_create_rect(&raw mut empty);
}
unsafe extern "C" fn ipc_json_create_node(
    mut id: ::core::ffi::c_int,
    mut r#type: *const ::core::ffi::c_char,
    mut name: *mut ::core::ffi::c_char,
    mut focused: bool,
    mut focus: *mut json_object,
    mut r#box: *mut wlr_box,
) -> *mut json_object {
    let mut object: *mut json_object = json_object_new_object();
    json_object_object_add(
        object,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(id as int32_t),
    );
    json_object_object_add(
        object,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(r#type),
    );
    json_object_object_add(
        object,
        b"orientation\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(
            ipc_json_orientation_description(
                sway_container_layout(
                    sway_container_layout_0::L_HORIZ.0 as ::core::ffi::c_int
                        as ::core::ffi::c_uint,
                ),
            ),
        ),
    );
    json_object_object_add(
        object,
        b"percent\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<json_object>(),
    );
    json_object_object_add(
        object,
        b"urgent\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(r#false),
    );
    json_object_object_add(
        object,
        b"marks\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_array(),
    );
    json_object_object_add(
        object,
        b"focused\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(focused as json_bool),
    );
    json_object_object_add(
        object,
        b"layout\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(
            ipc_json_layout_description(
                sway_container_layout(
                    sway_container_layout_0::L_HORIZ.0 as ::core::ffi::c_int
                        as ::core::ffi::c_uint,
                ),
            ),
        ),
    );
    json_object_object_add(
        object,
        b"border\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(
            ipc_json_border_description(sway_container_border::B_NONE),
        ),
    );
    json_object_object_add(
        object,
        b"current_border_width\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(0 as int32_t),
    );
    json_object_object_add(
        object,
        b"rect\0".as_ptr() as *const ::core::ffi::c_char,
        ipc_json_create_rect(r#box),
    );
    json_object_object_add(
        object,
        b"deco_rect\0".as_ptr() as *const ::core::ffi::c_char,
        ipc_json_create_empty_rect(),
    );
    json_object_object_add(
        object,
        b"window_rect\0".as_ptr() as *const ::core::ffi::c_char,
        ipc_json_create_empty_rect(),
    );
    json_object_object_add(
        object,
        b"geometry\0".as_ptr() as *const ::core::ffi::c_char,
        ipc_json_create_empty_rect(),
    );
    json_object_object_add(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        if !name.is_null() {
            json_object_new_string(name)
        } else {
            ::core::ptr::null_mut::<json_object>()
        },
    );
    json_object_object_add(
        object,
        b"window\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<json_object>(),
    );
    json_object_object_add(
        object,
        b"nodes\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_array(),
    );
    json_object_object_add(
        object,
        b"floating_nodes\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_array(),
    );
    json_object_object_add(
        object,
        b"focus\0".as_ptr() as *const ::core::ffi::c_char,
        focus,
    );
    json_object_object_add(
        object,
        b"fullscreen_mode\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(0 as int32_t),
    );
    json_object_object_add(
        object,
        b"sticky\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(r#false),
    );
    json_object_object_add(
        object,
        b"floating\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<json_object>(),
    );
    json_object_object_add(
        object,
        b"scratchpad_state\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<json_object>(),
    );
    return object;
}
unsafe extern "C" fn ipc_json_describe_wlr_output(
    mut wlr_output: *mut wlr_output,
    mut object: *mut json_object,
) {
    json_object_object_add(
        object,
        b"primary\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(r#false),
    );
    json_object_object_add(
        object,
        b"make\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(
            if !(*wlr_output).make.is_null() {
                (*wlr_output).make
            } else {
                b"Unknown\0".as_ptr() as *const ::core::ffi::c_char
            },
        ),
    );
    json_object_object_add(
        object,
        b"model\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(
            if !(*wlr_output).model.is_null() {
                (*wlr_output).model
            } else {
                b"Unknown\0".as_ptr() as *const ::core::ffi::c_char
            },
        ),
    );
    json_object_object_add(
        object,
        b"serial\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(
            if !(*wlr_output).serial.is_null() {
                (*wlr_output).serial
            } else {
                b"Unknown\0".as_ptr() as *const ::core::ffi::c_char
            },
        ),
    );
    let mut modes_array: *mut json_object = json_object_new_array();
    let mut mode: *mut wlr_output_mode = ::core::ptr::null_mut::<wlr_output_mode>();
    mode = ((*wlr_output).modes.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut wlr_output_mode
        as *mut wlr_output_mode;
    while &raw mut (*mode).link != &raw mut (*wlr_output).modes {
        let mut mode_object: *mut json_object = json_object_new_object();
        json_object_object_add(
            mode_object,
            b"width\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int((*mode).width),
        );
        json_object_object_add(
            mode_object,
            b"height\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int((*mode).height),
        );
        json_object_object_add(
            mode_object,
            b"refresh\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int((*mode).refresh),
        );
        json_object_array_add(modes_array, mode_object);
        mode = ((*mode).link.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut wlr_output_mode
            as *mut wlr_output_mode;
    }
    json_object_object_add(
        object,
        b"modes\0".as_ptr() as *const ::core::ffi::c_char,
        modes_array,
    );
    let mut features_object: *mut json_object = json_object_new_object();
    json_object_object_add(
        features_object,
        b"adaptive_sync\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(
            ((*wlr_output).adaptive_sync_supported as ::core::ffi::c_int != 0
                || (*wlr_output).adaptive_sync_status.0
                    == wlr_output_adaptive_sync_status::WLR_OUTPUT_ADAPTIVE_SYNC_ENABLED
                        .0) as ::core::ffi::c_int,
        ),
    );
    json_object_object_add(
        features_object,
        b"hdr\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(
            output_supports_hdr(
                wlr_output,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ) as json_bool,
        ),
    );
    json_object_object_add(
        object,
        b"features\0".as_ptr() as *const ::core::ffi::c_char,
        features_object,
    );
}
unsafe extern "C" fn ipc_json_describe_output(
    mut output: *mut sway_output,
    mut object: *mut json_object,
) {
    ipc_json_describe_wlr_output((*output).wlr_output, object);
}
unsafe extern "C" fn ipc_json_describe_enabled_output(
    mut output: *mut sway_output,
    mut object: *mut json_object,
) {
    ipc_json_describe_output(output, object);
    let mut wlr_output: *mut wlr_output = (*output).wlr_output;
    json_object_object_add(
        object,
        b"non_desktop\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(r#false),
    );
    json_object_object_add(
        object,
        b"active\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(r#true),
    );
    json_object_object_add(
        object,
        b"dpms\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean((*wlr_output).enabled as json_bool),
    );
    json_object_object_add(
        object,
        b"power\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean((*wlr_output).enabled as json_bool),
    );
    json_object_object_add(
        object,
        b"layout\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(b"output\0".as_ptr() as *const ::core::ffi::c_char),
    );
    json_object_object_add(
        object,
        b"orientation\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(
            ipc_json_orientation_description(
                sway_container_layout(
                    sway_container_layout_0::L_NONE.0 as ::core::ffi::c_int
                        as ::core::ffi::c_uint,
                ),
            ),
        ),
    );
    json_object_object_add(
        object,
        b"scale\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_double((*wlr_output).scale as ::core::ffi::c_double),
    );
    json_object_object_add(
        object,
        b"scale_filter\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(
            sway_output_scale_filter_to_string((*output).scale_filter),
        ),
    );
    json_object_object_add(
        object,
        b"transform\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(
            ipc_json_output_transform_description((*wlr_output).transform),
        ),
    );
    let mut adaptive_sync_status: *const ::core::ffi::c_char = ipc_json_output_adaptive_sync_status_description(
        (*wlr_output).adaptive_sync_status,
    );
    json_object_object_add(
        object,
        b"adaptive_sync_status\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(adaptive_sync_status),
    );
    let mut ws: *mut sway_workspace = output_get_active_workspace(output);
    if !_sway_assert(
        !ws.is_null(),
        b"[%s:%d] %s:Expected output to have a workspace\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/ipc-json.c\0".as_ptr() as *const ::core::ffi::c_char,
        366 as ::core::ffi::c_int,
        b"void ipc_json_describe_enabled_output(struct sway_output *, json_object *)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    json_object_object_add(
        object,
        b"current_workspace\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*ws).name),
    );
    let mut modes_array: *mut json_object = json_object_new_array();
    let mut mode: *mut wlr_output_mode = ::core::ptr::null_mut::<wlr_output_mode>();
    mode = ((*wlr_output).modes.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut wlr_output_mode
        as *mut wlr_output_mode;
    while &raw mut (*mode).link != &raw mut (*wlr_output).modes {
        let mut mode_object: *mut json_object = ipc_json_output_mode_description(mode);
        json_object_array_add(modes_array, mode_object);
        mode = ((*mode).link.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut wlr_output_mode
            as *mut wlr_output_mode;
    }
    json_object_object_add(
        object,
        b"modes\0".as_ptr() as *const ::core::ffi::c_char,
        modes_array,
    );
    let mut current_mode_object: *mut json_object = ::core::ptr::null_mut::<
        json_object,
    >();
    if !(*wlr_output).current_mode.is_null() {
        current_mode_object = ipc_json_output_mode_description(
            (*wlr_output).current_mode,
        );
    } else {
        current_mode_object = json_object_new_object() as *mut json_object;
        json_object_object_add(
            current_mode_object,
            b"width\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int((*wlr_output).width),
        );
        json_object_object_add(
            current_mode_object,
            b"height\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int((*wlr_output).height),
        );
        json_object_object_add(
            current_mode_object,
            b"refresh\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int((*wlr_output).refresh),
        );
    }
    json_object_object_add(
        object,
        b"current_mode\0".as_ptr() as *const ::core::ffi::c_char,
        current_mode_object,
    );
    let mut parent: *mut sway_node = node_get_parent(&raw mut (*output).node);
    let mut parent_box: wlr_box = wlr_box {
        x: 0 as ::core::ffi::c_int,
        y: 0 as ::core::ffi::c_int,
        width: 0 as ::core::ffi::c_int,
        height: 0 as ::core::ffi::c_int,
    };
    if !parent.is_null() {
        node_get_box(parent, &raw mut parent_box);
    }
    if parent_box.width != 0 as ::core::ffi::c_int
        && parent_box.height != 0 as ::core::ffi::c_int
    {
        let mut percent: ::core::ffi::c_double = (*output).width as ::core::ffi::c_double
            / parent_box.width as ::core::ffi::c_double
            * ((*output).height as ::core::ffi::c_double
                / parent_box.height as ::core::ffi::c_double);
        json_object_object_add(
            object,
            b"percent\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_double(percent),
        );
    }
    json_object_object_add(
        object,
        b"max_render_time\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*output).max_render_time as int32_t),
    );
    json_object_object_add(
        object,
        b"allow_tearing\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean((*output).allow_tearing as json_bool),
    );
    json_object_object_add(
        object,
        b"hdr\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean((*output).hdr as json_bool),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ipc_json_describe_disabled_output(
    mut output: *mut sway_output,
) -> *mut json_object {
    let mut wlr_output: *mut wlr_output = (*output).wlr_output;
    let mut object: *mut json_object = json_object_new_object();
    ipc_json_describe_output(output, object);
    json_object_object_add(
        object,
        b"non_desktop\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(r#false),
    );
    json_object_object_add(
        object,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(b"output\0".as_ptr() as *const ::core::ffi::c_char),
    );
    json_object_object_add(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*wlr_output).name),
    );
    json_object_object_add(
        object,
        b"active\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(r#false),
    );
    json_object_object_add(
        object,
        b"dpms\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(r#false),
    );
    json_object_object_add(
        object,
        b"power\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(r#false),
    );
    json_object_object_add(
        object,
        b"current_workspace\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<json_object>(),
    );
    let mut rect_object: *mut json_object = json_object_new_object();
    json_object_object_add(
        rect_object,
        b"x\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(0 as int32_t),
    );
    json_object_object_add(
        rect_object,
        b"y\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(0 as int32_t),
    );
    json_object_object_add(
        rect_object,
        b"width\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(0 as int32_t),
    );
    json_object_object_add(
        rect_object,
        b"height\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(0 as int32_t),
    );
    json_object_object_add(
        object,
        b"rect\0".as_ptr() as *const ::core::ffi::c_char,
        rect_object,
    );
    json_object_object_add(
        object,
        b"percent\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<json_object>(),
    );
    return object;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_json_describe_non_desktop_output(
    mut output: *mut sway_output_non_desktop,
) -> *mut json_object {
    let mut wlr_output: *mut wlr_output = (*output).wlr_output;
    let mut object: *mut json_object = json_object_new_object();
    ipc_json_describe_wlr_output(wlr_output, object);
    json_object_object_add(
        object,
        b"non_desktop\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(r#true),
    );
    json_object_object_add(
        object,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(b"output\0".as_ptr() as *const ::core::ffi::c_char),
    );
    json_object_object_add(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*wlr_output).name),
    );
    return object;
}
unsafe extern "C" fn ipc_json_describe_scratchpad_output() -> *mut json_object {
    let mut r#box: wlr_box = wlr_box {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    root_get_box(root, &raw mut r#box);
    let mut workspace_focus: *mut json_object = json_object_new_array();
    let mut i: ::core::ffi::c_int = (*(*root).scratchpad).length
        - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut container: *mut sway_container = *(*(*root).scratchpad)
            .items
            .offset(i as isize) as *mut sway_container;
        json_object_array_add(
            workspace_focus,
            json_object_new_int((*container).node.id as int32_t),
        );
        i -= 1;
    }
    let mut workspace: *mut json_object = ipc_json_create_node(
        i3_scratch_id,
        b"workspace\0".as_ptr() as *const ::core::ffi::c_char,
        b"__i3_scratch\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        r#false != 0,
        workspace_focus,
        &raw mut r#box,
    );
    json_object_object_add(
        workspace,
        b"fullscreen_mode\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(1 as int32_t),
    );
    let mut floating_array: *mut json_object = json_object_new_array();
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*(*root).scratchpad).length {
        let mut container_0: *mut sway_container = *(*(*root).scratchpad)
            .items
            .offset(i_0 as isize) as *mut sway_container;
        if container_is_scratchpad_hidden(container_0) {
            json_object_array_add(
                floating_array,
                ipc_json_describe_node_recursive(&raw mut (*container_0).node),
            );
        }
        i_0 += 1;
    }
    json_object_object_add(
        workspace,
        b"floating_nodes\0".as_ptr() as *const ::core::ffi::c_char,
        floating_array,
    );
    let mut output_focus: *mut json_object = json_object_new_array();
    json_object_array_add(output_focus, json_object_new_int(i3_scratch_id as int32_t));
    let mut output: *mut json_object = ipc_json_create_node(
        i3_output_id,
        b"output\0".as_ptr() as *const ::core::ffi::c_char,
        b"__i3\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        r#false != 0,
        output_focus,
        &raw mut r#box,
    );
    json_object_object_add(
        output,
        b"layout\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(b"output\0".as_ptr() as *const ::core::ffi::c_char),
    );
    let mut nodes: *mut json_object = json_object_new_array();
    json_object_array_add(nodes, workspace);
    json_object_object_add(
        output,
        b"nodes\0".as_ptr() as *const ::core::ffi::c_char,
        nodes,
    );
    return output;
}
unsafe extern "C" fn ipc_json_describe_workspace(
    mut workspace: *mut sway_workspace,
    mut object: *mut json_object,
) {
    let mut num: ::core::ffi::c_int = 0;
    if *(*__ctype_b_loc())
        .offset(*(*workspace).name.offset(0isize) as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int != 0
    {
        *__errno_location() = 0 as ::core::ffi::c_int;
        let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut parsed_num: ::core::ffi::c_longlong = strtoll(
            (*workspace).name,
            &raw mut endptr,
            10 as ::core::ffi::c_int,
        );
        if *__errno_location() != 0 as ::core::ffi::c_int
            || parsed_num > INT32_MAX as ::core::ffi::c_longlong
            || parsed_num < 0 as ::core::ffi::c_longlong || endptr == (*workspace).name
        {
            num = -1 as ::core::ffi::c_int;
        } else {
            num = parsed_num as ::core::ffi::c_int;
        }
    } else {
        num = -1 as ::core::ffi::c_int;
    }
    json_object_object_add(
        object,
        b"num\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(num as int32_t),
    );
    json_object_object_add(
        object,
        b"fullscreen_mode\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(1 as int32_t),
    );
    json_object_object_add(
        object,
        b"output\0".as_ptr() as *const ::core::ffi::c_char,
        if !(*workspace).output.is_null() {
            json_object_new_string((*(*(*workspace).output).wlr_output).name)
        } else {
            ::core::ptr::null_mut::<json_object>()
        },
    );
    json_object_object_add(
        object,
        b"urgent\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean((*workspace).urgent as json_bool),
    );
    json_object_object_add(
        object,
        b"representation\0".as_ptr() as *const ::core::ffi::c_char,
        if !(*workspace).representation.is_null() {
            json_object_new_string((*workspace).representation)
        } else {
            ::core::ptr::null_mut::<json_object>()
        },
    );
    json_object_object_add(
        object,
        b"layout\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(ipc_json_layout_description((*workspace).layout)),
    );
    json_object_object_add(
        object,
        b"orientation\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(ipc_json_orientation_description((*workspace).layout)),
    );
    let mut floating_array: *mut json_object = json_object_new_array();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*workspace).floating).length {
        let mut floater: *mut sway_container = *(*(*workspace).floating)
            .items
            .offset(i as isize) as *mut sway_container;
        json_object_array_add(
            floating_array,
            ipc_json_describe_node_recursive(&raw mut (*floater).node),
        );
        i += 1;
    }
    json_object_object_add(
        object,
        b"floating_nodes\0".as_ptr() as *const ::core::ffi::c_char,
        floating_array,
    );
}
unsafe extern "C" fn get_deco_rect(
    mut c: *mut sway_container,
    mut deco_rect: *mut wlr_box,
) {
    let mut parent_layout: sway_container_layout = container_parent_layout(c);
    let mut siblings: *mut list_t = container_get_siblings(c);
    let mut tab_or_stack: bool = (parent_layout.0 == sway_container_layout_0::L_TABBED.0
        || parent_layout.0 == sway_container_layout_0::L_STACKED.0)
        && (!siblings.is_null() && (*siblings).length > 1 as ::core::ffi::c_int
            || !(*config).hide_lone_tab);
    if (!tab_or_stack || container_is_floating(c) as ::core::ffi::c_int != 0)
        && (*c).current.border.0 != sway_container_border::B_NORMAL.0
        || (*c).pending.fullscreen_mode.0 != sway_fullscreen_mode::FULLSCREEN_NONE.0
        || (*c).pending.workspace.is_null()
    {
        (*deco_rect).height = 0 as ::core::ffi::c_int;
        (*deco_rect).width = (*deco_rect).height;
        (*deco_rect).y = (*deco_rect).width;
        (*deco_rect).x = (*deco_rect).y;
        return;
    }
    if !(*c).pending.parent.is_null() {
        (*deco_rect).x = ((*c).pending.x - (*(*c).pending.parent).pending.x)
            as ::core::ffi::c_int;
        (*deco_rect).y = ((*c).pending.y - (*(*c).pending.parent).pending.y)
            as ::core::ffi::c_int;
    } else {
        (*deco_rect).x = ((*c).pending.x - (*(*c).pending.workspace).x)
            as ::core::ffi::c_int;
        (*deco_rect).y = ((*c).pending.y - (*(*c).pending.workspace).y)
            as ::core::ffi::c_int;
    }
    (*deco_rect).width = (*c).pending.width as ::core::ffi::c_int;
    (*deco_rect).height = container_titlebar_height() as ::core::ffi::c_int;
    if !container_is_floating(c) {
        if parent_layout.0 == sway_container_layout_0::L_TABBED.0 {
            (*deco_rect).width = (if !(*c).pending.parent.is_null() {
                (*(*c).pending.parent).pending.width
                    / (*(*(*c).pending.parent).pending.children).length
                        as ::core::ffi::c_double
            } else {
                ((*(*c).pending.workspace).width
                    / (*(*(*c).pending.workspace).tiling).length)
                    as ::core::ffi::c_double
            }) as ::core::ffi::c_int;
            (*deco_rect).x += (*deco_rect).width * container_sibling_index(c);
        } else if parent_layout.0 == sway_container_layout_0::L_STACKED.0 {
            if (*c).view.is_null() {
                let mut siblings_0: size_t = (*container_get_siblings(c)).length
                    as size_t;
                (*deco_rect).y = ((*deco_rect).y as size_t)
                    .wrapping_sub(
                        ((*deco_rect).height as size_t).wrapping_mul(siblings_0),
                    ) as ::core::ffi::c_int;
            }
            (*deco_rect).y += (*deco_rect).height * container_sibling_index(c);
        }
    }
}
unsafe extern "C" fn ipc_json_describe_view(
    mut c: *mut sway_container,
    mut object: *mut json_object,
) {
    json_object_object_add(
        object,
        b"pid\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*(*c).view).pid as int32_t),
    );
    let mut app_id: *const ::core::ffi::c_char = view_get_app_id((*c).view);
    json_object_object_add(
        object,
        b"app_id\0".as_ptr() as *const ::core::ffi::c_char,
        if !app_id.is_null() {
            json_object_new_string(app_id)
        } else {
            ::core::ptr::null_mut::<json_object>()
        },
    );
    json_object_object_add(
        object,
        b"foreign_toplevel_identifier\0".as_ptr() as *const ::core::ffi::c_char,
        if !(*(*c).view).ext_foreign_toplevel.is_null() {
            json_object_new_string((*(*(*c).view).ext_foreign_toplevel).identifier)
        } else {
            ::core::ptr::null_mut::<json_object>()
        },
    );
    let mut visible: bool = view_is_visible((*c).view);
    json_object_object_add(
        object,
        b"visible\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(visible as json_bool),
    );
    let mut has_titlebar: bool = (*(*c).title_bar.tree).node.enabled;
    let mut window_box: wlr_box = wlr_box {
        x: ((*c).pending.content_x - (*c).pending.x) as ::core::ffi::c_int,
        y: (if has_titlebar as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            (*c).pending.content_y - (*c).pending.y
        }) as ::core::ffi::c_int,
        width: (*c).pending.content_width as ::core::ffi::c_int,
        height: (*c).pending.content_height as ::core::ffi::c_int,
    };
    json_object_object_add(
        object,
        b"window_rect\0".as_ptr() as *const ::core::ffi::c_char,
        ipc_json_create_rect(&raw mut window_box),
    );
    let mut geometry: wlr_box = wlr_box {
        x: 0 as ::core::ffi::c_int,
        y: 0 as ::core::ffi::c_int,
        width: (*(*c).view).natural_width,
        height: (*(*c).view).natural_height,
    };
    json_object_object_add(
        object,
        b"geometry\0".as_ptr() as *const ::core::ffi::c_char,
        ipc_json_create_rect(&raw mut geometry),
    );
    json_object_object_add(
        object,
        b"max_render_time\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*(*c).view).max_render_time as int32_t),
    );
    json_object_object_add(
        object,
        b"allow_tearing\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(view_can_tear((*c).view) as json_bool),
    );
    json_object_object_add(
        object,
        b"shell\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(view_get_shell((*c).view)),
    );
    json_object_object_add(
        object,
        b"inhibit_idle\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(view_inhibit_idle((*c).view) as json_bool),
    );
    let mut sandbox_engine: *const ::core::ffi::c_char = view_get_sandbox_engine(
        (*c).view,
    );
    json_object_object_add(
        object,
        b"sandbox_engine\0".as_ptr() as *const ::core::ffi::c_char,
        if !sandbox_engine.is_null() {
            json_object_new_string(sandbox_engine)
        } else {
            ::core::ptr::null_mut::<json_object>()
        },
    );
    let mut sandbox_app_id: *const ::core::ffi::c_char = view_get_sandbox_app_id(
        (*c).view,
    );
    json_object_object_add(
        object,
        b"sandbox_app_id\0".as_ptr() as *const ::core::ffi::c_char,
        if !sandbox_app_id.is_null() {
            json_object_new_string(sandbox_app_id)
        } else {
            ::core::ptr::null_mut::<json_object>()
        },
    );
    let mut sandbox_instance_id: *const ::core::ffi::c_char = view_get_sandbox_instance_id(
        (*c).view,
    );
    json_object_object_add(
        object,
        b"sandbox_instance_id\0".as_ptr() as *const ::core::ffi::c_char,
        if !sandbox_instance_id.is_null() {
            json_object_new_string(sandbox_instance_id)
        } else {
            ::core::ptr::null_mut::<json_object>()
        },
    );
    let mut tag: *const ::core::ffi::c_char = view_get_tag((*c).view);
    json_object_object_add(
        object,
        b"tag\0".as_ptr() as *const ::core::ffi::c_char,
        if !tag.is_null() {
            json_object_new_string(tag)
        } else {
            ::core::ptr::null_mut::<json_object>()
        },
    );
    let mut idle_inhibitors: *mut json_object = json_object_new_object();
    let mut user_inhibitor: *mut sway_idle_inhibitor_v1 = sway_idle_inhibit_v1_user_inhibitor_for_view(
        (*c).view,
    );
    if !user_inhibitor.is_null() {
        json_object_object_add(
            idle_inhibitors,
            b"user\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(
                ipc_json_user_idle_inhibitor_description((*user_inhibitor).mode),
            ),
        );
    } else {
        json_object_object_add(
            idle_inhibitors,
            b"user\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(b"none\0".as_ptr() as *const ::core::ffi::c_char),
        );
    }
    let mut application_inhibitor: *mut sway_idle_inhibitor_v1 = sway_idle_inhibit_v1_application_inhibitor_for_view(
        (*c).view,
    );
    if !application_inhibitor.is_null() {
        json_object_object_add(
            idle_inhibitors,
            b"application\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(b"enabled\0".as_ptr() as *const ::core::ffi::c_char),
        );
    } else {
        json_object_object_add(
            idle_inhibitors,
            b"application\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(b"none\0".as_ptr() as *const ::core::ffi::c_char),
        );
    }
    json_object_object_add(
        object,
        b"idle_inhibitors\0".as_ptr() as *const ::core::ffi::c_char,
        idle_inhibitors,
    );
    let mut content_type: wp_content_type_v1_type = wp_content_type_v1_type::WP_CONTENT_TYPE_V1_TYPE_NONE;
    if !(*(*c).view).surface.is_null() {
        content_type = wlr_surface_get_content_type_v1(
            server.content_type_manager_v1,
            (*(*c).view).surface,
        );
    }
    if content_type.0 != wp_content_type_v1_type::WP_CONTENT_TYPE_V1_TYPE_NONE.0 {
        json_object_object_add(
            object,
            b"content_type\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(ipc_json_content_type_description(content_type)),
        );
    }
    if (*(*c).view).r#type.0 == sway_view_type::SWAY_VIEW_XWAYLAND.0 {
        json_object_object_add(
            object,
            b"window\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int(view_get_x11_window_id((*c).view) as int32_t),
        );
        let mut window_props: *mut json_object = json_object_new_object();
        let mut class: *const ::core::ffi::c_char = view_get_class((*c).view);
        if !class.is_null() {
            json_object_object_add(
                window_props,
                b"class\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_new_string(class),
            );
        }
        let mut instance: *const ::core::ffi::c_char = view_get_instance((*c).view);
        if !instance.is_null() {
            json_object_object_add(
                window_props,
                b"instance\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_new_string(instance),
            );
        }
        if !(*c).title.is_null() {
            json_object_object_add(
                window_props,
                b"title\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_new_string((*c).title),
            );
        }
        let mut parent_id: uint32_t = view_get_x11_parent_id((*c).view);
        json_object_object_add(
            window_props,
            b"transient_for\0".as_ptr() as *const ::core::ffi::c_char,
            if parent_id != 0 {
                json_object_new_int(parent_id as int32_t)
            } else {
                ::core::ptr::null_mut::<json_object>()
            },
        );
        let mut role: *const ::core::ffi::c_char = view_get_window_role((*c).view);
        if !role.is_null() {
            json_object_object_add(
                window_props,
                b"window_role\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_new_string(role),
            );
        }
        let mut window_type: uint32_t = view_get_window_type((*c).view);
        if window_type != 0 {
            json_object_object_add(
                window_props,
                b"window_type\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_new_string(ipc_json_xwindow_type_description((*c).view)),
            );
        }
        json_object_object_add(
            object,
            b"window_properties\0".as_ptr() as *const ::core::ffi::c_char,
            window_props,
        );
    }
}
unsafe extern "C" fn ipc_json_describe_container(
    mut c: *mut sway_container,
    mut object: *mut json_object,
) {
    json_object_object_add(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        if !(*c).title.is_null() {
            json_object_new_string((*c).title)
        } else {
            ::core::ptr::null_mut::<json_object>()
        },
    );
    let mut floating: bool = container_is_floating(c);
    if floating {
        json_object_object_add(
            object,
            b"type\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(
                b"floating_con\0".as_ptr() as *const ::core::ffi::c_char,
            ),
        );
    }
    json_object_object_add(
        object,
        b"layout\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(ipc_json_layout_description((*c).pending.layout)),
    );
    json_object_object_add(
        object,
        b"orientation\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(ipc_json_orientation_description((*c).pending.layout)),
    );
    let mut urgent: bool = if !(*c).view.is_null() {
        view_is_urgent((*c).view) as ::core::ffi::c_int
    } else {
        container_has_urgent_child(c) as ::core::ffi::c_int
    } != 0;
    json_object_object_add(
        object,
        b"urgent\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(urgent as json_bool),
    );
    json_object_object_add(
        object,
        b"sticky\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean((*c).is_sticky as json_bool),
    );
    json_object_object_add(
        object,
        b"floating\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(
            if floating as ::core::ffi::c_int != 0 {
                b"user_on\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"auto_off\0".as_ptr() as *const ::core::ffi::c_char
            },
        ),
    );
    json_object_object_add(
        object,
        b"fullscreen_mode\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*c).pending.fullscreen_mode.0 as int32_t),
    );
    json_object_object_add(
        object,
        b"scratchpad_state\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(
            if !(*c).scratchpad {
                b"none\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"fresh\0".as_ptr() as *const ::core::ffi::c_char
            },
        ),
    );
    let mut parent: *mut sway_node = node_get_parent(&raw mut (*c).node);
    let mut parent_box: wlr_box = wlr_box {
        x: 0 as ::core::ffi::c_int,
        y: 0 as ::core::ffi::c_int,
        width: 0 as ::core::ffi::c_int,
        height: 0 as ::core::ffi::c_int,
    };
    if !parent.is_null() {
        node_get_box(parent, &raw mut parent_box);
    }
    if parent_box.width != 0 as ::core::ffi::c_int
        && parent_box.height != 0 as ::core::ffi::c_int
    {
        let mut percent: ::core::ffi::c_double = (*c).pending.width
            / parent_box.width as ::core::ffi::c_double
            * ((*c).pending.height / parent_box.height as ::core::ffi::c_double);
        json_object_object_add(
            object,
            b"percent\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_double(percent),
        );
    }
    json_object_object_add(
        object,
        b"border\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(ipc_json_border_description((*c).current.border)),
    );
    json_object_object_add(
        object,
        b"current_border_width\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*c).current.border_thickness as int32_t),
    );
    json_object_object_add(
        object,
        b"floating_nodes\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_array(),
    );
    let mut deco_box: wlr_box = wlr_box {
        x: 0 as ::core::ffi::c_int,
        y: 0 as ::core::ffi::c_int,
        width: 0 as ::core::ffi::c_int,
        height: 0 as ::core::ffi::c_int,
    };
    get_deco_rect(c, &raw mut deco_box);
    json_object_object_add(
        object,
        b"deco_rect\0".as_ptr() as *const ::core::ffi::c_char,
        ipc_json_create_rect(&raw mut deco_box),
    );
    let mut marks: *mut json_object = json_object_new_array();
    let mut con_marks: *mut list_t = (*c).marks;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*con_marks).length {
        json_object_array_add(
            marks,
            json_object_new_string(
                *(*con_marks).items.offset(i as isize) as *const ::core::ffi::c_char,
            ),
        );
        i += 1;
    }
    json_object_object_add(
        object,
        b"marks\0".as_ptr() as *const ::core::ffi::c_char,
        marks,
    );
    if !(*c).view.is_null() {
        ipc_json_describe_view(c, object);
    }
}
unsafe extern "C" fn focus_inactive_children_iterator(
    mut node: *mut sway_node,
    mut _data: *mut ::core::ffi::c_void,
) {
    let mut data: *mut focus_inactive_data = _data as *mut focus_inactive_data;
    let mut focus: *mut json_object = (*data).object;
    if (*data).node == &raw mut (*root).node {
        let mut output: *mut sway_output = node_get_output(node);
        if output.is_null() {
            return;
        }
        let mut id: size_t = (*output).node.id;
        let mut len: ::core::ffi::c_int = json_object_array_length(focus)
            as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < len {
            if json_object_get_int(json_object_array_get_idx(focus, i as size_t))
                as size_t == id
            {
                return;
            }
            i += 1;
        }
        node = &raw mut (*output).node;
    } else if node_get_parent(node) != (*data).node {
        return
    }
    json_object_array_add(focus, json_object_new_int((*node).id as int32_t));
}
#[no_mangle]
pub unsafe extern "C" fn ipc_json_describe_node(
    mut node: *mut sway_node,
) -> *mut json_object {
    let mut seat: *mut sway_seat = input_manager_get_default_seat();
    let mut focused: bool = seat_get_focus(seat) == node;
    let mut name: *mut ::core::ffi::c_char = node_get_name(node);
    let mut r#box: wlr_box = wlr_box {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    node_get_box(node, &raw mut r#box);
    if (*node).r#type.0 == sway_node_type::N_CONTAINER.0 {
        let mut deco_rect: wlr_box = wlr_box {
            x: 0 as ::core::ffi::c_int,
            y: 0 as ::core::ffi::c_int,
            width: 0 as ::core::ffi::c_int,
            height: 0 as ::core::ffi::c_int,
        };
        get_deco_rect((*node).c2rust_unnamed.sway_container, &raw mut deco_rect);
        let mut count: size_t = 1 as size_t;
        if container_parent_layout((*node).c2rust_unnamed.sway_container).0
            == sway_container_layout_0::L_STACKED.0
        {
            count = (*container_get_siblings((*node).c2rust_unnamed.sway_container))
                .length as size_t;
        }
        r#box.y = (r#box.y as size_t)
            .wrapping_add((deco_rect.height as size_t).wrapping_mul(count))
            as ::core::ffi::c_int;
        r#box.height = (r#box.height as size_t)
            .wrapping_sub((deco_rect.height as size_t).wrapping_mul(count))
            as ::core::ffi::c_int;
    }
    let mut focus: *mut json_object = json_object_new_array();
    let mut data: focus_inactive_data = focus_inactive_data {
        node: node,
        object: focus,
    };
    seat_for_each_node(
        seat,
        Some(
            focus_inactive_children_iterator
                as unsafe extern "C" fn(*mut sway_node, *mut ::core::ffi::c_void) -> (),
        ),
        &raw mut data as *mut ::core::ffi::c_void,
    );
    let mut object: *mut json_object = ipc_json_create_node(
        (*node).id as ::core::ffi::c_int,
        ipc_json_node_type_description((*node).r#type),
        name,
        focused,
        focus,
        &raw mut r#box,
    );
    match (*node).r#type {
        sway_node_type::N_OUTPUT => {
            ipc_json_describe_enabled_output((*node).c2rust_unnamed.sway_output, object);
        }
        sway_node_type::N_CONTAINER => {
            ipc_json_describe_container((*node).c2rust_unnamed.sway_container, object);
        }
        sway_node_type::N_WORKSPACE => {
            ipc_json_describe_workspace((*node).c2rust_unnamed.sway_workspace, object);
        }
        sway_node_type::N_ROOT | _ => {}
    }
    return object;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_json_describe_node_recursive(
    mut node: *mut sway_node,
) -> *mut json_object {
    let mut object: *mut json_object = ipc_json_describe_node(node);
    let mut i: ::core::ffi::c_int = 0;
    let mut children: *mut json_object = json_object_new_array();
    match (*node).r#type {
        sway_node_type::N_ROOT => {
            json_object_array_add(children, ipc_json_describe_scratchpad_output());
            i = 0 as ::core::ffi::c_int;
            while i < (*(*root).outputs).length {
                let mut output: *mut sway_output = *(*(*root).outputs)
                    .items
                    .offset(i as isize) as *mut sway_output;
                json_object_array_add(
                    children,
                    ipc_json_describe_node_recursive(&raw mut (*output).node),
                );
                i += 1;
            }
        }
        sway_node_type::N_OUTPUT => {
            i = 0 as ::core::ffi::c_int;
            while i < (*(*(*node).c2rust_unnamed.sway_output).workspaces).length {
                let mut ws: *mut sway_workspace = *(*(*(*node)
                    .c2rust_unnamed
                    .sway_output)
                    .workspaces)
                    .items
                    .offset(i as isize) as *mut sway_workspace;
                json_object_array_add(
                    children,
                    ipc_json_describe_node_recursive(&raw mut (*ws).node),
                );
                i += 1;
            }
        }
        sway_node_type::N_WORKSPACE => {
            i = 0 as ::core::ffi::c_int;
            while i < (*(*(*node).c2rust_unnamed.sway_workspace).tiling).length {
                let mut con: *mut sway_container = *(*(*(*node)
                    .c2rust_unnamed
                    .sway_workspace)
                    .tiling)
                    .items
                    .offset(i as isize) as *mut sway_container;
                json_object_array_add(
                    children,
                    ipc_json_describe_node_recursive(&raw mut (*con).node),
                );
                i += 1;
            }
        }
        sway_node_type::N_CONTAINER => {
            if !(*(*node).c2rust_unnamed.sway_container).pending.children.is_null() {
                i = 0 as ::core::ffi::c_int;
                while i
                    < (*(*(*node).c2rust_unnamed.sway_container).pending.children).length
                {
                    let mut child: *mut sway_container = *(*(*(*node)
                        .c2rust_unnamed
                        .sway_container)
                        .pending
                        .children)
                        .items
                        .offset(i as isize) as *mut sway_container;
                    json_object_array_add(
                        children,
                        ipc_json_describe_node_recursive(&raw mut (*child).node),
                    );
                    i += 1;
                }
            }
        }
        _ => {}
    }
    json_object_object_add(
        object,
        b"nodes\0".as_ptr() as *const ::core::ffi::c_char,
        children,
    );
    return object;
}
unsafe extern "C" fn describe_libinput_device(
    mut device: *mut libinput_device,
) -> *mut json_object {
    let mut object: *mut json_object = json_object_new_object();
    let mut events: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
        as *const ::core::ffi::c_char;
    match libinput_device_config_send_events_get_mode(device) {
        0 => {
            events = b"enabled\0".as_ptr() as *const ::core::ffi::c_char;
        }
        2 => {
            events = b"disabled_on_external_mouse\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        1 => {
            events = b"disabled\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    json_object_object_add(
        object,
        b"send_events\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(events),
    );
    if libinput_device_config_tap_get_finger_count(device) > 0 as ::core::ffi::c_int {
        let mut tap: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
            as *const ::core::ffi::c_char;
        match libinput_device_config_tap_get_enabled(device) {
            libinput_config_tap_state::LIBINPUT_CONFIG_TAP_ENABLED => {
                tap = b"enabled\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_tap_state::LIBINPUT_CONFIG_TAP_DISABLED => {
                tap = b"disabled\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        json_object_object_add(
            object,
            b"tap\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(tap),
        );
        let mut button_map: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
            as *const ::core::ffi::c_char;
        match libinput_device_config_tap_get_button_map(device) {
            libinput_config_tap_button_map::LIBINPUT_CONFIG_TAP_MAP_LRM => {
                button_map = b"lrm\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_tap_button_map::LIBINPUT_CONFIG_TAP_MAP_LMR => {
                button_map = b"lmr\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        json_object_object_add(
            object,
            b"tap_button_map\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(button_map),
        );
        let mut drag_3fg: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
            as *const ::core::ffi::c_char;
        match libinput_device_config_3fg_drag_get_enabled(device) {
            libinput_config_3fg_drag_state::LIBINPUT_CONFIG_3FG_DRAG_ENABLED_3FG => {
                drag_3fg = b"three_finger\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_3fg_drag_state::LIBINPUT_CONFIG_3FG_DRAG_ENABLED_4FG => {
                drag_3fg = b"four_finger\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_3fg_drag_state::LIBINPUT_CONFIG_3FG_DRAG_DISABLED => {
                drag_3fg = b"disabled\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        json_object_object_add(
            object,
            b"three_finger_drag\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(drag_3fg),
        );
        let mut drag: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
            as *const ::core::ffi::c_char;
        match libinput_device_config_tap_get_drag_enabled(device) {
            libinput_config_drag_state::LIBINPUT_CONFIG_DRAG_ENABLED => {
                drag = b"enabled\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_drag_state::LIBINPUT_CONFIG_DRAG_DISABLED => {
                drag = b"disabled\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        json_object_object_add(
            object,
            b"tap_drag\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(drag),
        );
        let mut drag_lock: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
            as *const ::core::ffi::c_char;
        match libinput_device_config_tap_get_drag_lock_enabled(device) {
            libinput_config_drag_lock_state::LIBINPUT_CONFIG_DRAG_LOCK_ENABLED => {
                drag_lock = b"enabled\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_drag_lock_state::LIBINPUT_CONFIG_DRAG_LOCK_DISABLED => {
                drag_lock = b"disabled\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_drag_lock_state::LIBINPUT_CONFIG_DRAG_LOCK_ENABLED_STICKY => {
                drag_lock = b"enabled_sticky\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        json_object_object_add(
            object,
            b"tap_drag_lock\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(drag_lock),
        );
    }
    if libinput_device_config_accel_is_available(device) != 0 {
        let mut accel: ::core::ffi::c_double = libinput_device_config_accel_get_speed(
            device,
        );
        json_object_object_add(
            object,
            b"accel_speed\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_double(accel),
        );
        let mut accel_profile: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
            as *const ::core::ffi::c_char;
        match libinput_device_config_accel_get_profile(device) {
            libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_NONE => {
                accel_profile = b"none\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_FLAT => {
                accel_profile = b"flat\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_ADAPTIVE => {
                accel_profile = b"adaptive\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_CUSTOM => {
                accel_profile = b"custom\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        json_object_object_add(
            object,
            b"accel_profile\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(accel_profile),
        );
    }
    if libinput_device_config_scroll_has_natural_scroll(device) != 0 {
        let mut natural_scroll: *const ::core::ffi::c_char = b"disabled\0".as_ptr()
            as *const ::core::ffi::c_char;
        if libinput_device_config_scroll_get_natural_scroll_enabled(device) != 0 {
            natural_scroll = b"enabled\0".as_ptr() as *const ::core::ffi::c_char;
        }
        json_object_object_add(
            object,
            b"natural_scroll\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(natural_scroll),
        );
    }
    if libinput_device_config_left_handed_is_available(device) != 0 {
        let mut left_handed: *const ::core::ffi::c_char = b"disabled\0".as_ptr()
            as *const ::core::ffi::c_char;
        if libinput_device_config_left_handed_get(device) != 0 as ::core::ffi::c_int {
            left_handed = b"enabled\0".as_ptr() as *const ::core::ffi::c_char;
        }
        json_object_object_add(
            object,
            b"left_handed\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(left_handed),
        );
    }
    let mut click_methods: uint32_t = libinput_device_config_click_get_methods(device);
    if click_methods
        & !(libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_NONE.0
            as ::core::ffi::c_int) as uint32_t != 0 as uint32_t
    {
        let mut click_method: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
            as *const ::core::ffi::c_char;
        match libinput_device_config_click_get_method(device) {
            libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_NONE => {
                click_method = b"none\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_BUTTON_AREAS => {
                click_method = b"button_areas\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_CLICKFINGER => {
                click_method = b"clickfinger\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        json_object_object_add(
            object,
            b"click_method\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(click_method),
        );
        let mut button_map_0: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
            as *const ::core::ffi::c_char;
        match libinput_device_config_click_get_clickfinger_button_map(device) {
            libinput_config_clickfinger_button_map::LIBINPUT_CONFIG_CLICKFINGER_MAP_LRM => {
                button_map_0 = b"lrm\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_clickfinger_button_map::LIBINPUT_CONFIG_CLICKFINGER_MAP_LMR => {
                button_map_0 = b"lmr\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        json_object_object_add(
            object,
            b"clickfinger_button_map\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(button_map_0),
        );
    }
    if libinput_device_config_middle_emulation_is_available(device) != 0 {
        let mut middle_emulation: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
            as *const ::core::ffi::c_char;
        match libinput_device_config_middle_emulation_get_enabled(device) {
            libinput_config_middle_emulation_state::LIBINPUT_CONFIG_MIDDLE_EMULATION_ENABLED => {
                middle_emulation = b"enabled\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_middle_emulation_state::LIBINPUT_CONFIG_MIDDLE_EMULATION_DISABLED => {
                middle_emulation = b"disabled\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        json_object_object_add(
            object,
            b"middle_emulation\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(middle_emulation),
        );
    }
    let mut scroll_methods: uint32_t = libinput_device_config_scroll_get_methods(device);
    if scroll_methods
        & !(libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_NO_SCROLL.0
            as ::core::ffi::c_int) as uint32_t != 0 as uint32_t
    {
        let mut scroll_method: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
            as *const ::core::ffi::c_char;
        match libinput_device_config_scroll_get_method(device) {
            libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_NO_SCROLL => {
                scroll_method = b"none\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_2FG => {
                scroll_method = b"two_finger\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_EDGE => {
                scroll_method = b"edge\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_ON_BUTTON_DOWN => {
                scroll_method = b"on_button_down\0".as_ptr()
                    as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        json_object_object_add(
            object,
            b"scroll_method\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(scroll_method),
        );
        if scroll_methods
            & libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_ON_BUTTON_DOWN.0
                as ::core::ffi::c_int as uint32_t != 0 as uint32_t
        {
            let mut button: uint32_t = libinput_device_config_scroll_get_button(device);
            json_object_object_add(
                object,
                b"scroll_button\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_new_int(button as int32_t),
            );
            let mut lock: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
                as *const ::core::ffi::c_char;
            match libinput_device_config_scroll_get_button_lock(device) {
                libinput_config_scroll_button_lock_state::LIBINPUT_CONFIG_SCROLL_BUTTON_LOCK_ENABLED => {
                    lock = b"enabled\0".as_ptr() as *const ::core::ffi::c_char;
                }
                libinput_config_scroll_button_lock_state::LIBINPUT_CONFIG_SCROLL_BUTTON_LOCK_DISABLED => {
                    lock = b"disabled\0".as_ptr() as *const ::core::ffi::c_char;
                }
                _ => {}
            }
            json_object_object_add(
                object,
                b"scroll_button_lock\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_new_string(lock),
            );
        }
    }
    if libinput_device_config_dwt_is_available(device) != 0 {
        let mut dwt: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
            as *const ::core::ffi::c_char;
        match libinput_device_config_dwt_get_enabled(device) {
            libinput_config_dwt_state::LIBINPUT_CONFIG_DWT_ENABLED => {
                dwt = b"enabled\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_dwt_state::LIBINPUT_CONFIG_DWT_DISABLED => {
                dwt = b"disabled\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        json_object_object_add(
            object,
            b"dwt\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(dwt),
        );
    }
    if libinput_device_config_dwtp_is_available(device) != 0 {
        let mut dwtp: *const ::core::ffi::c_char = b"unknown\0".as_ptr()
            as *const ::core::ffi::c_char;
        match libinput_device_config_dwtp_get_enabled(device) {
            libinput_config_dwtp_state::LIBINPUT_CONFIG_DWTP_ENABLED => {
                dwtp = b"enabled\0".as_ptr() as *const ::core::ffi::c_char;
            }
            libinput_config_dwtp_state::LIBINPUT_CONFIG_DWTP_DISABLED => {
                dwtp = b"disabled\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        json_object_object_add(
            object,
            b"dwtp\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string(dwtp),
        );
    }
    if libinput_device_config_calibration_has_matrix(device) != 0 {
        let mut matrix: [::core::ffi::c_float; 6] = [0.; 6];
        libinput_device_config_calibration_get_matrix(
            device,
            &raw mut matrix as *mut ::core::ffi::c_float,
        );
        let mut array: *mut json_object = json_object_new_array();
        let mut x: *mut json_object = ::core::ptr::null_mut::<json_object>();
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 6 as ::core::ffi::c_int {
            x = json_object_new_double(matrix[i as usize] as ::core::ffi::c_double);
            json_object_array_add(array, x);
            i += 1;
        }
        json_object_object_add(
            object,
            b"calibration_matrix\0".as_ptr() as *const ::core::ffi::c_char,
            array,
        );
    }
    return object;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_json_describe_input(
    mut device: *mut sway_input_device,
) -> *mut json_object {
    if !_sway_assert(
        !device.is_null(),
        b"[%s:%d] %s:Device must not be null\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-json.c\0".as_ptr() as *const ::core::ffi::c_char,
        1161 as ::core::ffi::c_int,
        b"json_object *ipc_json_describe_input(struct sway_input_device *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return ::core::ptr::null_mut::<json_object>();
    }
    let mut object: *mut json_object = json_object_new_object();
    json_object_object_add(
        object,
        b"identifier\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*device).identifier),
    );
    json_object_object_add(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*(*device).wlr_device).name),
    );
    json_object_object_add(
        object,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(input_device_get_type(device)),
    );
    if (*(*device).wlr_device).r#type.0
        == wlr_input_device_type::WLR_INPUT_DEVICE_KEYBOARD.0
    {
        let mut keyboard: *mut wlr_keyboard = wlr_keyboard_from_input_device(
            (*device).wlr_device,
        );
        let mut keymap: *mut xkb_keymap = (*keyboard).keymap;
        let mut state: *mut xkb_state = (*keyboard).xkb_state;
        json_object_object_add(
            object,
            b"repeat_delay\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int((*keyboard).repeat_info.delay),
        );
        json_object_object_add(
            object,
            b"repeat_rate\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int((*keyboard).repeat_info.rate),
        );
        let mut layouts_arr: *mut json_object = json_object_new_array();
        json_object_object_add(
            object,
            b"xkb_layout_names\0".as_ptr() as *const ::core::ffi::c_char,
            layouts_arr,
        );
        let mut num_layouts: xkb_layout_index_t = if !keymap.is_null() {
            xkb_keymap_num_layouts(keymap)
        } else {
            0 as xkb_layout_index_t
        };
        let mut layout_idx: xkb_layout_index_t = 0;
        layout_idx = 0 as xkb_layout_index_t;
        while layout_idx < num_layouts {
            let mut layout: *const ::core::ffi::c_char = xkb_keymap_layout_get_name(
                keymap,
                layout_idx,
            );
            json_object_array_add(
                layouts_arr,
                if !layout.is_null() {
                    json_object_new_string(layout)
                } else {
                    ::core::ptr::null_mut::<json_object>()
                },
            );
            let mut is_active: bool = xkb_state_layout_index_is_active(
                state,
                layout_idx,
                xkb_state_component::XKB_STATE_LAYOUT_EFFECTIVE,
            ) != 0;
            if is_active {
                json_object_object_add(
                    object,
                    b"xkb_active_layout_index\0".as_ptr() as *const ::core::ffi::c_char,
                    json_object_new_int(layout_idx as int32_t),
                );
                json_object_object_add(
                    object,
                    b"xkb_active_layout_name\0".as_ptr() as *const ::core::ffi::c_char,
                    if !layout.is_null() {
                        json_object_new_string(layout)
                    } else {
                        ::core::ptr::null_mut::<json_object>()
                    },
                );
            }
            layout_idx = layout_idx.wrapping_add(1);
        }
    }
    if (*(*device).wlr_device).r#type.0
        == wlr_input_device_type::WLR_INPUT_DEVICE_POINTER.0
    {
        let mut ic: *mut input_config = input_device_get_config(device);
        let mut scroll_factor: ::core::ffi::c_float = 1.0f32;
        if !ic.is_null() && (*ic).scroll_factor.is_nan() as i32 == 0
            && (*ic).scroll_factor != FLT_MIN
        {
            scroll_factor = (*ic).scroll_factor;
        }
        json_object_object_add(
            object,
            b"scroll_factor\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_double(scroll_factor as ::core::ffi::c_double),
        );
    }
    if wlr_input_device_is_libinput((*device).wlr_device) {
        let mut libinput_dev: *mut libinput_device = ::core::ptr::null_mut::<
            libinput_device,
        >();
        libinput_dev = wlr_libinput_get_device_handle((*device).wlr_device);
        json_object_object_add(
            object,
            b"libinput\0".as_ptr() as *const ::core::ffi::c_char,
            describe_libinput_device(libinput_dev),
        );
        json_object_object_add(
            object,
            b"vendor\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int(libinput_device_get_id_vendor(libinput_dev) as int32_t),
        );
        json_object_object_add(
            object,
            b"product\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int(libinput_device_get_id_product(libinput_dev) as int32_t),
        );
    }
    return object;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_json_describe_seat(
    mut seat: *mut sway_seat,
) -> *mut json_object {
    if !_sway_assert(
        !seat.is_null(),
        b"[%s:%d] %s:Seat must not be null\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-json.c\0".as_ptr() as *const ::core::ffi::c_char,
        1237 as ::core::ffi::c_int,
        b"json_object *ipc_json_describe_seat(struct sway_seat *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return ::core::ptr::null_mut::<json_object>();
    }
    let mut object: *mut json_object = json_object_new_object();
    let mut focus: *mut sway_node = seat_get_focus(seat);
    json_object_object_add(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*(*seat).wlr_seat).name),
    );
    json_object_object_add(
        object,
        b"capabilities\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*(*seat).wlr_seat).capabilities as int32_t),
    );
    json_object_object_add(
        object,
        b"focus\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(
            (if !focus.is_null() { (*focus).id } else { 0 as size_t }) as int32_t,
        ),
    );
    let mut devices: *mut json_object = json_object_new_array();
    let mut device: *mut sway_seat_device = ::core::ptr::null_mut::<sway_seat_device>();
    device = ((*seat).devices.next as *mut ::core::ffi::c_char)
        .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
        as *mut sway_seat_device;
    while &raw mut (*device).link != &raw mut (*seat).devices {
        json_object_array_add(devices, ipc_json_describe_input((*device).input_device));
        device = ((*device).link.next as *mut ::core::ffi::c_char)
            .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
            as *mut sway_seat_device;
    }
    json_object_object_add(
        object,
        b"devices\0".as_ptr() as *const ::core::ffi::c_char,
        devices,
    );
    return object;
}
unsafe extern "C" fn event_to_x11_button(mut event: uint32_t) -> uint32_t {
    match event {
        BTN_LEFT => return 1 as uint32_t,
        BTN_MIDDLE => return 2 as uint32_t,
        BTN_RIGHT => return 3 as uint32_t,
        SWAY_SCROLL_UP => return 4 as uint32_t,
        SWAY_SCROLL_DOWN => return 5 as uint32_t,
        SWAY_SCROLL_LEFT => return 6 as uint32_t,
        SWAY_SCROLL_RIGHT => return 7 as uint32_t,
        BTN_SIDE => return 8 as uint32_t,
        BTN_EXTRA => return 9 as uint32_t,
        _ => return 0 as uint32_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn ipc_json_describe_bar_config(
    mut bar: *mut bar_config,
) -> *mut json_object {
    if !_sway_assert(
        !bar.is_null(),
        b"[%s:%d] %s:Bar must not be NULL\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-json.c\0".as_ptr() as *const ::core::ffi::c_char,
        1287 as ::core::ffi::c_int,
        b"json_object *ipc_json_describe_bar_config(struct bar_config *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return ::core::ptr::null_mut::<json_object>();
    }
    let mut json: *mut json_object = json_object_new_object();
    json_object_object_add(
        json,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).id),
    );
    json_object_object_add(
        json,
        b"mode\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).mode),
    );
    json_object_object_add(
        json,
        b"hidden_state\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).hidden_state),
    );
    json_object_object_add(
        json,
        b"position\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).position),
    );
    json_object_object_add(
        json,
        b"status_command\0".as_ptr() as *const ::core::ffi::c_char,
        if !(*bar).status_command.is_null() {
            json_object_new_string((*bar).status_command)
        } else {
            ::core::ptr::null_mut::<json_object>()
        },
    );
    json_object_object_add(
        json,
        b"font\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(
            if !(*bar).font.is_null() { (*bar).font } else { (*config).font },
        ),
    );
    let mut gaps: *mut json_object = json_object_new_object();
    json_object_object_add(
        gaps,
        b"top\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*bar).gaps.top as int32_t),
    );
    json_object_object_add(
        gaps,
        b"right\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*bar).gaps.right as int32_t),
    );
    json_object_object_add(
        gaps,
        b"bottom\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*bar).gaps.bottom as int32_t),
    );
    json_object_object_add(
        gaps,
        b"left\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*bar).gaps.left as int32_t),
    );
    json_object_object_add(json, b"gaps\0".as_ptr() as *const ::core::ffi::c_char, gaps);
    if !(*bar).separator_symbol.is_null() {
        json_object_object_add(
            json,
            b"separator_symbol\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).separator_symbol),
        );
    }
    json_object_object_add(
        json,
        b"bar_height\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*bar).height as int32_t),
    );
    json_object_object_add(
        json,
        b"status_padding\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*bar).status_padding as int32_t),
    );
    json_object_object_add(
        json,
        b"status_edge_padding\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*bar).status_edge_padding as int32_t),
    );
    json_object_object_add(
        json,
        b"wrap_scroll\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean((*bar).wrap_scroll as json_bool),
    );
    json_object_object_add(
        json,
        b"workspace_buttons\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean((*bar).workspace_buttons as json_bool),
    );
    json_object_object_add(
        json,
        b"strip_workspace_numbers\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean((*bar).strip_workspace_numbers as json_bool),
    );
    json_object_object_add(
        json,
        b"strip_workspace_name\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean((*bar).strip_workspace_name as json_bool),
    );
    json_object_object_add(
        json,
        b"workspace_min_width\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*bar).workspace_min_width as int32_t),
    );
    json_object_object_add(
        json,
        b"binding_mode_indicator\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean((*bar).binding_mode_indicator as json_bool),
    );
    json_object_object_add(
        json,
        b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean((*bar).verbose as json_bool),
    );
    json_object_object_add(
        json,
        b"pango_markup\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(
            (if (*bar).pango_markup.0 == pango_markup_config::PANGO_MARKUP_DEFAULT.0 {
                (*config).pango_markup as ::core::ffi::c_uint
            } else {
                (*bar).pango_markup.0
            }) as json_bool,
        ),
    );
    let mut colors: *mut json_object = json_object_new_object();
    json_object_object_add(
        colors,
        b"background\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.background),
    );
    json_object_object_add(
        colors,
        b"statusline\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.statusline),
    );
    json_object_object_add(
        colors,
        b"separator\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.separator),
    );
    if !(*bar).colors.focused_background.is_null() {
        json_object_object_add(
            colors,
            b"focused_background\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).colors.focused_background),
        );
    } else {
        json_object_object_add(
            colors,
            b"focused_background\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).colors.background),
        );
    }
    if !(*bar).colors.focused_statusline.is_null() {
        json_object_object_add(
            colors,
            b"focused_statusline\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).colors.focused_statusline),
        );
    } else {
        json_object_object_add(
            colors,
            b"focused_statusline\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).colors.statusline),
        );
    }
    if !(*bar).colors.focused_separator.is_null() {
        json_object_object_add(
            colors,
            b"focused_separator\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).colors.focused_separator),
        );
    } else {
        json_object_object_add(
            colors,
            b"focused_separator\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).colors.separator),
        );
    }
    json_object_object_add(
        colors,
        b"focused_workspace_border\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.focused_workspace_border),
    );
    json_object_object_add(
        colors,
        b"focused_workspace_bg\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.focused_workspace_bg),
    );
    json_object_object_add(
        colors,
        b"focused_workspace_text\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.focused_workspace_text),
    );
    json_object_object_add(
        colors,
        b"inactive_workspace_border\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.inactive_workspace_border),
    );
    json_object_object_add(
        colors,
        b"inactive_workspace_bg\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.inactive_workspace_bg),
    );
    json_object_object_add(
        colors,
        b"inactive_workspace_text\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.inactive_workspace_text),
    );
    json_object_object_add(
        colors,
        b"active_workspace_border\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.active_workspace_border),
    );
    json_object_object_add(
        colors,
        b"active_workspace_bg\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.active_workspace_bg),
    );
    json_object_object_add(
        colors,
        b"active_workspace_text\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.active_workspace_text),
    );
    json_object_object_add(
        colors,
        b"urgent_workspace_border\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.urgent_workspace_border),
    );
    json_object_object_add(
        colors,
        b"urgent_workspace_bg\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.urgent_workspace_bg),
    );
    json_object_object_add(
        colors,
        b"urgent_workspace_text\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).colors.urgent_workspace_text),
    );
    if !(*bar).colors.binding_mode_border.is_null() {
        json_object_object_add(
            colors,
            b"binding_mode_border\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).colors.binding_mode_border),
        );
    } else {
        json_object_object_add(
            colors,
            b"binding_mode_border\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).colors.urgent_workspace_border),
        );
    }
    if !(*bar).colors.binding_mode_bg.is_null() {
        json_object_object_add(
            colors,
            b"binding_mode_bg\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).colors.binding_mode_bg),
        );
    } else {
        json_object_object_add(
            colors,
            b"binding_mode_bg\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).colors.urgent_workspace_bg),
        );
    }
    if !(*bar).colors.binding_mode_text.is_null() {
        json_object_object_add(
            colors,
            b"binding_mode_text\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).colors.binding_mode_text),
        );
    } else {
        json_object_object_add(
            colors,
            b"binding_mode_text\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).colors.urgent_workspace_text),
        );
    }
    json_object_object_add(
        json,
        b"colors\0".as_ptr() as *const ::core::ffi::c_char,
        colors,
    );
    if (*(*bar).bindings).length > 0 as ::core::ffi::c_int {
        let mut bindings: *mut json_object = json_object_new_array();
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*bar).bindings).length {
            let mut binding: *mut bar_binding = *(*(*bar).bindings)
                .items
                .offset(i as isize) as *mut bar_binding;
            let mut bind: *mut json_object = json_object_new_object();
            json_object_object_add(
                bind,
                b"input_code\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_new_int(event_to_x11_button((*binding).button) as int32_t),
            );
            json_object_object_add(
                bind,
                b"event_code\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_new_int((*binding).button as int32_t),
            );
            json_object_object_add(
                bind,
                b"command\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_new_string((*binding).command),
            );
            json_object_object_add(
                bind,
                b"release\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_new_boolean((*binding).release as json_bool),
            );
            json_object_array_add(bindings, bind);
            i += 1;
        }
        json_object_object_add(
            json,
            b"bindings\0".as_ptr() as *const ::core::ffi::c_char,
            bindings,
        );
    }
    if !(*bar).outputs.is_null() && (*(*bar).outputs).length > 0 as ::core::ffi::c_int {
        let mut outputs: *mut json_object = json_object_new_array();
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*(*bar).outputs).length {
            let mut name: *const ::core::ffi::c_char = *(*(*bar).outputs)
                .items
                .offset(i_0 as isize) as *const ::core::ffi::c_char;
            json_object_array_add(outputs, json_object_new_string(name));
            i_0 += 1;
        }
        json_object_object_add(
            json,
            b"outputs\0".as_ptr() as *const ::core::ffi::c_char,
            outputs,
        );
    }
    if !(*bar).tray_outputs.is_null()
        && (*(*bar).tray_outputs).length > 0 as ::core::ffi::c_int
    {
        let mut tray_outputs: *mut json_object = json_object_new_array();
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < (*(*bar).tray_outputs).length {
            let mut name_0: *const ::core::ffi::c_char = *(*(*bar).tray_outputs)
                .items
                .offset(i_1 as isize) as *const ::core::ffi::c_char;
            json_object_array_add(tray_outputs, json_object_new_string(name_0));
            i_1 += 1;
        }
        json_object_object_add(
            json,
            b"tray_outputs\0".as_ptr() as *const ::core::ffi::c_char,
            tray_outputs,
        );
    }
    let mut tray_bindings: *mut json_object = json_object_new_array();
    let mut tray_bind: *mut tray_binding = ::core::ptr::null_mut::<tray_binding>();
    tray_bind = ((*bar).tray_bindings.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut tray_binding
        as *mut tray_binding;
    while &raw mut (*tray_bind).link != &raw mut (*bar).tray_bindings {
        let mut bind_0: *mut json_object = json_object_new_object();
        json_object_object_add(
            bind_0,
            b"input_code\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int(event_to_x11_button((*tray_bind).button) as int32_t),
        );
        json_object_object_add(
            bind_0,
            b"event_code\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_int((*tray_bind).button as int32_t),
        );
        json_object_object_add(
            bind_0,
            b"command\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*tray_bind).command),
        );
        json_object_array_add(tray_bindings, bind_0);
        tray_bind = ((*tray_bind).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut tray_binding
            as *mut tray_binding;
    }
    if json_object_array_length(tray_bindings) > 0 as size_t {
        json_object_object_add(
            json,
            b"tray_bindings\0".as_ptr() as *const ::core::ffi::c_char,
            tray_bindings,
        );
    } else {
        json_object_put(tray_bindings);
    }
    if !(*bar).icon_theme.is_null() {
        json_object_object_add(
            json,
            b"icon_theme\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_string((*bar).icon_theme),
        );
    }
    json_object_object_add(
        json,
        b"tray_padding\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int((*bar).tray_padding as int32_t),
    );
    return json;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_json_get_binding_mode() -> *mut json_object {
    let mut current_mode: *mut json_object = json_object_new_object();
    json_object_object_add(
        current_mode,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*(*config).current_mode).name),
    );
    return current_mode;
}
pub const SWAY_SCROLL_UP: uint32_t = 768 as uint32_t;
pub const SWAY_SCROLL_DOWN: uint32_t = 769 as uint32_t;
pub const SWAY_SCROLL_LEFT: uint32_t = 770 as uint32_t;
pub const SWAY_SCROLL_RIGHT: uint32_t = 771 as uint32_t;
pub const FLT_MIN: ::core::ffi::c_float = __FLT_MIN__;
pub const SWAY_VERSION: [::core::ffi::c_char; 9] = unsafe {
    ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"1.13-dev\0")
};
pub const __FLT_MIN__: ::core::ffi::c_float = 1.17549435e-38f32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
