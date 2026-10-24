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
    pub type wlr_xwm;
    pub type wlr_xwayland_shell_v1;
    pub type pcre2_real_code_8;
    pub type wlr_linux_dmabuf_feedback_v1_compiled;
    pub type wlr_output_layer_feedback_event;
    pub type wlr_gamma_control_v1;
    pub type wlr_color_manager_v1;
    pub type wlr_gamma_control_manager_v1;
    pub type sway_transaction;
    pub type wlr_xdg_activation_v1;
    pub type wlr_ext_foreign_toplevel_image_capture_source_manager_v1;
    pub type wlr_export_dmabuf_manager_v1;
    pub type wlr_ext_image_copy_capture_manager_v1;
    pub type wlr_screencopy_manager_v1;
    pub type wlr_ext_data_control_manager_v1;
    pub type wlr_data_control_manager_v1;
    pub type wlr_content_type_manager_v1;
    pub type wlr_output_power_manager_v1;
    pub type wlr_output_manager_v1;
    pub type wlr_xdg_output_manager_v1;
    pub type wlr_drm_lease_v1_manager;
    pub type wlr_relative_pointer_manager_v1;
    pub type wlr_xcursor_manager;
    pub type wlr_tablet_manager_v2;
    pub type wlr_idle_notifier_v1;
    pub type wlr_data_device_manager;
    pub type wlr_session;
    pub type sway_transaction_instruction;
    pub type sway_text_node;
    pub type wlr_tablet_v2_tablet_tool;
    pub type wlr_tablet_v2_tablet;
    pub type wlr_tablet_v2_tablet_pad;
    pub type wlr_tablet_pad;
    pub type sway_switch;
    pub type sway_keyboard;
    pub type wlr_touch_impl;
    pub type wlr_cursor;
    pub type wlr_xdg_activation_token_v1;
    pub type wlr_ext_image_capture_source_v1;
    pub type _PangoFontDescription;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fmax(
        __x: ::core::ffi::c_double,
        __y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn wl_list_init(list: *mut wl_list);
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn wl_list_empty(list: *const wl_list) -> ::core::ffi::c_int;
    fn wl_event_source_remove(source: *mut wl_event_source) -> ::core::ffi::c_int;
    fn wl_client_get_credentials(
        client: *const wl_client,
        pid: *mut pid_t,
        uid: *mut uid_t,
        gid: *mut gid_t,
    );
    fn wl_signal_emit_mutable(signal: *mut wl_signal, data: *mut ::core::ffi::c_void);
    fn wl_resource_get_client(resource: *mut wl_resource) -> *mut wl_client;
    fn wlr_ext_foreign_toplevel_handle_v1_create(
        list: *mut wlr_ext_foreign_toplevel_list_v1,
        state: *const wlr_ext_foreign_toplevel_handle_v1_state,
    ) -> *mut wlr_ext_foreign_toplevel_handle_v1;
    fn wlr_ext_foreign_toplevel_handle_v1_destroy(
        toplevel: *mut wlr_ext_foreign_toplevel_handle_v1,
    );
    fn wlr_ext_foreign_toplevel_handle_v1_update_state(
        toplevel: *mut wlr_ext_foreign_toplevel_handle_v1,
        state: *const wlr_ext_foreign_toplevel_handle_v1_state,
    );
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn wlr_foreign_toplevel_handle_v1_create(
        manager: *mut wlr_foreign_toplevel_manager_v1,
    ) -> *mut wlr_foreign_toplevel_handle_v1;
    fn wlr_foreign_toplevel_handle_v1_destroy(
        toplevel: *mut wlr_foreign_toplevel_handle_v1,
    );
    fn wlr_foreign_toplevel_handle_v1_set_title(
        toplevel: *mut wlr_foreign_toplevel_handle_v1,
        title: *const ::core::ffi::c_char,
    );
    fn wlr_foreign_toplevel_handle_v1_set_app_id(
        toplevel: *mut wlr_foreign_toplevel_handle_v1,
        app_id: *const ::core::ffi::c_char,
    );
    fn wlr_foreign_toplevel_handle_v1_output_enter(
        toplevel: *mut wlr_foreign_toplevel_handle_v1,
        output: *mut wlr_output,
    );
    fn wlr_foreign_toplevel_handle_v1_output_leave(
        toplevel: *mut wlr_foreign_toplevel_handle_v1,
        output: *mut wlr_output,
    );
    fn wlr_foreign_toplevel_handle_v1_set_activated(
        toplevel: *mut wlr_foreign_toplevel_handle_v1,
        activated: bool,
    );
    fn wlr_surface_send_frame_done(surface: *mut wlr_surface, when: *const timespec);
    fn wlr_surface_set_preferred_buffer_scale(surface: *mut wlr_surface, scale: int32_t);
    fn wlr_subsurface_try_from_wlr_surface(
        surface: *mut wlr_surface,
    ) -> *mut wlr_subsurface;
    fn wlr_fractional_scale_v1_notify_scale(
        surface: *mut wlr_surface,
        scale: ::core::ffi::c_double,
    );
    fn wlr_security_context_manager_v1_lookup_client(
        manager: *mut wlr_security_context_manager_v1,
        client: *const wl_client,
    ) -> *const wlr_security_context_v1_state;
    fn wlr_xdg_toplevel_decoration_v1_set_mode(
        decoration: *mut wlr_xdg_toplevel_decoration_v1,
        mode: wlr_xdg_toplevel_decoration_v1_mode,
    ) -> uint32_t;
    fn wlr_xdg_surface_try_from_wlr_surface(
        surface: *mut wlr_surface,
    ) -> *mut wlr_xdg_surface;
    fn create_list() -> *mut list_t;
    fn list_free(list: *mut list_t);
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn list_del(list: *mut list_t, index: ::core::ffi::c_int);
    fn wlr_xwayland_surface_try_from_wlr_surface(
        surface: *mut wlr_surface,
    ) -> *mut wlr_xwayland_surface;
    fn wlr_xwayland_surface_icccm_input_model(
        xsurface: *const wlr_xwayland_surface,
    ) -> wlr_xwayland_icccm_input_model;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    fn wlr_scene_node_destroy(node: *mut wlr_scene_node);
    fn wlr_scene_node_set_enabled(node: *mut wlr_scene_node, enabled: bool);
    fn wlr_scene_node_set_position(
        node: *mut wlr_scene_node,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn wlr_scene_node_place_below(
        node: *mut wlr_scene_node,
        sibling: *mut wlr_scene_node,
    );
    fn wlr_scene_node_for_each_buffer(
        node: *mut wlr_scene_node,
        iterator: wlr_scene_buffer_iterator_func_t,
        user_data: *mut ::core::ffi::c_void,
    );
    fn wlr_scene_create() -> *mut wlr_scene;
    fn wlr_scene_tree_create(parent: *mut wlr_scene_tree) -> *mut wlr_scene_tree;
    fn wlr_scene_surface_try_from_buffer(
        scene_buffer: *mut wlr_scene_buffer,
    ) -> *mut wlr_scene_surface;
    fn wlr_scene_buffer_create(
        parent: *mut wlr_scene_tree,
        buffer: *mut wlr_buffer,
    ) -> *mut wlr_scene_buffer;
    fn wlr_scene_buffer_set_buffer(
        scene_buffer: *mut wlr_scene_buffer,
        buffer: *mut wlr_buffer,
    );
    fn wlr_scene_buffer_set_opaque_region(
        scene_buffer: *mut wlr_scene_buffer,
        region: *const pixman_region32_t,
    );
    fn wlr_scene_buffer_set_source_box(
        scene_buffer: *mut wlr_scene_buffer,
        r#box: *const wlr_fbox,
    );
    fn wlr_scene_buffer_set_dest_size(
        scene_buffer: *mut wlr_scene_buffer,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn wlr_scene_buffer_set_transform(
        scene_buffer: *mut wlr_scene_buffer,
        transform: wl_output_transform,
    );
    fn wlr_scene_buffer_set_opacity(
        scene_buffer: *mut wlr_scene_buffer,
        opacity: ::core::ffi::c_float,
    );
    fn wlr_scene_buffer_set_filter_mode(
        scene_buffer: *mut wlr_scene_buffer,
        filter_mode: wlr_scale_filter_mode,
    );
    fn wlr_scene_buffer_set_transfer_function(
        scene_buffer: *mut wlr_scene_buffer,
        transfer_function: wlr_color_transfer_function,
    );
    fn wlr_scene_buffer_set_primaries(
        scene_buffer: *mut wlr_scene_buffer,
        primaries: wlr_color_named_primaries,
    );
    fn wlr_scene_subsurface_tree_set_clip(
        node: *mut wlr_scene_node,
        clip: *const wlr_box,
    );
    fn wlr_layer_surface_v1_try_from_wlr_surface(
        surface: *mut wlr_surface,
    ) -> *mut wlr_layer_surface_v1;
    static mut root: *mut sway_root;
    fn root_scratchpad_show(con: *mut sway_container);
    fn output_by_name_or_id(name_or_id: *const ::core::ffi::c_char) -> *mut sway_output;
    fn output_get_active_workspace(output: *mut sway_output) -> *mut sway_workspace;
    fn container_create(view: *mut sway_view) -> *mut sway_container;
    fn container_begin_destroy(con: *mut sway_container);
    fn container_reap_empty(con: *mut sway_container);
    fn parse_title_format(
        container: *mut sway_container,
        buffer: *mut ::core::ffi::c_char,
    ) -> size_t;
    fn container_update_representation(container: *mut sway_container);
    fn container_titlebar_height() -> size_t;
    fn container_set_floating(container: *mut sway_container, enable: bool);
    fn container_set_geometry_from_content(con: *mut sway_container);
    fn container_is_floating(container: *mut sway_container) -> bool;
    fn container_set_fullscreen(con: *mut sway_container, mode: sway_fullscreen_mode);
    fn container_is_floating_or_child(container: *mut sway_container) -> bool;
    fn container_is_fullscreen_or_child(container: *mut sway_container) -> bool;
    fn container_parent_layout(con: *mut sway_container) -> sway_container_layout;
    fn container_get_siblings(container: *mut sway_container) -> *mut list_t;
    fn container_add_sibling(
        parent: *mut sway_container,
        child: *mut sway_container,
        after: bool,
    );
    fn container_is_transient_for(
        child: *mut sway_container,
        ancestor: *mut sway_container,
    ) -> bool;
    fn container_raise_floating(con: *mut sway_container);
    fn container_is_scratchpad_hidden(con: *mut sway_container) -> bool;
    fn container_is_scratchpad_hidden_or_child(con: *mut sway_container) -> bool;
    fn container_is_sticky_or_child(con: *mut sway_container) -> bool;
    fn container_update_itself_and_parents(con: *mut sway_container);
    fn workspace_create(
        output: *mut sway_output,
        name: *const ::core::ffi::c_char,
    ) -> *mut sway_workspace;
    fn workspace_consider_destroy(ws: *mut sway_workspace);
    fn workspace_by_number(name: *const ::core::ffi::c_char) -> *mut sway_workspace;
    fn workspace_by_name(_: *const ::core::ffi::c_char) -> *mut sway_workspace;
    fn workspace_is_visible(ws: *mut sway_workspace) -> bool;
    fn workspace_detect_urgent(workspace: *mut sway_workspace);
    fn workspace_add_tiling(
        workspace: *mut sway_workspace,
        con: *mut sway_container,
    ) -> *mut sway_container;
    fn workspace_add_floating(workspace: *mut sway_workspace, con: *mut sway_container);
    fn node_set_dirty(node: *mut sway_node);
    fn alloc_scene_tree(
        parent: *mut wlr_scene_tree,
        failed: *mut bool,
    ) -> *mut wlr_scene_tree;
    fn criteria_for_view(view: *mut sway_view, types: criteria_type) -> *mut list_t;
    fn execute_command(
        command: *mut ::core::ffi::c_char,
        seat: *mut sway_seat,
        con: *mut sway_container,
    ) -> *mut list_t;
    fn free_cmd_results(results: *mut cmd_results);
    fn transaction_commit_dirty();
    fn seat_set_focus_container(seat: *mut sway_seat, con: *mut sway_container);
    fn seat_get_focused_workspace(seat: *mut sway_seat) -> *mut sway_workspace;
    fn seat_get_last_known_workspace(seat: *mut sway_seat) -> *mut sway_workspace;
    fn seat_get_focused_container(seat: *mut sway_seat) -> *mut sway_container;
    fn seat_get_focus_inactive(
        seat: *mut sway_seat,
        node: *mut sway_node,
    ) -> *mut sway_node;
    fn seat_get_focus_inactive_tiling(
        seat: *mut sway_seat,
        workspace: *mut sway_workspace,
    ) -> *mut sway_container;
    fn seat_get_focus_inactive_view(
        seat: *mut sway_seat,
        ancestor: *mut sway_node,
    ) -> *mut sway_container;
    fn seat_get_active_tiling_child(
        seat: *mut sway_seat,
        parent: *mut sway_node,
    ) -> *mut sway_node;
    fn seat_consider_warp_to_focus(seat: *mut sway_seat);
    fn sway_idle_inhibit_v1_is_active(inhibitor: *mut sway_idle_inhibitor_v1) -> bool;
    fn sway_idle_inhibit_v1_user_inhibitor_for_view(
        view: *mut sway_view,
    ) -> *mut sway_idle_inhibitor_v1;
    fn sway_idle_inhibit_v1_application_inhibitor_for_view(
        view: *mut sway_view,
    ) -> *mut sway_idle_inhibitor_v1;
    fn wlr_session_lock_surface_v1_try_from_wlr_surface(
        surface: *mut wlr_surface,
    ) -> *mut wlr_session_lock_surface_v1;
    fn view_from_wlr_xdg_surface(xdg_surface: *mut wlr_xdg_surface) -> *mut sway_view;
    fn view_from_wlr_xwayland_surface(
        xsurface: *mut wlr_xwayland_surface,
    ) -> *mut sway_view;
    static mut config: *mut sway_config;
    fn sway_cursor_constrain(
        cursor: *mut sway_cursor,
        constraint: *mut wlr_pointer_constraint_v1,
    );
    fn ipc_event_window(window: *mut sway_container, change: *const ::core::ffi::c_char);
    fn arrange_container(container: *mut sway_container);
    fn arrange_workspace(workspace: *mut sway_workspace);
    fn arrange_root();
    fn scene_descriptor_assign(
        node: *mut wlr_scene_node,
        r#type: sway_scene_descriptor_type,
        data: *mut ::core::ffi::c_void,
    ) -> bool;
    static mut server: sway_server;
    fn launcher_ctx_find_pid(pid: pid_t) -> *mut launcher_ctx;
    fn launcher_ctx_get_workspace(ctx: *mut launcher_ctx) -> *mut sway_workspace;
    fn launcher_ctx_consume(ctx: *mut launcher_ctx);
    fn launcher_ctx_destroy(ctx: *mut launcher_ctx);
    fn input_manager_set_focus(node: *mut sway_node);
    fn input_manager_current_seat() -> *mut sway_seat;
}
pub type size_t = usize;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type clockid_t = __clockid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_foreign_toplevel_list_v1 {
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub toplevels: wl_list,
    pub events: C2Rust_Unnamed_10,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
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
    pub events: C2Rust_Unnamed_11,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_foreign_toplevel_handle_v1_state {
    pub title: *const ::core::ffi::c_char,
    pub app_id: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
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
    pub events: C2Rust_Unnamed_15,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
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
    pub mastering_luminance: C2Rust_Unnamed_14,
    pub max_cll: ::core::ffi::c_double,
    pub max_fall: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub min: ::core::ffi::c_double,
    pub max: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
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
    pub custom_mode: C2Rust_Unnamed_16,
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
pub struct C2Rust_Unnamed_16 {
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
    pub events: C2Rust_Unnamed_18,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_foreign_toplevel_handle_v1_output {
    pub link: wl_list,
    pub output: *mut wlr_output,
    pub toplevel: *mut wlr_foreign_toplevel_handle_v1,
    pub WLR_PRIVATE: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub output_bind: wl_listener,
    pub output_destroy: wl_listener,
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
    pub events: C2Rust_Unnamed_20,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
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
pub struct wlr_foreign_toplevel_handle_v1_activated_event {
    pub toplevel: *mut wlr_foreign_toplevel_handle_v1,
    pub seat: *mut wlr_seat,
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
    pub events: C2Rust_Unnamed_22,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
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
    pub events: C2Rust_Unnamed_24,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
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
    pub events: C2Rust_Unnamed_26,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
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
pub struct C2Rust_Unnamed_26 {
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
    pub events: C2Rust_Unnamed_29,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_28,
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
pub struct C2Rust_Unnamed_28 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
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
    pub viewport: C2Rust_Unnamed_30,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
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
    pub events: C2Rust_Unnamed_32,
    pub WLR_PRIVATE: C2Rust_Unnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
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
    pub repeat_info: C2Rust_Unnamed_36,
    pub events: C2Rust_Unnamed_35,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
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
    pub events: C2Rust_Unnamed_37,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_37 {
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
    pub events: C2Rust_Unnamed_39,
    pub WLR_PRIVATE: C2Rust_Unnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
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
pub struct wlr_foreign_toplevel_handle_v1_fullscreen_event {
    pub toplevel: *mut wlr_foreign_toplevel_handle_v1,
    pub fullscreen: bool,
    pub output: *mut wlr_output,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_layout {
    pub outputs: wl_list,
    pub display: *mut wl_display,
    pub events: C2Rust_Unnamed_41,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_40 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_41 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_security_context_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_43,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_42,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
    pub contexts: wl_list,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
    pub destroy: wl_signal,
    pub commit: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_security_context_v1_state {
    pub sandbox_engine: *mut ::core::ffi::c_char,
    pub app_id: *mut ::core::ffi::c_char,
    pub instance_id: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_server_decoration_manager {
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub decorations: wl_list,
    pub default_mode: uint32_t,
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
    pub new_decoration: wl_signal,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_subsurface_parent_state {
    pub x: int32_t,
    pub y: int32_t,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_46,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
    pub synced: *mut wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_subsurface {
    pub resource: *mut wl_resource,
    pub surface: *mut wlr_surface,
    pub parent: *mut wlr_surface,
    pub current: wlr_subsurface_parent_state,
    pub pending: wlr_subsurface_parent_state,
    pub cached_seq: uint32_t,
    pub has_cache: bool,
    pub synchronized: bool,
    pub added: bool,
    pub events: C2Rust_Unnamed_48,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_47,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
    pub parent_synced: wlr_surface_synced,
    pub surface_client_commit: wl_listener,
    pub parent_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
    pub destroy: wl_signal,
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
    pub events: C2Rust_Unnamed_49,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
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
pub struct wlr_xdg_shell {
    pub global: *mut wl_global,
    pub version: uint32_t,
    pub clients: wl_list,
    pub popup_grabs: wl_list,
    pub ping_timeout: uint32_t,
    pub events: C2Rust_Unnamed_51,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_50,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
    pub new_surface: wl_signal,
    pub new_toplevel: wl_signal,
    pub new_popup: wl_signal,
    pub destroy: wl_signal,
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
pub struct wlr_xdg_positioner_rules {
    pub anchor_rect: wlr_box,
    pub anchor: xdg_positioner_anchor,
    pub gravity: xdg_positioner_gravity,
    pub constraint_adjustment: xdg_positioner_constraint_adjustment,
    pub reactive: bool,
    pub has_parent_configure_serial: bool,
    pub parent_configure_serial: uint32_t,
    pub size: C2Rust_Unnamed_53,
    pub parent_size: C2Rust_Unnamed_53,
    pub offset: C2Rust_Unnamed_52,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_52 {
    pub x: int32_t,
    pub y: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_53 {
    pub width: int32_t,
    pub height: int32_t,
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
pub struct wlr_xdg_popup {
    pub base: *mut wlr_xdg_surface,
    pub link: wl_list,
    pub resource: *mut wl_resource,
    pub parent: *mut wlr_surface,
    pub seat: *mut wlr_seat,
    pub scheduled: wlr_xdg_popup_configure,
    pub current: wlr_xdg_popup_state,
    pub pending: wlr_xdg_popup_state,
    pub events: C2Rust_Unnamed_55,
    pub grab_link: wl_list,
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
    pub reposition: wl_signal,
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
    pub c2rust_unnamed: C2Rust_Unnamed_58,
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
    pub events: C2Rust_Unnamed_57,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_56,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_56 {
    pub synced: wlr_surface_synced,
    pub role_resource_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_57 {
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
pub union C2Rust_Unnamed_58 {
    pub toplevel: *mut wlr_xdg_toplevel,
    pub popup: *mut wlr_xdg_popup,
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
    pub events: C2Rust_Unnamed_60,
    pub WLR_PRIVATE: C2Rust_Unnamed_59,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_59 {
    pub synced: wlr_surface_synced,
    pub parent_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_60 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_61,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_61 {
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
    pub bounds: C2Rust_Unnamed_62,
    pub wm_capabilities: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
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
pub struct wlr_xdg_decoration_manager_v1 {
    pub global: *mut wl_global,
    pub decorations: wl_list,
    pub events: C2Rust_Unnamed_64,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_63,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
    pub new_toplevel_decoration: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_toplevel_decoration_v1_state {
    pub mode: wlr_xdg_toplevel_decoration_v1_mode,
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
    pub events: C2Rust_Unnamed_66,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_65,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_65 {
    pub toplevel_destroy: wl_listener,
    pub surface_configure: wl_listener,
    pub surface_ack_configure: wl_listener,
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
    pub destroy: wl_signal,
    pub request_mode: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_session_lock_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_68,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_67,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
    pub new_lock: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_session_lock_v1 {
    pub resource: *mut wl_resource,
    pub surfaces: wl_list,
    pub events: C2Rust_Unnamed_70,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_69,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
    pub locked_sent: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
    pub new_surface: wl_signal,
    pub unlock: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_session_lock_surface_v1_state {
    pub width: uint32_t,
    pub height: uint32_t,
    pub configure_serial: uint32_t,
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
    pub events: C2Rust_Unnamed_72,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_71,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_71 {
    pub synced: wlr_surface_synced,
    pub output_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
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
pub struct xcb_generic_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub pad: [uint32_t; 7],
    pub full_sequence: uint32_t,
}
pub type xcb_window_t = uint32_t;
pub type xcb_pixmap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
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
    pub events: C2Rust_Unnamed_77,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_75,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_75 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_76,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_76 {
    pub x: int32_t,
    pub y: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_77 {
    pub destroy: wl_signal,
    pub ready: wl_signal,
    pub new_surface: wl_signal,
    pub remove_startup_info: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_xwayland_icccm_input_model(pub ::core::ffi::c_uint);
impl wlr_xwayland_icccm_input_model {
    pub const WLR_ICCCM_INPUT_MODEL_NONE: Self = Self(0);
    pub const WLR_ICCCM_INPUT_MODEL_PASSIVE: Self = Self(1);
    pub const WLR_ICCCM_INPUT_MODEL_LOCAL: Self = Self(2);
    pub const WLR_ICCCM_INPUT_MODEL_GLOBAL: Self = Self(3);
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
    pub events: C2Rust_Unnamed_79,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_78,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_78 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_79 {
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
pub type pcre2_code_8 = pcre2_real_code_8;
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
pub struct wlr_damage_ring {
    pub current: pixman_region32_t,
    pub WLR_PRIVATE: C2Rust_Unnamed_80,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_80 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_82,
    pub WLR_PRIVATE: C2Rust_Unnamed_81,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_81 {
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
pub struct C2Rust_Unnamed_82 {
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
    pub events: C2Rust_Unnamed_84,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_83,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_83 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_84 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_85,
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
pub struct C2Rust_Unnamed_85 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_87,
    pub data: *mut ::core::ffi::c_void,
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
    pub events: C2Rust_Unnamed_89,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_88,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_88 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_89 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_90,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_90 {
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
    pub events: C2Rust_Unnamed_95,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_91,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_91 {
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
    pub events: C2Rust_Unnamed_93,
    pub WLR_PRIVATE: C2Rust_Unnamed_92,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_92 {
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
pub struct C2Rust_Unnamed_93 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_94,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_94 {
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
pub struct C2Rust_Unnamed_95 {
    pub outputs_update: wl_signal,
    pub output_sample: wl_signal,
    pub frame_done: wl_signal,
}
pub type wlr_scene_buffer_iterator_func_t = Option<
    unsafe extern "C" fn(
        *mut wlr_scene_buffer,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_surface {
    pub buffer: *mut wlr_scene_buffer,
    pub surface: *mut wlr_surface,
    pub WLR_PRIVATE: C2Rust_Unnamed_96,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_96 {
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
pub struct wlr_scene_outputs_update_event {
    pub active: *mut *mut wlr_scene_output,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_root {
    pub node: sway_node,
    pub output_layout: *mut wlr_output_layout,
    pub root_scene: *mut wlr_scene,
    pub staging: *mut wlr_scene_tree,
    pub layer_tree: *mut wlr_scene_tree,
    pub layers: C2Rust_Unnamed_143,
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
    pub events: C2Rust_Unnamed_97,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_97 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_131,
    pub border: C2Rust_Unnamed_130,
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
    pub events: C2Rust_Unnamed_98,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_98 {
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
    pub layers: C2Rust_Unnamed_129,
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
    pub events: C2Rust_Unnamed_100,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_99,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_99 {
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_100 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_group_handle_v1 {
    pub manager: *mut wlr_ext_workspace_manager_v1,
    pub caps: uint32_t,
    pub events: C2Rust_Unnamed_102,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_101,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_101 {
    pub outputs: wl_list,
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_102 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_manager_v1 {
    pub global: *mut wl_global,
    pub groups: wl_list,
    pub workspaces: wl_list,
    pub events: C2Rust_Unnamed_104,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_103,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_103 {
    pub resources: wl_list,
    pub idle_source: *mut wl_event_source,
    pub event_loop: *mut wl_event_loop,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_104 {
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
    pub layers: C2Rust_Unnamed_126,
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
    pub session_lock: C2Rust_Unnamed_111,
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
    pub events: C2Rust_Unnamed_106,
    pub data: *mut ::core::ffi::c_void,
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
    pub new_object: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_text_input_manager_v3 {
    pub global: *mut wl_global,
    pub text_inputs: wl_list,
    pub events: C2Rust_Unnamed_108,
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
    pub new_text_input: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_manager_v2 {
    pub global: *mut wl_global,
    pub input_methods: wl_list,
    pub events: C2Rust_Unnamed_110,
    pub WLR_PRIVATE: C2Rust_Unnamed_109,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_109 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_110 {
    pub new_input_method: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_111 {
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
    pub events: C2Rust_Unnamed_113,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_112,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_112 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_113 {
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
    pub events: C2Rust_Unnamed_115,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_114,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_114 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_115 {
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
    pub events: C2Rust_Unnamed_117,
    pub WLR_PRIVATE: C2Rust_Unnamed_116,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_116 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_117 {
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
    pub events: C2Rust_Unnamed_119,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_118,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_118 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_119 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_pointers: wl_list,
    pub events: C2Rust_Unnamed_121,
    pub WLR_PRIVATE: C2Rust_Unnamed_120,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_120 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_121 {
    pub new_virtual_pointer: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_keyboards: wl_list,
    pub events: C2Rust_Unnamed_123,
    pub WLR_PRIVATE: C2Rust_Unnamed_122,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_122 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_123 {
    pub new_virtual_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_125,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_124,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_124 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_125 {
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_126 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_128,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_127,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_127 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_128 {
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
pub struct C2Rust_Unnamed_129 {
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
pub struct C2Rust_Unnamed_130 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_131 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_133,
    pub events: C2Rust_Unnamed_132,
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
pub struct C2Rust_Unnamed_132 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_133 {
    pub wlr_xdg_toplevel: *mut wlr_xdg_toplevel,
    pub wlr_xwayland_surface: *mut wlr_xwayland_surface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct launcher_ctx {
    pub pid: pid_t,
    pub fallback_name: *mut ::core::ffi::c_char,
    pub token: *mut wlr_xdg_activation_token_v1,
    pub token_destroy: wl_listener,
    pub seat: *mut sway_seat,
    pub seat_destroy: wl_listener,
    pub activated: bool,
    pub had_focused_surface: bool,
    pub node: *mut sway_node,
    pub node_destroy: wl_listener,
    pub link: wl_list,
}
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
    pub events: C2Rust_Unnamed_135,
    pub WLR_PRIVATE: C2Rust_Unnamed_134,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_134 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_135 {
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
    pub events: C2Rust_Unnamed_137,
    pub WLR_PRIVATE: C2Rust_Unnamed_136,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_136 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_137 {
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
    pub events: C2Rust_Unnamed_138,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_138 {
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
pub struct sway_cursor {
    pub seat: *mut sway_seat,
    pub cursor: *mut wlr_cursor,
    pub previous: C2Rust_Unnamed_142,
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
    pub events: C2Rust_Unnamed_140,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_139,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_139 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub synced: wlr_surface_synced,
    pub destroying: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_140 {
    pub set_region: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_constraint_v1_state {
    pub committed: uint32_t,
    pub region: pixman_region32_t,
    pub cursor_hint: C2Rust_Unnamed_141,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_141 {
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
pub struct C2Rust_Unnamed_142 {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub node: *mut sway_node,
}
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
pub struct C2Rust_Unnamed_143 {
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
    pub xcursor_theme: C2Rust_Unnamed_144,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_144 {
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
    pub colors: C2Rust_Unnamed_145,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_145 {
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
    pub border_colors: C2Rust_Unnamed_148,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_146,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_146 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_147,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_147 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_148 {
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
    pub events: C2Rust_Unnamed_150,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_149,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_149 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_150 {
    pub destroy: wl_signal,
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
pub struct criteria_type(pub ::core::ffi::c_uint);
impl criteria_type {
    pub const CT_COMMAND: Self = Self(1);
    pub const CT_ASSIGN_OUTPUT: Self = Self(2);
    pub const CT_ASSIGN_WORKSPACE: Self = Self(4);
    pub const CT_ASSIGN_WORKSPACE_NUMBER: Self = Self(8);
    pub const CT_NO_FOCUS: Self = Self(16);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct criteria {
    pub r#type: criteria_type,
    pub raw: *mut ::core::ffi::c_char,
    pub cmdlist: *mut ::core::ffi::c_char,
    pub target: *mut ::core::ffi::c_char,
    pub title: *mut pattern,
    pub shell: *mut pattern,
    pub app_id: *mut pattern,
    pub con_mark: *mut pattern,
    pub con_id: uint32_t,
    pub class: *mut pattern,
    pub id: uint32_t,
    pub instance: *mut pattern,
    pub window_role: *mut pattern,
    pub window_type: atom_name,
    pub all: bool,
    pub floating: bool,
    pub tiling: bool,
    pub urgent: ::core::ffi::c_char,
    pub workspace: *mut pattern,
    pub pid: pid_t,
    pub sandbox_engine: *mut pattern,
    pub sandbox_app_id: *mut pattern,
    pub sandbox_instance_id: *mut pattern,
    pub tag: *mut pattern,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern {
    pub match_type: pattern_type,
    pub regex: *mut pcre2_code_8,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct pattern_type(pub ::core::ffi::c_uint);
impl pattern_type {
    pub const PATTERN_PCRE2: Self = Self(0);
    pub const PATTERN_FOCUSED: Self = Self(1);
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
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DBL_MAX: ::core::ffi::c_double = __DBL_MAX__;
#[inline]
unsafe extern "C" fn wl_signal_init(mut signal: *mut wl_signal) {
    wl_list_init(&raw mut (*signal).listener_list);
}
#[inline]
unsafe extern "C" fn wl_signal_add(
    mut signal: *mut wl_signal,
    mut listener: *mut wl_listener,
) {
    wl_list_insert((*signal).listener_list.prev, &raw mut (*listener).link);
}
unsafe extern "C" fn handle_outputs_update(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut view: *mut sway_view = (listener as *mut ::core::ffi::c_char)
        .offset(-(48 as ::core::ffi::c_ulong as isize)) as *mut sway_view;
    let mut event: *mut wlr_scene_outputs_update_event = data
        as *mut wlr_scene_outputs_update_event;
    let mut toplevel: *mut wlr_foreign_toplevel_handle_v1 = (*view).foreign_toplevel;
    if !toplevel.is_null() {
        let mut toplevel_output: *mut wlr_foreign_toplevel_handle_v1_output = ::core::ptr::null_mut::<
            wlr_foreign_toplevel_handle_v1_output,
        >();
        let mut tmp: *mut wlr_foreign_toplevel_handle_v1_output = ::core::ptr::null_mut::<
            wlr_foreign_toplevel_handle_v1_output,
        >();
        toplevel_output = ((*toplevel).outputs.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut wlr_foreign_toplevel_handle_v1_output
            as *mut wlr_foreign_toplevel_handle_v1_output;
        tmp = ((*toplevel_output).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut wlr_foreign_toplevel_handle_v1_output
            as *mut wlr_foreign_toplevel_handle_v1_output;
        while &raw mut (*toplevel_output).link != &raw mut (*toplevel).outputs {
            let mut active: bool = r#false != 0;
            let mut i: size_t = 0 as size_t;
            while i < (*event).size {
                let mut scene_output: *mut wlr_scene_output = *(*event)
                    .active
                    .offset(i as isize);
                if (*scene_output).output == (*toplevel_output).output {
                    active = r#true != 0;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            if !active {
                wlr_foreign_toplevel_handle_v1_output_leave(
                    toplevel,
                    (*toplevel_output).output,
                );
            }
            toplevel_output = tmp;
            tmp = ((*toplevel_output).link.next as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize))
                as *mut wlr_foreign_toplevel_handle_v1_output
                as *mut wlr_foreign_toplevel_handle_v1_output;
        }
        let mut i_0: size_t = 0 as size_t;
        while i_0 < (*event).size {
            let mut scene_output_0: *mut wlr_scene_output = *(*event)
                .active
                .offset(i_0 as isize);
            wlr_foreign_toplevel_handle_v1_output_enter(
                toplevel,
                (*scene_output_0).output,
            );
            i_0 = i_0.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn handle_point_accepts_input(
    mut buffer: *mut wlr_scene_buffer,
    mut x: *mut ::core::ffi::c_double,
    mut y: *mut ::core::ffi::c_double,
) -> bool {
    return r#false != 0;
}
#[no_mangle]
pub unsafe extern "C" fn view_init(
    mut view: *mut sway_view,
    mut r#type: sway_view_type,
    mut r#impl: *const sway_view_impl,
) -> bool {
    let mut failed: bool = r#false != 0;
    (*view).scene_tree = alloc_scene_tree((*root).staging, &raw mut failed);
    (*view).content_tree = alloc_scene_tree((*view).scene_tree, &raw mut failed);
    if !failed {
        if scene_descriptor_assign(
            &raw mut (*(*view).scene_tree).node,
            sway_scene_descriptor_type::SWAY_SCENE_DESC_VIEW,
            view as *mut ::core::ffi::c_void,
        ) {
            (*view).output_handler = wlr_scene_buffer_create(
                (*view).scene_tree,
                ::core::ptr::null_mut::<wlr_buffer>(),
            );
            if (*view).output_handler.is_null() {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] Failed to allocate a scene node\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
                    90 as ::core::ffi::c_int,
                );
            } else {
                (*view).image_capture_scene = wlr_scene_create();
                if !(*view).image_capture_scene.is_null() {
                    (*(*view).image_capture_scene).restack_xwayland_surfaces = r#false
                        != 0;
                    (*view).outputs_update.notify = Some(
                        handle_outputs_update
                            as unsafe extern "C" fn(
                                *mut wl_listener,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ) as wl_notify_func_t;
                    wl_signal_add(
                        &raw mut (*(*view).output_handler).events.outputs_update,
                        &raw mut (*view).outputs_update,
                    );
                    (*(*view).output_handler).point_accepts_input = Some(
                        handle_point_accepts_input
                            as unsafe extern "C" fn(
                                *mut wlr_scene_buffer,
                                *mut ::core::ffi::c_double,
                                *mut ::core::ffi::c_double,
                            ) -> bool,
                    ) as wlr_scene_buffer_point_accepts_input_func_t;
                    (*view).r#type = r#type;
                    (*view).r#impl = r#impl;
                    (*view).executed_criteria = create_list();
                    (*view).allow_request_urgent = r#true != 0;
                    (*view).shortcuts_inhibit = seat_config_shortcuts_inhibit::SHORTCUTS_INHIBIT_DEFAULT;
                    (*view).tearing_mode = sway_view_tearing_mode::TEARING_WINDOW_HINT;
                    wl_signal_init(&raw mut (*view).events.unmap);
                    return r#true != 0;
                }
            }
        }
    }
    wlr_scene_node_destroy(&raw mut (*(*view).scene_tree).node);
    return r#false != 0;
}
#[no_mangle]
pub unsafe extern "C" fn view_destroy(mut view: *mut sway_view) {
    if !_sway_assert(
        (*view).surface.is_null(),
        b"[%s:%d] %s:Tried to free mapped view\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
        120 as ::core::ffi::c_int,
        b"void view_destroy(struct sway_view *)\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if !_sway_assert(
        (*view).destroying,
        b"[%s:%d] %s:Tried to free view which wasn't marked as destroying\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
        123 as ::core::ffi::c_int,
        b"void view_destroy(struct sway_view *)\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if !_sway_assert(
        (*view).container.is_null(),
        b"[%s:%d] %s:Tried to free view which still has a container (might have a pending transaction?)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
        127 as ::core::ffi::c_int,
        b"void view_destroy(struct sway_view *)\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    wl_list_remove(&raw mut (*view).events.unmap.listener_list);
    list_free((*view).executed_criteria);
    view_assign_ctx(view, ::core::ptr::null_mut::<launcher_ctx>());
    wlr_scene_node_destroy(&raw mut (*(*view).image_capture_scene).tree.node);
    wlr_scene_node_destroy(&raw mut (*(*view).scene_tree).node);
    if (*(*view).r#impl).destroy.is_some() {
        (*(*view).r#impl).destroy.expect("non-null function pointer")(view);
    } else {
        free(view as *mut ::core::ffi::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn view_begin_destroy(mut view: *mut sway_view) {
    if !_sway_assert(
        (*view).surface.is_null(),
        b"[%s:%d] %s:Tried to destroy a mapped view\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
        146 as ::core::ffi::c_int,
        b"void view_begin_destroy(struct sway_view *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    (*view).destroying = r#true != 0;
    wl_list_remove(&raw mut (*view).outputs_update.link);
    if (*view).container.is_null() {
        view_destroy(view);
    }
}
#[no_mangle]
pub unsafe extern "C" fn view_get_title(
    mut view: *mut sway_view,
) -> *const ::core::ffi::c_char {
    if (*(*view).r#impl).get_string_prop.is_some() {
        return (*(*view).r#impl)
            .get_string_prop
            .expect("non-null function pointer")(view, sway_view_prop::VIEW_PROP_TITLE);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn view_get_app_id(
    mut view: *mut sway_view,
) -> *const ::core::ffi::c_char {
    if (*(*view).r#impl).get_string_prop.is_some() {
        return (*(*view).r#impl)
            .get_string_prop
            .expect("non-null function pointer")(view, sway_view_prop::VIEW_PROP_APP_ID);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn view_get_class(
    mut view: *mut sway_view,
) -> *const ::core::ffi::c_char {
    if (*(*view).r#impl).get_string_prop.is_some() {
        return (*(*view).r#impl)
            .get_string_prop
            .expect("non-null function pointer")(view, sway_view_prop::VIEW_PROP_CLASS);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn view_get_instance(
    mut view: *mut sway_view,
) -> *const ::core::ffi::c_char {
    if (*(*view).r#impl).get_string_prop.is_some() {
        return (*(*view).r#impl)
            .get_string_prop
            .expect(
                "non-null function pointer",
            )(view, sway_view_prop::VIEW_PROP_INSTANCE);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn view_get_x11_window_id(mut view: *mut sway_view) -> uint32_t {
    if (*(*view).r#impl).get_int_prop.is_some() {
        return (*(*view).r#impl)
            .get_int_prop
            .expect(
                "non-null function pointer",
            )(view, sway_view_prop::VIEW_PROP_X11_WINDOW_ID);
    }
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn view_get_x11_parent_id(mut view: *mut sway_view) -> uint32_t {
    if (*(*view).r#impl).get_int_prop.is_some() {
        return (*(*view).r#impl)
            .get_int_prop
            .expect(
                "non-null function pointer",
            )(view, sway_view_prop::VIEW_PROP_X11_PARENT_ID);
    }
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn view_get_window_role(
    mut view: *mut sway_view,
) -> *const ::core::ffi::c_char {
    if (*(*view).r#impl).get_string_prop.is_some() {
        return (*(*view).r#impl)
            .get_string_prop
            .expect(
                "non-null function pointer",
            )(view, sway_view_prop::VIEW_PROP_WINDOW_ROLE);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn view_get_window_type(mut view: *mut sway_view) -> uint32_t {
    if (*(*view).r#impl).get_int_prop.is_some() {
        return (*(*view).r#impl)
            .get_int_prop
            .expect(
                "non-null function pointer",
            )(view, sway_view_prop::VIEW_PROP_WINDOW_TYPE);
    }
    return 0 as uint32_t;
}
unsafe extern "C" fn security_context_from_view(
    mut view: *mut sway_view,
) -> *const wlr_security_context_v1_state {
    let mut client: *const wl_client = wl_resource_get_client(
        (*(*view).surface).resource,
    );
    let mut security_context: *const wlr_security_context_v1_state = wlr_security_context_manager_v1_lookup_client(
        server.security_context_manager_v1,
        client,
    );
    return security_context;
}
#[no_mangle]
pub unsafe extern "C" fn view_get_sandbox_engine(
    mut view: *mut sway_view,
) -> *const ::core::ffi::c_char {
    let mut security_context: *const wlr_security_context_v1_state = security_context_from_view(
        view,
    );
    return if !security_context.is_null() {
        (*security_context).sandbox_engine
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn view_get_sandbox_app_id(
    mut view: *mut sway_view,
) -> *const ::core::ffi::c_char {
    let mut security_context: *const wlr_security_context_v1_state = security_context_from_view(
        view,
    );
    return if !security_context.is_null() {
        (*security_context).app_id
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn view_get_sandbox_instance_id(
    mut view: *mut sway_view,
) -> *const ::core::ffi::c_char {
    let mut security_context: *const wlr_security_context_v1_state = security_context_from_view(
        view,
    );
    return if !security_context.is_null() {
        (*security_context).instance_id
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn view_get_tag(
    mut view: *mut sway_view,
) -> *const ::core::ffi::c_char {
    if (*(*view).r#impl).get_string_prop.is_some() {
        return (*(*view).r#impl)
            .get_string_prop
            .expect("non-null function pointer")(view, sway_view_prop::VIEW_PROP_TAG);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn view_get_shell(
    mut view: *mut sway_view,
) -> *const ::core::ffi::c_char {
    match (*view).r#type {
        sway_view_type::SWAY_VIEW_XDG_SHELL => {
            return b"xdg_shell\0".as_ptr() as *const ::core::ffi::c_char;
        }
        sway_view_type::SWAY_VIEW_XWAYLAND => {
            return b"xwayland\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn view_get_constraints(
    mut view: *mut sway_view,
    mut min_width: *mut ::core::ffi::c_double,
    mut max_width: *mut ::core::ffi::c_double,
    mut min_height: *mut ::core::ffi::c_double,
    mut max_height: *mut ::core::ffi::c_double,
) {
    if (*(*view).r#impl).get_constraints.is_some() {
        (*(*view).r#impl)
            .get_constraints
            .expect(
                "non-null function pointer",
            )(view, min_width, max_width, min_height, max_height);
    } else {
        *min_width = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *max_width = DBL_MAX;
        *min_height = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *max_height = DBL_MAX;
    };
}
#[no_mangle]
pub unsafe extern "C" fn view_configure(
    mut view: *mut sway_view,
    mut lx: ::core::ffi::c_double,
    mut ly: ::core::ffi::c_double,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> uint32_t {
    if (*(*view).r#impl).configure.is_some() {
        return (*(*view).r#impl)
            .configure
            .expect("non-null function pointer")(view, lx, ly, width, height);
    }
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn view_inhibit_idle(mut view: *mut sway_view) -> bool {
    if !server.session_lock.lock.is_null() {
        return r#false != 0;
    }
    let mut user_inhibitor: *mut sway_idle_inhibitor_v1 = sway_idle_inhibit_v1_user_inhibitor_for_view(
        view,
    );
    let mut application_inhibitor: *mut sway_idle_inhibitor_v1 = sway_idle_inhibit_v1_application_inhibitor_for_view(
        view,
    );
    if user_inhibitor.is_null() && application_inhibitor.is_null() {
        return r#false != 0;
    }
    if user_inhibitor.is_null() {
        return sway_idle_inhibit_v1_is_active(application_inhibitor);
    }
    if application_inhibitor.is_null() {
        return sway_idle_inhibit_v1_is_active(user_inhibitor);
    }
    return sway_idle_inhibit_v1_is_active(user_inhibitor) as ::core::ffi::c_int != 0
        || sway_idle_inhibit_v1_is_active(application_inhibitor) as ::core::ffi::c_int
            != 0;
}
#[no_mangle]
pub unsafe extern "C" fn view_ancestor_is_only_visible(
    mut view: *mut sway_view,
) -> bool {
    let mut only_visible: bool = r#true != 0;
    let mut con: *mut sway_container = (*view).container;
    while !con.is_null() {
        let mut layout: sway_container_layout = container_parent_layout(con);
        if layout.0 != sway_container_layout_0::L_TABBED.0
            && layout.0 != sway_container_layout_0::L_STACKED.0
        {
            let mut siblings: *mut list_t = container_get_siblings(con);
            if !siblings.is_null() && (*siblings).length > 1 as ::core::ffi::c_int {
                only_visible = r#false != 0;
            }
        } else {
            only_visible = r#true != 0;
        }
        con = (*con).pending.parent;
    }
    return only_visible;
}
unsafe extern "C" fn view_is_only_visible(mut view: *mut sway_view) -> bool {
    let mut con: *mut sway_container = (*view).container;
    while !con.is_null() {
        let mut layout: sway_container_layout = container_parent_layout(con);
        if layout.0 != sway_container_layout_0::L_TABBED.0
            && layout.0 != sway_container_layout_0::L_STACKED.0
        {
            let mut siblings: *mut list_t = container_get_siblings(con);
            if !siblings.is_null() && (*siblings).length > 1 as ::core::ffi::c_int {
                return r#false != 0;
            }
        }
        con = (*con).pending.parent;
    }
    return r#true != 0;
}
unsafe extern "C" fn gaps_to_edge(mut view: *mut sway_view) -> bool {
    let mut gaps: side_gaps = (*(*(*view).container).pending.workspace).current_gaps;
    return gaps.top > 0 as ::core::ffi::c_int || gaps.right > 0 as ::core::ffi::c_int
        || gaps.bottom > 0 as ::core::ffi::c_int || gaps.left > 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn view_autoconfigure(mut view: *mut sway_view) {
    let mut con: *mut sway_container = (*view).container;
    let mut ws: *mut sway_workspace = (*con).pending.workspace;
    if container_is_scratchpad_hidden(con) as ::core::ffi::c_int != 0
        && (*con).pending.fullscreen_mode.0 != sway_fullscreen_mode::FULLSCREEN_GLOBAL.0
    {
        return;
    }
    let mut output: *mut sway_output = if !ws.is_null() {
        (*ws).output
    } else {
        ::core::ptr::null_mut::<sway_output>()
    };
    if !output.is_null()
        && (*con).pending.fullscreen_mode.0
            == sway_fullscreen_mode::FULLSCREEN_WORKSPACE.0
    {
        (*con).pending.content_x = (*output).lx as ::core::ffi::c_double;
        (*con).pending.content_y = (*output).ly as ::core::ffi::c_double;
        (*con).pending.content_width = (*output).width as ::core::ffi::c_double;
        (*con).pending.content_height = (*output).height as ::core::ffi::c_double;
        return;
    } else if (*con).pending.fullscreen_mode.0
        == sway_fullscreen_mode::FULLSCREEN_GLOBAL.0
    {
        (*con).pending.content_x = (*root).x;
        (*con).pending.content_y = (*root).y;
        (*con).pending.content_width = (*root).width;
        (*con).pending.content_height = (*root).height;
        return;
    }
    (*con).pending.border_bottom = r#true != 0;
    (*con).pending.border_top = (*con).pending.border_bottom;
    (*con).pending.border_right = r#true != 0;
    (*con).pending.border_left = (*con).pending.border_right;
    let mut y_offset: ::core::ffi::c_double = 0 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    if !container_is_floating_or_child(con) && !ws.is_null() {
        if (*config).hide_edge_borders.0 == edge_border_types::E_BOTH.0
            || (*config).hide_edge_borders.0 == edge_border_types::E_VERTICAL.0
        {
            (*con).pending.border_left = (*con).pending.x != (*ws).x;
            let mut right_x: ::core::ffi::c_int = ((*con).pending.x
                + (*con).pending.width) as ::core::ffi::c_int;
            (*con).pending.border_right = right_x as ::core::ffi::c_double
                != (*ws).x + (*ws).width as ::core::ffi::c_double;
        }
        if (*config).hide_edge_borders.0 == edge_border_types::E_BOTH.0
            || (*config).hide_edge_borders.0 == edge_border_types::E_HORIZONTAL.0
        {
            (*con).pending.border_top = (*con).pending.y != (*ws).y;
            let mut bottom_y: ::core::ffi::c_int = ((*con).pending.y
                + (*con).pending.height) as ::core::ffi::c_int;
            (*con).pending.border_bottom = bottom_y as ::core::ffi::c_double
                != (*ws).y + (*ws).height as ::core::ffi::c_double;
        }
        let mut smart: bool = (*config).hide_edge_borders_smart.0
            == edge_border_smart_types::ESMART_ON.0
            || (*config).hide_edge_borders_smart.0
                == edge_border_smart_types::ESMART_NO_GAPS.0 && !gaps_to_edge(view);
        if smart {
            let mut show_border: bool = !view_is_only_visible(view);
            (*con).pending.border_left = (*con).pending.border_left as ::core::ffi::c_int
                & show_border as ::core::ffi::c_int != 0;
            (*con).pending.border_right = (*con).pending.border_right
                as ::core::ffi::c_int & show_border as ::core::ffi::c_int != 0;
            (*con).pending.border_top = (*con).pending.border_top as ::core::ffi::c_int
                & show_border as ::core::ffi::c_int != 0;
            (*con).pending.border_bottom = (*con).pending.border_bottom
                as ::core::ffi::c_int & show_border as ::core::ffi::c_int != 0;
        }
    }
    if !container_is_floating(con) {
        let mut siblings: *mut list_t = container_get_siblings(con);
        let mut show_titlebar: bool = !siblings.is_null()
            && (*siblings).length > 1 as ::core::ffi::c_int || !(*config).hide_lone_tab;
        if show_titlebar {
            let mut layout: sway_container_layout = container_parent_layout(con);
            if layout.0 == sway_container_layout_0::L_TABBED.0 {
                y_offset = container_titlebar_height() as ::core::ffi::c_double;
                (*con).pending.border_top = r#false != 0;
            } else if layout.0 == sway_container_layout_0::L_STACKED.0 {
                y_offset = container_titlebar_height()
                    .wrapping_mul((*siblings).length as size_t) as ::core::ffi::c_double;
                (*con).pending.border_top = r#false != 0;
            }
        }
    }
    let mut x: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    let mut width: ::core::ffi::c_double = 0.;
    let mut height: ::core::ffi::c_double = 0.;
    match (*con).pending.border {
        sway_container_border::B_PIXEL => {
            x = (*con).pending.x
                + ((*con).pending.border_thickness
                    * (*con).pending.border_left as ::core::ffi::c_int)
                    as ::core::ffi::c_double;
            y = (*con).pending.y
                + ((*con).pending.border_thickness
                    * (*con).pending.border_top as ::core::ffi::c_int)
                    as ::core::ffi::c_double + y_offset;
            width = (*con).pending.width
                - ((*con).pending.border_thickness
                    * (*con).pending.border_left as ::core::ffi::c_int)
                    as ::core::ffi::c_double
                - ((*con).pending.border_thickness
                    * (*con).pending.border_right as ::core::ffi::c_int)
                    as ::core::ffi::c_double;
            height = (*con).pending.height - y_offset
                - ((*con).pending.border_thickness
                    * (*con).pending.border_top as ::core::ffi::c_int)
                    as ::core::ffi::c_double
                - ((*con).pending.border_thickness
                    * (*con).pending.border_bottom as ::core::ffi::c_int)
                    as ::core::ffi::c_double;
        }
        sway_container_border::B_NORMAL => {
            x = (*con).pending.x
                + ((*con).pending.border_thickness
                    * (*con).pending.border_left as ::core::ffi::c_int)
                    as ::core::ffi::c_double;
            width = (*con).pending.width
                - ((*con).pending.border_thickness
                    * (*con).pending.border_left as ::core::ffi::c_int)
                    as ::core::ffi::c_double
                - ((*con).pending.border_thickness
                    * (*con).pending.border_right as ::core::ffi::c_int)
                    as ::core::ffi::c_double;
            if y_offset != 0. {
                y = (*con).pending.y + y_offset;
                height = (*con).pending.height - y_offset
                    - ((*con).pending.border_thickness
                        * (*con).pending.border_bottom as ::core::ffi::c_int)
                        as ::core::ffi::c_double;
            } else {
                y = (*con).pending.y
                    + container_titlebar_height() as ::core::ffi::c_double;
                height = (*con).pending.height
                    - container_titlebar_height() as ::core::ffi::c_double
                    - ((*con).pending.border_thickness
                        * (*con).pending.border_bottom as ::core::ffi::c_int)
                        as ::core::ffi::c_double;
            }
        }
        sway_container_border::B_CSD | sway_container_border::B_NONE | _ => {
            x = (*con).pending.x;
            y = (*con).pending.y + y_offset;
            width = (*con).pending.width;
            height = (*con).pending.height - y_offset;
        }
    }
    (*con).pending.content_x = x;
    (*con).pending.content_y = y;
    (*con).pending.content_width = fmax(
        width,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    (*con).pending.content_height = fmax(
        height,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn view_set_activated(
    mut view: *mut sway_view,
    mut activated: bool,
) {
    if (*(*view).r#impl).set_activated.is_some() {
        (*(*view).r#impl)
            .set_activated
            .expect("non-null function pointer")(view, activated);
    }
    if !(*view).foreign_toplevel.is_null() {
        wlr_foreign_toplevel_handle_v1_set_activated(
            (*view).foreign_toplevel,
            activated,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn view_request_activate(
    mut view: *mut sway_view,
    mut seat: *mut sway_seat,
) {
    let mut ws: *mut sway_workspace = (*(*view).container).pending.workspace;
    if seat.is_null() {
        seat = input_manager_current_seat();
    }
    match (*config).focus_on_window_activation {
        sway_fowa::FOWA_SMART => {
            if !ws.is_null() && workspace_is_visible(ws) as ::core::ffi::c_int != 0 {
                seat_set_focus_container(seat, (*view).container);
                container_raise_floating((*view).container);
            } else {
                view_set_urgent(view, r#true != 0);
            }
        }
        sway_fowa::FOWA_URGENT => {
            view_set_urgent(view, r#true != 0);
        }
        sway_fowa::FOWA_FOCUS => {
            if container_is_scratchpad_hidden_or_child((*view).container) {
                root_scratchpad_show((*view).container);
            } else {
                seat_set_focus_container(seat, (*view).container);
                container_raise_floating((*view).container);
            }
        }
        sway_fowa::FOWA_NONE | _ => {}
    }
    transaction_commit_dirty();
}
#[no_mangle]
pub unsafe extern "C" fn view_request_urgent(mut view: *mut sway_view) {
    if (*config).focus_on_window_activation.0 != sway_fowa::FOWA_NONE.0 {
        view_set_urgent(view, r#true != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn view_set_csd_from_server(
    mut view: *mut sway_view,
    mut enabled: bool,
) {
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Telling view %p to set CSD to %i\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
        515 as ::core::ffi::c_int,
        view,
        enabled as ::core::ffi::c_int,
    );
    if !(*view).xdg_decoration.is_null() {
        let mut mode: uint32_t = (if enabled as ::core::ffi::c_int != 0 {
            wlr_xdg_toplevel_decoration_v1_mode::WLR_XDG_TOPLEVEL_DECORATION_V1_MODE_CLIENT_SIDE
                .0 as ::core::ffi::c_int
        } else {
            wlr_xdg_toplevel_decoration_v1_mode::WLR_XDG_TOPLEVEL_DECORATION_V1_MODE_SERVER_SIDE
                .0 as ::core::ffi::c_int
        }) as uint32_t;
        wlr_xdg_toplevel_decoration_v1_set_mode(
            (*(*view).xdg_decoration).wlr_xdg_decoration,
            wlr_xdg_toplevel_decoration_v1_mode(mode as ::core::ffi::c_uint),
        );
    }
    (*view).using_csd = enabled;
}
#[no_mangle]
pub unsafe extern "C" fn view_update_csd_from_client(
    mut view: *mut sway_view,
    mut enabled: bool,
) {
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] View %p updated CSD to %i\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
        527 as ::core::ffi::c_int,
        view,
        enabled as ::core::ffi::c_int,
    );
    let mut con: *mut sway_container = (*view).container;
    if enabled as ::core::ffi::c_int != 0 && !con.is_null()
        && (*con).pending.border.0 != sway_container_border::B_CSD.0
    {
        (*con).saved_border = (*con).pending.border;
        if container_is_floating(con) {
            (*con).pending.border = sway_container_border::B_CSD;
        }
    } else if !enabled && !con.is_null()
        && (*con).pending.border.0 == sway_container_border::B_CSD.0
    {
        (*con).pending.border = (*con).saved_border;
    }
    (*view).using_csd = enabled;
}
#[no_mangle]
pub unsafe extern "C" fn view_set_tiled(mut view: *mut sway_view, mut tiled: bool) {
    if (*(*view).r#impl).set_tiled.is_some() {
        (*(*view).r#impl).set_tiled.expect("non-null function pointer")(view, tiled);
    }
}
#[no_mangle]
pub unsafe extern "C" fn view_close(mut view: *mut sway_view) {
    if (*(*view).r#impl).close.is_some() {
        (*(*view).r#impl).close.expect("non-null function pointer")(view);
    }
}
#[no_mangle]
pub unsafe extern "C" fn view_close_popups(mut view: *mut sway_view) {
    if (*(*view).r#impl).close_popups.is_some() {
        (*(*view).r#impl).close_popups.expect("non-null function pointer")(view);
    }
}
unsafe extern "C" fn view_has_executed_criteria(
    mut view: *mut sway_view,
    mut criteria: *mut criteria,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*view).executed_criteria).length {
        let mut item: *mut criteria = *(*(*view).executed_criteria)
            .items
            .offset(i as isize) as *mut criteria;
        if item == criteria {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
#[no_mangle]
pub unsafe extern "C" fn view_execute_criteria(mut view: *mut sway_view) {
    let mut criterias: *mut list_t = criteria_for_view(view, criteria_type::CT_COMMAND);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*criterias).length {
        let mut criteria: *mut criteria = *(*criterias).items.offset(i as isize)
            as *mut criteria;
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Checking criteria %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
            573 as ::core::ffi::c_int,
            (*criteria).raw,
        );
        if view_has_executed_criteria(view, criteria) {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Criteria already executed\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
                575 as ::core::ffi::c_int,
            );
        } else {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] for_window '%s' matches view %p, cmd: '%s'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
                578 as ::core::ffi::c_int,
                (*criteria).raw,
                view,
                (*criteria).cmdlist,
            );
            list_add((*view).executed_criteria, criteria as *mut ::core::ffi::c_void);
            let mut res_list: *mut list_t = execute_command(
                (*criteria).cmdlist,
                ::core::ptr::null_mut::<sway_seat>(),
                (*view).container,
            );
            while (*res_list).length != 0 {
                let mut res: *mut cmd_results = *(*res_list).items.offset(0isize)
                    as *mut cmd_results;
                if (*res).status.0 != cmd_status::CMD_SUCCESS.0 {
                    _sway_log(
                        sway_log_importance_t::SWAY_ERROR,
                        b"[%s:%d] for_window '%s' failed: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
                        585 as ::core::ffi::c_int,
                        (*criteria).raw,
                        (*res).error,
                    );
                }
                free_cmd_results(res);
                list_del(res_list, 0 as ::core::ffi::c_int);
            }
            list_free(res_list);
        }
        i += 1;
    }
    list_free(criterias);
}
unsafe extern "C" fn view_populate_pid(mut view: *mut sway_view) {
    let mut pid: pid_t = 0;
    let mut surf: *mut wlr_xwayland_surface = ::core::ptr::null_mut::<
        wlr_xwayland_surface,
    >();
    match (*view).r#type {
        sway_view_type::SWAY_VIEW_XWAYLAND => {
            surf = wlr_xwayland_surface_try_from_wlr_surface((*view).surface);
            pid = (*surf).pid;
        }
        sway_view_type::SWAY_VIEW_XDG_SHELL => {
            let mut client: *mut wl_client = wl_resource_get_client(
                (*(*view).surface).resource,
            );
            wl_client_get_credentials(
                client,
                &raw mut pid,
                ::core::ptr::null_mut::<uid_t>(),
                ::core::ptr::null_mut::<gid_t>(),
            );
        }
        _ => {}
    }
    (*view).pid = pid;
}
#[no_mangle]
pub unsafe extern "C" fn view_assign_ctx(
    mut view: *mut sway_view,
    mut ctx: *mut launcher_ctx,
) {
    if !(*view).ctx.is_null() {
        launcher_ctx_destroy((*view).ctx);
        (*view).ctx = ::core::ptr::null_mut::<launcher_ctx>();
    }
    if ctx.is_null() {
        return;
    }
    launcher_ctx_consume(ctx);
    (*view).ctx = ctx as *mut launcher_ctx;
}
unsafe extern "C" fn select_workspace(mut view: *mut sway_view) -> *mut sway_workspace {
    let mut seat: *mut sway_seat = input_manager_current_seat();
    let mut criterias: *mut list_t = criteria_for_view(
        view,
        criteria_type(
            (criteria_type::CT_ASSIGN_WORKSPACE.0 as ::core::ffi::c_int
                | criteria_type::CT_ASSIGN_WORKSPACE_NUMBER.0 as ::core::ffi::c_int
                | criteria_type::CT_ASSIGN_OUTPUT.0 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        ),
    );
    let mut ws: *mut sway_workspace = ::core::ptr::null_mut::<sway_workspace>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*criterias).length {
        let mut criteria: *mut criteria = *(*criterias).items.offset(i as isize)
            as *mut criteria;
        if (*criteria).r#type.0 == criteria_type::CT_ASSIGN_OUTPUT.0 {
            let mut output: *mut sway_output = output_by_name_or_id((*criteria).target);
            if !output.is_null() {
                ws = output_get_active_workspace(output);
                break;
            } else {
                i += 1;
            }
        } else {
            ws = if (*criteria).r#type.0 == criteria_type::CT_ASSIGN_WORKSPACE_NUMBER.0 {
                workspace_by_number((*criteria).target)
            } else {
                workspace_by_name((*criteria).target)
            };
            if ws.is_null() {
                if strcasecmp(
                    (*criteria).target,
                    b"back_and_forth\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    if !(*seat).prev_workspace_name.is_null() {
                        ws = workspace_create(
                            ::core::ptr::null_mut::<sway_output>(),
                            (*seat).prev_workspace_name,
                        );
                    }
                } else {
                    ws = workspace_create(
                        ::core::ptr::null_mut::<sway_output>(),
                        (*criteria).target,
                    );
                }
            }
            break;
        }
    }
    list_free(criterias);
    if !ws.is_null() {
        view_assign_ctx(view, ::core::ptr::null_mut::<launcher_ctx>());
        return ws;
    }
    ws = if !(*view).ctx.is_null() {
        launcher_ctx_get_workspace((*view).ctx)
    } else {
        ::core::ptr::null_mut::<sway_workspace>()
    };
    if !ws.is_null() {
        view_assign_ctx(view, ::core::ptr::null_mut::<launcher_ctx>());
        return ws;
    }
    let mut node: *mut sway_node = seat_get_focus_inactive(seat, &raw mut (*root).node);
    if !node.is_null() && (*node).r#type.0 == sway_node_type::N_WORKSPACE.0 {
        return (*node).c2rust_unnamed.sway_workspace
    } else if !node.is_null() && (*node).r#type.0 == sway_node_type::N_CONTAINER.0 {
        return (*(*node).c2rust_unnamed.sway_container).pending.workspace
    }
    _sway_assert(
        false,
        b"[%s:%d] %s:Expected to find a workspace\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
        684 as ::core::ffi::c_int,
        b"struct sway_workspace *select_workspace(struct sway_view *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    return ::core::ptr::null_mut::<sway_workspace>();
}
unsafe extern "C" fn update_ext_foreign_toplevel(mut view: *mut sway_view) {
    let mut toplevel_state: wlr_ext_foreign_toplevel_handle_v1_state = wlr_ext_foreign_toplevel_handle_v1_state {
        title: view_get_title(view),
        app_id: view_get_app_id(view),
    };
    wlr_ext_foreign_toplevel_handle_v1_update_state(
        (*view).ext_foreign_toplevel,
        &raw mut toplevel_state,
    );
}
unsafe extern "C" fn should_focus(mut view: *mut sway_view) -> bool {
    let mut seat: *mut sway_seat = input_manager_current_seat();
    let mut prev_con: *mut sway_container = seat_get_focused_container(seat);
    let mut prev_ws: *mut sway_workspace = seat_get_focused_workspace(seat);
    let mut map_ws: *mut sway_workspace = (*(*view).container).pending.workspace;
    if (*(*view).container).pending.fullscreen_mode.0
        == sway_fullscreen_mode::FULLSCREEN_GLOBAL.0
    {
        return r#true != 0;
    }
    if !(*root).fullscreen_global.is_null() || map_ws.is_null()
        || !(*map_ws).fullscreen.is_null()
    {
        return r#false != 0;
    }
    if prev_ws != map_ws {
        return r#false != 0;
    }
    if (*(*view).container).pending.parent.is_null() && prev_con.is_null() {
        let mut num_children: size_t = ((*(*(*(*view).container).pending.workspace)
            .tiling)
            .length + (*(*(*(*view).container).pending.workspace).floating).length)
            as size_t;
        if num_children == 1 as size_t {
            return r#true != 0;
        }
    }
    let mut criterias: *mut list_t = criteria_for_view(view, criteria_type::CT_NO_FOCUS);
    let mut len: size_t = (*criterias).length as size_t;
    list_free(criterias);
    return len == 0 as size_t;
}
unsafe extern "C" fn handle_foreign_activate_request(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut view: *mut sway_view = (listener as *mut ::core::ffi::c_char)
        .offset(-(208 as ::core::ffi::c_ulong as isize)) as *mut sway_view;
    let mut event: *mut wlr_foreign_toplevel_handle_v1_activated_event = data
        as *mut wlr_foreign_toplevel_handle_v1_activated_event;
    let mut seat: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
    seat = ((*server.input).seats.next as *mut ::core::ffi::c_char)
        .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
        as *mut sway_seat;
    while &raw mut (*seat).link != &raw mut (*server.input).seats {
        if (*seat).wlr_seat == (*event).seat {
            if container_is_scratchpad_hidden_or_child((*view).container) {
                root_scratchpad_show((*view).container);
            }
            seat_set_focus_container(seat, (*view).container);
            seat_consider_warp_to_focus(seat);
            container_raise_floating((*view).container);
            break;
        } else {
            seat = ((*seat).link.next as *mut ::core::ffi::c_char)
                .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
                as *mut sway_seat;
        }
    }
    transaction_commit_dirty();
}
unsafe extern "C" fn handle_foreign_fullscreen_request(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut view: *mut sway_view = (listener as *mut ::core::ffi::c_char)
        .offset(-(232 as ::core::ffi::c_ulong as isize)) as *mut sway_view;
    let mut event: *mut wlr_foreign_toplevel_handle_v1_fullscreen_event = data
        as *mut wlr_foreign_toplevel_handle_v1_fullscreen_event;
    let mut container: *mut sway_container = (*view).container;
    if (*container).pending.workspace.is_null() {
        while !(*container).pending.parent.is_null() {
            container = (*container).pending.parent;
        }
    }
    if (*event).fullscreen as ::core::ffi::c_int != 0 && !(*event).output.is_null()
        && !(*(*event).output).data.is_null()
    {
        let mut output: *mut sway_output = (*(*event).output).data as *mut sway_output;
        let mut ws: *mut sway_workspace = output_get_active_workspace(output);
        if !ws.is_null() && !container_is_scratchpad_hidden((*view).container) {
            if container_is_floating((*view).container) {
                workspace_add_floating(ws, (*view).container);
            } else {
                workspace_add_tiling(ws, (*view).container);
            }
        }
    }
    container_set_fullscreen(
        container,
        sway_fullscreen_mode(
            (if (*event).fullscreen as ::core::ffi::c_int != 0 {
                sway_fullscreen_mode::FULLSCREEN_WORKSPACE.0 as ::core::ffi::c_int
            } else {
                sway_fullscreen_mode::FULLSCREEN_NONE.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        ),
    );
    if (*event).fullscreen {
        arrange_root();
    } else if !(*container).pending.parent.is_null() {
        arrange_container((*container).pending.parent);
    } else if !(*container).pending.workspace.is_null() {
        arrange_workspace((*container).pending.workspace);
    }
    transaction_commit_dirty();
}
unsafe extern "C" fn handle_foreign_close_request(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut view: *mut sway_view = (listener as *mut ::core::ffi::c_char)
        .offset(-(256 as ::core::ffi::c_ulong as isize)) as *mut sway_view;
    view_close(view);
}
unsafe extern "C" fn handle_foreign_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut view: *mut sway_view = (listener as *mut ::core::ffi::c_char)
        .offset(-(280 as ::core::ffi::c_ulong as isize)) as *mut sway_view;
    wl_list_remove(&raw mut (*view).foreign_activate_request.link);
    wl_list_remove(&raw mut (*view).foreign_fullscreen_request.link);
    wl_list_remove(&raw mut (*view).foreign_close_request.link);
    wl_list_remove(&raw mut (*view).foreign_destroy.link);
}
#[no_mangle]
pub unsafe extern "C" fn view_map(
    mut view: *mut sway_view,
    mut wlr_surface: *mut wlr_surface,
    mut fullscreen: bool,
    mut fullscreen_output: *mut wlr_output,
    mut decoration: bool,
) {
    if !_sway_assert(
        (*view).surface.is_null(),
        b"[%s:%d] %s:cannot map mapped view\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
        814 as ::core::ffi::c_int,
        b"void view_map(struct sway_view *, struct wlr_surface *, _Bool, struct wlr_output *, _Bool)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    (*view).surface = wlr_surface;
    view_populate_pid(view);
    (*view).container = container_create(view);
    if (*view).ctx.is_null() {
        let mut ctx: *mut launcher_ctx = launcher_ctx_find_pid((*view).pid);
        if !ctx.is_null() {
            view_assign_ctx(view, ctx);
        }
    }
    let mut ws: *mut sway_workspace = ::core::ptr::null_mut::<sway_workspace>();
    if !fullscreen_output.is_null() && !(*fullscreen_output).data.is_null() {
        let mut output: *mut sway_output = (*fullscreen_output).data as *mut sway_output;
        ws = output_get_active_workspace(output);
    }
    if ws.is_null() {
        ws = select_workspace(view);
    }
    if !ws.is_null() && !(*ws).output.is_null() {
        let mut scale: ::core::ffi::c_float = (*(*(*ws).output).wlr_output).scale;
        wlr_fractional_scale_v1_notify_scale(
            wlr_surface,
            scale as ::core::ffi::c_double,
        );
        wlr_surface_set_preferred_buffer_scale(
            wlr_surface,
            ceil(scale as ::core::ffi::c_double) as int32_t,
        );
    }
    let mut seat: *mut sway_seat = input_manager_current_seat();
    let mut node: *mut sway_node = seat_get_focus_inactive(
        seat,
        if !ws.is_null() { &raw mut (*ws).node } else { &raw mut (*root).node },
    );
    let mut target_sibling: *mut sway_container = ::core::ptr::null_mut::<
        sway_container,
    >();
    if !node.is_null() && (*node).r#type.0 == sway_node_type::N_CONTAINER.0 {
        if container_is_floating((*node).c2rust_unnamed.sway_container) {
            if !ws.is_null() {
                target_sibling = seat_get_focus_inactive_tiling(seat, ws);
                if !target_sibling.is_null() {
                    let mut con: *mut sway_container = seat_get_focus_inactive_view(
                        seat,
                        &raw mut (*target_sibling).node,
                    );
                    if !con.is_null() {
                        target_sibling = con;
                    }
                }
            } else {
                ws = seat_get_last_known_workspace(seat);
            }
        } else {
            target_sibling = (*node).c2rust_unnamed.sway_container;
        }
    }
    let mut foreign_toplevel_state: wlr_ext_foreign_toplevel_handle_v1_state = wlr_ext_foreign_toplevel_handle_v1_state {
        title: view_get_title(view),
        app_id: view_get_app_id(view),
    };
    (*view).ext_foreign_toplevel = wlr_ext_foreign_toplevel_handle_v1_create(
        server.foreign_toplevel_list,
        &raw mut foreign_toplevel_state,
    );
    (*(*view).ext_foreign_toplevel).data = view as *mut ::core::ffi::c_void;
    (*view).foreign_toplevel = wlr_foreign_toplevel_handle_v1_create(
        server.foreign_toplevel_manager,
    );
    (*view).foreign_activate_request.notify = Some(
        handle_foreign_activate_request
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*view).foreign_toplevel).events.request_activate,
        &raw mut (*view).foreign_activate_request,
    );
    (*view).foreign_fullscreen_request.notify = Some(
        handle_foreign_fullscreen_request
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*view).foreign_toplevel).events.request_fullscreen,
        &raw mut (*view).foreign_fullscreen_request,
    );
    (*view).foreign_close_request.notify = Some(
        handle_foreign_close_request
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*view).foreign_toplevel).events.request_close,
        &raw mut (*view).foreign_close_request,
    );
    (*view).foreign_destroy.notify = Some(
        handle_foreign_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*view).foreign_toplevel).events.destroy,
        &raw mut (*view).foreign_destroy,
    );
    let mut container: *mut sway_container = (*view).container;
    if !target_sibling.is_null() {
        container_add_sibling(target_sibling, container, true);
    } else if !ws.is_null() {
        container = workspace_add_tiling(ws, container);
    }
    ipc_event_window((*view).container, b"new\0".as_ptr() as *const ::core::ffi::c_char);
    if decoration {
        view_update_csd_from_client(view, decoration);
    }
    if (*(*view).r#impl).wants_floating.is_some()
        && (*(*view).r#impl).wants_floating.expect("non-null function pointer")(view)
            as ::core::ffi::c_int != 0
    {
        (*(*view).container).pending.border = (*config).floating_border;
        (*(*view).container).pending.border_thickness = (*config)
            .floating_border_thickness;
        container_set_floating((*view).container, r#true != 0);
    } else {
        (*(*view).container).pending.border = (*config).border;
        (*(*view).container).pending.border_thickness = (*config).border_thickness;
        view_set_tiled(view, r#true != 0);
    }
    if (*config).popup_during_fullscreen.0 == sway_popup_during_fullscreen::POPUP_LEAVE.0
        && !(*container).pending.workspace.is_null()
        && !(*(*container).pending.workspace).fullscreen.is_null()
        && !(*(*(*container).pending.workspace).fullscreen).view.is_null()
    {
        let mut fs: *mut sway_container = (*(*container).pending.workspace).fullscreen;
        if view_is_transient_for(view, (*fs).view) {
            container_set_fullscreen(
                fs,
                sway_fullscreen_mode(r#false as ::core::ffi::c_uint),
            );
        }
    }
    view_update_title(view, r#false != 0);
    container_update_representation(container);
    if fullscreen {
        container_set_fullscreen(
            (*view).container,
            sway_fullscreen_mode(r#true as ::core::ffi::c_uint),
        );
        arrange_workspace((*(*view).container).pending.workspace);
    } else if !(*container).pending.parent.is_null() {
        arrange_container((*container).pending.parent);
    } else if !(*container).pending.workspace.is_null() {
        arrange_workspace((*container).pending.workspace);
    }
    view_execute_criteria(view);
    let mut set_focus: bool = should_focus(view);
    let mut xsurface: *mut wlr_xwayland_surface = ::core::ptr::null_mut::<
        wlr_xwayland_surface,
    >();
    xsurface = wlr_xwayland_surface_try_from_wlr_surface(wlr_surface);
    if !xsurface.is_null() {
        set_focus = set_focus as ::core::ffi::c_int
            & (wlr_xwayland_surface_icccm_input_model(xsurface).0
                != wlr_xwayland_icccm_input_model::WLR_ICCCM_INPUT_MODEL_NONE.0)
                as ::core::ffi::c_int != 0;
    }
    if set_focus {
        input_manager_set_focus(&raw mut (*(*view).container).node);
    }
    if !(*view).ext_foreign_toplevel.is_null() {
        update_ext_foreign_toplevel(view);
    }
    let mut app_id: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut class: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    app_id = view_get_app_id(view);
    if !app_id.is_null() {
        wlr_foreign_toplevel_handle_v1_set_app_id((*view).foreign_toplevel, app_id);
    } else {
        class = view_get_class(view);
        if !class.is_null() {
            wlr_foreign_toplevel_handle_v1_set_app_id((*view).foreign_toplevel, class);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn view_unmap(mut view: *mut sway_view) {
    wl_signal_emit_mutable(
        &raw mut (*view).events.unmap,
        view as *mut ::core::ffi::c_void,
    );
    (*(*view).executed_criteria).length = 0 as ::core::ffi::c_int;
    if !(*view).urgent_timer.is_null() {
        wl_event_source_remove((*view).urgent_timer);
        (*view).urgent_timer = ::core::ptr::null_mut::<wl_event_source>();
    }
    if !(*view).ext_foreign_toplevel.is_null() {
        wlr_ext_foreign_toplevel_handle_v1_destroy((*view).ext_foreign_toplevel);
        (*view).ext_foreign_toplevel = ::core::ptr::null_mut::<
            wlr_ext_foreign_toplevel_handle_v1,
        >();
    }
    if !(*view).foreign_toplevel.is_null() {
        wlr_foreign_toplevel_handle_v1_destroy((*view).foreign_toplevel);
        (*view).foreign_toplevel = ::core::ptr::null_mut::<
            wlr_foreign_toplevel_handle_v1,
        >();
    }
    let mut parent: *mut sway_container = (*(*view).container).pending.parent;
    let mut ws: *mut sway_workspace = (*(*view).container).pending.workspace;
    container_begin_destroy((*view).container);
    if !parent.is_null() {
        container_reap_empty(parent);
    } else if !ws.is_null() {
        workspace_consider_destroy(ws);
    }
    if !(*root).fullscreen_global.is_null() {
        arrange_root();
    } else if !ws.is_null() && !(*ws).node.destroying {
        arrange_workspace(ws);
        workspace_detect_urgent(ws);
    }
    let mut seat: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
    seat = ((*server.input).seats.next as *mut ::core::ffi::c_char)
        .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
        as *mut sway_seat;
    while &raw mut (*seat).link != &raw mut (*server.input).seats {
        (*(*seat).cursor).image_surface = ::core::ptr::null_mut::<wlr_surface>();
        if !(*(*seat).cursor).active_constraint.is_null() {
            let mut constrain_surface: *mut wlr_surface = (*(*(*seat).cursor)
                .active_constraint)
                .surface;
            if view_from_wlr_surface(constrain_surface) == view {
                sway_cursor_constrain(
                    (*seat).cursor,
                    ::core::ptr::null_mut::<wlr_pointer_constraint_v1>(),
                );
            }
        }
        seat_consider_warp_to_focus(seat);
        seat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
            as *mut sway_seat;
    }
    transaction_commit_dirty();
    (*view).surface = ::core::ptr::null_mut::<wlr_surface>();
}
#[no_mangle]
pub unsafe extern "C" fn view_update_size(mut view: *mut sway_view) {
    let mut con: *mut sway_container = (*view).container;
    (*con).pending.content_width = (*view).geometry.width as ::core::ffi::c_double;
    (*con).pending.content_height = (*view).geometry.height as ::core::ffi::c_double;
    container_set_geometry_from_content(con);
}
#[no_mangle]
pub unsafe extern "C" fn view_center_and_clip_surface(mut view: *mut sway_view) {
    let mut con: *mut sway_container = (*view).container;
    let mut clip_to_geometry: bool = r#true != 0;
    if container_is_floating(con) as ::core::ffi::c_int != 0
        || (*con).pending.fullscreen_mode.0 != sway_fullscreen_mode::FULLSCREEN_NONE.0
    {
        let mut x: ::core::ffi::c_int = fmax(
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ((*con).current.content_width
                - (*view).geometry.width as ::core::ffi::c_double)
                / 2 as ::core::ffi::c_int as ::core::ffi::c_double,
        ) as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = fmax(
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ((*con).current.content_height
                - (*view).geometry.height as ::core::ffi::c_double)
                / 2 as ::core::ffi::c_int as ::core::ffi::c_double,
        ) as ::core::ffi::c_int;
        clip_to_geometry = !(*view).using_csd;
        wlr_scene_node_set_position(&raw mut (*(*view).content_tree).node, x, y);
    } else {
        wlr_scene_node_set_position(
            &raw mut (*(*view).content_tree).node,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
    if wl_list_empty(&raw mut (*(*(*con).view).content_tree).children) == 0 {
        let mut clip: wlr_box = wlr_box {
            x: 0 as ::core::ffi::c_int,
            y: 0,
            width: 0,
            height: 0,
        };
        if clip_to_geometry {
            clip = wlr_box {
                x: (*(*con).view).geometry.x,
                y: (*(*con).view).geometry.y,
                width: (*con).current.content_width as ::core::ffi::c_int,
                height: (*con).current.content_height as ::core::ffi::c_int,
            };
        }
        wlr_scene_subsurface_tree_set_clip(
            &raw mut (*(*(*con).view).content_tree).node,
            &raw mut clip,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn view_from_wlr_surface(
    mut wlr_surface: *mut wlr_surface,
) -> *mut sway_view {
    let mut xdg_surface: *mut wlr_xdg_surface = ::core::ptr::null_mut::<
        wlr_xdg_surface,
    >();
    xdg_surface = wlr_xdg_surface_try_from_wlr_surface(wlr_surface);
    if !xdg_surface.is_null() {
        return view_from_wlr_xdg_surface(xdg_surface);
    }
    let mut xsurface: *mut wlr_xwayland_surface = ::core::ptr::null_mut::<
        wlr_xwayland_surface,
    >();
    xsurface = wlr_xwayland_surface_try_from_wlr_surface(wlr_surface);
    if !xsurface.is_null() {
        return view_from_wlr_xwayland_surface(xsurface);
    }
    let mut subsurface: *mut wlr_subsurface = ::core::ptr::null_mut::<wlr_subsurface>();
    subsurface = wlr_subsurface_try_from_wlr_surface(wlr_surface);
    if !subsurface.is_null() {
        return view_from_wlr_surface((*subsurface).parent);
    }
    if !wlr_layer_surface_v1_try_from_wlr_surface(wlr_surface).is_null() {
        return ::core::ptr::null_mut::<sway_view>();
    }
    if !wlr_session_lock_surface_v1_try_from_wlr_surface(wlr_surface).is_null() {
        return ::core::ptr::null_mut::<sway_view>();
    }
    let mut role: *const ::core::ffi::c_char = if !(*wlr_surface).role.is_null() {
        (*(*wlr_surface).role).name
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Surface of unknown type (role %s): %p\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
        1087 as ::core::ffi::c_int,
        role,
        wlr_surface,
    );
    return ::core::ptr::null_mut::<sway_view>();
}
#[no_mangle]
pub unsafe extern "C" fn view_update_app_id(mut view: *mut sway_view) {
    let mut app_id: *const ::core::ffi::c_char = view_get_app_id(view);
    if !(*view).foreign_toplevel.is_null() && !app_id.is_null() {
        wlr_foreign_toplevel_handle_v1_set_app_id((*view).foreign_toplevel, app_id);
    }
    if !(*view).ext_foreign_toplevel.is_null() {
        update_ext_foreign_toplevel(view);
    }
}
#[no_mangle]
pub unsafe extern "C" fn view_update_title(mut view: *mut sway_view, mut force: bool) {
    let mut title: *const ::core::ffi::c_char = view_get_title(view);
    if !force {
        if !title.is_null() && !(*(*view).container).title.is_null()
            && strcmp(title, (*(*view).container).title) == 0 as ::core::ffi::c_int
        {
            return;
        }
        if title.is_null() && (*(*view).container).title.is_null() {
            return;
        }
    }
    free((*(*view).container).title as *mut ::core::ffi::c_void);
    (*(*view).container).title = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*(*view).container).pending.formatted_title as *mut ::core::ffi::c_void);
    (*(*view).container).pending.formatted_title = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut len: size_t = parse_title_format(
        (*view).container,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    if len != 0 {
        let mut buffer: *mut ::core::ffi::c_char = calloc(
            len.wrapping_add(1 as size_t),
            ::core::mem::size_of::<::core::ffi::c_char>(),
        ) as *mut ::core::ffi::c_char;
        if !_sway_assert(
            !buffer.is_null(),
            b"[%s:%d] %s:Unable to allocate title string\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
            1126 as ::core::ffi::c_int,
            b"void view_update_title(struct sway_view *, _Bool)\0".as_ptr()
                as *const ::core::ffi::c_char,
        ) {
            return;
        }
        parse_title_format((*view).container, buffer);
        (*(*view).container).pending.formatted_title = buffer;
    }
    (*(*view).container).title = if !title.is_null() {
        strdup(title)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    node_set_dirty(&raw mut (*(*view).container).node);
    ipc_event_window(
        (*view).container,
        b"title\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !(*view).foreign_toplevel.is_null() && !title.is_null() {
        wlr_foreign_toplevel_handle_v1_set_title((*view).foreign_toplevel, title);
    }
    if !(*view).ext_foreign_toplevel.is_null() {
        update_ext_foreign_toplevel(view);
    }
}
#[no_mangle]
pub unsafe extern "C" fn view_is_visible(mut view: *mut sway_view) -> bool {
    if (*(*view).container).node.destroying {
        return r#false != 0;
    }
    let mut workspace: *mut sway_workspace = (*(*view).container).pending.workspace;
    if workspace.is_null()
        && (*(*view).container).pending.fullscreen_mode.0
            != sway_fullscreen_mode::FULLSCREEN_GLOBAL.0
    {
        let mut fs_global_descendant: bool = r#false != 0;
        let mut parent: *mut sway_container = (*(*view).container).pending.parent;
        while !parent.is_null() {
            if (*parent).pending.fullscreen_mode.0
                == sway_fullscreen_mode::FULLSCREEN_GLOBAL.0
            {
                fs_global_descendant = r#true != 0;
            }
            parent = (*parent).pending.parent;
        }
        if !fs_global_descendant {
            return r#false != 0;
        }
    }
    if !container_is_sticky_or_child((*view).container) && !workspace.is_null()
        && !workspace_is_visible(workspace)
    {
        return r#false != 0;
    }
    let mut seat: *mut sway_seat = input_manager_current_seat();
    let mut con: *mut sway_container = (*view).container;
    while !con.is_null() {
        let mut layout: sway_container_layout = container_parent_layout(con);
        if (layout.0 == sway_container_layout_0::L_TABBED.0
            || layout.0 == sway_container_layout_0::L_STACKED.0)
            && !container_is_floating(con)
        {
            let mut parent_0: *mut sway_node = if !(*con).pending.parent.is_null() {
                &raw mut (*(*con).pending.parent).node
            } else {
                &raw mut (*(*con).pending.workspace).node
            };
            if seat_get_active_tiling_child(seat, parent_0) != &raw mut (*con).node {
                return r#false != 0;
            }
        }
        con = (*con).pending.parent;
    }
    let mut fs: *mut sway_container = if !(*root).fullscreen_global.is_null() {
        (*root).fullscreen_global
    } else {
        (*workspace).fullscreen
    };
    if !fs.is_null() && !container_is_fullscreen_or_child((*view).container)
        && !container_is_transient_for((*view).container, fs)
    {
        return r#false != 0;
    }
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn view_set_urgent(mut view: *mut sway_view, mut enable: bool) {
    if view_is_urgent(view) as ::core::ffi::c_int == enable as ::core::ffi::c_int {
        return;
    }
    if enable {
        let mut seat: *mut sway_seat = input_manager_current_seat();
        if seat_get_focused_container(seat) == (*view).container {
            return;
        }
        clock_gettime(CLOCK_MONOTONIC, &raw mut (*view).urgent);
        container_update_itself_and_parents((*view).container);
    } else {
        (*view).urgent = timespec {
            tv_sec: 0 as __time_t,
            tv_nsec: 0,
        };
        if !(*view).urgent_timer.is_null() {
            wl_event_source_remove((*view).urgent_timer);
            (*view).urgent_timer = ::core::ptr::null_mut::<wl_event_source>();
        }
    }
    ipc_event_window(
        (*view).container,
        b"urgent\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !container_is_scratchpad_hidden_or_child((*view).container) {
        workspace_detect_urgent((*(*view).container).pending.workspace);
    }
}
#[no_mangle]
pub unsafe extern "C" fn view_is_urgent(mut view: *mut sway_view) -> bool {
    return (*view).urgent.tv_sec != 0 || (*view).urgent.tv_nsec != 0;
}
#[no_mangle]
pub unsafe extern "C" fn view_remove_saved_buffer(mut view: *mut sway_view) {
    if !_sway_assert(
        !(*view).saved_surface_tree.is_null(),
        b"[%s:%d] %s:Expected a saved buffer\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
        1228 as ::core::ffi::c_int,
        b"void view_remove_saved_buffer(struct sway_view *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    wlr_scene_node_destroy(&raw mut (*(*view).saved_surface_tree).node);
    (*view).saved_surface_tree = ::core::ptr::null_mut::<wlr_scene_tree>();
    wlr_scene_node_set_enabled(&raw mut (*(*view).content_tree).node, r#true != 0);
}
unsafe extern "C" fn view_save_buffer_iterator(
    mut buffer: *mut wlr_scene_buffer,
    mut sx: ::core::ffi::c_int,
    mut sy: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut tree: *mut wlr_scene_tree = data as *mut wlr_scene_tree;
    let mut sbuf: *mut wlr_scene_buffer = wlr_scene_buffer_create(
        tree,
        ::core::ptr::null_mut::<wlr_buffer>(),
    );
    if sbuf.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Could not allocate a scene buffer when saving a surface\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
            1243 as ::core::ffi::c_int,
        );
        return;
    }
    wlr_scene_buffer_set_dest_size(sbuf, (*buffer).dst_width, (*buffer).dst_height);
    wlr_scene_buffer_set_opaque_region(sbuf, &raw mut (*buffer).opaque_region);
    wlr_scene_buffer_set_opacity(sbuf, (*buffer).opacity);
    wlr_scene_buffer_set_filter_mode(sbuf, (*buffer).filter_mode);
    wlr_scene_buffer_set_transfer_function(sbuf, (*buffer).transfer_function);
    wlr_scene_buffer_set_primaries(sbuf, (*buffer).primaries);
    wlr_scene_buffer_set_source_box(sbuf, &raw mut (*buffer).src_box);
    wlr_scene_node_set_position(&raw mut (*sbuf).node, sx, sy);
    wlr_scene_buffer_set_transform(sbuf, (*buffer).transform);
    wlr_scene_buffer_set_buffer(sbuf, (*buffer).buffer);
}
#[no_mangle]
pub unsafe extern "C" fn view_save_buffer(mut view: *mut sway_view) {
    if !_sway_assert(
        (*view).saved_surface_tree.is_null(),
        b"[%s:%d] %s:Didn't expect saved buffer\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
        1261 as ::core::ffi::c_int,
        b"void view_save_buffer(struct sway_view *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        view_remove_saved_buffer(view);
    }
    (*view).saved_surface_tree = wlr_scene_tree_create((*view).scene_tree);
    if (*view).saved_surface_tree.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Could not allocate a scene tree node when saving a surface\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/tree/view.c\0".as_ptr() as *const ::core::ffi::c_char,
            1267 as ::core::ffi::c_int,
        );
        return;
    }
    wlr_scene_node_place_below(
        &raw mut (*(*view).saved_surface_tree).node,
        &raw mut (*(*view).output_handler).node,
    );
    wlr_scene_node_set_enabled(
        &raw mut (*(*view).saved_surface_tree).node,
        r#false != 0,
    );
    wlr_scene_node_for_each_buffer(
        &raw mut (*(*view).content_tree).node,
        Some(
            view_save_buffer_iterator
                as unsafe extern "C" fn(
                    *mut wlr_scene_buffer,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        (*view).saved_surface_tree as *mut ::core::ffi::c_void,
    );
    wlr_scene_node_set_enabled(&raw mut (*(*view).content_tree).node, r#false != 0);
    wlr_scene_node_set_enabled(&raw mut (*(*view).saved_surface_tree).node, r#true != 0);
}
#[no_mangle]
pub unsafe extern "C" fn view_is_transient_for(
    mut child: *mut sway_view,
    mut ancestor: *mut sway_view,
) -> bool {
    return (*(*child).r#impl).is_transient_for.is_some()
        && (*(*child).r#impl)
            .is_transient_for
            .expect("non-null function pointer")(child, ancestor) as ::core::ffi::c_int
            != 0;
}
#[no_mangle]
pub unsafe extern "C" fn view_can_tear(mut view: *mut sway_view) -> bool {
    match (*view).tearing_mode {
        sway_view_tearing_mode::TEARING_OVERRIDE_FALSE => return r#false != 0,
        sway_view_tearing_mode::TEARING_OVERRIDE_TRUE => return r#true != 0,
        sway_view_tearing_mode::TEARING_WINDOW_HINT => {
            return (*view).tearing_hint.0
                == wp_tearing_control_v1_presentation_hint::WP_TEARING_CONTROL_V1_PRESENTATION_HINT_ASYNC
                    .0;
        }
        _ => {}
    }
    return r#false != 0;
}
unsafe extern "C" fn send_frame_done_iterator(
    mut scene_buffer: *mut wlr_scene_buffer,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut when: *mut timespec = data as *mut timespec;
    let mut scene_surface: *mut wlr_scene_surface = wlr_scene_surface_try_from_buffer(
        scene_buffer,
    );
    if scene_surface.is_null() {
        return;
    }
    wlr_surface_send_frame_done((*scene_surface).surface, when);
}
#[no_mangle]
pub unsafe extern "C" fn view_send_frame_done(mut view: *mut sway_view) {
    let mut when: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(CLOCK_MONOTONIC, &raw mut when);
    let mut node: *mut wlr_scene_node = ::core::ptr::null_mut::<wlr_scene_node>();
    node = ((*(*view).content_tree).children.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut wlr_scene_node
        as *mut wlr_scene_node;
    while &raw mut (*node).link != &raw mut (*(*view).content_tree).children {
        wlr_scene_node_for_each_buffer(
            node,
            Some(
                send_frame_done_iterator
                    as unsafe extern "C" fn(
                        *mut wlr_scene_buffer,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            &raw mut when as *mut ::core::ffi::c_void,
        );
        node = ((*node).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut wlr_scene_node
            as *mut wlr_scene_node;
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __DBL_MAX__: ::core::ffi::c_double = 1.7976931348623157e+308f64;
