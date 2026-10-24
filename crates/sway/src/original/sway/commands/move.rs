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
    pub type wlr_xcursor_manager;
    pub type wlr_cursor_state;
    pub type xkb_keymap;
    pub type xkb_state;
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
    pub type wlr_session;
    pub type sway_transaction_instruction;
    pub type sway_text_node;
    pub type wlr_xdg_toplevel;
    pub type wlr_foreign_toplevel_handle_v1;
    pub type wlr_ext_foreign_toplevel_handle_v1;
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
    pub type _PangoFontDescription;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn fmax(
        __x: ::core::ffi::c_double,
        __y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn join_args(
        argv: *mut *mut ::core::ffi::c_char,
        argc: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn list_find(
        list: *mut list_t,
        item: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn list_swap(list: *mut list_t, src: ::core::ffi::c_int, dest: ::core::ffi::c_int);
    static mut root: *mut sway_root;
    fn root_scratchpad_add_container(con: *mut sway_container, ws: *mut sway_workspace);
    fn root_scratchpad_show(con: *mut sway_container);
    fn root_scratchpad_hide(con: *mut sway_container);
    fn output_get_in_direction(
        reference: *mut sway_output,
        direction: wlr_direction,
    ) -> *mut sway_output;
    fn output_by_name_or_id(name_or_id: *const ::core::ffi::c_char) -> *mut sway_output;
    fn output_get_active_workspace(output: *mut sway_output) -> *mut sway_workspace;
    fn output_get_box(output: *mut sway_output, r#box: *mut wlr_box);
    fn opposite_direction(d: wlr_direction) -> wlr_direction;
    fn container_has_ancestor(
        container: *mut sway_container,
        ancestor: *mut sway_container,
    ) -> bool;
    fn container_reap_empty(con: *mut sway_container);
    fn container_update_representation(container: *mut sway_container);
    fn floating_fix_coordinates(
        con: *mut sway_container,
        old: *mut wlr_box,
        new: *mut wlr_box,
    );
    fn container_is_floating(container: *mut sway_container) -> bool;
    fn container_floating_move_to(
        con: *mut sway_container,
        lx: ::core::ffi::c_double,
        ly: ::core::ffi::c_double,
    );
    fn container_end_mouse_operation(container: *mut sway_container);
    fn container_is_floating_or_child(container: *mut sway_container) -> bool;
    fn container_parent_layout(con: *mut sway_container) -> sway_container_layout;
    fn container_get_siblings(container: *mut sway_container) -> *mut list_t;
    fn container_sibling_index(child: *mut sway_container) -> ::core::ffi::c_int;
    fn container_handle_fullscreen_reparent(con: *mut sway_container);
    fn container_add_child(parent: *mut sway_container, child: *mut sway_container);
    fn container_insert_child(
        parent: *mut sway_container,
        child: *mut sway_container,
        i: ::core::ffi::c_int,
    );
    fn container_add_sibling(
        parent: *mut sway_container,
        child: *mut sway_container,
        after: bool,
    );
    fn container_detach(child: *mut sway_container);
    fn container_find_mark(mark: *mut ::core::ffi::c_char) -> *mut sway_container;
    fn container_is_scratchpad_hidden(con: *mut sway_container) -> bool;
    fn container_is_scratchpad_hidden_or_child(con: *mut sway_container) -> bool;
    fn container_is_sticky_or_child(con: *mut sway_container) -> bool;
    fn workspace_get_initial_output(
        name: *const ::core::ffi::c_char,
    ) -> *mut sway_output;
    fn workspace_create(
        output: *mut sway_output,
        name: *const ::core::ffi::c_char,
    ) -> *mut sway_workspace;
    fn workspace_consider_destroy(ws: *mut sway_workspace);
    fn workspace_by_number(name: *const ::core::ffi::c_char) -> *mut sway_workspace;
    fn workspace_by_name(_: *const ::core::ffi::c_char) -> *mut sway_workspace;
    fn workspace_is_empty(ws: *mut sway_workspace) -> bool;
    fn workspace_detect_urgent(workspace: *mut sway_workspace);
    fn workspace_wrap_children(ws: *mut sway_workspace) -> *mut sway_container;
    fn workspace_unwrap_children(ws: *mut sway_workspace, wrap: *mut sway_container);
    fn workspace_add_tiling(
        workspace: *mut sway_workspace,
        con: *mut sway_container,
    ) -> *mut sway_container;
    fn workspace_add_floating(workspace: *mut sway_workspace, con: *mut sway_container);
    fn workspace_insert_tiling(
        workspace: *mut sway_workspace,
        con: *mut sway_container,
        index: ::core::ffi::c_int,
    ) -> *mut sway_container;
    fn workspace_update_representation(ws: *mut sway_workspace);
    fn workspace_get_box(workspace: *mut sway_workspace, r#box: *mut wlr_box);
    fn workspace_squash(workspace: *mut sway_workspace);
    fn workspace_move_to_output(
        workspace: *mut sway_workspace,
        output: *mut sway_output,
    );
    fn node_get_output(node: *mut sway_node) -> *mut sway_output;
    fn node_get_parent(node: *mut sway_node) -> *mut sway_node;
    fn node_has_ancestor(node: *mut sway_node, ancestor: *mut sway_node) -> bool;
    fn checkarg(
        argc: ::core::ffi::c_int,
        name: *const ::core::ffi::c_char,
        r#type: expected_args,
        val: ::core::ffi::c_int,
    ) -> *mut cmd_results;
    fn cmd_results_new(
        status: cmd_status,
        error: *const ::core::ffi::c_char,
        ...
    ) -> *mut cmd_results;
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
    fn wlr_output_layout_adjacent_output(
        layout: *mut wlr_output_layout,
        direction: wlr_direction,
        reference: *mut wlr_output,
        ref_lx: ::core::ffi::c_double,
        ref_ly: ::core::ffi::c_double,
    ) -> *mut wlr_output;
    fn wlr_output_layout_farthest_output(
        layout: *mut wlr_output_layout,
        direction: wlr_direction,
        reference: *mut wlr_output,
        ref_lx: ::core::ffi::c_double,
        ref_ly: ::core::ffi::c_double,
    ) -> *mut wlr_output;
    static mut config: *mut sway_config;
    fn ipc_event_window(window: *mut sway_container, change: *const ::core::ffi::c_char);
    fn arrange_workspace(workspace: *mut sway_workspace);
    fn arrange_output(output: *mut sway_output);
    fn arrange_root();
    fn arrange_node(node: *mut sway_node);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    fn parse_movement_amount(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
        amount: *mut movement_amount,
    ) -> ::core::ffi::c_int;
    fn seat_set_focus(seat: *mut sway_seat, node: *mut sway_node);
    fn seat_set_raw_focus(seat: *mut sway_seat, node: *mut sway_node);
    fn seat_get_focus(seat: *mut sway_seat) -> *mut sway_node;
    fn seat_get_focused_workspace(seat: *mut sway_seat) -> *mut sway_workspace;
    fn seat_get_focus_inactive(
        seat: *mut sway_seat,
        node: *mut sway_node,
    ) -> *mut sway_node;
    fn seat_get_focus_inactive_tiling(
        seat: *mut sway_seat,
        workspace: *mut sway_workspace,
    ) -> *mut sway_container;
    fn seat_get_active_tiling_child(
        seat: *mut sway_seat,
        parent: *mut sway_node,
    ) -> *mut sway_node;
    fn seat_consider_warp_to_focus(seat: *mut sway_seat);
    static mut server: sway_server;
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
    pub WLR_PRIVATE: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub addons: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon {
    pub r#impl: *const wlr_addon_interface,
    pub WLR_PRIVATE: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
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
    pub events: C2Rust_Unnamed_5,
    pub features: C2Rust_Unnamed_4,
    pub WLR_PRIVATE: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub r#impl: *const wlr_renderer_impl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub input_color_transform: bool,
    pub output_color_transform: bool,
    pub timeline: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
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
    pub events: C2Rust_Unnamed_6,
    pub addons: wlr_addon_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
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
    pub features: C2Rust_Unnamed_8,
    pub events: C2Rust_Unnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
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
    pub events: C2Rust_Unnamed_13,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
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
    pub mastering_luminance: C2Rust_Unnamed_12,
    pub max_cll: ::core::ffi::c_double,
    pub max_fall: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
    pub min: ::core::ffi::c_double,
    pub max: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
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
    pub custom_mode: C2Rust_Unnamed_14,
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
pub struct C2Rust_Unnamed_14 {
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
    pub events: C2Rust_Unnamed_16,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_direction(pub ::core::ffi::c_uint);
impl wlr_direction {
    pub const WLR_DIRECTION_UP: Self = Self(1);
    pub const WLR_DIRECTION_DOWN: Self = Self(2);
    pub const WLR_DIRECTION_LEFT: Self = Self(4);
    pub const WLR_DIRECTION_RIGHT: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_device {
    pub r#type: wlr_input_device_type,
    pub name: *mut ::core::ffi::c_char,
    pub events: C2Rust_Unnamed_17,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
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
    pub events: C2Rust_Unnamed_20,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_19,
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
pub struct C2Rust_Unnamed_19 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
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
    pub viewport: C2Rust_Unnamed_21,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
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
    pub events: C2Rust_Unnamed_23,
    pub WLR_PRIVATE: C2Rust_Unnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_cursor {
    pub state: *mut wlr_cursor_state,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub events: C2Rust_Unnamed_24,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
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
pub struct wlr_tablet_tool_tip_state(pub ::core::ffi::c_uint);
impl wlr_tablet_tool_tip_state {
    pub const WLR_TABLET_TOOL_TIP_UP: Self = Self(0);
    pub const WLR_TABLET_TOOL_TIP_DOWN: Self = Self(1);
}
pub type xkb_layout_index_t = uint32_t;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
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
    pub WLR_PRIVATE: C2Rust_Unnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_27,
    pub WLR_PRIVATE: C2Rust_Unnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
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
pub struct C2Rust_Unnamed_27 {
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
    pub events: C2Rust_Unnamed_29,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_30,
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
pub struct C2Rust_Unnamed_30 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_32,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
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
    pub events: C2Rust_Unnamed_34,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
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
    pub events: C2Rust_Unnamed_40,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
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
    pub events: C2Rust_Unnamed_38,
    pub WLR_PRIVATE: C2Rust_Unnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_37 {
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
pub struct C2Rust_Unnamed_38 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
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
pub struct C2Rust_Unnamed_40 {
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
    pub layers: C2Rust_Unnamed_97,
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
    pub events: C2Rust_Unnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_41 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_92,
    pub border: C2Rust_Unnamed_91,
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
    pub events: C2Rust_Unnamed_42,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
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
    pub layers: C2Rust_Unnamed_90,
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
    pub events: C2Rust_Unnamed_44,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_43,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_44 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_group_handle_v1 {
    pub manager: *mut wlr_ext_workspace_manager_v1,
    pub caps: uint32_t,
    pub events: C2Rust_Unnamed_46,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_45,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_45 {
    pub outputs: wl_list,
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_manager_v1 {
    pub global: *mut wl_global,
    pub groups: wl_list,
    pub workspaces: wl_list,
    pub events: C2Rust_Unnamed_48,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_47,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
    pub resources: wl_list,
    pub idle_source: *mut wl_event_source,
    pub event_loop: *mut wl_event_loop,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
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
    pub layers: C2Rust_Unnamed_87,
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
    pub session_lock: C2Rust_Unnamed_55,
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
    pub events: C2Rust_Unnamed_50,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_49,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
    pub new_object: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_text_input_manager_v3 {
    pub global: *mut wl_global,
    pub text_inputs: wl_list,
    pub events: C2Rust_Unnamed_52,
    pub WLR_PRIVATE: C2Rust_Unnamed_51,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_52 {
    pub new_text_input: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_manager_v2 {
    pub global: *mut wl_global,
    pub input_methods: wl_list,
    pub events: C2Rust_Unnamed_54,
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
    pub new_input_method: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
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
    pub events: C2Rust_Unnamed_60,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_58,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_58 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_59,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_59 {
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
pub struct C2Rust_Unnamed_60 {
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
    pub events: C2Rust_Unnamed_62,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_61,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_61 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
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
    pub events: C2Rust_Unnamed_64,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_63,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
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
    pub events: C2Rust_Unnamed_66,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_65,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_65 {
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
pub struct C2Rust_Unnamed_66 {
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
    pub events: C2Rust_Unnamed_68,
    pub WLR_PRIVATE: C2Rust_Unnamed_67,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
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
    pub repeat_info: C2Rust_Unnamed_70,
    pub events: C2Rust_Unnamed_69,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
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
    pub events: C2Rust_Unnamed_72,
    pub WLR_PRIVATE: C2Rust_Unnamed_71,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_71 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
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
    pub events: C2Rust_Unnamed_74,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_73,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_73 {
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_74 {
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
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_pointers: wl_list,
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
    pub new_virtual_pointer: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_keyboards: wl_list,
    pub events: C2Rust_Unnamed_84,
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
    pub new_virtual_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_86,
    pub data: *mut ::core::ffi::c_void,
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
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_87 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_89,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_88,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_88 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_89 {
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
pub struct C2Rust_Unnamed_90 {
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
pub struct C2Rust_Unnamed_91 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_92 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_94,
    pub events: C2Rust_Unnamed_93,
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
pub struct C2Rust_Unnamed_93 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_94 {
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
    pub events: C2Rust_Unnamed_96,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_95,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_95 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_96 {
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
pub struct C2Rust_Unnamed_97 {
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
    pub events: C2Rust_Unnamed_99,
    pub WLR_PRIVATE: C2Rust_Unnamed_98,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_98 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_99 {
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
    pub events: C2Rust_Unnamed_101,
    pub WLR_PRIVATE: C2Rust_Unnamed_100,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_100 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_101 {
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
    pub events: C2Rust_Unnamed_102,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_102 {
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
    pub events: C2Rust_Unnamed_103,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_103 {
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
    pub previous: C2Rust_Unnamed_107,
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
    pub events: C2Rust_Unnamed_105,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_104,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_104 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub synced: wlr_surface_synced,
    pub destroying: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_105 {
    pub set_region: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_constraint_v1_state {
    pub committed: uint32_t,
    pub region: pixman_region32_t,
    pub cursor_hint: C2Rust_Unnamed_106,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_106 {
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
pub struct C2Rust_Unnamed_107 {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub node: *mut sway_node,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_direction_0(pub ::core::ffi::c_uint);
impl wlr_direction_0 {
    pub const WLR_DIRECTION_UP: Self = Self(1);
    pub const WLR_DIRECTION_DOWN: Self = Self(2);
    pub const WLR_DIRECTION_LEFT: Self = Self(4);
    pub const WLR_DIRECTION_RIGHT: Self = Self(8);
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
    pub xcursor_theme: C2Rust_Unnamed_108,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_108 {
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
    pub colors: C2Rust_Unnamed_109,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_109 {
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
    pub border_colors: C2Rust_Unnamed_112,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_110,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_110 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_111,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_111 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_112 {
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
pub struct cmd_results {
    pub status: cmd_status,
    pub error: *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct cmd_status(pub ::core::ffi::c_uint);
impl cmd_status {
    pub const CMD_SUCCESS: Self = Self(0);
    pub const CMD_FAILURE: Self = Self(1);
    pub const CMD_INVALID: Self = Self(2);
    pub const CMD_DEFER: Self = Self(3);
    pub const CMD_BLOCK: Self = Self(4);
    pub const CMD_BLOCK_COMMANDS: Self = Self(5);
    pub const CMD_BLOCK_END: Self = Self(6);
}
pub type sway_cmd = unsafe extern "C" fn(
    ::core::ffi::c_int,
    *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct expected_args(pub ::core::ffi::c_uint);
impl expected_args {
    pub const EXPECTED_AT_LEAST: Self = Self(0);
    pub const EXPECTED_AT_MOST: Self = Self(1);
    pub const EXPECTED_EQUAL_TO: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct movement_amount {
    pub amount: ::core::ffi::c_int,
    pub unit: movement_unit,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct movement_unit(pub ::core::ffi::c_uint);
impl movement_unit {
    pub const MOVEMENT_UNIT_PX: Self = Self(0);
    pub const MOVEMENT_UNIT_PPT: Self = Self(1);
    pub const MOVEMENT_UNIT_DEFAULT: Self = Self(2);
    pub const MOVEMENT_UNIT_INVALID: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_113 {
    pub name: *mut ::core::ffi::c_char,
    pub direction: wlr_direction,
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
static mut expected_syntax: [::core::ffi::c_char; 239] = unsafe {
    ::core::mem::transmute::<
        [u8; 239],
        [::core::ffi::c_char; 239],
    >(
        *b"Expected 'move <left|right|up|down> <[px] px>' or 'move [--no-auto-back-and-forth] <container|window> [to] workspace <name>' or 'move <container|window|workspace> [to] output <name|direction>' or 'move <container|window> [to] mark <mark>'\0",
    )
};
unsafe extern "C" fn output_in_direction(
    mut direction_string: *const ::core::ffi::c_char,
    mut reference: *mut sway_output,
    mut ref_lx: ::core::ffi::c_int,
    mut ref_ly: ::core::ffi::c_int,
) -> *mut sway_output {
    if strcasecmp(direction_string, b"current\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        let mut active_ws: *mut sway_workspace = seat_get_focused_workspace(
            (*config).handler_context.seat,
        );
        if active_ws.is_null() {
            return ::core::ptr::null_mut::<sway_output>();
        }
        return (*active_ws).output;
    }
    let mut names: [C2Rust_Unnamed_113; 4] = [
        C2Rust_Unnamed_113 {
            name: b"up\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            direction: wlr_direction(
                wlr_direction_0::WLR_DIRECTION_UP.0 as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            ),
        },
        C2Rust_Unnamed_113 {
            name: b"down\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            direction: wlr_direction(
                wlr_direction_0::WLR_DIRECTION_DOWN.0 as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            ),
        },
        C2Rust_Unnamed_113 {
            name: b"left\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            direction: wlr_direction(
                wlr_direction_0::WLR_DIRECTION_LEFT.0 as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            ),
        },
        C2Rust_Unnamed_113 {
            name: b"right\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            direction: wlr_direction(
                wlr_direction_0::WLR_DIRECTION_RIGHT.0 as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            ),
        },
    ];
    let mut direction: wlr_direction = wlr_direction(
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    let mut i: size_t = 0 as size_t;
    while i
        < ::core::mem::size_of::<[C2Rust_Unnamed_113; 4]>()
            .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_113>())
    {
        if strcasecmp(names[i].name, direction_string) == 0 as ::core::ffi::c_int {
            direction = names[i].direction;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if !reference.is_null() && direction.0 != 0 {
        let mut target: *mut wlr_output = wlr_output_layout_adjacent_output(
            (*root).output_layout,
            direction,
            (*reference).wlr_output,
            ref_lx as ::core::ffi::c_double,
            ref_ly as ::core::ffi::c_double,
        );
        if target.is_null() {
            target = wlr_output_layout_farthest_output(
                (*root).output_layout,
                opposite_direction(direction),
                (*reference).wlr_output,
                ref_lx as ::core::ffi::c_double,
                ref_ly as ::core::ffi::c_double,
            );
        }
        if !target.is_null() {
            return (*target).data as *mut sway_output;
        }
    }
    return output_by_name_or_id(direction_string);
}
unsafe extern "C" fn is_parallel(
    mut layout: sway_container_layout,
    mut dir: wlr_direction,
) -> bool {
    match layout {
        sway_container_layout(4) | sway_container_layout(1) => {
            return dir.0 == wlr_direction_0::WLR_DIRECTION_LEFT.0
                || dir.0 == wlr_direction_0::WLR_DIRECTION_RIGHT.0;
        }
        sway_container_layout(3) | sway_container_layout(2) => {
            return dir.0 == wlr_direction_0::WLR_DIRECTION_UP.0
                || dir.0 == wlr_direction_0::WLR_DIRECTION_DOWN.0;
        }
        _ => return r#false != 0,
    };
}
unsafe extern "C" fn workspace_focus_fullscreen(mut workspace: *mut sway_workspace) {
    if (*workspace).fullscreen.is_null() {
        return;
    }
    let mut seat: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
    let mut focus_ws: *mut sway_workspace = ::core::ptr::null_mut::<sway_workspace>();
    seat = ((*server.input).seats.next as *mut ::core::ffi::c_char)
        .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
        as *mut sway_seat;
    while &raw mut (*seat).link != &raw mut (*server.input).seats {
        focus_ws = seat_get_focused_workspace(seat);
        if focus_ws == workspace {
            let mut new_focus: *mut sway_node = seat_get_focus_inactive(
                seat,
                &raw mut (*(*workspace).fullscreen).node,
            );
            seat_set_raw_focus(seat, new_focus);
        }
        seat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
            as *mut sway_seat;
    }
}
unsafe extern "C" fn container_move_to_container_from_direction(
    mut container: *mut sway_container,
    mut destination: *mut sway_container,
    mut move_dir: wlr_direction,
) {
    if !(*destination).view.is_null() {
        if (*destination).pending.parent == (*container).pending.parent
            && (*destination).pending.workspace == (*container).pending.workspace
        {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Swapping siblings\0".as_ptr() as *const ::core::ffi::c_char,
                b"sway/commands/move.c\0".as_ptr() as *const ::core::ffi::c_char,
                118 as ::core::ffi::c_int,
            );
            let mut siblings: *mut list_t = container_get_siblings(container);
            let mut container_index: ::core::ffi::c_int = list_find(
                siblings,
                container as *const ::core::ffi::c_void,
            );
            let mut destination_index: ::core::ffi::c_int = list_find(
                siblings,
                destination as *const ::core::ffi::c_void,
            );
            list_swap(siblings, container_index, destination_index);
            container_update_representation(container);
        } else {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Promoting to sibling of cousin\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/commands/move.c\0".as_ptr() as *const ::core::ffi::c_char,
                125 as ::core::ffi::c_int,
            );
            let mut offset: ::core::ffi::c_int = (move_dir.0
                == wlr_direction_0::WLR_DIRECTION_LEFT.0
                || move_dir.0 == wlr_direction_0::WLR_DIRECTION_UP.0)
                as ::core::ffi::c_int;
            let mut index: ::core::ffi::c_int = container_sibling_index(destination)
                + offset;
            if !(*destination).pending.parent.is_null() {
                container_insert_child((*destination).pending.parent, container, index);
            } else {
                workspace_insert_tiling(
                    (*destination).pending.workspace,
                    container,
                    index,
                );
            }
            (*container).pending.height = 0 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            (*container).pending.width = (*container).pending.height;
            (*container).height_fraction = 0 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            (*container).width_fraction = (*container).height_fraction;
            workspace_squash((*destination).pending.workspace);
        }
        return;
    }
    if is_parallel((*destination).pending.layout, move_dir) {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Reparenting container (parallel)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/commands/move.c\0".as_ptr() as *const ::core::ffi::c_char,
            143 as ::core::ffi::c_int,
        );
        let mut index_0: ::core::ffi::c_int = if move_dir.0
            == wlr_direction_0::WLR_DIRECTION_RIGHT.0
            || move_dir.0 == wlr_direction_0::WLR_DIRECTION_DOWN.0
        {
            0 as ::core::ffi::c_int
        } else {
            (*(*destination).pending.children).length
        };
        container_insert_child(destination, container, index_0);
        (*container).pending.height = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*container).pending.width = (*container).pending.height;
        (*container).height_fraction = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*container).width_fraction = (*container).height_fraction;
        workspace_squash((*destination).pending.workspace);
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Reparenting container (perpendicular)\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/commands/move.c\0".as_ptr() as *const ::core::ffi::c_char,
        154 as ::core::ffi::c_int,
    );
    let mut focus_inactive: *mut sway_node = seat_get_active_tiling_child(
        (*config).handler_context.seat,
        &raw mut (*destination).node,
    );
    if focus_inactive.is_null() || focus_inactive == &raw mut (*destination).node {
        container_add_child(destination, container);
        return;
    }
    container_move_to_container_from_direction(
        container,
        (*focus_inactive).c2rust_unnamed.sway_container,
        move_dir,
    );
}
unsafe extern "C" fn container_move_to_workspace_from_direction(
    mut container: *mut sway_container,
    mut workspace: *mut sway_workspace,
    mut move_dir: wlr_direction,
) {
    (*container).pending.height = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*container).pending.width = (*container).pending.height;
    (*container).height_fraction = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*container).width_fraction = (*container).height_fraction;
    if is_parallel((*workspace).layout, move_dir) {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Reparenting container (parallel)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/commands/move.c\0".as_ptr() as *const ::core::ffi::c_char,
            175 as ::core::ffi::c_int,
        );
        let mut index: ::core::ffi::c_int = if move_dir.0
            == wlr_direction_0::WLR_DIRECTION_RIGHT.0
            || move_dir.0 == wlr_direction_0::WLR_DIRECTION_DOWN.0
        {
            0 as ::core::ffi::c_int
        } else {
            (*(*workspace).tiling).length
        };
        workspace_insert_tiling(workspace, container, index);
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Reparenting container (perpendicular)\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/commands/move.c\0".as_ptr() as *const ::core::ffi::c_char,
        183 as ::core::ffi::c_int,
    );
    let mut focus_inactive: *mut sway_container = seat_get_focus_inactive_tiling(
        (*config).handler_context.seat,
        workspace,
    );
    if focus_inactive.is_null() {
        workspace_add_tiling(workspace, container);
        return;
    }
    while !(*focus_inactive).pending.parent.is_null() {
        focus_inactive = (*focus_inactive).pending.parent;
    }
    container_move_to_container_from_direction(container, focus_inactive, move_dir);
}
unsafe extern "C" fn container_move_to_workspace(
    mut container: *mut sway_container,
    mut workspace: *mut sway_workspace,
) {
    if (*container).pending.workspace == workspace {
        return;
    }
    let mut old_workspace: *mut sway_workspace = (*container).pending.workspace;
    if container_is_floating(container) {
        let mut old_output: *mut sway_output = (*(*container).pending.workspace).output;
        container_detach(container);
        workspace_add_floating(workspace, container);
        container_handle_fullscreen_reparent(container);
        if old_output != (*workspace).output
            && (*container).pending.fullscreen_mode.0 == 0
        {
            let mut workspace_box: wlr_box = wlr_box {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            let mut old_workspace_box: wlr_box = wlr_box {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            workspace_get_box(workspace, &raw mut workspace_box);
            workspace_get_box(old_workspace, &raw mut old_workspace_box);
            floating_fix_coordinates(
                container,
                &raw mut old_workspace_box,
                &raw mut workspace_box,
            );
            if (*container).scratchpad as ::core::ffi::c_int != 0
                && !(*workspace).output.is_null()
            {
                let mut output_box: wlr_box = wlr_box {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                };
                output_get_box((*workspace).output, &raw mut output_box);
                (*container).transform = workspace_box;
            }
        }
    } else {
        container_detach(container);
        if workspace_is_empty(workspace) as ::core::ffi::c_int != 0
            && !(*container).pending.children.is_null()
        {
            workspace_unwrap_children(workspace, container);
            container_reap_empty(container);
        } else {
            (*container).pending.height = 0 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            (*container).pending.width = (*container).pending.height;
            (*container).height_fraction = 0 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            (*container).width_fraction = (*container).height_fraction;
            workspace_add_tiling(workspace, container);
        }
        container_update_representation(container);
    }
    if !(*container).view.is_null() {
        ipc_event_window(container, b"move\0".as_ptr() as *const ::core::ffi::c_char);
    }
    workspace_detect_urgent(old_workspace);
    workspace_detect_urgent(workspace);
    workspace_focus_fullscreen(workspace);
}
unsafe extern "C" fn container_move_to_container(
    mut container: *mut sway_container,
    mut destination: *mut sway_container,
) {
    if container == destination
        || container_has_ancestor(destination, container) as ::core::ffi::c_int != 0
    {
        return;
    }
    if container_is_floating(container) {
        container_move_to_workspace(container, (*destination).pending.workspace);
        return;
    }
    let mut old_workspace: *mut sway_workspace = (*container).pending.workspace;
    container_detach(container);
    (*container).pending.height = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*container).pending.width = (*container).pending.height;
    (*container).height_fraction = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*container).width_fraction = (*container).height_fraction;
    if !(*destination).view.is_null() {
        container_add_sibling(destination, container, true);
    } else {
        container_add_child(destination, container);
    }
    if !(*container).view.is_null() {
        ipc_event_window(container, b"move\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if !(*destination).pending.workspace.is_null() {
        workspace_focus_fullscreen((*destination).pending.workspace);
        workspace_detect_urgent((*destination).pending.workspace);
    }
    if !old_workspace.is_null() && old_workspace != (*destination).pending.workspace {
        workspace_detect_urgent(old_workspace);
    }
}
unsafe extern "C" fn container_move_to_next_output(
    mut container: *mut sway_container,
    mut output: *mut sway_output,
    mut move_dir: wlr_direction,
) -> bool {
    let mut next_output: *mut sway_output = output_get_in_direction(output, move_dir);
    if !next_output.is_null() {
        let mut ws: *mut sway_workspace = output_get_active_workspace(next_output);
        if !_sway_assert(
            !ws.is_null(),
            b"[%s:%d] %s:Expected output to have a workspace\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/commands/move.c\0".as_ptr() as *const ::core::ffi::c_char,
            283 as ::core::ffi::c_int,
            b"_Bool container_move_to_next_output(struct sway_container *, struct sway_output *, enum wlr_direction)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        ) {
            return r#false != 0;
        }
        match (*container).pending.fullscreen_mode {
            sway_fullscreen_mode::FULLSCREEN_NONE => {
                container_move_to_workspace_from_direction(container, ws, move_dir);
                return r#true != 0;
            }
            sway_fullscreen_mode::FULLSCREEN_WORKSPACE => {
                container_move_to_workspace(container, ws);
                return r#true != 0;
            }
            sway_fullscreen_mode::FULLSCREEN_GLOBAL => return r#false != 0,
            _ => {}
        }
    }
    return r#false != 0;
}
unsafe extern "C" fn container_move_in_direction(
    mut container: *mut sway_container,
    mut move_dir: wlr_direction,
) -> bool {
    match (*container).pending.fullscreen_mode {
        sway_fullscreen_mode::FULLSCREEN_WORKSPACE => {
            return container_move_to_next_output(
                container,
                (*(*container).pending.workspace).output,
                move_dir,
            );
        }
        sway_fullscreen_mode::FULLSCREEN_GLOBAL => return r#false != 0,
        sway_fullscreen_mode::FULLSCREEN_NONE | _ => {}
    }
    let mut offs: ::core::ffi::c_int = if move_dir.0
        == wlr_direction_0::WLR_DIRECTION_LEFT.0
        || move_dir.0 == wlr_direction_0::WLR_DIRECTION_UP.0
    {
        -1 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    let mut index: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut desired: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut siblings: *mut list_t = ::core::ptr::null_mut::<list_t>();
    let mut target: *mut sway_container = ::core::ptr::null_mut::<sway_container>();
    let mut ancestor: *mut sway_container = ::core::ptr::null_mut::<sway_container>();
    let mut current: *mut sway_container = container;
    let mut wrapped: bool = r#false != 0;
    while ancestor.is_null() {
        if (*current).pending.fullscreen_mode.0 != 0
            || container_is_floating(current) as ::core::ffi::c_int != 0
        {
            return r#false != 0;
        }
        let mut parent_layout: sway_container_layout = container_parent_layout(current);
        if !is_parallel(parent_layout, move_dir) {
            if (*current).pending.parent.is_null() {
                current = workspace_wrap_children((*current).pending.workspace);
                (*(*current).pending.workspace).layout = sway_container_layout(
                    (if move_dir.0 == wlr_direction_0::WLR_DIRECTION_LEFT.0
                        || move_dir.0 == wlr_direction_0::WLR_DIRECTION_RIGHT.0
                    {
                        sway_container_layout_0::L_HORIZ.0 as ::core::ffi::c_int
                    } else {
                        sway_container_layout_0::L_VERT.0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                );
                (*container).pending.width = 0 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                (*container).pending.height = (*container).pending.width;
                (*container).width_fraction = 0 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                (*container).height_fraction = (*container).width_fraction;
                workspace_update_representation((*current).pending.workspace);
                wrapped = r#true != 0;
            } else {
                current = (*current).pending.parent;
            }
        } else {
            siblings = container_get_siblings(current);
            index = list_find(siblings, current as *const ::core::ffi::c_void);
            desired = index + offs;
            target = (if desired == -1 as ::core::ffi::c_int
                || desired == (*siblings).length
            {
                NULL
            } else {
                *(*siblings).items.offset(desired as isize)
            }) as *mut sway_container;
            if current == container {
                if !target.is_null() {
                    container_move_to_container_from_direction(
                        container,
                        target,
                        move_dir,
                    );
                    return r#true != 0;
                } else if (*container).pending.parent.is_null() {
                    return container_move_to_next_output(
                        container,
                        (*(*current).pending.workspace).output,
                        move_dir,
                    )
                } else {
                    current = (*current).pending.parent;
                }
            } else {
                ancestor = current;
            }
        }
    }
    if !target.is_null() {
        container_move_to_container_from_direction(container, target, move_dir);
        return r#true != 0;
    } else if !wrapped && (*(*container).pending.parent).pending.parent.is_null()
        && (*(*(*container).pending.parent).pending.children).length
            == 1 as ::core::ffi::c_int
    {
        return container_move_to_next_output(
            container,
            (*(*ancestor).pending.workspace).output,
            move_dir,
        )
    } else {
        let mut old_parent: *mut sway_container = (*container).pending.parent;
        if !(*ancestor).pending.parent.is_null() {
            container_insert_child(
                (*ancestor).pending.parent,
                container,
                index
                    + if offs < 0 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    },
            );
        } else {
            workspace_insert_tiling(
                (*ancestor).pending.workspace,
                container,
                index
                    + if offs < 0 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    },
            );
        }
        (*ancestor).pending.width = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*ancestor).pending.height = (*ancestor).pending.width;
        (*ancestor).width_fraction = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*ancestor).height_fraction = (*ancestor).width_fraction;
        if !old_parent.is_null() {
            container_reap_empty(old_parent);
        }
        workspace_squash((*container).pending.workspace);
        return r#true != 0;
    };
}
unsafe extern "C" fn cmd_move_container(
    mut no_auto_back_and_forth: bool,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    let mut error: *mut cmd_results = ::core::ptr::null_mut::<cmd_results>();
    error = checkarg(
        argc,
        b"move container/window\0".as_ptr() as *const ::core::ffi::c_char,
        expected_args::EXPECTED_AT_LEAST,
        2 as ::core::ffi::c_int,
    );
    if !error.is_null() {
        return error;
    }
    let mut node: *mut sway_node = (*config).handler_context.node;
    let mut workspace: *mut sway_workspace = (*config).handler_context.workspace;
    let mut container: *mut sway_container = (*config).handler_context.container;
    if (*node).r#type.0 == sway_node_type::N_WORKSPACE.0 {
        if (*(*workspace).tiling).length == 0 as ::core::ffi::c_int {
            return cmd_results_new(
                cmd_status::CMD_FAILURE,
                b"Can't move an empty workspace\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        container = workspace_wrap_children(workspace);
    }
    if (*container).pending.fullscreen_mode.0
        == sway_fullscreen_mode::FULLSCREEN_GLOBAL.0
    {
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"Can't move fullscreen global container\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut seat: *mut sway_seat = (*config).handler_context.seat;
    let mut old_parent: *mut sway_container = (*container).pending.parent;
    let mut old_ws: *mut sway_workspace = (*container).pending.workspace;
    let mut old_output: *mut sway_output = if !old_ws.is_null() {
        (*old_ws).output
    } else {
        ::core::ptr::null_mut::<sway_output>()
    };
    let mut destination: *mut sway_node = ::core::ptr::null_mut::<sway_node>();
    if strcasecmp(
        *argv.offset(0isize),
        b"workspace\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut ws: *mut sway_workspace = ::core::ptr::null_mut::<sway_workspace>();
        let mut ws_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if strcasecmp(
            *argv.offset(1isize),
            b"next\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                *argv.offset(1isize),
                b"prev\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                *argv.offset(1isize),
                b"next_on_output\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                *argv.offset(1isize),
                b"prev_on_output\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                *argv.offset(1isize),
                b"current\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            ws = workspace_by_name(*argv.offset(1isize));
        } else if strcasecmp(
            *argv.offset(1isize),
            b"back_and_forth\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            ws = workspace_by_name(*argv.offset(1isize));
            if ws.is_null() {
                if !(*seat).prev_workspace_name.is_null() {
                    ws_name = strdup((*seat).prev_workspace_name);
                } else {
                    return cmd_results_new(
                        cmd_status::CMD_FAILURE,
                        b"No workspace was previously active.\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    )
                }
            }
        } else {
            if strcasecmp(
                *argv.offset(1isize),
                b"number\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                if argc < 3 as ::core::ffi::c_int {
                    return cmd_results_new(
                        cmd_status::CMD_INVALID,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw const expected_syntax as *const ::core::ffi::c_char,
                    );
                }
                if *(*__ctype_b_loc())
                    .offset(
                        *(*argv.offset(2isize)).offset(0isize) as ::core::ffi::c_int
                            as isize,
                    ) as ::core::ffi::c_int
                    & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int
                        as ::core::ffi::c_ushort as ::core::ffi::c_int == 0
                {
                    return cmd_results_new(
                        cmd_status::CMD_INVALID,
                        b"Invalid workspace number '%s'\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        *argv.offset(2isize),
                    );
                }
                ws_name = join_args(
                    argv.offset(2 as ::core::ffi::c_int as isize),
                    argc - 2 as ::core::ffi::c_int,
                );
                ws = workspace_by_number(ws_name);
            } else {
                ws_name = join_args(
                    argv.offset(1 as ::core::ffi::c_int as isize),
                    argc - 1 as ::core::ffi::c_int,
                );
                ws = workspace_by_name(ws_name);
            }
            if !no_auto_back_and_forth
                && (*config).auto_back_and_forth as ::core::ffi::c_int != 0
                && !(*seat).prev_workspace_name.is_null()
            {
                if !old_ws.is_null() && !(*old_ws).name.is_null()
                    && strcmp((*old_ws).name, ws_name) == 0 as ::core::ffi::c_int
                {
                    free(ws_name as *mut ::core::ffi::c_void);
                    ws_name = strdup((*seat).prev_workspace_name);
                    ws = workspace_by_name(ws_name);
                }
            }
        }
        if ws.is_null() {
            if container_is_sticky_or_child(container) {
                let mut new_output: *mut sway_output = workspace_get_initial_output(
                    ws_name,
                );
                if old_output == new_output {
                    free(ws_name as *mut ::core::ffi::c_void);
                    return cmd_results_new(
                        cmd_status::CMD_FAILURE,
                        b"Can't move sticky container to another workspace on the same output\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            }
            ws = workspace_create(::core::ptr::null_mut::<sway_output>(), ws_name);
            arrange_workspace(ws);
        }
        free(ws_name as *mut ::core::ffi::c_void);
        let mut dst: *mut sway_container = seat_get_focus_inactive_tiling(seat, ws);
        destination = if !dst.is_null() {
            &raw mut (*dst).node
        } else {
            &raw mut (*ws).node
        };
    } else if strcasecmp(
        *argv.offset(0isize),
        b"output\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut new_output_0: *mut sway_output = output_in_direction(
            *argv.offset(1isize),
            old_output,
            (*container).pending.x as ::core::ffi::c_int,
            (*container).pending.y as ::core::ffi::c_int,
        );
        if new_output_0.is_null() {
            return cmd_results_new(
                cmd_status::CMD_FAILURE,
                b"Can't find output with name/direction '%s'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                *argv.offset(1isize),
            );
        }
        destination = seat_get_focus_inactive(seat, &raw mut (*new_output_0).node);
    } else if strcasecmp(
        *argv.offset(0isize),
        b"mark\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut dest_con: *mut sway_container = container_find_mark(
            *argv.offset(1isize),
        );
        if dest_con.is_null() {
            return cmd_results_new(
                cmd_status::CMD_FAILURE,
                b"Mark '%s' not found\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(1isize),
            );
        }
        destination = &raw mut (*dest_con).node;
    } else {
        return cmd_results_new(
            cmd_status::CMD_INVALID,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const expected_syntax as *const ::core::ffi::c_char,
        )
    }
    if (*destination).r#type.0 == sway_node_type::N_CONTAINER.0
        && container_is_scratchpad_hidden((*destination).c2rust_unnamed.sway_container)
            as ::core::ffi::c_int != 0
    {
        return cmd_move_to_scratchpad();
    }
    if container_is_sticky_or_child(container) as ::core::ffi::c_int != 0
        && !old_output.is_null()
        && node_has_ancestor(destination, &raw mut (*old_output).node)
            as ::core::ffi::c_int != 0
    {
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"Can't move sticky container to another workspace on the same output\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut new_output_1: *mut sway_output = node_get_output(destination);
    let mut new_output_last_ws: *mut sway_workspace = ::core::ptr::null_mut::<
        sway_workspace,
    >();
    if !new_output_1.is_null() && old_output != new_output_1 {
        new_output_last_ws = output_get_active_workspace(new_output_1);
    }
    let mut focus: *mut sway_node = seat_get_focus(seat);
    if container_is_scratchpad_hidden_or_child(container) {
        container_detach(container);
        root_scratchpad_show(container);
    }
    match (*destination).r#type {
        sway_node_type::N_WORKSPACE => {
            container_move_to_workspace(
                container,
                (*destination).c2rust_unnamed.sway_workspace,
            );
        }
        sway_node_type::N_OUTPUT => {
            let mut output: *mut sway_output = (*destination).c2rust_unnamed.sway_output;
            let mut ws_0: *mut sway_workspace = output_get_active_workspace(output);
            if !_sway_assert(
                !ws_0.is_null(),
                b"[%s:%d] %s:Expected output to have a workspace\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/commands/move.c\0".as_ptr() as *const ::core::ffi::c_char,
                569 as ::core::ffi::c_int,
                b"struct cmd_results *cmd_move_container(_Bool, int, char **)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            ) {
                return cmd_results_new(
                    cmd_status::CMD_FAILURE,
                    b"Expected output to have a workspace\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            container_move_to_workspace(container, ws_0);
        }
        sway_node_type::N_CONTAINER => {
            container_move_to_container(
                container,
                (*destination).c2rust_unnamed.sway_container,
            );
        }
        sway_node_type::N_ROOT | _ => {}
    }
    let mut new_workspace: *mut sway_workspace = if !new_output_1.is_null() {
        output_get_active_workspace(new_output_1)
    } else {
        ::core::ptr::null_mut::<sway_workspace>()
    };
    if !new_output_1.is_null()
        && !_sway_assert(
            !new_workspace.is_null(),
            b"[%s:%d] %s:Expected output to have a workspace\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/commands/move.c\0".as_ptr() as *const ::core::ffi::c_char,
            587 as ::core::ffi::c_int,
            b"struct cmd_results *cmd_move_container(_Bool, int, char **)\0".as_ptr()
                as *const ::core::ffi::c_char,
        )
    {
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"Expected output to have a workspace\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !new_output_last_ws.is_null() && new_output_last_ws != new_workspace {
        let mut new_output_last_focus: *mut sway_node = seat_get_focus_inactive(
            seat,
            &raw mut (*new_output_last_ws).node,
        );
        seat_set_raw_focus(seat, new_output_last_focus);
    }
    if focus == &raw mut (*container).node {
        focus = ::core::ptr::null_mut::<sway_node>();
        if !old_parent.is_null() {
            focus = seat_get_focus_inactive(seat, &raw mut (*old_parent).node);
        }
        if focus.is_null() && !old_ws.is_null() {
            focus = seat_get_focus_inactive(seat, &raw mut (*old_ws).node);
        }
    }
    seat_set_focus(seat, focus);
    if !old_parent.is_null() {
        container_reap_empty(old_parent);
    } else if !old_ws.is_null() {
        workspace_consider_destroy(old_ws);
    }
    if !(*root).fullscreen_global.is_null() {
        arrange_root();
    } else {
        if !old_ws.is_null() && !(*old_ws).node.destroying {
            arrange_workspace(old_ws);
        }
        arrange_node(node_get_parent(destination));
    }
    return cmd_results_new(
        cmd_status::CMD_SUCCESS,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn cmd_move_workspace(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    let mut error: *mut cmd_results = ::core::ptr::null_mut::<cmd_results>();
    error = checkarg(
        argc,
        b"move workspace\0".as_ptr() as *const ::core::ffi::c_char,
        expected_args::EXPECTED_AT_LEAST,
        1 as ::core::ffi::c_int,
    );
    if !error.is_null() {
        return error;
    }
    if strcasecmp(
        *argv.offset(0isize),
        b"output\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        argc -= 1;
        argv = argv.offset(1);
    }
    if argc == 0 {
        return cmd_results_new(
            cmd_status::CMD_INVALID,
            b"Expected 'move workspace to [output] <output>'\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut workspace: *mut sway_workspace = (*config).handler_context.workspace;
    if workspace.is_null() {
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"No workspace to move\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut old_output: *mut sway_output = (*workspace).output;
    let mut center_x: ::core::ffi::c_int = (((*workspace).width
        / 2 as ::core::ffi::c_int) as ::core::ffi::c_double + (*workspace).x)
        as ::core::ffi::c_int;
    let mut center_y: ::core::ffi::c_int = (((*workspace).height
        / 2 as ::core::ffi::c_int) as ::core::ffi::c_double + (*workspace).y)
        as ::core::ffi::c_int;
    let mut new_output: *mut sway_output = output_in_direction(
        *argv.offset(0isize),
        old_output,
        center_x,
        center_y,
    );
    if new_output.is_null() {
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"Can't find output with name/direction '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            *argv.offset(0isize),
        );
    }
    workspace_move_to_output(workspace, new_output);
    arrange_output(old_output);
    arrange_output(new_output);
    let mut seat: *mut sway_seat = (*config).handler_context.seat;
    let mut focus: *mut sway_node = seat_get_focus_inactive(
        seat,
        &raw mut (*workspace).node,
    );
    seat_set_focus(seat, focus);
    seat_consider_warp_to_focus(seat);
    return cmd_results_new(
        cmd_status::CMD_SUCCESS,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn cmd_move_in_direction(
    mut direction: wlr_direction,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    let mut move_amt: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    if argc != 0 {
        let mut inv: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        move_amt = strtol(*argv.offset(0isize), &raw mut inv, 10 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        if *inv as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && strcasecmp(inv, b"px\0".as_ptr() as *const ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
        {
            return cmd_results_new(
                cmd_status::CMD_FAILURE,
                b"Invalid distance specified\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    let mut container: *mut sway_container = (*config).handler_context.container;
    if container.is_null() {
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"Cannot move workspaces in a direction\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if container_is_floating(container) {
        if (*container).pending.fullscreen_mode.0 != 0 {
            return cmd_results_new(
                cmd_status::CMD_FAILURE,
                b"Cannot move fullscreen floating container\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        let mut lx: ::core::ffi::c_double = (*container).pending.x;
        let mut ly: ::core::ffi::c_double = (*container).pending.y;
        match direction {
            wlr_direction(4) => {
                lx -= move_amt as ::core::ffi::c_double;
            }
            wlr_direction(8) => {
                lx += move_amt as ::core::ffi::c_double;
            }
            wlr_direction(1) => {
                ly -= move_amt as ::core::ffi::c_double;
            }
            wlr_direction(2) => {
                ly += move_amt as ::core::ffi::c_double;
            }
            _ => {}
        }
        container_floating_move_to(container, lx, ly);
        return cmd_results_new(
            cmd_status::CMD_SUCCESS,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    let mut old_ws: *mut sway_workspace = (*container).pending.workspace;
    let mut old_parent: *mut sway_container = (*container).pending.parent;
    if !container_move_in_direction(container, direction) {
        return cmd_results_new(
            cmd_status::CMD_SUCCESS,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if !old_parent.is_null() {
        container_reap_empty(old_parent);
    } else if !old_ws.is_null() {
        workspace_consider_destroy(old_ws);
    }
    let mut new_ws: *mut sway_workspace = (*container).pending.workspace;
    if !(*root).fullscreen_global.is_null() {
        arrange_root();
    } else {
        arrange_workspace(old_ws);
        if new_ws != old_ws {
            arrange_workspace(new_ws);
        }
    }
    if !(*container).view.is_null() {
        ipc_event_window(container, b"move\0".as_ptr() as *const ::core::ffi::c_char);
    }
    container_end_mouse_operation(container);
    return cmd_results_new(
        cmd_status::CMD_SUCCESS,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn cmd_move_to_position_pointer(
    mut container: *mut sway_container,
) -> *mut cmd_results {
    let mut seat: *mut sway_seat = (*config).handler_context.seat;
    if (*seat).cursor.is_null() {
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"No cursor device\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut cursor: *mut wlr_cursor = (*(*seat).cursor).cursor;
    let mut lx: ::core::ffi::c_double = (*cursor).x
        - (*container).pending.width / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut ly: ::core::ffi::c_double = (*cursor).y
        - (*container).pending.height / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut output: *mut wlr_output = wlr_output_layout_output_at(
        (*root).output_layout,
        (*cursor).x,
        (*cursor).y,
    );
    if !output.is_null() {
        let mut r#box: wlr_box = wlr_box {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        wlr_output_layout_get_box((*root).output_layout, output, &raw mut r#box);
        lx = fmax(lx, r#box.x as ::core::ffi::c_double);
        ly = fmax(ly, r#box.y as ::core::ffi::c_double);
        if lx + (*container).pending.width
            > (r#box.x + r#box.width) as ::core::ffi::c_double
        {
            lx = (r#box.x + r#box.width) as ::core::ffi::c_double
                - (*container).pending.width;
        }
        if ly + (*container).pending.height
            > (r#box.y + r#box.height) as ::core::ffi::c_double
        {
            ly = (r#box.y + r#box.height) as ::core::ffi::c_double
                - (*container).pending.height;
        }
    }
    container_floating_move_to(container, lx, ly);
    return cmd_results_new(
        cmd_status::CMD_SUCCESS,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
static mut expected_position_syntax: [::core::ffi::c_char; 131] = unsafe {
    ::core::mem::transmute::<
        [u8; 131],
        [::core::ffi::c_char; 131],
    >(
        *b"Expected 'move [absolute] position <x> [px] <y> [px]' or 'move [absolute] position center' or 'move position cursor|mouse|pointer'\0",
    )
};
unsafe extern "C" fn cmd_move_to_position(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    let mut container: *mut sway_container = (*config).handler_context.container;
    if container.is_null() || !container_is_floating(container) {
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"Only floating containers can be moved to an absolute position\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if argc == 0 {
        return cmd_results_new(
            cmd_status::CMD_INVALID,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const expected_position_syntax as *const ::core::ffi::c_char,
        );
    }
    let mut absolute: bool = r#false != 0;
    if strcmp(*argv.offset(0isize), b"absolute\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        absolute = r#true != 0;
        argc -= 1;
        argv = argv.offset(1);
    }
    if argc == 0 {
        return cmd_results_new(
            cmd_status::CMD_INVALID,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const expected_position_syntax as *const ::core::ffi::c_char,
        );
    }
    if strcmp(*argv.offset(0isize), b"position\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        argc -= 1;
        argv = argv.offset(1);
    }
    if argc == 0 {
        return cmd_results_new(
            cmd_status::CMD_INVALID,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const expected_position_syntax as *const ::core::ffi::c_char,
        );
    }
    if strcmp(*argv.offset(0isize), b"cursor\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcmp(
            *argv.offset(0isize),
            b"mouse\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            *argv.offset(0isize),
            b"pointer\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        if absolute {
            return cmd_results_new(
                cmd_status::CMD_INVALID,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const expected_position_syntax as *const ::core::ffi::c_char,
            );
        }
        return cmd_move_to_position_pointer(container);
    } else if strcmp(
        *argv.offset(0isize),
        b"center\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut lx: ::core::ffi::c_double = 0.;
        let mut ly: ::core::ffi::c_double = 0.;
        if absolute {
            lx = (*root).x
                + ((*root).width - (*container).pending.width)
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            ly = (*root).y
                + ((*root).height - (*container).pending.height)
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            let mut ws: *mut sway_workspace = (*container).pending.workspace;
            if ws.is_null() {
                let mut seat: *mut sway_seat = (*config).handler_context.seat;
                ws = seat_get_focused_workspace(seat);
            }
            lx = (*ws).x
                + ((*ws).width as ::core::ffi::c_double - (*container).pending.width)
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            ly = (*ws).y
                + ((*ws).height as ::core::ffi::c_double - (*container).pending.height)
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        container_floating_move_to(container, lx, ly);
        return cmd_results_new(
            cmd_status::CMD_SUCCESS,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if argc < 2 as ::core::ffi::c_int {
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const expected_position_syntax as *const ::core::ffi::c_char,
        );
    }
    let mut lx_0: movement_amount = movement_amount {
        amount: 0 as ::core::ffi::c_int,
        unit: movement_unit::MOVEMENT_UNIT_INVALID,
    };
    let mut num_consumed_args: ::core::ffi::c_int = parse_movement_amount(
        argc,
        argv,
        &raw mut lx_0,
    );
    argc -= num_consumed_args;
    argv = argv.offset(num_consumed_args as isize);
    if lx_0.unit.0 == movement_unit::MOVEMENT_UNIT_INVALID.0 {
        return cmd_results_new(
            cmd_status::CMD_INVALID,
            b"Invalid x position specified\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if argc < 1 as ::core::ffi::c_int {
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const expected_position_syntax as *const ::core::ffi::c_char,
        );
    }
    let mut ly_0: movement_amount = movement_amount {
        amount: 0 as ::core::ffi::c_int,
        unit: movement_unit::MOVEMENT_UNIT_INVALID,
    };
    num_consumed_args = parse_movement_amount(argc, argv, &raw mut ly_0);
    argc -= num_consumed_args;
    argv = argv.offset(num_consumed_args as isize);
    if argc > 0 as ::core::ffi::c_int {
        return cmd_results_new(
            cmd_status::CMD_INVALID,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const expected_position_syntax as *const ::core::ffi::c_char,
        );
    }
    if ly_0.unit.0 == movement_unit::MOVEMENT_UNIT_INVALID.0 {
        return cmd_results_new(
            cmd_status::CMD_INVALID,
            b"Invalid y position specified\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut ws_0: *mut sway_workspace = (*container).pending.workspace;
    if ws_0.is_null() {
        let mut seat_0: *mut sway_seat = (*config).handler_context.seat;
        ws_0 = seat_get_focused_workspace(seat_0);
    }
    match lx_0.unit {
        movement_unit::MOVEMENT_UNIT_PPT => {
            if container_is_scratchpad_hidden(container) {
                return cmd_results_new(
                    cmd_status::CMD_FAILURE,
                    b"Cannot move a hidden scratchpad container by ppt\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if absolute {
                return cmd_results_new(
                    cmd_status::CMD_FAILURE,
                    b"Cannot move to absolute positions by ppt\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            lx_0.amount = (*ws_0).width * lx_0.amount / 100 as ::core::ffi::c_int;
            lx_0.unit = movement_unit::MOVEMENT_UNIT_PX;
        }
        movement_unit::MOVEMENT_UNIT_INVALID => {
            _sway_assert(
                false,
                b"[%s:%d] %s:invalid x unit\0".as_ptr() as *const ::core::ffi::c_char,
                b"sway/commands/move.c\0".as_ptr() as *const ::core::ffi::c_char,
                888 as ::core::ffi::c_int,
                b"struct cmd_results *cmd_move_to_position(int, char **)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        movement_unit::MOVEMENT_UNIT_PX | movement_unit::MOVEMENT_UNIT_DEFAULT | _ => {}
    }
    match ly_0.unit {
        movement_unit::MOVEMENT_UNIT_PPT => {
            if container_is_scratchpad_hidden(container) {
                return cmd_results_new(
                    cmd_status::CMD_FAILURE,
                    b"Cannot move a hidden scratchpad container by ppt\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if absolute {
                return cmd_results_new(
                    cmd_status::CMD_FAILURE,
                    b"Cannot move to absolute positions by ppt\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            ly_0.amount = (*ws_0).height * ly_0.amount / 100 as ::core::ffi::c_int;
            ly_0.unit = movement_unit::MOVEMENT_UNIT_PX;
        }
        movement_unit::MOVEMENT_UNIT_INVALID => {
            _sway_assert(
                false,
                b"[%s:%d] %s:invalid y unit\0".as_ptr() as *const ::core::ffi::c_char,
                b"sway/commands/move.c\0".as_ptr() as *const ::core::ffi::c_char,
                910 as ::core::ffi::c_int,
                b"struct cmd_results *cmd_move_to_position(int, char **)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        movement_unit::MOVEMENT_UNIT_PX | movement_unit::MOVEMENT_UNIT_DEFAULT | _ => {}
    }
    if !absolute {
        lx_0.amount = (lx_0.amount as ::core::ffi::c_double + (*ws_0).x)
            as ::core::ffi::c_int;
        ly_0.amount = (ly_0.amount as ::core::ffi::c_double + (*ws_0).y)
            as ::core::ffi::c_int;
    }
    container_floating_move_to(
        container,
        lx_0.amount as ::core::ffi::c_double,
        ly_0.amount as ::core::ffi::c_double,
    );
    return cmd_results_new(
        cmd_status::CMD_SUCCESS,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn cmd_move_to_scratchpad() -> *mut cmd_results {
    let mut node: *mut sway_node = (*config).handler_context.node;
    let mut con: *mut sway_container = (*config).handler_context.container;
    let mut ws: *mut sway_workspace = (*config).handler_context.workspace;
    if (*node).r#type.0 == sway_node_type::N_WORKSPACE.0
        && (*(*ws).tiling).length == 0 as ::core::ffi::c_int
    {
        return cmd_results_new(
            cmd_status::CMD_INVALID,
            b"Can't move an empty workspace to the scratchpad\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if (*node).r#type.0 == sway_node_type::N_WORKSPACE.0 {
        con = workspace_wrap_children(ws);
        (*ws).layout = sway_container_layout(
            sway_container_layout_0::L_HORIZ.0 as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
    }
    if container_is_floating_or_child(con) {
        while !(*con).pending.parent.is_null() {
            con = (*con).pending.parent;
        }
    }
    if !(*con).scratchpad {
        root_scratchpad_add_container(con, ::core::ptr::null_mut::<sway_workspace>());
    } else if !(*con).pending.workspace.is_null() {
        root_scratchpad_hide(con);
    }
    return cmd_results_new(
        cmd_status::CMD_SUCCESS,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
static mut expected_full_syntax: [::core::ffi::c_char; 612] = unsafe {
    ::core::mem::transmute::<
        [u8; 612],
        [::core::ffi::c_char; 612],
    >(
        *b"Expected 'move left|right|up|down [<amount> [px]]' or 'move [--no-auto-back-and-forth] [window|container] [to] workspace  <name>|next|prev|next_on_output|prev_on_output|current|(number <num>)' or 'move [window|container] [to] output <name/id>|left|right|up|down' or 'move [window|container] [to] mark <mark>' or 'move [window|container] [to] scratchpad' or 'move workspace to [output] <name/id>|left|right|up|down' or 'move [window|container] [to] [absolute] position <x> [px] <y> [px]' or 'move [window|container] [to] [absolute] position center' or 'move [window|container] [to] position mouse|cursor|pointer'\0",
    )
};
#[no_mangle]
pub unsafe extern "C" fn cmd_move(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    let mut error: *mut cmd_results = ::core::ptr::null_mut::<cmd_results>();
    error = checkarg(
        argc,
        b"move\0".as_ptr() as *const ::core::ffi::c_char,
        expected_args::EXPECTED_AT_LEAST,
        1 as ::core::ffi::c_int,
    );
    if !error.is_null() {
        return error;
    }
    if (*(*root).outputs).length == 0 {
        return cmd_results_new(
            cmd_status::CMD_INVALID,
            b"Can't run this command while there's no outputs connected.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if strcasecmp(*argv.offset(0isize), b"left\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        argc -= 1;
        argv = argv.offset(1);
        return cmd_move_in_direction(
            wlr_direction(
                wlr_direction_0::WLR_DIRECTION_LEFT.0 as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            ),
            argc,
            argv,
        );
    } else if strcasecmp(
        *argv.offset(0isize),
        b"right\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        argc -= 1;
        argv = argv.offset(1);
        return cmd_move_in_direction(
            wlr_direction(
                wlr_direction_0::WLR_DIRECTION_RIGHT.0 as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            ),
            argc,
            argv,
        );
    } else if strcasecmp(
        *argv.offset(0isize),
        b"up\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        argc -= 1;
        argv = argv.offset(1);
        return cmd_move_in_direction(
            wlr_direction(
                wlr_direction_0::WLR_DIRECTION_UP.0 as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            ),
            argc,
            argv,
        );
    } else if strcasecmp(
        *argv.offset(0isize),
        b"down\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        argc -= 1;
        argv = argv.offset(1);
        return cmd_move_in_direction(
            wlr_direction(
                wlr_direction_0::WLR_DIRECTION_DOWN.0 as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            ),
            argc,
            argv,
        );
    } else if strcasecmp(
        *argv.offset(0isize),
        b"workspace\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int && argc >= 2 as ::core::ffi::c_int
        && (strcasecmp(
            *argv.offset(1isize),
            b"to\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                *argv.offset(1isize),
                b"output\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
    {
        argc -= 2 as ::core::ffi::c_int;
        argv = argv.offset(2 as ::core::ffi::c_int as isize);
        return cmd_move_workspace(argc, argv);
    }
    let mut no_auto_back_and_forth: bool = r#false != 0;
    if strcasecmp(
        *argv.offset(0isize),
        b"--no-auto-back-and-forth\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        no_auto_back_and_forth = r#true != 0;
        argc -= 1;
        argv = argv.offset(1);
    }
    if argc > 0 as ::core::ffi::c_int
        && (strcasecmp(
            *argv.offset(0isize),
            b"window\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                *argv.offset(0isize),
                b"container\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
    {
        argc -= 1;
        argv = argv.offset(1);
    }
    if argc > 0 as ::core::ffi::c_int
        && strcasecmp(
            *argv.offset(0isize),
            b"to\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        argc -= 1;
        argv = argv.offset(1);
    }
    if argc == 0 {
        return cmd_results_new(
            cmd_status::CMD_INVALID,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const expected_full_syntax as *const ::core::ffi::c_char,
        );
    }
    if no_auto_back_and_forth as ::core::ffi::c_int != 0
        && strcasecmp(
            *argv.offset(0isize),
            b"workspace\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        return cmd_results_new(
            cmd_status::CMD_INVALID,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const expected_full_syntax as *const ::core::ffi::c_char,
        );
    }
    if strcasecmp(
        *argv.offset(0isize),
        b"workspace\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            *argv.offset(0isize),
            b"output\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            *argv.offset(0isize),
            b"mark\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        return cmd_move_container(no_auto_back_and_forth, argc, argv)
    } else if strcasecmp(
        *argv.offset(0isize),
        b"scratchpad\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return cmd_move_to_scratchpad()
    } else if strcasecmp(
        *argv.offset(0isize),
        b"position\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || argc > 1 as ::core::ffi::c_int
            && strcasecmp(
                *argv.offset(0isize),
                b"absolute\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            && strcasecmp(
                *argv.offset(1isize),
                b"position\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
    {
        return cmd_move_to_position(argc, argv)
    }
    return cmd_results_new(
        cmd_status::CMD_INVALID,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const expected_full_syntax as *const ::core::ffi::c_char,
    );
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
