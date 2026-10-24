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
    pub type wlr_tablet_client_v2;
    pub type wlr_tablet_impl;
    pub type wlr_tablet_tool_client_v2;
    pub type xkb_state;
    pub type xkb_keymap;
    pub type wlr_keyboard_group;
    pub type wlr_keyboard_impl;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_primary_selection_source;
    pub type wlr_touch_impl;
    pub type wlr_tablet_pad_client_v2;
    pub type wlr_tablet_pad_impl;
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
    pub type wlr_xwayland_shell_v1;
    pub type wlr_xwm;
    pub type wlr_xdg_shell;
    pub type wlr_idle_notifier_v1;
    pub type wlr_data_device_manager;
    pub type wlr_session;
    pub type sway_transaction_instruction;
    pub type sway_text_node;
    pub type wlr_xdg_toplevel;
    pub type wlr_foreign_toplevel_handle_v1;
    pub type wlr_ext_foreign_toplevel_handle_v1;
    pub type launcher_ctx;
    pub type sway_xdg_decoration;
    pub type wlr_ext_image_capture_source_v1;
    pub type sway_switch;
    pub type _PangoFontDescription;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn libevdev_event_code_get_name(
        r#type: ::core::ffi::c_uint,
        code: ::core::ffi::c_uint,
    ) -> *const ::core::ffi::c_char;
    fn libevdev_event_code_from_name(
        r#type: ::core::ffi::c_uint,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn wl_list_init(list: *mut wl_list);
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn wl_event_loop_add_timer(
        r#loop: *mut wl_event_loop,
        func: wl_event_loop_timer_func_t,
        data: *mut ::core::ffi::c_void,
    ) -> *mut wl_event_source;
    fn wl_event_source_timer_update(
        source: *mut wl_event_source,
        ms_delay: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn wl_event_source_remove(source: *mut wl_event_source) -> ::core::ffi::c_int;
    fn wl_resource_get_client(resource: *mut wl_resource) -> *mut wl_client;
    fn pixman_region32_copy(
        dest: *mut pixman_region32_t,
        source: *const pixman_region32_t,
    ) -> pixman_bool_t;
    fn pixman_region32_intersect(
        new_reg: *mut pixman_region32_t,
        reg1: *const pixman_region32_t,
        reg2: *const pixman_region32_t,
    ) -> pixman_bool_t;
    fn pixman_region32_contains_point(
        region: *const pixman_region32_t,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        r#box: *mut pixman_box32_t,
    ) -> pixman_bool_t;
    fn pixman_region32_not_empty(region: *const pixman_region32_t) -> pixman_bool_t;
    fn pixman_region32_rectangles(
        region: *const pixman_region32_t,
        n_rects: *mut ::core::ffi::c_int,
    ) -> *mut pixman_box32_t;
    fn pixman_region32_clear(region: *mut pixman_region32_t);
    fn wlr_box_contains_point(
        r#box: *const wlr_box,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> bool;
    fn wlr_cursor_create() -> *mut wlr_cursor;
    fn wlr_cursor_destroy(cur: *mut wlr_cursor);
    fn wlr_cursor_warp(
        cur: *mut wlr_cursor,
        dev: *mut wlr_input_device,
        lx: ::core::ffi::c_double,
        ly: ::core::ffi::c_double,
    ) -> bool;
    fn wlr_cursor_absolute_to_layout_coords(
        cur: *mut wlr_cursor,
        dev: *mut wlr_input_device,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        lx: *mut ::core::ffi::c_double,
        ly: *mut ::core::ffi::c_double,
    );
    fn wlr_cursor_warp_closest(
        cur: *mut wlr_cursor,
        dev: *mut wlr_input_device,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    );
    fn wlr_cursor_warp_absolute(
        cur: *mut wlr_cursor,
        dev: *mut wlr_input_device,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    );
    fn wlr_cursor_move(
        cur: *mut wlr_cursor,
        dev: *mut wlr_input_device,
        delta_x: ::core::ffi::c_double,
        delta_y: ::core::ffi::c_double,
    );
    fn wlr_cursor_unset_image(cur: *mut wlr_cursor);
    fn wlr_cursor_set_xcursor(
        cur: *mut wlr_cursor,
        manager: *mut wlr_xcursor_manager,
        name: *const ::core::ffi::c_char,
    );
    fn wlr_cursor_set_surface(
        cur: *mut wlr_cursor,
        surface: *mut wlr_surface,
        hotspot_x: int32_t,
        hotspot_y: int32_t,
    );
    fn wlr_cursor_attach_output_layout(cur: *mut wlr_cursor, l: *mut wlr_output_layout);
    fn wlr_keyboard_get_modifiers(keyboard: *mut wlr_keyboard) -> uint32_t;
    fn wlr_tablet_v2_tablet_tool_notify_pressure(
        tool: *mut wlr_tablet_v2_tablet_tool,
        pressure: ::core::ffi::c_double,
    );
    fn wlr_tablet_v2_tablet_tool_notify_distance(
        tool: *mut wlr_tablet_v2_tablet_tool,
        distance: ::core::ffi::c_double,
    );
    fn wlr_tablet_v2_tablet_tool_notify_tilt(
        tool: *mut wlr_tablet_v2_tablet_tool,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    );
    fn wlr_tablet_v2_tablet_tool_notify_rotation(
        tool: *mut wlr_tablet_v2_tablet_tool,
        degrees: ::core::ffi::c_double,
    );
    fn wlr_tablet_v2_tablet_tool_notify_slider(
        tool: *mut wlr_tablet_v2_tablet_tool,
        position: ::core::ffi::c_double,
    );
    fn wlr_tablet_v2_tablet_tool_notify_wheel(
        tool: *mut wlr_tablet_v2_tablet_tool,
        degrees: ::core::ffi::c_double,
        clicks: int32_t,
    );
    fn wlr_tablet_v2_tablet_tool_notify_proximity_out(
        tool: *mut wlr_tablet_v2_tablet_tool,
    );
    fn wlr_tablet_v2_tablet_tool_notify_button(
        tool: *mut wlr_tablet_v2_tablet_tool,
        button: uint32_t,
        state: zwp_tablet_pad_v2_button_state,
    );
    fn wlr_tablet_tool_v2_has_implicit_grab(
        tool: *mut wlr_tablet_v2_tablet_tool,
    ) -> bool;
    fn wlr_surface_accepts_tablet_v2(
        surface: *mut wlr_surface,
        tablet: *mut wlr_tablet_v2_tablet,
    ) -> bool;
    fn wlr_region_confine(
        region: *const pixman_region32_t,
        x1: ::core::ffi::c_double,
        y1: ::core::ffi::c_double,
        x2: ::core::ffi::c_double,
        y2: ::core::ffi::c_double,
        x2_out: *mut ::core::ffi::c_double,
        y2_out: *mut ::core::ffi::c_double,
    ) -> bool;
    fn wlr_xcursor_get_resize_name(edges: wlr_edges) -> *const ::core::ffi::c_char;
    fn wlr_tablet_from_input_device(
        input_device: *mut wlr_input_device,
    ) -> *mut wlr_tablet;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn wlr_relative_pointer_manager_v1_send_relative_motion(
        manager: *mut wlr_relative_pointer_manager_v1,
        seat: *mut wlr_seat,
        time_usec: uint64_t,
        dx: ::core::ffi::c_double,
        dy: ::core::ffi::c_double,
        dx_unaccel: ::core::ffi::c_double,
        dy_unaccel: ::core::ffi::c_double,
    );
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    fn get_current_time_in_msec() -> uint32_t;
    fn wlr_seat_pointer_notify_clear_focus(wlr_seat: *mut wlr_seat);
    fn wlr_seat_pointer_warp(
        wlr_seat: *mut wlr_seat,
        sx: ::core::ffi::c_double,
        sy: ::core::ffi::c_double,
    );
    fn wlr_seat_pointer_notify_frame(wlr_seat: *mut wlr_seat);
    fn wlr_seat_get_keyboard(seat: *mut wlr_seat) -> *mut wlr_keyboard;
    fn wlr_seat_touch_notify_frame(seat: *mut wlr_seat);
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn format_str(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn has_prefix(
        str: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    ) -> bool;
    fn wlr_cursor_shape_v1_name(
        shape: wp_cursor_shape_device_v1_shape,
    ) -> *const ::core::ffi::c_char;
    fn wlr_scene_node_at(
        node: *mut wlr_scene_node,
        lx: ::core::ffi::c_double,
        ly: ::core::ffi::c_double,
        nx: *mut ::core::ffi::c_double,
        ny: *mut ::core::ffi::c_double,
    ) -> *mut wlr_scene_node;
    fn wlr_scene_buffer_from_node(node: *mut wlr_scene_node) -> *mut wlr_scene_buffer;
    fn wlr_scene_tree_from_node(node: *mut wlr_scene_node) -> *mut wlr_scene_tree;
    fn wlr_scene_surface_try_from_buffer(
        scene_buffer: *mut wlr_scene_buffer,
    ) -> *mut wlr_scene_surface;
    static mut root: *mut sway_root;
    fn output_get_active_workspace(output: *mut sway_output) -> *mut sway_workspace;
    fn container_is_floating(container: *mut sway_container) -> bool;
    fn container_get_box(container: *mut sway_container, r#box: *mut wlr_box);
    fn wlr_output_layout_output_at(
        layout: *mut wlr_output_layout,
        lx: ::core::ffi::c_double,
        ly: ::core::ffi::c_double,
    ) -> *mut wlr_output;
    fn wlr_xcursor_manager_destroy(manager: *mut wlr_xcursor_manager);
    fn view_from_wlr_surface(surface: *mut wlr_surface) -> *mut sway_view;
    static mut config: *mut sway_config;
    fn sway_tablet_tool_configure(
        tablet: *mut sway_tablet,
        wlr_tool: *mut wlr_tablet_tool,
    );
    fn wlr_pointer_constraint_v1_send_activated(
        constraint: *mut wlr_pointer_constraint_v1,
    );
    fn wlr_pointer_constraint_v1_send_deactivated(
        constraint: *mut wlr_pointer_constraint_v1,
    );
    fn seat_get_config(seat: *mut sway_seat) -> *mut seat_config;
    fn seat_get_config_by_name(name: *const ::core::ffi::c_char) -> *mut seat_config;
    fn seat_idle_notify_activity(seat: *mut sway_seat, source: sway_input_idle_source);
    fn drag_icons_update_position(seat: *mut sway_seat);
    fn find_resize_edge(
        cont: *mut sway_container,
        surface: *mut wlr_surface,
        cursor: *mut sway_cursor,
    ) -> wlr_edges;
    fn seatop_button(
        seat: *mut sway_seat,
        time_msec: uint32_t,
        device: *mut wlr_input_device,
        button: uint32_t,
        state: wl_pointer_button_state,
    );
    fn seatop_pointer_motion(seat: *mut sway_seat, time_msec: uint32_t);
    fn seatop_pointer_axis(seat: *mut sway_seat, event: *mut wlr_pointer_axis_event);
    fn seatop_tablet_tool_tip(
        seat: *mut sway_seat,
        tool: *mut sway_tablet_tool,
        time_msec: uint32_t,
        state: wlr_tablet_tool_tip_state,
    );
    fn seatop_tablet_tool_motion(
        seat: *mut sway_seat,
        tool: *mut sway_tablet_tool,
        time_msec: uint32_t,
    );
    fn seatop_hold_begin(seat: *mut sway_seat, event: *mut wlr_pointer_hold_begin_event);
    fn seatop_hold_end(seat: *mut sway_seat, event: *mut wlr_pointer_hold_end_event);
    fn seatop_pinch_begin(
        seat: *mut sway_seat,
        event: *mut wlr_pointer_pinch_begin_event,
    );
    fn seatop_pinch_update(
        seat: *mut sway_seat,
        event: *mut wlr_pointer_pinch_update_event,
    );
    fn seatop_pinch_end(seat: *mut sway_seat, event: *mut wlr_pointer_pinch_end_event);
    fn seatop_swipe_begin(
        seat: *mut sway_seat,
        event: *mut wlr_pointer_swipe_begin_event,
    );
    fn seatop_swipe_update(
        seat: *mut sway_seat,
        event: *mut wlr_pointer_swipe_update_event,
    );
    fn seatop_swipe_end(seat: *mut sway_seat, event: *mut wlr_pointer_swipe_end_event);
    fn seatop_touch_motion(
        seat: *mut sway_seat,
        event: *mut wlr_touch_motion_event,
        lx: ::core::ffi::c_double,
        ly: ::core::ffi::c_double,
    );
    fn seatop_touch_up(seat: *mut sway_seat, event: *mut wlr_touch_up_event);
    fn seatop_touch_down(
        seat: *mut sway_seat,
        event: *mut wlr_touch_down_event,
        lx: ::core::ffi::c_double,
        ly: ::core::ffi::c_double,
    );
    fn seatop_touch_cancel(seat: *mut sway_seat, event: *mut wlr_touch_cancel_event);
    fn seatop_rebase(seat: *mut sway_seat, time_msec: uint32_t);
    fn seatop_allows_set_cursor(seat: *mut sway_seat) -> bool;
    fn scene_descriptor_try_get(
        node: *mut wlr_scene_node,
        r#type: sway_scene_descriptor_type,
    ) -> *mut ::core::ffi::c_void;
    static mut server: sway_server;
    fn input_device_get_config(device: *mut sway_input_device) -> *mut input_config;
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
pub type wl_event_loop_timer_func_t = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
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
pub type pixman_bool_t = ::core::ffi::c_int;
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
pub struct wl_seat_capability(pub ::core::ffi::c_uint);
impl wl_seat_capability {
    pub const WL_SEAT_CAPABILITY_POINTER: Self = Self(1);
    pub const WL_SEAT_CAPABILITY_KEYBOARD: Self = Self(2);
    pub const WL_SEAT_CAPABILITY_TOUCH: Self = Self(4);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_cursor_shape_manager_v1_device_type(pub ::core::ffi::c_uint);
impl wlr_cursor_shape_manager_v1_device_type {
    pub const WLR_CURSOR_SHAPE_MANAGER_V1_DEVICE_TYPE_POINTER: Self = Self(0);
    pub const WLR_CURSOR_SHAPE_MANAGER_V1_DEVICE_TYPE_TABLET_TOOL: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_cursor_shape_manager_v1_request_set_shape_event {
    pub seat_client: *mut wlr_seat_client,
    pub device_type: wlr_cursor_shape_manager_v1_device_type,
    pub tablet_tool: *mut wlr_tablet_v2_tablet_tool,
    pub serial: uint32_t,
    pub shape: wp_cursor_shape_device_v1_shape,
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
    pub events: C2Rust_Unnamed_25,
    pub WLR_PRIVATE: C2Rust_Unnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub surface_destroy: wl_listener,
    pub tool_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub set_cursor: wl_signal,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct zwp_tablet_pad_v2_button_state(pub ::core::ffi::c_uint);
impl zwp_tablet_pad_v2_button_state {
    pub const ZWP_TABLET_PAD_V2_BUTTON_STATE_RELEASED: Self = Self(0);
    pub const ZWP_TABLET_PAD_V2_BUTTON_STATE_PRESSED: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_v2_tablet {
    pub link: wl_list,
    pub wlr_tablet: *mut wlr_tablet,
    pub wlr_device: *mut wlr_input_device,
    pub clients: wl_list,
    pub current_client: *mut wlr_tablet_client_v2,
    pub WLR_PRIVATE: C2Rust_Unnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
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
    pub events: C2Rust_Unnamed_27,
    pub paths: wl_array,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub axis: wl_signal,
    pub proximity: wl_signal,
    pub tip: wl_signal,
    pub button: wl_signal,
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
    pub events: C2Rust_Unnamed_28,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
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
pub struct wlr_seat_client {
    pub client: *mut wl_client,
    pub seat: *mut wlr_seat,
    pub link: wl_list,
    pub resources: wl_list,
    pub pointers: wl_list,
    pub keyboards: wl_list,
    pub touches: wl_list,
    pub data_devices: wl_list,
    pub events: C2Rust_Unnamed_30,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
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
pub struct C2Rust_Unnamed_30 {
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
    pub events: C2Rust_Unnamed_32,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
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
    pub events: C2Rust_Unnamed_34,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
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
    pub events: C2Rust_Unnamed_36,
    pub WLR_PRIVATE: C2Rust_Unnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
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
pub type xkb_layout_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
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
    pub repeat_info: C2Rust_Unnamed_38,
    pub events: C2Rust_Unnamed_37,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_37 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
    pub rate: int32_t,
    pub delay: int32_t,
}
pub type xkb_mod_index_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
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
    pub events: C2Rust_Unnamed_40,
    pub WLR_PRIVATE: C2Rust_Unnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_40 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_button_state(pub ::core::ffi::c_uint);
impl wlr_button_state {
    pub const WLR_BUTTON_RELEASED: Self = Self(0);
    pub const WLR_BUTTON_PRESSED: Self = Self(1);
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
    pub events: C2Rust_Unnamed_41,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_41 {
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
pub struct wlr_pointer_motion_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub delta_x: ::core::ffi::c_double,
    pub delta_y: ::core::ffi::c_double,
    pub unaccel_dx: ::core::ffi::c_double,
    pub unaccel_dy: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_motion_absolute_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_button_event {
    pub pointer: *mut wlr_pointer,
    pub time_msec: uint32_t,
    pub button: uint32_t,
    pub state: wl_pointer_button_state,
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
pub struct wlr_relative_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub relative_pointers: wl_list,
    pub events: C2Rust_Unnamed_43,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_42,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
    pub display_destroy_listener: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
    pub destroy: wl_signal,
    pub new_relative_pointer: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_touch {
    pub base: wlr_input_device,
    pub r#impl: *const wlr_touch_impl,
    pub output_name: *mut ::core::ffi::c_char,
    pub width_mm: ::core::ffi::c_double,
    pub height_mm: ::core::ffi::c_double,
    pub events: C2Rust_Unnamed_44,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_44 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_pointer_request_set_cursor_event {
    pub seat_client: *mut wlr_seat_client,
    pub surface: *mut wlr_surface,
    pub serial: uint32_t,
    pub hotspot_x: int32_t,
    pub hotspot_y: int32_t,
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
    pub events: C2Rust_Unnamed_46,
    pub WLR_PRIVATE: C2Rust_Unnamed_45,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_45 {
    pub pad_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
    pub button_feedback: wl_signal,
    pub strip_feedback: wl_signal,
    pub ring_feedback: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_pad {
    pub base: wlr_input_device,
    pub r#impl: *const wlr_tablet_pad_impl,
    pub events: C2Rust_Unnamed_47,
    pub button_count: size_t,
    pub ring_count: size_t,
    pub strip_count: size_t,
    pub groups: wl_list,
    pub paths: wl_array,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
    pub button: wl_signal,
    pub ring: wl_signal,
    pub strip: wl_signal,
    pub attach_tablet: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_manager_v2 {
    pub wl_global: *mut wl_global,
    pub clients: wl_list,
    pub seats: wl_list,
    pub events: C2Rust_Unnamed_49,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
    pub destroy: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_tablet_tool_axes(pub ::core::ffi::c_uint);
impl wlr_tablet_tool_axes {
    pub const WLR_TABLET_TOOL_AXIS_X: Self = Self(1);
    pub const WLR_TABLET_TOOL_AXIS_Y: Self = Self(2);
    pub const WLR_TABLET_TOOL_AXIS_DISTANCE: Self = Self(4);
    pub const WLR_TABLET_TOOL_AXIS_PRESSURE: Self = Self(8);
    pub const WLR_TABLET_TOOL_AXIS_TILT_X: Self = Self(16);
    pub const WLR_TABLET_TOOL_AXIS_TILT_Y: Self = Self(32);
    pub const WLR_TABLET_TOOL_AXIS_ROTATION: Self = Self(64);
    pub const WLR_TABLET_TOOL_AXIS_SLIDER: Self = Self(128);
    pub const WLR_TABLET_TOOL_AXIS_WHEEL: Self = Self(256);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_tool_axis_event {
    pub tablet: *mut wlr_tablet,
    pub tool: *mut wlr_tablet_tool,
    pub time_msec: uint32_t,
    pub updated_axes: uint32_t,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub dx: ::core::ffi::c_double,
    pub dy: ::core::ffi::c_double,
    pub pressure: ::core::ffi::c_double,
    pub distance: ::core::ffi::c_double,
    pub tilt_x: ::core::ffi::c_double,
    pub tilt_y: ::core::ffi::c_double,
    pub rotation: ::core::ffi::c_double,
    pub slider: ::core::ffi::c_double,
    pub wheel_delta: ::core::ffi::c_double,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_tablet_tool_proximity_state(pub ::core::ffi::c_uint);
impl wlr_tablet_tool_proximity_state {
    pub const WLR_TABLET_TOOL_PROXIMITY_OUT: Self = Self(0);
    pub const WLR_TABLET_TOOL_PROXIMITY_IN: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_tool_proximity_event {
    pub tablet: *mut wlr_tablet,
    pub tool: *mut wlr_tablet_tool,
    pub time_msec: uint32_t,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub state: wlr_tablet_tool_proximity_state,
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
pub struct wlr_tablet_tool_tip_event {
    pub tablet: *mut wlr_tablet,
    pub tool: *mut wlr_tablet_tool,
    pub time_msec: uint32_t,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub state: wlr_tablet_tool_tip_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_tool_button_event {
    pub tablet: *mut wlr_tablet,
    pub tool: *mut wlr_tablet_tool,
    pub time_msec: uint32_t,
    pub button: uint32_t,
    pub state: wlr_button_state,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_50,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_52,
    pub WLR_PRIVATE: C2Rust_Unnamed_51,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
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
pub struct C2Rust_Unnamed_52 {
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
    pub events: C2Rust_Unnamed_54,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_53,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_53 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_54 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_55,
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
pub struct C2Rust_Unnamed_55 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_57,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_56,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_56 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_57 {
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
    pub events: C2Rust_Unnamed_59,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_58,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_58 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_59 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_60,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_60 {
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
    pub events: C2Rust_Unnamed_65,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_61,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_61 {
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
    pub events: C2Rust_Unnamed_63,
    pub WLR_PRIVATE: C2Rust_Unnamed_62,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
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
pub struct C2Rust_Unnamed_63 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
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
pub struct C2Rust_Unnamed_65 {
    pub outputs_update: wl_signal,
    pub output_sample: wl_signal,
    pub frame_done: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_surface {
    pub buffer: *mut wlr_scene_buffer,
    pub surface: *mut wlr_surface,
    pub WLR_PRIVATE: C2Rust_Unnamed_66,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
    pub clip: wlr_box,
    pub addon: wlr_addon,
    pub outputs_update: wl_listener,
    pub output_sample: wl_listener,
    pub frame_done: wl_listener,
    pub surface_destroy: wl_listener,
    pub surface_commit: wl_listener,
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
    pub layers: C2Rust_Unnamed_111,
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
    pub events: C2Rust_Unnamed_67,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
    pub new_node: wl_signal,
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
    pub events: C2Rust_Unnamed_68,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
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
    pub events: C2Rust_Unnamed_70,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_69,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_group_handle_v1 {
    pub manager: *mut wlr_ext_workspace_manager_v1,
    pub caps: uint32_t,
    pub events: C2Rust_Unnamed_72,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_71,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_71 {
    pub outputs: wl_list,
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_manager_v1 {
    pub global: *mut wl_global,
    pub groups: wl_list,
    pub workspaces: wl_list,
    pub events: C2Rust_Unnamed_74,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_73,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_73 {
    pub resources: wl_list,
    pub idle_source: *mut wl_event_source,
    pub event_loop: *mut wl_event_loop,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_74 {
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
    pub layers: C2Rust_Unnamed_101,
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
    pub session_lock: C2Rust_Unnamed_81,
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
    pub events: C2Rust_Unnamed_76,
    pub data: *mut ::core::ffi::c_void,
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
    pub new_object: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_text_input_manager_v3 {
    pub global: *mut wl_global,
    pub text_inputs: wl_list,
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
    pub new_text_input: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_manager_v2 {
    pub global: *mut wl_global,
    pub input_methods: wl_list,
    pub events: C2Rust_Unnamed_80,
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
    pub new_input_method: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_81 {
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
    pub events: C2Rust_Unnamed_86,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_84,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_84 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_85,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_85 {
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
pub struct C2Rust_Unnamed_86 {
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
    pub events: C2Rust_Unnamed_88,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_87,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_87 {
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_88 {
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
    pub events: C2Rust_Unnamed_92,
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
    pub create_seat: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_gestures_v1 {
    pub global: *mut wl_global,
    pub swipes: wl_list,
    pub pinches: wl_list,
    pub holds: wl_list,
    pub events: C2Rust_Unnamed_94,
    pub data: *mut ::core::ffi::c_void,
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
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_pointers: wl_list,
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
    pub new_virtual_pointer: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_keyboards: wl_list,
    pub events: C2Rust_Unnamed_98,
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
    pub new_virtual_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_100,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_99,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_99 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_100 {
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_101 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_103,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_102,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_102 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_103 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_108,
    pub events: C2Rust_Unnamed_107,
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
pub struct C2Rust_Unnamed_107 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_108 {
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
    pub events: C2Rust_Unnamed_110,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_109,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_109 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_110 {
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
pub struct C2Rust_Unnamed_111 {
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
    pub events: C2Rust_Unnamed_113,
    pub WLR_PRIVATE: C2Rust_Unnamed_112,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_112 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_113 {
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
    pub events: C2Rust_Unnamed_115,
    pub WLR_PRIVATE: C2Rust_Unnamed_114,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_114 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_115 {
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
pub struct sway_keyboard {
    pub seat_device: *mut sway_seat_device,
    pub wlr: *mut wlr_keyboard,
    pub keymap: *mut xkb_keymap,
    pub effective_layout: xkb_layout_index_t,
    pub repeat_rate: int32_t,
    pub repeat_delay: int32_t,
    pub keyboard_key: wl_listener,
    pub keyboard_modifiers: wl_listener,
    pub state_keysyms_translated: sway_shortcut_state,
    pub state_keysyms_raw: sway_shortcut_state,
    pub state_keycodes: sway_shortcut_state,
    pub state_pressed_sent: sway_shortcut_state,
    pub held_binding: *mut sway_binding,
    pub key_repeat_source: *mut wl_event_source,
    pub repeat_binding: *mut sway_binding,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_shortcut_state {
    pub pressed_keys: [uint32_t; 32],
    pub pressed_keycodes: [uint32_t; 32],
    pub last_keycode: uint32_t,
    pub last_raw_modifiers: uint32_t,
    pub npressed: size_t,
    pub current_key: uint32_t,
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
    pub previous: C2Rust_Unnamed_119,
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
    pub events: C2Rust_Unnamed_117,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_116,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_116 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub synced: wlr_surface_synced,
    pub destroying: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_117 {
    pub set_region: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_constraint_v1_state {
    pub committed: uint32_t,
    pub region: pixman_region32_t,
    pub cursor_hint: C2Rust_Unnamed_118,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_118 {
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
pub struct C2Rust_Unnamed_119 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_input_idle_source(pub ::core::ffi::c_uint);
impl sway_input_idle_source {
    pub const IDLE_SOURCE_KEYBOARD: Self = Self(1);
    pub const IDLE_SOURCE_POINTER: Self = Self(2);
    pub const IDLE_SOURCE_TOUCH: Self = Self(4);
    pub const IDLE_SOURCE_TABLET_PAD: Self = Self(8);
    pub const IDLE_SOURCE_TABLET_TOOL: Self = Self(16);
    pub const IDLE_SOURCE_SWITCH: Self = Self(32);
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
    pub xcursor_theme: C2Rust_Unnamed_120,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_120 {
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
    pub colors: C2Rust_Unnamed_121,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_121 {
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
    pub border_colors: C2Rust_Unnamed_124,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_122,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_122 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_123,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_123 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_124 {
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
pub struct sway_pointer_constraint {
    pub cursor: *mut sway_cursor,
    pub constraint: *mut wlr_pointer_constraint_v1,
    pub set_region: wl_listener,
    pub destroy: wl_listener,
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
pub struct sway_popup_desc {
    pub relative: *mut wlr_scene_node,
    pub view: *mut sway_view,
}
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[inline]
unsafe extern "C" fn wl_signal_add(
    mut signal: *mut wl_signal,
    mut listener: *mut wl_listener,
) {
    wl_list_insert((*signal).listener_list.prev, &raw mut (*listener).link);
}
#[no_mangle]
pub unsafe extern "C" fn node_at_coords(
    mut seat: *mut sway_seat,
    mut lx: ::core::ffi::c_double,
    mut ly: ::core::ffi::c_double,
    mut surface: *mut *mut wlr_surface,
    mut sx: *mut ::core::ffi::c_double,
    mut sy: *mut ::core::ffi::c_double,
) -> *mut sway_node {
    let mut scene_node: *mut wlr_scene_node = ::core::ptr::null_mut::<wlr_scene_node>();
    let mut node: *mut wlr_scene_node = ::core::ptr::null_mut::<wlr_scene_node>();
    node = ((*(*root).layer_tree).children.prev as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut wlr_scene_node
        as *mut wlr_scene_node;
    while &raw mut (*node).link != &raw mut (*(*root).layer_tree).children {
        let mut layer: *mut wlr_scene_tree = wlr_scene_tree_from_node(node);
        let mut non_interactive: bool = !scene_descriptor_try_get(
                &raw mut (*layer).node,
                sway_scene_descriptor_type::SWAY_SCENE_DESC_NON_INTERACTIVE,
            )
            .is_null();
        if !non_interactive {
            scene_node = wlr_scene_node_at(&raw mut (*layer).node, lx, ly, sx, sy);
            if !scene_node.is_null() {
                break;
            }
        }
        node = ((*node).link.prev as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut wlr_scene_node
            as *mut wlr_scene_node;
    }
    if !scene_node.is_null() {
        if (*scene_node).r#type.0 == wlr_scene_node_type::WLR_SCENE_NODE_BUFFER.0 {
            let mut scene_buffer: *mut wlr_scene_buffer = wlr_scene_buffer_from_node(
                scene_node,
            );
            let mut scene_surface: *mut wlr_scene_surface = wlr_scene_surface_try_from_buffer(
                scene_buffer,
            );
            if !scene_surface.is_null() {
                *surface = (*scene_surface).surface;
            }
        }
        let mut current: *mut wlr_scene_node = scene_node;
        loop {
            let mut con: *mut sway_container = scene_descriptor_try_get(
                current,
                sway_scene_descriptor_type::SWAY_SCENE_DESC_CONTAINER,
            ) as *mut sway_container;
            if con.is_null() {
                let mut view: *mut sway_view = scene_descriptor_try_get(
                    current,
                    sway_scene_descriptor_type::SWAY_SCENE_DESC_VIEW,
                ) as *mut sway_view;
                if !view.is_null() {
                    con = (*view).container;
                }
            }
            if con.is_null() {
                let mut popup: *mut sway_popup_desc = scene_descriptor_try_get(
                    current,
                    sway_scene_descriptor_type::SWAY_SCENE_DESC_POPUP,
                ) as *mut sway_popup_desc;
                if !popup.is_null() && !(*popup).view.is_null() {
                    con = (*(*popup).view).container;
                }
            }
            if !con.is_null()
                && ((*con).view.is_null() || !(*(*con).view).surface.is_null())
            {
                return &raw mut (*con).node;
            }
            if !scene_descriptor_try_get(
                    current,
                    sway_scene_descriptor_type::SWAY_SCENE_DESC_LAYER_SHELL,
                )
                .is_null()
            {
                return ::core::ptr::null_mut::<sway_node>();
            }
            if !scene_descriptor_try_get(
                    current,
                    sway_scene_descriptor_type::SWAY_SCENE_DESC_XWAYLAND_UNMANAGED,
                )
                .is_null()
            {
                return ::core::ptr::null_mut::<sway_node>();
            }
            if (*current).parent.is_null() {
                break;
            }
            current = &raw mut (*(*current).parent).node;
        }
    }
    let mut wlr_output: *mut wlr_output = wlr_output_layout_output_at(
        (*root).output_layout,
        lx,
        ly,
    );
    if wlr_output.is_null() {
        return ::core::ptr::null_mut::<sway_node>();
    }
    let mut output: *mut sway_output = (*wlr_output).data as *mut sway_output;
    if output.is_null() || !(*output).enabled {
        return ::core::ptr::null_mut::<sway_node>();
    }
    let mut ws: *mut sway_workspace = output_get_active_workspace(output);
    if ws.is_null() {
        return ::core::ptr::null_mut::<sway_node>();
    }
    return &raw mut (*ws).node;
}
#[no_mangle]
pub unsafe extern "C" fn cursor_rebase(mut cursor: *mut sway_cursor) {
    let mut time_msec: uint32_t = get_current_time_in_msec();
    seatop_rebase((*cursor).seat, time_msec);
}
#[no_mangle]
pub unsafe extern "C" fn cursor_rebase_all() {
    if (*(*root).outputs).length == 0 {
        return;
    }
    let mut seat: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
    seat = ((*server.input).seats.next as *mut ::core::ffi::c_char)
        .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
        as *mut sway_seat;
    while &raw mut (*seat).link != &raw mut (*server.input).seats {
        cursor_rebase((*seat).cursor);
        seat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
            as *mut sway_seat;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cursor_update_image(
    mut cursor: *mut sway_cursor,
    mut node: *mut sway_node,
) {
    if !node.is_null() && (*node).r#type.0 == sway_node_type::N_CONTAINER.0 {
        let mut edge: wlr_edges = find_resize_edge(
            (*node).c2rust_unnamed.sway_container,
            ::core::ptr::null_mut::<wlr_surface>(),
            cursor,
        );
        if edge.0 == wlr_edges::WLR_EDGE_NONE.0 {
            cursor_set_image(
                cursor,
                b"default\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<wl_client>(),
            );
        } else if container_is_floating((*node).c2rust_unnamed.sway_container) {
            cursor_set_image(
                cursor,
                wlr_xcursor_get_resize_name(edge),
                ::core::ptr::null_mut::<wl_client>(),
            );
        } else if edge.0
            & (wlr_edges::WLR_EDGE_LEFT.0 as ::core::ffi::c_int
                | wlr_edges::WLR_EDGE_RIGHT.0 as ::core::ffi::c_int)
                as ::core::ffi::c_uint != 0
        {
            cursor_set_image(
                cursor,
                b"col-resize\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<wl_client>(),
            );
        } else {
            cursor_set_image(
                cursor,
                b"row-resize\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<wl_client>(),
            );
        }
    } else {
        cursor_set_image(
            cursor,
            b"default\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<wl_client>(),
        );
    };
}
unsafe extern "C" fn cursor_hide(mut cursor: *mut sway_cursor) {
    wlr_cursor_unset_image((*cursor).cursor);
    (*cursor).hidden = r#true != 0;
    wlr_seat_pointer_notify_clear_focus((*(*cursor).seat).wlr_seat);
}
unsafe extern "C" fn hide_notify(
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut cursor: *mut sway_cursor = data as *mut sway_cursor;
    cursor_hide(cursor);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cursor_get_timeout(
    mut cursor: *mut sway_cursor,
) -> ::core::ffi::c_int {
    if (*cursor).pressed_button_count > 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    let mut sc: *mut seat_config = seat_get_config((*cursor).seat);
    if sc.is_null() {
        sc = seat_get_config_by_name(b"*\0".as_ptr() as *const ::core::ffi::c_char);
    }
    let mut timeout: ::core::ffi::c_int = if !sc.is_null() {
        (*sc).hide_cursor_timeout
    } else {
        0 as ::core::ffi::c_int
    };
    if timeout < 0 as ::core::ffi::c_int {
        timeout = 0 as ::core::ffi::c_int;
    }
    return timeout;
}
#[no_mangle]
pub unsafe extern "C" fn cursor_notify_key_press(mut cursor: *mut sway_cursor) {
    if (*cursor).hidden {
        return;
    }
    if (*cursor).hide_when_typing.0
        == seat_config_hide_cursor_when_typing::HIDE_WHEN_TYPING_DEFAULT.0
    {
        let mut seat_config: *const seat_config = seat_get_config((*cursor).seat);
        if seat_config.is_null() {
            seat_config = seat_get_config_by_name(
                b"*\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if seat_config.is_null() {
                return;
            }
        }
        (*cursor).hide_when_typing = (*seat_config).hide_cursor_when_typing;
        if (*cursor).hide_when_typing.0
            == seat_config_hide_cursor_when_typing::HIDE_WHEN_TYPING_DEFAULT.0
        {
            (*cursor).hide_when_typing = seat_config_hide_cursor_when_typing::HIDE_WHEN_TYPING_DISABLE;
        }
    }
    if (*cursor).hide_when_typing.0
        == seat_config_hide_cursor_when_typing::HIDE_WHEN_TYPING_ENABLE.0
    {
        cursor_hide(cursor);
    }
}
unsafe extern "C" fn idle_source_from_device(
    mut device: *mut wlr_input_device,
) -> sway_input_idle_source {
    match (*device).r#type {
        wlr_input_device_type::WLR_INPUT_DEVICE_KEYBOARD => {
            return sway_input_idle_source::IDLE_SOURCE_KEYBOARD;
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_POINTER => {
            return sway_input_idle_source::IDLE_SOURCE_POINTER;
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_TOUCH => {
            return sway_input_idle_source::IDLE_SOURCE_TOUCH;
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_TABLET => {
            return sway_input_idle_source::IDLE_SOURCE_TABLET_TOOL;
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_TABLET_PAD => {
            return sway_input_idle_source::IDLE_SOURCE_TABLET_PAD;
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_SWITCH => {
            return sway_input_idle_source::IDLE_SOURCE_SWITCH;
        }
        _ => {}
    }
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn cursor_handle_activity_from_idle_source(
    mut cursor: *mut sway_cursor,
    mut idle_source: sway_input_idle_source,
) {
    wl_event_source_timer_update((*cursor).hide_source, cursor_get_timeout(cursor));
    seat_idle_notify_activity((*cursor).seat, idle_source);
    if idle_source.0 != sway_input_idle_source::IDLE_SOURCE_TOUCH.0 {
        cursor_unhide(cursor);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cursor_handle_activity_from_device(
    mut cursor: *mut sway_cursor,
    mut device: *mut wlr_input_device,
) {
    let mut idle_source: sway_input_idle_source = idle_source_from_device(device);
    cursor_handle_activity_from_idle_source(cursor, idle_source);
}
#[no_mangle]
pub unsafe extern "C" fn cursor_unhide(mut cursor: *mut sway_cursor) {
    if !(*cursor).hidden {
        return;
    }
    (*cursor).hidden = r#false != 0;
    if !(*cursor).image_surface.is_null() {
        cursor_set_image_surface(
            cursor,
            (*cursor).image_surface,
            (*cursor).hotspot_x as int32_t,
            (*cursor).hotspot_y as int32_t,
            (*cursor).image_client,
        );
    } else {
        let mut image: *const ::core::ffi::c_char = (*cursor).image;
        (*cursor).image = ::core::ptr::null::<::core::ffi::c_char>();
        cursor_set_image(cursor, image, (*cursor).image_client);
    }
    cursor_rebase(cursor);
    wl_event_source_timer_update((*cursor).hide_source, cursor_get_timeout(cursor));
}
#[no_mangle]
pub unsafe extern "C" fn pointer_motion(
    mut cursor: *mut sway_cursor,
    mut time_msec: uint32_t,
    mut device: *mut wlr_input_device,
    mut dx: ::core::ffi::c_double,
    mut dy: ::core::ffi::c_double,
    mut dx_unaccel: ::core::ffi::c_double,
    mut dy_unaccel: ::core::ffi::c_double,
) {
    wlr_relative_pointer_manager_v1_send_relative_motion(
        server.relative_pointer_manager,
        (*(*cursor).seat).wlr_seat,
        (time_msec as uint64_t).wrapping_mul(1000 as uint64_t),
        dx,
        dy,
        dx_unaccel,
        dy_unaccel,
    );
    if !(*cursor).active_constraint.is_null()
        && (*device).r#type.0 == wlr_input_device_type::WLR_INPUT_DEVICE_POINTER.0
    {
        let mut surface: *mut wlr_surface = ::core::ptr::null_mut::<wlr_surface>();
        let mut sx: ::core::ffi::c_double = 0.;
        let mut sy: ::core::ffi::c_double = 0.;
        node_at_coords(
            (*cursor).seat,
            (*(*cursor).cursor).x,
            (*(*cursor).cursor).y,
            &raw mut surface,
            &raw mut sx,
            &raw mut sy,
        );
        if (*(*cursor).active_constraint).surface != surface {
            return;
        }
        let mut sx_confined: ::core::ffi::c_double = 0.;
        let mut sy_confined: ::core::ffi::c_double = 0.;
        if !wlr_region_confine(
            &raw mut (*cursor).confine,
            sx,
            sy,
            sx + dx,
            sy + dy,
            &raw mut sx_confined,
            &raw mut sy_confined,
        ) {
            return;
        }
        dx = sx_confined - sx;
        dy = sy_confined - sy;
    }
    wlr_cursor_move((*cursor).cursor, device, dx, dy);
    seatop_pointer_motion((*cursor).seat, time_msec);
}
unsafe extern "C" fn handle_pointer_motion_relative(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(344 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut e: *mut wlr_pointer_motion_event = data as *mut wlr_pointer_motion_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*e).pointer).base);
    pointer_motion(
        cursor,
        (*e).time_msec,
        &raw mut (*(*e).pointer).base,
        (*e).delta_x,
        (*e).delta_y,
        (*e).unaccel_dx,
        (*e).unaccel_dy,
    );
}
unsafe extern "C" fn handle_pointer_motion_absolute(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(368 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_pointer_motion_absolute_event = data
        as *mut wlr_pointer_motion_absolute_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).pointer).base);
    let mut lx: ::core::ffi::c_double = 0.;
    let mut ly: ::core::ffi::c_double = 0.;
    wlr_cursor_absolute_to_layout_coords(
        (*cursor).cursor,
        &raw mut (*(*event).pointer).base,
        (*event).x,
        (*event).y,
        &raw mut lx,
        &raw mut ly,
    );
    let mut dx: ::core::ffi::c_double = lx - (*(*cursor).cursor).x;
    let mut dy: ::core::ffi::c_double = ly - (*(*cursor).cursor).y;
    pointer_motion(
        cursor,
        (*event).time_msec,
        &raw mut (*(*event).pointer).base,
        dx,
        dy,
        dx,
        dy,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dispatch_cursor_button(
    mut cursor: *mut sway_cursor,
    mut device: *mut wlr_input_device,
    mut time_msec: uint32_t,
    mut button: uint32_t,
    mut state: wl_pointer_button_state,
) {
    if time_msec == 0 as uint32_t {
        time_msec = get_current_time_in_msec();
    }
    seatop_button((*cursor).seat, time_msec, device, button, state);
}
unsafe extern "C" fn handle_pointer_button(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(392 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_pointer_button_event = data as *mut wlr_pointer_button_event;
    if (*event).state.0 == wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED.0 {
        (*cursor).pressed_button_count = (*cursor).pressed_button_count.wrapping_add(1);
    } else if (*cursor).pressed_button_count > 0 as size_t {
        (*cursor).pressed_button_count = (*cursor).pressed_button_count.wrapping_sub(1);
    } else {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Pressed button count was wrong\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/cursor.c\0".as_ptr() as *const ::core::ffi::c_char,
            372 as ::core::ffi::c_int,
        );
    }
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).pointer).base);
    dispatch_cursor_button(
        cursor,
        &raw mut (*(*event).pointer).base,
        (*event).time_msec,
        (*event).button,
        (*event).state,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dispatch_cursor_axis(
    mut cursor: *mut sway_cursor,
    mut event: *mut wlr_pointer_axis_event,
) {
    seatop_pointer_axis((*cursor).seat, event);
}
unsafe extern "C" fn handle_pointer_axis(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(416 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_pointer_axis_event = data as *mut wlr_pointer_axis_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).pointer).base);
    dispatch_cursor_axis(cursor, event);
}
unsafe extern "C" fn handle_pointer_frame(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(440 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    wlr_seat_pointer_notify_frame((*(*cursor).seat).wlr_seat);
}
unsafe extern "C" fn handle_touch_down(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(464 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_touch_down_event = data as *mut wlr_touch_down_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).touch).base);
    cursor_hide(cursor);
    let mut seat: *mut sway_seat = (*cursor).seat;
    let mut lx: ::core::ffi::c_double = 0.;
    let mut ly: ::core::ffi::c_double = 0.;
    wlr_cursor_absolute_to_layout_coords(
        (*cursor).cursor,
        &raw mut (*(*event).touch).base,
        (*event).x,
        (*event).y,
        &raw mut lx,
        &raw mut ly,
    );
    (*seat).touch_id = (*event).touch_id;
    (*seat).touch_x = lx;
    (*seat).touch_y = ly;
    seatop_touch_down(seat, event, lx, ly);
}
unsafe extern "C" fn handle_touch_up(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(488 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_touch_up_event = data as *mut wlr_touch_up_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).touch).base);
    let mut seat: *mut sway_seat = (*cursor).seat;
    if (*cursor).simulating_pointer_from_touch {
        if (*cursor).pointer_touch_id == (*(*cursor).seat).touch_id {
            (*cursor).pointer_touch_up = r#true != 0;
            dispatch_cursor_button(
                cursor,
                &raw mut (*(*event).touch).base,
                (*event).time_msec,
                BTN_LEFT as uint32_t,
                wl_pointer_button_state::WL_POINTER_BUTTON_STATE_RELEASED,
            );
        }
    } else {
        seatop_touch_up(seat, event);
    };
}
unsafe extern "C" fn handle_touch_cancel(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(512 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_touch_cancel_event = data as *mut wlr_touch_cancel_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).touch).base);
    let mut seat: *mut sway_seat = (*cursor).seat;
    if (*cursor).simulating_pointer_from_touch {
        if (*cursor).pointer_touch_id == (*(*cursor).seat).touch_id {
            (*cursor).pointer_touch_up = r#true != 0;
            dispatch_cursor_button(
                cursor,
                &raw mut (*(*event).touch).base,
                (*event).time_msec,
                BTN_LEFT as uint32_t,
                wl_pointer_button_state::WL_POINTER_BUTTON_STATE_RELEASED,
            );
        }
    } else {
        seatop_touch_cancel(seat, event);
    };
}
unsafe extern "C" fn handle_touch_motion(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(536 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_touch_motion_event = data as *mut wlr_touch_motion_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).touch).base);
    let mut seat: *mut sway_seat = (*cursor).seat;
    let mut lx: ::core::ffi::c_double = 0.;
    let mut ly: ::core::ffi::c_double = 0.;
    wlr_cursor_absolute_to_layout_coords(
        (*cursor).cursor,
        &raw mut (*(*event).touch).base,
        (*event).x,
        (*event).y,
        &raw mut lx,
        &raw mut ly,
    );
    if (*seat).touch_id == (*event).touch_id {
        (*seat).touch_x = lx;
        (*seat).touch_y = ly;
        drag_icons_update_position(seat);
    }
    if (*cursor).simulating_pointer_from_touch {
        if (*seat).touch_id == (*cursor).pointer_touch_id {
            let mut dx: ::core::ffi::c_double = 0.;
            let mut dy: ::core::ffi::c_double = 0.;
            dx = lx - (*(*cursor).cursor).x;
            dy = ly - (*(*cursor).cursor).y;
            pointer_motion(
                cursor,
                (*event).time_msec,
                &raw mut (*(*event).touch).base,
                dx,
                dy,
                dx,
                dy,
            );
        }
    } else {
        seatop_touch_motion(seat, event, lx, ly);
    };
}
unsafe extern "C" fn handle_touch_frame(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(560 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut wlr_seat: *mut wlr_seat = (*(*cursor).seat).wlr_seat;
    if (*cursor).simulating_pointer_from_touch {
        wlr_seat_pointer_notify_frame(wlr_seat);
        if (*cursor).pointer_touch_up {
            (*cursor).pointer_touch_up = r#false != 0;
            (*cursor).simulating_pointer_from_touch = r#false != 0;
        }
    } else {
        wlr_seat_touch_notify_frame(wlr_seat);
    };
}
unsafe extern "C" fn apply_mapping_from_coord(
    mut low: ::core::ffi::c_double,
    mut high: ::core::ffi::c_double,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if value.is_nan() as i32 != 0 {
        return value;
    }
    return (value - low) / (high - low);
}
unsafe extern "C" fn apply_mapping_from_region(
    mut device: *mut wlr_input_device,
    mut region: *mut input_config_mapped_from_region,
    mut x: *mut ::core::ffi::c_double,
    mut y: *mut ::core::ffi::c_double,
) {
    let mut x1: ::core::ffi::c_double = (*region).x1;
    let mut x2: ::core::ffi::c_double = (*region).x2;
    let mut y1: ::core::ffi::c_double = (*region).y1;
    let mut y2: ::core::ffi::c_double = (*region).y2;
    if (*region).mm as ::core::ffi::c_int != 0
        && (*device).r#type.0 == wlr_input_device_type::WLR_INPUT_DEVICE_TABLET.0
    {
        let mut tablet: *mut wlr_tablet = wlr_tablet_from_input_device(device);
        if (*tablet).width_mm == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || (*tablet).height_mm == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            return;
        }
        x1 /= (*tablet).width_mm;
        x2 /= (*tablet).width_mm;
        y1 /= (*tablet).height_mm;
        y2 /= (*tablet).height_mm;
    }
    *x = apply_mapping_from_coord(x1, x2, *x);
    *y = apply_mapping_from_coord(y1, y2, *y);
}
unsafe extern "C" fn handle_tablet_tool_position(
    mut cursor: *mut sway_cursor,
    mut tool: *mut sway_tablet_tool,
    mut change_x: bool,
    mut change_y: bool,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut dx: ::core::ffi::c_double,
    mut dy: ::core::ffi::c_double,
    mut time_msec: int32_t,
) {
    if !change_x && !change_y {
        return;
    }
    let mut tablet: *mut sway_tablet = (*tool).tablet;
    let mut input_device: *mut sway_input_device = (*(*tablet).seat_device).input_device;
    let mut ic: *mut input_config = input_device_get_config(input_device);
    if !ic.is_null() && !(*ic).mapped_from_region.is_null() {
        apply_mapping_from_region(
            (*input_device).wlr_device,
            (*ic).mapped_from_region,
            &raw mut x,
            &raw mut y,
        );
    }
    match (*tool).mode {
        sway_tablet_tool_mode::SWAY_TABLET_TOOL_MODE_ABSOLUTE => {
            wlr_cursor_warp_absolute(
                (*cursor).cursor,
                (*input_device).wlr_device,
                if change_x as ::core::ffi::c_int != 0 {
                    x
                } else {
                    ::core::f32::NAN as ::core::ffi::c_double
                },
                if change_y as ::core::ffi::c_int != 0 {
                    y
                } else {
                    ::core::f32::NAN as ::core::ffi::c_double
                },
            );
        }
        sway_tablet_tool_mode::SWAY_TABLET_TOOL_MODE_RELATIVE => {
            wlr_cursor_move((*cursor).cursor, (*input_device).wlr_device, dx, dy);
        }
        _ => {}
    }
    let mut sx: ::core::ffi::c_double = 0.;
    let mut sy: ::core::ffi::c_double = 0.;
    let mut surface: *mut wlr_surface = ::core::ptr::null_mut::<wlr_surface>();
    let mut seat: *mut sway_seat = (*cursor).seat;
    node_at_coords(
        seat,
        (*(*cursor).cursor).x,
        (*(*cursor).cursor).y,
        &raw mut surface,
        &raw mut sx,
        &raw mut sy,
    );
    if !(*cursor).simulating_pointer_from_tool_tip
        && (!surface.is_null()
            && wlr_surface_accepts_tablet_v2(surface, (*tablet).tablet_v2)
                as ::core::ffi::c_int != 0
            || wlr_tablet_tool_v2_has_implicit_grab((*tool).tablet_v2_tool)
                as ::core::ffi::c_int != 0)
    {
        seatop_tablet_tool_motion(seat, tool, time_msec as uint32_t);
    } else {
        wlr_tablet_v2_tablet_tool_notify_proximity_out((*tool).tablet_v2_tool);
        pointer_motion(
            cursor,
            time_msec as uint32_t,
            (*input_device).wlr_device,
            dx,
            dy,
            dx,
            dy,
        );
        wlr_seat_pointer_notify_frame((*(*cursor).seat).wlr_seat);
    };
}
unsafe extern "C" fn handle_tool_axis(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(592 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_tablet_tool_axis_event = data
        as *mut wlr_tablet_tool_axis_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).tablet).base);
    let mut sway_tool: *mut sway_tablet_tool = (*(*event).tool).data
        as *mut sway_tablet_tool;
    if sway_tool.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] tool axis before proximity\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/cursor.c\0".as_ptr() as *const ::core::ffi::c_char,
            592 as ::core::ffi::c_int,
        );
        return;
    }
    handle_tablet_tool_position(
        cursor,
        sway_tool,
        (*event).updated_axes
            & wlr_tablet_tool_axes::WLR_TABLET_TOOL_AXIS_X.0 as ::core::ffi::c_int
                as uint32_t != 0,
        (*event).updated_axes
            & wlr_tablet_tool_axes::WLR_TABLET_TOOL_AXIS_Y.0 as ::core::ffi::c_int
                as uint32_t != 0,
        (*event).x,
        (*event).y,
        (*event).dx,
        (*event).dy,
        (*event).time_msec as int32_t,
    );
    if (*event).updated_axes
        & wlr_tablet_tool_axes::WLR_TABLET_TOOL_AXIS_PRESSURE.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        wlr_tablet_v2_tablet_tool_notify_pressure(
            (*sway_tool).tablet_v2_tool,
            (*event).pressure,
        );
    }
    if (*event).updated_axes
        & wlr_tablet_tool_axes::WLR_TABLET_TOOL_AXIS_DISTANCE.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        wlr_tablet_v2_tablet_tool_notify_distance(
            (*sway_tool).tablet_v2_tool,
            (*event).distance,
        );
    }
    if (*event).updated_axes
        & wlr_tablet_tool_axes::WLR_TABLET_TOOL_AXIS_TILT_X.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        (*sway_tool).tilt_x = (*event).tilt_x;
    }
    if (*event).updated_axes
        & wlr_tablet_tool_axes::WLR_TABLET_TOOL_AXIS_TILT_Y.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        (*sway_tool).tilt_y = (*event).tilt_y;
    }
    if (*event).updated_axes
        & (wlr_tablet_tool_axes::WLR_TABLET_TOOL_AXIS_TILT_X.0 as ::core::ffi::c_int
            | wlr_tablet_tool_axes::WLR_TABLET_TOOL_AXIS_TILT_Y.0 as ::core::ffi::c_int)
            as uint32_t != 0
    {
        wlr_tablet_v2_tablet_tool_notify_tilt(
            (*sway_tool).tablet_v2_tool,
            (*sway_tool).tilt_x,
            (*sway_tool).tilt_y,
        );
    }
    if (*event).updated_axes
        & wlr_tablet_tool_axes::WLR_TABLET_TOOL_AXIS_ROTATION.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        wlr_tablet_v2_tablet_tool_notify_rotation(
            (*sway_tool).tablet_v2_tool,
            (*event).rotation,
        );
    }
    if (*event).updated_axes
        & wlr_tablet_tool_axes::WLR_TABLET_TOOL_AXIS_SLIDER.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        wlr_tablet_v2_tablet_tool_notify_slider(
            (*sway_tool).tablet_v2_tool,
            (*event).slider,
        );
    }
    if (*event).updated_axes
        & wlr_tablet_tool_axes::WLR_TABLET_TOOL_AXIS_WHEEL.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        wlr_tablet_v2_tablet_tool_notify_wheel(
            (*sway_tool).tablet_v2_tool,
            (*event).wheel_delta,
            0 as int32_t,
        );
    }
}
unsafe extern "C" fn handle_tool_tip(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(616 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_tablet_tool_tip_event = data
        as *mut wlr_tablet_tool_tip_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).tablet).base);
    let mut sway_tool: *mut sway_tablet_tool = (*(*event).tool).data
        as *mut sway_tablet_tool;
    if sway_tool.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] tool tip before proximity\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/cursor.c\0".as_ptr() as *const ::core::ffi::c_char,
            648 as ::core::ffi::c_int,
        );
        return;
    }
    let mut tablet_v2: *mut wlr_tablet_v2_tablet = (*(*sway_tool).tablet).tablet_v2;
    let mut seat: *mut sway_seat = (*cursor).seat;
    let mut sx: ::core::ffi::c_double = 0.;
    let mut sy: ::core::ffi::c_double = 0.;
    let mut surface: *mut wlr_surface = ::core::ptr::null_mut::<wlr_surface>();
    node_at_coords(
        seat,
        (*(*cursor).cursor).x,
        (*(*cursor).cursor).y,
        &raw mut surface,
        &raw mut sx,
        &raw mut sy,
    );
    if (*cursor).simulating_pointer_from_tool_tip as ::core::ffi::c_int != 0
        && (*event).state.0 == wlr_tablet_tool_tip_state::WLR_TABLET_TOOL_TIP_UP.0
    {
        (*cursor).simulating_pointer_from_tool_tip = r#false != 0;
        dispatch_cursor_button(
            cursor,
            &raw mut (*(*event).tablet).base,
            (*event).time_msec,
            BTN_LEFT as uint32_t,
            wl_pointer_button_state::WL_POINTER_BUTTON_STATE_RELEASED,
        );
        wlr_seat_pointer_notify_frame((*(*cursor).seat).wlr_seat);
    } else if surface.is_null() || !wlr_surface_accepts_tablet_v2(surface, tablet_v2) {
        if (*event).state.0 == wlr_tablet_tool_tip_state::WLR_TABLET_TOOL_TIP_UP.0 {
            seatop_tablet_tool_tip(
                seat,
                sway_tool,
                (*event).time_msec,
                wlr_tablet_tool_tip_state::WLR_TABLET_TOOL_TIP_UP,
            );
        } else {
            (*cursor).simulating_pointer_from_tool_tip = r#true != 0;
            dispatch_cursor_button(
                cursor,
                &raw mut (*(*event).tablet).base,
                (*event).time_msec,
                BTN_LEFT as uint32_t,
                wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED,
            );
            wlr_seat_pointer_notify_frame((*(*cursor).seat).wlr_seat);
        }
    } else {
        seatop_tablet_tool_tip(seat, sway_tool, (*event).time_msec, (*event).state);
    };
}
unsafe extern "C" fn get_tablet_for_device(
    mut cursor: *mut sway_cursor,
    mut device: *mut wlr_input_device,
) -> *mut sway_tablet {
    let mut tablet: *mut sway_tablet = ::core::ptr::null_mut::<sway_tablet>();
    tablet = ((*cursor).tablets.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut sway_tablet
        as *mut sway_tablet;
    while &raw mut (*tablet).link != &raw mut (*cursor).tablets {
        if (*(*(*tablet).seat_device).input_device).wlr_device == device {
            return tablet;
        }
        tablet = ((*tablet).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut sway_tablet
            as *mut sway_tablet;
    }
    return ::core::ptr::null_mut::<sway_tablet>();
}
unsafe extern "C" fn handle_tool_proximity(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(640 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_tablet_tool_proximity_event = data
        as *mut wlr_tablet_tool_proximity_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).tablet).base);
    let mut tool: *mut wlr_tablet_tool = (*event).tool;
    if (*tool).data.is_null() {
        let mut tablet: *mut sway_tablet = get_tablet_for_device(
            cursor,
            &raw mut (*(*event).tablet).base,
        );
        if tablet.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] no tablet for tablet tool\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/cursor.c\0".as_ptr() as *const ::core::ffi::c_char,
                707 as ::core::ffi::c_int,
            );
            return;
        }
        sway_tablet_tool_configure(tablet, tool);
    }
    let mut sway_tool: *mut sway_tablet_tool = (*tool).data as *mut sway_tablet_tool;
    if sway_tool.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] tablet tool not initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/cursor.c\0".as_ptr() as *const ::core::ffi::c_char,
            715 as ::core::ffi::c_int,
        );
        return;
    }
    if (*event).state.0
        == wlr_tablet_tool_proximity_state::WLR_TABLET_TOOL_PROXIMITY_OUT.0
    {
        wlr_tablet_v2_tablet_tool_notify_proximity_out((*sway_tool).tablet_v2_tool);
        return;
    }
    handle_tablet_tool_position(
        cursor,
        sway_tool,
        r#true != 0,
        r#true != 0,
        (*event).x,
        (*event).y,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*event).time_msec as int32_t,
    );
}
unsafe extern "C" fn handle_tool_button(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(664 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_tablet_tool_button_event = data
        as *mut wlr_tablet_tool_button_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).tablet).base);
    let mut sway_tool: *mut sway_tablet_tool = (*(*event).tool).data
        as *mut sway_tablet_tool;
    if sway_tool.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] tool button before proximity\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/cursor.c\0".as_ptr() as *const ::core::ffi::c_char,
            735 as ::core::ffi::c_int,
        );
        return;
    }
    let mut tablet_v2: *mut wlr_tablet_v2_tablet = (*(*sway_tool).tablet).tablet_v2;
    let mut sx: ::core::ffi::c_double = 0.;
    let mut sy: ::core::ffi::c_double = 0.;
    let mut surface: *mut wlr_surface = ::core::ptr::null_mut::<wlr_surface>();
    node_at_coords(
        (*cursor).seat,
        (*(*cursor).cursor).x,
        (*(*cursor).cursor).y,
        &raw mut surface,
        &raw mut sx,
        &raw mut sy,
    );
    let mut keyboard: *mut wlr_keyboard = wlr_seat_get_keyboard(
        (*(*cursor).seat).wlr_seat,
    );
    let mut modifiers: uint32_t = if !keyboard.is_null() {
        wlr_keyboard_get_modifiers(keyboard)
    } else {
        0 as uint32_t
    };
    let mut mod_pressed: bool = modifiers & (*config).floating_mod != 0;
    let mut surface_supports_tablet_events: bool = !surface.is_null()
        && wlr_surface_accepts_tablet_v2(surface, tablet_v2) as ::core::ffi::c_int != 0;
    let mut should_simulate_pointer: bool = mod_pressed as ::core::ffi::c_int != 0
        || !surface_supports_tablet_events;
    if (*cursor).tool_buttons > 0 as uint32_t
        && (*cursor).simulating_pointer_from_tool_button as ::core::ffi::c_int != 0
        || (*cursor).tool_buttons == 0 as uint32_t
            && should_simulate_pointer as ::core::ffi::c_int != 0
    {
        (*cursor).simulating_pointer_from_tool_button = r#true != 0;
        match (*event).state {
            wlr_button_state::WLR_BUTTON_PRESSED => {
                if (*cursor).tool_buttons == 0 as uint32_t {
                    dispatch_cursor_button(
                        cursor,
                        &raw mut (*(*event).tablet).base,
                        (*event).time_msec,
                        BTN_RIGHT as uint32_t,
                        wl_pointer_button_state::WL_POINTER_BUTTON_STATE_PRESSED,
                    );
                }
            }
            wlr_button_state::WLR_BUTTON_RELEASED => {
                if (*cursor).tool_buttons <= 1 as uint32_t {
                    dispatch_cursor_button(
                        cursor,
                        &raw mut (*(*event).tablet).base,
                        (*event).time_msec,
                        BTN_RIGHT as uint32_t,
                        wl_pointer_button_state::WL_POINTER_BUTTON_STATE_RELEASED,
                    );
                }
            }
            _ => {}
        }
        wlr_seat_pointer_notify_frame((*(*cursor).seat).wlr_seat);
    } else {
        (*cursor).simulating_pointer_from_tool_button = r#false != 0;
        wlr_tablet_v2_tablet_tool_notify_button(
            (*sway_tool).tablet_v2_tool,
            (*event).button,
            zwp_tablet_pad_v2_button_state((*event).state.0),
        );
    }
    match (*event).state {
        wlr_button_state::WLR_BUTTON_PRESSED => {
            (*cursor).tool_buttons = (*cursor).tool_buttons.wrapping_add(1);
        }
        wlr_button_state::WLR_BUTTON_RELEASED => {
            if (*cursor).tool_buttons == 0 as uint32_t {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] inconsistent tablet tool button events\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/input/cursor.c\0".as_ptr() as *const ::core::ffi::c_char,
                    805 as ::core::ffi::c_int,
                );
            } else {
                (*cursor).tool_buttons = (*cursor).tool_buttons.wrapping_sub(1);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn check_constraint_region(mut cursor: *mut sway_cursor) {
    let mut constraint: *mut wlr_pointer_constraint_v1 = (*cursor).active_constraint;
    let mut region: *mut pixman_region32_t = &raw mut (*constraint).region;
    let mut view: *mut sway_view = view_from_wlr_surface((*constraint).surface);
    if (*cursor).active_confine_requires_warp as ::core::ffi::c_int != 0
        && !view.is_null()
    {
        (*cursor).active_confine_requires_warp = r#false != 0;
        let mut con: *mut sway_container = (*view).container;
        let mut sx: ::core::ffi::c_double = (*(*cursor).cursor).x
            - (*con).pending.content_x + (*view).geometry.x as ::core::ffi::c_double;
        let mut sy: ::core::ffi::c_double = (*(*cursor).cursor).y
            - (*con).pending.content_y + (*view).geometry.y as ::core::ffi::c_double;
        if pixman_region32_contains_point(
            region,
            floor(sx) as ::core::ffi::c_int,
            floor(sy) as ::core::ffi::c_int,
            ::core::ptr::null_mut::<pixman_box32_t>(),
        ) == 0
        {
            let mut nboxes: ::core::ffi::c_int = 0;
            let mut boxes: *mut pixman_box32_t = pixman_region32_rectangles(
                region,
                &raw mut nboxes,
            );
            if nboxes > 0 as ::core::ffi::c_int {
                let mut sx_0: ::core::ffi::c_double = ((*boxes.offset(0isize)).x1
                    + (*boxes.offset(0isize)).x2) as ::core::ffi::c_double / 2.0f64;
                let mut sy_0: ::core::ffi::c_double = ((*boxes.offset(0isize)).y1
                    + (*boxes.offset(0isize)).y2) as ::core::ffi::c_double / 2.0f64;
                wlr_cursor_warp_closest(
                    (*cursor).cursor,
                    ::core::ptr::null_mut::<wlr_input_device>(),
                    sx_0 + (*con).pending.content_x
                        - (*view).geometry.x as ::core::ffi::c_double,
                    sy_0 + (*con).pending.content_y
                        - (*view).geometry.y as ::core::ffi::c_double,
                );
                cursor_rebase(cursor);
            }
        }
    }
    if (*constraint).r#type.0
        == wlr_pointer_constraint_v1_type::WLR_POINTER_CONSTRAINT_V1_CONFINED.0
    {
        pixman_region32_copy(&raw mut (*cursor).confine, region);
    } else {
        pixman_region32_clear(&raw mut (*cursor).confine);
    };
}
unsafe extern "C" fn handle_constraint_commit(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(744 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    check_constraint_region(cursor);
}
unsafe extern "C" fn handle_pointer_constraint_set_region(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut sway_constraint: *mut sway_pointer_constraint = (listener
        as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_pointer_constraint;
    let mut cursor: *mut sway_cursor = (*sway_constraint).cursor;
    (*cursor).active_confine_requires_warp = r#true != 0;
}
unsafe extern "C" fn handle_request_pointer_set_cursor(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(696 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    if !seatop_allows_set_cursor((*cursor).seat) {
        return;
    }
    let mut event: *mut wlr_seat_pointer_request_set_cursor_event = data
        as *mut wlr_seat_pointer_request_set_cursor_event;
    let mut focused_client: *mut wl_client = ::core::ptr::null_mut::<wl_client>();
    let mut focused_surface: *mut wlr_surface = (*(*(*cursor).seat).wlr_seat)
        .pointer_state
        .focused_surface;
    if !focused_surface.is_null() {
        focused_client = wl_resource_get_client((*focused_surface).resource);
    }
    if focused_client.is_null() || (*(*event).seat_client).client != focused_client {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] denying request to set cursor from unfocused client\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/cursor.c\0".as_ptr() as *const ::core::ffi::c_char,
            886 as ::core::ffi::c_int,
        );
        return;
    }
    cursor_set_image_surface(
        cursor,
        (*event).surface,
        (*event).hotspot_x,
        (*event).hotspot_y,
        focused_client,
    );
}
unsafe extern "C" fn handle_pointer_hold_begin(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(152 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_pointer_hold_begin_event = data
        as *mut wlr_pointer_hold_begin_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).pointer).base);
    seatop_hold_begin((*cursor).seat, event);
}
unsafe extern "C" fn handle_pointer_hold_end(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(176 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_pointer_hold_end_event = data
        as *mut wlr_pointer_hold_end_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).pointer).base);
    seatop_hold_end((*cursor).seat, event);
}
unsafe extern "C" fn handle_pointer_pinch_begin(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(200 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_pointer_pinch_begin_event = data
        as *mut wlr_pointer_pinch_begin_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).pointer).base);
    seatop_pinch_begin((*cursor).seat, event);
}
unsafe extern "C" fn handle_pointer_pinch_update(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(224 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_pointer_pinch_update_event = data
        as *mut wlr_pointer_pinch_update_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).pointer).base);
    seatop_pinch_update((*cursor).seat, event);
}
unsafe extern "C" fn handle_pointer_pinch_end(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(248 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_pointer_pinch_end_event = data
        as *mut wlr_pointer_pinch_end_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).pointer).base);
    seatop_pinch_end((*cursor).seat, event);
}
unsafe extern "C" fn handle_pointer_swipe_begin(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(272 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_pointer_swipe_begin_event = data
        as *mut wlr_pointer_swipe_begin_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).pointer).base);
    seatop_swipe_begin((*cursor).seat, event);
}
unsafe extern "C" fn handle_pointer_swipe_update(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(296 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_pointer_swipe_update_event = data
        as *mut wlr_pointer_swipe_update_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).pointer).base);
    seatop_swipe_update((*cursor).seat, event);
}
unsafe extern "C" fn handle_pointer_swipe_end(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(320 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    let mut event: *mut wlr_pointer_swipe_end_event = data
        as *mut wlr_pointer_swipe_end_event;
    cursor_handle_activity_from_device(cursor, &raw mut (*(*event).pointer).base);
    seatop_swipe_end((*cursor).seat, event);
}
unsafe extern "C" fn handle_image_surface_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut cursor: *mut sway_cursor = (listener as *mut ::core::ffi::c_char)
        .offset(-(720 as ::core::ffi::c_ulong as isize)) as *mut sway_cursor;
    cursor_set_image(
        cursor,
        ::core::ptr::null::<::core::ffi::c_char>(),
        (*cursor).image_client,
    );
    cursor_rebase(cursor);
}
unsafe extern "C" fn set_image_surface(
    mut cursor: *mut sway_cursor,
    mut surface: *mut wlr_surface,
) {
    wl_list_remove(&raw mut (*cursor).image_surface_destroy.link);
    (*cursor).image_surface = surface;
    if !surface.is_null() {
        wl_signal_add(
            &raw mut (*surface).events.destroy,
            &raw mut (*cursor).image_surface_destroy,
        );
    } else {
        wl_list_init(&raw mut (*cursor).image_surface_destroy.link);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cursor_set_image(
    mut cursor: *mut sway_cursor,
    mut image: *const ::core::ffi::c_char,
    mut client: *mut wl_client,
) {
    if (*(*(*cursor).seat).wlr_seat).capabilities
        & wl_seat_capability::WL_SEAT_CAPABILITY_POINTER.0 as ::core::ffi::c_int
            as uint32_t == 0
    {
        return;
    }
    let mut current_image: *const ::core::ffi::c_char = (*cursor).image;
    set_image_surface(cursor, ::core::ptr::null_mut::<wlr_surface>());
    (*cursor).image = image;
    (*cursor).hotspot_y = 0 as ::core::ffi::c_int;
    (*cursor).hotspot_x = (*cursor).hotspot_y;
    (*cursor).image_client = client;
    if (*cursor).hidden {
        return;
    }
    if image.is_null() {
        wlr_cursor_unset_image((*cursor).cursor);
    } else if current_image.is_null()
        || strcmp(current_image, image) != 0 as ::core::ffi::c_int
    {
        wlr_cursor_set_xcursor((*cursor).cursor, (*cursor).xcursor_manager, image);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cursor_set_image_surface(
    mut cursor: *mut sway_cursor,
    mut surface: *mut wlr_surface,
    mut hotspot_x: int32_t,
    mut hotspot_y: int32_t,
    mut client: *mut wl_client,
) {
    if (*(*(*cursor).seat).wlr_seat).capabilities
        & wl_seat_capability::WL_SEAT_CAPABILITY_POINTER.0 as ::core::ffi::c_int
            as uint32_t == 0
    {
        return;
    }
    set_image_surface(cursor, surface);
    (*cursor).image = ::core::ptr::null::<::core::ffi::c_char>();
    (*cursor).hotspot_x = hotspot_x as ::core::ffi::c_int;
    (*cursor).hotspot_y = hotspot_y as ::core::ffi::c_int;
    (*cursor).image_client = client;
    if (*cursor).hidden {
        return;
    }
    wlr_cursor_set_surface((*cursor).cursor, surface, hotspot_x, hotspot_y);
}
#[no_mangle]
pub unsafe extern "C" fn sway_cursor_destroy(mut cursor: *mut sway_cursor) {
    if cursor.is_null() {
        return;
    }
    wl_event_source_remove((*cursor).hide_source);
    wl_list_remove(&raw mut (*cursor).image_surface_destroy.link);
    wl_list_remove(&raw mut (*cursor).hold_begin.link);
    wl_list_remove(&raw mut (*cursor).hold_end.link);
    wl_list_remove(&raw mut (*cursor).pinch_begin.link);
    wl_list_remove(&raw mut (*cursor).pinch_update.link);
    wl_list_remove(&raw mut (*cursor).pinch_end.link);
    wl_list_remove(&raw mut (*cursor).swipe_begin.link);
    wl_list_remove(&raw mut (*cursor).swipe_update.link);
    wl_list_remove(&raw mut (*cursor).swipe_end.link);
    wl_list_remove(&raw mut (*cursor).motion.link);
    wl_list_remove(&raw mut (*cursor).motion_absolute.link);
    wl_list_remove(&raw mut (*cursor).button.link);
    wl_list_remove(&raw mut (*cursor).axis.link);
    wl_list_remove(&raw mut (*cursor).frame.link);
    wl_list_remove(&raw mut (*cursor).touch_down.link);
    wl_list_remove(&raw mut (*cursor).touch_up.link);
    wl_list_remove(&raw mut (*cursor).touch_cancel.link);
    wl_list_remove(&raw mut (*cursor).touch_motion.link);
    wl_list_remove(&raw mut (*cursor).touch_frame.link);
    wl_list_remove(&raw mut (*cursor).tool_axis.link);
    wl_list_remove(&raw mut (*cursor).tool_tip.link);
    wl_list_remove(&raw mut (*cursor).tool_proximity.link);
    wl_list_remove(&raw mut (*cursor).tool_button.link);
    wl_list_remove(&raw mut (*cursor).request_set_cursor.link);
    wlr_xcursor_manager_destroy((*cursor).xcursor_manager);
    wlr_cursor_destroy((*cursor).cursor);
    free(cursor as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sway_cursor_create(
    mut seat: *mut sway_seat,
) -> *mut sway_cursor {
    let mut cursor: *mut sway_cursor = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_cursor>(),
    ) as *mut sway_cursor;
    if !_sway_assert(
        !cursor.is_null(),
        b"[%s:%d] %s:could not allocate sway cursor\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/cursor.c\0".as_ptr() as *const ::core::ffi::c_char,
        1059 as ::core::ffi::c_int,
        b"struct sway_cursor *sway_cursor_create(struct sway_seat *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return ::core::ptr::null_mut::<sway_cursor>();
    }
    let mut wlr_cursor: *mut wlr_cursor = wlr_cursor_create();
    if !_sway_assert(
        !wlr_cursor.is_null(),
        b"[%s:%d] %s:could not allocate wlr cursor\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/cursor.c\0".as_ptr() as *const ::core::ffi::c_char,
        1064 as ::core::ffi::c_int,
        b"struct sway_cursor *sway_cursor_create(struct sway_seat *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        free(cursor as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<sway_cursor>();
    }
    (*cursor).previous.x = (*wlr_cursor).x;
    (*cursor).previous.y = (*wlr_cursor).y;
    (*cursor).seat = seat;
    wlr_cursor_attach_output_layout(wlr_cursor, (*root).output_layout);
    (*cursor).hide_source = wl_event_loop_add_timer(
        server.wl_event_loop,
        Some(
            hide_notify
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        cursor as *mut ::core::ffi::c_void,
    );
    wl_list_init(&raw mut (*cursor).image_surface_destroy.link);
    (*cursor).image_surface_destroy.notify = Some(
        handle_image_surface_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.hold_begin,
        &raw mut (*cursor).hold_begin,
    );
    (*cursor).hold_begin.notify = Some(
        handle_pointer_hold_begin
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(&raw mut (*wlr_cursor).events.hold_end, &raw mut (*cursor).hold_end);
    (*cursor).hold_end.notify = Some(
        handle_pointer_hold_end
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.pinch_begin,
        &raw mut (*cursor).pinch_begin,
    );
    (*cursor).pinch_begin.notify = Some(
        handle_pointer_pinch_begin
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.pinch_update,
        &raw mut (*cursor).pinch_update,
    );
    (*cursor).pinch_update.notify = Some(
        handle_pointer_pinch_update
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(&raw mut (*wlr_cursor).events.pinch_end, &raw mut (*cursor).pinch_end);
    (*cursor).pinch_end.notify = Some(
        handle_pointer_pinch_end
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.swipe_begin,
        &raw mut (*cursor).swipe_begin,
    );
    (*cursor).swipe_begin.notify = Some(
        handle_pointer_swipe_begin
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.swipe_update,
        &raw mut (*cursor).swipe_update,
    );
    (*cursor).swipe_update.notify = Some(
        handle_pointer_swipe_update
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(&raw mut (*wlr_cursor).events.swipe_end, &raw mut (*cursor).swipe_end);
    (*cursor).swipe_end.notify = Some(
        handle_pointer_swipe_end
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(&raw mut (*wlr_cursor).events.motion, &raw mut (*cursor).motion);
    (*cursor).motion.notify = Some(
        handle_pointer_motion_relative
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.motion_absolute,
        &raw mut (*cursor).motion_absolute,
    );
    (*cursor).motion_absolute.notify = Some(
        handle_pointer_motion_absolute
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(&raw mut (*wlr_cursor).events.button, &raw mut (*cursor).button);
    (*cursor).button.notify = Some(
        handle_pointer_button
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(&raw mut (*wlr_cursor).events.axis, &raw mut (*cursor).axis);
    (*cursor).axis.notify = Some(
        handle_pointer_axis
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(&raw mut (*wlr_cursor).events.frame, &raw mut (*cursor).frame);
    (*cursor).frame.notify = Some(
        handle_pointer_frame
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.touch_down,
        &raw mut (*cursor).touch_down,
    );
    (*cursor).touch_down.notify = Some(
        handle_touch_down
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(&raw mut (*wlr_cursor).events.touch_up, &raw mut (*cursor).touch_up);
    (*cursor).touch_up.notify = Some(
        handle_touch_up
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.touch_cancel,
        &raw mut (*cursor).touch_cancel,
    );
    (*cursor).touch_cancel.notify = Some(
        handle_touch_cancel
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.touch_motion,
        &raw mut (*cursor).touch_motion,
    );
    (*cursor).touch_motion.notify = Some(
        handle_touch_motion
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.touch_frame,
        &raw mut (*cursor).touch_frame,
    );
    (*cursor).touch_frame.notify = Some(
        handle_touch_frame
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.tablet_tool_axis,
        &raw mut (*cursor).tool_axis,
    );
    (*cursor).tool_axis.notify = Some(
        handle_tool_axis
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.tablet_tool_tip,
        &raw mut (*cursor).tool_tip,
    );
    (*cursor).tool_tip.notify = Some(
        handle_tool_tip
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.tablet_tool_proximity,
        &raw mut (*cursor).tool_proximity,
    );
    (*cursor).tool_proximity.notify = Some(
        handle_tool_proximity
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*wlr_cursor).events.tablet_tool_button,
        &raw mut (*cursor).tool_button,
    );
    (*cursor).tool_button.notify = Some(
        handle_tool_button
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*seat).wlr_seat).events.request_set_cursor,
        &raw mut (*cursor).request_set_cursor,
    );
    (*cursor).request_set_cursor.notify = Some(
        handle_request_pointer_set_cursor
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_list_init(&raw mut (*cursor).constraint_commit.link);
    wl_list_init(&raw mut (*cursor).tablets);
    wl_list_init(&raw mut (*cursor).tablet_pads);
    (*cursor).cursor = wlr_cursor;
    return cursor;
}
#[no_mangle]
pub unsafe extern "C" fn cursor_warp_to_container(
    mut cursor: *mut sway_cursor,
    mut container: *mut sway_container,
    mut force: bool,
) {
    if container.is_null() {
        return;
    }
    let mut r#box: wlr_box = wlr_box {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    container_get_box(container, &raw mut r#box);
    if !force
        && wlr_box_contains_point(
            &raw mut r#box,
            (*(*cursor).cursor).x,
            (*(*cursor).cursor).y,
        ) as ::core::ffi::c_int != 0
    {
        return;
    }
    let mut x: ::core::ffi::c_double = (*container).pending.x
        + (*container).pending.width / 2.0f64;
    let mut y: ::core::ffi::c_double = (*container).pending.y
        + (*container).pending.height / 2.0f64;
    wlr_cursor_warp((*cursor).cursor, ::core::ptr::null_mut::<wlr_input_device>(), x, y);
    cursor_unhide(cursor);
}
#[no_mangle]
pub unsafe extern "C" fn cursor_warp_to_workspace(
    mut cursor: *mut sway_cursor,
    mut workspace: *mut sway_workspace,
) {
    if workspace.is_null() {
        return;
    }
    let mut x: ::core::ffi::c_double = (*workspace).x
        + (*workspace).width as ::core::ffi::c_double / 2.0f64;
    let mut y: ::core::ffi::c_double = (*workspace).y
        + (*workspace).height as ::core::ffi::c_double / 2.0f64;
    wlr_cursor_warp((*cursor).cursor, ::core::ptr::null_mut::<wlr_input_device>(), x, y);
    cursor_unhide(cursor);
}
#[no_mangle]
pub unsafe extern "C" fn get_mouse_bindsym(
    mut name: *const ::core::ffi::c_char,
    mut error: *mut *mut ::core::ffi::c_char,
) -> uint32_t {
    if strncasecmp(
        name,
        b"button\0".as_ptr() as *const ::core::ffi::c_char,
        strlen(b"button\0".as_ptr() as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        let mut number: ::core::ffi::c_int = *name
            .offset(strlen(b"button\0".as_ptr() as *const ::core::ffi::c_char) as isize)
            as ::core::ffi::c_int - '0' as ::core::ffi::c_int;
        if number < 1 as ::core::ffi::c_int || number > 9 as ::core::ffi::c_int
            || strlen(name) > strlen(b"button0\0".as_ptr() as *const ::core::ffi::c_char)
        {
            *error = strdup(
                b"Only buttons 1-9 are supported. For other mouse buttons, use the name of the event code.\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return 0 as uint32_t;
        }
        static mut buttons: [uint32_t; 9] = [
            BTN_LEFT as uint32_t,
            BTN_MIDDLE as uint32_t,
            BTN_RIGHT as uint32_t,
            SWAY_SCROLL_UP as uint32_t,
            SWAY_SCROLL_DOWN as uint32_t,
            SWAY_SCROLL_LEFT as uint32_t,
            SWAY_SCROLL_RIGHT as uint32_t,
            BTN_SIDE as uint32_t,
            BTN_EXTRA as uint32_t,
        ];
        return buttons[(number - 1 as ::core::ffi::c_int) as usize];
    } else if has_prefix(name, b"BTN_\0".as_ptr() as *const ::core::ffi::c_char) {
        let mut code: ::core::ffi::c_int = libevdev_event_code_from_name(
            EV_KEY as ::core::ffi::c_uint,
            name,
        );
        if code == -1 as ::core::ffi::c_int {
            *error = format_str(
                b"Unknown event %s\0".as_ptr() as *const ::core::ffi::c_char,
                name,
            );
            return 0 as uint32_t;
        }
        return code as uint32_t;
    }
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn get_mouse_bindcode(
    mut name: *const ::core::ffi::c_char,
    mut error: *mut *mut ::core::ffi::c_char,
) -> uint32_t {
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut code: ::core::ffi::c_int = strtol(
        name,
        &raw mut endptr,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if endptr == name as *mut ::core::ffi::c_char && code <= 0 as ::core::ffi::c_int {
        *error = strdup(
            b"Button event code must be a positive integer.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as uint32_t;
    } else if *__errno_location() == ERANGE {
        *error = strdup(
            b"Button event code out of range.\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as uint32_t;
    }
    let mut event: *const ::core::ffi::c_char = libevdev_event_code_get_name(
        EV_KEY as ::core::ffi::c_uint,
        code as ::core::ffi::c_uint,
    );
    if event.is_null()
        || !has_prefix(event, b"BTN_\0".as_ptr() as *const ::core::ffi::c_char)
    {
        *error = format_str(
            b"Event code %d (%s) is not a button\0".as_ptr()
                as *const ::core::ffi::c_char,
            code,
            if !event.is_null() {
                event
            } else {
                b"(null)\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return 0 as uint32_t;
    }
    return code as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn get_mouse_button(
    mut name: *const ::core::ffi::c_char,
    mut error: *mut *mut ::core::ffi::c_char,
) -> uint32_t {
    let mut button: uint32_t = get_mouse_bindsym(name, error);
    if button == 0 && (*error).is_null() {
        button = get_mouse_bindcode(name, error);
    }
    return button;
}
#[no_mangle]
pub unsafe extern "C" fn get_mouse_button_name(
    mut button: uint32_t,
) -> *const ::core::ffi::c_char {
    let mut name: *const ::core::ffi::c_char = libevdev_event_code_get_name(
        EV_KEY as ::core::ffi::c_uint,
        button as ::core::ffi::c_uint,
    );
    if name.is_null() {
        if button == SWAY_SCROLL_UP as uint32_t {
            name = b"SWAY_SCROLL_UP\0".as_ptr() as *const ::core::ffi::c_char;
        } else if button == SWAY_SCROLL_DOWN as uint32_t {
            name = b"SWAY_SCROLL_DOWN\0".as_ptr() as *const ::core::ffi::c_char;
        } else if button == SWAY_SCROLL_LEFT as uint32_t {
            name = b"SWAY_SCROLL_LEFT\0".as_ptr() as *const ::core::ffi::c_char;
        } else if button == SWAY_SCROLL_RIGHT as uint32_t {
            name = b"SWAY_SCROLL_RIGHT\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    return name;
}
unsafe extern "C" fn warp_to_constraint_cursor_hint(mut cursor: *mut sway_cursor) {
    let mut constraint: *mut wlr_pointer_constraint_v1 = (*cursor).active_constraint;
    if (*constraint).current.cursor_hint.enabled {
        let mut sx: ::core::ffi::c_double = (*constraint).current.cursor_hint.x;
        let mut sy: ::core::ffi::c_double = (*constraint).current.cursor_hint.y;
        let mut view: *mut sway_view = view_from_wlr_surface((*constraint).surface);
        if view.is_null() {
            return;
        }
        let mut con: *mut sway_container = (*view).container;
        let mut lx: ::core::ffi::c_double = sx + (*con).pending.content_x
            - (*view).geometry.x as ::core::ffi::c_double;
        let mut ly: ::core::ffi::c_double = sy + (*con).pending.content_y
            - (*view).geometry.y as ::core::ffi::c_double;
        wlr_cursor_warp(
            (*cursor).cursor,
            ::core::ptr::null_mut::<wlr_input_device>(),
            lx,
            ly,
        );
        wlr_seat_pointer_warp((*constraint).seat, sx, sy);
    }
}
#[no_mangle]
pub unsafe extern "C" fn handle_constraint_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut sway_constraint: *mut sway_pointer_constraint = (listener
        as *mut ::core::ffi::c_char)
        .offset(-(40 as ::core::ffi::c_ulong as isize)) as *mut sway_pointer_constraint;
    let mut constraint: *mut wlr_pointer_constraint_v1 = (*sway_constraint).constraint;
    let mut cursor: *mut sway_cursor = (*sway_constraint).cursor;
    wl_list_remove(&raw mut (*sway_constraint).set_region.link);
    wl_list_remove(&raw mut (*sway_constraint).destroy.link);
    if (*cursor).active_constraint == constraint {
        warp_to_constraint_cursor_hint(cursor);
        if !(*cursor).constraint_commit.link.next.is_null() {
            wl_list_remove(&raw mut (*cursor).constraint_commit.link);
        }
        wl_list_init(&raw mut (*cursor).constraint_commit.link);
        (*cursor).active_constraint = ::core::ptr::null_mut::<
            wlr_pointer_constraint_v1,
        >();
    }
    free(sway_constraint as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn handle_pointer_constraint(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut constraint: *mut wlr_pointer_constraint_v1 = data
        as *mut wlr_pointer_constraint_v1;
    let mut seat: *mut sway_seat = (*(*constraint).seat).data as *mut sway_seat;
    let mut sway_constraint: *mut sway_pointer_constraint = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_pointer_constraint>(),
    ) as *mut sway_pointer_constraint;
    (*sway_constraint).cursor = (*seat).cursor as *mut sway_cursor;
    (*sway_constraint).constraint = constraint;
    (*sway_constraint).set_region.notify = Some(
        handle_pointer_constraint_set_region
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*constraint).events.set_region,
        &raw mut (*sway_constraint).set_region,
    );
    (*sway_constraint).destroy.notify = Some(
        handle_constraint_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*constraint).events.destroy,
        &raw mut (*sway_constraint).destroy,
    );
    let mut surface: *mut wlr_surface = (*(*seat).wlr_seat)
        .keyboard_state
        .focused_surface;
    if !surface.is_null() && surface == (*constraint).surface {
        sway_cursor_constrain((*seat).cursor, constraint);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sway_cursor_constrain(
    mut cursor: *mut sway_cursor,
    mut constraint: *mut wlr_pointer_constraint_v1,
) {
    let mut config_0: *mut seat_config = seat_get_config((*cursor).seat);
    if config_0.is_null() {
        config_0 = seat_get_config_by_name(
            b"*\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if config_0.is_null()
        || (*config_0).allow_constrain.0
            == seat_config_allow_constrain::CONSTRAIN_DISABLE.0
    {
        return;
    }
    if (*cursor).active_constraint == constraint {
        return;
    }
    wl_list_remove(&raw mut (*cursor).constraint_commit.link);
    if !(*cursor).active_constraint.is_null() {
        if constraint.is_null() {
            warp_to_constraint_cursor_hint(cursor);
        }
        wlr_pointer_constraint_v1_send_deactivated((*cursor).active_constraint);
    }
    (*cursor).active_constraint = constraint;
    if constraint.is_null() {
        wl_list_init(&raw mut (*cursor).constraint_commit.link);
        return;
    }
    (*cursor).active_confine_requires_warp = r#true != 0;
    if pixman_region32_not_empty(&raw mut (*constraint).current.region) != 0 {
        pixman_region32_intersect(
            &raw mut (*constraint).region,
            &raw mut (*(*constraint).surface).input_region,
            &raw mut (*constraint).current.region,
        );
    } else {
        pixman_region32_copy(
            &raw mut (*constraint).region,
            &raw mut (*(*constraint).surface).input_region,
        );
    }
    check_constraint_region(cursor);
    wlr_pointer_constraint_v1_send_activated(constraint);
    (*cursor).constraint_commit.notify = Some(
        handle_constraint_commit
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*constraint).surface).events.commit,
        &raw mut (*cursor).constraint_commit,
    );
}
#[no_mangle]
pub unsafe extern "C" fn handle_request_set_cursor_shape(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut event: *const wlr_cursor_shape_manager_v1_request_set_shape_event = data
        as *const wlr_cursor_shape_manager_v1_request_set_shape_event;
    let mut seat: *mut sway_seat = (*(*(*event).seat_client).seat).data
        as *mut sway_seat;
    if !seatop_allows_set_cursor(seat) {
        return;
    }
    let mut focused_client: *mut wl_client = ::core::ptr::null_mut::<wl_client>();
    let mut focused_surface: *mut wlr_surface = (*(*seat).wlr_seat)
        .pointer_state
        .focused_surface;
    if !focused_surface.is_null() {
        focused_client = wl_resource_get_client((*focused_surface).resource);
    }
    if focused_client.is_null() || (*(*event).seat_client).client != focused_client {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] denying request to set cursor from unfocused client\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/cursor.c\0".as_ptr() as *const ::core::ffi::c_char,
            1410 as ::core::ffi::c_int,
        );
        return;
    }
    cursor_set_image(
        (*seat).cursor,
        wlr_cursor_shape_v1_name((*event).shape),
        focused_client,
    );
}
pub const SWAY_SCROLL_UP: ::core::ffi::c_int = KEY_MAX + 1 as ::core::ffi::c_int;
pub const SWAY_SCROLL_DOWN: ::core::ffi::c_int = KEY_MAX + 2 as ::core::ffi::c_int;
pub const SWAY_SCROLL_LEFT: ::core::ffi::c_int = KEY_MAX + 3 as ::core::ffi::c_int;
pub const SWAY_SCROLL_RIGHT: ::core::ffi::c_int = KEY_MAX + 4 as ::core::ffi::c_int;
pub const EV_KEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const BTN_LEFT: ::core::ffi::c_int = 0x110 as ::core::ffi::c_int;
pub const BTN_RIGHT: ::core::ffi::c_int = 0x111 as ::core::ffi::c_int;
pub const BTN_MIDDLE: ::core::ffi::c_int = 0x112 as ::core::ffi::c_int;
pub const BTN_SIDE: ::core::ffi::c_int = 0x113 as ::core::ffi::c_int;
pub const BTN_EXTRA: ::core::ffi::c_int = 0x114 as ::core::ffi::c_int;
pub const KEY_MAX: ::core::ffi::c_int = 0x2ff as ::core::ffi::c_int;
