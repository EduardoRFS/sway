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
    pub type wlr_tablet_pad_client_v2;
    pub type wlr_tablet_pad;
    pub type wlr_tablet_client_v2;
    pub type wlr_tablet_impl;
    pub type wlr_tablet_tool_client_v2;
    pub type wlr_touch_impl;
    pub type wlr_readonly_data_buffer;
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
    pub type wlr_relative_pointer_manager_v1;
    pub type wlr_xwayland_shell_v1;
    pub type wlr_xwm;
    pub type wlr_xdg_shell;
    pub type wlr_session;
    pub type sway_transaction_instruction;
    pub type sway_text_node;
    pub type wlr_xdg_toplevel;
    pub type wlr_foreign_toplevel_handle_v1;
    pub type wlr_ext_foreign_toplevel_handle_v1;
    pub type launcher_ctx;
    pub type sway_xdg_decoration;
    pub type wlr_ext_image_capture_source_v1;
    pub type _PangoFontDescription;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn wl_list_init(list: *mut wl_list);
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn wl_list_empty(list: *const wl_list) -> ::core::ffi::c_int;
    fn wl_event_loop_add_timer(
        r#loop: *mut wl_event_loop,
        func: wl_event_loop_timer_func_t,
        data: *mut ::core::ffi::c_void,
    ) -> *mut wl_event_source;
    fn wl_event_source_timer_update(
        source: *mut wl_event_source,
        ms_delay: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn wl_resource_get_client(resource: *mut wl_resource) -> *mut wl_client;
    fn wlr_box_contains_point(
        r#box: *const wlr_box,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> bool;
    fn wlr_cursor_warp(
        cur: *mut wlr_cursor,
        dev: *mut wlr_input_device,
        lx: ::core::ffi::c_double,
        ly: ::core::ffi::c_double,
    ) -> bool;
    fn wlr_cursor_attach_input_device(cur: *mut wlr_cursor, dev: *mut wlr_input_device);
    fn wlr_cursor_detach_input_device(cur: *mut wlr_cursor, dev: *mut wlr_input_device);
    fn wlr_cursor_map_input_to_output(
        cur: *mut wlr_cursor,
        dev: *mut wlr_input_device,
        output: *mut wlr_output,
    );
    fn wlr_cursor_map_input_to_region(
        cur: *mut wlr_cursor,
        dev: *mut wlr_input_device,
        r#box: *const wlr_box,
    );
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn wlr_keyboard_from_input_device(
        input_device: *mut wlr_input_device,
    ) -> *mut wlr_keyboard;
    fn wlr_pointer_from_input_device(
        input_device: *mut wlr_input_device,
    ) -> *mut wlr_pointer;
    fn wlr_seat_create(
        display: *mut wl_display,
        name: *const ::core::ffi::c_char,
    ) -> *mut wlr_seat;
    fn wlr_seat_destroy(wlr_seat: *mut wlr_seat);
    fn wlr_seat_set_capabilities(wlr_seat: *mut wlr_seat, capabilities: uint32_t);
    fn wlr_seat_pointer_notify_clear_focus(wlr_seat: *mut wlr_seat);
    fn wlr_seat_pointer_notify_button(
        wlr_seat: *mut wlr_seat,
        time_msec: uint32_t,
        button: uint32_t,
        state: wl_pointer_button_state,
    ) -> uint32_t;
    fn wlr_seat_get_keyboard(seat: *mut wlr_seat) -> *mut wlr_keyboard;
    fn wlr_seat_keyboard_notify_enter(
        seat: *mut wlr_seat,
        surface: *mut wlr_surface,
        keycodes: *const uint32_t,
        num_keycodes: size_t,
        modifiers: *const wlr_keyboard_modifiers,
    );
    fn wlr_seat_keyboard_notify_clear_focus(wlr_seat: *mut wlr_seat);
    fn wlr_seat_touch_get_point(
        seat: *mut wlr_seat,
        touch_id: int32_t,
    ) -> *mut wlr_touch_point;
    fn wlr_seat_touch_point_clear_focus(
        seat: *mut wlr_seat,
        time_msec: uint32_t,
        touch_id: int32_t,
    );
    fn wlr_seat_validate_pointer_grab_serial(
        seat: *mut wlr_seat,
        origin: *mut wlr_surface,
        serial: uint32_t,
    ) -> bool;
    fn wlr_seat_validate_touch_grab_serial(
        seat: *mut wlr_seat,
        origin: *mut wlr_surface,
        serial: uint32_t,
        point_ptr: *mut *mut wlr_touch_point,
    ) -> bool;
    fn wlr_ext_workspace_handle_v1_set_active(
        workspace: *mut wlr_ext_workspace_handle_v1,
        enabled: bool,
    );
    fn wlr_idle_notifier_v1_notify_activity(
        notifier: *mut wlr_idle_notifier_v1,
        seat: *mut wlr_seat,
    );
    fn wlr_touch_from_input_device(
        input_device: *mut wlr_input_device,
    ) -> *mut wlr_touch;
    fn wlr_seat_set_selection(
        seat: *mut wlr_seat,
        source: *mut wlr_data_source,
        serial: uint32_t,
    );
    fn wlr_seat_start_pointer_drag(
        seat: *mut wlr_seat,
        drag: *mut wlr_drag,
        serial: uint32_t,
    );
    fn wlr_seat_start_touch_drag(
        seat: *mut wlr_seat,
        drag: *mut wlr_drag,
        serial: uint32_t,
        point: *mut wlr_touch_point,
    );
    fn wlr_data_source_destroy(source: *mut wlr_data_source);
    fn wlr_xcursor_image_get_buffer(image: *mut wlr_xcursor_image) -> *mut wlr_buffer;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn create_list() -> *mut list_t;
    fn list_free(list: *mut list_t);
    fn list_find(
        list: *mut list_t,
        item: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn has_prefix(
        str: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    ) -> bool;
    fn wlr_scene_node_destroy(node: *mut wlr_scene_node);
    fn wlr_scene_node_set_position(
        node: *mut wlr_scene_node,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn wlr_scene_drag_icon_create(
        parent: *mut wlr_scene_tree,
        drag_icon: *mut wlr_drag_icon,
    ) -> *mut wlr_scene_tree;
    static mut root: *mut sway_root;
    fn root_for_each_workspace(
        f: Option<
            unsafe extern "C" fn(*mut sway_workspace, *mut ::core::ffi::c_void) -> (),
        >,
        data: *mut ::core::ffi::c_void,
    );
    fn root_for_each_container(
        f: Option<
            unsafe extern "C" fn(*mut sway_container, *mut ::core::ffi::c_void) -> (),
        >,
        data: *mut ::core::ffi::c_void,
    );
    fn output_by_name_or_id(name_or_id: *const ::core::ffi::c_char) -> *mut sway_output;
    fn output_get_active_workspace(output: *mut sway_output) -> *mut sway_workspace;
    fn output_get_box(output: *mut sway_output, r#box: *mut wlr_box);
    fn container_for_each_child(
        container: *mut sway_container,
        f: Option<
            unsafe extern "C" fn(*mut sway_container, *mut ::core::ffi::c_void) -> (),
        >,
        data: *mut ::core::ffi::c_void,
    );
    fn container_obstructing_fullscreen_container(
        container: *mut sway_container,
    ) -> *mut sway_container;
    fn container_is_floating_or_child(container: *mut sway_container) -> bool;
    fn container_detach(child: *mut sway_container);
    fn container_is_sticky(con: *mut sway_container) -> bool;
    fn container_update_itself_and_parents(con: *mut sway_container);
    fn workspace_consider_destroy(ws: *mut sway_workspace);
    fn workspace_is_visible(ws: *mut sway_workspace) -> bool;
    fn workspace_for_each_container(
        ws: *mut sway_workspace,
        f: Option<
            unsafe extern "C" fn(*mut sway_container, *mut ::core::ffi::c_void) -> (),
        >,
        data: *mut ::core::ffi::c_void,
    );
    fn workspace_add_floating(workspace: *mut sway_workspace, con: *mut sway_container);
    fn sway_keyboard_create(
        seat: *mut sway_seat,
        device: *mut sway_seat_device,
    ) -> *mut sway_keyboard;
    fn sway_keyboard_configure(keyboard: *mut sway_keyboard);
    fn sway_keyboard_destroy(keyboard: *mut sway_keyboard);
    fn sway_keyboard_disarm_key_repeat(keyboard: *mut sway_keyboard);
    fn node_set_dirty(node: *mut sway_node);
    fn node_is_view(node: *mut sway_node) -> bool;
    fn node_get_output(node: *mut sway_node) -> *mut sway_output;
    fn node_get_parent(node: *mut sway_node) -> *mut sway_node;
    fn node_get_children(node: *mut sway_node) -> *mut list_t;
    fn node_has_ancestor(node: *mut sway_node, ancestor: *mut sway_node) -> bool;
    fn alloc_scene_tree(
        parent: *mut wlr_scene_tree,
        failed: *mut bool,
    ) -> *mut wlr_scene_tree;
    fn scene_descriptor_assign(
        node: *mut wlr_scene_node,
        r#type: sway_scene_descriptor_type,
        data: *mut ::core::ffi::c_void,
    ) -> bool;
    fn scene_descriptor_try_get(
        node: *mut wlr_scene_node,
        r#type: sway_scene_descriptor_type,
    ) -> *mut ::core::ffi::c_void;
    fn sway_libinput_device_is_builtin(device: *mut sway_input_device) -> bool;
    fn free_sway_binding(sb: *mut sway_binding);
    static mut config: *mut sway_config;
    fn wlr_seat_set_primary_selection(
        seat: *mut wlr_seat,
        source: *mut wlr_primary_selection_source,
        serial: uint32_t,
    );
    fn wlr_xcursor_manager_create(
        name: *const ::core::ffi::c_char,
        size: uint32_t,
    ) -> *mut wlr_xcursor_manager;
    fn wlr_xcursor_manager_destroy(manager: *mut wlr_xcursor_manager);
    fn wlr_xcursor_manager_load(
        manager: *mut wlr_xcursor_manager,
        scale: ::core::ffi::c_float,
    ) -> bool;
    fn wlr_xcursor_manager_get_xcursor(
        manager: *mut wlr_xcursor_manager,
        name: *const ::core::ffi::c_char,
        scale: ::core::ffi::c_float,
    ) -> *mut wlr_xcursor;
    fn wlr_xwayland_set_cursor(
        wlr_xwayland: *mut wlr_xwayland,
        buffer: *mut wlr_buffer,
        hotspot_x: int32_t,
        hotspot_y: int32_t,
    );
    fn wlr_xwayland_set_seat(xwayland: *mut wlr_xwayland, seat: *mut wlr_seat);
    fn view_set_activated(view: *mut sway_view, activated: bool);
    fn view_close_popups(view: *mut sway_view);
    fn view_set_urgent(view: *mut sway_view, enable: bool);
    fn view_is_urgent(view: *mut sway_view) -> bool;
    fn sway_input_method_relay_init(
        seat: *mut sway_seat,
        relay: *mut sway_input_method_relay,
    );
    fn sway_input_method_relay_finish(relay: *mut sway_input_method_relay);
    fn sway_input_method_relay_set_focus(
        relay: *mut sway_input_method_relay,
        surface: *mut wlr_surface,
    );
    fn sway_switch_create(
        seat: *mut sway_seat,
        device: *mut sway_seat_device,
    ) -> *mut sway_switch;
    fn sway_switch_configure(sway_switch: *mut sway_switch);
    fn sway_switch_destroy(sway_switch: *mut sway_switch);
    fn ipc_event_workspace(
        old: *mut sway_workspace,
        new: *mut sway_workspace,
        change: *const ::core::ffi::c_char,
    );
    fn ipc_event_window(window: *mut sway_container, change: *const ::core::ffi::c_char);
    fn sway_cursor_destroy(cursor: *mut sway_cursor);
    fn sway_cursor_create(seat: *mut sway_seat) -> *mut sway_cursor;
    fn cursor_handle_activity_from_device(
        cursor: *mut sway_cursor,
        device: *mut wlr_input_device,
    );
    fn cursor_get_timeout(cursor: *mut sway_cursor) -> ::core::ffi::c_int;
    fn cursor_set_image(
        cursor: *mut sway_cursor,
        image: *const ::core::ffi::c_char,
        client: *mut wl_client,
    );
    fn cursor_warp_to_container(
        cursor: *mut sway_cursor,
        container: *mut sway_container,
        force: bool,
    );
    fn cursor_warp_to_workspace(
        cursor: *mut sway_cursor,
        workspace: *mut sway_workspace,
    );
    fn sway_cursor_constrain(
        cursor: *mut sway_cursor,
        constraint: *mut wlr_pointer_constraint_v1,
    );
    fn sway_tablet_create(
        seat: *mut sway_seat,
        device: *mut sway_seat_device,
    ) -> *mut sway_tablet;
    fn sway_configure_tablet(tablet: *mut sway_tablet);
    fn sway_tablet_destroy(tablet: *mut sway_tablet);
    fn sway_tablet_pad_create(
        seat: *mut sway_seat,
        device: *mut sway_seat_device,
    ) -> *mut sway_tablet_pad;
    fn sway_configure_tablet_pad(tablet_pad: *mut sway_tablet_pad);
    fn sway_tablet_pad_destroy(tablet_pad: *mut sway_tablet_pad);
    fn sway_tablet_pad_set_focus(
        tablet_pad: *mut sway_tablet_pad,
        surface: *mut wlr_surface,
    );
    fn wlr_pointer_constraints_v1_constraint_for_surface(
        pointer_constraints: *mut wlr_pointer_constraints_v1,
        surface: *mut wlr_surface,
        seat: *mut wlr_seat,
    ) -> *mut wlr_pointer_constraint_v1;
    fn seatop_begin_default(seat: *mut sway_seat);
    fn arrange_workspace(workspace: *mut sway_workspace);
    static mut server: sway_server;
    fn sway_session_lock_has_surface(
        lock: *mut sway_session_lock,
        surface: *mut wlr_surface,
    ) -> bool;
    fn input_manager_get_default_seat() -> *mut sway_seat;
    fn input_manager_current_seat() -> *mut sway_seat;
    fn input_device_get_config(device: *mut sway_input_device) -> *mut input_config;
}
pub type size_t = usize;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
pub type pid_t = __pid_t;
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
pub struct wl_data_device_manager_dnd_action(pub ::core::ffi::c_uint);
impl wl_data_device_manager_dnd_action {
    pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_NONE: Self = Self(0);
    pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY: Self = Self(1);
    pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE: Self = Self(2);
    pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_ASK: Self = Self(4);
}
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
pub struct wlr_keyboard_group {
    pub keyboard: wlr_keyboard,
    pub devices: wl_list,
    pub keys: wl_list,
    pub events: C2Rust_Unnamed_26,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub enter: wl_signal,
    pub leave: wl_signal,
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
    pub events: C2Rust_Unnamed_27,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
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
    pub events: C2Rust_Unnamed_29,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub acc_discrete: [int32_t; 2],
    pub last_discrete: [int32_t; 2],
    pub acc_axis: [::core::ffi::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
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
    pub events: C2Rust_Unnamed_31,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
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
    pub events: C2Rust_Unnamed_33,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
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
    pub events: C2Rust_Unnamed_35,
    pub WLR_PRIVATE: C2Rust_Unnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
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
    pub events: C2Rust_Unnamed_37,
    pub WLR_PRIVATE: C2Rust_Unnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_37 {
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
pub struct wlr_data_source {
    pub r#impl: *const wlr_data_source_impl,
    pub mime_types: wl_array,
    pub actions: int32_t,
    pub accepted: bool,
    pub current_dnd_action: wl_data_device_manager_dnd_action,
    pub compositor_action: uint32_t,
    pub events: C2Rust_Unnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_data_source_impl {
    pub send: Option<
        unsafe extern "C" fn(
            *mut wlr_data_source,
            *const ::core::ffi::c_char,
            int32_t,
        ) -> (),
    >,
    pub accept: Option<
        unsafe extern "C" fn(
            *mut wlr_data_source,
            uint32_t,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub destroy: Option<unsafe extern "C" fn(*mut wlr_data_source) -> ()>,
    pub dnd_drop: Option<unsafe extern "C" fn(*mut wlr_data_source) -> ()>,
    pub dnd_finish: Option<unsafe extern "C" fn(*mut wlr_data_source) -> ()>,
    pub dnd_action: Option<
        unsafe extern "C" fn(
            *mut wlr_data_source,
            wl_data_device_manager_dnd_action,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_drag {
    pub grab_type: wlr_drag_grab_type,
    pub keyboard_grab: wlr_seat_keyboard_grab,
    pub pointer_grab: wlr_seat_pointer_grab,
    pub touch_grab: wlr_seat_touch_grab,
    pub seat: *mut wlr_seat,
    pub seat_client: *mut wlr_seat_client,
    pub focus_client: *mut wlr_seat_client,
    pub icon: *mut wlr_drag_icon,
    pub focus: *mut wlr_surface,
    pub source: *mut wlr_data_source,
    pub started: bool,
    pub dropped: bool,
    pub cancelling: bool,
    pub grab_touch_id: int32_t,
    pub touch_id: int32_t,
    pub events: C2Rust_Unnamed_40,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
    pub source_destroy: wl_listener,
    pub seat_client_destroy: wl_listener,
    pub focus_destroy: wl_listener,
    pub icon_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_40 {
    pub focus: wl_signal,
    pub motion: wl_signal,
    pub drop: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_drag_icon {
    pub drag: *mut wlr_drag,
    pub surface: *mut wlr_surface,
    pub events: C2Rust_Unnamed_42,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_41 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
    pub destroy: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_drag_grab_type(pub ::core::ffi::c_uint);
impl wlr_drag_grab_type {
    pub const WLR_DRAG_GRAB_KEYBOARD: Self = Self(0);
    pub const WLR_DRAG_GRAB_KEYBOARD_POINTER: Self = Self(1);
    pub const WLR_DRAG_GRAB_KEYBOARD_TOUCH: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_primary_selection_source {
    pub r#impl: *const wlr_primary_selection_source_impl,
    pub mime_types: wl_array,
    pub events: C2Rust_Unnamed_43,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_primary_selection_source_impl {
    pub send: Option<
        unsafe extern "C" fn(
            *mut wlr_primary_selection_source,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub destroy: Option<unsafe extern "C" fn(*mut wlr_primary_selection_source) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_request_set_selection_event {
    pub source: *mut wlr_data_source,
    pub serial: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_request_set_primary_selection_event {
    pub source: *mut wlr_primary_selection_source,
    pub serial: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_seat_request_start_drag_event {
    pub drag: *mut wlr_drag,
    pub origin: *mut wlr_surface,
    pub serial: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_data_device_manager {
    pub global: *mut wl_global,
    pub data_sources: wl_list,
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
    pub events: C2Rust_Unnamed_47,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_46,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_group_handle_v1 {
    pub manager: *mut wlr_ext_workspace_manager_v1,
    pub caps: uint32_t,
    pub events: C2Rust_Unnamed_49,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
    pub outputs: wl_list,
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_manager_v1 {
    pub global: *mut wl_global,
    pub groups: wl_list,
    pub workspaces: wl_list,
    pub events: C2Rust_Unnamed_51,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_50,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
    pub resources: wl_list,
    pub idle_source: *mut wl_event_source,
    pub event_loop: *mut wl_event_loop,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
    pub commit: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_idle_notifier_v1 {
    pub global: *mut wl_global,
    pub WLR_PRIVATE: C2Rust_Unnamed_52,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_52 {
    pub inhibited: bool,
    pub notifications: wl_list,
    pub display_destroy: wl_listener,
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
    pub events: C2Rust_Unnamed_54,
    pub WLR_PRIVATE: C2Rust_Unnamed_53,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_53 {
    pub pad_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_54 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_55,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
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
    pub events: C2Rust_Unnamed_56,
    pub paths: wl_array,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_56 {
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
    pub events: C2Rust_Unnamed_58,
    pub WLR_PRIVATE: C2Rust_Unnamed_57,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_57 {
    pub surface_destroy: wl_listener,
    pub tool_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_58 {
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
    pub events: C2Rust_Unnamed_59,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_59 {
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_touch {
    pub base: wlr_input_device,
    pub r#impl: *const wlr_touch_impl,
    pub output_name: *mut ::core::ffi::c_char,
    pub width_mm: ::core::ffi::c_double,
    pub height_mm: ::core::ffi::c_double,
    pub events: C2Rust_Unnamed_62,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
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
pub struct wlr_xcursor_image {
    pub width: uint32_t,
    pub height: uint32_t,
    pub hotspot_x: uint32_t,
    pub hotspot_y: uint32_t,
    pub delay: uint32_t,
    pub buffer: *mut uint8_t,
    pub WLR_PRIVATE: C2Rust_Unnamed_63,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
    pub readonly_buffer: *mut wlr_readonly_data_buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xcursor {
    pub image_count: ::core::ffi::c_uint,
    pub images: *mut *mut wlr_xcursor_image,
    pub name: *mut ::core::ffi::c_char,
    pub total_delay: uint32_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_switch_impl {
    pub name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_switch {
    pub base: wlr_input_device,
    pub r#impl: *const wlr_switch_impl,
    pub events: C2Rust_Unnamed_64,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
    pub toggle: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_switch_type(pub ::core::ffi::c_uint);
impl wlr_switch_type {
    pub const WLR_SWITCH_TYPE_LID: Self = Self(0);
    pub const WLR_SWITCH_TYPE_TABLET_MODE: Self = Self(1);
    pub const WLR_SWITCH_TYPE_KEYPAD_SLIDE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_switch_state(pub ::core::ffi::c_uint);
impl wlr_switch_state {
    pub const WLR_SWITCH_STATE_OFF: Self = Self(0);
    pub const WLR_SWITCH_STATE_ON: Self = Self(1);
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
    pub WLR_PRIVATE: C2Rust_Unnamed_65,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_65 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_67,
    pub WLR_PRIVATE: C2Rust_Unnamed_66,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
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
pub struct C2Rust_Unnamed_67 {
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
    pub events: C2Rust_Unnamed_69,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_68,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_70,
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
pub struct C2Rust_Unnamed_70 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_72,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_71,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_71 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
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
    pub events: C2Rust_Unnamed_74,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_73,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_73 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_74 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_75,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_75 {
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
    pub events: C2Rust_Unnamed_80,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_76,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_76 {
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
    pub events: C2Rust_Unnamed_78,
    pub WLR_PRIVATE: C2Rust_Unnamed_77,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_77 {
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
pub struct C2Rust_Unnamed_78 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_79,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_79 {
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
pub struct C2Rust_Unnamed_80 {
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
    pub layers: C2Rust_Unnamed_119,
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
    pub events: C2Rust_Unnamed_81,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_81 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_114,
    pub border: C2Rust_Unnamed_113,
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
    pub events: C2Rust_Unnamed_82,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_82 {
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
    pub layers: C2Rust_Unnamed_112,
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
    pub layers: C2Rust_Unnamed_109,
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
    pub session_lock: C2Rust_Unnamed_89,
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
    pub new_object: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_text_input_manager_v3 {
    pub global: *mut wl_global,
    pub text_inputs: wl_list,
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
    pub new_text_input: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_manager_v2 {
    pub global: *mut wl_global,
    pub input_methods: wl_list,
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
    pub new_input_method: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_89 {
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
    pub events: C2Rust_Unnamed_91,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_90,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_90 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_91 {
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
    pub events: C2Rust_Unnamed_94,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_92,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_92 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_93,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_93 {
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
pub struct C2Rust_Unnamed_94 {
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
    pub events: C2Rust_Unnamed_96,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_95,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_95 {
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_96 {
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
    pub events: C2Rust_Unnamed_100,
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
    pub events: C2Rust_Unnamed_102,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_101,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_101 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_102 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_pointers: wl_list,
    pub events: C2Rust_Unnamed_104,
    pub WLR_PRIVATE: C2Rust_Unnamed_103,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_103 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_104 {
    pub new_virtual_pointer: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_keyboards: wl_list,
    pub events: C2Rust_Unnamed_106,
    pub WLR_PRIVATE: C2Rust_Unnamed_105,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_105 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_106 {
    pub new_virtual_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_108,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_107,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_107 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_108 {
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_109 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_111,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_110,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_110 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_111 {
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
pub struct C2Rust_Unnamed_112 {
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
pub struct C2Rust_Unnamed_113 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_114 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_116,
    pub events: C2Rust_Unnamed_115,
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
pub struct C2Rust_Unnamed_115 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_116 {
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
    pub events: C2Rust_Unnamed_118,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_117,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_117 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_118 {
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
pub struct C2Rust_Unnamed_119 {
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
    pub events: C2Rust_Unnamed_121,
    pub WLR_PRIVATE: C2Rust_Unnamed_120,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_120 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_121 {
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
    pub events: C2Rust_Unnamed_123,
    pub WLR_PRIVATE: C2Rust_Unnamed_122,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_122 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_123 {
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
pub struct sway_switch {
    pub seat_device: *mut sway_seat_device,
    pub wlr: *mut wlr_switch,
    pub state: wlr_switch_state,
    pub r#type: wlr_switch_type,
    pub switch_toggle: wl_listener,
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
    pub previous: C2Rust_Unnamed_127,
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
    pub events: C2Rust_Unnamed_125,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_124,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_124 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub synced: wlr_surface_synced,
    pub destroying: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_125 {
    pub set_region: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_constraint_v1_state {
    pub committed: uint32_t,
    pub region: pixman_region32_t,
    pub cursor_hint: C2Rust_Unnamed_126,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_126 {
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
pub struct C2Rust_Unnamed_127 {
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
    pub xcursor_theme: C2Rust_Unnamed_128,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_128 {
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
    pub colors: C2Rust_Unnamed_129,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_129 {
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
    pub border_colors: C2Rust_Unnamed_132,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_130,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_130 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_131,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_131 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_132 {
    pub focused: border_colors,
    pub focused_inactive: border_colors,
    pub focused_tab_title: border_colors,
    pub unfocused: border_colors,
    pub urgent: border_colors,
    pub placeholder: border_colors,
    pub background: [::core::ffi::c_float; 4],
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
pub struct wlr_keyboard_shortcuts_inhibitor_v1 {
    pub surface: *mut wlr_surface,
    pub seat: *mut wlr_seat,
    pub active: bool,
    pub resource: *mut wl_resource,
    pub link: wl_list,
    pub events: C2Rust_Unnamed_134,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_133,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_133 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_134 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_seat_node {
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub link: wl_list,
    pub destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_drag {
    pub seat: *mut sway_seat,
    pub wlr_drag: *mut wlr_drag,
    pub destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_keyboard_shortcuts_inhibitor {
    pub inhibitor: *mut wlr_keyboard_shortcuts_inhibitor_v1,
    pub destroy: wl_listener,
    pub link: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_keyboard_group {
    pub wlr_group: *mut wlr_keyboard_group,
    pub seat_device: *mut sway_seat_device,
    pub keyboard_key: wl_listener,
    pub keyboard_modifiers: wl_listener,
    pub enter: wl_listener,
    pub leave: wl_listener,
    pub link: wl_list,
}
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
unsafe extern "C" fn seat_device_destroy(mut seat_device: *mut sway_seat_device) {
    if seat_device.is_null() {
        return;
    }
    sway_keyboard_destroy((*seat_device).keyboard);
    sway_tablet_destroy((*seat_device).tablet);
    sway_tablet_pad_destroy((*seat_device).tablet_pad);
    sway_switch_destroy((*seat_device).switch_device);
    wlr_cursor_detach_input_device(
        (*(*(*seat_device).sway_seat).cursor).cursor,
        (*(*seat_device).input_device).wlr_device,
    );
    wl_list_remove(&raw mut (*seat_device).link);
    free(seat_device as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn seat_node_destroy(mut seat_node: *mut sway_seat_node) {
    wl_list_remove(&raw mut (*seat_node).destroy.link);
    wl_list_remove(&raw mut (*seat_node).link);
    if wl_list_empty(&raw mut (*(*seat_node).seat).focus_stack) != 0 {
        (*(*seat_node).seat).has_focus = r#false != 0;
    }
    free(seat_node as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn seat_destroy(mut seat: *mut sway_seat) {
    wlr_seat_destroy((*seat).wlr_seat);
}
unsafe extern "C" fn handle_seat_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut seat: *mut sway_seat = (listener as *mut ::core::ffi::c_char)
        .offset(-(552 as ::core::ffi::c_ulong as isize)) as *mut sway_seat;
    if !config.is_null() && seat == (*config).handler_context.seat {
        (*config).handler_context.seat = input_manager_get_default_seat();
    }
    let mut seat_device: *mut sway_seat_device = ::core::ptr::null_mut::<
        sway_seat_device,
    >();
    let mut next: *mut sway_seat_device = ::core::ptr::null_mut::<sway_seat_device>();
    seat_device = ((*seat).devices.next as *mut ::core::ffi::c_char)
        .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
        as *mut sway_seat_device;
    next = ((*seat_device).link.next as *mut ::core::ffi::c_char)
        .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
        as *mut sway_seat_device;
    while &raw mut (*seat_device).link != &raw mut (*seat).devices {
        seat_device_destroy(seat_device);
        seat_device = next;
        next = ((*seat_device).link.next as *mut ::core::ffi::c_char)
            .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
            as *mut sway_seat_device;
    }
    let mut seat_node: *mut sway_seat_node = ::core::ptr::null_mut::<sway_seat_node>();
    let mut next_seat_node: *mut sway_seat_node = ::core::ptr::null_mut::<
        sway_seat_node,
    >();
    seat_node = ((*seat).focus_stack.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
        as *mut sway_seat_node;
    next_seat_node = ((*seat_node).link.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
        as *mut sway_seat_node;
    while &raw mut (*seat_node).link != &raw mut (*seat).focus_stack {
        seat_node_destroy(seat_node);
        seat_node = next_seat_node;
        next_seat_node = ((*seat_node).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
            as *mut sway_seat_node;
    }
    sway_input_method_relay_finish(&raw mut (*seat).im_relay);
    sway_cursor_destroy((*seat).cursor);
    wl_list_remove(&raw mut (*seat).new_node.link);
    wl_list_remove(&raw mut (*seat).request_start_drag.link);
    wl_list_remove(&raw mut (*seat).start_drag.link);
    wl_list_remove(&raw mut (*seat).request_set_selection.link);
    wl_list_remove(&raw mut (*seat).request_set_primary_selection.link);
    wl_list_remove(&raw mut (*seat).link);
    wl_list_remove(&raw mut (*seat).destroy.link);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*seat).deferred_bindings).length {
        free_sway_binding(
            *(*(*seat).deferred_bindings).items.offset(i as isize) as *mut sway_binding,
        );
        i += 1;
    }
    wlr_scene_node_destroy(&raw mut (*(*seat).scene_tree).node);
    list_free((*seat).deferred_bindings);
    free((*seat).prev_workspace_name as *mut ::core::ffi::c_void);
    free(seat as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn seat_idle_notify_activity(
    mut seat: *mut sway_seat,
    mut source: sway_input_idle_source,
) {
    if source.0 as uint32_t & (*seat).idle_inhibit_sources == 0 as uint32_t {
        return;
    }
    wlr_idle_notifier_v1_notify_activity(server.idle_notifier_v1, (*seat).wlr_seat);
}
unsafe extern "C" fn seat_send_activate(
    mut node: *mut sway_node,
    mut seat: *mut sway_seat,
) {
    if node_is_view(node) {
        if !seat_is_input_allowed(
            seat,
            (*(*(*node).c2rust_unnamed.sway_container).view).surface,
        ) {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Refusing to set focus, input is inhibited\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int,
            );
            return;
        }
        view_set_activated((*(*node).c2rust_unnamed.sway_container).view, r#true != 0);
    } else {
        let mut children: *mut list_t = node_get_children(node);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*children).length {
            let mut child: *mut sway_container = *(*children).items.offset(i as isize)
                as *mut sway_container;
            seat_send_activate(&raw mut (*child).node, seat);
            i += 1;
        }
    };
}
unsafe extern "C" fn sway_keyboard_for_wlr_keyboard(
    mut seat: *mut sway_seat,
    mut wlr_keyboard: *mut wlr_keyboard,
) -> *mut sway_keyboard {
    let mut seat_device: *mut sway_seat_device = ::core::ptr::null_mut::<
        sway_seat_device,
    >();
    seat_device = ((*seat).devices.next as *mut ::core::ffi::c_char)
        .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
        as *mut sway_seat_device;
    while &raw mut (*seat_device).link != &raw mut (*seat).devices {
        let mut input_device: *mut sway_input_device = (*seat_device).input_device;
        if (*(*input_device).wlr_device).r#type.0
            == wlr_input_device_type::WLR_INPUT_DEVICE_KEYBOARD.0
        {
            if (*input_device).wlr_device == &raw mut (*wlr_keyboard).base {
                return (*seat_device).keyboard;
            }
        }
        seat_device = ((*seat_device).link.next as *mut ::core::ffi::c_char)
            .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
            as *mut sway_seat_device;
    }
    let mut group: *mut sway_keyboard_group = ::core::ptr::null_mut::<
        sway_keyboard_group,
    >();
    group = ((*seat).keyboard_groups.next as *mut ::core::ffi::c_char)
        .offset(-(112 as ::core::ffi::c_ulong as isize)) as *mut sway_keyboard_group
        as *mut sway_keyboard_group;
    while &raw mut (*group).link != &raw mut (*seat).keyboard_groups {
        let mut input_device_0: *mut sway_input_device = (*(*group).seat_device)
            .input_device;
        if (*input_device_0).wlr_device == &raw mut (*wlr_keyboard).base {
            return (*(*group).seat_device).keyboard;
        }
        group = ((*group).link.next as *mut ::core::ffi::c_char)
            .offset(-(112 as ::core::ffi::c_ulong as isize)) as *mut sway_keyboard_group
            as *mut sway_keyboard_group;
    }
    return ::core::ptr::null_mut::<sway_keyboard>();
}
unsafe extern "C" fn seat_keyboard_notify_enter(
    mut seat: *mut sway_seat,
    mut surface: *mut wlr_surface,
) {
    let mut keyboard: *mut wlr_keyboard = wlr_seat_get_keyboard((*seat).wlr_seat);
    if keyboard.is_null() {
        wlr_seat_keyboard_notify_enter(
            (*seat).wlr_seat,
            surface,
            ::core::ptr::null::<uint32_t>(),
            0 as size_t,
            ::core::ptr::null::<wlr_keyboard_modifiers>(),
        );
        return;
    }
    let mut sway_keyboard: *mut sway_keyboard = sway_keyboard_for_wlr_keyboard(
        seat,
        keyboard,
    );
    if !sway_keyboard.is_null()
        && !(b"Cannot find sway_keyboard for seat keyboard\0".as_ptr()
            as *const ::core::ffi::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"sway_keyboard && \"Cannot find sway_keyboard for seat keyboard\"\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
            166 as ::core::ffi::c_uint,
            b"void seat_keyboard_notify_enter(struct sway_seat *, struct wlr_surface *)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    let mut state: *mut sway_shortcut_state = &raw mut (*sway_keyboard)
        .state_pressed_sent;
    wlr_seat_keyboard_notify_enter(
        (*seat).wlr_seat,
        surface,
        &raw mut (*state).pressed_keycodes as *mut uint32_t,
        (*state).npressed,
        &raw mut (*keyboard).modifiers,
    );
}
unsafe extern "C" fn seat_tablet_pads_set_focus(
    mut seat: *mut sway_seat,
    mut surface: *mut wlr_surface,
) {
    let mut seat_device: *mut sway_seat_device = ::core::ptr::null_mut::<
        sway_seat_device,
    >();
    seat_device = ((*seat).devices.next as *mut ::core::ffi::c_char)
        .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
        as *mut sway_seat_device;
    while &raw mut (*seat_device).link != &raw mut (*seat).devices {
        sway_tablet_pad_set_focus((*seat_device).tablet_pad, surface);
        seat_device = ((*seat_device).link.next as *mut ::core::ffi::c_char)
            .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
            as *mut sway_seat_device;
    }
}
unsafe extern "C" fn seat_send_focus(
    mut node: *mut sway_node,
    mut seat: *mut sway_seat,
) {
    seat_send_activate(node, seat);
    let mut view: *mut sway_view = if (*node).r#type.0 == sway_node_type::N_CONTAINER.0 {
        (*(*node).c2rust_unnamed.sway_container).view
    } else {
        ::core::ptr::null_mut::<sway_view>()
    };
    if !view.is_null()
        && seat_is_input_allowed(seat, (*view).surface) as ::core::ffi::c_int != 0
    {
        if (*view).r#type.0 == sway_view_type::SWAY_VIEW_XWAYLAND.0 {
            let mut xwayland: *mut wlr_xwayland = server.xwayland.wlr_xwayland;
            wlr_xwayland_set_seat(xwayland, (*seat).wlr_seat);
        }
        seat_keyboard_notify_enter(seat, (*view).surface);
        seat_tablet_pads_set_focus(seat, (*view).surface);
        sway_input_method_relay_set_focus(&raw mut (*seat).im_relay, (*view).surface);
        let mut constraint: *mut wlr_pointer_constraint_v1 = wlr_pointer_constraints_v1_constraint_for_surface(
            server.pointer_constraints,
            (*view).surface,
            (*seat).wlr_seat,
        );
        sway_cursor_constrain((*seat).cursor, constraint);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seat_for_each_node(
    mut seat: *mut sway_seat,
    mut f: Option<unsafe extern "C" fn(*mut sway_node, *mut ::core::ffi::c_void) -> ()>,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut current: *mut sway_seat_node = ::core::ptr::null_mut::<sway_seat_node>();
    current = ((*seat).focus_stack.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
        as *mut sway_seat_node;
    while &raw mut (*current).link != &raw mut (*seat).focus_stack {
        f.expect("non-null function pointer")((*current).node, data);
        current = ((*current).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
            as *mut sway_seat_node;
    }
}
#[no_mangle]
pub unsafe extern "C" fn seat_get_focus_inactive_view(
    mut seat: *mut sway_seat,
    mut ancestor: *mut sway_node,
) -> *mut sway_container {
    if node_is_view(ancestor) {
        return (*ancestor).c2rust_unnamed.sway_container;
    }
    let mut current: *mut sway_seat_node = ::core::ptr::null_mut::<sway_seat_node>();
    current = ((*seat).focus_stack.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
        as *mut sway_seat_node;
    while &raw mut (*current).link != &raw mut (*seat).focus_stack {
        let mut node: *mut sway_node = (*current).node;
        if node_is_view(node) as ::core::ffi::c_int != 0
            && node_has_ancestor(node, ancestor) as ::core::ffi::c_int != 0
        {
            return (*node).c2rust_unnamed.sway_container;
        }
        current = ((*current).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
            as *mut sway_seat_node;
    }
    return ::core::ptr::null_mut::<sway_container>();
}
unsafe extern "C" fn handle_seat_node_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut seat_node: *mut sway_seat_node = (listener as *mut ::core::ffi::c_char)
        .offset(-(32 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node;
    let mut seat: *mut sway_seat = (*seat_node).seat;
    let mut node: *mut sway_node = (*seat_node).node;
    let mut parent: *mut sway_node = node_get_parent(node);
    let mut focus: *mut sway_node = seat_get_focus(seat);
    if (*node).r#type.0 == sway_node_type::N_WORKSPACE.0 {
        seat_node_destroy(seat_node);
        if (*seat).workspace == (*node).c2rust_unnamed.sway_workspace {
            let mut node_0: *mut sway_node = seat_get_focus_inactive(
                seat,
                &raw mut (*root).node,
            );
            seat_set_focus(seat, ::core::ptr::null_mut::<sway_node>());
            if !node_0.is_null() {
                seat_set_focus(seat, node_0);
            } else {
                (*seat).workspace = ::core::ptr::null_mut::<sway_workspace>();
            }
        }
        return;
    }
    let mut needs_new_focus: bool = !focus.is_null()
        && (focus == node || node_has_ancestor(focus, node) as ::core::ffi::c_int != 0);
    seat_node_destroy(seat_node);
    if parent.is_null() && !needs_new_focus {
        return;
    }
    let mut next_focus: *mut sway_node = ::core::ptr::null_mut::<sway_node>();
    while next_focus.is_null() && !parent.is_null() {
        let mut con: *mut sway_container = seat_get_focus_inactive_view(seat, parent);
        next_focus = if !con.is_null() {
            &raw mut (*con).node
        } else {
            ::core::ptr::null_mut::<sway_node>()
        };
        if next_focus.is_null() && (*parent).r#type.0 == sway_node_type::N_WORKSPACE.0 {
            next_focus = parent;
            break;
        } else {
            parent = node_get_parent(parent);
        }
    }
    if next_focus.is_null() {
        let mut ws: *mut sway_workspace = seat_get_last_known_workspace(seat);
        if ws.is_null() {
            return;
        }
        let mut con_0: *mut sway_container = seat_get_focus_inactive_view(
            seat,
            &raw mut (*ws).node,
        );
        next_focus = if !con_0.is_null() {
            &raw mut (*con_0).node
        } else {
            &raw mut (*ws).node
        };
    }
    if (*next_focus).r#type.0 == sway_node_type::N_WORKSPACE.0
        && !workspace_is_visible((*next_focus).c2rust_unnamed.sway_workspace)
    {
        return;
    }
    if needs_new_focus {
        if (*node).r#type.0 == sway_node_type::N_CONTAINER.0
            && (*(*node).c2rust_unnamed.sway_container).scratchpad as ::core::ffi::c_int
                != 0
        {
            seat_set_focus(seat, ::core::ptr::null_mut::<sway_node>());
        }
        if seat_get_focus(seat) == next_focus {
            seat_send_focus(next_focus, seat);
        } else {
            seat_set_focus(seat, next_focus);
        }
    } else {
        focus = seat_get_focus_inactive(seat, &raw mut (*root).node);
        seat_set_raw_focus(seat, next_focus);
        if (*focus).r#type.0 == sway_node_type::N_CONTAINER.0
            && !(*(*focus).c2rust_unnamed.sway_container).pending.workspace.is_null()
        {
            seat_set_raw_focus(
                seat,
                &raw mut (*(*(*focus).c2rust_unnamed.sway_container).pending.workspace)
                    .node,
            );
        }
        seat_set_raw_focus(seat, focus);
    };
}
unsafe extern "C" fn seat_node_from_node(
    mut seat: *mut sway_seat,
    mut node: *mut sway_node,
) -> *mut sway_seat_node {
    if (*node).r#type.0 == sway_node_type::N_ROOT.0
        || (*node).r#type.0 == sway_node_type::N_OUTPUT.0
    {
        return ::core::ptr::null_mut::<sway_seat_node>();
    }
    let mut seat_node: *mut sway_seat_node = ::core::ptr::null_mut::<sway_seat_node>();
    seat_node = ((*seat).focus_stack.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
        as *mut sway_seat_node;
    while &raw mut (*seat_node).link != &raw mut (*seat).focus_stack {
        if (*seat_node).node == node {
            return seat_node;
        }
        seat_node = ((*seat_node).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
            as *mut sway_seat_node;
    }
    seat_node = calloc(1 as size_t, ::core::mem::size_of::<sway_seat_node>())
        as *mut sway_seat_node;
    if seat_node.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] could not allocate seat node\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
            343 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<sway_seat_node>();
    }
    (*seat_node).node = node;
    (*seat_node).seat = seat;
    wl_list_insert((*seat).focus_stack.prev, &raw mut (*seat_node).link);
    wl_signal_add(&raw mut (*node).events.destroy, &raw mut (*seat_node).destroy);
    (*seat_node).destroy.notify = Some(
        handle_seat_node_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    return seat_node;
}
unsafe extern "C" fn handle_new_node(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut seat: *mut sway_seat = (listener as *mut ::core::ffi::c_char)
        .offset(-(432 as ::core::ffi::c_ulong as isize)) as *mut sway_seat;
    let mut node: *mut sway_node = data as *mut sway_node;
    seat_node_from_node(seat, node);
}
unsafe extern "C" fn drag_icon_update_position(
    mut seat: *mut sway_seat,
    mut node: *mut wlr_scene_node,
) {
    let mut wlr_icon: *mut wlr_drag_icon = scene_descriptor_try_get(
        node,
        sway_scene_descriptor_type::SWAY_SCENE_DESC_DRAG_ICON,
    ) as *mut wlr_drag_icon;
    let mut cursor: *mut wlr_cursor = (*(*seat).cursor).cursor;
    match (*(*wlr_icon).drag).grab_type {
        wlr_drag_grab_type::WLR_DRAG_GRAB_KEYBOARD => return,
        wlr_drag_grab_type::WLR_DRAG_GRAB_KEYBOARD_POINTER => {
            wlr_scene_node_set_position(
                node,
                (*cursor).x as ::core::ffi::c_int,
                (*cursor).y as ::core::ffi::c_int,
            );
        }
        wlr_drag_grab_type::WLR_DRAG_GRAB_KEYBOARD_TOUCH => {
            let mut point: *mut wlr_touch_point = wlr_seat_touch_get_point(
                (*seat).wlr_seat,
                (*(*wlr_icon).drag).touch_id,
            );
            if point.is_null() {
                return;
            }
            wlr_scene_node_set_position(
                node,
                (*seat).touch_x as ::core::ffi::c_int,
                (*seat).touch_y as ::core::ffi::c_int,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn drag_icons_update_position(mut seat: *mut sway_seat) {
    let mut node: *mut wlr_scene_node = ::core::ptr::null_mut::<wlr_scene_node>();
    node = ((*(*seat).drag_icons).children.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut wlr_scene_node
        as *mut wlr_scene_node;
    while &raw mut (*node).link != &raw mut (*(*seat).drag_icons).children {
        drag_icon_update_position(seat, node);
        node = ((*node).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut wlr_scene_node
            as *mut wlr_scene_node;
    }
}
unsafe extern "C" fn drag_handle_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut drag: *mut sway_drag = (listener as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_drag;
    let mut seat: *mut sway_seat = (*drag).seat;
    let mut focus: *mut sway_node = seat_get_focus(seat);
    if !focus.is_null() {
        seat_set_focus(seat, ::core::ptr::null_mut::<sway_node>());
        seat_set_focus(seat, focus);
    } else if !(*seat).focused_layer.is_null() {
        let mut layer: *mut wlr_layer_surface_v1 = (*seat).focused_layer;
        seat_set_focus_layer(seat, ::core::ptr::null_mut::<wlr_layer_surface_v1>());
        seat_set_focus_layer(seat, layer);
    } else {
        let mut unmanaged: *mut wlr_surface = (*(*seat).wlr_seat)
            .keyboard_state
            .focused_surface;
        seat_set_focus_surface(
            seat,
            ::core::ptr::null_mut::<wlr_surface>(),
            r#false != 0,
        );
        seat_set_focus_surface(seat, unmanaged, r#false != 0);
    }
    (*(*drag).wlr_drag).data = NULL;
    wl_list_remove(&raw mut (*drag).destroy.link);
    free(drag as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn handle_request_start_drag(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut seat: *mut sway_seat = (listener as *mut ::core::ffi::c_char)
        .offset(-(456 as ::core::ffi::c_ulong as isize)) as *mut sway_seat;
    let mut event: *mut wlr_seat_request_start_drag_event = data
        as *mut wlr_seat_request_start_drag_event;
    if wlr_seat_validate_pointer_grab_serial(
        (*seat).wlr_seat,
        (*event).origin,
        (*event).serial,
    ) {
        wlr_seat_start_pointer_drag((*seat).wlr_seat, (*event).drag, (*event).serial);
        return;
    }
    let mut point: *mut wlr_touch_point = ::core::ptr::null_mut::<wlr_touch_point>();
    if wlr_seat_validate_touch_grab_serial(
        (*seat).wlr_seat,
        (*event).origin,
        (*event).serial,
        &raw mut point,
    ) {
        wlr_seat_start_touch_drag(
            (*seat).wlr_seat,
            (*event).drag,
            (*event).serial,
            point,
        );
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Ignoring start_drag request: could not validate pointer or touch serial %u\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
        435 as ::core::ffi::c_int,
        (*event).serial,
    );
    wlr_data_source_destroy((*(*event).drag).source);
}
unsafe extern "C" fn handle_start_drag(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut seat: *mut sway_seat = (listener as *mut ::core::ffi::c_char)
        .offset(-(480 as ::core::ffi::c_ulong as isize)) as *mut sway_seat;
    let mut wlr_drag: *mut wlr_drag = data as *mut wlr_drag;
    let mut drag: *mut sway_drag = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_drag>(),
    ) as *mut sway_drag;
    if drag.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
            446 as ::core::ffi::c_int,
        );
        return;
    }
    (*drag).seat = seat;
    (*drag).wlr_drag = wlr_drag;
    (*wlr_drag).data = drag as *mut ::core::ffi::c_void;
    (*drag).destroy.notify = Some(
        drag_handle_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(&raw mut (*wlr_drag).events.destroy, &raw mut (*drag).destroy);
    let mut wlr_drag_icon: *mut wlr_drag_icon = (*wlr_drag).icon;
    if !wlr_drag_icon.is_null() {
        let mut tree: *mut wlr_scene_tree = wlr_scene_drag_icon_create(
            (*seat).drag_icons,
            wlr_drag_icon,
        );
        if tree.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Failed to allocate a drag icon scene tree\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
                460 as ::core::ffi::c_int,
            );
            return;
        }
        if !scene_descriptor_assign(
            &raw mut (*tree).node,
            sway_scene_descriptor_type::SWAY_SCENE_DESC_DRAG_ICON,
            wlr_drag_icon as *mut ::core::ffi::c_void,
        ) {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Failed to allocate a drag icon scene descriptor\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
                466 as ::core::ffi::c_int,
            );
            wlr_scene_node_destroy(&raw mut (*tree).node);
            return;
        }
        drag_icon_update_position(seat, &raw mut (*tree).node);
    }
    seatop_begin_default(seat);
}
unsafe extern "C" fn handle_request_set_selection(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut seat: *mut sway_seat = (listener as *mut ::core::ffi::c_char)
        .offset(-(504 as ::core::ffi::c_ulong as isize)) as *mut sway_seat;
    let mut event: *mut wlr_seat_request_set_selection_event = data
        as *mut wlr_seat_request_set_selection_event;
    wlr_seat_set_selection((*seat).wlr_seat, (*event).source, (*event).serial);
}
unsafe extern "C" fn handle_request_set_primary_selection(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut seat: *mut sway_seat = (listener as *mut ::core::ffi::c_char)
        .offset(-(528 as ::core::ffi::c_ulong as isize)) as *mut sway_seat;
    let mut event: *mut wlr_seat_request_set_primary_selection_event = data
        as *mut wlr_seat_request_set_primary_selection_event;
    wlr_seat_set_primary_selection((*seat).wlr_seat, (*event).source, (*event).serial);
}
unsafe extern "C" fn collect_focus_iter(
    mut node: *mut sway_node,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut seat: *mut sway_seat = data as *mut sway_seat;
    let mut seat_node: *mut sway_seat_node = seat_node_from_node(seat, node);
    if seat_node.is_null() {
        return;
    }
    wl_list_remove(&raw mut (*seat_node).link);
    wl_list_insert(&raw mut (*seat).focus_stack, &raw mut (*seat_node).link);
}
unsafe extern "C" fn collect_focus_workspace_iter(
    mut workspace: *mut sway_workspace,
    mut data: *mut ::core::ffi::c_void,
) {
    collect_focus_iter(&raw mut (*workspace).node, data);
}
unsafe extern "C" fn collect_focus_container_iter(
    mut container: *mut sway_container,
    mut data: *mut ::core::ffi::c_void,
) {
    collect_focus_iter(&raw mut (*container).node, data);
}
#[no_mangle]
pub unsafe extern "C" fn seat_create(
    mut seat_name: *const ::core::ffi::c_char,
) -> *mut sway_seat {
    let mut seat: *mut sway_seat = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_seat>(),
    ) as *mut sway_seat;
    if seat.is_null() {
        return ::core::ptr::null_mut::<sway_seat>();
    }
    let mut failed: bool = r#false != 0;
    (*seat).scene_tree = alloc_scene_tree((*root).layers.seat, &raw mut failed);
    (*seat).drag_icons = alloc_scene_tree((*seat).scene_tree, &raw mut failed);
    if failed {
        wlr_scene_node_destroy(&raw mut (*(*seat).scene_tree).node);
        free(seat as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<sway_seat>();
    }
    (*seat).wlr_seat = wlr_seat_create(server.wl_display, seat_name);
    if !_sway_assert(
        !(*seat).wlr_seat.is_null(),
        b"[%s:%d] %s:could not allocate seat\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
        528 as ::core::ffi::c_int,
        b"struct sway_seat *seat_create(const char *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        wlr_scene_node_destroy(&raw mut (*(*seat).scene_tree).node);
        free(seat as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<sway_seat>();
    }
    (*(*seat).wlr_seat).data = seat as *mut ::core::ffi::c_void;
    (*seat).cursor = sway_cursor_create(seat) as *mut sway_cursor;
    if (*seat).cursor.is_null() {
        wlr_scene_node_destroy(&raw mut (*(*seat).scene_tree).node);
        wlr_seat_destroy((*seat).wlr_seat);
        free(seat as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<sway_seat>();
    }
    (*seat).destroy.notify = Some(
        handle_seat_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(&raw mut (*(*seat).wlr_seat).events.destroy, &raw mut (*seat).destroy);
    (*seat).idle_wake_sources = (sway_input_idle_source::IDLE_SOURCE_KEYBOARD.0
        as ::core::ffi::c_int
        | sway_input_idle_source::IDLE_SOURCE_POINTER.0 as ::core::ffi::c_int
        | sway_input_idle_source::IDLE_SOURCE_TOUCH.0 as ::core::ffi::c_int
        | sway_input_idle_source::IDLE_SOURCE_TABLET_PAD.0 as ::core::ffi::c_int
        | sway_input_idle_source::IDLE_SOURCE_TABLET_TOOL.0 as ::core::ffi::c_int
        | sway_input_idle_source::IDLE_SOURCE_SWITCH.0 as ::core::ffi::c_int)
        as uint32_t;
    (*seat).idle_inhibit_sources = (*seat).idle_wake_sources;
    wl_list_init(&raw mut (*seat).focus_stack);
    wl_list_init(&raw mut (*seat).devices);
    root_for_each_workspace(
        Some(
            collect_focus_workspace_iter
                as unsafe extern "C" fn(
                    *mut sway_workspace,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        seat as *mut ::core::ffi::c_void,
    );
    root_for_each_container(
        Some(
            collect_focus_container_iter
                as unsafe extern "C" fn(
                    *mut sway_container,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        seat as *mut ::core::ffi::c_void,
    );
    (*seat).deferred_bindings = create_list();
    wl_signal_add(&raw mut (*root).events.new_node, &raw mut (*seat).new_node);
    (*seat).new_node.notify = Some(
        handle_new_node
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*seat).wlr_seat).events.request_start_drag,
        &raw mut (*seat).request_start_drag,
    );
    (*seat).request_start_drag.notify = Some(
        handle_request_start_drag
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*seat).wlr_seat).events.start_drag,
        &raw mut (*seat).start_drag,
    );
    (*seat).start_drag.notify = Some(
        handle_start_drag
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*seat).wlr_seat).events.request_set_selection,
        &raw mut (*seat).request_set_selection,
    );
    (*seat).request_set_selection.notify = Some(
        handle_request_set_selection
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*seat).wlr_seat).events.request_set_primary_selection,
        &raw mut (*seat).request_set_primary_selection,
    );
    (*seat).request_set_primary_selection.notify = Some(
        handle_request_set_primary_selection
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_list_init(&raw mut (*seat).keyboard_groups);
    wl_list_init(&raw mut (*seat).keyboard_shortcuts_inhibitors);
    sway_input_method_relay_init(seat, &raw mut (*seat).im_relay);
    let mut first: bool = wl_list_empty(&raw mut (*server.input).seats) != 0;
    wl_list_insert(&raw mut (*server.input).seats, &raw mut (*seat).link);
    if !first {
        let mut current_seat: *mut sway_seat = input_manager_current_seat();
        let mut current_focus: *mut sway_node = seat_get_focus_inactive(
            current_seat,
            &raw mut (*root).node,
        );
        seat_set_focus(seat, current_focus);
    }
    seatop_begin_default(seat);
    return seat;
}
unsafe extern "C" fn seat_update_capabilities(mut seat: *mut sway_seat) {
    let mut caps: uint32_t = 0 as uint32_t;
    let mut previous_caps: uint32_t = (*(*seat).wlr_seat).capabilities;
    let mut seat_device: *mut sway_seat_device = ::core::ptr::null_mut::<
        sway_seat_device,
    >();
    seat_device = ((*seat).devices.next as *mut ::core::ffi::c_char)
        .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
        as *mut sway_seat_device;
    while &raw mut (*seat_device).link != &raw mut (*seat).devices {
        match (*(*(*seat_device).input_device).wlr_device).r#type {
            wlr_input_device_type::WLR_INPUT_DEVICE_KEYBOARD => {
                caps
                    |= wl_seat_capability::WL_SEAT_CAPABILITY_KEYBOARD.0
                        as ::core::ffi::c_int as uint32_t;
            }
            wlr_input_device_type::WLR_INPUT_DEVICE_POINTER => {
                caps
                    |= wl_seat_capability::WL_SEAT_CAPABILITY_POINTER.0
                        as ::core::ffi::c_int as uint32_t;
            }
            wlr_input_device_type::WLR_INPUT_DEVICE_TOUCH => {
                caps
                    |= wl_seat_capability::WL_SEAT_CAPABILITY_TOUCH.0
                        as ::core::ffi::c_int as uint32_t;
            }
            wlr_input_device_type::WLR_INPUT_DEVICE_TABLET => {
                caps
                    |= wl_seat_capability::WL_SEAT_CAPABILITY_POINTER.0
                        as ::core::ffi::c_int as uint32_t;
            }
            wlr_input_device_type::WLR_INPUT_DEVICE_SWITCH
            | wlr_input_device_type::WLR_INPUT_DEVICE_TABLET_PAD
            | _ => {}
        }
        seat_device = ((*seat_device).link.next as *mut ::core::ffi::c_char)
            .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
            as *mut sway_seat_device;
    }
    if caps
        & wl_seat_capability::WL_SEAT_CAPABILITY_POINTER.0 as ::core::ffi::c_int
            as uint32_t == 0 as uint32_t
    {
        cursor_set_image(
            (*seat).cursor,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<wl_client>(),
        );
        wlr_seat_set_capabilities((*seat).wlr_seat, caps);
    } else {
        wlr_seat_set_capabilities((*seat).wlr_seat, caps);
        if previous_caps
            & wl_seat_capability::WL_SEAT_CAPABILITY_POINTER.0 as ::core::ffi::c_int
                as uint32_t == 0 as uint32_t
        {
            cursor_set_image(
                (*seat).cursor,
                b"default\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<wl_client>(),
            );
        }
    };
}
unsafe extern "C" fn seat_reset_input_config(
    mut seat: *mut sway_seat,
    mut sway_device: *mut sway_seat_device,
) {
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Resetting output mapping for input device %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
        644 as ::core::ffi::c_int,
        (*(*sway_device).input_device).identifier,
    );
    wlr_cursor_map_input_to_output(
        (*(*seat).cursor).cursor,
        (*(*sway_device).input_device).wlr_device,
        ::core::ptr::null_mut::<wlr_output>(),
    );
}
unsafe extern "C" fn get_builtin_output_name() -> *const ::core::ffi::c_char {
    let mut r#match: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*root).outputs).length {
        let mut output: *mut sway_output = *(*(*root).outputs).items.offset(i as isize)
            as *mut sway_output;
        let mut name: *const ::core::ffi::c_char = (*(*output).wlr_output).name;
        if has_prefix(name, b"eDP-\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int != 0
            || has_prefix(name, b"LVDS-\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int != 0
            || has_prefix(name, b"DSI-\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int != 0
        {
            if !r#match.is_null() {
                return ::core::ptr::null::<::core::ffi::c_char>();
            }
            r#match = name;
        }
        i += 1;
    }
    return r#match;
}
unsafe extern "C" fn is_touch_or_tablet_tool(
    mut seat_device: *mut sway_seat_device,
) -> bool {
    match (*(*(*seat_device).input_device).wlr_device).r#type {
        wlr_input_device_type::WLR_INPUT_DEVICE_TOUCH
        | wlr_input_device_type::WLR_INPUT_DEVICE_TABLET => return r#true != 0,
        _ => return r#false != 0,
    };
}
unsafe extern "C" fn seat_apply_input_mapping(
    mut seat: *mut sway_seat,
    mut sway_device: *mut sway_seat_device,
) {
    let mut ic: *mut input_config = input_device_get_config((*sway_device).input_device);
    match (*(*(*sway_device).input_device).wlr_device).r#type {
        wlr_input_device_type::WLR_INPUT_DEVICE_POINTER
        | wlr_input_device_type::WLR_INPUT_DEVICE_TOUCH
        | wlr_input_device_type::WLR_INPUT_DEVICE_TABLET => {}
        _ => return,
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Applying input mapping to %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
        694 as ::core::ffi::c_int,
        (*(*sway_device).input_device).identifier,
    );
    let mut mapped_to_output: *const ::core::ffi::c_char = if ic.is_null() {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    } else {
        (*ic).mapped_to_output
    };
    let mut mapped_to_region: *mut wlr_box = if ic.is_null() {
        ::core::ptr::null_mut::<wlr_box>()
    } else {
        (*ic).mapped_to_region
    };
    let mut mapped_to: input_config_mapped_to = input_config_mapped_to(
        if ic.is_null() {
            input_config_mapped_to::MAPPED_TO_DEFAULT.0
        } else {
            (*ic).mapped_to.0
        },
    );
    let mut dev: *mut wlr_input_device = ::core::ptr::null_mut::<wlr_input_device>();
    let mut output: *mut sway_output = ::core::ptr::null_mut::<sway_output>();
    match mapped_to {
        input_config_mapped_to::MAPPED_TO_DEFAULT => {
            dev = (*(*sway_device).input_device).wlr_device;
            match (*dev).r#type {
                wlr_input_device_type::WLR_INPUT_DEVICE_POINTER => {
                    mapped_to_output = (*wlr_pointer_from_input_device(dev)).output_name;
                }
                wlr_input_device_type::WLR_INPUT_DEVICE_TOUCH => {
                    mapped_to_output = (*wlr_touch_from_input_device(dev)).output_name;
                }
                _ => {
                    mapped_to_output = ::core::ptr::null::<::core::ffi::c_char>();
                }
            }
            if mapped_to_output.is_null()
                && is_touch_or_tablet_tool(sway_device) as ::core::ffi::c_int != 0
                && sway_libinput_device_is_builtin((*sway_device).input_device)
                    as ::core::ffi::c_int != 0
            {
                mapped_to_output = get_builtin_output_name();
                if !mapped_to_output.is_null() {
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Auto-detected output '%s' for device '%s'\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
                        727 as ::core::ffi::c_int,
                        mapped_to_output,
                        (*(*sway_device).input_device).identifier,
                    );
                }
            }
            if mapped_to_output.is_null() {
                return;
            }
        }
        input_config_mapped_to::MAPPED_TO_OUTPUT => {}
        input_config_mapped_to::MAPPED_TO_REGION => {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Mapping input device %s to %d,%d %dx%d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
                764 as ::core::ffi::c_int,
                (*(*sway_device).input_device).identifier,
                (*mapped_to_region).x,
                (*mapped_to_region).y,
                (*mapped_to_region).width,
                (*mapped_to_region).height,
            );
            wlr_cursor_map_input_to_output(
                (*(*seat).cursor).cursor,
                (*(*sway_device).input_device).wlr_device,
                ::core::ptr::null_mut::<wlr_output>(),
            );
            wlr_cursor_map_input_to_region(
                (*(*seat).cursor).cursor,
                (*(*sway_device).input_device).wlr_device,
                mapped_to_region,
            );
            return;
        }
        _ => {
            return;
        }
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Mapping input device %s to output %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
        740 as ::core::ffi::c_int,
        (*(*sway_device).input_device).identifier,
        mapped_to_output,
    );
    if strcmp(b"*\0".as_ptr() as *const ::core::ffi::c_char, mapped_to_output)
        == 0 as ::core::ffi::c_int
    {
        wlr_cursor_map_input_to_output(
            (*(*seat).cursor).cursor,
            (*(*sway_device).input_device).wlr_device,
            ::core::ptr::null_mut::<wlr_output>(),
        );
        wlr_cursor_map_input_to_region(
            (*(*seat).cursor).cursor,
            (*(*sway_device).input_device).wlr_device,
            ::core::ptr::null::<wlr_box>(),
        );
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Reset output mapping\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
            747 as ::core::ffi::c_int,
        );
        return;
    }
    output = output_by_name_or_id(mapped_to_output);
    if output.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Requested output %s for device %s isn't present\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
            752 as ::core::ffi::c_int,
            mapped_to_output,
            (*(*sway_device).input_device).identifier,
        );
        return;
    }
    wlr_cursor_map_input_to_output(
        (*(*seat).cursor).cursor,
        (*(*sway_device).input_device).wlr_device,
        (*output).wlr_output,
    );
    wlr_cursor_map_input_to_region(
        (*(*seat).cursor).cursor,
        (*(*sway_device).input_device).wlr_device,
        ::core::ptr::null::<wlr_box>(),
    );
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Mapped to output %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
        760 as ::core::ffi::c_int,
        (*(*output).wlr_output).name,
    );
    return;
}
unsafe extern "C" fn seat_configure_pointer(
    mut seat: *mut sway_seat,
    mut sway_device: *mut sway_seat_device,
) {
    seat_configure_xcursor(seat);
    wlr_cursor_attach_input_device(
        (*(*seat).cursor).cursor,
        (*(*sway_device).input_device).wlr_device,
    );
    wl_event_source_timer_update(
        (*(*seat).cursor).hide_source,
        cursor_get_timeout((*seat).cursor),
    );
}
unsafe extern "C" fn seat_configure_keyboard(
    mut seat: *mut sway_seat,
    mut seat_device: *mut sway_seat_device,
) {
    if (*seat_device).keyboard.is_null() {
        sway_keyboard_create(seat, seat_device);
    }
    sway_keyboard_configure((*seat_device).keyboard);
    let mut wlr_keyboard: *mut wlr_keyboard = wlr_keyboard_from_input_device(
        (*(*seat_device).input_device).wlr_device,
    );
    let mut current_keyboard: *mut wlr_keyboard = (*(*seat).wlr_seat)
        .keyboard_state
        .keyboard;
    if wlr_keyboard != current_keyboard {
        return;
    }
    let mut surface: *mut wlr_surface = (*(*seat).wlr_seat)
        .keyboard_state
        .focused_surface;
    if !surface.is_null() {
        seat_keyboard_notify_enter(seat, surface);
    }
}
unsafe extern "C" fn seat_configure_switch(
    mut seat: *mut sway_seat,
    mut seat_device: *mut sway_seat_device,
) {
    if (*seat_device).switch_device.is_null() {
        sway_switch_create(seat, seat_device);
    }
    sway_switch_configure((*seat_device).switch_device);
}
unsafe extern "C" fn seat_configure_touch(
    mut seat: *mut sway_seat,
    mut sway_device: *mut sway_seat_device,
) {
    wlr_cursor_attach_input_device(
        (*(*seat).cursor).cursor,
        (*(*sway_device).input_device).wlr_device,
    );
}
unsafe extern "C" fn seat_configure_tablet_tool(
    mut seat: *mut sway_seat,
    mut sway_device: *mut sway_seat_device,
) {
    if (*sway_device).tablet.is_null() {
        (*sway_device).tablet = sway_tablet_create(seat, sway_device);
    }
    sway_configure_tablet((*sway_device).tablet);
    wlr_cursor_attach_input_device(
        (*(*seat).cursor).cursor,
        (*(*sway_device).input_device).wlr_device,
    );
}
unsafe extern "C" fn seat_configure_tablet_pad(
    mut seat: *mut sway_seat,
    mut sway_device: *mut sway_seat_device,
) {
    if (*sway_device).tablet_pad.is_null() {
        (*sway_device).tablet_pad = sway_tablet_pad_create(seat, sway_device);
    }
    sway_configure_tablet_pad((*sway_device).tablet_pad);
}
unsafe extern "C" fn seat_get_device(
    mut seat: *mut sway_seat,
    mut input_device: *mut sway_input_device,
) -> *mut sway_seat_device {
    let mut seat_device: *mut sway_seat_device = ::core::ptr::null_mut::<
        sway_seat_device,
    >();
    seat_device = ((*seat).devices.next as *mut ::core::ffi::c_char)
        .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
        as *mut sway_seat_device;
    while &raw mut (*seat_device).link != &raw mut (*seat).devices {
        if (*seat_device).input_device == input_device {
            return seat_device;
        }
        seat_device = ((*seat_device).link.next as *mut ::core::ffi::c_char)
            .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
            as *mut sway_seat_device;
    }
    let mut group: *mut sway_keyboard_group = ::core::ptr::null_mut::<
        sway_keyboard_group,
    >();
    group = ((*seat).keyboard_groups.next as *mut ::core::ffi::c_char)
        .offset(-(112 as ::core::ffi::c_ulong as isize)) as *mut sway_keyboard_group
        as *mut sway_keyboard_group;
    while &raw mut (*group).link != &raw mut (*seat).keyboard_groups {
        if (*(*group).seat_device).input_device == input_device {
            return (*group).seat_device;
        }
        group = ((*group).link.next as *mut ::core::ffi::c_char)
            .offset(-(112 as ::core::ffi::c_ulong as isize)) as *mut sway_keyboard_group
            as *mut sway_keyboard_group;
    }
    return ::core::ptr::null_mut::<sway_seat_device>();
}
#[no_mangle]
pub unsafe extern "C" fn seat_configure_device(
    mut seat: *mut sway_seat,
    mut input_device: *mut sway_input_device,
) {
    let mut seat_device: *mut sway_seat_device = seat_get_device(seat, input_device);
    if seat_device.is_null() {
        return;
    }
    match (*(*input_device).wlr_device).r#type {
        wlr_input_device_type::WLR_INPUT_DEVICE_POINTER => {
            seat_configure_pointer(seat, seat_device);
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_KEYBOARD => {
            seat_configure_keyboard(seat, seat_device);
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_SWITCH => {
            seat_configure_switch(seat, seat_device);
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_TOUCH => {
            seat_configure_touch(seat, seat_device);
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_TABLET => {
            seat_configure_tablet_tool(seat, seat_device);
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_TABLET_PAD => {
            seat_configure_tablet_pad(seat, seat_device);
        }
        _ => {}
    }
    seat_apply_input_mapping(seat, seat_device);
}
#[no_mangle]
pub unsafe extern "C" fn seat_configure_device_mapping(
    mut seat: *mut sway_seat,
    mut input_device: *mut sway_input_device,
) {
    let mut seat_device: *mut sway_seat_device = seat_get_device(seat, input_device);
    if seat_device.is_null() {
        return;
    }
    seat_apply_input_mapping(seat, seat_device);
}
#[no_mangle]
pub unsafe extern "C" fn seat_reset_device(
    mut seat: *mut sway_seat,
    mut input_device: *mut sway_input_device,
) {
    let mut seat_device: *mut sway_seat_device = seat_get_device(seat, input_device);
    if seat_device.is_null() {
        return;
    }
    match (*(*input_device).wlr_device).r#type {
        wlr_input_device_type::WLR_INPUT_DEVICE_POINTER => {
            seat_reset_input_config(seat, seat_device);
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_KEYBOARD => {
            sway_keyboard_disarm_key_repeat((*seat_device).keyboard);
            sway_keyboard_configure((*seat_device).keyboard);
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_TOUCH => {
            seat_reset_input_config(seat, seat_device);
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_TABLET => {
            seat_reset_input_config(seat, seat_device);
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_TABLET_PAD => {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] TODO: reset tablet pad\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
                923 as ::core::ffi::c_int,
            );
        }
        wlr_input_device_type::WLR_INPUT_DEVICE_SWITCH => {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] TODO: reset switch device\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
                926 as ::core::ffi::c_int,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn seat_add_device(
    mut seat: *mut sway_seat,
    mut input_device: *mut sway_input_device,
) {
    if !seat_get_device(seat, input_device).is_null() {
        seat_configure_device(seat, input_device);
        return;
    }
    let mut seat_device: *mut sway_seat_device = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_seat_device>(),
    ) as *mut sway_seat_device;
    if seat_device.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] could not allocate seat device\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
            941 as ::core::ffi::c_int,
        );
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] adding device %s to seat %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
        945 as ::core::ffi::c_int,
        (*input_device).identifier,
        (*(*seat).wlr_seat).name,
    );
    (*seat_device).sway_seat = seat;
    (*seat_device).input_device = input_device;
    wl_list_insert(&raw mut (*seat).devices, &raw mut (*seat_device).link);
    seat_configure_device(seat, input_device);
    seat_update_capabilities(seat);
}
#[no_mangle]
pub unsafe extern "C" fn seat_remove_device(
    mut seat: *mut sway_seat,
    mut input_device: *mut sway_input_device,
) {
    let mut seat_device: *mut sway_seat_device = seat_get_device(seat, input_device);
    if seat_device.is_null() {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] removing device %s from seat %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
        965 as ::core::ffi::c_int,
        (*input_device).identifier,
        (*(*seat).wlr_seat).name,
    );
    seat_device_destroy(seat_device);
    seat_update_capabilities(seat);
}
unsafe extern "C" fn xcursor_manager_is_named(
    mut manager: *const wlr_xcursor_manager,
    mut name: *const ::core::ffi::c_char,
) -> bool {
    return (*manager).name.is_null() && name.is_null()
        || !name.is_null() && !(*manager).name.is_null()
            && strcmp(name, (*manager).name) == 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn seat_configure_xcursor(mut seat: *mut sway_seat) {
    let mut cursor_size: ::core::ffi::c_uint = 24 as ::core::ffi::c_uint;
    let mut cursor_theme: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut seat_config: *const seat_config = seat_get_config(seat);
    if seat_config.is_null() {
        seat_config = seat_get_config_by_name(
            b"*\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !seat_config.is_null() {
        cursor_size = (*seat_config).xcursor_theme.size as ::core::ffi::c_uint;
        cursor_theme = (*seat_config).xcursor_theme.name;
    }
    if seat == input_manager_get_default_seat() {
        let mut cursor_size_fmt: [::core::ffi::c_char; 16] = [0; 16];
        snprintf(
            &raw mut cursor_size_fmt as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            cursor_size,
        );
        setenv(
            b"XCURSOR_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut cursor_size_fmt as *mut ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        if !cursor_theme.is_null() {
            setenv(
                b"XCURSOR_THEME\0".as_ptr() as *const ::core::ffi::c_char,
                cursor_theme,
                1 as ::core::ffi::c_int,
            );
        }
        if !server.xwayland.wlr_xwayland.is_null()
            && (server.xwayland.xcursor_manager.is_null()
                || !xcursor_manager_is_named(
                    server.xwayland.xcursor_manager,
                    cursor_theme,
                ) || (*server.xwayland.xcursor_manager).size != cursor_size as uint32_t)
        {
            wlr_xcursor_manager_destroy(server.xwayland.xcursor_manager);
            server.xwayland.xcursor_manager = wlr_xcursor_manager_create(
                cursor_theme,
                cursor_size as uint32_t,
            );
            _sway_assert(
                !server.xwayland.xcursor_manager.is_null(),
                b"[%s:%d] %s:Cannot create XCursor manager for theme\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
                1010 as ::core::ffi::c_int,
                b"void seat_configure_xcursor(struct sway_seat *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            wlr_xcursor_manager_load(
                server.xwayland.xcursor_manager,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
            );
            let mut xcursor: *mut wlr_xcursor = wlr_xcursor_manager_get_xcursor(
                server.xwayland.xcursor_manager,
                b"default\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
            );
            if !xcursor.is_null() {
                let mut image: *mut wlr_xcursor_image = *(*xcursor)
                    .images
                    .offset(0isize);
                let mut buffer: *mut wlr_buffer = wlr_xcursor_image_get_buffer(image);
                wlr_xwayland_set_cursor(
                    server.xwayland.wlr_xwayland,
                    buffer,
                    (*image).hotspot_x as int32_t,
                    (*image).hotspot_y as int32_t,
                );
            }
        }
    }
    if (*(*seat).cursor).xcursor_manager.is_null()
        || !xcursor_manager_is_named((*(*seat).cursor).xcursor_manager, cursor_theme)
        || (*(*(*seat).cursor).xcursor_manager).size != cursor_size as uint32_t
    {
        wlr_xcursor_manager_destroy((*(*seat).cursor).xcursor_manager);
        (*(*seat).cursor).xcursor_manager = wlr_xcursor_manager_create(
            cursor_theme,
            cursor_size as uint32_t,
        );
        if (*(*seat).cursor).xcursor_manager.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Cannot create XCursor manager for theme '%s'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
                1038 as ::core::ffi::c_int,
                cursor_theme,
            );
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*root).outputs).length {
            let mut sway_output: *mut sway_output = *(*(*root).outputs)
                .items
                .offset(i as isize) as *mut sway_output;
            let mut output: *mut wlr_output = (*sway_output).wlr_output;
            let mut result: bool = wlr_xcursor_manager_load(
                (*(*seat).cursor).xcursor_manager,
                (*output).scale,
            );
            if !result {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] Cannot load xcursor theme for output '%s' with scale %f\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1050 as ::core::ffi::c_int,
                    (*output).name,
                    (*output).scale as ::core::ffi::c_double,
                );
            }
            i += 1;
        }
        cursor_set_image(
            (*seat).cursor,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<wl_client>(),
        );
        cursor_set_image(
            (*seat).cursor,
            b"default\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<wl_client>(),
        );
        wlr_cursor_warp(
            (*(*seat).cursor).cursor,
            ::core::ptr::null_mut::<wlr_input_device>(),
            (*(*(*seat).cursor).cursor).x,
            (*(*(*seat).cursor).cursor).y,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn seat_is_input_allowed(
    mut seat: *mut sway_seat,
    mut surface: *mut wlr_surface,
) -> bool {
    if !server.session_lock.lock.is_null() {
        return sway_session_lock_has_surface(server.session_lock.lock, surface);
    }
    return r#true != 0;
}
unsafe extern "C" fn send_unfocus(
    mut con: *mut sway_container,
    mut data: *mut ::core::ffi::c_void,
) {
    if !(*con).view.is_null() {
        view_close_popups((*con).view);
        view_set_activated((*con).view, r#false != 0);
    }
}
unsafe extern "C" fn seat_send_unfocus(
    mut node: *mut sway_node,
    mut seat: *mut sway_seat,
) {
    sway_cursor_constrain(
        (*seat).cursor,
        ::core::ptr::null_mut::<wlr_pointer_constraint_v1>(),
    );
    wlr_seat_keyboard_notify_clear_focus((*seat).wlr_seat);
    if (*node).r#type.0 == sway_node_type::N_WORKSPACE.0 {
        workspace_for_each_container(
            (*node).c2rust_unnamed.sway_workspace,
            Some(
                send_unfocus
                    as unsafe extern "C" fn(
                        *mut sway_container,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            seat as *mut ::core::ffi::c_void,
        );
    } else {
        send_unfocus(
            (*node).c2rust_unnamed.sway_container,
            seat as *mut ::core::ffi::c_void,
        );
        container_for_each_child(
            (*node).c2rust_unnamed.sway_container,
            Some(
                send_unfocus
                    as unsafe extern "C" fn(
                        *mut sway_container,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            seat as *mut ::core::ffi::c_void,
        );
    };
}
unsafe extern "C" fn handle_urgent_timeout(
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut view: *mut sway_view = data as *mut sway_view;
    view_set_urgent(view, r#false != 0);
    container_update_itself_and_parents((*view).container);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn set_workspace(
    mut seat: *mut sway_seat,
    mut new_ws: *mut sway_workspace,
) {
    if (*seat).workspace == new_ws {
        return;
    }
    if !(*seat).workspace.is_null() {
        free((*seat).prev_workspace_name as *mut ::core::ffi::c_void);
        (*seat).prev_workspace_name = strdup((*(*seat).workspace).name);
        if (*seat).prev_workspace_name.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Unable to allocate previous workspace name\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
                1108 as ::core::ffi::c_int,
            );
        }
    }
    ipc_event_workspace(
        (*seat).workspace,
        new_ws,
        b"focus\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*seat).workspace = new_ws;
}
#[no_mangle]
pub unsafe extern "C" fn seat_set_raw_focus(
    mut seat: *mut sway_seat,
    mut node: *mut sway_node,
) {
    let mut seat_node: *mut sway_seat_node = seat_node_from_node(seat, node);
    wl_list_remove(&raw mut (*seat_node).link);
    wl_list_insert(&raw mut (*seat).focus_stack, &raw mut (*seat_node).link);
    node_set_dirty(node);
    let mut parent: *mut sway_node = node_get_parent(node);
    if !parent.is_null() {
        node_set_dirty(parent);
    }
}
unsafe extern "C" fn seat_set_workspace_focus(
    mut seat: *mut sway_seat,
    mut node: *mut sway_node,
) {
    let mut last_focus: *mut sway_node = seat_get_focus(seat);
    if last_focus == node {
        return;
    }
    if node.is_null() {
        seat_send_unfocus(last_focus, seat);
        sway_input_method_relay_set_focus(
            &raw mut (*seat).im_relay,
            ::core::ptr::null_mut::<wlr_surface>(),
        );
        (*seat).has_focus = r#false != 0;
        return;
    }
    let mut new_workspace: *mut sway_workspace = if (*node).r#type.0
        == sway_node_type::N_WORKSPACE.0
    {
        (*node).c2rust_unnamed.sway_workspace
    } else {
        (*(*node).c2rust_unnamed.sway_container).pending.workspace
    };
    let mut container: *mut sway_container = if (*node).r#type.0
        == sway_node_type::N_CONTAINER.0
    {
        (*node).c2rust_unnamed.sway_container
    } else {
        ::core::ptr::null_mut::<sway_container>()
    };
    if !container.is_null()
        && !container_obstructing_fullscreen_container(container).is_null()
    {
        return;
    }
    if !(*root).fullscreen_global.is_null() && container.is_null()
        && !new_workspace.is_null()
    {
        return;
    }
    let mut new_output: *mut sway_output = if !new_workspace.is_null() {
        (*new_workspace).output
    } else {
        ::core::ptr::null_mut::<sway_output>()
    };
    let mut last_workspace: *mut sway_workspace = if !new_output.is_null() {
        output_get_active_workspace(new_output)
    } else {
        ::core::ptr::null_mut::<sway_workspace>()
    };
    if last_workspace != new_workspace && !new_output.is_null() {
        node_set_dirty(&raw mut (*new_output).node);
    }
    if !last_focus.is_null() {
        seat_send_unfocus(last_focus, seat);
        node_set_dirty(last_focus);
        let mut parent: *mut sway_node = node_get_parent(last_focus);
        if !parent.is_null() {
            node_set_dirty(parent);
        }
    }
    if !container.is_null() {
        let mut parent_0: *mut sway_container = (*container).pending.parent;
        while !parent_0.is_null() {
            seat_set_raw_focus(seat, &raw mut (*parent_0).node);
            parent_0 = (*parent_0).pending.parent;
        }
    }
    if !new_workspace.is_null() {
        seat_set_raw_focus(seat, &raw mut (*new_workspace).node);
    }
    if !container.is_null() {
        seat_set_raw_focus(seat, &raw mut (*container).node);
        seat_send_focus(&raw mut (*container).node, seat);
    }
    set_workspace(seat, new_workspace);
    if !container.is_null() && !(*container).view.is_null() {
        ipc_event_window(container, b"focus\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if !last_workspace.is_null() && last_workspace != new_workspace {
        wlr_ext_workspace_handle_v1_set_active(
            (*last_workspace).ext_workspace,
            workspace_is_visible(last_workspace),
        );
    }
    if !new_workspace.is_null() {
        wlr_ext_workspace_handle_v1_set_active(
            (*new_workspace).ext_workspace,
            workspace_is_visible(new_workspace),
        );
    }
    if !new_workspace.is_null() && !last_workspace.is_null()
        && new_workspace != last_workspace
    {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*last_workspace).floating).length {
            let mut floater: *mut sway_container = *(*(*last_workspace).floating)
                .items
                .offset(i as isize) as *mut sway_container;
            if container_is_sticky(floater) {
                container_detach(floater);
                workspace_add_floating(new_workspace, floater);
                i -= 1;
            }
            i += 1;
        }
    }
    if !container.is_null() && !(*container).view.is_null()
        && view_is_urgent((*container).view) as ::core::ffi::c_int != 0
        && (*(*container).view).urgent_timer.is_null()
    {
        let mut view: *mut sway_view = (*container).view;
        if !last_workspace.is_null() && last_workspace != new_workspace
            && (*config).urgent_timeout > 0 as size_t
        {
            (*view).urgent_timer = wl_event_loop_add_timer(
                server.wl_event_loop,
                Some(
                    handle_urgent_timeout
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                view as *mut ::core::ffi::c_void,
            );
            if !(*view).urgent_timer.is_null() {
                wl_event_source_timer_update(
                    (*view).urgent_timer,
                    (*config).urgent_timeout as ::core::ffi::c_int,
                );
            } else {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] Unable to create urgency timer: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1235 as ::core::ffi::c_int,
                    strerror(*__errno_location()),
                );
                handle_urgent_timeout(view as *mut ::core::ffi::c_void);
            }
        } else {
            view_set_urgent(view, r#false != 0);
        }
    }
    if !last_workspace.is_null() {
        workspace_consider_destroy(last_workspace);
    }
    (*seat).has_focus = r#true != 0;
    if (*config).smart_gaps.0 != 0 && !new_workspace.is_null() {
        arrange_workspace(new_workspace);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seat_set_focus(
    mut seat: *mut sway_seat,
    mut node: *mut sway_node,
) {
    if (*seat).has_exclusive_layer {
        let mut layer: *mut wlr_layer_surface_v1 = (*seat).focused_layer;
        seat_set_focus_layer(seat, ::core::ptr::null_mut::<wlr_layer_surface_v1>());
        seat_set_workspace_focus(seat, node);
        seat_set_focus_layer(seat, layer);
    } else if !(*seat).focused_layer.is_null() {
        seat_set_focus_layer(seat, ::core::ptr::null_mut::<wlr_layer_surface_v1>());
        seat_set_workspace_focus(seat, node);
    } else {
        seat_set_workspace_focus(seat, node);
    }
    if !server.session_lock.lock.is_null() {
        seat_set_focus_surface(seat, (*server.session_lock.lock).focused, r#false != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seat_set_focus_container(
    mut seat: *mut sway_seat,
    mut con: *mut sway_container,
) {
    seat_set_focus(
        seat,
        if !con.is_null() {
            &raw mut (*con).node
        } else {
            ::core::ptr::null_mut::<sway_node>()
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn seat_set_focus_workspace(
    mut seat: *mut sway_seat,
    mut ws: *mut sway_workspace,
) {
    seat_set_focus(
        seat,
        if !ws.is_null() {
            &raw mut (*ws).node
        } else {
            ::core::ptr::null_mut::<sway_node>()
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn seat_set_focus_surface(
    mut seat: *mut sway_seat,
    mut surface: *mut wlr_surface,
    mut unfocus: bool,
) {
    if (*seat).has_focus as ::core::ffi::c_int != 0 && unfocus as ::core::ffi::c_int != 0
    {
        let mut focus: *mut sway_node = seat_get_focus(seat);
        seat_send_unfocus(focus, seat);
        node_set_dirty(focus);
        (*seat).has_focus = r#false != 0;
    }
    if !surface.is_null() {
        seat_keyboard_notify_enter(seat, surface);
    } else {
        wlr_seat_keyboard_notify_clear_focus((*seat).wlr_seat);
    }
    sway_input_method_relay_set_focus(&raw mut (*seat).im_relay, surface);
    seat_tablet_pads_set_focus(seat, surface);
}
#[no_mangle]
pub unsafe extern "C" fn seat_set_focus_layer(
    mut seat: *mut sway_seat,
    mut layer: *mut wlr_layer_surface_v1,
) {
    if layer.is_null() && !(*seat).focused_layer.is_null() {
        (*seat).focused_layer = ::core::ptr::null_mut::<wlr_layer_surface_v1>();
        let mut previous: *mut sway_node = seat_get_focus_inactive(
            seat,
            &raw mut (*root).node,
        );
        if !previous.is_null() {
            seat_set_focus(seat, ::core::ptr::null_mut::<sway_node>());
            seat_set_focus(seat, previous);
        }
        return;
    } else if layer.is_null() {
        return
    }
    if !server.session_lock.lock.is_null() {
        return;
    }
    if (*(*layer).surface).mapped as ::core::ffi::c_int != 0 {} else {
        __assert_fail(
            b"layer->surface->mapped\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
            1319 as ::core::ffi::c_uint,
            b"void seat_set_focus_layer(struct sway_seat *, struct wlr_layer_surface_v1 *)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    if (*layer).current.layer.0
        >= zwlr_layer_shell_v1_layer::ZWLR_LAYER_SHELL_V1_LAYER_TOP.0
        && (*layer).current.keyboard_interactive.0
            == zwlr_layer_surface_v1_keyboard_interactivity::ZWLR_LAYER_SURFACE_V1_KEYBOARD_INTERACTIVITY_EXCLUSIVE
                .0
    {
        (*seat).has_exclusive_layer = r#true != 0;
    }
    if (*seat).focused_layer == layer {
        return;
    }
    seat_set_focus_surface(seat, (*layer).surface, r#true != 0);
    (*seat).focused_layer = layer;
}
#[no_mangle]
pub unsafe extern "C" fn seat_unfocus_unless_client(
    mut seat: *mut sway_seat,
    mut client: *mut wl_client,
) {
    if !(*seat).focused_layer.is_null() {
        if wl_resource_get_client((*(*seat).focused_layer).resource) != client {
            seat_set_focus_layer(seat, ::core::ptr::null_mut::<wlr_layer_surface_v1>());
        }
    }
    if (*seat).has_focus {
        let mut focus: *mut sway_node = seat_get_focus(seat);
        if node_is_view(focus) as ::core::ffi::c_int != 0
            && wl_resource_get_client(
                (*(*(*(*focus).c2rust_unnamed.sway_container).view).surface).resource,
            ) != client
        {
            seat_set_focus(seat, ::core::ptr::null_mut::<sway_node>());
        }
    }
    if !(*(*seat).wlr_seat).pointer_state.focused_client.is_null() {
        if (*(*(*seat).wlr_seat).pointer_state.focused_client).client != client {
            wlr_seat_pointer_notify_clear_focus((*seat).wlr_seat);
        }
    }
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(CLOCK_MONOTONIC, &raw mut now);
    let mut point: *mut wlr_touch_point = ::core::ptr::null_mut::<wlr_touch_point>();
    point = ((*(*seat).wlr_seat).touch_state.touch_points.next
        as *mut ::core::ffi::c_char)
        .offset(-(72 as ::core::ffi::c_ulong as isize)) as *mut wlr_touch_point
        as *mut wlr_touch_point;
    while &raw mut (*point).link != &raw mut (*(*seat).wlr_seat).touch_state.touch_points
    {
        if (*(*point).client).client != client {
            wlr_seat_touch_point_clear_focus(
                (*seat).wlr_seat,
                (now.tv_nsec / 1000 as __syscall_slong_t) as uint32_t,
                (*point).touch_id,
            );
        }
        point = ((*point).link.next as *mut ::core::ffi::c_char)
            .offset(-(72 as ::core::ffi::c_ulong as isize)) as *mut wlr_touch_point
            as *mut wlr_touch_point;
    }
}
#[no_mangle]
pub unsafe extern "C" fn seat_get_focus_inactive(
    mut seat: *mut sway_seat,
    mut node: *mut sway_node,
) -> *mut sway_node {
    if node_is_view(node) {
        return node;
    }
    let mut current: *mut sway_seat_node = ::core::ptr::null_mut::<sway_seat_node>();
    current = ((*seat).focus_stack.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
        as *mut sway_seat_node;
    while &raw mut (*current).link != &raw mut (*seat).focus_stack {
        if node_has_ancestor((*current).node, node) {
            return (*current).node;
        }
        current = ((*current).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
            as *mut sway_seat_node;
    }
    if (*node).r#type.0 == sway_node_type::N_WORKSPACE.0 {
        return node;
    }
    return ::core::ptr::null_mut::<sway_node>();
}
#[no_mangle]
pub unsafe extern "C" fn seat_get_focus_inactive_tiling(
    mut seat: *mut sway_seat,
    mut workspace: *mut sway_workspace,
) -> *mut sway_container {
    if (*(*workspace).tiling).length == 0 {
        return ::core::ptr::null_mut::<sway_container>();
    }
    let mut current: *mut sway_seat_node = ::core::ptr::null_mut::<sway_seat_node>();
    current = ((*seat).focus_stack.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
        as *mut sway_seat_node;
    while &raw mut (*current).link != &raw mut (*seat).focus_stack {
        let mut node: *mut sway_node = (*current).node;
        if (*node).r#type.0 == sway_node_type::N_CONTAINER.0
            && !container_is_floating_or_child((*node).c2rust_unnamed.sway_container)
            && (*(*node).c2rust_unnamed.sway_container).pending.workspace == workspace
        {
            return (*node).c2rust_unnamed.sway_container;
        }
        current = ((*current).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
            as *mut sway_seat_node;
    }
    return ::core::ptr::null_mut::<sway_container>();
}
#[no_mangle]
pub unsafe extern "C" fn seat_get_focus_inactive_floating(
    mut seat: *mut sway_seat,
    mut workspace: *mut sway_workspace,
) -> *mut sway_container {
    if (*(*workspace).floating).length == 0 {
        return ::core::ptr::null_mut::<sway_container>();
    }
    let mut current: *mut sway_seat_node = ::core::ptr::null_mut::<sway_seat_node>();
    current = ((*seat).focus_stack.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
        as *mut sway_seat_node;
    while &raw mut (*current).link != &raw mut (*seat).focus_stack {
        let mut node: *mut sway_node = (*current).node;
        if (*node).r#type.0 == sway_node_type::N_CONTAINER.0
            && container_is_floating_or_child((*node).c2rust_unnamed.sway_container)
                as ::core::ffi::c_int != 0
            && (*(*node).c2rust_unnamed.sway_container).pending.workspace == workspace
        {
            return (*node).c2rust_unnamed.sway_container;
        }
        current = ((*current).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
            as *mut sway_seat_node;
    }
    return ::core::ptr::null_mut::<sway_container>();
}
#[no_mangle]
pub unsafe extern "C" fn seat_get_active_tiling_child(
    mut seat: *mut sway_seat,
    mut parent: *mut sway_node,
) -> *mut sway_node {
    if node_is_view(parent) {
        return parent;
    }
    let mut current: *mut sway_seat_node = ::core::ptr::null_mut::<sway_seat_node>();
    current = ((*seat).focus_stack.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
        as *mut sway_seat_node;
    while &raw mut (*current).link != &raw mut (*seat).focus_stack {
        let mut node: *mut sway_node = (*current).node;
        's_16: {
            if node_get_parent(node) == parent {
                if (*parent).r#type.0 == sway_node_type::N_WORKSPACE.0 {
                    let mut ws: *mut sway_workspace = (*parent)
                        .c2rust_unnamed
                        .sway_workspace;
                    if list_find(
                        (*ws).tiling,
                        (*node).c2rust_unnamed.sway_container
                            as *const ::core::ffi::c_void,
                    ) == -1 as ::core::ffi::c_int
                    {
                        break 's_16;
                    }
                }
                return node;
            }
        }
        current = ((*current).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
            as *mut sway_seat_node;
    }
    return ::core::ptr::null_mut::<sway_node>();
}
#[no_mangle]
pub unsafe extern "C" fn seat_get_focus(mut seat: *mut sway_seat) -> *mut sway_node {
    if !(*seat).has_focus {
        return ::core::ptr::null_mut::<sway_node>();
    }
    _sway_assert(
        wl_list_empty(&raw mut (*seat).focus_stack) == 0,
        b"[%s:%d] %s:focus_stack is empty, but has_focus is true\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/seat.c\0".as_ptr() as *const ::core::ffi::c_char,
        1439 as ::core::ffi::c_int,
        b"struct sway_node *seat_get_focus(struct sway_seat *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut current: *mut sway_seat_node = ((*seat).focus_stack.next
        as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node;
    return (*current).node;
}
#[no_mangle]
pub unsafe extern "C" fn seat_get_focused_workspace(
    mut seat: *mut sway_seat,
) -> *mut sway_workspace {
    let mut focus: *mut sway_node = seat_get_focus_inactive(seat, &raw mut (*root).node);
    if focus.is_null() {
        return ::core::ptr::null_mut::<sway_workspace>();
    }
    if (*focus).r#type.0 == sway_node_type::N_CONTAINER.0 {
        return (*(*focus).c2rust_unnamed.sway_container).pending.workspace;
    }
    if (*focus).r#type.0 == sway_node_type::N_WORKSPACE.0 {
        return (*focus).c2rust_unnamed.sway_workspace;
    }
    return ::core::ptr::null_mut::<sway_workspace>();
}
#[no_mangle]
pub unsafe extern "C" fn seat_get_last_known_workspace(
    mut seat: *mut sway_seat,
) -> *mut sway_workspace {
    let mut current: *mut sway_seat_node = ::core::ptr::null_mut::<sway_seat_node>();
    current = ((*seat).focus_stack.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
        as *mut sway_seat_node;
    while &raw mut (*current).link != &raw mut (*seat).focus_stack {
        let mut node: *mut sway_node = (*current).node;
        if (*node).r#type.0 == sway_node_type::N_CONTAINER.0
            && !(*(*node).c2rust_unnamed.sway_container).pending.workspace.is_null()
        {
            return (*(*node).c2rust_unnamed.sway_container).pending.workspace
        } else if (*node).r#type.0 == sway_node_type::N_WORKSPACE.0 {
            return (*node).c2rust_unnamed.sway_workspace
        }
        current = ((*current).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_node
            as *mut sway_seat_node;
    }
    return ::core::ptr::null_mut::<sway_workspace>();
}
#[no_mangle]
pub unsafe extern "C" fn seat_get_focused_container(
    mut seat: *mut sway_seat,
) -> *mut sway_container {
    let mut focus: *mut sway_node = seat_get_focus(seat);
    if !focus.is_null() && (*focus).r#type.0 == sway_node_type::N_CONTAINER.0 {
        return (*focus).c2rust_unnamed.sway_container;
    }
    return ::core::ptr::null_mut::<sway_container>();
}
#[no_mangle]
pub unsafe extern "C" fn seat_apply_config(
    mut seat: *mut sway_seat,
    mut seat_config: *mut seat_config,
) {
    let mut seat_device: *mut sway_seat_device = ::core::ptr::null_mut::<
        sway_seat_device,
    >();
    if seat_config.is_null() {
        return;
    }
    (*seat).idle_inhibit_sources = (*seat_config).idle_inhibit_sources;
    (*seat).idle_wake_sources = (*seat_config).idle_wake_sources;
    seat_device = ((*seat).devices.next as *mut ::core::ffi::c_char)
        .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
        as *mut sway_seat_device;
    while &raw mut (*seat_device).link != &raw mut (*seat).devices {
        seat_configure_device(seat, (*seat_device).input_device);
        cursor_handle_activity_from_device(
            (*seat).cursor,
            (*(*seat_device).input_device).wlr_device,
        );
        seat_device = ((*seat_device).link.next as *mut ::core::ffi::c_char)
            .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_seat_device
            as *mut sway_seat_device;
    }
}
#[no_mangle]
pub unsafe extern "C" fn seat_get_config(mut seat: *mut sway_seat) -> *mut seat_config {
    let mut seat_config: *mut seat_config = ::core::ptr::null_mut::<seat_config>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*config).seat_configs).length {
        seat_config = *(*(*config).seat_configs).items.offset(i as isize)
            as *mut seat_config;
        if strcmp((*(*seat).wlr_seat).name, (*seat_config).name)
            == 0 as ::core::ffi::c_int
        {
            return seat_config;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<seat_config>();
}
#[no_mangle]
pub unsafe extern "C" fn seat_get_config_by_name(
    mut name: *const ::core::ffi::c_char,
) -> *mut seat_config {
    let mut seat_config: *mut seat_config = ::core::ptr::null_mut::<seat_config>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*config).seat_configs).length {
        seat_config = *(*(*config).seat_configs).items.offset(i as isize)
            as *mut seat_config;
        if strcmp(name, (*seat_config).name) == 0 as ::core::ffi::c_int {
            return seat_config;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<seat_config>();
}
#[no_mangle]
pub unsafe extern "C" fn seat_pointer_notify_button(
    mut seat: *mut sway_seat,
    mut time_msec: uint32_t,
    mut button: uint32_t,
    mut state: wl_pointer_button_state,
) {
    (*seat).last_button_serial = wlr_seat_pointer_notify_button(
        (*seat).wlr_seat,
        time_msec,
        button,
        state,
    );
}
#[no_mangle]
pub unsafe extern "C" fn seat_consider_warp_to_focus(mut seat: *mut sway_seat) {
    let mut focus: *mut sway_node = seat_get_focus(seat);
    if (*config).mouse_warping.0 == mouse_warping_mode::WARP_NO.0 || focus.is_null() {
        return;
    }
    if (*config).mouse_warping.0 == mouse_warping_mode::WARP_OUTPUT.0 {
        let mut output: *mut sway_output = node_get_output(focus);
        if !output.is_null() {
            let mut r#box: wlr_box = wlr_box {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            output_get_box(output, &raw mut r#box);
            if wlr_box_contains_point(
                &raw mut r#box,
                (*(*(*seat).cursor).cursor).x,
                (*(*(*seat).cursor).cursor).y,
            ) {
                return;
            }
        }
    }
    if (*focus).r#type.0 == sway_node_type::N_CONTAINER.0 {
        cursor_warp_to_container(
            (*seat).cursor,
            (*focus).c2rust_unnamed.sway_container,
            r#false != 0,
        );
    } else {
        cursor_warp_to_workspace((*seat).cursor, (*focus).c2rust_unnamed.sway_workspace);
    };
}
#[no_mangle]
pub unsafe extern "C" fn seatop_unref(
    mut seat: *mut sway_seat,
    mut con: *mut sway_container,
) {
    if (*(*seat).seatop_impl).unref.is_some() {
        (*(*seat).seatop_impl).unref.expect("non-null function pointer")(seat, con);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_button(
    mut seat: *mut sway_seat,
    mut time_msec: uint32_t,
    mut device: *mut wlr_input_device,
    mut button: uint32_t,
    mut state: wl_pointer_button_state,
) {
    if (*(*seat).seatop_impl).button.is_some() {
        (*(*seat).seatop_impl)
            .button
            .expect("non-null function pointer")(seat, time_msec, device, button, state);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_pointer_motion(
    mut seat: *mut sway_seat,
    mut time_msec: uint32_t,
) {
    if (*(*seat).seatop_impl).pointer_motion.is_some() {
        (*(*seat).seatop_impl)
            .pointer_motion
            .expect("non-null function pointer")(seat, time_msec);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_pointer_axis(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_axis_event,
) {
    if (*(*seat).seatop_impl).pointer_axis.is_some() {
        (*(*seat).seatop_impl)
            .pointer_axis
            .expect("non-null function pointer")(seat, event);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_touch_motion(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_touch_motion_event,
    mut lx: ::core::ffi::c_double,
    mut ly: ::core::ffi::c_double,
) {
    if (*(*seat).seatop_impl).touch_motion.is_some() {
        (*(*seat).seatop_impl)
            .touch_motion
            .expect("non-null function pointer")(seat, event, lx, ly);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_touch_up(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_touch_up_event,
) {
    if (*(*seat).seatop_impl).touch_up.is_some() {
        (*(*seat).seatop_impl).touch_up.expect("non-null function pointer")(seat, event);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_touch_down(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_touch_down_event,
    mut lx: ::core::ffi::c_double,
    mut ly: ::core::ffi::c_double,
) {
    if (*(*seat).seatop_impl).touch_down.is_some() {
        (*(*seat).seatop_impl)
            .touch_down
            .expect("non-null function pointer")(seat, event, lx, ly);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_touch_cancel(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_touch_cancel_event,
) {
    if (*(*seat).seatop_impl).touch_cancel.is_some() {
        (*(*seat).seatop_impl)
            .touch_cancel
            .expect("non-null function pointer")(seat, event);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_tablet_tool_tip(
    mut seat: *mut sway_seat,
    mut tool: *mut sway_tablet_tool,
    mut time_msec: uint32_t,
    mut state: wlr_tablet_tool_tip_state,
) {
    if (*(*seat).seatop_impl).tablet_tool_tip.is_some() {
        (*(*seat).seatop_impl)
            .tablet_tool_tip
            .expect("non-null function pointer")(seat, tool, time_msec, state);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_tablet_tool_motion(
    mut seat: *mut sway_seat,
    mut tool: *mut sway_tablet_tool,
    mut time_msec: uint32_t,
) {
    if (*(*seat).seatop_impl).tablet_tool_motion.is_some() {
        (*(*seat).seatop_impl)
            .tablet_tool_motion
            .expect("non-null function pointer")(seat, tool, time_msec);
    } else {
        seatop_pointer_motion(seat, time_msec);
    };
}
#[no_mangle]
pub unsafe extern "C" fn seatop_hold_begin(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_hold_begin_event,
) {
    if (*(*seat).seatop_impl).hold_begin.is_some() {
        (*(*seat).seatop_impl)
            .hold_begin
            .expect("non-null function pointer")(seat, event);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_hold_end(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_hold_end_event,
) {
    if (*(*seat).seatop_impl).hold_end.is_some() {
        (*(*seat).seatop_impl).hold_end.expect("non-null function pointer")(seat, event);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_pinch_begin(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_pinch_begin_event,
) {
    if (*(*seat).seatop_impl).pinch_begin.is_some() {
        (*(*seat).seatop_impl)
            .pinch_begin
            .expect("non-null function pointer")(seat, event);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_pinch_update(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_pinch_update_event,
) {
    if (*(*seat).seatop_impl).pinch_update.is_some() {
        (*(*seat).seatop_impl)
            .pinch_update
            .expect("non-null function pointer")(seat, event);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_pinch_end(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_pinch_end_event,
) {
    if (*(*seat).seatop_impl).pinch_end.is_some() {
        (*(*seat).seatop_impl)
            .pinch_end
            .expect("non-null function pointer")(seat, event);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_swipe_begin(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_swipe_begin_event,
) {
    if (*(*seat).seatop_impl).swipe_begin.is_some() {
        (*(*seat).seatop_impl)
            .swipe_begin
            .expect("non-null function pointer")(seat, event);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_swipe_update(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_swipe_update_event,
) {
    if (*(*seat).seatop_impl).swipe_update.is_some() {
        (*(*seat).seatop_impl)
            .swipe_update
            .expect("non-null function pointer")(seat, event);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_swipe_end(
    mut seat: *mut sway_seat,
    mut event: *mut wlr_pointer_swipe_end_event,
) {
    if (*(*seat).seatop_impl).swipe_end.is_some() {
        (*(*seat).seatop_impl)
            .swipe_end
            .expect("non-null function pointer")(seat, event);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_rebase(
    mut seat: *mut sway_seat,
    mut time_msec: uint32_t,
) {
    if (*(*seat).seatop_impl).rebase.is_some() {
        (*(*seat).seatop_impl)
            .rebase
            .expect("non-null function pointer")(seat, time_msec);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seatop_end(mut seat: *mut sway_seat) {
    if !(*seat).seatop_impl.is_null() && (*(*seat).seatop_impl).end.is_some() {
        (*(*seat).seatop_impl).end.expect("non-null function pointer")(seat);
    }
    free((*seat).seatop_data);
    (*seat).seatop_data = NULL;
    (*seat).seatop_impl = ::core::ptr::null::<sway_seatop_impl>();
}
#[no_mangle]
pub unsafe extern "C" fn seatop_allows_set_cursor(mut seat: *mut sway_seat) -> bool {
    return (*(*seat).seatop_impl).allow_set_cursor;
}
#[no_mangle]
pub unsafe extern "C" fn keyboard_shortcuts_inhibitor_get_for_surface(
    mut seat: *const sway_seat,
    mut surface: *const wlr_surface,
) -> *mut sway_keyboard_shortcuts_inhibitor {
    let mut sway_inhibitor: *mut sway_keyboard_shortcuts_inhibitor = ::core::ptr::null_mut::<
        sway_keyboard_shortcuts_inhibitor,
    >();
    sway_inhibitor = ((*seat).keyboard_shortcuts_inhibitors.next
        as *mut ::core::ffi::c_char)
        .offset(-(32 as ::core::ffi::c_ulong as isize))
        as *mut sway_keyboard_shortcuts_inhibitor
        as *mut sway_keyboard_shortcuts_inhibitor;
    while &raw mut (*sway_inhibitor).link
        != &raw const (*seat).keyboard_shortcuts_inhibitors as *mut wl_list
    {
        if (*(*sway_inhibitor).inhibitor).surface == surface as *mut wlr_surface {
            return sway_inhibitor;
        }
        sway_inhibitor = ((*sway_inhibitor).link.next as *mut ::core::ffi::c_char)
            .offset(-(32 as ::core::ffi::c_ulong as isize))
            as *mut sway_keyboard_shortcuts_inhibitor
            as *mut sway_keyboard_shortcuts_inhibitor;
    }
    return ::core::ptr::null_mut::<sway_keyboard_shortcuts_inhibitor>();
}
#[no_mangle]
pub unsafe extern "C" fn keyboard_shortcuts_inhibitor_get_for_focused_surface(
    mut seat: *const sway_seat,
) -> *mut sway_keyboard_shortcuts_inhibitor {
    return keyboard_shortcuts_inhibitor_get_for_surface(
        seat,
        (*(*seat).wlr_seat).keyboard_state.focused_surface,
    );
}
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
