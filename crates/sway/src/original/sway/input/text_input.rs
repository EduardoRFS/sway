extern "C" {
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    pub type wlr_buffer_impl;
    pub type wlr_renderer_impl;
    pub type wlr_texture_impl;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_primary_selection_source;
    pub type wlr_swapchain;
    pub type wlr_allocator;
    pub type wlr_drm_syncobj_timeline;
    pub type wlr_backend_impl;
    pub type wlr_output_layer_state;
    pub type wlr_linux_dmabuf_feedback_v1_compiled;
    pub type wlr_output_layer_feedback_event;
    pub type wlr_gamma_control_v1;
    pub type wlr_color_manager_v1;
    pub type wlr_gamma_control_manager_v1;
    pub type wlr_touch_impl;
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
    pub type sway_transaction;
    pub type wlr_ext_workspace_manager_v1;
    pub type wlr_xdg_activation_v1;
    pub type wlr_ext_foreign_toplevel_image_capture_source_manager_v1;
    pub type wlr_security_context_manager_v1;
    pub type wlr_export_dmabuf_manager_v1;
    pub type wlr_ext_image_copy_capture_manager_v1;
    pub type wlr_screencopy_manager_v1;
    pub type wlr_ext_data_control_manager_v1;
    pub type wlr_data_control_manager_v1;
    pub type wlr_content_type_manager_v1;
    pub type wlr_foreign_toplevel_manager_v1;
    pub type wlr_ext_foreign_toplevel_list_v1;
    pub type wlr_output_power_manager_v1;
    pub type wlr_output_manager_v1;
    pub type wlr_xdg_output_manager_v1;
    pub type wlr_pointer_constraints_v1;
    pub type wlr_drm_lease_v1_manager;
    pub type wlr_xdg_decoration_manager_v1;
    pub type wlr_server_decoration_manager;
    pub type wlr_relative_pointer_manager_v1;
    pub type wlr_xcursor_manager;
    pub type wlr_xwayland_shell_v1;
    pub type wlr_tablet_manager_v2;
    pub type wlr_xdg_shell;
    pub type wlr_idle_notifier_v1;
    pub type wlr_pointer_gestures_v1;
    pub type wlr_data_device_manager;
    pub type wlr_session;
    pub type wlr_tablet_v2_tablet_tool;
    pub type wlr_tablet_v2_tablet;
    pub type wlr_tablet_v2_tablet_pad;
    pub type wlr_tablet_pad;
    pub type sway_switch;
    pub type sway_keyboard;
    pub type sway_cursor;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn wl_list_init(list: *mut wl_list);
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn wl_resource_get_client(resource: *mut wl_resource) -> *mut wl_client;
    fn wlr_seat_set_keyboard(seat: *mut wlr_seat, keyboard: *mut wlr_keyboard);
    fn wlr_seat_get_keyboard(seat: *mut wlr_seat) -> *mut wlr_keyboard;
    fn wlr_seat_keyboard_notify_modifiers(
        seat: *mut wlr_seat,
        modifiers: *const wlr_keyboard_modifiers,
    );
    fn wlr_scene_node_destroy(node: *mut wlr_scene_node);
    fn wlr_scene_node_set_enabled(node: *mut wlr_scene_node, enabled: bool);
    fn wlr_scene_node_set_position(
        node: *mut wlr_scene_node,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn wlr_scene_node_coords(
        node: *mut wlr_scene_node,
        lx: *mut ::core::ffi::c_int,
        ly: *mut ::core::ffi::c_int,
    ) -> bool;
    fn wlr_scene_tree_create(parent: *mut wlr_scene_tree) -> *mut wlr_scene_tree;
    fn wlr_scene_subsurface_tree_create(
        parent: *mut wlr_scene_tree,
        surface: *mut wlr_surface,
    ) -> *mut wlr_scene_tree;
    fn wlr_output_layout_output_at(
        layout: *mut wlr_output_layout,
        lx: ::core::ffi::c_double,
        ly: ::core::ffi::c_double,
    ) -> *mut wlr_output;
    fn wlr_output_layout_get_box(
        layout: *mut wlr_output_layout,
        reference: *mut wlr_output,
        dest_box: *mut wlr_box,
    );
    static mut root: *mut sway_root;
    static mut server: sway_server;
    fn scene_descriptor_assign(
        node: *mut wlr_scene_node,
        r#type: sway_scene_descriptor_type,
        data: *mut ::core::ffi::c_void,
    ) -> bool;
    fn scene_descriptor_destroy(
        node: *mut wlr_scene_node,
        r#type: sway_scene_descriptor_type,
    );
    fn wlr_layer_surface_v1_try_from_wlr_surface(
        surface: *mut wlr_surface,
    ) -> *mut wlr_layer_surface_v1;
    fn view_from_wlr_surface(surface: *mut wlr_surface) -> *mut sway_view;
    fn wlr_input_method_v2_send_activate(input_method: *mut wlr_input_method_v2);
    fn wlr_input_method_v2_send_deactivate(input_method: *mut wlr_input_method_v2);
    fn wlr_input_method_v2_send_surrounding_text(
        input_method: *mut wlr_input_method_v2,
        text: *const ::core::ffi::c_char,
        cursor: uint32_t,
        anchor: uint32_t,
    );
    fn wlr_input_method_v2_send_content_type(
        input_method: *mut wlr_input_method_v2,
        hint: uint32_t,
        purpose: uint32_t,
    );
    fn wlr_input_method_v2_send_text_change_cause(
        input_method: *mut wlr_input_method_v2,
        cause: uint32_t,
    );
    fn wlr_input_method_v2_send_done(input_method: *mut wlr_input_method_v2);
    fn wlr_input_method_v2_send_unavailable(input_method: *mut wlr_input_method_v2);
    fn wlr_input_popup_surface_v2_send_text_input_rectangle(
        popup_surface: *mut wlr_input_popup_surface_v2,
        sbox: *mut wlr_box,
    );
    fn wlr_input_method_keyboard_grab_v2_set_keyboard(
        keyboard_grab: *mut wlr_input_method_keyboard_grab_v2,
        keyboard: *mut wlr_keyboard,
    );
    fn wlr_text_input_v3_send_enter(
        text_input: *mut wlr_text_input_v3,
        wlr_surface: *mut wlr_surface,
    );
    fn wlr_text_input_v3_send_leave(text_input: *mut wlr_text_input_v3);
    fn wlr_text_input_v3_send_preedit_string(
        text_input: *mut wlr_text_input_v3,
        text: *const ::core::ffi::c_char,
        cursor_begin: int32_t,
        cursor_end: int32_t,
    );
    fn wlr_text_input_v3_send_commit_string(
        text_input: *mut wlr_text_input_v3,
        text: *const ::core::ffi::c_char,
    );
    fn wlr_text_input_v3_send_delete_surrounding_text(
        text_input: *mut wlr_text_input_v3,
        before_length: uint32_t,
        after_length: uint32_t,
    );
    fn wlr_text_input_v3_send_done(text_input: *mut wlr_text_input_v3);
    fn wlr_session_lock_surface_v1_try_from_wlr_surface(
        surface: *mut wlr_surface,
    ) -> *mut wlr_session_lock_surface_v1;
}
pub type size_t = usize;
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
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
    pub events: C2Rust_Unnamed,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
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
pub type xkb_layout_index_t = uint32_t;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
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
    pub repeat_info: C2Rust_Unnamed_1,
    pub events: C2Rust_Unnamed_0,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub rate: int32_t,
    pub delay: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_impl {
    pub name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer {
    pub base: wlr_input_device,
    pub r#impl: *const wlr_pointer_impl,
    pub output_name: *mut ::core::ffi::c_char,
    pub buttons: [uint32_t; 16],
    pub button_count: size_t,
    pub events: C2Rust_Unnamed_2,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
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
pub struct wlr_pointer_swipe_begin_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub fingers: uint32_t,
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
pub struct wlr_pointer_swipe_end_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub cancelled: bool,
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
pub struct wlr_pointer_pinch_end_event {
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
pub struct wlr_pointer_hold_end_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub cancelled: bool,
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
    pub events: C2Rust_Unnamed_6,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_4,
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
pub struct C2Rust_Unnamed_4 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub addons: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
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
    pub viewport: C2Rust_Unnamed_7,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
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
    pub events: C2Rust_Unnamed_8,
    pub addons: wlr_addon_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub destroy: wl_signal,
    pub release: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_client_buffer {
    pub base: wlr_buffer,
    pub texture: *mut wlr_texture,
    pub source: *mut wlr_buffer,
    pub WLR_PRIVATE: C2Rust_Unnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
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
    pub events: C2Rust_Unnamed_12,
    pub features: C2Rust_Unnamed_11,
    pub WLR_PRIVATE: C2Rust_Unnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub r#impl: *const wlr_renderer_impl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub input_color_transform: bool,
    pub output_color_transform: bool,
    pub timeline: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
    pub destroy: wl_signal,
    pub lost: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_compositor {
    pub global: *mut wl_global,
    pub renderer: *mut wlr_renderer,
    pub events: C2Rust_Unnamed_14,
    pub WLR_PRIVATE: C2Rust_Unnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_serial_range {
    pub min_incl: uint32_t,
    pub max_incl: uint32_t,
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
pub struct wlr_seat_client {
    pub client: *mut wl_client,
    pub seat: *mut wlr_seat,
    pub link: wl_list,
    pub resources: wl_list,
    pub pointers: wl_list,
    pub keyboards: wl_list,
    pub touches: wl_list,
    pub data_devices: wl_list,
    pub events: C2Rust_Unnamed_16,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub acc_discrete: [int32_t; 2],
    pub last_discrete: [int32_t; 2],
    pub acc_axis: [::core::ffi::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub destroy: wl_signal,
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
    pub events: C2Rust_Unnamed_18,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
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
    pub events: C2Rust_Unnamed_20,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
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
    pub events: C2Rust_Unnamed_22,
    pub WLR_PRIVATE: C2Rust_Unnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
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
    pub events: C2Rust_Unnamed_24,
    pub WLR_PRIVATE: C2Rust_Unnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
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
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_26,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon {
    pub r#impl: *const wlr_addon_interface,
    pub WLR_PRIVATE: C2Rust_Unnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub owner: *const ::core::ffi::c_void,
    pub link: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon_interface {
    pub name: *const ::core::ffi::c_char,
    pub destroy: Option<unsafe extern "C" fn(*mut wlr_addon) -> ()>,
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
    pub events: C2Rust_Unnamed_32,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub display_destroy: wl_listener,
    pub image_description_value: wlr_output_image_description,
    pub color_transform: *mut wlr_color_transform,
    pub default_primaries_value: wlr_color_primaries,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_color_primaries {
    pub red: wlr_color_cie1931_xy,
    pub green: wlr_color_cie1931_xy,
    pub blue: wlr_color_cie1931_xy,
    pub white: wlr_color_cie1931_xy,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_color_cie1931_xy {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_color_transform {
    pub r#type: wlr_color_transform_type,
    pub WLR_PRIVATE: C2Rust_Unnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
    pub ref_count: ::core::ffi::c_int,
    pub addons: wlr_addon_set,
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
pub struct wlr_output_image_description {
    pub primaries: wlr_color_named_primaries,
    pub transfer_function: wlr_color_transfer_function,
    pub mastering_display_primaries: wlr_color_primaries,
    pub mastering_luminance: C2Rust_Unnamed_30,
    pub max_cll: ::core::ffi::c_double,
    pub max_fall: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
    pub min: ::core::ffi::c_double,
    pub max: ::core::ffi::c_double,
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
pub struct wlr_color_named_primaries(pub ::core::ffi::c_uint);
impl wlr_color_named_primaries {
    pub const WLR_COLOR_NAMED_PRIMARIES_SRGB: Self = Self(1);
    pub const WLR_COLOR_NAMED_PRIMARIES_BT2020: Self = Self(2);
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
    pub WLR_PRIVATE: C2Rust_Unnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub renderer_destroy: wl_listener,
    pub color_transform: *mut wlr_color_transform,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
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
pub struct wlr_color_range(pub ::core::ffi::c_uint);
impl wlr_color_range {
    pub const WLR_COLOR_RANGE_NONE: Self = Self(0);
    pub const WLR_COLOR_RANGE_LIMITED: Self = Self(1);
    pub const WLR_COLOR_RANGE_FULL: Self = Self(2);
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
pub struct wlr_output_adaptive_sync_status(pub ::core::ffi::c_uint);
impl wlr_output_adaptive_sync_status {
    pub const WLR_OUTPUT_ADAPTIVE_SYNC_DISABLED: Self = Self(0);
    pub const WLR_OUTPUT_ADAPTIVE_SYNC_ENABLED: Self = Self(1);
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
pub struct wlr_backend {
    pub r#impl: *const wlr_backend_impl,
    pub buffer_caps: uint32_t,
    pub features: C2Rust_Unnamed_34,
    pub events: C2Rust_Unnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
    pub timeline: bool,
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
    pub custom_mode: C2Rust_Unnamed_35,
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
pub struct C2Rust_Unnamed_35 {
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
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_37,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_37 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_38,
    pub keyboard_interactive: zwlr_layer_surface_v1_keyboard_interactivity,
    pub desired_width: uint32_t,
    pub desired_height: uint32_t,
    pub layer: zwlr_layer_shell_v1_layer,
    pub exclusive_edge: uint32_t,
    pub configure_serial: uint32_t,
    pub actual_width: uint32_t,
    pub actual_height: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
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
    pub events: C2Rust_Unnamed_40,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_40 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
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
pub struct wlr_damage_ring {
    pub current: pixman_region32_t,
    pub WLR_PRIVATE: C2Rust_Unnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_41 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_43,
    pub WLR_PRIVATE: C2Rust_Unnamed_42,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
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
pub struct C2Rust_Unnamed_43 {
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
    pub events: C2Rust_Unnamed_45,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_44,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_44 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_45 {
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
    pub events: C2Rust_Unnamed_47,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_46,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
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
    pub events: C2Rust_Unnamed_53,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_49,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
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
    pub events: C2Rust_Unnamed_51,
    pub WLR_PRIVATE: C2Rust_Unnamed_50,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
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
pub struct C2Rust_Unnamed_51 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_52,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_52 {
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
pub struct C2Rust_Unnamed_53 {
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
pub struct wlr_scene_layer_surface_v1 {
    pub tree: *mut wlr_scene_tree,
    pub layer_surface: *mut wlr_layer_surface_v1,
    pub WLR_PRIVATE: C2Rust_Unnamed_54,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_54 {
    pub tree_destroy: wl_listener,
    pub layer_surface_destroy: wl_listener,
    pub layer_surface_map: wl_listener,
    pub layer_surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_touch {
    pub base: wlr_input_device,
    pub r#impl: *const wlr_touch_impl,
    pub output_name: *mut ::core::ffi::c_char,
    pub width_mm: ::core::ffi::c_double,
    pub height_mm: ::core::ffi::c_double,
    pub events: C2Rust_Unnamed_55,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
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
pub struct wlr_touch_cancel_event {
    pub touch: *mut wlr_touch,
    pub time_msec: uint32_t,
    pub touch_id: int32_t,
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
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_root {
    pub node: sway_node,
    pub output_layout: *mut wlr_output_layout,
    pub root_scene: *mut wlr_scene,
    pub staging: *mut wlr_scene_tree,
    pub layer_tree: *mut wlr_scene_tree,
    pub layers: C2Rust_Unnamed_91,
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
    pub events: C2Rust_Unnamed_56,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_56 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_59,
    pub border: C2Rust_Unnamed_58,
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
    pub events: C2Rust_Unnamed_57,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_57 {
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
pub struct C2Rust_Unnamed_58 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_59 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_61,
    pub events: C2Rust_Unnamed_60,
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
pub struct C2Rust_Unnamed_60 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_61 {
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
    pub events: C2Rust_Unnamed_63,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_62,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_65,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_65 {
    pub sway_root: *mut sway_root,
    pub sway_output: *mut sway_output,
    pub sway_workspace: *mut sway_workspace,
    pub sway_container: *mut sway_container,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_output {
    pub node: sway_node,
    pub layers: C2Rust_Unnamed_90,
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
    pub session_lock: C2Rust_Unnamed_72,
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
    pub events: C2Rust_Unnamed_67,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_66,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
    pub new_object: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_text_input_manager_v3 {
    pub global: *mut wl_global,
    pub text_inputs: wl_list,
    pub events: C2Rust_Unnamed_69,
    pub WLR_PRIVATE: C2Rust_Unnamed_68,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
    pub new_text_input: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_manager_v2 {
    pub global: *mut wl_global,
    pub input_methods: wl_list,
    pub events: C2Rust_Unnamed_71,
    pub WLR_PRIVATE: C2Rust_Unnamed_70,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_71 {
    pub new_input_method: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
    pub lock: *mut sway_session_lock,
    pub manager: *mut wlr_session_lock_manager_v1,
    pub new_lock: wl_listener,
    pub manager_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_session_lock_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_74,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_73,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_73 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_74 {
    pub new_lock: wl_signal,
    pub destroy: wl_signal,
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
pub struct wlr_session_lock_v1 {
    pub resource: *mut wl_resource,
    pub surfaces: wl_list,
    pub events: C2Rust_Unnamed_76,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_75,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_75 {
    pub locked_sent: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_76 {
    pub new_surface: wl_signal,
    pub unlock: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_xwayland {
    pub wlr_xwayland: *mut wlr_xwayland,
    pub xcursor_manager: *mut wlr_xcursor_manager,
    pub atoms: [xcb_atom_t; 11],
}
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
    pub events: C2Rust_Unnamed_79,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_77,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_77 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_78,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_78 {
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
pub struct C2Rust_Unnamed_79 {
    pub destroy: wl_signal,
    pub ready: wl_signal,
    pub new_surface: wl_signal,
    pub remove_startup_info: wl_signal,
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
    pub events: C2Rust_Unnamed_81,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_80,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_80 {
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_81 {
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
    pub events: C2Rust_Unnamed_83,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_82,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_82 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_83 {
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
    pub events: C2Rust_Unnamed_85,
    pub WLR_PRIVATE: C2Rust_Unnamed_84,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_84 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_85 {
    pub destroy: wl_signal,
    pub create_seat: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_pointers: wl_list,
    pub events: C2Rust_Unnamed_87,
    pub WLR_PRIVATE: C2Rust_Unnamed_86,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_86 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_87 {
    pub new_virtual_pointer: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_keyboards: wl_list,
    pub events: C2Rust_Unnamed_89,
    pub WLR_PRIVATE: C2Rust_Unnamed_88,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_88 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_89 {
    pub new_virtual_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_90 {
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
pub struct C2Rust_Unnamed_91 {
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
    pub events: C2Rust_Unnamed_93,
    pub WLR_PRIVATE: C2Rust_Unnamed_92,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_92 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_93 {
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
    pub events: C2Rust_Unnamed_95,
    pub WLR_PRIVATE: C2Rust_Unnamed_94,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_94 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_95 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_text_input_v3_features(pub ::core::ffi::c_uint);
impl wlr_text_input_v3_features {
    pub const WLR_TEXT_INPUT_V3_FEATURE_SURROUNDING_TEXT: Self = Self(1);
    pub const WLR_TEXT_INPUT_V3_FEATURE_CONTENT_TYPE: Self = Self(2);
    pub const WLR_TEXT_INPUT_V3_FEATURE_CURSOR_RECTANGLE: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_text_input_v3_state {
    pub surrounding: C2Rust_Unnamed_97,
    pub text_change_cause: uint32_t,
    pub content_type: C2Rust_Unnamed_96,
    pub cursor_rectangle: wlr_box,
    pub features: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_96 {
    pub hint: uint32_t,
    pub purpose: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_97 {
    pub text: *mut ::core::ffi::c_char,
    pub cursor: uint32_t,
    pub anchor: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_text_input_v3 {
    pub seat: *mut wlr_seat,
    pub resource: *mut wl_resource,
    pub focused_surface: *mut wlr_surface,
    pub pending: wlr_text_input_v3_state,
    pub current: wlr_text_input_v3_state,
    pub current_serial: uint32_t,
    pub pending_enabled: bool,
    pub current_enabled: bool,
    pub active_features: uint32_t,
    pub link: wl_list,
    pub events: C2Rust_Unnamed_99,
    pub WLR_PRIVATE: C2Rust_Unnamed_98,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_98 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_99 {
    pub enable: wl_signal,
    pub commit: wl_signal,
    pub disable: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_popup_surface_v2 {
    pub resource: *mut wl_resource,
    pub input_method: *mut wlr_input_method_v2,
    pub link: wl_list,
    pub surface: *mut wlr_surface,
    pub events: C2Rust_Unnamed_100,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_100 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_text_input {
    pub relay: *mut sway_input_method_relay,
    pub input: *mut wlr_text_input_v3,
    pub pending_focused_surface: *mut wlr_surface,
    pub link: wl_list,
    pub pending_focused_surface_destroy: wl_listener,
    pub text_input_enable: wl_listener,
    pub text_input_commit: wl_listener,
    pub text_input_disable: wl_listener,
    pub text_input_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_input_popup {
    pub relay: *mut sway_input_method_relay,
    pub scene_tree: *mut wlr_scene_tree,
    pub desc: sway_popup_desc,
    pub popup_surface: *mut wlr_input_popup_surface_v2,
    pub fixed_output: *mut wlr_output,
    pub link: wl_list,
    pub popup_destroy: wl_listener,
    pub popup_surface_commit: wl_listener,
    pub popup_surface_map: wl_listener,
    pub popup_surface_unmap: wl_listener,
    pub focused_surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_popup_desc {
    pub relative: *mut wlr_scene_node,
    pub view: *mut sway_view,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_scene_descriptor_type(pub ::core::ffi::c_uint);
impl sway_scene_descriptor_type {
    pub const SWAY_SCENE_DESC_BUFFER_TIMER: Self = Self(0);
    pub const SWAY_SCENE_DESC_NON_INTERACTIVE: Self = Self(1);
    pub const SWAY_SCENE_DESC_CONTAINER: Self = Self(2);
    pub const SWAY_SCENE_DESC_VIEW: Self = Self(3);
    pub const SWAY_SCENE_DESC_LAYER_SHELL: Self = Self(4);
    pub const SWAY_SCENE_DESC_XWAYLAND_UNMANAGED: Self = Self(5);
    pub const SWAY_SCENE_DESC_POPUP: Self = Self(6);
    pub const SWAY_SCENE_DESC_DRAG_ICON: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_layer_surface {
    pub map: wl_listener,
    pub unmap: wl_listener,
    pub surface_commit: wl_listener,
    pub node_destroy: wl_listener,
    pub new_popup: wl_listener,
    pub mapped: bool,
    pub popups: *mut wlr_scene_tree,
    pub desc: sway_popup_desc,
    pub output: *mut sway_output,
    pub link: wl_list,
    pub scene: *mut wlr_scene_layer_surface_v1,
    pub tree: *mut wlr_scene_tree,
    pub layer_surface: *mut wlr_layer_surface_v1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_session_lock_surface_v1 {
    pub resource: *mut wl_resource,
    pub link: wl_list,
    pub output: *mut wlr_output,
    pub surface: *mut wlr_surface,
    pub configured: bool,
    pub configure_list: wl_list,
    pub current: wlr_session_lock_surface_v1_state,
    pub pending: wlr_session_lock_surface_v1_state,
    pub events: C2Rust_Unnamed_102,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_101,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_101 {
    pub synced: wlr_surface_synced,
    pub output_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_102 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_session_lock_surface_v1_state {
    pub width: uint32_t,
    pub height: uint32_t,
    pub configure_serial: uint32_t,
}
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 95] = unsafe {
    ::core::mem::transmute::<
        [u8; 95],
        [::core::ffi::c_char; 95],
    >(
        *b"void sway_input_method_relay_set_focus(struct sway_input_method_relay *, struct wlr_surface *)\0",
    )
};
#[inline]
unsafe extern "C" fn wl_signal_add(
    mut signal: *mut wl_signal,
    mut listener: *mut wl_listener,
) {
    wl_list_insert((*signal).listener_list.prev, &raw mut (*listener).link);
}
unsafe extern "C" fn relay_get_focusable_text_input(
    mut relay: *mut sway_input_method_relay,
) -> *mut sway_text_input {
    let mut text_input: *mut sway_text_input = ::core::ptr::null_mut::<
        sway_text_input,
    >();
    text_input = ((*relay).text_inputs.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut sway_text_input
        as *mut sway_text_input;
    while &raw mut (*text_input).link != &raw mut (*relay).text_inputs {
        if !(*text_input).pending_focused_surface.is_null() {
            return text_input;
        }
        text_input = ((*text_input).link.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut sway_text_input
            as *mut sway_text_input;
    }
    return ::core::ptr::null_mut::<sway_text_input>();
}
unsafe extern "C" fn relay_get_focused_text_input(
    mut relay: *mut sway_input_method_relay,
) -> *mut sway_text_input {
    let mut text_input: *mut sway_text_input = ::core::ptr::null_mut::<
        sway_text_input,
    >();
    text_input = ((*relay).text_inputs.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut sway_text_input
        as *mut sway_text_input;
    while &raw mut (*text_input).link != &raw mut (*relay).text_inputs {
        if !(*(*text_input).input).focused_surface.is_null() {
            return text_input;
        }
        text_input = ((*text_input).link.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut sway_text_input
            as *mut sway_text_input;
    }
    return ::core::ptr::null_mut::<sway_text_input>();
}
unsafe extern "C" fn handle_im_commit(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut relay: *mut sway_input_method_relay = (listener as *mut ::core::ffi::c_char)
        .offset(-(144 as ::core::ffi::c_ulong as isize)) as *mut sway_input_method_relay;
    let mut text_input: *mut sway_text_input = relay_get_focused_text_input(relay);
    if text_input.is_null() {
        return;
    }
    if !(*(*relay).input_method).current.preedit.text.is_null() {
        wlr_text_input_v3_send_preedit_string(
            (*text_input).input,
            (*(*relay).input_method).current.preedit.text,
            (*(*relay).input_method).current.preedit.cursor_begin,
            (*(*relay).input_method).current.preedit.cursor_end,
        );
    }
    if !(*(*relay).input_method).current.commit_text.is_null() {
        wlr_text_input_v3_send_commit_string(
            (*text_input).input,
            (*(*relay).input_method).current.commit_text,
        );
    }
    if (*(*relay).input_method).current.delete.before_length != 0
        || (*(*relay).input_method).current.delete.after_length != 0
    {
        wlr_text_input_v3_send_delete_surrounding_text(
            (*text_input).input,
            (*(*relay).input_method).current.delete.before_length,
            (*(*relay).input_method).current.delete.after_length,
        );
    }
    wlr_text_input_v3_send_done((*text_input).input);
}
unsafe extern "C" fn handle_im_keyboard_grab_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut relay: *mut sway_input_method_relay = (listener as *mut ::core::ffi::c_char)
        .offset(-(240 as ::core::ffi::c_ulong as isize)) as *mut sway_input_method_relay;
    let mut keyboard_grab: *mut wlr_input_method_keyboard_grab_v2 = (*(*relay)
        .input_method)
        .keyboard_grab;
    let mut wlr_seat: *mut wlr_seat = (*(*keyboard_grab).input_method).seat;
    wl_list_remove(&raw mut (*relay).input_method_keyboard_grab_destroy.link);
    if !(*keyboard_grab).keyboard.is_null() {
        wlr_seat_set_keyboard(wlr_seat, (*keyboard_grab).keyboard);
        wlr_seat_keyboard_notify_modifiers(
            wlr_seat,
            &raw mut (*(*keyboard_grab).keyboard).modifiers,
        );
    }
}
unsafe extern "C" fn handle_im_grab_keyboard(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut relay: *mut sway_input_method_relay = (listener as *mut ::core::ffi::c_char)
        .offset(-(192 as ::core::ffi::c_ulong as isize)) as *mut sway_input_method_relay;
    let mut keyboard_grab: *mut wlr_input_method_keyboard_grab_v2 = data
        as *mut wlr_input_method_keyboard_grab_v2;
    let mut active_keyboard: *mut wlr_keyboard = wlr_seat_get_keyboard(
        (*(*relay).seat).wlr_seat,
    );
    wlr_input_method_keyboard_grab_v2_set_keyboard(keyboard_grab, active_keyboard);
    wl_signal_add(
        &raw mut (*keyboard_grab).events.destroy,
        &raw mut (*relay).input_method_keyboard_grab_destroy,
    );
    (*relay).input_method_keyboard_grab_destroy.notify = Some(
        handle_im_keyboard_grab_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
}
unsafe extern "C" fn text_input_set_pending_focused_surface(
    mut text_input: *mut sway_text_input,
    mut surface: *mut wlr_surface,
) {
    wl_list_remove(&raw mut (*text_input).pending_focused_surface_destroy.link);
    (*text_input).pending_focused_surface = surface;
    if !surface.is_null() {
        wl_signal_add(
            &raw mut (*surface).events.destroy,
            &raw mut (*text_input).pending_focused_surface_destroy,
        );
    } else {
        wl_list_init(&raw mut (*text_input).pending_focused_surface_destroy.link);
    };
}
unsafe extern "C" fn handle_im_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut relay: *mut sway_input_method_relay = (listener as *mut ::core::ffi::c_char)
        .offset(-(216 as ::core::ffi::c_ulong as isize)) as *mut sway_input_method_relay;
    wl_list_remove(&raw mut (*relay).input_method_commit.link);
    wl_list_remove(&raw mut (*relay).input_method_grab_keyboard.link);
    wl_list_remove(&raw mut (*relay).input_method_destroy.link);
    wl_list_remove(&raw mut (*relay).input_method_new_popup_surface.link);
    (*relay).input_method = ::core::ptr::null_mut::<wlr_input_method_v2>();
    let mut text_input: *mut sway_text_input = relay_get_focused_text_input(relay);
    if !text_input.is_null() {
        text_input_set_pending_focused_surface(
            text_input,
            (*(*text_input).input).focused_surface,
        );
        wlr_text_input_v3_send_leave((*text_input).input);
    }
}
unsafe extern "C" fn constrain_popup(mut popup: *mut sway_input_popup) {
    let mut text_input: *mut sway_text_input = relay_get_focused_text_input(
        (*popup).relay,
    );
    if (*popup).desc.relative.is_null() {
        return;
    }
    let mut parent: wlr_box = wlr_box {
        x: 0 as ::core::ffi::c_int,
        y: 0,
        width: 0,
        height: 0,
    };
    wlr_scene_node_coords(
        &raw mut (*(*(*popup).desc.relative).parent).node,
        &raw mut parent.x,
        &raw mut parent.y,
    );
    let mut geo: wlr_box = wlr_box {
        x: 0 as ::core::ffi::c_int,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut output: *mut wlr_output = ::core::ptr::null_mut::<wlr_output>();
    if !(*popup).desc.view.is_null() {
        let mut view: *mut sway_view = (*popup).desc.view;
        output = wlr_output_layout_output_at(
            (*root).output_layout,
            (*(*view).container).pending.content_x
                + (*view).geometry.x as ::core::ffi::c_double,
            (*(*view).container).pending.content_y
                + (*view).geometry.y as ::core::ffi::c_double,
        );
        parent.width = (*view).geometry.width;
        parent.height = (*view).geometry.height;
        geo = (*view).geometry;
    } else {
        output = (*popup).fixed_output;
    }
    let mut output_box: wlr_box = wlr_box {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    wlr_output_layout_get_box((*root).output_layout, output, &raw mut output_box);
    let mut cursor_rect: bool = (*(*text_input).input).current.features
        & wlr_text_input_v3_features::WLR_TEXT_INPUT_V3_FEATURE_CURSOR_RECTANGLE.0
            as ::core::ffi::c_int as uint32_t != 0;
    let mut cursor_area: wlr_box = wlr_box {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if cursor_rect {
        cursor_area = (*(*text_input).input).current.cursor_rectangle;
    } else {
        cursor_area = wlr_box {
            x: 0,
            y: 0,
            width: parent.width,
            height: parent.height,
        };
    }
    let mut popup_width: ::core::ffi::c_int = (*(*(*popup).popup_surface).surface)
        .current
        .width;
    let mut popup_height: ::core::ffi::c_int = (*(*(*popup).popup_surface).surface)
        .current
        .height;
    let mut x1: ::core::ffi::c_int = parent.x + cursor_area.x;
    let mut x2: ::core::ffi::c_int = parent.x + cursor_area.x + cursor_area.width;
    let mut y1: ::core::ffi::c_int = parent.y + cursor_area.y;
    let mut y2: ::core::ffi::c_int = parent.y + cursor_area.y + cursor_area.height;
    let mut x: ::core::ffi::c_int = x1;
    let mut y: ::core::ffi::c_int = y2;
    let mut available_right: ::core::ffi::c_int = output_box.x + output_box.width - x1;
    let mut available_left: ::core::ffi::c_int = x2 - output_box.x;
    if available_right < popup_width && available_left > available_right {
        x = x2 - popup_width;
    }
    let mut available_down: ::core::ffi::c_int = output_box.y + output_box.height - y2;
    let mut available_up: ::core::ffi::c_int = y1 - output_box.y;
    if available_down < popup_height && available_up > available_down {
        y = y1 - popup_height;
    }
    wlr_scene_node_set_position(
        (*popup).desc.relative,
        x - parent.x - geo.x,
        y - parent.y - geo.y,
    );
    if cursor_rect {
        let mut r#box: wlr_box = wlr_box {
            x: x1 - x,
            y: y1 - y,
            width: cursor_area.width,
            height: cursor_area.height,
        };
        wlr_input_popup_surface_v2_send_text_input_rectangle(
            (*popup).popup_surface,
            &raw mut r#box,
        );
    }
    if !(*popup).scene_tree.is_null() {
        wlr_scene_node_set_position(
            &raw mut (*(*popup).scene_tree).node,
            x - geo.x,
            y - geo.y,
        );
    }
}
unsafe extern "C" fn relay_send_im_state(
    mut relay: *mut sway_input_method_relay,
    mut input: *mut wlr_text_input_v3,
) {
    let mut input_method: *mut wlr_input_method_v2 = (*relay).input_method;
    if input_method.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Sending IM_DONE but im is gone\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
            213 as ::core::ffi::c_int,
        );
        return;
    }
    if (*input).active_features
        & wlr_text_input_v3_features::WLR_TEXT_INPUT_V3_FEATURE_SURROUNDING_TEXT.0
            as ::core::ffi::c_int as uint32_t != 0
    {
        wlr_input_method_v2_send_surrounding_text(
            input_method,
            (*input).current.surrounding.text,
            (*input).current.surrounding.cursor,
            (*input).current.surrounding.anchor,
        );
    }
    wlr_input_method_v2_send_text_change_cause(
        input_method,
        (*input).current.text_change_cause,
    );
    if (*input).active_features
        & wlr_text_input_v3_features::WLR_TEXT_INPUT_V3_FEATURE_CONTENT_TYPE.0
            as ::core::ffi::c_int as uint32_t != 0
    {
        wlr_input_method_v2_send_content_type(
            input_method,
            (*input).current.content_type.hint,
            (*input).current.content_type.purpose,
        );
    }
    let mut text_input: *mut sway_text_input = relay_get_focused_text_input(relay);
    let mut popup: *mut sway_input_popup = ::core::ptr::null_mut::<sway_input_popup>();
    popup = ((*relay).input_popups.next as *mut ::core::ffi::c_char)
        .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_input_popup
        as *mut sway_input_popup;
    while &raw mut (*popup).link != &raw mut (*relay).input_popups {
        if !text_input.is_null() {
            input_popup_set_focus(popup, (*(*text_input).input).focused_surface);
        } else {
            input_popup_set_focus(popup, ::core::ptr::null_mut::<wlr_surface>());
        }
        popup = ((*popup).link.next as *mut ::core::ffi::c_char)
            .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_input_popup
            as *mut sway_input_popup;
    }
    wlr_input_method_v2_send_done(input_method);
}
unsafe extern "C" fn handle_text_input_enable(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut text_input: *mut sway_text_input = (listener as *mut ::core::ffi::c_char)
        .offset(-(64 as ::core::ffi::c_ulong as isize)) as *mut sway_text_input;
    if (*(*text_input).input).focused_surface.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Enabling text input, but no longer focused\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
            248 as ::core::ffi::c_int,
        );
        return;
    }
    if (*(*text_input).relay).input_method.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Enabling text input when input method is gone\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
            252 as ::core::ffi::c_int,
        );
        return;
    }
    wlr_input_method_v2_send_activate((*(*text_input).relay).input_method);
    relay_send_im_state((*text_input).relay, (*text_input).input);
}
unsafe extern "C" fn handle_text_input_commit(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut text_input: *mut sway_text_input = (listener as *mut ::core::ffi::c_char)
        .offset(-(88 as ::core::ffi::c_ulong as isize)) as *mut sway_text_input;
    if (*(*text_input).input).focused_surface.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Unfocused text input tried to commit an update\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
            264 as ::core::ffi::c_int,
        );
        return;
    }
    if !(*(*text_input).input).current_enabled {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Inactive text input tried to commit an update\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
            268 as ::core::ffi::c_int,
        );
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Text input committed update\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
        271 as ::core::ffi::c_int,
    );
    if (*(*text_input).relay).input_method.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Text input committed, but input method is gone\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
            273 as ::core::ffi::c_int,
        );
        return;
    }
    relay_send_im_state((*text_input).relay, (*text_input).input);
}
unsafe extern "C" fn relay_disable_text_input(
    mut relay: *mut sway_input_method_relay,
    mut text_input: *mut sway_text_input,
) {
    if (*relay).input_method.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Disabling text input, but input method is gone\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
            282 as ::core::ffi::c_int,
        );
        return;
    }
    wlr_input_method_v2_send_deactivate((*relay).input_method);
    relay_send_im_state(relay, (*text_input).input);
}
unsafe extern "C" fn handle_text_input_disable(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut text_input: *mut sway_text_input = (listener as *mut ::core::ffi::c_char)
        .offset(-(112 as ::core::ffi::c_ulong as isize)) as *mut sway_text_input;
    if (*(*text_input).input).focused_surface.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Disabling text input, but no longer focused\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
            294 as ::core::ffi::c_int,
        );
        return;
    }
    relay_disable_text_input((*text_input).relay, text_input);
}
unsafe extern "C" fn handle_text_input_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut text_input: *mut sway_text_input = (listener as *mut ::core::ffi::c_char)
        .offset(-(136 as ::core::ffi::c_ulong as isize)) as *mut sway_text_input;
    if (*(*text_input).input).current_enabled {
        relay_disable_text_input((*text_input).relay, text_input);
    }
    text_input_set_pending_focused_surface(
        text_input,
        ::core::ptr::null_mut::<wlr_surface>(),
    );
    wl_list_remove(&raw mut (*text_input).text_input_commit.link);
    wl_list_remove(&raw mut (*text_input).text_input_destroy.link);
    wl_list_remove(&raw mut (*text_input).text_input_disable.link);
    wl_list_remove(&raw mut (*text_input).text_input_enable.link);
    wl_list_remove(&raw mut (*text_input).link);
    free(text_input as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn handle_pending_focused_surface_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut text_input: *mut sway_text_input = (listener as *mut ::core::ffi::c_char)
        .offset(-(40 as ::core::ffi::c_ulong as isize)) as *mut sway_text_input;
    (*text_input).pending_focused_surface = ::core::ptr::null_mut::<wlr_surface>();
    wl_list_remove(&raw mut (*text_input).pending_focused_surface_destroy.link);
    wl_list_init(&raw mut (*text_input).pending_focused_surface_destroy.link);
}
#[no_mangle]
pub unsafe extern "C" fn sway_text_input_create(
    mut relay: *mut sway_input_method_relay,
    mut text_input: *mut wlr_text_input_v3,
) -> *mut sway_text_input {
    let mut input: *mut sway_text_input = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_text_input>(),
    ) as *mut sway_text_input;
    if input.is_null() {
        return ::core::ptr::null_mut::<sway_text_input>();
    }
    (*input).input = text_input;
    (*input).relay = relay;
    wl_list_insert(&raw mut (*relay).text_inputs, &raw mut (*input).link);
    (*input).text_input_enable.notify = Some(
        handle_text_input_enable
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*text_input).events.enable,
        &raw mut (*input).text_input_enable,
    );
    (*input).text_input_commit.notify = Some(
        handle_text_input_commit
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*text_input).events.commit,
        &raw mut (*input).text_input_commit,
    );
    (*input).text_input_disable.notify = Some(
        handle_text_input_disable
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*text_input).events.disable,
        &raw mut (*input).text_input_disable,
    );
    (*input).text_input_destroy.notify = Some(
        handle_text_input_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*text_input).events.destroy,
        &raw mut (*input).text_input_destroy,
    );
    (*input).pending_focused_surface_destroy.notify = Some(
        handle_pending_focused_surface_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_list_init(&raw mut (*input).pending_focused_surface_destroy.link);
    return input;
}
unsafe extern "C" fn relay_handle_text_input(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut relay: *mut sway_input_method_relay = (listener as *mut ::core::ffi::c_char)
        .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_input_method_relay;
    let mut wlr_text_input: *mut wlr_text_input_v3 = data as *mut wlr_text_input_v3;
    if (*(*relay).seat).wlr_seat != (*wlr_text_input).seat {
        return;
    }
    sway_text_input_create(relay, wlr_text_input);
}
unsafe extern "C" fn input_popup_set_focus(
    mut popup: *mut sway_input_popup,
    mut surface: *mut wlr_surface,
) {
    wl_list_remove(&raw mut (*popup).focused_surface_unmap.link);
    if (*popup).scene_tree.is_null() {
        wl_list_init(&raw mut (*popup).focused_surface_unmap.link);
        return;
    }
    if !(*popup).desc.relative.is_null() {
        scene_descriptor_destroy(
            &raw mut (*(*popup).scene_tree).node,
            sway_scene_descriptor_type::SWAY_SCENE_DESC_POPUP,
        );
        wlr_scene_node_destroy((*popup).desc.relative);
        (*popup).desc.relative = ::core::ptr::null_mut::<wlr_scene_node>();
    }
    if surface.is_null() {
        wl_list_init(&raw mut (*popup).focused_surface_unmap.link);
        wlr_scene_node_set_enabled(&raw mut (*(*popup).scene_tree).node, r#false != 0);
        return;
    }
    let mut layer_surface: *mut wlr_layer_surface_v1 = wlr_layer_surface_v1_try_from_wlr_surface(
        surface,
    );
    let mut lock_surface: *mut wlr_session_lock_surface_v1 = wlr_session_lock_surface_v1_try_from_wlr_surface(
        surface,
    );
    let mut relative_parent: *mut wlr_scene_tree = ::core::ptr::null_mut::<
        wlr_scene_tree,
    >();
    if !layer_surface.is_null() {
        wl_signal_add(
            &raw mut (*(*layer_surface).surface).events.unmap,
            &raw mut (*popup).focused_surface_unmap,
        );
        let mut layer: *mut sway_layer_surface = (*layer_surface).data
            as *mut sway_layer_surface;
        if layer.is_null() {
            return;
        }
        relative_parent = (*(*layer).scene).tree;
        (*popup).desc.view = ::core::ptr::null_mut::<sway_view>();
        (*popup).fixed_output = (*(*layer).layer_surface).output;
    } else if !lock_surface.is_null() {
        wl_signal_add(
            &raw mut (*(*lock_surface).surface).events.unmap,
            &raw mut (*popup).focused_surface_unmap,
        );
        let mut lock: *mut sway_layer_surface = (*lock_surface).data
            as *mut sway_layer_surface;
        if lock.is_null() {
            return;
        }
        relative_parent = (*(*lock).scene).tree;
        (*popup).desc.view = ::core::ptr::null_mut::<sway_view>();
        (*popup).fixed_output = (*(*lock).layer_surface).output;
    } else {
        let mut view: *mut sway_view = view_from_wlr_surface(surface);
        if view.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Unsupported IME focus surface\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
                433 as ::core::ffi::c_int,
            );
            return;
        }
        wl_signal_add(
            &raw mut (*view).events.unmap,
            &raw mut (*popup).focused_surface_unmap,
        );
        relative_parent = (*view).scene_tree;
        (*popup).desc.view = view;
    }
    let mut relative: *mut wlr_scene_tree = wlr_scene_tree_create(relative_parent);
    (*popup).desc.relative = &raw mut (*relative).node;
    if !scene_descriptor_assign(
        &raw mut (*(*popup).scene_tree).node,
        sway_scene_descriptor_type::SWAY_SCENE_DESC_POPUP,
        &raw mut (*popup).desc as *mut ::core::ffi::c_void,
    ) {
        wlr_scene_node_destroy(&raw mut (*(*popup).scene_tree).node);
        (*popup).scene_tree = ::core::ptr::null_mut::<wlr_scene_tree>();
        return;
    }
    constrain_popup(popup);
    wlr_scene_node_set_enabled(&raw mut (*(*popup).scene_tree).node, r#true != 0);
}
unsafe extern "C" fn handle_im_popup_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut popup: *mut sway_input_popup = (listener as *mut ::core::ffi::c_char)
        .offset(-(64 as ::core::ffi::c_ulong as isize)) as *mut sway_input_popup;
    wlr_scene_node_destroy(&raw mut (*(*popup).scene_tree).node);
    wl_list_remove(&raw mut (*popup).focused_surface_unmap.link);
    wl_list_remove(&raw mut (*popup).popup_surface_commit.link);
    wl_list_remove(&raw mut (*popup).popup_surface_map.link);
    wl_list_remove(&raw mut (*popup).popup_surface_unmap.link);
    wl_list_remove(&raw mut (*popup).popup_destroy.link);
    wl_list_remove(&raw mut (*popup).link);
    free(popup as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn handle_im_popup_surface_map(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut popup: *mut sway_input_popup = (listener as *mut ::core::ffi::c_char)
        .offset(-(112 as ::core::ffi::c_ulong as isize)) as *mut sway_input_popup;
    let mut text_input: *mut sway_text_input = relay_get_focused_text_input(
        (*popup).relay,
    );
    if !text_input.is_null() {
        input_popup_set_focus(popup, (*(*text_input).input).focused_surface);
    } else {
        input_popup_set_focus(popup, ::core::ptr::null_mut::<wlr_surface>());
    };
}
unsafe extern "C" fn handle_im_popup_surface_unmap(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut popup: *mut sway_input_popup = (listener as *mut ::core::ffi::c_char)
        .offset(-(136 as ::core::ffi::c_ulong as isize)) as *mut sway_input_popup;
    scene_descriptor_destroy(
        &raw mut (*(*popup).scene_tree).node,
        sway_scene_descriptor_type::SWAY_SCENE_DESC_POPUP,
    );
    (*popup).desc.relative = ::core::ptr::null_mut::<wlr_scene_node>();
    input_popup_set_focus(popup, ::core::ptr::null_mut::<wlr_surface>());
}
unsafe extern "C" fn handle_im_popup_surface_commit(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut popup: *mut sway_input_popup = (listener as *mut ::core::ffi::c_char)
        .offset(-(88 as ::core::ffi::c_ulong as isize)) as *mut sway_input_popup;
    constrain_popup(popup);
}
unsafe extern "C" fn handle_im_focused_surface_unmap(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut popup: *mut sway_input_popup = (listener as *mut ::core::ffi::c_char)
        .offset(-(160 as ::core::ffi::c_ulong as isize)) as *mut sway_input_popup;
    input_popup_set_focus(popup, ::core::ptr::null_mut::<wlr_surface>());
}
unsafe extern "C" fn handle_im_new_popup_surface(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut relay: *mut sway_input_method_relay = (listener as *mut ::core::ffi::c_char)
        .offset(-(168 as ::core::ffi::c_ulong as isize)) as *mut sway_input_method_relay;
    let mut popup: *mut sway_input_popup = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_input_popup>(),
    ) as *mut sway_input_popup;
    if popup.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to allocate an input method popup\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
            512 as ::core::ffi::c_int,
        );
        return;
    }
    (*popup).relay = relay;
    (*popup).popup_surface = data as *mut wlr_input_popup_surface_v2;
    (*(*popup).popup_surface).data = popup as *mut ::core::ffi::c_void;
    (*popup).scene_tree = wlr_scene_tree_create((*root).layers.popup);
    if (*popup).scene_tree.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to allocate scene tree\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
            522 as ::core::ffi::c_int,
        );
        free(popup as *mut ::core::ffi::c_void);
        return;
    }
    if wlr_scene_subsurface_tree_create(
            (*popup).scene_tree,
            (*(*popup).popup_surface).surface,
        )
        .is_null()
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to allocate subsurface tree\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
            529 as ::core::ffi::c_int,
        );
        wlr_scene_node_destroy(&raw mut (*(*popup).scene_tree).node);
        free(popup as *mut ::core::ffi::c_void);
        return;
    }
    wl_signal_add(
        &raw mut (*(*popup).popup_surface).events.destroy,
        &raw mut (*popup).popup_destroy,
    );
    (*popup).popup_destroy.notify = Some(
        handle_im_popup_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*(*popup).popup_surface).surface).events.commit,
        &raw mut (*popup).popup_surface_commit,
    );
    (*popup).popup_surface_commit.notify = Some(
        handle_im_popup_surface_commit
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*(*popup).popup_surface).surface).events.map,
        &raw mut (*popup).popup_surface_map,
    );
    (*popup).popup_surface_map.notify = Some(
        handle_im_popup_surface_map
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*(*popup).popup_surface).surface).events.unmap,
        &raw mut (*popup).popup_surface_unmap,
    );
    (*popup).popup_surface_unmap.notify = Some(
        handle_im_popup_surface_unmap
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_list_init(&raw mut (*popup).focused_surface_unmap.link);
    (*popup).focused_surface_unmap.notify = Some(
        handle_im_focused_surface_unmap
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    let mut text_input: *mut sway_text_input = relay_get_focused_text_input(relay);
    if !text_input.is_null() {
        input_popup_set_focus(popup, (*(*text_input).input).focused_surface);
    } else {
        input_popup_set_focus(popup, ::core::ptr::null_mut::<wlr_surface>());
    }
    wl_list_insert(&raw mut (*relay).input_popups, &raw mut (*popup).link);
}
unsafe extern "C" fn text_input_send_enter(
    mut text_input: *mut sway_text_input,
    mut surface: *mut wlr_surface,
) {
    wlr_text_input_v3_send_enter((*text_input).input, surface);
    let mut popup: *mut sway_input_popup = ::core::ptr::null_mut::<sway_input_popup>();
    popup = ((*(*text_input).relay).input_popups.next as *mut ::core::ffi::c_char)
        .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_input_popup
        as *mut sway_input_popup;
    while &raw mut (*popup).link != &raw mut (*(*text_input).relay).input_popups {
        input_popup_set_focus(popup, surface);
        popup = ((*popup).link.next as *mut ::core::ffi::c_char)
            .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_input_popup
            as *mut sway_input_popup;
    }
}
unsafe extern "C" fn relay_handle_input_method(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut relay: *mut sway_input_method_relay = (listener as *mut ::core::ffi::c_char)
        .offset(-(96 as ::core::ffi::c_ulong as isize)) as *mut sway_input_method_relay;
    let mut input_method: *mut wlr_input_method_v2 = data as *mut wlr_input_method_v2;
    if (*(*relay).seat).wlr_seat != (*input_method).seat {
        return;
    }
    if !(*relay).input_method.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Attempted to connect second input method to a seat\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/text_input.c\0".as_ptr() as *const ::core::ffi::c_char,
            575 as ::core::ffi::c_int,
        );
        wlr_input_method_v2_send_unavailable(input_method);
        return;
    }
    (*relay).input_method = input_method;
    wl_signal_add(
        &raw mut (*(*relay).input_method).events.commit,
        &raw mut (*relay).input_method_commit,
    );
    (*relay).input_method_commit.notify = Some(
        handle_im_commit
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*relay).input_method).events.grab_keyboard,
        &raw mut (*relay).input_method_grab_keyboard,
    );
    (*relay).input_method_grab_keyboard.notify = Some(
        handle_im_grab_keyboard
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*relay).input_method).events.destroy,
        &raw mut (*relay).input_method_destroy,
    );
    (*relay).input_method_destroy.notify = Some(
        handle_im_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*relay).input_method).events.new_popup_surface,
        &raw mut (*relay).input_method_new_popup_surface,
    );
    (*relay).input_method_new_popup_surface.notify = Some(
        handle_im_new_popup_surface
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    let mut text_input: *mut sway_text_input = relay_get_focusable_text_input(relay);
    if !text_input.is_null() {
        text_input_send_enter(text_input, (*text_input).pending_focused_surface);
        text_input_set_pending_focused_surface(
            text_input,
            ::core::ptr::null_mut::<wlr_surface>(),
        );
    }
}
unsafe extern "C" fn sway_input_method_relay_finish_text_input(
    mut relay: *mut sway_input_method_relay,
) {
    wl_list_remove(&raw mut (*relay).text_input_new.link);
    wl_list_remove(&raw mut (*relay).text_input_manager_destroy.link);
    wl_list_init(&raw mut (*relay).text_input_new.link);
    wl_list_init(&raw mut (*relay).text_input_manager_destroy.link);
}
unsafe extern "C" fn relay_handle_text_input_manager_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut relay: *mut sway_input_method_relay = (listener as *mut ::core::ffi::c_char)
        .offset(-(72 as ::core::ffi::c_ulong as isize)) as *mut sway_input_method_relay;
    sway_input_method_relay_finish_text_input(relay);
}
unsafe extern "C" fn sway_input_method_relay_finish_input_method(
    mut relay: *mut sway_input_method_relay,
) {
    wl_list_remove(&raw mut (*relay).input_method_new.link);
    wl_list_remove(&raw mut (*relay).input_method_manager_destroy.link);
    wl_list_init(&raw mut (*relay).input_method_new.link);
    wl_list_init(&raw mut (*relay).input_method_manager_destroy.link);
}
unsafe extern "C" fn relay_handle_input_method_manager_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut relay: *mut sway_input_method_relay = (listener as *mut ::core::ffi::c_char)
        .offset(-(120 as ::core::ffi::c_ulong as isize)) as *mut sway_input_method_relay;
    sway_input_method_relay_finish_input_method(relay);
}
#[no_mangle]
pub unsafe extern "C" fn sway_input_method_relay_init(
    mut seat: *mut sway_seat,
    mut relay: *mut sway_input_method_relay,
) {
    (*relay).seat = seat;
    wl_list_init(&raw mut (*relay).text_inputs);
    wl_list_init(&raw mut (*relay).input_popups);
    (*relay).text_input_new.notify = Some(
        relay_handle_text_input
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*server.text_input).events.new_text_input,
        &raw mut (*relay).text_input_new,
    );
    (*relay).text_input_manager_destroy.notify = Some(
        relay_handle_text_input_manager_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*server.text_input).events.destroy,
        &raw mut (*relay).text_input_manager_destroy,
    );
    (*relay).input_method_new.notify = Some(
        relay_handle_input_method
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*server.input_method).events.new_input_method,
        &raw mut (*relay).input_method_new,
    );
    (*relay).input_method_manager_destroy.notify = Some(
        relay_handle_input_method_manager_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*server.input_method).events.destroy,
        &raw mut (*relay).input_method_manager_destroy,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sway_input_method_relay_finish(
    mut relay: *mut sway_input_method_relay,
) {
    sway_input_method_relay_finish_text_input(relay);
    sway_input_method_relay_finish_input_method(relay);
}
#[no_mangle]
pub unsafe extern "C" fn sway_input_method_relay_set_focus(
    mut relay: *mut sway_input_method_relay,
    mut surface: *mut wlr_surface,
) {
    let mut text_input: *mut sway_text_input = ::core::ptr::null_mut::<
        sway_text_input,
    >();
    text_input = ((*relay).text_inputs.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut sway_text_input
        as *mut sway_text_input;
    while &raw mut (*text_input).link != &raw mut (*relay).text_inputs {
        's_6: {
            if !(*text_input).pending_focused_surface.is_null() {
                if (*(*text_input).input).focused_surface.is_null() {} else {
                    __assert_fail(
                        b"text_input->input->focused_surface == NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/input/text_input.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        662 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
                if surface != (*text_input).pending_focused_surface {
                    text_input_set_pending_focused_surface(
                        text_input,
                        ::core::ptr::null_mut::<wlr_surface>(),
                    );
                }
            } else if !(*(*text_input).input).focused_surface.is_null() {
                if (*text_input).pending_focused_surface.is_null() {} else {
                    __assert_fail(
                        b"text_input->pending_focused_surface == NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/input/text_input.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        667 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
                if surface != (*(*text_input).input).focused_surface {
                    wlr_text_input_v3_send_leave((*text_input).input);
                    relay_disable_text_input(relay, text_input);
                } else {
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] IM relay set_focus already focused\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/input/text_input.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        672 as ::core::ffi::c_int,
                    );
                    break 's_6;
                }
            }
            if !surface.is_null()
                && wl_resource_get_client((*(*text_input).input).resource)
                    == wl_resource_get_client((*surface).resource)
            {
                if !(*relay).input_method.is_null() {
                    wlr_text_input_v3_send_enter((*text_input).input, surface);
                } else {
                    text_input_set_pending_focused_surface(text_input, surface);
                }
            }
        }
        text_input = ((*text_input).link.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut sway_text_input
            as *mut sway_text_input;
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
