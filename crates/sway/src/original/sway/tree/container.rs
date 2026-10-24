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
    pub type xkb_state;
    pub type xkb_keymap;
    pub type wlr_keyboard_group;
    pub type wlr_keyboard_impl;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_primary_selection_source;
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
    pub type wlr_ext_foreign_toplevel_list_v1;
    pub type wlr_output_power_manager_v1;
    pub type wlr_session_lock_manager_v1;
    pub type wlr_session_lock_v1;
    pub type wlr_output_manager_v1;
    pub type wlr_xdg_output_manager_v1;
    pub type wlr_pointer_constraints_v1;
    pub type wlr_drm_lease_v1_manager;
    pub type wlr_server_decoration_manager;
    pub type wlr_relative_pointer_manager_v1;
    pub type wlr_xcursor_manager;
    pub type wlr_xwayland_shell_v1;
    pub type wlr_xwm;
    pub type wlr_tablet_manager_v2;
    pub type wlr_idle_notifier_v1;
    pub type wlr_pointer_gestures_v1;
    pub type wlr_data_device_manager;
    pub type wlr_session;
    pub type sway_transaction_instruction;
    pub type wlr_ext_foreign_toplevel_handle_v1;
    pub type launcher_ctx;
    pub type wlr_ext_image_capture_source_v1;
    pub type wlr_tablet_v2_tablet_tool;
    pub type wlr_tablet_v2_tablet;
    pub type wlr_tablet_v2_tablet_pad;
    pub type wlr_tablet_pad;
    pub type sway_switch;
    pub type sway_keyboard;
    pub type wlr_touch_impl;
    pub type sway_cursor;
    pub type _PangoFontDescription;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn fmax(
        __x: ::core::ffi::c_double,
        __y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn fmin(
        __x: ::core::ffi::c_double,
        __y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn wl_list_init(list: *mut wl_list);
    fn wl_signal_emit_mutable(signal: *mut wl_signal, data: *mut ::core::ffi::c_void);
    fn pixman_region32_init(region: *mut pixman_region32_t);
    fn pixman_region32_init_rect(
        region: *mut pixman_region32_t,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
    );
    fn pixman_region32_fini(region: *mut pixman_region32_t);
    fn pixman_region32_union_rect(
        dest: *mut pixman_region32_t,
        source: *const pixman_region32_t,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
    ) -> pixman_bool_t;
    fn pixman_region32_subtract(
        reg_d: *mut pixman_region32_t,
        reg_m: *const pixman_region32_t,
        reg_s: *const pixman_region32_t,
    ) -> pixman_bool_t;
    fn pixman_region32_rectangles(
        region: *const pixman_region32_t,
        n_rects: *mut ::core::ffi::c_int,
    ) -> *mut pixman_box32_t;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn lenient_strcat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn lenient_strncat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        len: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn has_prefix(
        str: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    ) -> bool;
    fn create_list() -> *mut list_t;
    fn list_free(list: *mut list_t);
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn list_insert(
        list: *mut list_t,
        index: ::core::ffi::c_int,
        item: *mut ::core::ffi::c_void,
    );
    fn list_del(list: *mut list_t, index: ::core::ffi::c_int);
    fn list_find(
        list: *mut list_t,
        item: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn list_move_to_end(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn list_free_items_and_destroy(list: *mut list_t);
    fn wlr_output_layout_get_box(
        layout: *mut wlr_output_layout,
        reference: *mut wlr_output,
        dest_box: *mut wlr_box,
    );
    fn wlr_scene_node_destroy(node: *mut wlr_scene_node);
    fn wlr_scene_node_set_enabled(node: *mut wlr_scene_node, enabled: bool);
    fn wlr_scene_node_set_position(
        node: *mut wlr_scene_node,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn wlr_scene_node_raise_to_top(node: *mut wlr_scene_node);
    fn wlr_scene_rect_from_node(node: *mut wlr_scene_node) -> *mut wlr_scene_rect;
    fn wlr_scene_rect_create(
        parent: *mut wlr_scene_tree,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        color: *const ::core::ffi::c_float,
    ) -> *mut wlr_scene_rect;
    fn wlr_scene_rect_set_size(
        rect: *mut wlr_scene_rect,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn wlr_scene_rect_set_color(
        rect: *mut wlr_scene_rect,
        color: *const ::core::ffi::c_float,
    );
    fn workspace_consider_destroy(ws: *mut sway_workspace);
    fn workspace_is_visible(ws: *mut sway_workspace) -> bool;
    fn workspace_detect_urgent(workspace: *mut sway_workspace);
    fn workspace_add_tiling(
        workspace: *mut sway_workspace,
        con: *mut sway_container,
    ) -> *mut sway_container;
    fn workspace_add_floating(workspace: *mut sway_workspace, con: *mut sway_container);
    fn workspace_insert_tiling_direct(
        workspace: *mut sway_workspace,
        con: *mut sway_container,
        index: ::core::ffi::c_int,
    );
    fn workspace_insert_tiling(
        workspace: *mut sway_workspace,
        con: *mut sway_container,
        index: ::core::ffi::c_int,
    ) -> *mut sway_container;
    fn workspace_update_representation(ws: *mut sway_workspace);
    fn workspace_get_box(workspace: *mut sway_workspace, r#box: *mut wlr_box);
    fn output_get_active_workspace(output: *mut sway_output) -> *mut sway_workspace;
    fn output_get_box(output: *mut sway_output, r#box: *mut wlr_box);
    fn node_init(
        node: *mut sway_node,
        r#type: sway_node_type,
        thing: *mut ::core::ffi::c_void,
    );
    fn node_set_dirty(node: *mut sway_node);
    fn scene_node_disown_children(tree: *mut wlr_scene_tree);
    fn alloc_scene_tree(
        parent: *mut wlr_scene_tree,
        failed: *mut bool,
    ) -> *mut wlr_scene_tree;
    fn wlr_box_closest_point(
        r#box: *const wlr_box,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        dest_x: *mut ::core::ffi::c_double,
        dest_y: *mut ::core::ffi::c_double,
    );
    fn wlr_box_empty(r#box: *const wlr_box) -> bool;
    fn wlr_foreign_toplevel_handle_v1_set_fullscreen(
        toplevel: *mut wlr_foreign_toplevel_handle_v1,
        fullscreen: bool,
    );
    fn view_get_title(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_app_id(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_class(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_instance(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_sandbox_engine(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_sandbox_app_id(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_sandbox_instance_id(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_get_shell(view: *mut sway_view) -> *const ::core::ffi::c_char;
    fn view_set_tiled(view: *mut sway_view, tiled: bool);
    fn view_destroy(view: *mut sway_view);
    fn view_is_urgent(view: *mut sway_view) -> bool;
    fn view_is_transient_for(child: *mut sway_view, ancestor: *mut sway_view) -> bool;
    static mut config: *mut sway_config;
    fn ipc_event_window(window: *mut sway_container, change: *const ::core::ffi::c_char);
    fn scene_descriptor_assign(
        node: *mut wlr_scene_node,
        r#type: sway_scene_descriptor_type,
        data: *mut ::core::ffi::c_void,
    ) -> bool;
    fn arrange_workspace(workspace: *mut sway_workspace);
    fn arrange_root();
    fn escape_markup_text(
        src: *const ::core::ffi::c_char,
        dest: *mut ::core::ffi::c_char,
    ) -> size_t;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    fn seat_set_focus(seat: *mut sway_seat, node: *mut sway_node);
    fn seat_set_focus_container(seat: *mut sway_seat, con: *mut sway_container);
    fn seat_set_raw_focus(seat: *mut sway_seat, node: *mut sway_node);
    fn seat_get_focus(seat: *mut sway_seat) -> *mut sway_node;
    fn seat_get_focused_workspace(seat: *mut sway_seat) -> *mut sway_workspace;
    fn seat_get_focused_container(seat: *mut sway_seat) -> *mut sway_container;
    fn seat_get_focus_inactive(
        seat: *mut sway_seat,
        node: *mut sway_node,
    ) -> *mut sway_node;
    fn seat_get_focus_inactive_tiling(
        seat: *mut sway_seat,
        workspace: *mut sway_workspace,
    ) -> *mut sway_container;
    fn seatop_unref(seat: *mut sway_seat, con: *mut sway_container);
    fn wlr_xdg_toplevel_decoration_v1_set_mode(
        decoration: *mut wlr_xdg_toplevel_decoration_v1,
        mode: wlr_xdg_toplevel_decoration_v1_mode,
    ) -> uint32_t;
    static mut server: sway_server;
    fn input_manager_get_default_seat() -> *mut sway_seat;
    fn input_manager_current_seat() -> *mut sway_seat;
    static mut root: *mut sway_root;
    fn root_scratchpad_add_container(con: *mut sway_container, ws: *mut sway_workspace);
    fn root_scratchpad_remove_container(con: *mut sway_container);
    fn root_scratchpad_show(con: *mut sway_container);
    fn root_find_container(
        test: Option<
            unsafe extern "C" fn(*mut sway_container, *mut ::core::ffi::c_void) -> bool,
        >,
        data: *mut ::core::ffi::c_void,
    ) -> *mut sway_container;
    fn sway_text_node_create(
        parent: *mut wlr_scene_tree,
        text: *mut ::core::ffi::c_char,
        color: *mut ::core::ffi::c_float,
        pango_markup: bool,
    ) -> *mut sway_text_node;
    fn sway_text_node_set_color(
        node: *mut sway_text_node,
        color: *mut ::core::ffi::c_float,
    );
    fn sway_text_node_set_text(
        node: *mut sway_text_node,
        text: *mut ::core::ffi::c_char,
    );
    fn sway_text_node_set_max_width(
        node: *mut sway_text_node,
        max_width: ::core::ffi::c_int,
    );
    fn sway_text_node_set_background(
        node: *mut sway_text_node,
        background: *mut ::core::ffi::c_float,
    );
}
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
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
pub struct wlr_foreign_toplevel_manager_v1 {
    pub event_loop: *mut wl_event_loop,
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub toplevels: wl_list,
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
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_foreign_toplevel_handle_v1 {
    pub manager: *mut wlr_foreign_toplevel_manager_v1,
    pub resources: wl_list,
    pub link: wl_list,
    pub idle_source: *mut wl_event_source,
    pub title: *mut ::core::ffi::c_char,
    pub app_id: *mut ::core::ffi::c_char,
    pub parent: *mut wlr_foreign_toplevel_handle_v1,
    pub outputs: wl_list,
    pub state: uint32_t,
    pub events: C2Rust_Unnamed_16,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub request_maximize: wl_signal,
    pub request_minimize: wl_signal,
    pub request_activate: wl_signal,
    pub request_fullscreen: wl_signal,
    pub request_close: wl_signal,
    pub set_rectangle: wl_signal,
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
pub struct wlr_seat_client {
    pub client: *mut wl_client,
    pub seat: *mut wlr_seat,
    pub link: wl_list,
    pub resources: wl_list,
    pub pointers: wl_list,
    pub keyboards: wl_list,
    pub touches: wl_list,
    pub data_devices: wl_list,
    pub events: C2Rust_Unnamed_22,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
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
pub struct C2Rust_Unnamed_22 {
    pub destroy: wl_signal,
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
    pub events: C2Rust_Unnamed_25,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_24,
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
pub struct C2Rust_Unnamed_24 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
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
    pub viewport: C2Rust_Unnamed_26,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
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
    pub events: C2Rust_Unnamed_28,
    pub WLR_PRIVATE: C2Rust_Unnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub new_surface: wl_signal,
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
    pub events: C2Rust_Unnamed_30,
    pub WLR_PRIVATE: C2Rust_Unnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
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
    pub repeat_info: C2Rust_Unnamed_32,
    pub events: C2Rust_Unnamed_31,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
    pub rate: int32_t,
    pub delay: int32_t,
}
pub type xkb_mod_index_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_device {
    pub r#type: wlr_input_device_type,
    pub name: *mut ::core::ffi::c_char,
    pub events: C2Rust_Unnamed_33,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
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
    pub events: C2Rust_Unnamed_35,
    pub WLR_PRIVATE: C2Rust_Unnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
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
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_37,
    pub WLR_PRIVATE: C2Rust_Unnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
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
pub struct C2Rust_Unnamed_37 {
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
    pub events: C2Rust_Unnamed_39,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
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
pub struct wlr_damage_ring {
    pub current: pixman_region32_t,
    pub WLR_PRIVATE: C2Rust_Unnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_40 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_surface {
    pub client: *mut wlr_xdg_client,
    pub resource: *mut wl_resource,
    pub surface: *mut wlr_surface,
    pub link: wl_list,
    pub role: wlr_xdg_surface_role,
    pub role_resource: *mut wl_resource,
    pub c2rust_unnamed: C2Rust_Unnamed_43,
    pub popups: wl_list,
    pub configured: bool,
    pub configure_idle: *mut wl_event_source,
    pub scheduled_serial: uint32_t,
    pub configure_list: wl_list,
    pub current: wlr_xdg_surface_state,
    pub pending: wlr_xdg_surface_state,
    pub initialized: bool,
    pub initial_commit: bool,
    pub geometry: wlr_box,
    pub events: C2Rust_Unnamed_42,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_41 {
    pub synced: wlr_surface_synced,
    pub role_resource_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
    pub destroy: wl_signal,
    pub ping_timeout: wl_signal,
    pub new_popup: wl_signal,
    pub configure: wl_signal,
    pub ack_configure: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_surface_state {
    pub committed: uint32_t,
    pub geometry: wlr_box,
    pub configure_serial: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_43 {
    pub toplevel: *mut wlr_xdg_toplevel,
    pub popup: *mut wlr_xdg_popup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_popup {
    pub base: *mut wlr_xdg_surface,
    pub link: wl_list,
    pub resource: *mut wl_resource,
    pub parent: *mut wlr_surface,
    pub seat: *mut wlr_seat,
    pub scheduled: wlr_xdg_popup_configure,
    pub current: wlr_xdg_popup_state,
    pub pending: wlr_xdg_popup_state,
    pub events: C2Rust_Unnamed_45,
    pub grab_link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_44,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_44 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_45 {
    pub destroy: wl_signal,
    pub reposition: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_popup_state {
    pub geometry: wlr_box,
    pub reactive: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_popup_configure {
    pub fields: uint32_t,
    pub geometry: wlr_box,
    pub rules: wlr_xdg_positioner_rules,
    pub reposition_token: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_positioner_rules {
    pub anchor_rect: wlr_box,
    pub anchor: xdg_positioner_anchor,
    pub gravity: xdg_positioner_gravity,
    pub constraint_adjustment: xdg_positioner_constraint_adjustment,
    pub reactive: bool,
    pub has_parent_configure_serial: bool,
    pub parent_configure_serial: uint32_t,
    pub size: C2Rust_Unnamed_47,
    pub parent_size: C2Rust_Unnamed_47,
    pub offset: C2Rust_Unnamed_46,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
    pub x: int32_t,
    pub y: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
    pub width: int32_t,
    pub height: int32_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xdg_positioner_constraint_adjustment(pub ::core::ffi::c_uint);
impl xdg_positioner_constraint_adjustment {
    pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_NONE: Self = Self(0);
    pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_SLIDE_X: Self = Self(1);
    pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_SLIDE_Y: Self = Self(2);
    pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_FLIP_X: Self = Self(4);
    pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_FLIP_Y: Self = Self(8);
    pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_RESIZE_X: Self = Self(16);
    pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_RESIZE_Y: Self = Self(32);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xdg_positioner_gravity(pub ::core::ffi::c_uint);
impl xdg_positioner_gravity {
    pub const XDG_POSITIONER_GRAVITY_NONE: Self = Self(0);
    pub const XDG_POSITIONER_GRAVITY_TOP: Self = Self(1);
    pub const XDG_POSITIONER_GRAVITY_BOTTOM: Self = Self(2);
    pub const XDG_POSITIONER_GRAVITY_LEFT: Self = Self(3);
    pub const XDG_POSITIONER_GRAVITY_RIGHT: Self = Self(4);
    pub const XDG_POSITIONER_GRAVITY_TOP_LEFT: Self = Self(5);
    pub const XDG_POSITIONER_GRAVITY_BOTTOM_LEFT: Self = Self(6);
    pub const XDG_POSITIONER_GRAVITY_TOP_RIGHT: Self = Self(7);
    pub const XDG_POSITIONER_GRAVITY_BOTTOM_RIGHT: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xdg_positioner_anchor(pub ::core::ffi::c_uint);
impl xdg_positioner_anchor {
    pub const XDG_POSITIONER_ANCHOR_NONE: Self = Self(0);
    pub const XDG_POSITIONER_ANCHOR_TOP: Self = Self(1);
    pub const XDG_POSITIONER_ANCHOR_BOTTOM: Self = Self(2);
    pub const XDG_POSITIONER_ANCHOR_LEFT: Self = Self(3);
    pub const XDG_POSITIONER_ANCHOR_RIGHT: Self = Self(4);
    pub const XDG_POSITIONER_ANCHOR_TOP_LEFT: Self = Self(5);
    pub const XDG_POSITIONER_ANCHOR_BOTTOM_LEFT: Self = Self(6);
    pub const XDG_POSITIONER_ANCHOR_TOP_RIGHT: Self = Self(7);
    pub const XDG_POSITIONER_ANCHOR_BOTTOM_RIGHT: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_toplevel {
    pub resource: *mut wl_resource,
    pub base: *mut wlr_xdg_surface,
    pub parent: *mut wlr_xdg_toplevel,
    pub current: wlr_xdg_toplevel_state,
    pub pending: wlr_xdg_toplevel_state,
    pub scheduled: wlr_xdg_toplevel_configure,
    pub requested: wlr_xdg_toplevel_requested,
    pub title: *mut ::core::ffi::c_char,
    pub app_id: *mut ::core::ffi::c_char,
    pub events: C2Rust_Unnamed_49,
    pub WLR_PRIVATE: C2Rust_Unnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
    pub synced: wlr_surface_synced,
    pub parent_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
    pub destroy: wl_signal,
    pub request_maximize: wl_signal,
    pub request_fullscreen: wl_signal,
    pub request_minimize: wl_signal,
    pub request_move: wl_signal,
    pub request_resize: wl_signal,
    pub request_show_window_menu: wl_signal,
    pub set_parent: wl_signal,
    pub set_title: wl_signal,
    pub set_app_id: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_toplevel_requested {
    pub maximized: bool,
    pub minimized: bool,
    pub fullscreen: bool,
    pub fullscreen_output: *mut wlr_output,
    pub WLR_PRIVATE: C2Rust_Unnamed_50,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
    pub fullscreen_output_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_toplevel_configure {
    pub fields: uint32_t,
    pub maximized: bool,
    pub fullscreen: bool,
    pub resizing: bool,
    pub activated: bool,
    pub suspended: bool,
    pub tiled: uint32_t,
    pub constrained: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub bounds: C2Rust_Unnamed_51,
    pub wm_capabilities: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
    pub width: int32_t,
    pub height: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_toplevel_state {
    pub maximized: bool,
    pub fullscreen: bool,
    pub resizing: bool,
    pub activated: bool,
    pub suspended: bool,
    pub tiled: uint32_t,
    pub constrained: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub max_width: int32_t,
    pub max_height: int32_t,
    pub min_width: int32_t,
    pub min_height: int32_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_xdg_surface_role(pub ::core::ffi::c_uint);
impl wlr_xdg_surface_role {
    pub const WLR_XDG_SURFACE_ROLE_NONE: Self = Self(0);
    pub const WLR_XDG_SURFACE_ROLE_TOPLEVEL: Self = Self(1);
    pub const WLR_XDG_SURFACE_ROLE_POPUP: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_client {
    pub shell: *mut wlr_xdg_shell,
    pub resource: *mut wl_resource,
    pub client: *mut wl_client,
    pub surfaces: wl_list,
    pub link: wl_list,
    pub ping_serial: uint32_t,
    pub ping_timer: *mut wl_event_source,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_shell {
    pub global: *mut wl_global,
    pub version: uint32_t,
    pub clients: wl_list,
    pub popup_grabs: wl_list,
    pub ping_timeout: uint32_t,
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
    pub new_toplevel: wl_signal,
    pub new_popup: wl_signal,
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
    pub events: C2Rust_Unnamed_55,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_54,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_54 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_56,
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
pub struct C2Rust_Unnamed_56 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_58,
    pub data: *mut ::core::ffi::c_void,
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
    pub events: C2Rust_Unnamed_60,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_59,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_59 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_60 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_61,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_61 {
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
    pub events: C2Rust_Unnamed_66,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_62,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
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
    pub events: C2Rust_Unnamed_64,
    pub WLR_PRIVATE: C2Rust_Unnamed_63,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
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
pub struct C2Rust_Unnamed_64 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_65,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_65 {
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
pub struct C2Rust_Unnamed_66 {
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
    pub title_bar: C2Rust_Unnamed_104,
    pub border: C2Rust_Unnamed_103,
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
    pub layers: C2Rust_Unnamed_102,
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
pub struct wlr_xdg_decoration_manager_v1 {
    pub global: *mut wl_global,
    pub decorations: wl_list,
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
    pub new_toplevel_decoration: wl_signal,
    pub destroy: wl_signal,
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
pub struct C2Rust_Unnamed_102 {
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
pub struct C2Rust_Unnamed_103 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_104 {
    pub tree: *mut wlr_scene_tree,
    pub border: *mut wlr_scene_tree,
    pub background: *mut wlr_scene_tree,
    pub title_text: *mut sway_text_node,
    pub marks_text: *mut sway_text_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_text_node {
    pub width: ::core::ffi::c_int,
    pub max_width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub baseline: ::core::ffi::c_int,
    pub pango_markup: bool,
    pub color: [::core::ffi::c_float; 4],
    pub background: [::core::ffi::c_float; 4],
    pub node: *mut wlr_scene_node,
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
    pub c2rust_unnamed: C2Rust_Unnamed_106,
    pub events: C2Rust_Unnamed_105,
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
pub struct C2Rust_Unnamed_105 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_106 {
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
    pub events: C2Rust_Unnamed_108,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_107,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_107 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_108 {
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
pub struct sway_xdg_decoration {
    pub wlr_xdg_decoration: *mut wlr_xdg_toplevel_decoration_v1,
    pub link: wl_list,
    pub view: *mut sway_view,
    pub destroy: wl_listener,
    pub request_mode: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_toplevel_decoration_v1 {
    pub resource: *mut wl_resource,
    pub toplevel: *mut wlr_xdg_toplevel,
    pub manager: *mut wlr_xdg_decoration_manager_v1,
    pub link: wl_list,
    pub current: wlr_xdg_toplevel_decoration_v1_state,
    pub pending: wlr_xdg_toplevel_decoration_v1_state,
    pub scheduled_mode: wlr_xdg_toplevel_decoration_v1_mode,
    pub requested_mode: wlr_xdg_toplevel_decoration_v1_mode,
    pub configure_list: wl_list,
    pub events: C2Rust_Unnamed_110,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_109,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_109 {
    pub toplevel_destroy: wl_listener,
    pub surface_configure: wl_listener,
    pub surface_ack_configure: wl_listener,
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_110 {
    pub destroy: wl_signal,
    pub request_mode: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_xdg_toplevel_decoration_v1_mode(pub ::core::ffi::c_uint);
impl wlr_xdg_toplevel_decoration_v1_mode {
    pub const WLR_XDG_TOPLEVEL_DECORATION_V1_MODE_NONE: Self = Self(0);
    pub const WLR_XDG_TOPLEVEL_DECORATION_V1_MODE_CLIENT_SIDE: Self = Self(1);
    pub const WLR_XDG_TOPLEVEL_DECORATION_V1_MODE_SERVER_SIDE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_toplevel_decoration_v1_state {
    pub mode: wlr_xdg_toplevel_decoration_v1_mode,
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
    pub events: C2Rust_Unnamed_116,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_116 {
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
    pub events: C2Rust_Unnamed_117,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_117 {
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
pub struct border_colors {
    pub border: [::core::ffi::c_float; 4],
    pub background: [::core::ffi::c_float; 4],
    pub text: [::core::ffi::c_float; 4],
    pub indicator: [::core::ffi::c_float; 4],
    pub child_border: [::core::ffi::c_float; 4],
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
    pub handler_context: C2Rust_Unnamed_119,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_119 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_120,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_120 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
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
    pub xcursor_theme: C2Rust_Unnamed_121,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_121 {
    pub name: *mut ::core::ffi::c_char,
    pub size: ::core::ffi::c_int,
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
pub struct seat_config_allow_constrain(pub ::core::ffi::c_uint);
impl seat_config_allow_constrain {
    pub const CONSTRAIN_DEFAULT: Self = Self(0);
    pub const CONSTRAIN_ENABLE: Self = Self(1);
    pub const CONSTRAIN_DISABLE: Self = Self(2);
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
pub struct color_profile(pub ::core::ffi::c_uint);
impl color_profile {
    pub const COLOR_PROFILE_DEFAULT: Self = Self(0);
    pub const COLOR_PROFILE_TRANSFORM: Self = Self(1);
    pub const COLOR_PROFILE_TRANSFORM_WITH_DEVICE_PRIMARIES: Self = Self(2);
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
pub struct input_config_mapped_to(pub ::core::ffi::c_uint);
impl input_config_mapped_to {
    pub const MAPPED_TO_DEFAULT: Self = Self(0);
    pub const MAPPED_TO_OUTPUT: Self = Self(1);
    pub const MAPPED_TO_REGION: Self = Self(2);
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
pub struct edge_border_smart_types(pub ::core::ffi::c_uint);
impl edge_border_smart_types {
    pub const ESMART_OFF: Self = Self(0);
    pub const ESMART_ON: Self = Self(1);
    pub const ESMART_NO_GAPS: Self = Self(2);
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
pub struct smart_gaps_mode(pub ::core::ffi::c_uint);
impl smart_gaps_mode {
    pub const SMART_GAPS_OFF: Self = Self(0);
    pub const SMART_GAPS_ON: Self = Self(1);
    pub const SMART_GAPS_INVERSE_OUTER: Self = Self(2);
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
pub struct focus_follows_mouse_mode(pub ::core::ffi::c_uint);
impl focus_follows_mouse_mode {
    pub const FOLLOWS_NO: Self = Self(0);
    pub const FOLLOWS_YES: Self = Self(1);
    pub const FOLLOWS_ALWAYS: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xwayland_mode(pub ::core::ffi::c_uint);
impl xwayland_mode {
    pub const XWAYLAND_MODE_DISABLED: Self = Self(0);
    pub const XWAYLAND_MODE_LAZY: Self = Self(1);
    pub const XWAYLAND_MODE_IMMEDIATE: Self = Self(2);
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
pub struct sway_fowa(pub ::core::ffi::c_uint);
impl sway_fowa {
    pub const FOWA_SMART: Self = Self(0);
    pub const FOWA_URGENT: Self = Self(1);
    pub const FOWA_FOCUS: Self = Self(2);
    pub const FOWA_NONE: Self = Self(3);
}
pub type PangoFontDescription = _PangoFontDescription;
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
    pub colors: C2Rust_Unnamed_122,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_122 {
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
pub struct sway_mode {
    pub name: *mut ::core::ffi::c_char,
    pub keysym_bindings: *mut list_t,
    pub keycode_bindings: *mut list_t,
    pub mouse_bindings: *mut list_t,
    pub switch_bindings: *mut list_t,
    pub gesture_bindings: *mut list_t,
    pub pango: bool,
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
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const DBL_MAX: ::core::ffi::c_double = __DBL_MAX__;
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&raw mut (*signal).listener_list);
}
unsafe extern "C" fn alloc_rect_node(
    mut parent: *mut wlr_scene_tree,
    mut failed: *mut bool,
) -> *mut wlr_scene_rect {
    if *failed {
        return ::core::ptr::null_mut::<wlr_scene_rect>();
    }
    let mut c2rust_lvalue: [::core::ffi::c_float; 4] = [0.0f32, 0.0f32, 0.0f32, 1.0f32];
    let mut rect: *mut wlr_scene_rect = wlr_scene_rect_create(
        parent,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        &raw mut c2rust_lvalue as *mut ::core::ffi::c_float,
    );
    if rect.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to allocate a wlr_scene_rect\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
            39 as ::core::ffi::c_int,
        );
        *failed = r#true != 0;
    }
    return rect;
}
#[no_mangle]
pub unsafe extern "C" fn container_create(
    mut view: *mut sway_view,
) -> *mut sway_container {
    let mut c: *mut sway_container = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_container>(),
    ) as *mut sway_container;
    if c.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to allocate sway_container\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
            49 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<sway_container>();
    }
    node_init(
        &raw mut (*c).node,
        sway_node_type::N_CONTAINER,
        c as *mut ::core::ffi::c_void,
    );
    let mut failed: bool = r#false != 0;
    (*c).scene_tree = alloc_scene_tree((*root).staging, &raw mut failed);
    (*c).title_bar.tree = alloc_scene_tree((*c).scene_tree, &raw mut failed);
    (*c).title_bar.border = alloc_scene_tree((*c).title_bar.tree, &raw mut failed);
    (*c).title_bar.background = alloc_scene_tree((*c).title_bar.tree, &raw mut failed);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        alloc_rect_node((*c).title_bar.border, &raw mut failed);
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 5 as ::core::ffi::c_int {
        alloc_rect_node((*c).title_bar.background, &raw mut failed);
        i_0 += 1;
    }
    (*c).border.tree = alloc_scene_tree((*c).scene_tree, &raw mut failed);
    (*c).content_tree = alloc_scene_tree((*c).border.tree, &raw mut failed);
    if !view.is_null() {
        (*c).border.top = alloc_rect_node((*c).border.tree, &raw mut failed);
        (*c).border.bottom = alloc_rect_node((*c).border.tree, &raw mut failed);
        (*c).border.left = alloc_rect_node((*c).border.tree, &raw mut failed);
        (*c).border.right = alloc_rect_node((*c).border.tree, &raw mut failed);
    }
    if !failed
        && !scene_descriptor_assign(
            &raw mut (*(*c).scene_tree).node,
            sway_scene_descriptor_type::SWAY_SCENE_DESC_CONTAINER,
            c as *mut ::core::ffi::c_void,
        )
    {
        failed = r#true != 0;
    }
    if failed {
        wlr_scene_node_destroy(&raw mut (*(*c).scene_tree).node);
        free(c as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<sway_container>();
    }
    if view.is_null() {
        (*c).pending.children = create_list();
        (*c).current.children = create_list();
    }
    (*c).pending.layout = sway_container_layout(
        sway_container_layout_0::L_NONE.0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*c).view = view;
    (*c).alpha = 1.0f32;
    (*c).marks = create_list();
    wl_signal_init(&raw mut (*c).events.destroy);
    wl_signal_emit_mutable(
        &raw mut (*root).events.new_node,
        &raw mut (*c).node as *mut ::core::ffi::c_void,
    );
    container_update(c);
    return c;
}
unsafe extern "C" fn container_is_focused(
    mut con: *mut sway_container,
    mut data: *mut ::core::ffi::c_void,
) -> bool {
    return (*con).current.focused;
}
unsafe extern "C" fn container_has_focused_child(mut con: *mut sway_container) -> bool {
    return !container_find_child(
            con,
            Some(
                container_is_focused
                    as unsafe extern "C" fn(
                        *mut sway_container,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
            ),
            NULL,
        )
        .is_null();
}
unsafe extern "C" fn container_is_current_parent_focused(
    mut con: *mut sway_container,
) -> bool {
    if !(*con).current.parent.is_null() {
        let mut parent: *mut sway_container = (*con).current.parent;
        return (*parent).current.focused as ::core::ffi::c_int != 0
            || container_is_current_parent_focused(parent) as ::core::ffi::c_int != 0;
    } else if !(*con).current.workspace.is_null() {
        let mut ws: *mut sway_workspace = (*con).current.workspace;
        return (*ws).current.focused;
    }
    return r#false != 0;
}
unsafe extern "C" fn container_get_current_colors(
    mut con: *mut sway_container,
) -> *mut border_colors {
    let mut colors: *mut border_colors = ::core::ptr::null_mut::<border_colors>();
    let mut urgent: bool = if !(*con).view.is_null() {
        view_is_urgent((*con).view) as ::core::ffi::c_int
    } else {
        container_has_urgent_child(con) as ::core::ffi::c_int
    } != 0;
    let mut active_child: *mut sway_container = ::core::ptr::null_mut::<
        sway_container,
    >();
    if !(*con).current.parent.is_null() {
        active_child = (*(*con).current.parent).current.focused_inactive_child;
    } else if !(*con).current.workspace.is_null() {
        active_child = (*(*con).current.workspace).current.focused_inactive_child;
    } else {
        active_child = ::core::ptr::null_mut::<sway_container>();
    }
    if urgent {
        colors = &raw mut (*config).border_colors.urgent;
    } else if (*con).current.focused as ::core::ffi::c_int != 0
        || container_is_current_parent_focused(con) as ::core::ffi::c_int != 0
    {
        colors = &raw mut (*config).border_colors.focused;
    } else if (*config).has_focused_tab_title as ::core::ffi::c_int != 0
        && container_has_focused_child(con) as ::core::ffi::c_int != 0
    {
        colors = &raw mut (*config).border_colors.focused_tab_title;
    } else if con == active_child {
        colors = &raw mut (*config).border_colors.focused_inactive;
    } else {
        colors = &raw mut (*config).border_colors.unfocused;
    }
    return colors;
}
unsafe extern "C" fn container_is_current_floating(
    mut container: *mut sway_container,
) -> bool {
    if (*container).current.parent.is_null() && !(*container).current.workspace.is_null()
        && list_find(
            (*(*container).current.workspace).floating,
            container as *const ::core::ffi::c_void,
        ) != -1 as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    if (*container).scratchpad {
        return r#true != 0;
    }
    return r#false != 0;
}
unsafe extern "C" fn scene_rect_set_color(
    mut rect: *mut wlr_scene_rect,
    mut color: *const ::core::ffi::c_float,
    mut opacity: ::core::ffi::c_float,
) {
    let premultiplied: [::core::ffi::c_float; 4] = [
        *color.offset(0isize) * *color.offset(3isize) * opacity,
        *color.offset(1isize) * *color.offset(3isize) * opacity,
        *color.offset(2isize) * *color.offset(3isize) * opacity,
        *color.offset(3isize) * opacity,
    ];
    wlr_scene_rect_set_color(
        rect,
        &raw const premultiplied as *const ::core::ffi::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn container_update(mut con: *mut sway_container) {
    let mut colors: *mut border_colors = container_get_current_colors(con);
    let mut siblings: *mut list_t = ::core::ptr::null_mut::<list_t>();
    let mut layout: sway_container_layout = sway_container_layout(
        sway_container_layout_0::L_NONE.0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    let mut alpha: ::core::ffi::c_float = (*con).alpha;
    if !(*con).current.parent.is_null() {
        siblings = (*(*con).current.parent).current.children;
        layout = (*(*con).current.parent).current.layout;
    } else if !(*con).current.workspace.is_null() {
        siblings = (*(*con).current.workspace).current.tiling;
        layout = (*(*con).current.workspace).current.layout;
    }
    let mut bottom: [::core::ffi::c_float; 4] = [0.; 4];
    let mut right: [::core::ffi::c_float; 4] = [0.; 4];
    memcpy(
        &raw mut bottom as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        &raw mut (*colors).child_border as *mut ::core::ffi::c_float
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_float; 4]>(),
    );
    memcpy(
        &raw mut right as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        &raw mut (*colors).child_border as *mut ::core::ffi::c_float
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_float; 4]>(),
    );
    if !container_is_current_floating(con) && !siblings.is_null()
        && (*siblings).length == 1 as ::core::ffi::c_int
    {
        if layout.0 == sway_container_layout_0::L_HORIZ.0 {
            memcpy(
                &raw mut right as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
                &raw mut (*colors).indicator as *mut ::core::ffi::c_float
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_float; 4]>(),
            );
        } else if layout.0 == sway_container_layout_0::L_VERT.0 {
            memcpy(
                &raw mut bottom as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
                &raw mut (*colors).indicator as *mut ::core::ffi::c_float
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_float; 4]>(),
            );
        }
    }
    let mut node: *mut wlr_scene_node = ::core::ptr::null_mut::<wlr_scene_node>();
    node = ((*(*con).title_bar.border).children.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut wlr_scene_node
        as *mut wlr_scene_node;
    while &raw mut (*node).link != &raw mut (*(*con).title_bar.border).children {
        let mut rect: *mut wlr_scene_rect = wlr_scene_rect_from_node(node);
        scene_rect_set_color(
            rect,
            &raw mut (*colors).border as *mut ::core::ffi::c_float,
            alpha,
        );
        node = ((*node).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut wlr_scene_node
            as *mut wlr_scene_node;
    }
    node = ((*(*con).title_bar.background).children.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut wlr_scene_node
        as *mut wlr_scene_node;
    while &raw mut (*node).link != &raw mut (*(*con).title_bar.background).children {
        let mut rect_0: *mut wlr_scene_rect = wlr_scene_rect_from_node(node);
        scene_rect_set_color(
            rect_0,
            &raw mut (*colors).background as *mut ::core::ffi::c_float,
            alpha,
        );
        node = ((*node).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut wlr_scene_node
            as *mut wlr_scene_node;
    }
    if !(*con).view.is_null() {
        scene_rect_set_color(
            (*con).border.top,
            &raw mut (*colors).child_border as *mut ::core::ffi::c_float,
            alpha,
        );
        scene_rect_set_color(
            (*con).border.bottom,
            &raw mut bottom as *mut ::core::ffi::c_float,
            alpha,
        );
        scene_rect_set_color(
            (*con).border.left,
            &raw mut (*colors).child_border as *mut ::core::ffi::c_float,
            alpha,
        );
        scene_rect_set_color(
            (*con).border.right,
            &raw mut right as *mut ::core::ffi::c_float,
            alpha,
        );
    }
    if !(*con).title_bar.title_text.is_null() {
        sway_text_node_set_color(
            (*con).title_bar.title_text,
            &raw mut (*colors).text as *mut ::core::ffi::c_float,
        );
        sway_text_node_set_background(
            (*con).title_bar.title_text,
            &raw mut (*colors).background as *mut ::core::ffi::c_float,
        );
    }
    if !(*con).title_bar.marks_text.is_null() {
        sway_text_node_set_color(
            (*con).title_bar.marks_text,
            &raw mut (*colors).text as *mut ::core::ffi::c_float,
        );
        sway_text_node_set_background(
            (*con).title_bar.marks_text,
            &raw mut (*colors).background as *mut ::core::ffi::c_float,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn container_update_itself_and_parents(
    mut con: *mut sway_container,
) {
    container_update(con);
    if !(*con).current.parent.is_null() {
        container_update_itself_and_parents((*con).current.parent);
    }
}
unsafe extern "C" fn update_rect_list(
    mut tree: *mut wlr_scene_tree,
    mut region: *mut pixman_region32_t,
) {
    let mut len: ::core::ffi::c_int = 0;
    let mut rects: *const pixman_box32_t = pixman_region32_rectangles(
        region,
        &raw mut len,
    );
    wlr_scene_node_set_enabled(&raw mut (*tree).node, len > 0 as ::core::ffi::c_int);
    if len == 0 as ::core::ffi::c_int {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut node: *mut wlr_scene_node = ::core::ptr::null_mut::<wlr_scene_node>();
    node = ((*tree).children.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut wlr_scene_node
        as *mut wlr_scene_node;
    while &raw mut (*node).link != &raw mut (*tree).children {
        let mut rect: *mut wlr_scene_rect = wlr_scene_rect_from_node(node);
        wlr_scene_node_set_enabled(&raw mut (*rect).node, i < len);
        if i < len {
            let c2rust_fresh0 = i;
            i += 1;
            let mut r#box: *const pixman_box32_t = rects.offset(c2rust_fresh0 as isize);
            wlr_scene_node_set_position(
                &raw mut (*rect).node,
                (*r#box).x1 as ::core::ffi::c_int,
                (*r#box).y1 as ::core::ffi::c_int,
            );
            wlr_scene_rect_set_size(
                rect,
                (*r#box).x2 as ::core::ffi::c_int - (*r#box).x1 as ::core::ffi::c_int,
                (*r#box).y2 as ::core::ffi::c_int - (*r#box).y1 as ::core::ffi::c_int,
            );
        }
        node = ((*node).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut wlr_scene_node
            as *mut wlr_scene_node;
    }
}
#[no_mangle]
pub unsafe extern "C" fn container_arrange_title_bar(mut con: *mut sway_container) {
    let mut title_align: alignment = (*config).title_align;
    let mut marks_buffer_width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut width: ::core::ffi::c_int = (*con).title_width;
    let mut height: ::core::ffi::c_int = container_titlebar_height()
        as ::core::ffi::c_int;
    let mut text_area: pixman_region32_t = pixman_region32_t {
        extents: pixman_box32_t {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        },
        data: ::core::ptr::null_mut::<pixman_region32_data_t>(),
    };
    pixman_region32_init(&raw mut text_area);
    if !(*con).title_bar.marks_text.is_null() {
        let mut node: *mut sway_text_node = (*con).title_bar.marks_text;
        marks_buffer_width = (*node).width;
        let mut h_padding: ::core::ffi::c_int = 0;
        if title_align.0 == alignment::ALIGN_RIGHT.0 {
            h_padding = (*config).titlebar_h_padding;
        } else {
            h_padding = width - (*config).titlebar_h_padding - marks_buffer_width;
        }
        h_padding = if h_padding > (*config).titlebar_h_padding {
            h_padding
        } else {
            (*config).titlebar_h_padding
        };
        let mut alloc_width: ::core::ffi::c_int = if (*node).width
            < width - h_padding - (*config).titlebar_h_padding
        {
            (*node).width
        } else {
            width - h_padding - (*config).titlebar_h_padding
        };
        alloc_width = if alloc_width > 0 as ::core::ffi::c_int {
            alloc_width
        } else {
            0 as ::core::ffi::c_int
        };
        sway_text_node_set_max_width(node, alloc_width);
        wlr_scene_node_set_position(
            (*node).node,
            h_padding,
            height - (*node).height >> 1 as ::core::ffi::c_int,
        );
        pixman_region32_union_rect(
            &raw mut text_area,
            &raw mut text_area,
            (*(*node).node).x,
            (*(*node).node).y,
            alloc_width as ::core::ffi::c_uint,
            (*node).height as ::core::ffi::c_uint,
        );
    }
    if !(*con).title_bar.title_text.is_null() {
        let mut node_0: *mut sway_text_node = (*con).title_bar.title_text;
        let mut h_padding_0: ::core::ffi::c_int = 0;
        if title_align.0 == alignment::ALIGN_RIGHT.0 {
            h_padding_0 = width - (*config).titlebar_h_padding - (*node_0).width;
        } else if title_align.0 == alignment::ALIGN_CENTER.0 {
            h_padding_0 = width - marks_buffer_width - (*node_0).width
                >> 1 as ::core::ffi::c_int;
        } else {
            h_padding_0 = (*config).titlebar_h_padding;
        }
        h_padding_0 = if h_padding_0 > (*config).titlebar_h_padding {
            h_padding_0
        } else {
            (*config).titlebar_h_padding
        };
        let mut alloc_width_0: ::core::ffi::c_int = if (*node_0).width
            < width - h_padding_0 - (*config).titlebar_h_padding
        {
            (*node_0).width
        } else {
            width - h_padding_0 - (*config).titlebar_h_padding
        };
        alloc_width_0 = if alloc_width_0 > 0 as ::core::ffi::c_int {
            alloc_width_0
        } else {
            0 as ::core::ffi::c_int
        };
        sway_text_node_set_max_width(node_0, alloc_width_0);
        wlr_scene_node_set_position(
            (*node_0).node,
            h_padding_0,
            height - (*node_0).height >> 1 as ::core::ffi::c_int,
        );
        pixman_region32_union_rect(
            &raw mut text_area,
            &raw mut text_area,
            (*(*node_0).node).x,
            (*(*node_0).node).y,
            alloc_width_0 as ::core::ffi::c_uint,
            (*node_0).height as ::core::ffi::c_uint,
        );
    }
    if width <= 0 as ::core::ffi::c_int || height <= 0 as ::core::ffi::c_int {
        pixman_region32_fini(&raw mut text_area);
        return;
    }
    let mut background: pixman_region32_t = pixman_region32_t {
        extents: pixman_box32_t {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        },
        data: ::core::ptr::null_mut::<pixman_region32_data_t>(),
    };
    let mut border: pixman_region32_t = pixman_region32_t {
        extents: pixman_box32_t {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        },
        data: ::core::ptr::null_mut::<pixman_region32_data_t>(),
    };
    let mut thickness: ::core::ffi::c_int = (*config).titlebar_border_thickness;
    pixman_region32_init_rect(
        &raw mut background,
        thickness,
        thickness,
        (width - thickness * 2 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        (height - thickness * 2 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    pixman_region32_init_rect(
        &raw mut border,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        width as ::core::ffi::c_uint,
        height as ::core::ffi::c_uint,
    );
    pixman_region32_subtract(&raw mut border, &raw mut border, &raw mut background);
    pixman_region32_subtract(
        &raw mut background,
        &raw mut background,
        &raw mut text_area,
    );
    pixman_region32_fini(&raw mut text_area);
    update_rect_list((*con).title_bar.background, &raw mut background);
    pixman_region32_fini(&raw mut background);
    update_rect_list((*con).title_bar.border, &raw mut border);
    pixman_region32_fini(&raw mut border);
    container_update(con);
}
#[no_mangle]
pub unsafe extern "C" fn container_update_marks(mut con: *mut sway_container) {
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if (*config).show_marks as ::core::ffi::c_int != 0 && (*(*con).marks).length != 0 {
        let mut len: size_t = 0 as size_t;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*con).marks).length {
            let mut mark: *mut ::core::ffi::c_char = *(*(*con).marks)
                .items
                .offset(i as isize) as *mut ::core::ffi::c_char;
            if *mark.offset(0isize) as ::core::ffi::c_int != '_' as ::core::ffi::c_int {
                len = len.wrapping_add(strlen(mark).wrapping_add(2 as size_t));
            }
            i += 1;
        }
        buffer = calloc(len.wrapping_add(1 as size_t), 1 as size_t)
            as *mut ::core::ffi::c_char;
        let mut part: *mut ::core::ffi::c_char = malloc(len.wrapping_add(1 as size_t))
            as *mut ::core::ffi::c_char;
        if !_sway_assert(
            !buffer.is_null() && !part.is_null(),
            b"[%s:%d] %s:Unable to allocate memory\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
            387 as ::core::ffi::c_int,
            b"void container_update_marks(struct sway_container *)\0".as_ptr()
                as *const ::core::ffi::c_char,
        ) {
            free(buffer as *mut ::core::ffi::c_void);
            return;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*(*con).marks).length {
            let mut mark_0: *mut ::core::ffi::c_char = *(*(*con).marks)
                .items
                .offset(i_0 as isize) as *mut ::core::ffi::c_char;
            if *mark_0.offset(0isize) as ::core::ffi::c_int != '_' as ::core::ffi::c_int
            {
                snprintf(
                    part,
                    len.wrapping_add(1 as size_t),
                    b"[%s]\0".as_ptr() as *const ::core::ffi::c_char,
                    mark_0,
                );
                strcat(buffer, part);
            }
            i_0 += 1;
        }
        free(part as *mut ::core::ffi::c_void);
    }
    if buffer.is_null() {
        if !(*con).title_bar.marks_text.is_null() {
            wlr_scene_node_destroy((*(*con).title_bar.marks_text).node);
            (*con).title_bar.marks_text = ::core::ptr::null_mut::<sway_text_node>();
        }
    } else if (*con).title_bar.marks_text.is_null() {
        let mut colors: *mut border_colors = container_get_current_colors(con);
        (*con).title_bar.marks_text = sway_text_node_create(
            (*con).title_bar.tree,
            buffer,
            &raw mut (*colors).text as *mut ::core::ffi::c_float,
            r#false != 0,
        );
    } else {
        sway_text_node_set_text((*con).title_bar.marks_text, buffer);
    }
    container_arrange_title_bar(con);
    free(buffer as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn container_update_title_text(mut con: *mut sway_container) {
    if (*con).current.formatted_title.is_null() || *(*con).current.formatted_title == 0 {
        if !(*con).title_bar.title_text.is_null() {
            wlr_scene_node_destroy((*(*con).title_bar.title_text).node);
            (*con).title_bar.title_text = ::core::ptr::null_mut::<sway_text_node>();
        }
        return;
    }
    if (*con).title_bar.title_text.is_null() {
        let mut colors: *mut border_colors = container_get_current_colors(con);
        (*con).title_bar.title_text = sway_text_node_create(
            (*con).title_bar.tree,
            (*con).current.formatted_title,
            &raw mut (*colors).text as *mut ::core::ffi::c_float,
            (*config).pango_markup,
        ) as *mut sway_text_node;
    } else {
        sway_text_node_set_text(
            (*con).title_bar.title_text,
            (*con).current.formatted_title,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn container_update_title_bar(mut con: *mut sway_container) {
    if !(*con).title_bar.title_text.is_null() {
        wlr_scene_node_destroy((*(*con).title_bar.title_text).node);
        (*con).title_bar.title_text = ::core::ptr::null_mut::<sway_text_node>();
    }
    if !(*con).current.formatted_title.is_null()
        && *(*con).current.formatted_title as ::core::ffi::c_int != 0
    {
        let mut colors: *mut border_colors = container_get_current_colors(con);
        (*con).title_bar.title_text = sway_text_node_create(
            (*con).title_bar.tree,
            (*con).current.formatted_title,
            &raw mut (*colors).text as *mut ::core::ffi::c_float,
            (*config).pango_markup,
        ) as *mut sway_text_node;
    }
    if !(*con).title_bar.marks_text.is_null() {
        wlr_scene_node_destroy((*(*con).title_bar.marks_text).node);
        (*con).title_bar.marks_text = ::core::ptr::null_mut::<sway_text_node>();
    }
    container_update_marks(con);
    container_arrange_title_bar(con);
}
#[no_mangle]
pub unsafe extern "C" fn container_destroy(mut con: *mut sway_container) {
    if !_sway_assert(
        (*con).node.destroying,
        b"[%s:%d] %s:Tried to free container which wasn't marked as destroying\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        463 as ::core::ffi::c_int,
        b"void container_destroy(struct sway_container *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if !_sway_assert(
        (*con).node.ntxnrefs == 0 as size_t,
        b"[%s:%d] %s:Tried to free container which is still referenced by transactions\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        467 as ::core::ffi::c_int,
        b"void container_destroy(struct sway_container *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    free((*con).title as *mut ::core::ffi::c_void);
    free((*con).pending.formatted_title as *mut ::core::ffi::c_void);
    free((*con).current.formatted_title as *mut ::core::ffi::c_void);
    free((*con).title_format as *mut ::core::ffi::c_void);
    list_free((*con).pending.children);
    list_free((*con).current.children);
    list_free_items_and_destroy((*con).marks);
    if !(*con).view.is_null() && (*(*con).view).container == con {
        (*(*con).view).container = ::core::ptr::null_mut::<sway_container>();
        if (*(*con).view).destroying {
            view_destroy((*con).view);
        }
    }
    scene_node_disown_children((*con).content_tree);
    wlr_scene_node_destroy(&raw mut (*(*con).scene_tree).node);
    free(con as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn container_begin_destroy(mut con: *mut sway_container) {
    if !(*con).view.is_null() {
        ipc_event_window(con, b"close\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if (*con).pending.fullscreen_mode.0 == sway_fullscreen_mode::FULLSCREEN_WORKSPACE.0
        && !(*con).pending.workspace.is_null()
    {
        (*(*con).pending.workspace).fullscreen = ::core::ptr::null_mut::<
            sway_container,
        >();
    }
    if (*con).scratchpad as ::core::ffi::c_int != 0
        && (*con).pending.fullscreen_mode.0 == sway_fullscreen_mode::FULLSCREEN_GLOBAL.0
    {
        container_fullscreen_disable(con);
    }
    wl_signal_emit_mutable(
        &raw mut (*con).node.events.destroy,
        &raw mut (*con).node as *mut ::core::ffi::c_void,
    );
    container_end_mouse_operation(con);
    node_set_dirty(&raw mut (*con).node);
    (*con).node.destroying = r#true != 0;
    if (*con).scratchpad {
        root_scratchpad_remove_container(con);
    }
    if (*con).pending.fullscreen_mode.0 == sway_fullscreen_mode::FULLSCREEN_GLOBAL.0 {
        container_fullscreen_disable(con);
    }
    if !(*con).pending.parent.is_null() || !(*con).pending.workspace.is_null() {
        container_detach(con);
    }
}
#[no_mangle]
pub unsafe extern "C" fn container_reap_empty(mut con: *mut sway_container) {
    if !(*con).view.is_null() {
        return;
    }
    let mut ws: *mut sway_workspace = (*con).pending.workspace;
    while !con.is_null() {
        if (*(*con).pending.children).length != 0 {
            return;
        }
        let mut parent: *mut sway_container = (*con).pending.parent;
        container_begin_destroy(con);
        con = parent;
    }
    if !ws.is_null() {
        workspace_consider_destroy(ws);
    }
}
#[no_mangle]
pub unsafe extern "C" fn container_flatten(
    mut container: *mut sway_container,
) -> *mut sway_container {
    if !(*container).view.is_null() {
        return ::core::ptr::null_mut::<sway_container>();
    }
    while !container.is_null()
        && (*(*container).pending.children).length == 1 as ::core::ffi::c_int
    {
        let mut child: *mut sway_container = *(*(*container).pending.children)
            .items
            .offset(0isize) as *mut sway_container;
        let mut parent: *mut sway_container = (*container).pending.parent;
        container_replace(container, child);
        container_begin_destroy(container);
        container = parent;
    }
    return container;
}
#[no_mangle]
pub unsafe extern "C" fn container_find_child(
    mut container: *mut sway_container,
    mut test: Option<
        unsafe extern "C" fn(*mut sway_container, *mut ::core::ffi::c_void) -> bool,
    >,
    mut data: *mut ::core::ffi::c_void,
) -> *mut sway_container {
    if (*container).pending.children.is_null() {
        return ::core::ptr::null_mut::<sway_container>();
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*container).pending.children).length {
        let mut child: *mut sway_container = *(*(*container).pending.children)
            .items
            .offset(i as isize) as *mut sway_container;
        if test.expect("non-null function pointer")(child, data) {
            return child;
        }
        let mut res: *mut sway_container = container_find_child(child, test, data);
        if !res.is_null() {
            return res;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<sway_container>();
}
#[no_mangle]
pub unsafe extern "C" fn container_for_each_child(
    mut container: *mut sway_container,
    mut f: Option<
        unsafe extern "C" fn(*mut sway_container, *mut ::core::ffi::c_void) -> (),
    >,
    mut data: *mut ::core::ffi::c_void,
) {
    if !(*container).pending.children.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*container).pending.children).length {
            let mut child: *mut sway_container = *(*(*container).pending.children)
                .items
                .offset(i as isize) as *mut sway_container;
            f.expect("non-null function pointer")(child, data);
            container_for_each_child(child, f, data);
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn container_obstructing_fullscreen_container(
    mut container: *mut sway_container,
) -> *mut sway_container {
    let mut workspace: *mut sway_workspace = (*container).pending.workspace;
    if !workspace.is_null() && !(*workspace).fullscreen.is_null()
        && !container_is_fullscreen_or_child(container)
    {
        if container_is_transient_for(container, (*workspace).fullscreen) {
            return ::core::ptr::null_mut::<sway_container>();
        }
        return (*workspace).fullscreen;
    }
    let mut fullscreen_global: *mut sway_container = (*root).fullscreen_global;
    if !fullscreen_global.is_null() && container != fullscreen_global
        && !container_has_ancestor(container, fullscreen_global)
    {
        if container_is_transient_for(container, fullscreen_global) {
            return ::core::ptr::null_mut::<sway_container>();
        }
        return fullscreen_global;
    }
    return ::core::ptr::null_mut::<sway_container>();
}
#[no_mangle]
pub unsafe extern "C" fn container_has_ancestor(
    mut descendant: *mut sway_container,
    mut ancestor: *mut sway_container,
) -> bool {
    while !descendant.is_null() {
        descendant = (*descendant).pending.parent;
        if descendant == ancestor {
            return r#true != 0;
        }
    }
    return r#false != 0;
}
unsafe extern "C" fn escape_pango_markup(
    mut buffer: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut length: size_t = escape_markup_text(
        buffer,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    let mut escaped_title: *mut ::core::ffi::c_char = calloc(
        length.wrapping_add(1 as size_t),
        ::core::mem::size_of::<::core::ffi::c_char>(),
    ) as *mut ::core::ffi::c_char;
    escape_markup_text(buffer, escaped_title);
    return escaped_title;
}
unsafe extern "C" fn append_prop(
    mut buffer: *mut ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> size_t {
    if value.is_null() {
        return 0 as size_t;
    }
    if (*config).pango_markup {
        let mut escaped_value: *mut ::core::ffi::c_char = escape_pango_markup(value);
        lenient_strcat(buffer, escaped_value);
        let mut len: size_t = strlen(escaped_value);
        free(escaped_value as *mut ::core::ffi::c_void);
        return len;
    } else {
        lenient_strcat(buffer, value);
        return strlen(value);
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_title_format(
    mut container: *mut sway_container,
    mut buffer: *mut ::core::ffi::c_char,
) -> size_t {
    if (*container).title_format.is_null()
        || strcmp(
            (*container).title_format,
            b"%title\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        if !(*container).view.is_null() {
            return append_prop(buffer, view_get_title((*container).view))
        } else {
            return container_build_representation(
                (*container).pending.layout,
                (*container).pending.children,
                buffer,
            )
        }
    }
    let mut len: size_t = 0 as size_t;
    let mut format: *mut ::core::ffi::c_char = (*container).title_format;
    let mut next: *mut ::core::ffi::c_char = strchr(format, '%' as ::core::ffi::c_int);
    while !next.is_null() {
        lenient_strncat(buffer, format, next.offset_from(format) as size_t);
        len = len.wrapping_add(next.offset_from(format) as size_t);
        format = next;
        if has_prefix(next, b"%title\0".as_ptr() as *const ::core::ffi::c_char) {
            if !(*container).view.is_null() {
                len = len
                    .wrapping_add(
                        append_prop(buffer, view_get_title((*container).view)),
                    );
            } else {
                len = len
                    .wrapping_add(
                        container_build_representation(
                            (*container).pending.layout,
                            (*container).pending.children,
                            buffer,
                        ),
                    );
            }
            format = format
                .offset(
                    strlen(b"%title\0".as_ptr() as *const ::core::ffi::c_char) as isize,
                );
        } else if !(*container).view.is_null() {
            if has_prefix(next, b"%app_id\0".as_ptr() as *const ::core::ffi::c_char) {
                len = len
                    .wrapping_add(
                        append_prop(buffer, view_get_app_id((*container).view)),
                    );
                format = format
                    .offset(
                        strlen(b"%app_id\0".as_ptr() as *const ::core::ffi::c_char)
                            as isize,
                    );
            } else if has_prefix(
                next,
                b"%class\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                len = len
                    .wrapping_add(
                        append_prop(buffer, view_get_class((*container).view)),
                    );
                format = format
                    .offset(
                        strlen(b"%class\0".as_ptr() as *const ::core::ffi::c_char)
                            as isize,
                    );
            } else if has_prefix(
                next,
                b"%instance\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                len = len
                    .wrapping_add(
                        append_prop(buffer, view_get_instance((*container).view)),
                    );
                format = format
                    .offset(
                        strlen(b"%instance\0".as_ptr() as *const ::core::ffi::c_char)
                            as isize,
                    );
            } else if has_prefix(
                next,
                b"%shell\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                len = len
                    .wrapping_add(
                        append_prop(buffer, view_get_shell((*container).view)),
                    );
                format = format
                    .offset(
                        strlen(b"%shell\0".as_ptr() as *const ::core::ffi::c_char)
                            as isize,
                    );
            } else if has_prefix(
                next,
                b"%sandbox_engine\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                len = len
                    .wrapping_add(
                        append_prop(buffer, view_get_sandbox_engine((*container).view)),
                    );
                format = format
                    .offset(
                        strlen(
                            b"%sandbox_engine\0".as_ptr() as *const ::core::ffi::c_char,
                        ) as isize,
                    );
            } else if has_prefix(
                next,
                b"%sandbox_app_id\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                len = len
                    .wrapping_add(
                        append_prop(buffer, view_get_sandbox_app_id((*container).view)),
                    );
                format = format
                    .offset(
                        strlen(
                            b"%sandbox_app_id\0".as_ptr() as *const ::core::ffi::c_char,
                        ) as isize,
                    );
            } else if has_prefix(
                next,
                b"%sandbox_instance_id\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                len = len
                    .wrapping_add(
                        append_prop(
                            buffer,
                            view_get_sandbox_instance_id((*container).view),
                        ),
                    );
                format = format
                    .offset(
                        strlen(
                            b"%sandbox_instance_id\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ) as isize,
                    );
            } else {
                lenient_strcat(buffer, b"%\0".as_ptr() as *const ::core::ffi::c_char);
                format = format.offset(1);
                len = len.wrapping_add(1);
            }
        } else {
            lenient_strcat(buffer, b"%\0".as_ptr() as *const ::core::ffi::c_char);
            format = format.offset(1);
            len = len.wrapping_add(1);
        }
        next = strchr(format, '%' as ::core::ffi::c_int);
    }
    lenient_strcat(buffer, format);
    len = len.wrapping_add(strlen(format));
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn container_build_representation(
    mut layout: sway_container_layout,
    mut children: *mut list_t,
    mut buffer: *mut ::core::ffi::c_char,
) -> size_t {
    let mut len: size_t = 2 as size_t;
    match layout {
        sway_container_layout(2) => {
            lenient_strcat(buffer, b"V[\0".as_ptr() as *const ::core::ffi::c_char);
        }
        sway_container_layout(1) => {
            lenient_strcat(buffer, b"H[\0".as_ptr() as *const ::core::ffi::c_char);
        }
        sway_container_layout(4) => {
            lenient_strcat(buffer, b"T[\0".as_ptr() as *const ::core::ffi::c_char);
        }
        sway_container_layout(3) => {
            lenient_strcat(buffer, b"S[\0".as_ptr() as *const ::core::ffi::c_char);
        }
        sway_container_layout(0) => {
            lenient_strcat(buffer, b"D[\0".as_ptr() as *const ::core::ffi::c_char);
        }
        _ => {}
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*children).length {
        if i != 0 as ::core::ffi::c_int {
            len = len.wrapping_add(1);
            lenient_strcat(buffer, b" \0".as_ptr() as *const ::core::ffi::c_char);
        }
        let mut child: *mut sway_container = *(*children).items.offset(i as isize)
            as *mut sway_container;
        let mut identifier: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if !(*child).view.is_null() {
            identifier = view_get_class((*child).view);
            if identifier.is_null() {
                identifier = view_get_app_id((*child).view);
            }
        } else {
            identifier = (*child).pending.formatted_title;
        }
        if !identifier.is_null() {
            len = len.wrapping_add(strlen(identifier));
            lenient_strcat(buffer, identifier);
        } else {
            len = len
                .wrapping_add(
                    strlen(b"(null)\0".as_ptr() as *const ::core::ffi::c_char),
                );
            lenient_strcat(buffer, b"(null)\0".as_ptr() as *const ::core::ffi::c_char);
        }
        i += 1;
    }
    len = len.wrapping_add(1);
    lenient_strcat(buffer, b"]\0".as_ptr() as *const ::core::ffi::c_char);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn container_update_representation(mut con: *mut sway_container) {
    if (*con).view.is_null() {
        let mut len: size_t = parse_title_format(
            con,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        free((*con).pending.formatted_title as *mut ::core::ffi::c_void);
        (*con).pending.formatted_title = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if len != 0 {
            (*con).pending.formatted_title = calloc(
                len.wrapping_add(1 as size_t),
                ::core::mem::size_of::<::core::ffi::c_char>(),
            ) as *mut ::core::ffi::c_char;
            if !_sway_assert(
                !(*con).pending.formatted_title.is_null(),
                b"[%s:%d] %s:Unable to allocate title string\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
                774 as ::core::ffi::c_int,
                b"void container_update_representation(struct sway_container *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            ) {
                return;
            }
            parse_title_format(con, (*con).pending.formatted_title);
        }
        node_set_dirty(&raw mut (*con).node);
    }
    if !(*con).pending.parent.is_null() {
        container_update_representation((*con).pending.parent);
    } else if !(*con).pending.workspace.is_null() {
        workspace_update_representation((*con).pending.workspace);
    }
}
#[no_mangle]
pub unsafe extern "C" fn container_titlebar_height() -> size_t {
    return ((*config).font_height
        + (*config).titlebar_v_padding * 2 as ::core::ffi::c_int) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn floating_calculate_constraints(
    mut min_width: *mut ::core::ffi::c_int,
    mut max_width: *mut ::core::ffi::c_int,
    mut min_height: *mut ::core::ffi::c_int,
    mut max_height: *mut ::core::ffi::c_int,
) {
    if (*config).floating_minimum_width == -1 as int32_t {
        *min_width = 0 as ::core::ffi::c_int;
    } else if (*config).floating_minimum_width == 0 as int32_t {
        *min_width = 75 as ::core::ffi::c_int;
    } else {
        *min_width = (*config).floating_minimum_width as ::core::ffi::c_int;
    }
    if (*config).floating_minimum_height == -1 as int32_t {
        *min_height = 0 as ::core::ffi::c_int;
    } else if (*config).floating_minimum_height == 0 as int32_t {
        *min_height = 50 as ::core::ffi::c_int;
    } else {
        *min_height = (*config).floating_minimum_height as ::core::ffi::c_int;
    }
    let mut r#box: wlr_box = wlr_box {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    wlr_output_layout_get_box(
        (*root).output_layout,
        ::core::ptr::null_mut::<wlr_output>(),
        &raw mut r#box,
    );
    if (*config).floating_maximum_width == -1 as int32_t {
        *max_width = INT_MAX;
    } else if (*config).floating_maximum_width == 0 as int32_t {
        *max_width = r#box.width;
    } else {
        *max_width = (*config).floating_maximum_width as ::core::ffi::c_int;
    }
    if (*config).floating_maximum_height == -1 as int32_t {
        *max_height = INT_MAX;
    } else if (*config).floating_maximum_height == 0 as int32_t {
        *max_height = r#box.height;
    } else {
        *max_height = (*config).floating_maximum_height as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn floating_fix_coordinates(
    mut con: *mut sway_container,
    mut old: *mut wlr_box,
    mut new: *mut wlr_box,
) {
    if (*old).width == 0 || (*old).height == 0 {
        container_floating_move_to_center(con);
    } else {
        let mut rel_x: ::core::ffi::c_double = (*con).pending.x
            - (*old).x as ::core::ffi::c_double
            + (*con).pending.width / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        let mut rel_y: ::core::ffi::c_double = (*con).pending.y
            - (*old).y as ::core::ffi::c_double
            + (*con).pending.height / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*con).pending.x = (*new).x as ::core::ffi::c_double
            + rel_x * (*new).width as ::core::ffi::c_double
                / (*old).width as ::core::ffi::c_double
            - (*con).pending.width / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*con).pending.y = (*new).y as ::core::ffi::c_double
            + rel_y * (*new).height as ::core::ffi::c_double
                / (*old).height as ::core::ffi::c_double
            - (*con).pending.height / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Transformed container %p to coords (%f, %f)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
            843 as ::core::ffi::c_int,
            con,
            (*con).pending.x,
            (*con).pending.y,
        );
    };
}
unsafe extern "C" fn floating_natural_resize(mut con: *mut sway_container) {
    let mut min_width: ::core::ffi::c_int = 0;
    let mut max_width: ::core::ffi::c_int = 0;
    let mut min_height: ::core::ffi::c_int = 0;
    let mut max_height: ::core::ffi::c_int = 0;
    floating_calculate_constraints(
        &raw mut min_width,
        &raw mut max_width,
        &raw mut min_height,
        &raw mut max_height,
    );
    if (*con).view.is_null() {
        (*con).pending.width = fmax(
            min_width as ::core::ffi::c_double,
            fmin((*con).pending.width, max_width as ::core::ffi::c_double),
        );
        (*con).pending.height = fmax(
            min_height as ::core::ffi::c_double,
            fmin((*con).pending.height, max_height as ::core::ffi::c_double),
        );
    } else {
        let mut view: *mut sway_view = (*con).view;
        (*con).pending.content_width = fmax(
            min_width as ::core::ffi::c_double,
            fmin(
                (*view).natural_width as ::core::ffi::c_double,
                max_width as ::core::ffi::c_double,
            ),
        );
        (*con).pending.content_height = fmax(
            min_height as ::core::ffi::c_double,
            fmin(
                (*view).natural_height as ::core::ffi::c_double,
                max_height as ::core::ffi::c_double,
            ),
        );
        container_set_geometry_from_content(con);
    };
}
#[no_mangle]
pub unsafe extern "C" fn container_floating_resize_and_center(
    mut con: *mut sway_container,
) {
    let mut ws: *mut sway_workspace = (*con).pending.workspace;
    if ws.is_null() {
        floating_natural_resize(con);
        return;
    }
    let mut ob: wlr_box = wlr_box {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    wlr_output_layout_get_box(
        (*root).output_layout,
        (*(*ws).output).wlr_output,
        &raw mut ob,
    );
    if wlr_box_empty(&raw mut ob) {
        (*con).pending.x = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*con).pending.y = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*con).pending.width = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*con).pending.height = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        return;
    }
    floating_natural_resize(con);
    if (*con).view.is_null() {
        if (*con).pending.width > (*ws).width as ::core::ffi::c_double
            || (*con).pending.height > (*ws).height as ::core::ffi::c_double
        {
            (*con).pending.x = ob.x as ::core::ffi::c_double
                + (ob.width as ::core::ffi::c_double - (*con).pending.width)
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            (*con).pending.y = ob.y as ::core::ffi::c_double
                + (ob.height as ::core::ffi::c_double - (*con).pending.height)
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            (*con).pending.x = (*ws).x
                + ((*ws).width as ::core::ffi::c_double - (*con).pending.width)
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            (*con).pending.y = (*ws).y
                + ((*ws).height as ::core::ffi::c_double - (*con).pending.height)
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    } else {
        if (*con).pending.content_width > (*ws).width as ::core::ffi::c_double
            || (*con).pending.content_height > (*ws).height as ::core::ffi::c_double
        {
            (*con).pending.content_x = ob.x as ::core::ffi::c_double
                + (ob.width as ::core::ffi::c_double - (*con).pending.content_width)
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            (*con).pending.content_y = ob.y as ::core::ffi::c_double
                + (ob.height as ::core::ffi::c_double - (*con).pending.content_height)
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            (*con).pending.content_x = (*ws).x
                + ((*ws).width as ::core::ffi::c_double - (*con).pending.content_width)
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            (*con).pending.content_y = (*ws).y
                + ((*ws).height as ::core::ffi::c_double - (*con).pending.content_height)
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        (*con).pending.border_bottom = r#true != 0;
        (*con).pending.border_top = (*con).pending.border_bottom;
        (*con).pending.border_right = r#true != 0;
        (*con).pending.border_left = (*con).pending.border_right;
        container_set_geometry_from_content(con);
    };
}
#[no_mangle]
pub unsafe extern "C" fn container_floating_set_default_size(
    mut con: *mut sway_container,
) {
    if !_sway_assert(
        !(*con).pending.workspace.is_null(),
        b"[%s:%d] %s:Expected a container on a workspace\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        911 as ::core::ffi::c_int,
        b"void container_floating_set_default_size(struct sway_container *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    let mut min_width: ::core::ffi::c_int = 0;
    let mut max_width: ::core::ffi::c_int = 0;
    let mut min_height: ::core::ffi::c_int = 0;
    let mut max_height: ::core::ffi::c_int = 0;
    floating_calculate_constraints(
        &raw mut min_width,
        &raw mut max_width,
        &raw mut min_height,
        &raw mut max_height,
    );
    let mut r#box: wlr_box = wlr_box {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    workspace_get_box((*con).pending.workspace, &raw mut r#box);
    let mut width: ::core::ffi::c_double = fmax(
        min_width as ::core::ffi::c_double,
        fmin(
            r#box.width as ::core::ffi::c_double * 0.5f64,
            max_width as ::core::ffi::c_double,
        ),
    );
    let mut height: ::core::ffi::c_double = fmax(
        min_height as ::core::ffi::c_double,
        fmin(
            r#box.height as ::core::ffi::c_double * 0.75f64,
            max_height as ::core::ffi::c_double,
        ),
    );
    if (*con).view.is_null() {
        (*con).pending.width = width;
        (*con).pending.height = height;
    } else {
        (*con).pending.content_width = width;
        (*con).pending.content_height = height;
        container_set_geometry_from_content(con);
    };
}
#[no_mangle]
pub unsafe extern "C" fn container_set_resizing(
    mut con: *mut sway_container,
    mut resizing: bool,
) {
    if con.is_null() {
        return;
    }
    if !(*con).view.is_null() {
        if (*(*(*con).view).r#impl).set_resizing.is_some() {
            (*(*(*con).view).r#impl)
                .set_resizing
                .expect("non-null function pointer")((*con).view, resizing);
        }
    } else {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*con).pending.children).length {
            let mut child: *mut sway_container = *(*(*con).pending.children)
                .items
                .offset(i as isize) as *mut sway_container;
            container_set_resizing(child, resizing);
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn container_set_floating(
    mut container: *mut sway_container,
    mut enable: bool,
) {
    if container_is_floating(container) as ::core::ffi::c_int
        == enable as ::core::ffi::c_int
    {
        return;
    }
    let mut seat: *mut sway_seat = input_manager_current_seat();
    let mut workspace: *mut sway_workspace = (*container).pending.workspace;
    let mut focus: *mut sway_container = seat_get_focused_container(seat);
    let mut set_focus: bool = focus == container;
    if enable {
        let mut old_parent: *mut sway_container = (*container).pending.parent;
        container_detach(container);
        workspace_add_floating(workspace, container);
        if !(*container).view.is_null() {
            view_set_tiled((*container).view, r#false != 0);
            if (*(*container).view).using_csd {
                (*container).saved_border = (*container).pending.border;
                (*container).pending.border = sway_container_border::B_CSD;
                if !(*(*container).view).xdg_decoration.is_null() {
                    let mut deco: *mut sway_xdg_decoration = (*(*container).view)
                        .xdg_decoration;
                    wlr_xdg_toplevel_decoration_v1_set_mode(
                        (*deco).wlr_xdg_decoration,
                        wlr_xdg_toplevel_decoration_v1_mode::WLR_XDG_TOPLEVEL_DECORATION_V1_MODE_CLIENT_SIDE,
                    );
                }
            }
        }
        container_floating_set_default_size(container);
        container_floating_resize_and_center(container);
        if !old_parent.is_null() {
            if set_focus {
                seat_set_raw_focus(seat, &raw mut (*old_parent).node);
                seat_set_raw_focus(seat, &raw mut (*container).node);
            }
            container_reap_empty(old_parent);
        }
    } else {
        if (*container).scratchpad {
            root_scratchpad_remove_container(container);
        }
        container_detach(container);
        let mut reference: *mut sway_container = seat_get_focus_inactive_tiling(
            seat,
            workspace,
        );
        if !reference.is_null() {
            if !(*reference).view.is_null() {
                container_add_sibling(reference, container, true);
            } else {
                container_add_child(reference, container);
            }
            (*container).pending.width = (*reference).pending.width;
            (*container).pending.height = (*reference).pending.height;
        } else {
            let mut other: *mut sway_container = workspace_add_tiling(
                workspace,
                container,
            );
            (*other).pending.width = (*workspace).width as ::core::ffi::c_double;
            (*other).pending.height = (*workspace).height as ::core::ffi::c_double;
        }
        if !(*container).view.is_null() {
            view_set_tiled((*container).view, r#true != 0);
            if (*(*container).view).using_csd {
                (*container).pending.border = (*container).saved_border;
                if !(*(*container).view).xdg_decoration.is_null() {
                    let mut deco_0: *mut sway_xdg_decoration = (*(*container).view)
                        .xdg_decoration;
                    wlr_xdg_toplevel_decoration_v1_set_mode(
                        (*deco_0).wlr_xdg_decoration,
                        wlr_xdg_toplevel_decoration_v1_mode::WLR_XDG_TOPLEVEL_DECORATION_V1_MODE_SERVER_SIDE,
                    );
                }
            }
        }
        (*container).width_fraction = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*container).height_fraction = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    container_end_mouse_operation(container);
    ipc_event_window(container, b"floating\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn container_set_geometry_from_content(
    mut con: *mut sway_container,
) {
    if !_sway_assert(
        !(*con).view.is_null(),
        b"[%s:%d] %s:Expected a view\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        1033 as ::core::ffi::c_int,
        b"void container_set_geometry_from_content(struct sway_container *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if !_sway_assert(
        container_is_floating(con),
        b"[%s:%d] %s:Expected a floating view\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        1036 as ::core::ffi::c_int,
        b"void container_set_geometry_from_content(struct sway_container *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    let mut border_width: size_t = 0 as size_t;
    let mut top: size_t = 0 as size_t;
    if (*con).pending.border.0 != sway_container_border::B_CSD.0
        && (*con).pending.fullscreen_mode.0 == 0
    {
        border_width = ((*con).pending.border_thickness
            * ((*con).pending.border.0 != sway_container_border::B_NONE.0)
                as ::core::ffi::c_int) as size_t;
        top = if (*con).pending.border.0 == sway_container_border::B_NORMAL.0 {
            container_titlebar_height()
        } else {
            border_width
        };
    }
    (*con).pending.x = (*con).pending.content_x - border_width as ::core::ffi::c_double;
    (*con).pending.y = (*con).pending.content_y - top as ::core::ffi::c_double;
    (*con).pending.width = (*con).pending.content_width
        + border_width.wrapping_mul(2 as size_t) as ::core::ffi::c_double;
    (*con).pending.height = top as ::core::ffi::c_double + (*con).pending.content_height
        + border_width as ::core::ffi::c_double;
    node_set_dirty(&raw mut (*con).node);
}
#[no_mangle]
pub unsafe extern "C" fn container_is_floating(
    mut container: *mut sway_container,
) -> bool {
    if (*container).pending.parent.is_null() && !(*container).pending.workspace.is_null()
        && list_find(
            (*(*container).pending.workspace).floating,
            container as *const ::core::ffi::c_void,
        ) != -1 as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    if (*container).scratchpad {
        return r#true != 0;
    }
    return r#false != 0;
}
#[no_mangle]
pub unsafe extern "C" fn container_get_box(
    mut container: *mut sway_container,
    mut r#box: *mut wlr_box,
) {
    (*r#box).x = (*container).pending.x as ::core::ffi::c_int;
    (*r#box).y = (*container).pending.y as ::core::ffi::c_int;
    (*r#box).width = (*container).pending.width as ::core::ffi::c_int;
    (*r#box).height = (*container).pending.height as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn container_floating_translate(
    mut con: *mut sway_container,
    mut x_amount: ::core::ffi::c_double,
    mut y_amount: ::core::ffi::c_double,
) {
    (*con).pending.x += x_amount;
    (*con).pending.y += y_amount;
    (*con).pending.content_x += x_amount;
    (*con).pending.content_y += y_amount;
    if !(*con).pending.children.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*con).pending.children).length {
            let mut child: *mut sway_container = *(*(*con).pending.children)
                .items
                .offset(i as isize) as *mut sway_container;
            container_floating_translate(child, x_amount, y_amount);
            i += 1;
        }
    }
    node_set_dirty(&raw mut (*con).node);
}
#[no_mangle]
pub unsafe extern "C" fn container_floating_find_output(
    mut con: *mut sway_container,
) -> *mut sway_output {
    let mut center_x: ::core::ffi::c_double = (*con).pending.x
        + (*con).pending.width / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut center_y: ::core::ffi::c_double = (*con).pending.y
        + (*con).pending.height / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut closest_output: *mut sway_output = ::core::ptr::null_mut::<sway_output>();
    let mut closest_distance: ::core::ffi::c_double = DBL_MAX;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*root).outputs).length {
        let mut output: *mut sway_output = *(*(*root).outputs).items.offset(i as isize)
            as *mut sway_output;
        let mut output_box: wlr_box = wlr_box {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let mut closest_x: ::core::ffi::c_double = 0.;
        let mut closest_y: ::core::ffi::c_double = 0.;
        output_get_box(output, &raw mut output_box);
        wlr_box_closest_point(
            &raw mut output_box,
            center_x,
            center_y,
            &raw mut closest_x,
            &raw mut closest_y,
        );
        if center_x == closest_x && center_y == closest_y {
            return output;
        }
        let mut x_dist: ::core::ffi::c_double = closest_x - center_x;
        let mut y_dist: ::core::ffi::c_double = closest_y - center_y;
        let mut distance: ::core::ffi::c_double = x_dist * x_dist + y_dist * y_dist;
        if distance < closest_distance {
            closest_output = output;
            closest_distance = distance;
        }
        i += 1;
    }
    return closest_output;
}
#[no_mangle]
pub unsafe extern "C" fn container_floating_move_to(
    mut con: *mut sway_container,
    mut lx: ::core::ffi::c_double,
    mut ly: ::core::ffi::c_double,
) {
    if !_sway_assert(
        container_is_floating(con),
        b"[%s:%d] %s:Expected a floating container\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        1129 as ::core::ffi::c_int,
        b"void container_floating_move_to(struct sway_container *, double, double)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    container_floating_translate(con, lx - (*con).pending.x, ly - (*con).pending.y);
    if container_is_scratchpad_hidden(con) {
        return;
    }
    let mut old_workspace: *mut sway_workspace = (*con).pending.workspace;
    let mut new_output: *mut sway_output = container_floating_find_output(con);
    if !_sway_assert(
        !new_output.is_null(),
        b"[%s:%d] %s:Unable to find any output\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        1139 as ::core::ffi::c_int,
        b"void container_floating_move_to(struct sway_container *, double, double)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    let mut new_workspace: *mut sway_workspace = output_get_active_workspace(new_output);
    if !new_workspace.is_null() && old_workspace != new_workspace {
        container_detach(con);
        workspace_add_floating(new_workspace, con);
        arrange_workspace(old_workspace);
        arrange_workspace(new_workspace);
        if (*con).scratchpad {
            let mut output_box: wlr_box = wlr_box {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            output_get_box(new_output, &raw mut output_box);
            (*con).transform = output_box;
        }
        workspace_detect_urgent(old_workspace);
        workspace_detect_urgent(new_workspace);
    }
}
#[no_mangle]
pub unsafe extern "C" fn container_floating_move_to_center(
    mut con: *mut sway_container,
) {
    if !_sway_assert(
        container_is_floating(con),
        b"[%s:%d] %s:Expected a floating container\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        1162 as ::core::ffi::c_int,
        b"void container_floating_move_to_center(struct sway_container *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    let mut ws: *mut sway_workspace = (*con).pending.workspace;
    let mut new_lx: ::core::ffi::c_double = (*ws).x
        + ((*ws).width as ::core::ffi::c_double - (*con).pending.width)
            / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut new_ly: ::core::ffi::c_double = (*ws).y
        + ((*ws).height as ::core::ffi::c_double - (*con).pending.height)
            / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    container_floating_translate(
        con,
        new_lx - (*con).pending.x,
        new_ly - (*con).pending.y,
    );
}
unsafe extern "C" fn find_urgent_iterator(
    mut con: *mut sway_container,
    mut data: *mut ::core::ffi::c_void,
) -> bool {
    return !(*con).view.is_null()
        && view_is_urgent((*con).view) as ::core::ffi::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn container_has_urgent_child(
    mut container: *mut sway_container,
) -> bool {
    return !container_find_child(
            container,
            Some(
                find_urgent_iterator
                    as unsafe extern "C" fn(
                        *mut sway_container,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
            ),
            NULL,
        )
        .is_null();
}
#[no_mangle]
pub unsafe extern "C" fn container_end_mouse_operation(
    mut container: *mut sway_container,
) {
    let mut seat: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
    seat = ((*server.input).seats.next as *mut ::core::ffi::c_char)
        .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
        as *mut sway_seat;
    while &raw mut (*seat).link != &raw mut (*server.input).seats {
        seatop_unref(seat, container);
        seat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
            as *mut sway_seat;
    }
}
unsafe extern "C" fn set_fullscreen(mut con: *mut sway_container, mut enable: bool) {
    if (*con).view.is_null() {
        return;
    }
    if (*(*(*con).view).r#impl).set_fullscreen.is_some() {
        (*(*(*con).view).r#impl)
            .set_fullscreen
            .expect("non-null function pointer")((*con).view, enable);
        if !(*(*con).view).foreign_toplevel.is_null() {
            wlr_foreign_toplevel_handle_v1_set_fullscreen(
                (*(*con).view).foreign_toplevel,
                enable,
            );
        }
    }
}
unsafe extern "C" fn container_fullscreen_workspace(mut con: *mut sway_container) {
    if !_sway_assert(
        (*con).pending.fullscreen_mode.0 == sway_fullscreen_mode::FULLSCREEN_NONE.0,
        b"[%s:%d] %s:Expected a non-fullscreen container\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        1201 as ::core::ffi::c_int,
        b"void container_fullscreen_workspace(struct sway_container *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    set_fullscreen(con, r#true != 0);
    (*con).pending.fullscreen_mode = sway_fullscreen_mode::FULLSCREEN_WORKSPACE;
    (*con).saved_x = (*con).pending.x;
    (*con).saved_y = (*con).pending.y;
    (*con).saved_width = (*con).pending.width;
    (*con).saved_height = (*con).pending.height;
    if !(*con).pending.workspace.is_null() {
        (*(*con).pending.workspace).fullscreen = con;
        let mut seat: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
        let mut focus_ws: *mut sway_workspace = ::core::ptr::null_mut::<
            sway_workspace,
        >();
        seat = ((*server.input).seats.next as *mut ::core::ffi::c_char)
            .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
            as *mut sway_seat;
        while &raw mut (*seat).link != &raw mut (*server.input).seats {
            focus_ws = seat_get_focused_workspace(seat);
            if focus_ws == (*con).pending.workspace {
                seat_set_focus_container(seat, con);
            } else {
                let mut focus: *mut sway_node = seat_get_focus_inactive(
                    seat,
                    &raw mut (*root).node,
                );
                seat_set_raw_focus(seat, &raw mut (*con).node);
                seat_set_raw_focus(seat, focus);
            }
            seat = ((*seat).link.next as *mut ::core::ffi::c_char)
                .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
                as *mut sway_seat;
        }
    }
    container_end_mouse_operation(con);
    ipc_event_window(con, b"fullscreen_mode\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn container_fullscreen_global(mut con: *mut sway_container) {
    if !_sway_assert(
        (*con).pending.fullscreen_mode.0 == sway_fullscreen_mode::FULLSCREEN_NONE.0,
        b"[%s:%d] %s:Expected a non-fullscreen container\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        1235 as ::core::ffi::c_int,
        b"void container_fullscreen_global(struct sway_container *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    set_fullscreen(con, r#true != 0);
    (*root).fullscreen_global = con;
    (*con).saved_x = (*con).pending.x;
    (*con).saved_y = (*con).pending.y;
    (*con).saved_width = (*con).pending.width;
    (*con).saved_height = (*con).pending.height;
    let mut seat: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
    seat = ((*server.input).seats.next as *mut ::core::ffi::c_char)
        .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
        as *mut sway_seat;
    while &raw mut (*seat).link != &raw mut (*server.input).seats {
        let mut focus: *mut sway_container = seat_get_focused_container(seat);
        if !focus.is_null() && focus != con {
            seat_set_focus_container(seat, con);
        }
        seat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
            as *mut sway_seat;
    }
    (*con).pending.fullscreen_mode = sway_fullscreen_mode::FULLSCREEN_GLOBAL;
    container_end_mouse_operation(con);
    ipc_event_window(con, b"fullscreen_mode\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn container_fullscreen_disable(mut con: *mut sway_container) {
    if !_sway_assert(
        (*con).pending.fullscreen_mode.0 != sway_fullscreen_mode::FULLSCREEN_NONE.0,
        b"[%s:%d] %s:Expected a fullscreen container\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        1261 as ::core::ffi::c_int,
        b"void container_fullscreen_disable(struct sway_container *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    set_fullscreen(con, r#false != 0);
    if container_is_floating(con) {
        (*con).pending.x = (*con).saved_x;
        (*con).pending.y = (*con).saved_y;
        (*con).pending.width = (*con).saved_width;
        (*con).pending.height = (*con).saved_height;
    }
    if (*con).pending.fullscreen_mode.0 == sway_fullscreen_mode::FULLSCREEN_WORKSPACE.0 {
        if !(*con).pending.workspace.is_null() {
            (*(*con).pending.workspace).fullscreen = ::core::ptr::null_mut::<
                sway_container,
            >();
            if container_is_floating(con) {
                let mut output: *mut sway_output = container_floating_find_output(con);
                if (*(*con).pending.workspace).output != output {
                    container_floating_move_to_center(con);
                }
            }
        }
    } else {
        (*root).fullscreen_global = ::core::ptr::null_mut::<sway_container>();
    }
    if container_is_floating(con) as ::core::ffi::c_int != 0
        && ((*con).pending.width == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || (*con).pending.height == 0 as ::core::ffi::c_int as ::core::ffi::c_double)
    {
        container_floating_resize_and_center(con);
    }
    (*con).pending.fullscreen_mode = sway_fullscreen_mode::FULLSCREEN_NONE;
    container_end_mouse_operation(con);
    ipc_event_window(con, b"fullscreen_mode\0".as_ptr() as *const ::core::ffi::c_char);
    if (*con).scratchpad {
        let mut seat: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
        seat = ((*server.input).seats.next as *mut ::core::ffi::c_char)
            .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
            as *mut sway_seat;
        while &raw mut (*seat).link != &raw mut (*server.input).seats {
            let mut focus: *mut sway_container = seat_get_focused_container(seat);
            if focus == con
                || container_has_ancestor(focus, con) as ::core::ffi::c_int != 0
            {
                seat_set_focus(
                    seat,
                    seat_get_focus_inactive(seat, &raw mut (*root).node),
                );
            }
            seat = ((*seat).link.next as *mut ::core::ffi::c_char)
                .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
                as *mut sway_seat;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn container_set_fullscreen(
    mut con: *mut sway_container,
    mut mode: sway_fullscreen_mode,
) {
    if (*con).pending.fullscreen_mode.0 == mode.0 {
        return;
    }
    match mode {
        sway_fullscreen_mode::FULLSCREEN_NONE => {
            container_fullscreen_disable(con);
        }
        sway_fullscreen_mode::FULLSCREEN_WORKSPACE => {
            if !(*root).fullscreen_global.is_null() {
                container_fullscreen_disable((*root).fullscreen_global);
            }
            if !(*con).pending.workspace.is_null()
                && !(*(*con).pending.workspace).fullscreen.is_null()
            {
                container_fullscreen_disable((*(*con).pending.workspace).fullscreen);
            }
            container_fullscreen_workspace(con);
        }
        sway_fullscreen_mode::FULLSCREEN_GLOBAL => {
            if !(*root).fullscreen_global.is_null() {
                container_fullscreen_disable((*root).fullscreen_global);
            }
            if (*con).pending.fullscreen_mode.0
                == sway_fullscreen_mode::FULLSCREEN_WORKSPACE.0
            {
                container_fullscreen_disable(con);
            }
            container_fullscreen_global(con);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn container_toplevel_ancestor(
    mut container: *mut sway_container,
) -> *mut sway_container {
    while !(*container).pending.parent.is_null() {
        container = (*container).pending.parent;
    }
    return container;
}
#[no_mangle]
pub unsafe extern "C" fn container_is_floating_or_child(
    mut container: *mut sway_container,
) -> bool {
    return container_is_floating(container_toplevel_ancestor(container));
}
#[no_mangle]
pub unsafe extern "C" fn container_is_fullscreen_or_child(
    mut container: *mut sway_container,
) -> bool {
    loop {
        if (*container).pending.fullscreen_mode.0 != 0 {
            return r#true != 0;
        }
        container = (*container).pending.parent;
        if container.is_null() {
            break;
        }
    }
    return r#false != 0;
}
#[no_mangle]
pub unsafe extern "C" fn container_parent_layout(
    mut con: *mut sway_container,
) -> sway_container_layout {
    if !(*con).pending.parent.is_null() {
        return (*(*con).pending.parent).pending.layout;
    }
    if !(*con).pending.workspace.is_null() {
        return (*(*con).pending.workspace).layout;
    }
    return sway_container_layout(
        sway_container_layout_0::L_NONE.0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn container_get_siblings(
    mut container: *mut sway_container,
) -> *mut list_t {
    if !(*container).pending.parent.is_null() {
        return (*(*container).pending.parent).pending.children;
    }
    if (*container).pending.workspace.is_null() {
        return ::core::ptr::null_mut::<list_t>();
    }
    if list_find(
        (*(*container).pending.workspace).tiling,
        container as *const ::core::ffi::c_void,
    ) != -1 as ::core::ffi::c_int
    {
        return (*(*container).pending.workspace).tiling;
    }
    return (*(*container).pending.workspace).floating;
}
#[no_mangle]
pub unsafe extern "C" fn container_sibling_index(
    mut child: *mut sway_container,
) -> ::core::ffi::c_int {
    return list_find(container_get_siblings(child), child as *const ::core::ffi::c_void);
}
unsafe extern "C" fn find_fullscreen_workspace(
    mut con: *mut sway_container,
    mut data: *mut ::core::ffi::c_void,
) -> bool {
    return (*con).pending.fullscreen_mode.0
        == sway_fullscreen_mode::FULLSCREEN_WORKSPACE.0;
}
unsafe extern "C" fn find_fullscreen_global(
    mut con: *mut sway_container,
    mut data: *mut ::core::ffi::c_void,
) -> bool {
    return (*con).pending.fullscreen_mode.0 == sway_fullscreen_mode::FULLSCREEN_GLOBAL.0;
}
#[no_mangle]
pub unsafe extern "C" fn container_handle_fullscreen_reparent(
    mut con: *mut sway_container,
) {
    let mut fs: *mut sway_container = ::core::ptr::null_mut::<sway_container>();
    if (*con).pending.fullscreen_mode.0 == sway_fullscreen_mode::FULLSCREEN_GLOBAL.0 {
        fs = con;
    } else {
        fs = container_find_child(
            con,
            Some(
                find_fullscreen_global
                    as unsafe extern "C" fn(
                        *mut sway_container,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
            ),
            NULL,
        );
    }
    if !fs.is_null() {
        if (*root).fullscreen_global == fs {
            return;
        }
        if !(*root).fullscreen_global.is_null() {
            container_fullscreen_disable((*root).fullscreen_global);
        }
        (*root).fullscreen_global = fs;
        arrange_root();
        return;
    }
    if (*con).pending.fullscreen_mode.0 == sway_fullscreen_mode::FULLSCREEN_WORKSPACE.0 {
        fs = con;
    } else {
        fs = container_find_child(
            con,
            Some(
                find_fullscreen_workspace
                    as unsafe extern "C" fn(
                        *mut sway_container,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
            ),
            NULL,
        );
    }
    if fs.is_null() || (*fs).pending.workspace.is_null()
        || (*(*fs).pending.workspace).fullscreen == fs
    {
        return;
    }
    if !(*(*fs).pending.workspace).fullscreen.is_null() {
        container_fullscreen_disable((*(*fs).pending.workspace).fullscreen);
    }
    (*(*fs).pending.workspace).fullscreen = fs;
    arrange_workspace((*fs).pending.workspace);
}
unsafe extern "C" fn set_workspace(
    mut container: *mut sway_container,
    mut data: *mut ::core::ffi::c_void,
) {
    (*container).pending.workspace = (*(*container).pending.parent).pending.workspace;
}
#[no_mangle]
pub unsafe extern "C" fn container_insert_child(
    mut parent: *mut sway_container,
    mut child: *mut sway_container,
    mut i: ::core::ffi::c_int,
) {
    if !(*child).pending.workspace.is_null() {
        container_detach(child);
    }
    list_insert((*parent).pending.children, i, child as *mut ::core::ffi::c_void);
    (*child).pending.parent = parent;
    (*child).pending.workspace = (*parent).pending.workspace;
    container_for_each_child(
        child,
        Some(
            set_workspace
                as unsafe extern "C" fn(
                    *mut sway_container,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
    container_handle_fullscreen_reparent(child);
    container_update_representation(parent);
}
#[no_mangle]
pub unsafe extern "C" fn container_add_sibling(
    mut fixed: *mut sway_container,
    mut active: *mut sway_container,
    mut after: bool,
) {
    if !(*active).pending.workspace.is_null() {
        container_detach(active);
    }
    let mut siblings: *mut list_t = container_get_siblings(fixed);
    let mut index: ::core::ffi::c_int = list_find(
        siblings,
        fixed as *const ::core::ffi::c_void,
    );
    list_insert(
        siblings,
        index + after as ::core::ffi::c_int,
        active as *mut ::core::ffi::c_void,
    );
    (*active).pending.parent = (*fixed).pending.parent;
    (*active).pending.workspace = (*fixed).pending.workspace;
    container_for_each_child(
        active,
        Some(
            set_workspace
                as unsafe extern "C" fn(
                    *mut sway_container,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
    container_handle_fullscreen_reparent(active);
    container_update_representation(active);
}
#[no_mangle]
pub unsafe extern "C" fn container_add_child(
    mut parent: *mut sway_container,
    mut child: *mut sway_container,
) {
    if !(*child).pending.workspace.is_null() {
        container_detach(child);
    }
    list_add((*parent).pending.children, child as *mut ::core::ffi::c_void);
    (*child).pending.parent = parent;
    (*child).pending.workspace = (*parent).pending.workspace;
    container_for_each_child(
        child,
        Some(
            set_workspace
                as unsafe extern "C" fn(
                    *mut sway_container,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
    container_handle_fullscreen_reparent(child);
    container_update_representation(parent);
    node_set_dirty(&raw mut (*child).node);
    node_set_dirty(&raw mut (*parent).node);
}
#[no_mangle]
pub unsafe extern "C" fn container_detach(mut child: *mut sway_container) {
    let mut old_workspace: *mut sway_workspace = (*child).pending.workspace;
    if !old_workspace.is_null() && !(*old_workspace).fullscreen.is_null()
        && ((*old_workspace).fullscreen == child
            || container_has_ancestor((*old_workspace).fullscreen, child)
                as ::core::ffi::c_int != 0)
    {
        (*old_workspace).fullscreen = ::core::ptr::null_mut::<sway_container>();
    }
    if !(*root).fullscreen_global.is_null()
        && ((*root).fullscreen_global == child
            || container_has_ancestor((*root).fullscreen_global, child)
                as ::core::ffi::c_int != 0)
    {
        (*root).fullscreen_global = ::core::ptr::null_mut::<sway_container>();
    }
    let mut old_parent: *mut sway_container = (*child).pending.parent;
    let mut siblings: *mut list_t = container_get_siblings(child);
    if !siblings.is_null() {
        let mut index: ::core::ffi::c_int = list_find(
            siblings,
            child as *const ::core::ffi::c_void,
        );
        if index != -1 as ::core::ffi::c_int {
            list_del(siblings, index);
        }
    }
    (*child).pending.parent = ::core::ptr::null_mut::<sway_container>();
    (*child).pending.workspace = ::core::ptr::null_mut::<sway_workspace>();
    container_for_each_child(
        child,
        Some(
            set_workspace
                as unsafe extern "C" fn(
                    *mut sway_container,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
    if !old_parent.is_null() {
        container_update_representation(old_parent);
        node_set_dirty(&raw mut (*old_parent).node);
    } else if !old_workspace.is_null() {
        workspace_update_representation(old_workspace);
        node_set_dirty(&raw mut (*old_workspace).node);
    }
    node_set_dirty(&raw mut (*child).node);
}
#[no_mangle]
pub unsafe extern "C" fn container_replace(
    mut container: *mut sway_container,
    mut replacement: *mut sway_container,
) {
    let mut fullscreen: sway_fullscreen_mode = (*container).pending.fullscreen_mode;
    let mut scratchpad: bool = (*container).scratchpad;
    let mut ws: *mut sway_workspace = ::core::ptr::null_mut::<sway_workspace>();
    if fullscreen.0 != sway_fullscreen_mode::FULLSCREEN_NONE.0 {
        container_fullscreen_disable(container);
    }
    if scratchpad {
        ws = (*container).pending.workspace;
        root_scratchpad_show(container);
        root_scratchpad_remove_container(container);
    }
    if !(*container).pending.parent.is_null()
        || !(*container).pending.workspace.is_null()
    {
        let mut width_fraction: ::core::ffi::c_float = (*container).width_fraction
            as ::core::ffi::c_float;
        let mut height_fraction: ::core::ffi::c_float = (*container).height_fraction
            as ::core::ffi::c_float;
        container_add_sibling(container, replacement, true);
        container_detach(container);
        (*replacement).width_fraction = width_fraction as ::core::ffi::c_double;
        (*replacement).height_fraction = height_fraction as ::core::ffi::c_double;
    }
    if scratchpad {
        root_scratchpad_add_container(replacement, ws);
    }
    match fullscreen {
        sway_fullscreen_mode::FULLSCREEN_WORKSPACE => {
            container_fullscreen_workspace(replacement);
        }
        sway_fullscreen_mode::FULLSCREEN_GLOBAL => {
            container_fullscreen_global(replacement);
        }
        sway_fullscreen_mode::FULLSCREEN_NONE | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn container_split(
    mut child: *mut sway_container,
    mut layout: sway_container_layout,
) -> *mut sway_container {
    if !(*child).pending.parent.is_null() || !(*child).pending.workspace.is_null() {
        let mut siblings: *mut list_t = container_get_siblings(child);
        if (*siblings).length == 1 as ::core::ffi::c_int {
            let mut current: sway_container_layout = container_parent_layout(child);
            if container_is_floating(child) {
                current = sway_container_layout(
                    sway_container_layout_0::L_NONE.0 as ::core::ffi::c_int
                        as ::core::ffi::c_uint,
                );
            }
            if current.0 == sway_container_layout_0::L_HORIZ.0
                || current.0 == sway_container_layout_0::L_VERT.0
            {
                if !(*child).pending.parent.is_null() {
                    (*(*child).pending.parent).pending.layout = layout;
                    container_update_representation((*child).pending.parent);
                } else {
                    (*(*child).pending.workspace).layout = layout;
                    workspace_update_representation((*child).pending.workspace);
                }
                return child;
            }
        }
    }
    let mut seat: *mut sway_seat = input_manager_get_default_seat();
    let mut set_focus: bool = seat_get_focus(seat) == &raw mut (*child).node;
    if container_is_floating(child) as ::core::ffi::c_int != 0
        && !(*child).view.is_null()
    {
        view_set_tiled((*child).view, r#true != 0);
        if (*(*child).view).using_csd {
            (*child).pending.border = (*child).saved_border;
        }
    }
    let mut cont: *mut sway_container = container_create(
        ::core::ptr::null_mut::<sway_view>(),
    );
    (*cont).pending.width = (*child).pending.width;
    (*cont).pending.height = (*child).pending.height;
    (*cont).width_fraction = (*child).width_fraction;
    (*cont).height_fraction = (*child).height_fraction;
    (*cont).pending.x = (*child).pending.x;
    (*cont).pending.y = (*child).pending.y;
    (*cont).pending.layout = layout;
    container_replace(child, cont);
    container_add_child(cont, child);
    if set_focus {
        seat_set_raw_focus(seat, &raw mut (*cont).node);
        if (*cont).pending.fullscreen_mode.0 == sway_fullscreen_mode::FULLSCREEN_GLOBAL.0
        {
            seat_set_focus(seat, &raw mut (*child).node);
        } else {
            seat_set_raw_focus(seat, &raw mut (*child).node);
        }
    }
    return cont;
}
#[no_mangle]
pub unsafe extern "C" fn container_is_transient_for(
    mut child: *mut sway_container,
    mut ancestor: *mut sway_container,
) -> bool {
    return (*config).popup_during_fullscreen.0
        == sway_popup_during_fullscreen::POPUP_SMART.0 && !(*child).view.is_null()
        && !(*ancestor).view.is_null()
        && view_is_transient_for((*child).view, (*ancestor).view) as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn find_by_mark_iterator(
    mut con: *mut sway_container,
    mut data: *mut ::core::ffi::c_void,
) -> bool {
    let mut mark: *mut ::core::ffi::c_char = data as *mut ::core::ffi::c_char;
    return container_has_mark(con, mark);
}
#[no_mangle]
pub unsafe extern "C" fn container_find_mark(
    mut mark: *mut ::core::ffi::c_char,
) -> *mut sway_container {
    return root_find_container(
        Some(
            find_by_mark_iterator
                as unsafe extern "C" fn(
                    *mut sway_container,
                    *mut ::core::ffi::c_void,
                ) -> bool,
        ),
        mark as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn container_find_and_unmark(
    mut mark: *mut ::core::ffi::c_char,
) -> bool {
    let mut con: *mut sway_container = root_find_container(
        Some(
            find_by_mark_iterator
                as unsafe extern "C" fn(
                    *mut sway_container,
                    *mut ::core::ffi::c_void,
                ) -> bool,
        ),
        mark as *mut ::core::ffi::c_void,
    );
    if con.is_null() {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*con).marks).length {
        let mut con_mark: *mut ::core::ffi::c_char = *(*(*con).marks)
            .items
            .offset(i as isize) as *mut ::core::ffi::c_char;
        if strcmp(con_mark, mark) == 0 as ::core::ffi::c_int {
            free(con_mark as *mut ::core::ffi::c_void);
            list_del((*con).marks, i);
            container_update_marks(con);
            ipc_event_window(con, b"mark\0".as_ptr() as *const ::core::ffi::c_char);
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
#[no_mangle]
pub unsafe extern "C" fn container_clear_marks(mut con: *mut sway_container) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*con).marks).length {
        free(*(*(*con).marks).items.offset(i as isize));
        i += 1;
    }
    (*(*con).marks).length = 0 as ::core::ffi::c_int;
    ipc_event_window(con, b"mark\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn container_has_mark(
    mut con: *mut sway_container,
    mut mark: *mut ::core::ffi::c_char,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*con).marks).length {
        let mut item: *mut ::core::ffi::c_char = *(*(*con).marks)
            .items
            .offset(i as isize) as *mut ::core::ffi::c_char;
        if strcmp(item, mark) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
#[no_mangle]
pub unsafe extern "C" fn container_add_mark(
    mut con: *mut sway_container,
    mut mark: *mut ::core::ffi::c_char,
) {
    list_add((*con).marks, strdup(mark) as *mut ::core::ffi::c_void);
    ipc_event_window(con, b"mark\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn container_raise_floating(mut con: *mut sway_container) {
    let mut floater: *mut sway_container = container_toplevel_ancestor(con);
    if container_is_floating(floater) as ::core::ffi::c_int != 0
        && !(*floater).pending.workspace.is_null()
    {
        wlr_scene_node_raise_to_top(&raw mut (*(*floater).scene_tree).node);
        list_move_to_end(
            (*(*floater).pending.workspace).floating,
            floater as *mut ::core::ffi::c_void,
        );
        node_set_dirty(&raw mut (*(*floater).pending.workspace).node);
    }
}
#[no_mangle]
pub unsafe extern "C" fn container_is_scratchpad_hidden(
    mut con: *mut sway_container,
) -> bool {
    return (*con).scratchpad as ::core::ffi::c_int != 0
        && (*con).pending.workspace.is_null();
}
#[no_mangle]
pub unsafe extern "C" fn container_is_scratchpad_hidden_or_child(
    mut con: *mut sway_container,
) -> bool {
    con = container_toplevel_ancestor(con);
    return (*con).scratchpad as ::core::ffi::c_int != 0
        && (*con).pending.workspace.is_null();
}
#[no_mangle]
pub unsafe extern "C" fn container_is_sticky(mut con: *mut sway_container) -> bool {
    return (*con).is_sticky as ::core::ffi::c_int != 0
        && container_is_floating(con) as ::core::ffi::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn container_is_sticky_or_child(
    mut con: *mut sway_container,
) -> bool {
    return container_is_sticky(container_toplevel_ancestor(con));
}
unsafe extern "C" fn is_parallel(
    mut first: sway_container_layout,
    mut second: sway_container_layout,
) -> bool {
    match first {
        sway_container_layout(4) | sway_container_layout(1) => {
            return second.0 == sway_container_layout_0::L_TABBED.0
                || second.0 == sway_container_layout_0::L_HORIZ.0;
        }
        sway_container_layout(3) | sway_container_layout(2) => {
            return second.0 == sway_container_layout_0::L_STACKED.0
                || second.0 == sway_container_layout_0::L_VERT.0;
        }
        _ => return r#false != 0,
    };
}
unsafe extern "C" fn container_is_squashable(
    mut con: *mut sway_container,
    mut child: *mut sway_container,
) -> bool {
    let mut gp_layout: sway_container_layout = container_parent_layout(con);
    return ((*con).pending.layout.0 == sway_container_layout_0::L_HORIZ.0
        || (*con).pending.layout.0 == sway_container_layout_0::L_VERT.0)
        && ((*child).pending.layout.0 == sway_container_layout_0::L_HORIZ.0
            || (*child).pending.layout.0 == sway_container_layout_0::L_VERT.0)
        && !is_parallel((*con).pending.layout, (*child).pending.layout)
        && is_parallel(gp_layout, (*child).pending.layout) as ::core::ffi::c_int != 0;
}
unsafe extern "C" fn container_squash_children(mut con: *mut sway_container) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*con).pending.children).length {
        let mut child: *mut sway_container = *(*(*con).pending.children)
            .items
            .offset(i as isize) as *mut sway_container;
        i += container_squash(child);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn container_squash(
    mut con: *mut sway_container,
) -> ::core::ffi::c_int {
    if (*con).pending.children.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*(*con).pending.children).length != 1 as ::core::ffi::c_int {
        container_squash_children(con);
        return 0 as ::core::ffi::c_int;
    }
    let mut child: *mut sway_container = *(*(*con).pending.children).items.offset(0isize)
        as *mut sway_container;
    let mut idx: ::core::ffi::c_int = container_sibling_index(con);
    let mut change: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if container_is_squashable(con, child) {
        while (*(*child).pending.children).length != 0 {
            let mut current: *mut sway_container = *(*(*child).pending.children)
                .items
                .offset(0isize) as *mut sway_container;
            container_detach(current);
            if !(*con).pending.parent.is_null() {
                container_insert_child((*con).pending.parent, current, idx);
            } else {
                workspace_insert_tiling_direct((*con).pending.workspace, current, idx);
            }
            change += 1;
        }
        container_reap_empty(child);
        change -= 1;
    } else {
        container_squash_children(con);
    }
    return change;
}
unsafe extern "C" fn swap_places(
    mut con1: *mut sway_container,
    mut con2: *mut sway_container,
) {
    let mut temp: *mut sway_container = malloc(::core::mem::size_of::<sway_container>())
        as *mut sway_container;
    (*temp).pending.x = (*con1).pending.x;
    (*temp).pending.y = (*con1).pending.y;
    (*temp).pending.width = (*con1).pending.width;
    (*temp).pending.height = (*con1).pending.height;
    (*temp).width_fraction = (*con1).width_fraction;
    (*temp).height_fraction = (*con1).height_fraction;
    (*temp).pending.parent = (*con1).pending.parent;
    (*temp).pending.workspace = (*con1).pending.workspace;
    let mut temp_floating: bool = container_is_floating(con1);
    (*con1).pending.x = (*con2).pending.x;
    (*con1).pending.y = (*con2).pending.y;
    (*con1).pending.width = (*con2).pending.width;
    (*con1).pending.height = (*con2).pending.height;
    (*con1).width_fraction = (*con2).width_fraction;
    (*con1).height_fraction = (*con2).height_fraction;
    (*con2).pending.x = (*temp).pending.x;
    (*con2).pending.y = (*temp).pending.y;
    (*con2).pending.width = (*temp).pending.width;
    (*con2).pending.height = (*temp).pending.height;
    (*con2).width_fraction = (*temp).width_fraction;
    (*con2).height_fraction = (*temp).height_fraction;
    let mut temp_index: ::core::ffi::c_int = container_sibling_index(con1);
    if !(*con2).pending.parent.is_null() {
        container_insert_child(
            (*con2).pending.parent,
            con1,
            container_sibling_index(con2),
        );
    } else if container_is_floating(con2) {
        workspace_add_floating((*con2).pending.workspace, con1);
    } else {
        workspace_insert_tiling(
            (*con2).pending.workspace,
            con1,
            container_sibling_index(con2),
        );
    }
    if !(*temp).pending.parent.is_null() {
        container_insert_child((*temp).pending.parent, con2, temp_index);
    } else if temp_floating {
        workspace_add_floating((*temp).pending.workspace, con2);
    } else {
        workspace_insert_tiling((*temp).pending.workspace, con2, temp_index);
    }
    free(temp as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn swap_focus(
    mut con1: *mut sway_container,
    mut con2: *mut sway_container,
    mut seat: *mut sway_seat,
    mut focus: *mut sway_container,
) {
    if focus == con1 || focus == con2 {
        let mut ws1: *mut sway_workspace = (*con1).pending.workspace;
        let mut ws2: *mut sway_workspace = (*con2).pending.workspace;
        let mut layout1: sway_container_layout = container_parent_layout(con1);
        let mut layout2: sway_container_layout = container_parent_layout(con2);
        if focus == con1
            && (layout2.0 == sway_container_layout_0::L_TABBED.0
                || layout2.0 == sway_container_layout_0::L_STACKED.0)
        {
            if workspace_is_visible(ws2) {
                seat_set_focus(seat, &raw mut (*con2).node);
            }
            seat_set_focus_container(seat, if ws1 != ws2 { con2 } else { con1 });
        } else if focus == con2
            && (layout1.0 == sway_container_layout_0::L_TABBED.0
                || layout1.0 == sway_container_layout_0::L_STACKED.0)
        {
            if workspace_is_visible(ws1) {
                seat_set_focus(seat, &raw mut (*con1).node);
            }
            seat_set_focus_container(seat, if ws1 != ws2 { con1 } else { con2 });
        } else if ws1 != ws2 {
            seat_set_focus_container(seat, if focus == con1 { con2 } else { con1 });
        } else {
            seat_set_focus_container(seat, focus);
        }
    } else {
        seat_set_focus_container(seat, focus);
    }
    if !(*root).fullscreen_global.is_null() {
        seat_set_focus(
            seat,
            seat_get_focus_inactive(seat, &raw mut (*(*root).fullscreen_global).node),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn container_swap(
    mut con1: *mut sway_container,
    mut con2: *mut sway_container,
) {
    if !_sway_assert(
        !con1.is_null() && !con2.is_null(),
        b"[%s:%d] %s:Cannot swap with nothing\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        1858 as ::core::ffi::c_int,
        b"void container_swap(struct sway_container *, struct sway_container *)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if !_sway_assert(
        !container_has_ancestor(con1, con2) && !container_has_ancestor(con2, con1),
        b"[%s:%d] %s:Cannot swap ancestor and descendant\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        1861 as ::core::ffi::c_int,
        b"void container_swap(struct sway_container *, struct sway_container *)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Swapping containers %zu and %zu\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        1867 as ::core::ffi::c_int,
        (*con1).node.id,
        (*con2).node.id,
    );
    let mut scratch1: bool = (*con1).scratchpad;
    let mut hidden1: bool = container_is_scratchpad_hidden(con1);
    let mut scratch2: bool = (*con2).scratchpad;
    let mut hidden2: bool = container_is_scratchpad_hidden(con2);
    if scratch1 {
        if hidden1 {
            root_scratchpad_show(con1);
        }
        root_scratchpad_remove_container(con1);
    }
    if scratch2 {
        if hidden2 {
            root_scratchpad_show(con2);
        }
        root_scratchpad_remove_container(con2);
    }
    let mut fs1: sway_fullscreen_mode = (*con1).pending.fullscreen_mode;
    if fs1.0 != 0 {
        container_fullscreen_disable(con1);
    }
    let mut fs2: sway_fullscreen_mode = (*con2).pending.fullscreen_mode;
    if fs2.0 != 0 {
        container_fullscreen_disable(con2);
    }
    let mut seat: *mut sway_seat = input_manager_current_seat();
    let mut focus: *mut sway_container = seat_get_focused_container(seat);
    let mut vis1: *mut sway_workspace = output_get_active_workspace(
        (*(*con1).pending.workspace).output,
    );
    let mut vis2: *mut sway_workspace = output_get_active_workspace(
        (*(*con2).pending.workspace).output,
    );
    if !_sway_assert(
        !vis1.is_null() && !vis2.is_null(),
        b"[%s:%d] %s:con1 or con2 are on an output without aworkspace. This should not happen\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/container.c\0".as_ptr() as *const ::core::ffi::c_char,
        1902 as ::core::ffi::c_int,
        b"void container_swap(struct sway_container *, struct sway_container *)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    let mut stored_prev_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if !(*seat).prev_workspace_name.is_null() {
        stored_prev_name = strdup((*seat).prev_workspace_name);
    }
    swap_places(con1, con2);
    if !workspace_is_visible(vis1) {
        seat_set_focus(seat, seat_get_focus_inactive(seat, &raw mut (*vis1).node));
    }
    if !workspace_is_visible(vis2) {
        seat_set_focus(seat, seat_get_focus_inactive(seat, &raw mut (*vis2).node));
    }
    swap_focus(con1, con2, seat, focus);
    if !stored_prev_name.is_null() {
        free((*seat).prev_workspace_name as *mut ::core::ffi::c_void);
        (*seat).prev_workspace_name = stored_prev_name;
    }
    if scratch1 {
        root_scratchpad_add_container(con2, ::core::ptr::null_mut::<sway_workspace>());
        if !hidden1 {
            root_scratchpad_show(con2);
        }
    }
    if scratch2 {
        root_scratchpad_add_container(con1, ::core::ptr::null_mut::<sway_workspace>());
        if !hidden2 {
            root_scratchpad_show(con1);
        }
    }
    if fs1.0 != 0 {
        container_set_fullscreen(con2, fs1);
    }
    if fs2.0 != 0 {
        container_set_fullscreen(con1, fs2);
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __DBL_MAX__: ::core::ffi::c_double = 1.7976931348623157e+308f64;
