extern "C" {
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
    pub type wlr_cursor_state;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_primary_selection_source;
    pub type wlr_tablet_pad_client_v2;
    pub type wlr_tablet_pad;
    pub type wlr_tablet_client_v2;
    pub type wlr_tablet_impl;
    pub type wlr_tablet_tool_client_v2;
    pub type wlr_linux_dmabuf_feedback_v1_compiled;
    pub type wlr_output_layer_feedback_event;
    pub type wlr_gamma_control_v1;
    pub type wlr_color_manager_v1;
    pub type wlr_gamma_control_manager_v1;
    pub type sway_transaction_instruction;
    pub type wlr_xwm;
    pub type wlr_xdg_toplevel;
    pub type wlr_foreign_toplevel_handle_v1;
    pub type wlr_ext_foreign_toplevel_handle_v1;
    pub type launcher_ctx;
    pub type sway_xdg_decoration;
    pub type sway_transaction;
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
    pub type wlr_session_lock_manager_v1;
    pub type wlr_session_lock_v1;
    pub type wlr_output_manager_v1;
    pub type wlr_xdg_output_manager_v1;
    pub type wlr_drm_lease_v1_manager;
    pub type wlr_xdg_decoration_manager_v1;
    pub type wlr_server_decoration_manager;
    pub type wlr_relative_pointer_manager_v1;
    pub type wlr_xwayland_shell_v1;
    pub type wlr_xdg_shell;
    pub type wlr_idle_notifier_v1;
    pub type wlr_data_device_manager;
    pub type wlr_session;
    pub type sway_text_node;
    pub type wlr_ext_image_capture_source_v1;
    pub type wlr_touch_impl;
    pub type sway_switch;
    pub type sway_keyboard;
    pub type _PangoFontDescription;
    fn roundf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn wlr_keyboard_get_modifiers(keyboard: *mut wlr_keyboard) -> uint32_t;
    fn wlr_seat_pointer_notify_enter(
        wlr_seat: *mut wlr_seat,
        surface: *mut wlr_surface,
        sx: ::core::ffi::c_double,
        sy: ::core::ffi::c_double,
    );
    fn wlr_seat_pointer_notify_clear_focus(wlr_seat: *mut wlr_seat);
    fn wlr_seat_pointer_notify_motion(
        wlr_seat: *mut wlr_seat,
        time_msec: uint32_t,
        sx: ::core::ffi::c_double,
        sy: ::core::ffi::c_double,
    );
    fn wlr_seat_pointer_notify_axis(
        wlr_seat: *mut wlr_seat,
        time_msec: uint32_t,
        orientation: wl_pointer_axis,
        value: ::core::ffi::c_double,
        value_discrete: int32_t,
        source: wl_pointer_axis_source,
        relative_direction: wl_pointer_axis_relative_direction,
    );
    fn wlr_seat_pointer_notify_frame(wlr_seat: *mut wlr_seat);
    fn wlr_seat_get_keyboard(seat: *mut wlr_seat) -> *mut wlr_keyboard;
    fn wlr_surface_accepts_touch(
        surface: *mut wlr_surface,
        wlr_seat: *mut wlr_seat,
    ) -> bool;
    fn wlr_tablet_v2_tablet_tool_notify_proximity_in(
        tool: *mut wlr_tablet_v2_tablet_tool,
        tablet: *mut wlr_tablet_v2_tablet,
        surface: *mut wlr_surface,
    );
    fn wlr_tablet_v2_tablet_tool_notify_down(tool: *mut wlr_tablet_v2_tablet_tool);
    fn wlr_tablet_v2_tablet_tool_notify_up(tool: *mut wlr_tablet_v2_tablet_tool);
    fn wlr_tablet_v2_tablet_tool_notify_motion(
        tool: *mut wlr_tablet_v2_tablet_tool,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    );
    fn wlr_tablet_v2_tablet_tool_notify_proximity_out(
        tool: *mut wlr_tablet_v2_tablet_tool,
    );
    fn wlr_tablet_tool_v2_start_implicit_grab(tool: *mut wlr_tablet_v2_tablet_tool);
    fn gesture_to_string(gesture: *mut gesture) -> *mut ::core::ffi::c_char;
    fn gesture_check(
        target: *mut gesture,
        r#type: gesture_type,
        fingers: uint8_t,
    ) -> bool;
    fn gesture_match(target: *mut gesture, to_match: *mut gesture, exact: bool) -> bool;
    fn gesture_compare(a: *mut gesture, b: *mut gesture) -> int8_t;
    fn gesture_tracker_begin(
        tracker: *mut gesture_tracker,
        r#type: gesture_type,
        fingers: uint8_t,
    );
    fn gesture_tracker_check(
        tracker: *mut gesture_tracker,
        r#type: gesture_type,
    ) -> bool;
    fn gesture_tracker_update(
        tracker: *mut gesture_tracker,
        dx: ::core::ffi::c_double,
        dy: ::core::ffi::c_double,
        scale: ::core::ffi::c_double,
        rotation: ::core::ffi::c_double,
    );
    fn gesture_tracker_cancel(tracker: *mut gesture_tracker);
    fn gesture_tracker_end(tracker: *mut gesture_tracker) -> *mut gesture;
    fn wlr_layer_surface_v1_try_from_wlr_surface(
        surface: *mut wlr_surface,
    ) -> *mut wlr_layer_surface_v1;
    fn transaction_commit_dirty();
    fn view_is_visible(view: *mut sway_view) -> bool;
    static mut root: *mut sway_root;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn node_is_view(node: *mut sway_node) -> bool;
    fn node_get_output(node: *mut sway_node) -> *mut sway_output;
    fn node_get_parent(node: *mut sway_node) -> *mut sway_node;
    fn container_has_ancestor(
        container: *mut sway_container,
        ancestor: *mut sway_container,
    ) -> bool;
    fn container_is_floating(container: *mut sway_container) -> bool;
    fn container_toplevel_ancestor(
        container: *mut sway_container,
    ) -> *mut sway_container;
    fn container_is_floating_or_child(container: *mut sway_container) -> bool;
    fn container_is_fullscreen_or_child(container: *mut sway_container) -> bool;
    fn container_parent_layout(con: *mut sway_container) -> sway_container_layout;
    fn container_get_siblings(container: *mut sway_container) -> *mut list_t;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn seat_set_focus(seat: *mut sway_seat, node: *mut sway_node);
    fn seat_set_focus_container(seat: *mut sway_seat, con: *mut sway_container);
    fn seat_set_focus_surface(
        seat: *mut sway_seat,
        surface: *mut wlr_surface,
        unfocus: bool,
    );
    fn seat_set_focus_layer(seat: *mut sway_seat, layer: *mut wlr_layer_surface_v1);
    fn seat_get_focus(seat: *mut sway_seat) -> *mut sway_node;
    fn seat_get_focused_container(seat: *mut sway_seat) -> *mut sway_container;
    fn seat_get_focus_inactive(
        seat: *mut sway_seat,
        node: *mut sway_node,
    ) -> *mut sway_node;
    fn seat_get_focus_inactive_view(
        seat: *mut sway_seat,
        ancestor: *mut sway_node,
    ) -> *mut sway_container;
    fn seat_get_active_tiling_child(
        seat: *mut sway_seat,
        parent: *mut sway_node,
    ) -> *mut sway_node;
    fn seat_is_input_allowed(seat: *mut sway_seat, surface: *mut wlr_surface) -> bool;
    fn drag_icons_update_position(seat: *mut sway_seat);
    fn seatop_begin_down(
        seat: *mut sway_seat,
        con: *mut sway_container,
        sx: ::core::ffi::c_double,
        sy: ::core::ffi::c_double,
    );
    fn seatop_begin_down_on_surface(
        seat: *mut sway_seat,
        surface: *mut wlr_surface,
        sx: ::core::ffi::c_double,
        sy: ::core::ffi::c_double,
    );
    fn seatop_begin_touch_down(
        seat: *mut sway_seat,
        surface: *mut wlr_surface,
        event: *mut wlr_touch_down_event,
        sx: ::core::ffi::c_double,
        sy: ::core::ffi::c_double,
        lx: ::core::ffi::c_double,
        ly: ::core::ffi::c_double,
    );
    fn seatop_begin_move_floating(seat: *mut sway_seat, con: *mut sway_container);
    fn seatop_begin_move_tiling_threshold(
        seat: *mut sway_seat,
        con: *mut sway_container,
    );
    fn seatop_begin_move_tiling(seat: *mut sway_seat, con: *mut sway_container);
    fn seatop_begin_resize_floating(
        seat: *mut sway_seat,
        con: *mut sway_container,
        edge: wlr_edges,
    );
    fn seatop_begin_resize_tiling(
        seat: *mut sway_seat,
        con: *mut sway_container,
        edge: wlr_edges,
    );
    fn seat_pointer_notify_button(
        seat: *mut sway_seat,
        time_msec: uint32_t,
        button: uint32_t,
        state: wl_pointer_button_state,
    );
    fn seatop_rebase(seat: *mut sway_seat, time_msec: uint32_t);
    fn seatop_end(seat: *mut sway_seat);
    fn node_at_coords(
        seat: *mut sway_seat,
        lx: ::core::ffi::c_double,
        ly: ::core::ffi::c_double,
        surface: *mut *mut wlr_surface,
        sx: *mut ::core::ffi::c_double,
        sy: *mut ::core::ffi::c_double,
    ) -> *mut sway_node;
    fn cursor_update_image(cursor: *mut sway_cursor, node: *mut sway_node);
    fn pointer_motion(
        cursor: *mut sway_cursor,
        time_msec: uint32_t,
        device: *mut wlr_input_device,
        dx: ::core::ffi::c_double,
        dy: ::core::ffi::c_double,
        dx_unaccel: ::core::ffi::c_double,
        dy_unaccel: ::core::ffi::c_double,
    );
    fn dispatch_cursor_button(
        cursor: *mut sway_cursor,
        device: *mut wlr_input_device,
        time_msec: uint32_t,
        button: uint32_t,
        state: wl_pointer_button_state,
    );
    fn cursor_set_image(
        cursor: *mut sway_cursor,
        image: *const ::core::ffi::c_char,
        client: *mut wl_client,
    );
    static mut server: sway_server;
    fn output_get_active_workspace(output: *mut sway_output) -> *mut sway_workspace;
    fn input_device_get_config(device: *mut sway_input_device) -> *mut input_config;
    fn input_device_get_identifier(
        device: *mut wlr_input_device,
    ) -> *mut ::core::ffi::c_char;
    fn toplevel_layer_surface_from_surface(
        surface: *mut wlr_surface,
    ) -> *mut wlr_layer_surface_v1;
    fn wlr_output_layout_output_at(
        layout: *mut wlr_output_layout,
        lx: ::core::ffi::c_double,
        ly: ::core::ffi::c_double,
    ) -> *mut wlr_output;
    fn wlr_xwayland_set_seat(xwayland: *mut wlr_xwayland, seat: *mut wlr_seat);
    fn wlr_xwayland_surface_try_from_wlr_surface(
        surface: *mut wlr_surface,
    ) -> *mut wlr_xwayland_surface;
    fn wlr_xwayland_surface_override_redirect_wants_focus(
        xsurface: *const wlr_xwayland_surface,
    ) -> bool;
    fn wlr_pointer_gestures_v1_send_swipe_begin(
        gestures: *mut wlr_pointer_gestures_v1,
        seat: *mut wlr_seat,
        time_msec: uint32_t,
        fingers: uint32_t,
    );
    fn wlr_pointer_gestures_v1_send_swipe_update(
        gestures: *mut wlr_pointer_gestures_v1,
        seat: *mut wlr_seat,
        time_msec: uint32_t,
        dx: ::core::ffi::c_double,
        dy: ::core::ffi::c_double,
    );
    fn wlr_pointer_gestures_v1_send_swipe_end(
        gestures: *mut wlr_pointer_gestures_v1,
        seat: *mut wlr_seat,
        time_msec: uint32_t,
        cancelled: bool,
    );
    fn wlr_pointer_gestures_v1_send_pinch_begin(
        gestures: *mut wlr_pointer_gestures_v1,
        seat: *mut wlr_seat,
        time_msec: uint32_t,
        fingers: uint32_t,
    );
    fn wlr_pointer_gestures_v1_send_pinch_update(
        gestures: *mut wlr_pointer_gestures_v1,
        seat: *mut wlr_seat,
        time_msec: uint32_t,
        dx: ::core::ffi::c_double,
        dy: ::core::ffi::c_double,
        scale: ::core::ffi::c_double,
        rotation: ::core::ffi::c_double,
    );
    fn wlr_pointer_gestures_v1_send_pinch_end(
        gestures: *mut wlr_pointer_gestures_v1,
        seat: *mut wlr_seat,
        time_msec: uint32_t,
        cancelled: bool,
    );
    fn wlr_pointer_gestures_v1_send_hold_begin(
        gestures: *mut wlr_pointer_gestures_v1,
        seat: *mut wlr_seat,
        time_msec: uint32_t,
        fingers: uint32_t,
    );
    fn wlr_pointer_gestures_v1_send_hold_end(
        gestures: *mut wlr_pointer_gestures_v1,
        seat: *mut wlr_seat,
        time_msec: uint32_t,
        cancelled: bool,
    );
    fn seat_execute_command(seat: *mut sway_seat, binding: *mut sway_binding);
    static mut config: *mut sway_config;
    fn list_find(
        list: *mut list_t,
        item: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    fn get_current_time_in_msec() -> uint32_t;
}
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pid_t = __pid_t;
pub type size_t = usize;
pub type int8_t = i8;
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
pub struct wlr_output_layout {
    pub outputs: wl_list,
    pub display: *mut wl_display,
    pub events: C2Rust_Unnamed_15,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_device {
    pub r#type: wlr_input_device_type,
    pub name: *mut ::core::ffi::c_char,
    pub events: C2Rust_Unnamed_16,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
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
    pub events: C2Rust_Unnamed_19,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_18,
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
pub struct C2Rust_Unnamed_18 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
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
    pub viewport: C2Rust_Unnamed_20,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
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
    pub events: C2Rust_Unnamed_22,
    pub WLR_PRIVATE: C2Rust_Unnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xcursor_manager {
    pub name: *mut ::core::ffi::c_char,
    pub size: uint32_t,
    pub scaled_themes: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_cursor {
    pub state: *mut wlr_cursor_state,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub events: C2Rust_Unnamed_23,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
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
    pub touch_up: wl_signal,
    pub touch_down: wl_signal,
    pub touch_motion: wl_signal,
    pub touch_cancel: wl_signal,
    pub touch_frame: wl_signal,
    pub tablet_tool_axis: wl_signal,
    pub tablet_tool_proximity: wl_signal,
    pub tablet_tool_tip: wl_signal,
    pub tablet_tool_button: wl_signal,
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
pub struct zwp_tablet_pad_v2_button_state(pub ::core::ffi::c_uint);
impl zwp_tablet_pad_v2_button_state {
    pub const ZWP_TABLET_PAD_V2_BUTTON_STATE_RELEASED: Self = Self(0);
    pub const ZWP_TABLET_PAD_V2_BUTTON_STATE_PRESSED: Self = Self(1);
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
    pub repeat_info: C2Rust_Unnamed_25,
    pub events: C2Rust_Unnamed_24,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
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
    pub events: C2Rust_Unnamed_26,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
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
    pub events: C2Rust_Unnamed_28,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub acc_discrete: [int32_t; 2],
    pub last_discrete: [int32_t; 2],
    pub acc_axis: [::core::ffi::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
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
    pub events: C2Rust_Unnamed_30,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
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
    pub events: C2Rust_Unnamed_32,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
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
    pub events: C2Rust_Unnamed_34,
    pub WLR_PRIVATE: C2Rust_Unnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
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
    pub events: C2Rust_Unnamed_36,
    pub WLR_PRIVATE: C2Rust_Unnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
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
pub struct wlr_tablet_pad_v2_grab_interface {
    pub enter: Option<
        unsafe extern "C" fn(
            *mut wlr_tablet_pad_v2_grab,
            *mut wlr_tablet_v2_tablet,
            *mut wlr_surface,
        ) -> uint32_t,
    >,
    pub button: Option<
        unsafe extern "C" fn(
            *mut wlr_tablet_pad_v2_grab,
            size_t,
            uint32_t,
            zwp_tablet_pad_v2_button_state,
        ) -> (),
    >,
    pub strip: Option<
        unsafe extern "C" fn(
            *mut wlr_tablet_pad_v2_grab,
            uint32_t,
            ::core::ffi::c_double,
            bool,
            uint32_t,
        ) -> (),
    >,
    pub ring: Option<
        unsafe extern "C" fn(
            *mut wlr_tablet_pad_v2_grab,
            uint32_t,
            ::core::ffi::c_double,
            bool,
            uint32_t,
        ) -> (),
    >,
    pub leave: Option<
        unsafe extern "C" fn(*mut wlr_tablet_pad_v2_grab, *mut wlr_surface) -> uint32_t,
    >,
    pub mode: Option<
        unsafe extern "C" fn(
            *mut wlr_tablet_pad_v2_grab,
            size_t,
            uint32_t,
            uint32_t,
        ) -> uint32_t,
    >,
    pub cancel: Option<unsafe extern "C" fn(*mut wlr_tablet_pad_v2_grab) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_pad_v2_grab {
    pub interface: *const wlr_tablet_pad_v2_grab_interface,
    pub pad: *mut wlr_tablet_v2_tablet_pad,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_v2_tablet_pad {
    pub link: wl_list,
    pub wlr_pad: *mut wlr_tablet_pad,
    pub wlr_device: *mut wlr_input_device,
    pub clients: wl_list,
    pub group_count: size_t,
    pub groups: *mut uint32_t,
    pub current_client: *mut wlr_tablet_pad_client_v2,
    pub grab: *mut wlr_tablet_pad_v2_grab,
    pub default_grab: wlr_tablet_pad_v2_grab,
    pub events: C2Rust_Unnamed_38,
    pub WLR_PRIVATE: C2Rust_Unnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_37 {
    pub pad_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
    pub button_feedback: wl_signal,
    pub strip_feedback: wl_signal,
    pub ring_feedback: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_v2_tablet {
    pub link: wl_list,
    pub wlr_tablet: *mut wlr_tablet,
    pub wlr_device: *mut wlr_input_device,
    pub clients: wl_list,
    pub current_client: *mut wlr_tablet_client_v2,
    pub WLR_PRIVATE: C2Rust_Unnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
    pub tablet_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet {
    pub base: wlr_input_device,
    pub r#impl: *const wlr_tablet_impl,
    pub usb_vendor_id: uint16_t,
    pub usb_product_id: uint16_t,
    pub width_mm: ::core::ffi::c_double,
    pub height_mm: ::core::ffi::c_double,
    pub events: C2Rust_Unnamed_40,
    pub paths: wl_array,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_40 {
    pub axis: wl_signal,
    pub proximity: wl_signal,
    pub tip: wl_signal,
    pub button: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_tool_v2_grab_interface {
    pub proximity_in: Option<
        unsafe extern "C" fn(
            *mut wlr_tablet_tool_v2_grab,
            *mut wlr_tablet_v2_tablet,
            *mut wlr_surface,
        ) -> (),
    >,
    pub down: Option<unsafe extern "C" fn(*mut wlr_tablet_tool_v2_grab) -> ()>,
    pub up: Option<unsafe extern "C" fn(*mut wlr_tablet_tool_v2_grab) -> ()>,
    pub motion: Option<
        unsafe extern "C" fn(
            *mut wlr_tablet_tool_v2_grab,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub pressure: Option<
        unsafe extern "C" fn(*mut wlr_tablet_tool_v2_grab, ::core::ffi::c_double) -> (),
    >,
    pub distance: Option<
        unsafe extern "C" fn(*mut wlr_tablet_tool_v2_grab, ::core::ffi::c_double) -> (),
    >,
    pub tilt: Option<
        unsafe extern "C" fn(
            *mut wlr_tablet_tool_v2_grab,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub rotation: Option<
        unsafe extern "C" fn(*mut wlr_tablet_tool_v2_grab, ::core::ffi::c_double) -> (),
    >,
    pub slider: Option<
        unsafe extern "C" fn(*mut wlr_tablet_tool_v2_grab, ::core::ffi::c_double) -> (),
    >,
    pub wheel: Option<
        unsafe extern "C" fn(
            *mut wlr_tablet_tool_v2_grab,
            ::core::ffi::c_double,
            int32_t,
        ) -> (),
    >,
    pub proximity_out: Option<unsafe extern "C" fn(*mut wlr_tablet_tool_v2_grab) -> ()>,
    pub button: Option<
        unsafe extern "C" fn(
            *mut wlr_tablet_tool_v2_grab,
            uint32_t,
            zwp_tablet_pad_v2_button_state,
        ) -> (),
    >,
    pub cancel: Option<unsafe extern "C" fn(*mut wlr_tablet_tool_v2_grab) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_tool_v2_grab {
    pub interface: *const wlr_tablet_tool_v2_grab_interface,
    pub tool: *mut wlr_tablet_v2_tablet_tool,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_v2_tablet_tool {
    pub link: wl_list,
    pub wlr_tool: *mut wlr_tablet_tool,
    pub clients: wl_list,
    pub current_client: *mut wlr_tablet_tool_client_v2,
    pub focused_surface: *mut wlr_surface,
    pub grab: *mut wlr_tablet_tool_v2_grab,
    pub default_grab: wlr_tablet_tool_v2_grab,
    pub proximity_serial: uint32_t,
    pub is_down: bool,
    pub down_serial: uint32_t,
    pub num_buttons: size_t,
    pub pressed_buttons: [uint32_t; 16],
    pub pressed_serials: [uint32_t; 16],
    pub events: C2Rust_Unnamed_42,
    pub WLR_PRIVATE: C2Rust_Unnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_41 {
    pub surface_destroy: wl_listener,
    pub tool_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
    pub set_cursor: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_tool {
    pub r#type: wlr_tablet_tool_type,
    pub hardware_serial: uint64_t,
    pub hardware_wacom: uint64_t,
    pub tilt: bool,
    pub pressure: bool,
    pub distance: bool,
    pub rotation: bool,
    pub slider: bool,
    pub wheel: bool,
    pub events: C2Rust_Unnamed_43,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
    pub destroy: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_tablet_tool_type(pub ::core::ffi::c_uint);
impl wlr_tablet_tool_type {
    pub const WLR_TABLET_TOOL_TYPE_PEN: Self = Self(1);
    pub const WLR_TABLET_TOOL_TYPE_ERASER: Self = Self(2);
    pub const WLR_TABLET_TOOL_TYPE_BRUSH: Self = Self(3);
    pub const WLR_TABLET_TOOL_TYPE_PENCIL: Self = Self(4);
    pub const WLR_TABLET_TOOL_TYPE_AIRBRUSH: Self = Self(5);
    pub const WLR_TABLET_TOOL_TYPE_MOUSE: Self = Self(6);
    pub const WLR_TABLET_TOOL_TYPE_LENS: Self = Self(7);
    pub const WLR_TABLET_TOOL_TYPE_TOTEM: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_manager_v2 {
    pub wl_global: *mut wl_global,
    pub clients: wl_list,
    pub seats: wl_list,
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
    pub destroy: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_edges(pub ::core::ffi::c_uint);
impl wlr_edges {
    pub const WLR_EDGE_NONE: Self = Self(0);
    pub const WLR_EDGE_TOP: Self = Self(1);
    pub const WLR_EDGE_BOTTOM: Self = Self(2);
    pub const WLR_EDGE_LEFT: Self = Self(4);
    pub const WLR_EDGE_RIGHT: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct gesture_type(pub ::core::ffi::c_uint);
impl gesture_type {
    pub const GESTURE_TYPE_NONE: Self = Self(0);
    pub const GESTURE_TYPE_HOLD: Self = Self(1);
    pub const GESTURE_TYPE_PINCH: Self = Self(2);
    pub const GESTURE_TYPE_SWIPE: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gesture {
    pub r#type: gesture_type,
    pub fingers: uint8_t,
    pub directions: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gesture_tracker {
    pub r#type: gesture_type,
    pub fingers: uint8_t,
    pub dx: ::core::ffi::c_double,
    pub dy: ::core::ffi::c_double,
    pub scale: ::core::ffi::c_double,
    pub rotation: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_damage_ring {
    pub current: pixman_region32_t,
    pub WLR_PRIVATE: C2Rust_Unnamed_46,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_48,
    pub WLR_PRIVATE: C2Rust_Unnamed_47,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
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
pub struct C2Rust_Unnamed_48 {
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
    pub events: C2Rust_Unnamed_50,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_49,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_51,
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
pub struct C2Rust_Unnamed_51 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_53,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_52,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_52 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_53 {
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
    pub events: C2Rust_Unnamed_55,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_54,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_54 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_56,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_56 {
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
    pub events: C2Rust_Unnamed_61,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_57,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_57 {
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
    pub events: C2Rust_Unnamed_59,
    pub WLR_PRIVATE: C2Rust_Unnamed_58,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_58 {
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
pub struct C2Rust_Unnamed_59 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_60,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_60 {
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
pub struct C2Rust_Unnamed_61 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_63,
    pub events: C2Rust_Unnamed_62,
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
pub struct C2Rust_Unnamed_62 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_63 {
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
    pub events: C2Rust_Unnamed_65,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_65 {
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
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_106,
    pub border: C2Rust_Unnamed_105,
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
    pub events: C2Rust_Unnamed_66,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
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
    pub layers: C2Rust_Unnamed_104,
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
    pub events: C2Rust_Unnamed_68,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_67,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_group_handle_v1 {
    pub manager: *mut wlr_ext_workspace_manager_v1,
    pub caps: uint32_t,
    pub events: C2Rust_Unnamed_70,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_69,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
    pub outputs: wl_list,
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_manager_v1 {
    pub global: *mut wl_global,
    pub groups: wl_list,
    pub workspaces: wl_list,
    pub events: C2Rust_Unnamed_72,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_71,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_71 {
    pub resources: wl_list,
    pub idle_source: *mut wl_event_source,
    pub event_loop: *mut wl_event_loop,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
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
    pub layers: C2Rust_Unnamed_99,
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
    pub session_lock: C2Rust_Unnamed_79,
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
    pub new_object: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_text_input_manager_v3 {
    pub global: *mut wl_global,
    pub text_inputs: wl_list,
    pub events: C2Rust_Unnamed_76,
    pub WLR_PRIVATE: C2Rust_Unnamed_75,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_75 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_76 {
    pub new_text_input: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_manager_v2 {
    pub global: *mut wl_global,
    pub input_methods: wl_list,
    pub events: C2Rust_Unnamed_78,
    pub WLR_PRIVATE: C2Rust_Unnamed_77,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_77 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_78 {
    pub new_input_method: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_79 {
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
    pub events: C2Rust_Unnamed_81,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_80,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_80 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_81 {
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
    pub events: C2Rust_Unnamed_84,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_82,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_82 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_83,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_83 {
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
pub struct C2Rust_Unnamed_84 {
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
    pub events: C2Rust_Unnamed_86,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_85,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_85 {
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_86 {
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
    pub events: C2Rust_Unnamed_88,
    pub data: *mut ::core::ffi::c_void,
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
    pub events: C2Rust_Unnamed_90,
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
    pub events: C2Rust_Unnamed_92,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_91,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_91 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_92 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_pointers: wl_list,
    pub events: C2Rust_Unnamed_94,
    pub WLR_PRIVATE: C2Rust_Unnamed_93,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_93 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_94 {
    pub new_virtual_pointer: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_keyboards: wl_list,
    pub events: C2Rust_Unnamed_96,
    pub WLR_PRIVATE: C2Rust_Unnamed_95,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_95 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_96 {
    pub new_virtual_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_98,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_97,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_97 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_98 {
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_99 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_101,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_100,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_100 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_101 {
    pub sway_root: *mut sway_root,
    pub sway_output: *mut sway_output,
    pub sway_workspace: *mut sway_workspace,
    pub sway_container: *mut sway_container,
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
    pub events: C2Rust_Unnamed_102,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_102 {
    pub new_node: wl_signal,
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
pub struct C2Rust_Unnamed_104 {
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
pub struct C2Rust_Unnamed_105 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_106 {
    pub tree: *mut wlr_scene_tree,
    pub border: *mut wlr_scene_tree,
    pub background: *mut wlr_scene_tree,
    pub title_text: *mut sway_text_node,
    pub marks_text: *mut sway_text_node,
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct zwp_pointer_constraints_v1_lifetime(pub ::core::ffi::c_uint);
impl zwp_pointer_constraints_v1_lifetime {
    pub const ZWP_POINTER_CONSTRAINTS_V1_LIFETIME_ONESHOT: Self = Self(1);
    pub const ZWP_POINTER_CONSTRAINTS_V1_LIFETIME_PERSISTENT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_pointer_constraint_v1_type(pub ::core::ffi::c_uint);
impl wlr_pointer_constraint_v1_type {
    pub const WLR_POINTER_CONSTRAINT_V1_LOCKED: Self = Self(0);
    pub const WLR_POINTER_CONSTRAINT_V1_CONFINED: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_constraint_v1_state {
    pub committed: uint32_t,
    pub region: pixman_region32_t,
    pub cursor_hint: C2Rust_Unnamed_107,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_107 {
    pub enabled: bool,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
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
    pub events: C2Rust_Unnamed_109,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_108,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_108 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub synced: wlr_surface_synced,
    pub destroying: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_109 {
    pub set_region: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_touch {
    pub base: wlr_input_device,
    pub r#impl: *const wlr_touch_impl,
    pub output_name: *mut ::core::ffi::c_char,
    pub width_mm: ::core::ffi::c_double,
    pub height_mm: ::core::ffi::c_double,
    pub events: C2Rust_Unnamed_110,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_110 {
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
pub struct swaynag_instance {
    pub client: *mut wl_client,
    pub client_destroy: wl_listener,
    pub args: *const ::core::ffi::c_char,
    pub fd: [::core::ffi::c_int; 2],
    pub detailed: bool,
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
    pub events: C2Rust_Unnamed_112,
    pub WLR_PRIVATE: C2Rust_Unnamed_111,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_111 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_112 {
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
    pub events: C2Rust_Unnamed_114,
    pub WLR_PRIVATE: C2Rust_Unnamed_113,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_113 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_114 {
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
pub struct sway_cursor {
    pub seat: *mut sway_seat,
    pub cursor: *mut wlr_cursor,
    pub previous: C2Rust_Unnamed_115,
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
pub struct C2Rust_Unnamed_115 {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub node: *mut sway_node,
}
pub type PangoFontDescription = _PangoFontDescription;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct binding_input_type(pub ::core::ffi::c_uint);
impl binding_input_type {
    pub const BINDING_KEYCODE: Self = Self(0);
    pub const BINDING_KEYSYM: Self = Self(1);
    pub const BINDING_MOUSECODE: Self = Self(2);
    pub const BINDING_MOUSESYM: Self = Self(3);
    pub const BINDING_SWITCH: Self = Self(4);
    pub const BINDING_GESTURE: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct binding_flags(pub ::core::ffi::c_uint);
impl binding_flags {
    pub const BINDING_RELEASE: Self = Self(1);
    pub const BINDING_LOCKED: Self = Self(2);
    pub const BINDING_BORDER: Self = Self(4);
    pub const BINDING_CONTENTS: Self = Self(8);
    pub const BINDING_TITLEBAR: Self = Self(16);
    pub const BINDING_CODE: Self = Self(32);
    pub const BINDING_RELOAD: Self = Self(64);
    pub const BINDING_INHIBITED: Self = Self(128);
    pub const BINDING_NOREPEAT: Self = Self(256);
    pub const BINDING_EXACT: Self = Self(512);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_binding {
    pub r#type: binding_input_type,
    pub order: ::core::ffi::c_int,
    pub input: *mut ::core::ffi::c_char,
    pub flags: uint32_t,
    pub keys: *mut list_t,
    pub syms: *mut list_t,
    pub modifiers: uint32_t,
    pub group: xkb_layout_index_t,
    pub command: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_gesture_binding {
    pub input: *mut ::core::ffi::c_char,
    pub flags: uint32_t,
    pub gesture: gesture,
    pub command: *mut ::core::ffi::c_char,
}
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
    pub xcursor_theme: C2Rust_Unnamed_116,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_116 {
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
    pub colors: C2Rust_Unnamed_117,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_117 {
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
    pub border_colors: C2Rust_Unnamed_120,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_118,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_118 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_119,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_119 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_120 {
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
pub struct seatop_default_event {
    pub previous_node: *mut sway_node,
    pub pressed_buttons: [uint32_t; 32],
    pub pressed_button_count: size_t,
    pub gestures: gesture_tracker,
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
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const WLR_POINTER_AXIS_DISCRETE_STEP: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
pub const SWAY_CURSOR_PRESSED_BUTTONS_CAP: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const SWAY_SCROLL_UP: ::core::ffi::c_int = KEY_MAX + 1 as ::core::ffi::c_int;
pub const SWAY_SCROLL_DOWN: ::core::ffi::c_int = KEY_MAX + 2 as ::core::ffi::c_int;
pub const SWAY_SCROLL_LEFT: ::core::ffi::c_int = KEY_MAX + 3 as ::core::ffi::c_int;
pub const SWAY_SCROLL_RIGHT: ::core::ffi::c_int = KEY_MAX + 4 as ::core::ffi::c_int;
unsafe extern "C" fn edge_is_external(
    mut cont: *mut sway_container,
    mut edge: wlr_edges,
) -> bool {
    let mut layout: sway_container_layout = sway_container_layout(
        sway_container_layout_0::L_NONE.0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    match edge {
        wlr_edges::WLR_EDGE_TOP | wlr_edges::WLR_EDGE_BOTTOM => {
            layout = sway_container_layout(
                sway_container_layout_0::L_VERT.0 as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        }
        wlr_edges::WLR_EDGE_LEFT | wlr_edges::WLR_EDGE_RIGHT => {
            layout = sway_container_layout(
                sway_container_layout_0::L_HORIZ.0 as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        }
        wlr_edges::WLR_EDGE_NONE => {
            _sway_assert(
                false,
                b"[%s:%d] %s:Never reached\0".as_ptr() as *const ::core::ffi::c_char,
                b"sway/input/seatop_default.c\0".as_ptr() as *const ::core::ffi::c_char,
                51 as ::core::ffi::c_int,
                b"_Bool edge_is_external(struct sway_container *, enum wlr_edges)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        _ => {}
    }
    while !cont.is_null() {
        if container_parent_layout(cont).0 == layout.0 {
            let mut siblings: *mut list_t = container_get_siblings(cont);
            if siblings.is_null() {
                return r#false != 0;
            }
            let mut index: ::core::ffi::c_int = list_find(
                siblings,
                cont as *const ::core::ffi::c_void,
            );
            if index > 0 as ::core::ffi::c_int
                && (edge.0 == wlr_edges::WLR_EDGE_LEFT.0
                    || edge.0 == wlr_edges::WLR_EDGE_TOP.0)
            {
                return r#false != 0;
            }
            if index < (*siblings).length - 1 as ::core::ffi::c_int
                && (edge.0 == wlr_edges::WLR_EDGE_RIGHT.0
                    || edge.0 == wlr_edges::WLR_EDGE_BOTTOM.0)
            {
                return r#false != 0;
            }
        }
        cont = (*cont).pending.parent;
    }
    return r#true != 0;
}
unsafe extern "C" fn find_edge(
    mut cont: *mut sway_container,
    mut surface: *mut wlr_surface,
    mut cursor: *mut sway_cursor,
) -> wlr_edges {
    if (*cont).view.is_null() || !surface.is_null() && (*(*cont).view).surface != surface
    {
        return wlr_edges::WLR_EDGE_NONE;
    }
    if (*cont).pending.border.0 == sway_container_border::B_NONE.0
        || (*cont).pending.border_thickness == 0
        || (*cont).pending.border.0 == sway_container_border::B_CSD.0
    {
        return wlr_edges::WLR_EDGE_NONE;
    }
    if (*cont).pending.fullscreen_mode.0 != 0 {
        return wlr_edges::WLR_EDGE_NONE;
    }
    let mut edge: wlr_edges = wlr_edges(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    if (*(*cursor).cursor).x
        < (*cont).pending.x + (*cont).pending.border_thickness as ::core::ffi::c_double
    {
        edge = wlr_edges(edge.0 | wlr_edges::WLR_EDGE_LEFT.0);
    }
    if (*(*cursor).cursor).y
        < (*cont).pending.y + (*cont).pending.border_thickness as ::core::ffi::c_double
    {
        edge = wlr_edges(edge.0 | wlr_edges::WLR_EDGE_TOP.0);
    }
    if (*(*cursor).cursor).x
        >= (*cont).pending.x + (*cont).pending.width
            - (*cont).pending.border_thickness as ::core::ffi::c_double
    {
        edge = wlr_edges(edge.0 | wlr_edges::WLR_EDGE_RIGHT.0);
    }
    if (*(*cursor).cursor).y
        >= (*cont).pending.y + (*cont).pending.height
            - (*cont).pending.border_thickness as ::core::ffi::c_double
    {
        edge = wlr_edges(edge.0 | wlr_edges::WLR_EDGE_BOTTOM.0);
    }
    return edge;
}
#[no_mangle]
pub unsafe extern "C" fn find_resize_edge(
    mut cont: *mut sway_container,
    mut surface: *mut wlr_surface,
    mut cursor: *mut sway_cursor,
) -> wlr_edges {
    let mut edge: wlr_edges = find_edge(cont, surface, cursor);
    if edge.0 != 0 && !container_is_floating(cont)
        && edge_is_external(cont, edge) as ::core::ffi::c_int != 0
    {
        return wlr_edges::WLR_EDGE_NONE;
    }
    return edge;
}
unsafe extern "C" fn get_active_mouse_binding(
    mut e: *mut seatop_default_event,
    mut bindings: *mut list_t,
    mut modifiers: uint32_t,
    mut release: bool,
    mut on_titlebar: bool,
    mut on_border: bool,
    mut on_content: bool,
    mut on_workspace: bool,
    mut identifier: *const ::core::ffi::c_char,
) -> *mut sway_binding {
    let mut click_region: uint32_t = (if on_titlebar as ::core::ffi::c_int != 0
        || on_workspace as ::core::ffi::c_int != 0
    {
        binding_flags::BINDING_TITLEBAR.0 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }
        | if on_border as ::core::ffi::c_int != 0
            || on_workspace as ::core::ffi::c_int != 0
        {
            binding_flags::BINDING_BORDER.0 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }
        | if on_content as ::core::ffi::c_int != 0
            || on_workspace as ::core::ffi::c_int != 0
        {
            binding_flags::BINDING_CONTENTS.0 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t;
    let mut current: *mut sway_binding = ::core::ptr::null_mut::<sway_binding>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*bindings).length {
        let mut binding: *mut sway_binding = *(*bindings).items.offset(i as isize)
            as *mut sway_binding;
        if !(modifiers ^ (*binding).modifiers != 0
            || (*e).pressed_button_count != (*(*binding).keys).length as size_t
            || release as uint32_t
                != (*binding).flags
                    & binding_flags::BINDING_RELEASE.0 as ::core::ffi::c_int as uint32_t
            || click_region & (*binding).flags == 0
            || on_workspace as ::core::ffi::c_int != 0
                && click_region & (*binding).flags != click_region
            || strcmp((*binding).input, identifier) != 0 as ::core::ffi::c_int
                && strcmp(
                    (*binding).input,
                    b"*\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int)
        {
            let mut r#match: bool = r#true != 0;
            let mut j: size_t = 0 as size_t;
            while j < (*e).pressed_button_count {
                let mut key: uint32_t = *(*(*(*binding).keys).items.offset(j as isize)
                    as *mut uint32_t);
                if key != (*e).pressed_buttons[j] {
                    r#match = r#false != 0;
                    break;
                } else {
                    j = j.wrapping_add(1);
                }
            }
            if r#match {
                if current.is_null()
                    || strcmp(
                        (*current).input,
                        b"*\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    current = binding;
                    if strcmp((*current).input, identifier) == 0 as ::core::ffi::c_int {
                        break;
                    }
                }
            }
        }
        i += 1;
    }
    return current;
}
unsafe extern "C" fn state_erase_button(
    mut e: *mut seatop_default_event,
    mut button: uint32_t,
) {
    let mut j: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < (*e).pressed_button_count {
        if i > j {
            (*e).pressed_buttons[j] = (*e).pressed_buttons[i];
        }
        if (*e).pressed_buttons[i] != button {
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    while (*e).pressed_button_count > j {
        (*e).pressed_button_count = (*e).pressed_button_count.wrapping_sub(1);
        (*e).pressed_buttons[(*e).pressed_button_count] = 0 as uint32_t;
    }
}
unsafe extern "C" fn state_add_button(
    mut e: *mut seatop_default_event,
    mut button: uint32_t,
) {
    if (*e).pressed_button_count >= SWAY_CURSOR_PRESSED_BUTTONS_CAP as size_t {
        return;
    }
    let mut i: size_t = 0 as size_t;
    while i < (*e).pressed_button_count && (*e).pressed_buttons[i] < button {
        i = i.wrapping_add(1);
    }
    let mut j: size_t = (*e).pressed_button_count;
    while j > i {
        (*e).pressed_buttons[j] = (*e).pressed_buttons[j.wrapping_sub(1 as size_t)];
        j = j.wrapping_sub(1);
    }
    (*e).pressed_buttons[i] = button;
    (*e).pressed_button_count = (*e).pressed_button_count.wrapping_add(1);
}
unsafe extern "C" fn handle_tablet_tool_tip(
    mut seat: *mut sway_seat,
    mut tool: *mut sway_tablet_tool,
    mut time_msec: uint32_t,
    mut state: wlr_tablet_tool_tip_state,
) {
    if state.0 == wlr_tablet_tool_tip_state::WLR_TABLET_TOOL_TIP_UP.0 {
        wlr_tablet_v2_tablet_tool_notify_up((*tool).tablet_v2_tool);
        return;
    }
    let mut cursor: *mut sway_cursor = (*seat).cursor;
    let mut surface: *mut wlr_surface = ::core::ptr::null_mut::<wlr_surface>();
    let mut sx: ::core::ffi::c_double = 0.;
    let mut sy: ::core::ffi::c_double = 0.;
    let mut node: *mut sway_node = node_at_coords(
        seat,
        (*(*cursor).cursor).x,
        (*(*cursor).cursor).y,
        &raw mut surface,
        &raw mut sx,
        &raw mut sy,
    );
    if !_sway_assert(
        !surface.is_null(),
        b"[%s:%d] %s:Expected null-surface tablet input to route through pointer emulation\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/seatop_default.c\0".as_ptr() as *const ::core::ffi::c_char,
        230 as ::core::ffi::c_int,
        b"void handle_tablet_tool_tip(struct sway_seat *, struct sway_tablet_tool *, uint32_t, enum wlr_tablet_tool_tip_state)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    let mut cont: *mut sway_container = if !node.is_null()
        && (*node).r#type.0 == sway_node_type::N_CONTAINER.0
    {
        (*node).c2rust_unnamed.sway_container
    } else {
        ::core::ptr::null_mut::<sway_container>()
    };
    let mut layer: *mut wlr_layer_surface_v1 = ::core::ptr::null_mut::<
        wlr_layer_surface_v1,
    >();
    let mut xsurface: *mut wlr_xwayland_surface = ::core::ptr::null_mut::<
        wlr_xwayland_surface,
    >();
    layer = wlr_layer_surface_v1_try_from_wlr_surface(surface);
    if !layer.is_null() && (*layer).current.keyboard_interactive.0 != 0 {
        seat_set_focus_layer(seat, layer);
        transaction_commit_dirty();
    } else if !cont.is_null() {
        let mut is_floating_or_child: bool = container_is_floating_or_child(cont);
        let mut is_fullscreen_or_child: bool = container_is_fullscreen_or_child(cont);
        let mut keyboard: *mut wlr_keyboard = wlr_seat_get_keyboard((*seat).wlr_seat);
        let mut mod_pressed: bool = !keyboard.is_null()
            && wlr_keyboard_get_modifiers(keyboard) & (*config).floating_mod != 0;
        if is_floating_or_child as ::core::ffi::c_int != 0 && !is_fullscreen_or_child
            && mod_pressed as ::core::ffi::c_int != 0
        {
            seat_set_focus_container(
                seat,
                seat_get_focus_inactive_view(seat, &raw mut (*cont).node),
            );
            seatop_begin_move_floating(seat, container_toplevel_ancestor(cont));
            return;
        }
        if (*config).tiling_drag as ::core::ffi::c_int != 0
            && mod_pressed as ::core::ffi::c_int != 0 && !is_floating_or_child
            && (*cont).pending.fullscreen_mode.0
                == sway_fullscreen_mode::FULLSCREEN_NONE.0
        {
            seatop_begin_move_tiling(seat, cont);
            return;
        }
        seat_set_focus_container(seat, cont);
        seatop_begin_down(seat, (*node).c2rust_unnamed.sway_container, sx, sy);
    } else {
        xsurface = wlr_xwayland_surface_try_from_wlr_surface(surface);
        if !xsurface.is_null()
            && (*xsurface).override_redirect as ::core::ffi::c_int != 0
            && wlr_xwayland_surface_override_redirect_wants_focus(xsurface)
                as ::core::ffi::c_int != 0
        {
            let mut xwayland: *mut wlr_xwayland = server.xwayland.wlr_xwayland;
            wlr_xwayland_set_seat(xwayland, (*seat).wlr_seat);
            seat_set_focus_surface(seat, (*xsurface).surface, r#false != 0);
            transaction_commit_dirty();
        }
    }
    wlr_tablet_v2_tablet_tool_notify_down((*tool).tablet_v2_tool);
    wlr_tablet_tool_v2_start_implicit_grab((*tool).tablet_v2_tool);
}
unsafe extern "C" fn trigger_pointer_button_binding(
    mut seat: *mut sway_seat,
    mut device: *mut wlr_input_device,
    mut button: uint32_t,
    mut state: wl_pointer_button_state,
    mut modifiers: uint32_t,
    mut on_titlebar: bool,
    mut on_border: bool,
    mut on_contents: bool,
    mut on_workspace: bool,
) -> bool {
    if !device.is_null()
        && (*device).r#type.0 != wlr_input_device_type::WLR_INPUT_DEVICE_POINTER.0
    {
        return r#false != 0;
    }
    let mut e: *mut seatop_default_event = (*seat).seatop_data
        as *mut seatop_default_event;
    let mut device_identifier: *mut ::core::ffi::c_char = if !device.is_null() {
        input_device_get_identifier(device)
    } else {
        strdup(b"*\0".as_ptr() as *const ::core::ffi::c_char)
    };
    let mut binding: *mut sway_binding = ::core::ptr::null_mut::<sway_binding>();
    if state.0 == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0 {
        state_add_button(e, button);
        binding = get_active_mouse_binding(
            e,
            (*(*config).current_mode).mouse_bindings,
            modifiers,
            r#false != 0,
            on_titlebar,
            on_border,
            on_contents,
            on_workspace,
            device_identifier,
        );
    } else {
        binding = get_active_mouse_binding(
            e,
            (*(*config).current_mode).mouse_bindings,
            modifiers,
            r#true != 0,
            on_titlebar,
            on_border,
            on_contents,
            on_workspace,
            device_identifier,
        );
        state_erase_button(e, button);
    }
    free(device_identifier as *mut ::core::ffi::c_void);
    if !binding.is_null() {
        seat_execute_command(seat, binding);
        return r#true != 0;
    }
    return r#false != 0;
}
unsafe extern "C" fn handle_button(
    mut seat: *mut sway_seat,
    mut time_msec: uint32_t,
    mut device: *mut wlr_input_device,
    mut button: uint32_t,
    mut state: wl_pointer_button_state,
) {
    let mut cursor: *mut sway_cursor = (*seat).cursor;
    let mut surface: *mut wlr_surface = ::core::ptr::null_mut::<wlr_surface>();
    let mut sx: ::core::ffi::c_double = 0.;
    let mut sy: ::core::ffi::c_double = 0.;
    let mut node: *mut sway_node = node_at_coords(
        seat,
        (*(*cursor).cursor).x,
        (*(*cursor).cursor).y,
        &raw mut surface,
        &raw mut sx,
        &raw mut sy,
    );
    let mut cont: *mut sway_container = if !node.is_null()
        && (*node).r#type.0 == sway_node_type::N_CONTAINER.0
    {
        (*node).c2rust_unnamed.sway_container
    } else {
        ::core::ptr::null_mut::<sway_container>()
    };
    let mut is_floating: bool = !cont.is_null()
        && container_is_floating(cont) as ::core::ffi::c_int != 0;
    let mut is_floating_or_child: bool = !cont.is_null()
        && container_is_floating_or_child(cont) as ::core::ffi::c_int != 0;
    let mut is_fullscreen_or_child: bool = !cont.is_null()
        && container_is_fullscreen_or_child(cont) as ::core::ffi::c_int != 0;
    let mut edge: wlr_edges = wlr_edges(
        if !cont.is_null() {
            find_edge(cont, surface, cursor).0
        } else {
            wlr_edges::WLR_EDGE_NONE.0
        },
    );
    let mut resize_edge: wlr_edges = wlr_edges(
        if !cont.is_null() && edge.0 != 0 {
            find_resize_edge(cont, surface, cursor).0
        } else {
            wlr_edges::WLR_EDGE_NONE.0
        },
    );
    let mut on_border: bool = edge.0 != wlr_edges::WLR_EDGE_NONE.0;
    let mut on_contents: bool = !cont.is_null() && !on_border && !surface.is_null();
    let mut on_workspace: bool = !node.is_null()
        && (*node).r#type.0 == sway_node_type::N_WORKSPACE.0;
    let mut on_titlebar: bool = !cont.is_null() && !on_border && surface.is_null();
    let mut keyboard: *mut wlr_keyboard = wlr_seat_get_keyboard((*seat).wlr_seat);
    let mut modifiers: uint32_t = if !keyboard.is_null() {
        wlr_keyboard_get_modifiers(keyboard)
    } else {
        0 as uint32_t
    };
    let mut mod_pressed: bool = modifiers & (*config).floating_mod != 0;
    let mut mod_move_btn: uint32_t = (if (*config).floating_mod_inverse
        as ::core::ffi::c_int != 0
    {
        BTN_RIGHT
    } else {
        BTN_LEFT
    }) as uint32_t;
    let mut mod_resize_btn: uint32_t = (if (*config).floating_mod_inverse
        as ::core::ffi::c_int != 0
    {
        BTN_LEFT
    } else {
        BTN_RIGHT
    }) as uint32_t;
    let mut mod_move_btn_pressed: bool = mod_pressed as ::core::ffi::c_int != 0
        && button == mod_move_btn;
    let mut mod_resize_btn_pressed: bool = mod_pressed as ::core::ffi::c_int != 0
        && button == mod_resize_btn;
    let mut titlebar_left_btn_pressed: bool = on_titlebar as ::core::ffi::c_int != 0
        && button == BTN_LEFT as uint32_t;
    if trigger_pointer_button_binding(
        seat,
        device,
        button,
        state,
        modifiers,
        on_titlebar,
        on_border,
        on_contents,
        on_workspace,
    ) {
        return;
    }
    if !node.is_null() && (*node).r#type.0 == sway_node_type::N_WORKSPACE.0
        && surface.is_null()
    {
        if state.0 == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0 {
            seat_set_focus(seat, node);
            transaction_commit_dirty();
        }
        seat_pointer_notify_button(seat, time_msec, button, state);
        return;
    }
    let mut layer: *mut wlr_layer_surface_v1 = ::core::ptr::null_mut::<
        wlr_layer_surface_v1,
    >();
    layer = toplevel_layer_surface_from_surface(surface);
    if !layer.is_null() {
        if (*layer).current.keyboard_interactive.0 != 0 {
            seat_set_focus_layer(seat, layer);
            transaction_commit_dirty();
        }
        if state.0 == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0 {
            seatop_begin_down_on_surface(seat, surface, sx, sy);
        }
        seat_pointer_notify_button(seat, time_msec, button, state);
        return;
    }
    if !cont.is_null() && resize_edge.0 != 0 && button == BTN_LEFT as uint32_t
        && state.0 == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0
        && !is_floating
    {
        let mut cont_to_focus: *mut sway_container = cont;
        let mut layout: sway_container_layout = container_parent_layout(cont);
        if layout.0 == sway_container_layout_0::L_TABBED.0
            || layout.0 == sway_container_layout_0::L_STACKED.0
        {
            cont_to_focus = seat_get_focus_inactive_view(
                seat,
                &raw mut (*(*cont).pending.parent).node,
            );
        }
        seat_set_focus_container(seat, cont_to_focus);
        seatop_begin_resize_tiling(seat, cont, edge);
        return;
    }
    if !cont.is_null() && !is_floating_or_child && mod_pressed as ::core::ffi::c_int != 0
        && mod_resize_btn_pressed as ::core::ffi::c_int != 0
        && state.0 == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0
    {
        edge = wlr_edges(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
        edge = wlr_edges(
            edge.0
                | (if (*(*cursor).cursor).x
                    > (*cont).pending.x
                        + (*cont).pending.width
                            / 2 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    wlr_edges::WLR_EDGE_RIGHT.0 as ::core::ffi::c_int
                } else {
                    wlr_edges::WLR_EDGE_LEFT.0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
        );
        edge = wlr_edges(
            edge.0
                | (if (*(*cursor).cursor).y
                    > (*cont).pending.y
                        + (*cont).pending.height
                            / 2 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    wlr_edges::WLR_EDGE_BOTTOM.0 as ::core::ffi::c_int
                } else {
                    wlr_edges::WLR_EDGE_TOP.0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
        );
        let mut image: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if edge.0
            == (wlr_edges::WLR_EDGE_LEFT.0 as ::core::ffi::c_int
                | wlr_edges::WLR_EDGE_TOP.0 as ::core::ffi::c_int) as ::core::ffi::c_uint
        {
            image = b"nw-resize\0".as_ptr() as *const ::core::ffi::c_char;
        } else if edge.0
            == (wlr_edges::WLR_EDGE_TOP.0 as ::core::ffi::c_int
                | wlr_edges::WLR_EDGE_RIGHT.0 as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
            image = b"ne-resize\0".as_ptr() as *const ::core::ffi::c_char;
        } else if edge.0
            == (wlr_edges::WLR_EDGE_RIGHT.0 as ::core::ffi::c_int
                | wlr_edges::WLR_EDGE_BOTTOM.0 as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
            image = b"se-resize\0".as_ptr() as *const ::core::ffi::c_char;
        } else if edge.0
            == (wlr_edges::WLR_EDGE_BOTTOM.0 as ::core::ffi::c_int
                | wlr_edges::WLR_EDGE_LEFT.0 as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
            image = b"sw-resize\0".as_ptr() as *const ::core::ffi::c_char;
        }
        cursor_set_image((*seat).cursor, image, ::core::ptr::null_mut::<wl_client>());
        seat_set_focus_container(seat, cont);
        seatop_begin_resize_tiling(seat, cont, edge);
        return;
    }
    if !cont.is_null()
        && state.0 == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0
    {
        node = &raw mut (*cont).node;
        if on_titlebar {
            let mut focus: *mut sway_container = seat_get_focused_container(seat);
            if focus == cont || !container_has_ancestor(focus, cont) {
                node = seat_get_focus_inactive(seat, &raw mut (*cont).node);
            }
        }
        seat_set_focus(seat, node);
        transaction_commit_dirty();
    }
    if !cont.is_null() && is_floating_or_child as ::core::ffi::c_int != 0
        && !is_fullscreen_or_child
        && state.0 == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0
        && (mod_move_btn_pressed as ::core::ffi::c_int != 0
            || titlebar_left_btn_pressed as ::core::ffi::c_int != 0)
    {
        seatop_begin_move_floating(seat, container_toplevel_ancestor(cont));
        return;
    }
    if !cont.is_null() && is_floating_or_child as ::core::ffi::c_int != 0
        && !is_fullscreen_or_child
        && state.0 == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0
    {
        if button == BTN_LEFT as uint32_t && resize_edge.0 != wlr_edges::WLR_EDGE_NONE.0
        {
            seat_set_focus_container(seat, cont);
            seatop_begin_resize_floating(seat, cont, resize_edge);
            return;
        }
        if mod_resize_btn_pressed {
            let mut floater: *mut sway_container = container_toplevel_ancestor(cont);
            edge = wlr_edges(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
            edge = wlr_edges(
                edge.0
                    | (if (*(*cursor).cursor).x
                        > (*floater).pending.x
                            + (*floater).pending.width
                                / 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        wlr_edges::WLR_EDGE_RIGHT.0 as ::core::ffi::c_int
                    } else {
                        wlr_edges::WLR_EDGE_LEFT.0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
            );
            edge = wlr_edges(
                edge.0
                    | (if (*(*cursor).cursor).y
                        > (*floater).pending.y
                            + (*floater).pending.height
                                / 2 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        wlr_edges::WLR_EDGE_BOTTOM.0 as ::core::ffi::c_int
                    } else {
                        wlr_edges::WLR_EDGE_TOP.0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
            );
            seat_set_focus_container(seat, floater);
            seatop_begin_resize_floating(seat, floater, edge);
            return;
        }
    }
    if (*config).tiling_drag as ::core::ffi::c_int != 0
        && (mod_move_btn_pressed as ::core::ffi::c_int != 0
            || titlebar_left_btn_pressed as ::core::ffi::c_int != 0)
        && state.0 == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0
        && !is_floating_or_child && !cont.is_null()
        && (*cont).pending.fullscreen_mode.0 == sway_fullscreen_mode::FULLSCREEN_NONE.0
    {
        if !mod_pressed && (*config).tiling_drag_threshold > 0 as ::core::ffi::c_int {
            seatop_begin_move_tiling_threshold(seat, cont);
        } else {
            seatop_begin_move_tiling(seat, cont);
        }
        return;
    }
    if !surface.is_null() && !cont.is_null()
        && state.0 == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0
    {
        seatop_begin_down(seat, cont, sx, sy);
        seat_pointer_notify_button(
            seat,
            time_msec,
            button,
            wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED,
        );
        return;
    }
    if !cont.is_null()
        && state.0 == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0
    {
        seat_pointer_notify_button(seat, time_msec, button, state);
        return;
    }
    let mut xsurface: *mut wlr_xwayland_surface = ::core::ptr::null_mut::<
        wlr_xwayland_surface,
    >();
    if !surface.is_null()
        && {
            xsurface = wlr_xwayland_surface_try_from_wlr_surface(surface);
            !xsurface.is_null()
        } && (*xsurface).override_redirect as ::core::ffi::c_int != 0
        && wlr_xwayland_surface_override_redirect_wants_focus(xsurface)
            as ::core::ffi::c_int != 0
    {
        let mut xwayland: *mut wlr_xwayland = server.xwayland.wlr_xwayland;
        wlr_xwayland_set_seat(xwayland, (*seat).wlr_seat);
        seat_set_focus_surface(seat, (*xsurface).surface, r#false != 0);
        transaction_commit_dirty();
        seat_pointer_notify_button(seat, time_msec, button, state);
    }
    seat_pointer_notify_button(seat, time_msec, button, state);
}
unsafe extern "C" fn check_focus_follows_mouse(
    mut seat: *mut sway_seat,
    mut e: *mut seatop_default_event,
    mut hovered_node: *mut sway_node,
) {
    let mut focus: *mut sway_node = seat_get_focus(seat);
    if hovered_node.is_null() {
        let mut wlr_output: *mut wlr_output = wlr_output_layout_output_at(
            (*root).output_layout,
            (*(*(*seat).cursor).cursor).x,
            (*(*(*seat).cursor).cursor).y,
        );
        if wlr_output.is_null() {
            return;
        }
        let mut surface: *mut wlr_surface = ::core::ptr::null_mut::<wlr_surface>();
        let mut sx: ::core::ffi::c_double = 0.;
        let mut sy: ::core::ffi::c_double = 0.;
        node_at_coords(
            seat,
            (*(*(*seat).cursor).cursor).x,
            (*(*(*seat).cursor).cursor).y,
            &raw mut surface,
            &raw mut sx,
            &raw mut sy,
        );
        let mut layer: *mut wlr_layer_surface_v1 = ::core::ptr::null_mut::<
            wlr_layer_surface_v1,
        >();
        layer = toplevel_layer_surface_from_surface(surface);
        if !layer.is_null() && (*layer).current.keyboard_interactive.0 != 0 {
            seat_set_focus_layer(seat, layer);
            transaction_commit_dirty();
            return;
        }
        let mut hovered_output: *mut sway_output = (*wlr_output).data
            as *mut sway_output;
        if !focus.is_null() && hovered_output != node_get_output(focus) {
            let mut ws: *mut sway_workspace = output_get_active_workspace(
                hovered_output,
            );
            seat_set_focus(seat, &raw mut (*ws).node);
            transaction_commit_dirty();
        }
        return;
    }
    if !focus.is_null() && (*hovered_node).r#type.0 == sway_node_type::N_WORKSPACE.0 {
        let mut focused_output: *mut sway_output = node_get_output(focus);
        let mut hovered_output_0: *mut sway_output = node_get_output(hovered_node);
        if hovered_output_0 != focused_output {
            seat_set_focus(seat, seat_get_focus_inactive(seat, hovered_node));
            transaction_commit_dirty();
        }
        return;
    }
    if node_is_view(hovered_node) as ::core::ffi::c_int != 0
        && view_is_visible((*(*hovered_node).c2rust_unnamed.sway_container).view)
            as ::core::ffi::c_int != 0
    {
        if hovered_node != (*e).previous_node
            || (*config).focus_follows_mouse.0
                == focus_follows_mouse_mode::FOLLOWS_ALWAYS.0
        {
            seat_set_focus(seat, hovered_node);
            transaction_commit_dirty();
        }
    }
}
unsafe extern "C" fn handle_pointer_motion(
    mut seat: *mut sway_seat,
    mut time_msec: uint32_t,
) {
    let mut e: *mut seatop_default_event = (*seat).seatop_data
        as *mut seatop_default_event;
    let mut cursor: *mut sway_cursor = (*seat).cursor;
    let mut surface: *mut wlr_surface = ::core::ptr::null_mut::<wlr_surface>();
    let mut sx: ::core::ffi::c_double = 0.;
    let mut sy: ::core::ffi::c_double = 0.;
    let mut node: *mut sway_node = node_at_coords(
        seat,
        (*(*cursor).cursor).x,
        (*(*cursor).cursor).y,
        &raw mut surface,
        &raw mut sx,
        &raw mut sy,
    );
    if (*config).focus_follows_mouse.0 != focus_follows_mouse_mode::FOLLOWS_NO.0 {
        check_focus_follows_mouse(seat, e, node);
    }
    if !surface.is_null() {
        if seat_is_input_allowed(seat, surface) {
            wlr_seat_pointer_notify_enter((*seat).wlr_seat, surface, sx, sy);
            wlr_seat_pointer_notify_motion((*seat).wlr_seat, time_msec, sx, sy);
        }
    } else {
        cursor_update_image(cursor, node);
        wlr_seat_pointer_notify_clear_focus((*seat).wlr_seat);
    }
    drag_icons_update_position(seat);
    (*e).previous_node = node;
}
unsafe extern "C" fn handle_tablet_tool_motion(
    mut seat: *mut sway_seat,
    mut tool: *mut sway_tablet_tool,
    mut time_msec: uint32_t,
) {
    let mut e: *mut seatop_default_event = (*seat).seatop_data
        as *mut seatop_default_event;
    let mut cursor: *mut sway_cursor = (*seat).cursor;
    let mut surface: *mut wlr_surface = ::core::ptr::null_mut::<wlr_surface>();
    let mut sx: ::core::ffi::c_double = 0.;
    let mut sy: ::core::ffi::c_double = 0.;
    let mut node: *mut sway_node = node_at_coords(
        seat,
        (*(*cursor).cursor).x,
        (*(*cursor).cursor).y,
        &raw mut surface,
        &raw mut sx,
        &raw mut sy,
    );
    if (*config).focus_follows_mouse.0 != focus_follows_mouse_mode::FOLLOWS_NO.0 {
        check_focus_follows_mouse(seat, e, node);
    }
    if !surface.is_null() {
        if seat_is_input_allowed(seat, surface) {
            wlr_tablet_v2_tablet_tool_notify_proximity_in(
                (*tool).tablet_v2_tool,
                (*(*tool).tablet).tablet_v2,
                surface,
            );
            wlr_tablet_v2_tablet_tool_notify_motion((*tool).tablet_v2_tool, sx, sy);
        }
    } else {
        cursor_update_image(cursor, node);
        wlr_tablet_v2_tablet_tool_notify_proximity_out((*tool).tablet_v2_tool);
    }
    drag_icons_update_position(seat);
    (*e).previous_node = node;
}
unsafe extern "C" fn handle_touch_down(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_touch_down_event,
    mut lx: ::core::ffi::c_double,
    mut ly: ::core::ffi::c_double,
) {
    let mut surface: *mut wlr_surface = ::core::ptr::null_mut::<wlr_surface>();
    let mut wlr_seat: *mut wlr_seat = (*seat).wlr_seat;
    let mut cursor: *mut sway_cursor = (*seat).cursor;
    let mut sx: ::core::ffi::c_double = 0.;
    let mut sy: ::core::ffi::c_double = 0.;
    node_at_coords(
        seat,
        (*seat).touch_x,
        (*seat).touch_y,
        &raw mut surface,
        &raw mut sx,
        &raw mut sy,
    );
    if !surface.is_null()
        && wlr_surface_accepts_touch(surface, wlr_seat) as ::core::ffi::c_int != 0
    {
        if seat_is_input_allowed(seat, surface) {
            (*cursor).simulating_pointer_from_touch = r#false != 0;
            seatop_begin_touch_down(seat, surface, event, sx, sy, lx, ly);
        }
    } else if !(*cursor).simulating_pointer_from_touch
        && (surface.is_null()
            || seat_is_input_allowed(seat, surface) as ::core::ffi::c_int != 0)
    {
        (*cursor).simulating_pointer_from_touch = r#true != 0;
        (*cursor).pointer_touch_id = (*seat).touch_id;
        let mut dx: ::core::ffi::c_double = 0.;
        let mut dy: ::core::ffi::c_double = 0.;
        dx = (*seat).touch_x - (*(*cursor).cursor).x;
        dy = (*seat).touch_y - (*(*cursor).cursor).y;
        pointer_motion(
            cursor,
            (*event).time_msec,
            &raw mut (*(*event).touch).base,
            dx,
            dy,
            dx,
            dy,
        );
        dispatch_cursor_button(
            cursor,
            &raw mut (*(*event).touch).base,
            (*event).time_msec,
            BTN_LEFT as uint32_t,
            wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED,
        );
    }
}
unsafe extern "C" fn wl_axis_to_button(
    mut event: *mut wlr_pointer_axis_event,
) -> uint32_t {
    match (*event).orientation {
        wl_pointer_axis::WL_POINTER_AXIS_VERTICAL_SCROLL => {
            return (if (*event).delta < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                SWAY_SCROLL_UP
            } else {
                SWAY_SCROLL_DOWN
            }) as uint32_t;
        }
        wl_pointer_axis::WL_POINTER_AXIS_HORIZONTAL_SCROLL => {
            return (if (*event).delta < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                SWAY_SCROLL_LEFT
            } else {
                SWAY_SCROLL_RIGHT
            }) as uint32_t;
        }
        _ => {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Unknown axis orientation\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/seatop_default.c\0".as_ptr() as *const ::core::ffi::c_char,
                701 as ::core::ffi::c_int,
            );
            return 0 as uint32_t;
        }
    };
}
unsafe extern "C" fn handle_pointer_axis(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_axis_event,
) {
    let mut input_device: *mut sway_input_device = (if !(*event).pointer.is_null() {
        (*(*event).pointer).base.data
    } else {
        NULL
    }) as *mut sway_input_device;
    let mut ic: *mut input_config = if !input_device.is_null() {
        input_device_get_config(input_device)
    } else {
        ::core::ptr::null_mut::<input_config>()
    };
    let mut cursor: *mut sway_cursor = (*seat).cursor;
    let mut e: *mut seatop_default_event = (*seat).seatop_data
        as *mut seatop_default_event;
    let mut surface: *mut wlr_surface = ::core::ptr::null_mut::<wlr_surface>();
    let mut sx: ::core::ffi::c_double = 0.;
    let mut sy: ::core::ffi::c_double = 0.;
    let mut node: *mut sway_node = node_at_coords(
        seat,
        (*(*cursor).cursor).x,
        (*(*cursor).cursor).y,
        &raw mut surface,
        &raw mut sx,
        &raw mut sy,
    );
    let mut cont: *mut sway_container = if !node.is_null()
        && (*node).r#type.0 == sway_node_type::N_CONTAINER.0
    {
        (*node).c2rust_unnamed.sway_container
    } else {
        ::core::ptr::null_mut::<sway_container>()
    };
    let mut edge: wlr_edges = wlr_edges(
        if !cont.is_null() {
            find_edge(cont, surface, cursor).0
        } else {
            wlr_edges::WLR_EDGE_NONE.0
        },
    );
    let mut on_border: bool = edge.0 != wlr_edges::WLR_EDGE_NONE.0;
    let mut on_titlebar: bool = !cont.is_null() && !on_border && surface.is_null();
    let mut on_titlebar_border: bool = !cont.is_null()
        && on_border as ::core::ffi::c_int != 0
        && (*(*cursor).cursor).y < (*cont).pending.content_y;
    let mut on_contents: bool = !cont.is_null() && !on_border && !surface.is_null();
    let mut on_workspace: bool = !node.is_null()
        && (*node).r#type.0 == sway_node_type::N_WORKSPACE.0;
    let mut scroll_factor: ::core::ffi::c_float = if ic.is_null()
        || (*ic).scroll_factor == FLT_MIN
    {
        1.0f32
    } else {
        (*ic).scroll_factor
    };
    let mut handled: bool = r#false != 0;
    let mut keyboard: *mut wlr_keyboard = wlr_seat_get_keyboard((*seat).wlr_seat);
    let mut modifiers: uint32_t = if !keyboard.is_null() {
        wlr_keyboard_get_modifiers(keyboard)
    } else {
        0 as uint32_t
    };
    let mut device: *mut wlr_input_device = if !input_device.is_null() {
        (*input_device).wlr_device
    } else {
        ::core::ptr::null_mut::<wlr_input_device>()
    };
    let mut dev_id: *mut ::core::ffi::c_char = if !device.is_null() {
        input_device_get_identifier(device)
    } else {
        strdup(b"*\0".as_ptr() as *const ::core::ffi::c_char)
    };
    let mut button: uint32_t = wl_axis_to_button(event);
    let mut binding: *mut sway_binding = ::core::ptr::null_mut::<sway_binding>();
    state_add_button(e, button);
    binding = get_active_mouse_binding(
        e,
        (*(*config).current_mode).mouse_bindings,
        modifiers,
        r#false != 0,
        on_titlebar,
        on_border,
        on_contents,
        on_workspace,
        dev_id,
    );
    if !binding.is_null() {
        seat_execute_command(seat, binding);
        handled = r#true != 0;
    }
    if !handled
        && (on_titlebar as ::core::ffi::c_int != 0
            || on_titlebar_border as ::core::ffi::c_int != 0)
    {
        let mut new_focus: *mut sway_node = ::core::ptr::null_mut::<sway_node>();
        let mut layout: sway_container_layout = container_parent_layout(cont);
        if layout.0 == sway_container_layout_0::L_TABBED.0
            || layout.0 == sway_container_layout_0::L_STACKED.0
        {
            let mut tabcontainer: *mut sway_node = node_get_parent(node);
            let mut active: *mut sway_node = seat_get_active_tiling_child(
                seat,
                tabcontainer,
            );
            let mut siblings: *mut list_t = container_get_siblings(cont);
            let mut desired: ::core::ffi::c_int = (list_find(
                siblings,
                (*active).c2rust_unnamed.sway_container as *const ::core::ffi::c_void,
            ) as ::core::ffi::c_float
                + roundf(
                    scroll_factor * (*event).delta_discrete as ::core::ffi::c_float
                        / WLR_POINTER_AXIS_DISCRETE_STEP as ::core::ffi::c_float,
                )) as ::core::ffi::c_int;
            if desired < 0 as ::core::ffi::c_int {
                desired = 0 as ::core::ffi::c_int;
            } else if desired >= (*siblings).length {
                desired = (*siblings).length - 1 as ::core::ffi::c_int;
            }
            let mut new_sibling_con: *mut sway_container = *(*siblings)
                .items
                .offset(desired as isize) as *mut sway_container;
            let mut new_sibling: *mut sway_node = &raw mut (*new_sibling_con).node;
            new_focus = seat_get_focus_inactive(seat, new_sibling);
        } else {
            new_focus = seat_get_focus_inactive(seat, &raw mut (*cont).node);
        }
        seat_set_focus(seat, new_focus);
        transaction_commit_dirty();
        handled = r#true != 0;
    }
    binding = get_active_mouse_binding(
        e,
        (*(*config).current_mode).mouse_bindings,
        modifiers,
        r#true != 0,
        on_titlebar,
        on_border,
        on_contents,
        on_workspace,
        dev_id,
    );
    state_erase_button(e, button);
    if !binding.is_null() {
        seat_execute_command(seat, binding);
        handled = r#true != 0;
    }
    free(dev_id as *mut ::core::ffi::c_void);
    if !handled {
        wlr_seat_pointer_notify_axis(
            (*(*cursor).seat).wlr_seat,
            (*event).time_msec,
            (*event).orientation,
            scroll_factor as ::core::ffi::c_double * (*event).delta,
            roundf(scroll_factor * (*event).delta_discrete as ::core::ffi::c_float)
                as int32_t,
            (*event).source,
            (*event).relative_direction,
        );
    }
}
unsafe extern "C" fn gesture_binding_check(
    mut bindings: *mut list_t,
    mut r#type: gesture_type,
    mut fingers: uint8_t,
    mut device: *mut sway_input_device,
) -> bool {
    let mut input: *mut ::core::ffi::c_char = if !device.is_null() {
        input_device_get_identifier((*device).wlr_device)
    } else {
        strdup(b"*\0".as_ptr() as *const ::core::ffi::c_char)
    };
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*bindings).length {
        let mut binding: *mut sway_gesture_binding = *(*bindings)
            .items
            .offset(i as isize) as *mut sway_gesture_binding;
        if gesture_check(&raw mut (*binding).gesture, r#type, fingers) {
            if !(strcmp((*binding).input, b"*\0".as_ptr() as *const ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
                && strcmp((*binding).input, input) != 0 as ::core::ffi::c_int)
            {
                free(input as *mut ::core::ffi::c_void);
                return r#true != 0;
            }
        }
        i += 1;
    }
    free(input as *mut ::core::ffi::c_void);
    return r#false != 0;
}
unsafe extern "C" fn gesture_binding_match(
    mut bindings: *mut list_t,
    mut gesture: *mut gesture,
    mut input: *const ::core::ffi::c_char,
) -> *mut sway_gesture_binding {
    let mut current: *mut sway_gesture_binding = ::core::ptr::null_mut::<
        sway_gesture_binding,
    >();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    's_77: while i < (*bindings).length {
        let mut binding: *mut sway_gesture_binding = *(*bindings)
            .items
            .offset(i as isize) as *mut sway_gesture_binding;
        let mut exact: bool = (*binding).flags
            & binding_flags::BINDING_EXACT.0 as ::core::ffi::c_int as uint32_t != 0;
        's_6: {
            if gesture_match(&raw mut (*binding).gesture, gesture, exact) {
                if !(strcmp(
                    (*binding).input,
                    b"*\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                    && strcmp((*binding).input, input) != 0 as ::core::ffi::c_int)
                {
                    if !current.is_null() {
                        if strcmp((*current).input, (*binding).input)
                            == 0 as ::core::ffi::c_int
                        {
                            if !exact
                                && (*current).flags
                                    & binding_flags::BINDING_EXACT.0 as ::core::ffi::c_int
                                        as uint32_t != 0
                            {
                                break 's_6;
                            } else if gesture_compare(
                                &raw mut (*current).gesture,
                                &raw mut (*binding).gesture,
                            ) as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                            {
                                break 's_6;
                            }
                        } else if strcmp(
                            (*binding).input,
                            b"*\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break 's_6;
                        }
                    }
                    current = binding;
                    if strcmp((*current).input, input) == 0 as ::core::ffi::c_int
                        && gesture_compare(&raw mut (*current).gesture, gesture)
                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        break 's_77;
                    }
                }
            }
        }
        i += 1;
    }
    return current;
}
unsafe extern "C" fn gesture_tracker_end_and_match(
    mut tracker: *mut gesture_tracker,
    mut device: *mut sway_input_device,
) -> *mut sway_gesture_binding {
    let mut input: *mut ::core::ffi::c_char = if !device.is_null() {
        input_device_get_identifier((*device).wlr_device)
    } else {
        strdup(b"*\0".as_ptr() as *const ::core::ffi::c_char)
    };
    let mut gesture: *mut gesture = gesture_tracker_end(tracker);
    let mut binding: *mut sway_gesture_binding = gesture_binding_match(
        (*(*config).current_mode).gesture_bindings,
        gesture,
        input,
    );
    free(gesture as *mut ::core::ffi::c_void);
    free(input as *mut ::core::ffi::c_void);
    return binding;
}
unsafe extern "C" fn gesture_binding_execute(
    mut seat: *mut sway_seat,
    mut binding: *mut sway_gesture_binding,
) {
    let mut dummy_binding: *mut sway_binding = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_binding>(),
    ) as *mut sway_binding;
    (*dummy_binding).r#type = binding_input_type::BINDING_GESTURE;
    (*dummy_binding).command = (*binding).command;
    let mut description: *mut ::core::ffi::c_char = gesture_to_string(
        &raw mut (*binding).gesture,
    );
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] executing gesture binding: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/seatop_default.c\0".as_ptr() as *const ::core::ffi::c_char,
        924 as ::core::ffi::c_int,
        description,
    );
    free(description as *mut ::core::ffi::c_void);
    seat_execute_command(seat, dummy_binding);
    free(dummy_binding as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn handle_hold_begin(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_hold_begin_event,
) {
    let mut device: *mut sway_input_device = (if !(*event).pointer.is_null() {
        (*(*event).pointer).base.data
    } else {
        NULL
    }) as *mut sway_input_device;
    let mut bindings: *mut list_t = (*(*config).current_mode).gesture_bindings;
    if gesture_binding_check(
        bindings,
        gesture_type::GESTURE_TYPE_HOLD,
        (*event).fingers as uint8_t,
        device,
    ) {
        let mut seatop: *mut seatop_default_event = (*seat).seatop_data
            as *mut seatop_default_event;
        gesture_tracker_begin(
            &raw mut (*seatop).gestures,
            gesture_type::GESTURE_TYPE_HOLD,
            (*event).fingers as uint8_t,
        );
    } else {
        let mut cursor: *mut sway_cursor = (*seat).cursor;
        wlr_pointer_gestures_v1_send_hold_begin(
            (*server.input).pointer_gestures,
            (*(*cursor).seat).wlr_seat,
            (*event).time_msec,
            (*event).fingers,
        );
    };
}
unsafe extern "C" fn handle_hold_end(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_hold_end_event,
) {
    let mut seatop: *mut seatop_default_event = (*seat).seatop_data
        as *mut seatop_default_event;
    if !gesture_tracker_check(
        &raw mut (*seatop).gestures,
        gesture_type::GESTURE_TYPE_HOLD,
    ) {
        let mut cursor: *mut sway_cursor = (*seat).cursor;
        wlr_pointer_gestures_v1_send_hold_end(
            (*server.input).pointer_gestures,
            (*(*cursor).seat).wlr_seat,
            (*event).time_msec,
            (*event).cancelled,
        );
        return;
    }
    if (*event).cancelled {
        gesture_tracker_cancel(&raw mut (*seatop).gestures);
        return;
    }
    let mut device: *mut sway_input_device = (if !(*event).pointer.is_null() {
        (*(*event).pointer).base.data
    } else {
        NULL
    }) as *mut sway_input_device;
    let mut binding: *mut sway_gesture_binding = gesture_tracker_end_and_match(
        &raw mut (*seatop).gestures,
        device,
    );
    if !binding.is_null() {
        gesture_binding_execute(seat, binding);
    }
}
unsafe extern "C" fn handle_pinch_begin(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_pinch_begin_event,
) {
    let mut device: *mut sway_input_device = (if !(*event).pointer.is_null() {
        (*(*event).pointer).base.data
    } else {
        NULL
    }) as *mut sway_input_device;
    let mut bindings: *mut list_t = (*(*config).current_mode).gesture_bindings;
    if gesture_binding_check(
        bindings,
        gesture_type::GESTURE_TYPE_PINCH,
        (*event).fingers as uint8_t,
        device,
    ) {
        let mut seatop: *mut seatop_default_event = (*seat).seatop_data
            as *mut seatop_default_event;
        gesture_tracker_begin(
            &raw mut (*seatop).gestures,
            gesture_type::GESTURE_TYPE_PINCH,
            (*event).fingers as uint8_t,
        );
    } else {
        let mut cursor: *mut sway_cursor = (*seat).cursor;
        wlr_pointer_gestures_v1_send_pinch_begin(
            (*server.input).pointer_gestures,
            (*(*cursor).seat).wlr_seat,
            (*event).time_msec,
            (*event).fingers,
        );
    };
}
unsafe extern "C" fn handle_pinch_update(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_pinch_update_event,
) {
    let mut seatop: *mut seatop_default_event = (*seat).seatop_data
        as *mut seatop_default_event;
    if gesture_tracker_check(
        &raw mut (*seatop).gestures,
        gesture_type::GESTURE_TYPE_PINCH,
    ) {
        gesture_tracker_update(
            &raw mut (*seatop).gestures,
            (*event).dx,
            (*event).dy,
            (*event).scale,
            (*event).rotation,
        );
    } else {
        let mut cursor: *mut sway_cursor = (*seat).cursor;
        wlr_pointer_gestures_v1_send_pinch_update(
            (*server.input).pointer_gestures,
            (*(*cursor).seat).wlr_seat,
            (*event).time_msec,
            (*event).dx,
            (*event).dy,
            (*event).scale,
            (*event).rotation,
        );
    };
}
unsafe extern "C" fn handle_pinch_end(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_pinch_end_event,
) {
    let mut seatop: *mut seatop_default_event = (*seat).seatop_data
        as *mut seatop_default_event;
    if !gesture_tracker_check(
        &raw mut (*seatop).gestures,
        gesture_type::GESTURE_TYPE_PINCH,
    ) {
        let mut cursor: *mut sway_cursor = (*seat).cursor;
        wlr_pointer_gestures_v1_send_pinch_end(
            (*server.input).pointer_gestures,
            (*(*cursor).seat).wlr_seat,
            (*event).time_msec,
            (*event).cancelled,
        );
        return;
    }
    if (*event).cancelled {
        gesture_tracker_cancel(&raw mut (*seatop).gestures);
        return;
    }
    let mut device: *mut sway_input_device = (if !(*event).pointer.is_null() {
        (*(*event).pointer).base.data
    } else {
        NULL
    }) as *mut sway_input_device;
    let mut binding: *mut sway_gesture_binding = gesture_tracker_end_and_match(
        &raw mut (*seatop).gestures,
        device,
    );
    if !binding.is_null() {
        gesture_binding_execute(seat, binding);
    }
}
unsafe extern "C" fn handle_swipe_begin(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_swipe_begin_event,
) {
    let mut device: *mut sway_input_device = (if !(*event).pointer.is_null() {
        (*(*event).pointer).base.data
    } else {
        NULL
    }) as *mut sway_input_device;
    let mut bindings: *mut list_t = (*(*config).current_mode).gesture_bindings;
    if gesture_binding_check(
        bindings,
        gesture_type::GESTURE_TYPE_SWIPE,
        (*event).fingers as uint8_t,
        device,
    ) {
        let mut seatop: *mut seatop_default_event = (*seat).seatop_data
            as *mut seatop_default_event;
        gesture_tracker_begin(
            &raw mut (*seatop).gestures,
            gesture_type::GESTURE_TYPE_SWIPE,
            (*event).fingers as uint8_t,
        );
    } else {
        let mut cursor: *mut sway_cursor = (*seat).cursor;
        wlr_pointer_gestures_v1_send_swipe_begin(
            (*server.input).pointer_gestures,
            (*(*cursor).seat).wlr_seat,
            (*event).time_msec,
            (*event).fingers,
        );
    };
}
unsafe extern "C" fn handle_swipe_update(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_swipe_update_event,
) {
    let mut seatop: *mut seatop_default_event = (*seat).seatop_data
        as *mut seatop_default_event;
    if gesture_tracker_check(
        &raw mut (*seatop).gestures,
        gesture_type::GESTURE_TYPE_SWIPE,
    ) {
        gesture_tracker_update(
            &raw mut (*seatop).gestures,
            (*event).dx,
            (*event).dy,
            ::core::f32::NAN as ::core::ffi::c_double,
            ::core::f32::NAN as ::core::ffi::c_double,
        );
    } else {
        let mut cursor: *mut sway_cursor = (*seat).cursor;
        wlr_pointer_gestures_v1_send_swipe_update(
            (*server.input).pointer_gestures,
            (*(*cursor).seat).wlr_seat,
            (*event).time_msec,
            (*event).dx,
            (*event).dy,
        );
    };
}
unsafe extern "C" fn handle_swipe_end(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_swipe_end_event,
) {
    let mut seatop: *mut seatop_default_event = (*seat).seatop_data
        as *mut seatop_default_event;
    if !gesture_tracker_check(
        &raw mut (*seatop).gestures,
        gesture_type::GESTURE_TYPE_SWIPE,
    ) {
        let mut cursor: *mut sway_cursor = (*seat).cursor;
        wlr_pointer_gestures_v1_send_swipe_end(
            (*server.input).pointer_gestures,
            (*(*cursor).seat).wlr_seat,
            (*event).time_msec,
            (*event).cancelled,
        );
        return;
    }
    if (*event).cancelled {
        gesture_tracker_cancel(&raw mut (*seatop).gestures);
        return;
    }
    let mut device: *mut sway_input_device = (if !(*event).pointer.is_null() {
        (*(*event).pointer).base.data
    } else {
        NULL
    }) as *mut sway_input_device;
    let mut binding: *mut sway_gesture_binding = gesture_tracker_end_and_match(
        &raw mut (*seatop).gestures,
        device,
    );
    if !binding.is_null() {
        gesture_binding_execute(seat, binding);
    }
}
unsafe extern "C" fn handle_rebase(mut seat: *mut sway_seat, mut time_msec: uint32_t) {
    let mut e: *mut seatop_default_event = (*seat).seatop_data
        as *mut seatop_default_event;
    let mut cursor: *mut sway_cursor = (*seat).cursor;
    let mut surface: *mut wlr_surface = ::core::ptr::null_mut::<wlr_surface>();
    let mut sx: ::core::ffi::c_double = 0.0f64;
    let mut sy: ::core::ffi::c_double = 0.0f64;
    (*e).previous_node = node_at_coords(
        seat,
        (*(*cursor).cursor).x,
        (*(*cursor).cursor).y,
        &raw mut surface,
        &raw mut sx,
        &raw mut sy,
    );
    if !surface.is_null() {
        if seat_is_input_allowed(seat, surface) as ::core::ffi::c_int != 0
            && !(*cursor).hidden
        {
            wlr_seat_pointer_notify_enter((*seat).wlr_seat, surface, sx, sy);
            wlr_seat_pointer_notify_motion((*seat).wlr_seat, time_msec, sx, sy);
            wlr_seat_pointer_notify_frame((*seat).wlr_seat);
        }
    } else {
        cursor_update_image(cursor, (*e).previous_node);
        wlr_seat_pointer_notify_clear_focus((*seat).wlr_seat);
    };
}
static mut seatop_impl: sway_seatop_impl = sway_seatop_impl {
    button: Some(
        handle_button
            as unsafe extern "C" fn(
                *mut sway_seat,
                uint32_t,
                *mut wlr_input_device,
                uint32_t,
                wl_pointer_button_state,
            ) -> (),
    ),
    pointer_motion: Some(
        handle_pointer_motion as unsafe extern "C" fn(*mut sway_seat, uint32_t) -> (),
    ),
    pointer_axis: Some(
        handle_pointer_axis
            as unsafe extern "C" fn(*mut sway_seat, *mut wlr_pointer_axis_event) -> (),
    ),
    hold_begin: Some(
        handle_hold_begin
            as unsafe extern "C" fn(
                *mut sway_seat,
                *mut wlr_pointer_hold_begin_event,
            ) -> (),
    ),
    hold_end: Some(
        handle_hold_end
            as unsafe extern "C" fn(
                *mut sway_seat,
                *mut wlr_pointer_hold_end_event,
            ) -> (),
    ),
    pinch_begin: Some(
        handle_pinch_begin
            as unsafe extern "C" fn(
                *mut sway_seat,
                *mut wlr_pointer_pinch_begin_event,
            ) -> (),
    ),
    pinch_update: Some(
        handle_pinch_update
            as unsafe extern "C" fn(
                *mut sway_seat,
                *mut wlr_pointer_pinch_update_event,
            ) -> (),
    ),
    pinch_end: Some(
        handle_pinch_end
            as unsafe extern "C" fn(
                *mut sway_seat,
                *mut wlr_pointer_pinch_end_event,
            ) -> (),
    ),
    swipe_begin: Some(
        handle_swipe_begin
            as unsafe extern "C" fn(
                *mut sway_seat,
                *mut wlr_pointer_swipe_begin_event,
            ) -> (),
    ),
    swipe_update: Some(
        handle_swipe_update
            as unsafe extern "C" fn(
                *mut sway_seat,
                *mut wlr_pointer_swipe_update_event,
            ) -> (),
    ),
    swipe_end: Some(
        handle_swipe_end
            as unsafe extern "C" fn(
                *mut sway_seat,
                *mut wlr_pointer_swipe_end_event,
            ) -> (),
    ),
    rebase: Some(handle_rebase as unsafe extern "C" fn(*mut sway_seat, uint32_t) -> ()),
    touch_motion: None,
    touch_up: None,
    touch_down: Some(
        handle_touch_down
            as unsafe extern "C" fn(
                *mut sway_seat,
                *mut wlr_touch_down_event,
                ::core::ffi::c_double,
                ::core::ffi::c_double,
            ) -> (),
    ),
    touch_cancel: None,
    tablet_tool_motion: Some(
        handle_tablet_tool_motion
            as unsafe extern "C" fn(
                *mut sway_seat,
                *mut sway_tablet_tool,
                uint32_t,
            ) -> (),
    ),
    tablet_tool_tip: Some(
        handle_tablet_tool_tip
            as unsafe extern "C" fn(
                *mut sway_seat,
                *mut sway_tablet_tool,
                uint32_t,
                wlr_tablet_tool_tip_state,
            ) -> (),
    ),
    end: None,
    unref: None,
    allow_set_cursor: r#true != 0,
};
#[no_mangle]
pub unsafe extern "C" fn seatop_begin_default(mut seat: *mut sway_seat) {
    seatop_end(seat);
    let mut e: *mut seatop_default_event = calloc(
        1 as size_t,
        ::core::mem::size_of::<seatop_default_event>(),
    ) as *mut seatop_default_event;
    _sway_assert(
        !e.is_null(),
        b"[%s:%d] %s:Unable to allocate seatop_default_event\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/seatop_default.c\0".as_ptr() as *const ::core::ffi::c_char,
        1149 as ::core::ffi::c_int,
        b"void seatop_begin_default(struct sway_seat *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    (*seat).seatop_impl = &raw const seatop_impl;
    (*seat).seatop_data = e as *mut ::core::ffi::c_void;
    let mut time_msec: uint32_t = get_current_time_in_msec();
    seatop_rebase(seat, time_msec);
}
pub const FLT_MIN: ::core::ffi::c_float = __FLT_MIN__;
pub const __FLT_MIN__: ::core::ffi::c_float = 1.17549435e-38f32;
pub const BTN_LEFT: ::core::ffi::c_int = 0x110 as ::core::ffi::c_int;
pub const BTN_RIGHT: ::core::ffi::c_int = 0x111 as ::core::ffi::c_int;
pub const KEY_MAX: ::core::ffi::c_int = 0x2ff as ::core::ffi::c_int;
