extern "C" {
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_swapchain;
    pub type wlr_renderer_impl;
    pub type wlr_allocator;
    pub type wlr_buffer_impl;
    pub type wlr_drm_syncobj_timeline;
    pub type wlr_texture_impl;
    pub type wlr_backend_impl;
    pub type wlr_output_layer_state;
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
    pub type wlr_xcursor_manager;
    pub type xkb_state;
    pub type xkb_keymap;
    pub type wlr_keyboard_group;
    pub type wlr_keyboard_impl;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_primary_selection_source;
    pub type wlr_xwayland_shell_v1;
    pub type wlr_tablet_manager_v2;
    pub type wlr_xdg_shell;
    pub type wlr_idle_notifier_v1;
    pub type wlr_data_device_manager;
    pub type wlr_session;
    pub type sway_text_node;
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
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn fmax(
        __x: ::core::ffi::c_double,
        __y: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    fn wl_signal_emit_mutable(signal: *mut wl_signal, data: *mut ::core::ffi::c_void);
    fn wlr_ext_workspace_manager_v1_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_ext_workspace_manager_v1;
    fn wlr_ext_workspace_group_handle_v1_create(
        manager: *mut wlr_ext_workspace_manager_v1,
        caps: uint32_t,
    ) -> *mut wlr_ext_workspace_group_handle_v1;
    fn wlr_ext_workspace_group_handle_v1_destroy(
        group: *mut wlr_ext_workspace_group_handle_v1,
    );
    fn wlr_ext_workspace_group_handle_v1_output_enter(
        group: *mut wlr_ext_workspace_group_handle_v1,
        output: *mut wlr_output,
    );
    fn wlr_ext_workspace_handle_v1_create(
        manager: *mut wlr_ext_workspace_manager_v1,
        id: *const ::core::ffi::c_char,
        caps: uint32_t,
    ) -> *mut wlr_ext_workspace_handle_v1;
    fn wlr_ext_workspace_handle_v1_destroy(workspace: *mut wlr_ext_workspace_handle_v1);
    fn wlr_ext_workspace_handle_v1_set_group(
        workspace: *mut wlr_ext_workspace_handle_v1,
        group: *mut wlr_ext_workspace_group_handle_v1,
    );
    fn wlr_ext_workspace_handle_v1_set_name(
        workspace: *mut wlr_ext_workspace_handle_v1,
        name: *const ::core::ffi::c_char,
    );
    fn wlr_ext_workspace_handle_v1_set_active(
        workspace: *mut wlr_ext_workspace_handle_v1,
        enabled: bool,
    );
    fn wlr_ext_workspace_handle_v1_set_urgent(
        workspace: *mut wlr_ext_workspace_handle_v1,
        enabled: bool,
    );
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    fn create_list() -> *mut list_t;
    fn list_free(list: *mut list_t);
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn list_insert(
        list: *mut list_t,
        index: ::core::ffi::c_int,
        item: *mut ::core::ffi::c_void,
    );
    fn list_del(list: *mut list_t, index: ::core::ffi::c_int);
    fn list_seq_find(
        list: *mut list_t,
        compare: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        cmp_to: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn list_find(
        list: *mut list_t,
        item: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn list_free_items_and_destroy(list: *mut list_t);
    fn wlr_scene_node_destroy(node: *mut wlr_scene_node);
    fn transaction_commit_dirty();
    fn strip_quotes(str: *mut ::core::ffi::c_char);
    fn argsep(
        stringp: *mut *mut ::core::ffi::c_char,
        delim: *const ::core::ffi::c_char,
        matched_delim: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn has_prefix(
        str: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    ) -> bool;
    static mut root: *mut sway_root;
    fn root_find_workspace(
        test: Option<
            unsafe extern "C" fn(*mut sway_workspace, *mut ::core::ffi::c_void) -> bool,
        >,
        data: *mut ::core::ffi::c_void,
    ) -> *mut sway_workspace;
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn seat_get_focus_inactive_view(
        seat: *mut sway_seat,
        ancestor: *mut sway_node,
    ) -> *mut sway_container;
    fn input_manager_get_default_seat() -> *mut sway_seat;
    fn input_manager_current_seat() -> *mut sway_seat;
    static mut server: sway_server;
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
    fn container_create(view: *mut sway_view) -> *mut sway_container;
    fn container_find_child(
        container: *mut sway_container,
        test: Option<
            unsafe extern "C" fn(*mut sway_container, *mut ::core::ffi::c_void) -> bool,
        >,
        data: *mut ::core::ffi::c_void,
    ) -> *mut sway_container;
    fn container_for_each_child(
        container: *mut sway_container,
        f: Option<
            unsafe extern "C" fn(*mut sway_container, *mut ::core::ffi::c_void) -> (),
        >,
        data: *mut ::core::ffi::c_void,
    );
    fn container_reap_empty(con: *mut sway_container);
    fn container_build_representation(
        layout: sway_container_layout,
        children: *mut list_t,
        buffer: *mut ::core::ffi::c_char,
    ) -> size_t;
    fn container_is_floating_or_child(container: *mut sway_container) -> bool;
    fn container_handle_fullscreen_reparent(con: *mut sway_container);
    fn container_add_child(parent: *mut sway_container, child: *mut sway_container);
    fn container_detach(child: *mut sway_container);
    fn container_split(
        child: *mut sway_container,
        layout: sway_container_layout,
    ) -> *mut sway_container;
    fn container_is_sticky(con: *mut sway_container) -> bool;
    fn container_squash(con: *mut sway_container) -> ::core::ffi::c_int;
    fn view_ancestor_is_only_visible(view: *mut sway_view) -> bool;
    fn view_is_urgent(view: *mut sway_view) -> bool;
    fn do_var_replacement(str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn output_get_identifier(
        identifier: *mut ::core::ffi::c_char,
        len: size_t,
        output: *mut sway_output,
    );
    static mut config: *mut sway_config;
    fn ipc_event_workspace(
        old: *mut sway_workspace,
        new: *mut sway_workspace,
        change: *const ::core::ffi::c_char,
    );
    fn arrange_workspace(workspace: *mut sway_workspace);
    fn arrange_output(output: *mut sway_output);
    fn wrap(i: ::core::ffi::c_int, max: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn output_add_workspace(output: *mut sway_output, workspace: *mut sway_workspace);
    fn output_by_name_or_id(name_or_id: *const ::core::ffi::c_char) -> *mut sway_output;
    fn output_sort_workspaces(output: *mut sway_output);
    fn output_get_active_workspace(output: *mut sway_output) -> *mut sway_workspace;
    fn output_get_default_layout(output: *mut sway_output) -> sway_container_layout;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ext_workspace_group_handle_v1_group_capabilities(pub ::core::ffi::c_uint);
impl ext_workspace_group_handle_v1_group_capabilities {
    pub const EXT_WORKSPACE_GROUP_HANDLE_V1_GROUP_CAPABILITIES_CREATE_WORKSPACE: Self = Self(
        1,
    );
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ext_workspace_handle_v1_workspace_capabilities(pub ::core::ffi::c_uint);
impl ext_workspace_handle_v1_workspace_capabilities {
    pub const EXT_WORKSPACE_HANDLE_V1_WORKSPACE_CAPABILITIES_ACTIVATE: Self = Self(1);
    pub const EXT_WORKSPACE_HANDLE_V1_WORKSPACE_CAPABILITIES_DEACTIVATE: Self = Self(2);
    pub const EXT_WORKSPACE_HANDLE_V1_WORKSPACE_CAPABILITIES_REMOVE: Self = Self(4);
    pub const EXT_WORKSPACE_HANDLE_V1_WORKSPACE_CAPABILITIES_ASSIGN: Self = Self(8);
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
    pub events: C2Rust_Unnamed_9,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
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
pub struct wlr_addon_set {
    pub WLR_PRIVATE: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub addons: wl_list,
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
    pub mastering_luminance: C2Rust_Unnamed_3,
    pub max_cll: ::core::ffi::c_double,
    pub max_fall: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
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
pub struct wlr_renderer {
    pub render_buffer_caps: uint32_t,
    pub color_encodings: uint32_t,
    pub events: C2Rust_Unnamed_6,
    pub features: C2Rust_Unnamed_5,
    pub WLR_PRIVATE: C2Rust_Unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub r#impl: *const wlr_renderer_impl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub input_color_transform: bool,
    pub output_color_transform: bool,
    pub timeline: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
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
    pub events: C2Rust_Unnamed_7,
    pub addons: wlr_addon_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub destroy: wl_signal,
    pub release: wl_signal,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub renderer_destroy: wl_listener,
    pub color_transform: *mut wlr_color_transform,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_texture {
    pub r#impl: *const wlr_texture_impl,
    pub width: uint32_t,
    pub height: uint32_t,
    pub renderer: *mut wlr_renderer,
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
pub struct wlr_fbox {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
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
    pub features: C2Rust_Unnamed_11,
    pub events: C2Rust_Unnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
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
    pub custom_mode: C2Rust_Unnamed_12,
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
pub struct C2Rust_Unnamed_12 {
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
pub struct wlr_box {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_ext_workspace_v1_request_type(pub ::core::ffi::c_uint);
impl wlr_ext_workspace_v1_request_type {
    pub const WLR_EXT_WORKSPACE_V1_REQUEST_CREATE_WORKSPACE: Self = Self(0);
    pub const WLR_EXT_WORKSPACE_V1_REQUEST_ACTIVATE: Self = Self(1);
    pub const WLR_EXT_WORKSPACE_V1_REQUEST_DEACTIVATE: Self = Self(2);
    pub const WLR_EXT_WORKSPACE_V1_REQUEST_ASSIGN: Self = Self(3);
    pub const WLR_EXT_WORKSPACE_V1_REQUEST_REMOVE: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_v1_request {
    pub r#type: wlr_ext_workspace_v1_request_type,
    pub link: wl_list,
    pub c2rust_unnamed: C2Rust_Unnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_13 {
    pub create_workspace: C2Rust_Unnamed_24,
    pub activate: C2Rust_Unnamed_23,
    pub deactivate: C2Rust_Unnamed_22,
    pub assign: C2Rust_Unnamed_21,
    pub remove: C2Rust_Unnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub workspace: *mut wlr_ext_workspace_handle_v1,
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
    pub events: C2Rust_Unnamed_16,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_group_handle_v1 {
    pub manager: *mut wlr_ext_workspace_manager_v1,
    pub caps: uint32_t,
    pub events: C2Rust_Unnamed_18,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub outputs: wl_list,
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_manager_v1 {
    pub global: *mut wl_global,
    pub groups: wl_list,
    pub workspaces: wl_list,
    pub events: C2Rust_Unnamed_20,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub resources: wl_list,
    pub idle_source: *mut wl_event_source,
    pub event_loop: *mut wl_event_loop,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub commit: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub workspace: *mut wlr_ext_workspace_handle_v1,
    pub group: *mut wlr_ext_workspace_group_handle_v1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub workspace: *mut wlr_ext_workspace_handle_v1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub workspace: *mut wlr_ext_workspace_handle_v1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub name: *mut ::core::ffi::c_char,
    pub group: *mut wlr_ext_workspace_group_handle_v1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_v1_commit_event {
    pub requests: *mut wl_list,
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon {
    pub r#impl: *const wlr_addon_interface,
    pub WLR_PRIVATE: C2Rust_Unnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_client_buffer {
    pub base: wlr_buffer,
    pub texture: *mut wlr_texture,
    pub source: *mut wlr_buffer,
    pub WLR_PRIVATE: C2Rust_Unnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub source_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
    pub n_ignore_locks: size_t,
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
    pub events: C2Rust_Unnamed_30,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_29,
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
pub struct C2Rust_Unnamed_29 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
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
    pub viewport: C2Rust_Unnamed_31,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
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
    pub events: C2Rust_Unnamed_33,
    pub WLR_PRIVATE: C2Rust_Unnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_35,
    pub WLR_PRIVATE: C2Rust_Unnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
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
pub struct C2Rust_Unnamed_35 {
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
    pub events: C2Rust_Unnamed_39,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
    pub synced: wlr_surface_synced,
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
pub struct C2Rust_Unnamed_39 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_40,
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
pub struct C2Rust_Unnamed_40 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_42,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_41 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
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
    pub events: C2Rust_Unnamed_44,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_43,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_44 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_45,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_45 {
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
    pub events: C2Rust_Unnamed_50,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_46,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
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
    pub events: C2Rust_Unnamed_48,
    pub WLR_PRIVATE: C2Rust_Unnamed_47,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
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
pub struct C2Rust_Unnamed_48 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_49,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
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
pub struct C2Rust_Unnamed_50 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_52,
    pub events: C2Rust_Unnamed_51,
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
pub struct C2Rust_Unnamed_51 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_52 {
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
    pub events: C2Rust_Unnamed_54,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_53,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_53 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_54 {
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
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_102,
    pub border: C2Rust_Unnamed_101,
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
    pub events: C2Rust_Unnamed_55,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
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
    pub layers: C2Rust_Unnamed_100,
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
    pub layers: C2Rust_Unnamed_95,
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
    pub session_lock: C2Rust_Unnamed_62,
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
    pub new_object: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_text_input_manager_v3 {
    pub global: *mut wl_global,
    pub text_inputs: wl_list,
    pub events: C2Rust_Unnamed_59,
    pub WLR_PRIVATE: C2Rust_Unnamed_58,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_58 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_59 {
    pub new_text_input: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_manager_v2 {
    pub global: *mut wl_global,
    pub input_methods: wl_list,
    pub events: C2Rust_Unnamed_61,
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
    pub new_input_method: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
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
    pub events: C2Rust_Unnamed_67,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_65,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_65 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_66,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
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
pub struct C2Rust_Unnamed_67 {
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
    pub events: C2Rust_Unnamed_69,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_68,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
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
    pub events: C2Rust_Unnamed_71,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_70,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_71 {
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
    pub events: C2Rust_Unnamed_73,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_72,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
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
pub struct C2Rust_Unnamed_73 {
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
    pub events: C2Rust_Unnamed_75,
    pub WLR_PRIVATE: C2Rust_Unnamed_74,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_74 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_75 {
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
    pub repeat_info: C2Rust_Unnamed_77,
    pub events: C2Rust_Unnamed_76,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_76 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_77 {
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
    pub events: C2Rust_Unnamed_78,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_78 {
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
    pub events: C2Rust_Unnamed_80,
    pub WLR_PRIVATE: C2Rust_Unnamed_79,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_79 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_80 {
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
    pub events: C2Rust_Unnamed_82,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_81,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_81 {
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_82 {
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
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_pointers: wl_list,
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
    pub new_virtual_pointer: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_keyboards: wl_list,
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
    pub new_virtual_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
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
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_95 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_97,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_96,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_96 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_97 {
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
    pub layers: C2Rust_Unnamed_99,
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
    pub events: C2Rust_Unnamed_98,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_98 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_99 {
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
pub struct C2Rust_Unnamed_100 {
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
pub struct C2Rust_Unnamed_101 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_102 {
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
pub struct sway_cursor {
    pub seat: *mut sway_seat,
    pub cursor: *mut wlr_cursor,
    pub previous: C2Rust_Unnamed_112,
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
    pub events: C2Rust_Unnamed_110,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_109,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_109 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub synced: wlr_surface_synced,
    pub destroying: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_110 {
    pub set_region: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_constraint_v1_state {
    pub committed: uint32_t,
    pub region: pixman_region32_t,
    pub cursor_hint: C2Rust_Unnamed_111,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_111 {
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
pub struct C2Rust_Unnamed_112 {
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
    pub xcursor_theme: C2Rust_Unnamed_113,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_113 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct workspace_config {
    pub workspace: *mut ::core::ffi::c_char,
    pub outputs: *mut list_t,
    pub gaps_inner: ::core::ffi::c_int,
    pub gaps_outer: side_gaps,
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
    pub colors: C2Rust_Unnamed_114,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_114 {
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
    pub border_colors: C2Rust_Unnamed_117,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_115,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_115 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_116,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_116 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_117 {
    pub focused: border_colors,
    pub focused_inactive: border_colors,
    pub focused_tab_title: border_colors,
    pub unfocused: border_colors,
    pub urgent: border_colors,
    pub placeholder: border_colors,
    pub background: [::core::ffi::c_float; 4],
}
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_signal_add(
    mut signal: *mut wl_signal,
    mut listener: *mut wl_listener,
) {
    wl_list_insert((*signal).listener_list.prev, &raw mut (*listener).link);
}
pub const MIN_SANE_W: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const MIN_SANE_H: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
static mut WORKSPACE_CAPABILITIES: uint32_t = (ext_workspace_handle_v1_workspace_capabilities::EXT_WORKSPACE_HANDLE_V1_WORKSPACE_CAPABILITIES_ACTIVATE
    .0 as ::core::ffi::c_int
    | ext_workspace_handle_v1_workspace_capabilities::EXT_WORKSPACE_HANDLE_V1_WORKSPACE_CAPABILITIES_ASSIGN
        .0 as ::core::ffi::c_int) as uint32_t;
static mut GROUP_CAPABILITIES: uint32_t = ext_workspace_group_handle_v1_group_capabilities::EXT_WORKSPACE_GROUP_HANDLE_V1_GROUP_CAPABILITIES_CREATE_WORKSPACE
    .0 as ::core::ffi::c_int as uint32_t;
unsafe extern "C" fn group_to_output(
    mut group: *mut wlr_ext_workspace_group_handle_v1,
) -> *mut sway_output {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*root).outputs).length {
        let mut output: *mut sway_output = *(*(*root).outputs).items.offset(i as isize)
            as *mut sway_output;
        if (*output).ext_workspace_group == group {
            return output;
        }
        i += 1;
    }
    abort();
}
unsafe extern "C" fn handle_commit(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut server_0: *mut sway_server = (listener as *mut ::core::ffi::c_char)
        .offset(-(1056 as ::core::ffi::c_ulong as isize)) as *mut sway_server;
    let mut event: *mut wlr_ext_workspace_v1_commit_event = data
        as *mut wlr_ext_workspace_v1_commit_event;
    let mut req: *mut wlr_ext_workspace_v1_request = ::core::ptr::null_mut::<
        wlr_ext_workspace_v1_request,
    >();
    let mut tmp: *mut wlr_ext_workspace_v1_request = ::core::ptr::null_mut::<
        wlr_ext_workspace_v1_request,
    >();
    req = ((*(*event).requests).next as *mut ::core::ffi::c_char)
        .offset(-(8 as ::core::ffi::c_ulong as isize))
        as *mut wlr_ext_workspace_v1_request as *mut wlr_ext_workspace_v1_request;
    tmp = ((*req).link.next as *mut ::core::ffi::c_char)
        .offset(-(8 as ::core::ffi::c_ulong as isize))
        as *mut wlr_ext_workspace_v1_request as *mut wlr_ext_workspace_v1_request;
    while &raw mut (*req).link != (*event).requests {
        let mut output: *mut sway_output = ::core::ptr::null_mut::<sway_output>();
        let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut new_ws: *mut sway_workspace = ::core::ptr::null_mut::<sway_workspace>();
        's_125: {
            match (*req).r#type {
                wlr_ext_workspace_v1_request_type::WLR_EXT_WORKSPACE_V1_REQUEST_ACTIVATE => {
                    if !(*req).c2rust_unnamed.activate.workspace.is_null() {
                        workspace_switch(
                            (*(*req).c2rust_unnamed.activate.workspace).data
                                as *mut sway_workspace,
                        );
                    }
                }
                wlr_ext_workspace_v1_request_type::WLR_EXT_WORKSPACE_V1_REQUEST_CREATE_WORKSPACE => {
                    output = group_to_output(
                        (*req).c2rust_unnamed.create_workspace.group,
                    );
                    _sway_assert(
                        !output.is_null(),
                        b"[%s:%d] %s:NULL output given to create_workspace\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/tree/workspace.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        61 as ::core::ffi::c_int,
                        b"void handle_commit(struct wl_listener *, void *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    if !(*req).c2rust_unnamed.create_workspace.name.is_null() {
                        if !workspace_by_name(
                                (*req).c2rust_unnamed.create_workspace.name,
                            )
                            .is_null()
                        {
                            _sway_log(
                                sway_log_importance_t::SWAY_ERROR,
                                b"[%s:%d] Refusing to create workspace with duplicate name.\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"sway/tree/workspace.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                66 as ::core::ffi::c_int,
                            );
                            break 's_125;
                        } else {
                            name = strdup((*req).c2rust_unnamed.create_workspace.name);
                        }
                    } else {
                        name = workspace_next_name((*(*output).wlr_output).name);
                    }
                    new_ws = workspace_create(output, name);
                    if !new_ws.is_null() {
                        workspace_switch(new_ws);
                    }
                    free(name as *mut ::core::ffi::c_void);
                }
                wlr_ext_workspace_v1_request_type::WLR_EXT_WORKSPACE_V1_REQUEST_ASSIGN => {
                    if !((*req).c2rust_unnamed.assign.workspace.is_null()
                        || (*req).c2rust_unnamed.assign.group.is_null())
                    {
                        let mut ws: *mut sway_workspace = (*(*req)
                            .c2rust_unnamed
                            .assign
                            .workspace)
                            .data as *mut sway_workspace;
                        let mut new_output: *mut sway_output = group_to_output(
                            (*req).c2rust_unnamed.assign.group,
                        );
                        let mut old_output: *mut sway_output = (*ws).output;
                        workspace_move_to_output(ws, new_output);
                        arrange_output(old_output);
                        arrange_output(new_output);
                    }
                }
                wlr_ext_workspace_v1_request_type::WLR_EXT_WORKSPACE_V1_REQUEST_DEACTIVATE
                | wlr_ext_workspace_v1_request_type::WLR_EXT_WORKSPACE_V1_REQUEST_REMOVE
                | _ => {}
            }
        }
        req = tmp;
        tmp = ((*req).link.next as *mut ::core::ffi::c_char)
            .offset(-(8 as ::core::ffi::c_ulong as isize))
            as *mut wlr_ext_workspace_v1_request as *mut wlr_ext_workspace_v1_request;
    }
    transaction_commit_dirty();
}
#[no_mangle]
pub unsafe extern "C" fn sway_ext_workspace_init() -> bool {
    server.workspace_manager_v1 = wlr_ext_workspace_manager_v1_create(
        server.wl_display,
        1 as uint32_t,
    );
    if server.workspace_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create ext_workspace_manager_v1\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
            104 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    server.workspace_manager_v1_commit.notify = Some(
        handle_commit
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*server.workspace_manager_v1).events.commit,
        &raw mut server.workspace_manager_v1_commit,
    );
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn sway_ext_workspace_output_enable(mut output: *mut sway_output) {
    if (*output).wlr_output.is_null() {
        return;
    }
    (*output).ext_workspace_group = wlr_ext_workspace_group_handle_v1_create(
        server.workspace_manager_v1,
        GROUP_CAPABILITIES,
    );
    if (*output).ext_workspace_group.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create workspace group for output '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
            124 as ::core::ffi::c_int,
            (*(*output).wlr_output).name,
        );
        return;
    }
    wlr_ext_workspace_group_handle_v1_output_enter(
        (*output).ext_workspace_group,
        (*output).wlr_output,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sway_ext_workspace_output_disable(
    mut output: *mut sway_output,
) {
    if (*output).ext_workspace_group.is_null() {
        return;
    }
    wlr_ext_workspace_group_handle_v1_destroy((*output).ext_workspace_group);
    (*output).ext_workspace_group = ::core::ptr::null_mut::<
        wlr_ext_workspace_group_handle_v1,
    >();
}
#[no_mangle]
pub unsafe extern "C" fn workspace_find_config(
    mut ws_name: *const ::core::ffi::c_char,
) -> *mut workspace_config {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*config).workspace_configs).length {
        let mut wsc: *mut workspace_config = *(*(*config).workspace_configs)
            .items
            .offset(i as isize) as *mut workspace_config;
        if strcmp((*wsc).workspace, ws_name) == 0 as ::core::ffi::c_int {
            return wsc;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<workspace_config>();
}
#[no_mangle]
pub unsafe extern "C" fn workspace_get_initial_output(
    mut name: *const ::core::ffi::c_char,
) -> *mut sway_output {
    let mut wsc: *mut workspace_config = workspace_find_config(name);
    if !wsc.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*wsc).outputs).length {
            let mut output: *mut sway_output = output_by_name_or_id(
                *(*(*wsc).outputs).items.offset(i as isize) as *const ::core::ffi::c_char,
            );
            if !output.is_null() {
                return output;
            }
            i += 1;
        }
    }
    let mut seat: *mut sway_seat = input_manager_current_seat();
    let mut focus: *mut sway_node = seat_get_focus_inactive(seat, &raw mut (*root).node);
    if !focus.is_null() && (*focus).r#type.0 == sway_node_type::N_WORKSPACE.0 {
        return (*(*focus).c2rust_unnamed.sway_workspace).output
    } else if !focus.is_null() && (*focus).r#type.0 == sway_node_type::N_CONTAINER.0 {
        return (*(*(*focus).c2rust_unnamed.sway_container).pending.workspace).output
    }
    return (if (*(*root).outputs).length != 0 {
        *(*(*root).outputs).items.offset(0isize)
    } else {
        (*root).fallback_output as *mut ::core::ffi::c_void
    }) as *mut sway_output;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_create(
    mut output: *mut sway_output,
    mut name: *const ::core::ffi::c_char,
) -> *mut sway_workspace {
    _sway_assert(
        !name.is_null(),
        b"[%s:%d] %s:NULL name given to workspace_create\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
        179 as ::core::ffi::c_int,
        b"struct sway_workspace *workspace_create(struct sway_output *, const char *)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    if output.is_null() {
        output = workspace_get_initial_output(name);
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Adding workspace %s for output %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
        185 as ::core::ffi::c_int,
        name,
        (*(*output).wlr_output).name,
    );
    let mut ws: *mut sway_workspace = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_workspace>(),
    ) as *mut sway_workspace;
    if ws.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to allocate sway_workspace\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
            190 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<sway_workspace>();
    }
    (*ws).ext_workspace = wlr_ext_workspace_handle_v1_create(
        server.workspace_manager_v1,
        ::core::ptr::null::<::core::ffi::c_char>(),
        WORKSPACE_CAPABILITIES,
    );
    if (*ws).ext_workspace.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create ext_workspace for '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
            197 as ::core::ffi::c_int,
            name,
        );
        free(ws as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<sway_workspace>();
    }
    (*(*ws).ext_workspace).data = ws as *mut ::core::ffi::c_void;
    node_init(
        &raw mut (*ws).node,
        sway_node_type::N_WORKSPACE,
        ws as *mut ::core::ffi::c_void,
    );
    let mut failed: bool = r#false != 0;
    (*ws).layers.tiling = alloc_scene_tree((*root).staging, &raw mut failed);
    (*ws).layers.fullscreen = alloc_scene_tree((*root).staging, &raw mut failed);
    if failed {
        wlr_scene_node_destroy(&raw mut (*(*ws).layers.tiling).node);
        wlr_scene_node_destroy(&raw mut (*(*ws).layers.fullscreen).node);
        wlr_ext_workspace_handle_v1_destroy((*ws).ext_workspace);
        free(ws as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<sway_workspace>();
    }
    (*ws).name = strdup(name);
    (*ws).prev_split_layout = sway_container_layout(
        sway_container_layout_0::L_NONE.0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*ws).layout = output_get_default_layout(output);
    (*ws).floating = create_list();
    (*ws).tiling = create_list();
    (*ws).output_priority = create_list();
    (*ws).gaps_outer = (*config).gaps_outer;
    (*ws).gaps_inner = (*config).gaps_inner;
    if !name.is_null() {
        let mut wsc: *mut workspace_config = workspace_find_config(name);
        if !wsc.is_null() {
            if (*wsc).gaps_outer.top != INT_MIN {
                (*ws).gaps_outer.top = (*wsc).gaps_outer.top;
            }
            if (*wsc).gaps_outer.right != INT_MIN {
                (*ws).gaps_outer.right = (*wsc).gaps_outer.right;
            }
            if (*wsc).gaps_outer.bottom != INT_MIN {
                (*ws).gaps_outer.bottom = (*wsc).gaps_outer.bottom;
            }
            if (*wsc).gaps_outer.left != INT_MIN {
                (*ws).gaps_outer.left = (*wsc).gaps_outer.left;
            }
            if (*wsc).gaps_inner != INT_MIN {
                (*ws).gaps_inner = (*wsc).gaps_inner;
            }
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*(*wsc).outputs).length {
                let mut name_0: *mut ::core::ffi::c_char = *(*(*wsc).outputs)
                    .items
                    .offset(i as isize) as *mut ::core::ffi::c_char;
                if strcmp(name_0, b"*\0".as_ptr() as *const ::core::ffi::c_char)
                    != 0 as ::core::ffi::c_int
                {
                    list_add(
                        (*ws).output_priority,
                        strdup(name_0) as *mut ::core::ffi::c_void,
                    );
                }
                i += 1;
            }
        }
    }
    workspace_output_add_priority(ws, output);
    output_add_workspace(output, ws);
    output_sort_workspaces(output);
    wlr_ext_workspace_handle_v1_set_name((*ws).ext_workspace, (*ws).name);
    if !(*ws).output.is_null() && !(*(*ws).output).ext_workspace_group.is_null() {
        wlr_ext_workspace_handle_v1_set_group(
            (*ws).ext_workspace,
            (*(*ws).output).ext_workspace_group,
        );
    }
    wlr_ext_workspace_handle_v1_set_active(
        (*ws).ext_workspace,
        workspace_is_visible(ws),
    );
    ipc_event_workspace(
        ::core::ptr::null_mut::<sway_workspace>(),
        ws,
        b"init\0".as_ptr() as *const ::core::ffi::c_char,
    );
    wl_signal_emit_mutable(
        &raw mut (*root).events.new_node,
        &raw mut (*ws).node as *mut ::core::ffi::c_void,
    );
    return ws;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_destroy(mut workspace: *mut sway_workspace) {
    if !_sway_assert(
        (*workspace).node.destroying,
        b"[%s:%d] %s:Tried to free workspace which wasn't marked as destroying\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
        275 as ::core::ffi::c_int,
        b"void workspace_destroy(struct sway_workspace *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    if !_sway_assert(
        (*workspace).node.ntxnrefs == 0 as size_t,
        b"[%s:%d] %s:Tried to free workspace which is still referenced by transactions\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
        279 as ::core::ffi::c_int,
        b"void workspace_destroy(struct sway_workspace *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    scene_node_disown_children((*workspace).layers.tiling);
    scene_node_disown_children((*workspace).layers.fullscreen);
    wlr_scene_node_destroy(&raw mut (*(*workspace).layers.tiling).node);
    wlr_scene_node_destroy(&raw mut (*(*workspace).layers.fullscreen).node);
    free((*workspace).name as *mut ::core::ffi::c_void);
    free((*workspace).representation as *mut ::core::ffi::c_void);
    list_free_items_and_destroy((*workspace).output_priority);
    list_free((*workspace).floating);
    list_free((*workspace).tiling);
    list_free((*workspace).current.floating);
    list_free((*workspace).current.tiling);
    free(workspace as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn workspace_begin_destroy(mut workspace: *mut sway_workspace) {
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Destroying workspace '%s'\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
        300 as ::core::ffi::c_int,
        (*workspace).name,
    );
    ipc_event_workspace(
        ::core::ptr::null_mut::<sway_workspace>(),
        workspace,
        b"empty\0".as_ptr() as *const ::core::ffi::c_char,
    );
    wl_signal_emit_mutable(
        &raw mut (*workspace).node.events.destroy,
        &raw mut (*workspace).node as *mut ::core::ffi::c_void,
    );
    wlr_ext_workspace_handle_v1_destroy((*workspace).ext_workspace);
    (*workspace).ext_workspace = ::core::ptr::null_mut::<wlr_ext_workspace_handle_v1>();
    if !(*workspace).output.is_null() {
        workspace_detach(workspace);
    }
    node_set_dirty(&raw mut (*workspace).node);
    (*workspace).node.destroying = r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_consider_destroy(mut ws: *mut sway_workspace) {
    if (*(*ws).tiling).length != 0 || (*(*ws).floating).length != 0 {
        return;
    }
    if !(*ws).output.is_null() && output_get_active_workspace((*ws).output) == ws {
        return;
    }
    let mut seat: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
    seat = ((*server.input).seats.next as *mut ::core::ffi::c_char)
        .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
        as *mut sway_seat;
    while &raw mut (*seat).link != &raw mut (*server.input).seats {
        let mut node: *mut sway_node = seat_get_focus_inactive(
            seat,
            &raw mut (*root).node,
        );
        if node == &raw mut (*ws).node {
            return;
        }
        seat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
            as *mut sway_seat;
    }
    workspace_begin_destroy(ws);
}
unsafe extern "C" fn workspace_valid_on_output(
    mut output_name: *const ::core::ffi::c_char,
    mut ws_name: *const ::core::ffi::c_char,
) -> bool {
    let mut wsc: *mut workspace_config = workspace_find_config(ws_name);
    let mut output: *mut sway_output = output_by_name_or_id(output_name);
    if output.is_null() {
        return r#false != 0;
    }
    if wsc.is_null() {
        return r#true != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*wsc).outputs).length {
        let mut ws_output: *mut sway_output = output_by_name_or_id(
            *(*(*wsc).outputs).items.offset(i as isize) as *const ::core::ffi::c_char,
        );
        if !ws_output.is_null() {
            return ws_output == output;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn workspace_name_from_binding(
    mut binding: *const sway_binding,
    mut output_name: *const ::core::ffi::c_char,
    mut min_order: *mut ::core::ffi::c_int,
    mut earliest_name: *mut *mut ::core::ffi::c_char,
) {
    let mut cmdlist: *mut ::core::ffi::c_char = strdup((*binding).command);
    let mut dup: *mut ::core::ffi::c_char = cmdlist;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut cmd: *mut ::core::ffi::c_char = argsep(
        &raw mut cmdlist,
        b" \0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    if !cmdlist.is_null() {
        name = argsep(
            &raw mut cmdlist,
            b",;\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
    }
    if strcmp(b"workspace\0".as_ptr() as *const ::core::ffi::c_char, cmd)
        == 0 as ::core::ffi::c_int && !name.is_null()
    {
        let mut _target: *mut ::core::ffi::c_char = strdup(name);
        _target = do_var_replacement(_target);
        strip_quotes(_target);
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Got valid workspace command for target: '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
            374 as ::core::ffi::c_int,
            _target,
        );
        if strcmp(_target, b"next\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(_target, b"prev\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(
                _target,
                b"next_on_output\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                _target,
                b"prev_on_output\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(_target, b"number\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(
                _target,
                b"back_and_forth\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(_target, b"current\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            free(_target as *mut ::core::ffi::c_void);
            free(dup as *mut ::core::ffi::c_void);
            return;
        }
        if has_prefix(_target, b"number \0".as_ptr() as *const ::core::ffi::c_char) {
            let mut length: size_t = strlen(_target)
                .wrapping_sub(
                    strlen(b"number \0".as_ptr() as *const ::core::ffi::c_char),
                )
                .wrapping_add(1 as size_t);
            let mut temp: *mut ::core::ffi::c_char = malloc(length)
                as *mut ::core::ffi::c_char;
            strncpy(
                temp,
                _target
                    .offset(
                        strlen(b"number \0".as_ptr() as *const ::core::ffi::c_char)
                            as isize,
                    ),
                length.wrapping_sub(1 as size_t),
            );
            *temp.offset(length.wrapping_sub(1 as size_t) as isize) = '\0'
                as ::core::ffi::c_char;
            free(_target as *mut ::core::ffi::c_void);
            _target = temp;
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Isolated name from workspace number: '%s'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
                399 as ::core::ffi::c_int,
                _target,
            );
            if *(*__ctype_b_loc())
                .offset(*_target.offset(0isize) as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int
                    as ::core::ffi::c_ushort as ::core::ffi::c_int != 0
                && !workspace_by_number(_target).is_null()
            {
                free(_target as *mut ::core::ffi::c_void);
                free(dup as *mut ::core::ffi::c_void);
                return;
            }
        }
        if !workspace_by_name(_target).is_null() {
            free(_target as *mut ::core::ffi::c_void);
            free(dup as *mut ::core::ffi::c_void);
            return;
        }
        if !workspace_valid_on_output(output_name, _target) {
            free(_target as *mut ::core::ffi::c_void);
            free(dup as *mut ::core::ffi::c_void);
            return;
        }
        if (*binding).order < *min_order {
            *min_order = (*binding).order;
            free(*earliest_name as *mut ::core::ffi::c_void);
            *earliest_name = _target;
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Workspace: Found free name %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
                428 as ::core::ffi::c_int,
                _target,
            );
        } else {
            free(_target as *mut ::core::ffi::c_void);
        }
    }
    free(dup as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn workspace_next_name(
    mut output_name: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Workspace: Generating new workspace name for output %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
        437 as ::core::ffi::c_int,
        output_name,
    );
    let mut mode: *mut sway_mode = (*config).current_mode;
    let mut output: *mut sway_output = output_by_name_or_id(output_name);
    if output.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut order: ::core::ffi::c_int = INT_MAX;
    let mut target: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*mode).keysym_bindings).length {
        workspace_name_from_binding(
            *(*(*mode).keysym_bindings).items.offset(i as isize) as *const sway_binding,
            output_name,
            &raw mut order,
            &raw mut target,
        );
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*(*mode).keycode_bindings).length {
        workspace_name_from_binding(
            *(*(*mode).keycode_bindings).items.offset(i_0 as isize)
                as *const sway_binding,
            output_name,
            &raw mut order,
            &raw mut target,
        );
        i_0 += 1;
    }
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < (*(*config).workspace_configs).length {
        let mut wsc: *const workspace_config = *(*(*config).workspace_configs)
            .items
            .offset(i_1 as isize) as *const workspace_config;
        if workspace_by_name((*wsc).workspace).is_null() {
            let mut found: bool = r#false != 0;
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < (*(*wsc).outputs).length {
                let mut ws_output: *mut sway_output = output_by_name_or_id(
                    *(*(*wsc).outputs).items.offset(j as isize)
                        as *const ::core::ffi::c_char,
                );
                if !ws_output.is_null() {
                    if ws_output == output {
                        found = r#true != 0;
                        free(target as *mut ::core::ffi::c_void);
                        target = strdup((*wsc).workspace);
                    }
                    break;
                } else {
                    j += 1;
                }
            }
            if found {
                break;
            }
        }
        i_1 += 1;
    }
    if !target.is_null() {
        return target;
    }
    let mut name: [::core::ffi::c_char; 12] = ::core::mem::transmute::<
        [u8; 12],
        [::core::ffi::c_char; 12],
    >(*b"\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut ws_num: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
    loop {
        let c2rust_fresh0 = ws_num;
        ws_num = ws_num.wrapping_add(1);
        snprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 12]>(),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            c2rust_fresh0,
        );
        if workspace_by_number(&raw mut name as *mut ::core::ffi::c_char).is_null() {
            break;
        }
    }
    return strdup(&raw mut name as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn _workspace_by_number(
    mut ws: *mut sway_workspace,
    mut data: *mut ::core::ffi::c_void,
) -> bool {
    let mut name: *mut ::core::ffi::c_char = data as *mut ::core::ffi::c_char;
    let mut ws_name: *mut ::core::ffi::c_char = (*ws).name;
    while *(*__ctype_b_loc()).offset(*name as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int != 0
    {
        let c2rust_fresh1 = name;
        name = name.offset(1);
        let c2rust_fresh2 = ws_name;
        ws_name = ws_name.offset(1);
        if *c2rust_fresh1 as ::core::ffi::c_int != *c2rust_fresh2 as ::core::ffi::c_int {
            return r#false != 0;
        }
    }
    return *(*__ctype_b_loc()).offset(*ws_name as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int == 0;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_by_number(
    mut name: *const ::core::ffi::c_char,
) -> *mut sway_workspace {
    return root_find_workspace(
        Some(
            _workspace_by_number
                as unsafe extern "C" fn(
                    *mut sway_workspace,
                    *mut ::core::ffi::c_void,
                ) -> bool,
        ),
        name as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn _workspace_by_name(
    mut ws: *mut sway_workspace,
    mut data: *mut ::core::ffi::c_void,
) -> bool {
    return strcasecmp((*ws).name, data as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_by_name(
    mut name: *const ::core::ffi::c_char,
) -> *mut sway_workspace {
    let mut seat: *mut sway_seat = input_manager_current_seat();
    let mut current: *mut sway_workspace = seat_get_focused_workspace(seat);
    if !current.is_null()
        && strcmp(name, b"prev\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return workspace_prev(current)
    } else if !current.is_null()
        && strcmp(name, b"prev_on_output\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return workspace_output_prev(current)
    } else if !current.is_null()
        && strcmp(name, b"next\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return workspace_next(current)
    } else if !current.is_null()
        && strcmp(name, b"next_on_output\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return workspace_output_next(current)
    } else if strcmp(name, b"current\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return current
    } else if strcasecmp(
        name,
        b"back_and_forth\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut seat_0: *mut sway_seat = input_manager_current_seat();
        if (*seat_0).prev_workspace_name.is_null() {
            return ::core::ptr::null_mut::<sway_workspace>();
        }
        return root_find_workspace(
            Some(
                _workspace_by_name
                    as unsafe extern "C" fn(
                        *mut sway_workspace,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
            ),
            (*seat_0).prev_workspace_name as *mut ::core::ffi::c_void,
        );
    } else {
        return root_find_workspace(
            Some(
                _workspace_by_name
                    as unsafe extern "C" fn(
                        *mut sway_workspace,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
            ),
            name as *mut ::core::ffi::c_void,
        )
    };
}
unsafe extern "C" fn workspace_get_number(
    mut workspace: *mut sway_workspace,
) -> ::core::ffi::c_int {
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_longlong = strtoll(
        (*workspace).name,
        &raw mut endptr,
        10 as ::core::ffi::c_int,
    );
    if *__errno_location() != 0 as ::core::ffi::c_int
        || n > INT32_MAX as ::core::ffi::c_longlong || n < 0 as ::core::ffi::c_longlong
        || endptr == (*workspace).name
    {
        n = -1 as ::core::ffi::c_longlong;
    }
    return n as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_prev(
    mut workspace: *mut sway_workspace,
) -> *mut sway_workspace {
    let mut n: ::core::ffi::c_int = workspace_get_number(workspace);
    let mut prev: *mut sway_workspace = ::core::ptr::null_mut::<sway_workspace>();
    let mut last: *mut sway_workspace = ::core::ptr::null_mut::<sway_workspace>();
    let mut other: *mut sway_workspace = ::core::ptr::null_mut::<sway_workspace>();
    let mut found: bool = r#false != 0;
    if n < 0 as ::core::ffi::c_int {
        let mut othern: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = (*(*root).outputs).length
            - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            let mut output: *mut sway_output = *(*(*root).outputs)
                .items
                .offset(i as isize) as *mut sway_output;
            let mut j: ::core::ffi::c_int = (*(*output).workspaces).length
                - 1 as ::core::ffi::c_int;
            while j >= 0 as ::core::ffi::c_int {
                let mut ws: *mut sway_workspace = *(*(*output).workspaces)
                    .items
                    .offset(j as isize) as *mut sway_workspace;
                let mut wsn: ::core::ffi::c_int = workspace_get_number(ws);
                if last.is_null() {
                    last = ws;
                }
                if other.is_null() || wsn >= 0 as ::core::ffi::c_int && wsn > othern {
                    other = ws;
                    othern = workspace_get_number(other);
                }
                if ws == workspace {
                    found = r#true != 0;
                } else if wsn < 0 as ::core::ffi::c_int
                    && found as ::core::ffi::c_int != 0
                {
                    return ws
                }
                j -= 1;
            }
            i -= 1;
        }
    } else {
        let mut prevn: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut lastn: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut i_0: ::core::ffi::c_int = (*(*root).outputs).length
            - 1 as ::core::ffi::c_int;
        while i_0 >= 0 as ::core::ffi::c_int {
            let mut output_0: *mut sway_output = *(*(*root).outputs)
                .items
                .offset(i_0 as isize) as *mut sway_output;
            let mut j_0: ::core::ffi::c_int = (*(*output_0).workspaces).length
                - 1 as ::core::ffi::c_int;
            while j_0 >= 0 as ::core::ffi::c_int {
                let mut ws_0: *mut sway_workspace = *(*(*output_0).workspaces)
                    .items
                    .offset(j_0 as isize) as *mut sway_workspace;
                let mut wsn_0: ::core::ffi::c_int = workspace_get_number(ws_0);
                if last.is_null() || wsn_0 >= 0 as ::core::ffi::c_int && wsn_0 > lastn {
                    last = ws_0;
                    lastn = workspace_get_number(last);
                }
                if other.is_null() && wsn_0 < 0 as ::core::ffi::c_int {
                    other = ws_0;
                }
                if wsn_0 >= 0 as ::core::ffi::c_int {
                    if wsn_0 < n && (prev.is_null() || wsn_0 > prevn) {
                        prev = ws_0;
                        prevn = workspace_get_number(prev);
                    }
                }
                j_0 -= 1;
            }
            i_0 -= 1;
        }
    }
    if prev.is_null() {
        prev = if !other.is_null() { other } else { last };
    }
    return prev;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_next(
    mut workspace: *mut sway_workspace,
) -> *mut sway_workspace {
    let mut n: ::core::ffi::c_int = workspace_get_number(workspace);
    let mut next: *mut sway_workspace = ::core::ptr::null_mut::<sway_workspace>();
    let mut first: *mut sway_workspace = ::core::ptr::null_mut::<sway_workspace>();
    let mut other: *mut sway_workspace = ::core::ptr::null_mut::<sway_workspace>();
    let mut found: bool = r#false != 0;
    if n < 0 as ::core::ffi::c_int {
        let mut othern: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*root).outputs).length {
            let mut output: *mut sway_output = *(*(*root).outputs)
                .items
                .offset(i as isize) as *mut sway_output;
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < (*(*output).workspaces).length {
                let mut ws: *mut sway_workspace = *(*(*output).workspaces)
                    .items
                    .offset(j as isize) as *mut sway_workspace;
                let mut wsn: ::core::ffi::c_int = workspace_get_number(ws);
                if first.is_null() {
                    first = ws;
                }
                if other.is_null() || wsn >= 0 as ::core::ffi::c_int && wsn < othern {
                    other = ws;
                    othern = workspace_get_number(other);
                }
                if ws == workspace {
                    found = r#true != 0;
                } else if wsn < 0 as ::core::ffi::c_int
                    && found as ::core::ffi::c_int != 0
                {
                    return ws
                }
                j += 1;
            }
            i += 1;
        }
    } else {
        let mut nextn: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut firstn: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*(*root).outputs).length {
            let mut output_0: *mut sway_output = *(*(*root).outputs)
                .items
                .offset(i_0 as isize) as *mut sway_output;
            let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j_0 < (*(*output_0).workspaces).length {
                let mut ws_0: *mut sway_workspace = *(*(*output_0).workspaces)
                    .items
                    .offset(j_0 as isize) as *mut sway_workspace;
                let mut wsn_0: ::core::ffi::c_int = workspace_get_number(ws_0);
                if first.is_null() || wsn_0 >= 0 as ::core::ffi::c_int && wsn_0 < firstn
                {
                    first = ws_0;
                    firstn = workspace_get_number(first);
                }
                if other.is_null() && wsn_0 < 0 as ::core::ffi::c_int {
                    other = ws_0;
                }
                if wsn_0 < 0 as ::core::ffi::c_int {
                    break;
                }
                if n < wsn_0 && (next.is_null() || wsn_0 < nextn) {
                    next = ws_0;
                    nextn = workspace_get_number(next);
                }
                j_0 += 1;
            }
            i_0 += 1;
        }
    }
    if next.is_null() {
        next = if !other.is_null() { other } else { first };
    }
    return next;
}
unsafe extern "C" fn workspace_output_prev_next_impl(
    mut output: *mut sway_output,
    mut dir: ::core::ffi::c_int,
) -> *mut sway_workspace {
    let mut seat: *mut sway_seat = input_manager_current_seat();
    let mut workspace: *mut sway_workspace = seat_get_focused_workspace(seat);
    if workspace.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] No focused workspace to base prev/next on output off of\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
            690 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<sway_workspace>();
    }
    let mut index: ::core::ffi::c_int = list_find(
        (*output).workspaces,
        workspace as *const ::core::ffi::c_void,
    );
    let mut new_index: size_t = wrap(index + dir, (*(*output).workspaces).length)
        as size_t;
    return *(*(*output).workspaces).items.offset(new_index as isize)
        as *mut sway_workspace;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_output_next(
    mut current: *mut sway_workspace,
) -> *mut sway_workspace {
    return workspace_output_prev_next_impl((*current).output, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn workspace_output_prev(
    mut current: *mut sway_workspace,
) -> *mut sway_workspace {
    return workspace_output_prev_next_impl((*current).output, -1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn workspace_auto_back_and_forth(
    mut workspace: *mut sway_workspace,
) -> *mut sway_workspace {
    let mut seat: *mut sway_seat = input_manager_current_seat();
    let mut active_ws: *mut sway_workspace = ::core::ptr::null_mut::<sway_workspace>();
    let mut focus: *mut sway_node = seat_get_focus_inactive(seat, &raw mut (*root).node);
    if !focus.is_null() && (*focus).r#type.0 == sway_node_type::N_WORKSPACE.0 {
        active_ws = (*focus).c2rust_unnamed.sway_workspace;
    } else if !focus.is_null() && (*focus).r#type.0 == sway_node_type::N_CONTAINER.0 {
        active_ws = (*(*focus).c2rust_unnamed.sway_container).pending.workspace;
    }
    if (*config).auto_back_and_forth as ::core::ffi::c_int != 0 && !active_ws.is_null()
        && active_ws == workspace && !(*seat).prev_workspace_name.is_null()
    {
        let mut new_ws: *mut sway_workspace = workspace_by_name(
            (*seat).prev_workspace_name,
        );
        workspace = if !new_ws.is_null() {
            new_ws
        } else {
            workspace_create(
                ::core::ptr::null_mut::<sway_output>(),
                (*seat).prev_workspace_name,
            )
        };
    }
    return workspace;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_switch(mut workspace: *mut sway_workspace) -> bool {
    let mut seat: *mut sway_seat = input_manager_current_seat();
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Switching to workspace %p:%s\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
        734 as ::core::ffi::c_int,
        workspace,
        (*workspace).name,
    );
    let mut next: *mut sway_node = seat_get_focus_inactive(
        seat,
        &raw mut (*workspace).node,
    );
    if next.is_null() {
        next = &raw mut (*workspace).node;
    }
    seat_set_focus(seat, next);
    arrange_workspace(workspace);
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_is_visible(mut ws: *mut sway_workspace) -> bool {
    if (*ws).node.destroying {
        return r#false != 0;
    }
    return output_get_active_workspace((*ws).output) == ws;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_is_empty(mut ws: *mut sway_workspace) -> bool {
    if (*(*ws).tiling).length != 0 {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*ws).floating).length {
        let mut floater: *mut sway_container = *(*(*ws).floating)
            .items
            .offset(i as isize) as *mut sway_container;
        if !container_is_sticky(floater) {
            return r#false != 0;
        }
        i += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn find_output(
    mut id1: *const ::core::ffi::c_void,
    mut id2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(id1 as *const ::core::ffi::c_char, id2 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn workspace_output_get_priority(
    mut ws: *mut sway_workspace,
    mut output: *mut sway_output,
) -> ::core::ffi::c_int {
    let mut identifier: [::core::ffi::c_char; 128] = [0; 128];
    output_get_identifier(
        &raw mut identifier as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        output,
    );
    let mut index_id: ::core::ffi::c_int = list_seq_find(
        (*ws).output_priority,
        Some(
            find_output
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut identifier as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
    );
    let mut index_name: ::core::ffi::c_int = list_seq_find(
        (*ws).output_priority,
        Some(
            find_output
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        (*(*output).wlr_output).name as *const ::core::ffi::c_void,
    );
    return if index_name < 0 as ::core::ffi::c_int || index_id < index_name {
        index_id
    } else {
        index_name
    };
}
#[no_mangle]
pub unsafe extern "C" fn workspace_output_raise_priority(
    mut ws: *mut sway_workspace,
    mut old_output: *mut sway_output,
    mut output: *mut sway_output,
) {
    let mut old_index: ::core::ffi::c_int = workspace_output_get_priority(
        ws,
        old_output,
    );
    if old_index < 0 as ::core::ffi::c_int {
        return;
    }
    let mut new_index: ::core::ffi::c_int = workspace_output_get_priority(ws, output);
    if new_index < 0 as ::core::ffi::c_int {
        let mut identifier: [::core::ffi::c_char; 128] = [0; 128];
        output_get_identifier(
            &raw mut identifier as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            output,
        );
        list_insert(
            (*ws).output_priority,
            old_index,
            strdup(&raw mut identifier as *mut ::core::ffi::c_char)
                as *mut ::core::ffi::c_void,
        );
    } else if new_index > old_index {
        let mut name: *mut ::core::ffi::c_char = *(*(*ws).output_priority)
            .items
            .offset(new_index as isize) as *mut ::core::ffi::c_char;
        list_del((*ws).output_priority, new_index);
        list_insert((*ws).output_priority, old_index, name as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn workspace_output_add_priority(
    mut workspace: *mut sway_workspace,
    mut output: *mut sway_output,
) {
    if workspace_output_get_priority(workspace, output) < 0 as ::core::ffi::c_int {
        let mut identifier: [::core::ffi::c_char; 128] = [0; 128];
        output_get_identifier(
            &raw mut identifier as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            output,
        );
        list_add(
            (*workspace).output_priority,
            strdup(&raw mut identifier as *mut ::core::ffi::c_char)
                as *mut ::core::ffi::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn workspace_output_get_highest_available(
    mut ws: *mut sway_workspace,
) -> *mut sway_output {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*ws).output_priority).length {
        let mut name: *const ::core::ffi::c_char = *(*(*ws).output_priority)
            .items
            .offset(i as isize) as *const ::core::ffi::c_char;
        let mut output: *mut sway_output = output_by_name_or_id(name);
        if !output.is_null() {
            return output;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<sway_output>();
}
unsafe extern "C" fn find_urgent_iterator(
    mut con: *mut sway_container,
    mut data: *mut ::core::ffi::c_void,
) -> bool {
    return !(*con).view.is_null()
        && view_is_urgent((*con).view) as ::core::ffi::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_detect_urgent(mut workspace: *mut sway_workspace) {
    let mut new_urgent: bool = !workspace_find_container(
            workspace,
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
    if (*workspace).urgent as ::core::ffi::c_int != new_urgent as ::core::ffi::c_int {
        (*workspace).urgent = new_urgent;
        wlr_ext_workspace_handle_v1_set_urgent(
            (*workspace).ext_workspace,
            (*workspace).urgent,
        );
        ipc_event_workspace(
            ::core::ptr::null_mut::<sway_workspace>(),
            workspace,
            b"urgent\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn workspace_for_each_container(
    mut ws: *mut sway_workspace,
    mut f: Option<
        unsafe extern "C" fn(*mut sway_container, *mut ::core::ffi::c_void) -> (),
    >,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*ws).tiling).length {
        let mut container: *mut sway_container = *(*(*ws).tiling)
            .items
            .offset(i as isize) as *mut sway_container;
        f.expect("non-null function pointer")(container, data);
        container_for_each_child(container, f, data);
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*(*ws).floating).length {
        let mut container_0: *mut sway_container = *(*(*ws).floating)
            .items
            .offset(i_0 as isize) as *mut sway_container;
        f.expect("non-null function pointer")(container_0, data);
        container_for_each_child(container_0, f, data);
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn workspace_find_container(
    mut ws: *mut sway_workspace,
    mut test: Option<
        unsafe extern "C" fn(*mut sway_container, *mut ::core::ffi::c_void) -> bool,
    >,
    mut data: *mut ::core::ffi::c_void,
) -> *mut sway_container {
    let mut result: *mut sway_container = ::core::ptr::null_mut::<sway_container>();
    if ws.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Cannot find container with no workspace.\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
            856 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<sway_container>();
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*ws).tiling).length {
        let mut child: *mut sway_container = *(*(*ws).tiling).items.offset(i as isize)
            as *mut sway_container;
        if test.expect("non-null function pointer")(child, data) {
            return child;
        }
        result = container_find_child(child, test, data);
        if !result.is_null() {
            return result;
        }
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*(*ws).floating).length {
        let mut child_0: *mut sway_container = *(*(*ws).floating)
            .items
            .offset(i_0 as isize) as *mut sway_container;
        if test.expect("non-null function pointer")(child_0, data) {
            return child_0;
        }
        result = container_find_child(child_0, test, data);
        if !result.is_null() {
            return result;
        }
        i_0 += 1;
    }
    return ::core::ptr::null_mut::<sway_container>();
}
unsafe extern "C" fn set_workspace(
    mut container: *mut sway_container,
    mut data: *mut ::core::ffi::c_void,
) {
    (*container).pending.workspace = (*(*container).pending.parent).pending.workspace;
}
unsafe extern "C" fn workspace_attach_tiling(
    mut ws: *mut sway_workspace,
    mut con: *mut sway_container,
) {
    list_add((*ws).tiling, con as *mut ::core::ffi::c_void);
    (*con).pending.workspace = ws;
    container_for_each_child(
        con,
        Some(
            set_workspace
                as unsafe extern "C" fn(
                    *mut sway_container,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
    container_handle_fullscreen_reparent(con);
    workspace_update_representation(ws);
    node_set_dirty(&raw mut (*ws).node);
    node_set_dirty(&raw mut (*con).node);
}
#[no_mangle]
pub unsafe extern "C" fn workspace_wrap_children(
    mut ws: *mut sway_workspace,
) -> *mut sway_container {
    let mut fs: *mut sway_container = (*ws).fullscreen;
    let mut middle: *mut sway_container = container_create(
        ::core::ptr::null_mut::<sway_view>(),
    );
    (*middle).pending.layout = (*ws).layout;
    while (*(*ws).tiling).length != 0 {
        let mut child: *mut sway_container = *(*(*ws).tiling).items.offset(0isize)
            as *mut sway_container;
        container_detach(child);
        container_add_child(middle, child);
    }
    workspace_attach_tiling(ws, middle);
    (*ws).fullscreen = fs;
    return middle;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_unwrap_children(
    mut ws: *mut sway_workspace,
    mut wrap_0: *mut sway_container,
) {
    if !_sway_assert(
        workspace_is_empty(ws),
        b"[%s:%d] %s:target workspace must be empty\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
        914 as ::core::ffi::c_int,
        b"void workspace_unwrap_children(struct sway_workspace *, struct sway_container *)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    (*ws).layout = (*wrap_0).pending.layout;
    while (*(*wrap_0).pending.children).length != 0 {
        let mut child: *mut sway_container = *(*(*wrap_0).pending.children)
            .items
            .offset(0isize) as *mut sway_container;
        container_detach(child);
        workspace_insert_tiling_direct(ws, child, (*(*ws).tiling).length);
    }
}
#[no_mangle]
pub unsafe extern "C" fn workspace_detach(mut workspace: *mut sway_workspace) {
    let mut output: *mut sway_output = (*workspace).output;
    let mut index: ::core::ffi::c_int = list_find(
        (*output).workspaces,
        workspace as *const ::core::ffi::c_void,
    );
    if index != -1 as ::core::ffi::c_int {
        list_del((*output).workspaces, index);
    }
    (*workspace).output = ::core::ptr::null_mut::<sway_output>();
    node_set_dirty(&raw mut (*workspace).node);
    node_set_dirty(&raw mut (*output).node);
}
#[no_mangle]
pub unsafe extern "C" fn workspace_add_tiling(
    mut workspace: *mut sway_workspace,
    mut con: *mut sway_container,
) -> *mut sway_container {
    if !(*con).pending.workspace.is_null() {
        let mut old_parent: *mut sway_container = (*con).pending.parent;
        container_detach(con);
        if !old_parent.is_null() {
            container_reap_empty(old_parent);
        }
    }
    if (*config).default_layout.0 != sway_container_layout_0::L_NONE.0 {
        con = container_split(con, (*config).default_layout);
    }
    list_add((*workspace).tiling, con as *mut ::core::ffi::c_void);
    (*con).pending.workspace = workspace;
    container_for_each_child(
        con,
        Some(
            set_workspace
                as unsafe extern "C" fn(
                    *mut sway_container,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
    container_handle_fullscreen_reparent(con);
    workspace_update_representation(workspace);
    node_set_dirty(&raw mut (*workspace).node);
    node_set_dirty(&raw mut (*con).node);
    return con;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_add_floating(
    mut workspace: *mut sway_workspace,
    mut con: *mut sway_container,
) {
    if !(*con).pending.workspace.is_null() {
        container_detach(con);
    }
    list_add((*workspace).floating, con as *mut ::core::ffi::c_void);
    (*con).pending.workspace = workspace;
    container_for_each_child(
        con,
        Some(
            set_workspace
                as unsafe extern "C" fn(
                    *mut sway_container,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
    container_handle_fullscreen_reparent(con);
    node_set_dirty(&raw mut (*workspace).node);
    node_set_dirty(&raw mut (*con).node);
}
#[no_mangle]
pub unsafe extern "C" fn workspace_insert_tiling_direct(
    mut workspace: *mut sway_workspace,
    mut con: *mut sway_container,
    mut index: ::core::ffi::c_int,
) {
    list_insert((*workspace).tiling, index, con as *mut ::core::ffi::c_void);
    (*con).pending.workspace = workspace;
    container_for_each_child(
        con,
        Some(
            set_workspace
                as unsafe extern "C" fn(
                    *mut sway_container,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
    container_handle_fullscreen_reparent(con);
    workspace_update_representation(workspace);
    node_set_dirty(&raw mut (*workspace).node);
    node_set_dirty(&raw mut (*con).node);
}
#[no_mangle]
pub unsafe extern "C" fn workspace_insert_tiling(
    mut workspace: *mut sway_workspace,
    mut con: *mut sway_container,
    mut index: ::core::ffi::c_int,
) -> *mut sway_container {
    if !(*con).pending.workspace.is_null() {
        container_detach(con);
    }
    if (*config).default_layout.0 != sway_container_layout_0::L_NONE.0 {
        con = container_split(con, (*config).default_layout);
    }
    workspace_insert_tiling_direct(workspace, con, index);
    return con;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_has_single_visible_container(
    mut ws: *mut sway_workspace,
) -> bool {
    let mut seat: *mut sway_seat = input_manager_get_default_seat();
    let mut focus: *mut sway_container = seat_get_focus_inactive_tiling(seat, ws);
    if !focus.is_null() && (*focus).view.is_null() {
        focus = seat_get_focus_inactive_view(seat, &raw mut (*focus).node);
    }
    return !focus.is_null() && !(*focus).view.is_null()
        && view_ancestor_is_only_visible((*focus).view) as ::core::ffi::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_add_gaps(mut ws: *mut sway_workspace) {
    if (*config).smart_gaps.0 == smart_gaps_mode::SMART_GAPS_ON.0
        && workspace_has_single_visible_container(ws) as ::core::ffi::c_int != 0
    {
        (*ws).current_gaps.top = 0 as ::core::ffi::c_int;
        (*ws).current_gaps.right = 0 as ::core::ffi::c_int;
        (*ws).current_gaps.bottom = 0 as ::core::ffi::c_int;
        (*ws).current_gaps.left = 0 as ::core::ffi::c_int;
        return;
    }
    if (*config).smart_gaps.0 == smart_gaps_mode::SMART_GAPS_INVERSE_OUTER.0
        && !workspace_has_single_visible_container(ws)
    {
        (*ws).current_gaps.top = 0 as ::core::ffi::c_int;
        (*ws).current_gaps.right = 0 as ::core::ffi::c_int;
        (*ws).current_gaps.bottom = 0 as ::core::ffi::c_int;
        (*ws).current_gaps.left = 0 as ::core::ffi::c_int;
    } else {
        (*ws).current_gaps = (*ws).gaps_outer;
    }
    (*ws).current_gaps.top = fmax(
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        ((*ws).current_gaps.top + (*ws).gaps_inner) as ::core::ffi::c_double,
    ) as ::core::ffi::c_int;
    (*ws).current_gaps.right = fmax(
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        ((*ws).current_gaps.right + (*ws).gaps_inner) as ::core::ffi::c_double,
    ) as ::core::ffi::c_int;
    (*ws).current_gaps.bottom = fmax(
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        ((*ws).current_gaps.bottom + (*ws).gaps_inner) as ::core::ffi::c_double,
    ) as ::core::ffi::c_int;
    (*ws).current_gaps.left = fmax(
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        ((*ws).current_gaps.left + (*ws).gaps_inner) as ::core::ffi::c_double,
    ) as ::core::ffi::c_int;
    if (*ws).width - (*ws).current_gaps.left - (*ws).current_gaps.right < MIN_SANE_W
        && (*ws).current_gaps.left + (*ws).current_gaps.right > 0 as ::core::ffi::c_int
    {
        let mut total_gap: ::core::ffi::c_int = fmax(
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ((*ws).width - MIN_SANE_W) as ::core::ffi::c_double,
        ) as ::core::ffi::c_int;
        let mut left_gap_frac: ::core::ffi::c_double = (*ws).current_gaps.left
            as ::core::ffi::c_double
            / ((*ws).current_gaps.left as ::core::ffi::c_double
                + (*ws).current_gaps.right as ::core::ffi::c_double);
        (*ws).current_gaps.left = (left_gap_frac * total_gap as ::core::ffi::c_double)
            as ::core::ffi::c_int;
        (*ws).current_gaps.right = total_gap - (*ws).current_gaps.left;
    }
    if (*ws).height - (*ws).current_gaps.top - (*ws).current_gaps.bottom < MIN_SANE_H
        && (*ws).current_gaps.top + (*ws).current_gaps.bottom > 0 as ::core::ffi::c_int
    {
        let mut total_gap_0: ::core::ffi::c_int = fmax(
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ((*ws).height - MIN_SANE_H) as ::core::ffi::c_double,
        ) as ::core::ffi::c_int;
        let mut top_gap_frac: ::core::ffi::c_double = (*ws).current_gaps.top
            as ::core::ffi::c_double
            / ((*ws).current_gaps.top as ::core::ffi::c_double
                + (*ws).current_gaps.bottom as ::core::ffi::c_double);
        (*ws).current_gaps.top = (top_gap_frac * total_gap_0 as ::core::ffi::c_double)
            as ::core::ffi::c_int;
        (*ws).current_gaps.bottom = total_gap_0 - (*ws).current_gaps.top;
    }
    (*ws).x += (*ws).current_gaps.left as ::core::ffi::c_double;
    (*ws).y += (*ws).current_gaps.top as ::core::ffi::c_double;
    (*ws).width -= (*ws).current_gaps.left + (*ws).current_gaps.right;
    (*ws).height -= (*ws).current_gaps.top + (*ws).current_gaps.bottom;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_split(
    mut workspace: *mut sway_workspace,
    mut layout: sway_container_layout,
) -> *mut sway_container {
    if (*(*workspace).tiling).length == 0 as ::core::ffi::c_int {
        (*workspace).prev_split_layout = (*workspace).layout;
        (*workspace).layout = layout;
        return ::core::ptr::null_mut::<sway_container>();
    }
    let mut old_layout: sway_container_layout = (*workspace).layout;
    let mut middle: *mut sway_container = workspace_wrap_children(workspace);
    (*workspace).layout = layout;
    (*middle).pending.layout = old_layout;
    let mut seat: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
    seat = ((*server.input).seats.next as *mut ::core::ffi::c_char)
        .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
        as *mut sway_seat;
    while &raw mut (*seat).link != &raw mut (*server.input).seats {
        if seat_get_focus(seat) == &raw mut (*workspace).node {
            seat_set_focus(seat, &raw mut (*middle).node);
        }
        seat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
            as *mut sway_seat;
    }
    return middle;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_update_representation(mut ws: *mut sway_workspace) {
    let mut len: size_t = container_build_representation(
        (*ws).layout,
        (*ws).tiling,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    free((*ws).representation as *mut ::core::ffi::c_void);
    (*ws).representation = calloc(
        len.wrapping_add(1 as size_t),
        ::core::mem::size_of::<::core::ffi::c_char>(),
    ) as *mut ::core::ffi::c_char;
    if !_sway_assert(
        !(*ws).representation.is_null(),
        b"[%s:%d] %s:Unable to allocate title string\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
        1085 as ::core::ffi::c_int,
        b"void workspace_update_representation(struct sway_workspace *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    container_build_representation((*ws).layout, (*ws).tiling, (*ws).representation);
}
#[no_mangle]
pub unsafe extern "C" fn workspace_get_box(
    mut workspace: *mut sway_workspace,
    mut r#box: *mut wlr_box,
) {
    (*r#box).x = (*workspace).x as ::core::ffi::c_int;
    (*r#box).y = (*workspace).y as ::core::ffi::c_int;
    (*r#box).width = (*workspace).width;
    (*r#box).height = (*workspace).height;
}
unsafe extern "C" fn count_tiling_views(
    mut con: *mut sway_container,
    mut data: *mut ::core::ffi::c_void,
) {
    if !(*con).view.is_null() && !container_is_floating_or_child(con) {
        let mut count: *mut size_t = data as *mut size_t;
        *count = (*count).wrapping_add(1 as size_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn workspace_num_tiling_views(
    mut ws: *mut sway_workspace,
) -> size_t {
    let mut count: size_t = 0 as size_t;
    workspace_for_each_container(
        ws,
        Some(
            count_tiling_views
                as unsafe extern "C" fn(
                    *mut sway_container,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        &raw mut count as *mut ::core::ffi::c_void,
    );
    return count;
}
unsafe extern "C" fn count_sticky_containers(
    mut con: *mut sway_container,
    mut data: *mut ::core::ffi::c_void,
) {
    if container_is_sticky(con) {
        let mut count: *mut size_t = data as *mut size_t;
        *count = (*count).wrapping_add(1 as size_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn workspace_num_sticky_containers(
    mut ws: *mut sway_workspace,
) -> size_t {
    let mut count: size_t = 0 as size_t;
    workspace_for_each_container(
        ws,
        Some(
            count_sticky_containers
                as unsafe extern "C" fn(
                    *mut sway_container,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        &raw mut count as *mut ::core::ffi::c_void,
    );
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_squash(mut workspace: *mut sway_workspace) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*workspace).tiling).length {
        let mut child: *mut sway_container = *(*(*workspace).tiling)
            .items
            .offset(i as isize) as *mut sway_container;
        i += container_squash(child);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn workspace_move_to_output(
    mut workspace: *mut sway_workspace,
    mut output: *mut sway_output,
) {
    if (*workspace).output == output {
        return;
    }
    let mut old_output: *mut sway_output = (*workspace).output;
    workspace_detach(workspace);
    let mut new_output_old_ws: *mut sway_workspace = output_get_active_workspace(output);
    if !_sway_assert(
        !new_output_old_ws.is_null(),
        b"[%s:%d] %s:Expected output to have a workspace\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/tree/workspace.c\0".as_ptr() as *const ::core::ffi::c_char,
        1140 as ::core::ffi::c_int,
        b"void workspace_move_to_output(struct sway_workspace *, struct sway_output *)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    output_add_workspace(output, workspace);
    if (*(*old_output).workspaces).length == 0 as ::core::ffi::c_int {
        let mut ws_name: *mut ::core::ffi::c_char = workspace_next_name(
            (*(*old_output).wlr_output).name,
        );
        let mut ws: *mut sway_workspace = workspace_create(old_output, ws_name);
        free(ws_name as *mut ::core::ffi::c_void);
        let mut seat: *mut sway_seat = input_manager_current_seat();
        seat_set_raw_focus(seat, &raw mut (*ws).node);
    }
    workspace_consider_destroy(new_output_old_ws);
    output_sort_workspaces(output);
    workspace_output_raise_priority(workspace, old_output, output);
    ipc_event_workspace(
        ::core::ptr::null_mut::<sway_workspace>(),
        workspace,
        b"move\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
