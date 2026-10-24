extern "C" {
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_buffer_impl;
    pub type wlr_backend_impl;
    pub type wlr_renderer_impl;
    pub type wlr_render_timer;
    pub type wlr_drm_syncobj_timeline;
    pub type wlr_texture_impl;
    pub type wlr_swapchain;
    pub type wlr_output_layer_state;
    pub type wlr_xcursor_manager;
    pub type wlr_cursor_state;
    pub type udev;
    pub type udev_monitor;
    pub type xkb_keymap;
    pub type xkb_state;
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
    pub type wlr_tablet_manager_v2;
    pub type wlr_xdg_shell;
    pub type wlr_idle_notifier_v1;
    pub type wlr_data_device_manager;
    pub type libseat;
    pub type wlr_tablet_v2_tablet_tool;
    pub type wlr_tablet_v2_tablet;
    pub type wlr_tablet_v2_tablet_pad;
    pub type wlr_tablet_pad;
    pub type sway_switch;
    pub type sway_keyboard;
    pub type wlr_touch_impl;
    pub type _PangoFontDescription;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn socketpair(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
        __fds: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execvp(
        __file: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn fork() -> __pid_t;
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn ceilf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn roundf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn wl_list_init(list: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn wl_list_length(list: *const wl_list) -> ::core::ffi::c_int;
    fn wl_list_empty(list: *const wl_list) -> ::core::ffi::c_int;
    fn wl_client_create(
        display: *mut wl_display,
        fd: ::core::ffi::c_int,
    ) -> *mut wl_client;
    fn wl_client_destroy(client: *mut wl_client);
    fn wl_client_add_destroy_listener(
        client: *mut wl_client,
        listener: *mut wl_listener,
    );
    fn wlr_drm_format_set_get(
        set: *const wlr_drm_format_set,
        format: uint32_t,
    ) -> *const wlr_drm_format;
    fn wlr_color_transform_init_linear_to_inverse_eotf(
        tf: wlr_color_transfer_function,
    ) -> *mut wlr_color_transform;
    fn wlr_color_transform_init_matrix(
        matrix: *const ::core::ffi::c_float,
    ) -> *mut wlr_color_transform;
    fn wlr_color_transform_init_pipeline(
        transforms: *mut *mut wlr_color_transform,
        len: size_t,
    ) -> *mut wlr_color_transform;
    fn wlr_color_transform_ref(tr: *mut wlr_color_transform) -> *mut wlr_color_transform;
    fn wlr_color_transform_unref(tr: *mut wlr_color_transform);
    fn wlr_color_primaries_from_named(
        out: *mut wlr_color_primaries,
        named: wlr_color_named_primaries,
    );
    fn wlr_color_primaries_transform_absolute_colorimetric(
        source: *const wlr_color_primaries,
        destination: *const wlr_color_primaries,
        matrix: *mut ::core::ffi::c_float,
    );
    fn wlr_box_transform(
        dest: *mut wlr_box,
        r#box: *const wlr_box,
        transform: wl_output_transform,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn wlr_backend_commit(
        backend: *mut wlr_backend,
        states: *const wlr_backend_output_state,
        states_len: size_t,
    ) -> bool;
    fn wlr_output_preferred_mode(output: *mut wlr_output) -> *mut wlr_output_mode;
    fn wlr_output_get_primary_formats(
        output: *mut wlr_output,
        buffer_caps: uint32_t,
    ) -> *const wlr_drm_format_set;
    fn wlr_output_state_init(state: *mut wlr_output_state);
    fn wlr_output_state_finish(state: *mut wlr_output_state);
    fn wlr_output_state_set_enabled(state: *mut wlr_output_state, enabled: bool);
    fn wlr_output_state_set_mode(
        state: *mut wlr_output_state,
        mode: *mut wlr_output_mode,
    );
    fn wlr_output_state_set_custom_mode(
        state: *mut wlr_output_state,
        width: int32_t,
        height: int32_t,
        refresh: int32_t,
    );
    fn wlr_output_state_set_scale(
        state: *mut wlr_output_state,
        scale: ::core::ffi::c_float,
    );
    fn wlr_output_state_set_transform(
        state: *mut wlr_output_state,
        transform: wl_output_transform,
    );
    fn wlr_output_state_set_adaptive_sync_enabled(
        state: *mut wlr_output_state,
        enabled: bool,
    );
    fn wlr_output_state_set_render_format(
        state: *mut wlr_output_state,
        format: uint32_t,
    );
    fn wlr_output_state_set_subpixel(
        state: *mut wlr_output_state,
        subpixel: wl_output_subpixel,
    );
    fn wlr_output_state_set_image_description(
        state: *mut wlr_output_state,
        image_desc: *const wlr_output_image_description,
    ) -> bool;
    fn wlr_output_layout_add(
        layout: *mut wlr_output_layout,
        output: *mut wlr_output,
        lx: ::core::ffi::c_int,
        ly: ::core::ffi::c_int,
    ) -> *mut wlr_output_layout_output;
    fn wlr_output_layout_add_auto(
        layout: *mut wlr_output_layout,
        output: *mut wlr_output,
    ) -> *mut wlr_output_layout_output;
    fn wlr_output_layout_remove(layout: *mut wlr_output_layout, output: *mut wlr_output);
    fn wlr_output_layout_get_box(
        layout: *mut wlr_output_layout,
        reference: *mut wlr_output,
        dest_box: *mut wlr_box,
    );
    fn wlr_output_swapchain_manager_init(
        manager: *mut wlr_output_swapchain_manager,
        backend: *mut wlr_backend,
    );
    fn wlr_output_swapchain_manager_prepare(
        manager: *mut wlr_output_swapchain_manager,
        states: *const wlr_backend_output_state,
        states_len: size_t,
    ) -> bool;
    fn wlr_output_swapchain_manager_get_swapchain(
        manager: *mut wlr_output_swapchain_manager,
        output: *mut wlr_output,
    ) -> *mut wlr_swapchain;
    fn wlr_output_swapchain_manager_apply(manager: *mut wlr_output_swapchain_manager);
    fn wlr_output_swapchain_manager_finish(manager: *mut wlr_output_swapchain_manager);
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn drmGetFormatName(format: uint32_t) -> *mut ::core::ffi::c_char;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn wlr_damage_ring_add_whole(ring: *mut wlr_damage_ring);
    fn wlr_scene_output_build_state(
        scene_output: *mut wlr_scene_output,
        state: *mut wlr_output_state,
        options: *const wlr_scene_output_state_options,
    ) -> bool;
    static mut root: *mut sway_root;
    fn all_output_by_name_or_id(
        name_or_id: *const ::core::ffi::c_char,
    ) -> *mut sway_output;
    fn output_enable(output: *mut sway_output);
    fn output_disable(output: *mut sway_output);
    fn update_output_manager_config(server_0: *mut sway_server);
    fn input_manager_configure_xcursor();
    fn input_manager_configure_all_input_mappings();
    static mut server: sway_server;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn transaction_commit_dirty();
    fn arrange_layers(output: *mut sway_output);
    fn arrange_root();
    static mut config: *mut sway_config;
    fn cursor_rebase(cursor: *mut sway_cursor);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    fn sway_wl_output_subpixel_to_string(
        subpixel: wl_output_subpixel,
    ) -> *const ::core::ffi::c_char;
    fn sway_set_cloexec(fd: ::core::ffi::c_int, cloexec: bool) -> bool;
    fn wlr_output_is_drm(output: *const wlr_output) -> bool;
    fn wlr_drm_connector_add_mode(
        output: *mut wlr_output,
        mode: *const drmModeModeInfo,
    ) -> *mut wlr_output_mode;
    fn wlr_drm_connector_get_panel_orientation(
        output: *mut wlr_output,
    ) -> wl_output_transform;
    fn wlr_seat_pointer_notify_clear_focus(wlr_seat: *mut wlr_seat);
    fn arrange_locks();
}
pub type __u32 = ::core::ffi::c_uint;
pub type size_t = usize;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct __socket_type(pub ::core::ffi::c_uint);
impl __socket_type {
    pub const SOCK_STREAM: Self = Self(1);
    pub const SOCK_DGRAM: Self = Self(2);
    pub const SOCK_RAW: Self = Self(3);
    pub const SOCK_RDM: Self = Self(4);
    pub const SOCK_SEQPACKET: Self = Self(5);
    pub const SOCK_DCCP: Self = Self(6);
    pub const SOCK_PACKET: Self = Self(10);
    pub const SOCK_CLOEXEC: Self = Self(524288);
    pub const SOCK_NONBLOCK: Self = Self(2048);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub struct wlr_allocator {
    pub r#impl: *const wlr_allocator_interface,
    pub buffer_caps: uint32_t,
    pub events: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_allocator_interface {
    pub create_buffer: Option<
        unsafe extern "C" fn(
            *mut wlr_allocator,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const wlr_drm_format,
        ) -> *mut wlr_buffer,
    >,
    pub destroy: Option<unsafe extern "C" fn(*mut wlr_allocator) -> ()>,
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
pub struct wlr_buffer {
    pub r#impl: *const wlr_buffer_impl,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub dropped: bool,
    pub n_locks: size_t,
    pub accessing_data_ptr: bool,
    pub events: C2Rust_Unnamed_1,
    pub addons: wlr_addon_set,
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
pub struct C2Rust_Unnamed_1 {
    pub destroy: wl_signal,
    pub release: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_backend {
    pub r#impl: *const wlr_backend_impl,
    pub buffer_caps: uint32_t,
    pub features: C2Rust_Unnamed_3,
    pub events: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub timeline: bool,
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
pub struct wlr_addon {
    pub r#impl: *const wlr_addon_interface,
    pub WLR_PRIVATE: C2Rust_Unnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
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
pub struct wlr_drm_format_set {
    pub len: size_t,
    pub capacity: size_t,
    pub formats: *mut wlr_drm_format,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_output_state_field(pub ::core::ffi::c_uint);
impl wlr_output_state_field {
    pub const WLR_OUTPUT_STATE_BUFFER: Self = Self(1);
    pub const WLR_OUTPUT_STATE_DAMAGE: Self = Self(2);
    pub const WLR_OUTPUT_STATE_MODE: Self = Self(4);
    pub const WLR_OUTPUT_STATE_ENABLED: Self = Self(8);
    pub const WLR_OUTPUT_STATE_SCALE: Self = Self(16);
    pub const WLR_OUTPUT_STATE_TRANSFORM: Self = Self(32);
    pub const WLR_OUTPUT_STATE_ADAPTIVE_SYNC_ENABLED: Self = Self(64);
    pub const WLR_OUTPUT_STATE_RENDER_FORMAT: Self = Self(128);
    pub const WLR_OUTPUT_STATE_SUBPIXEL: Self = Self(256);
    pub const WLR_OUTPUT_STATE_LAYERS: Self = Self(512);
    pub const WLR_OUTPUT_STATE_WAIT_TIMELINE: Self = Self(1024);
    pub const WLR_OUTPUT_STATE_SIGNAL_TIMELINE: Self = Self(2048);
    pub const WLR_OUTPUT_STATE_COLOR_TRANSFORM: Self = Self(4096);
    pub const WLR_OUTPUT_STATE_IMAGE_DESCRIPTION: Self = Self(8192);
    pub const WLR_OUTPUT_STATE_COLOR_REPRESENTATION: Self = Self(16384);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_layout_output {
    pub layout: *mut wlr_output_layout,
    pub output: *mut wlr_output,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub link: wl_list,
    pub auto_configured: bool,
    pub events: C2Rust_Unnamed_18,
    pub WLR_PRIVATE: C2Rust_Unnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub addon: wlr_addon,
    pub commit: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_device {
    pub r#type: wlr_input_device_type,
    pub name: *mut ::core::ffi::c_char,
    pub events: C2Rust_Unnamed_19,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
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
    pub events: C2Rust_Unnamed_22,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_21,
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
pub struct C2Rust_Unnamed_21 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
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
    pub viewport: C2Rust_Unnamed_23,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
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
    pub events: C2Rust_Unnamed_25,
    pub WLR_PRIVATE: C2Rust_Unnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_cursor {
    pub state: *mut wlr_cursor_state,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
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
pub struct wlr_backend_output_state {
    pub output: *mut wlr_output,
    pub base: wlr_output_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_swapchain_manager {
    pub backend: *mut wlr_backend,
    pub WLR_PRIVATE: C2Rust_Unnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub outputs: wl_array,
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
    pub WLR_PRIVATE: C2Rust_Unnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_30,
    pub WLR_PRIVATE: C2Rust_Unnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
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
pub struct C2Rust_Unnamed_30 {
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
    pub events: C2Rust_Unnamed_32,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_33,
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
pub struct C2Rust_Unnamed_33 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_35,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
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
    pub events: C2Rust_Unnamed_37,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_37 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
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
    pub events: C2Rust_Unnamed_43,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
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
    pub events: C2Rust_Unnamed_41,
    pub WLR_PRIVATE: C2Rust_Unnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_40 {
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
pub struct C2Rust_Unnamed_41 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_42,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
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
pub struct C2Rust_Unnamed_43 {
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
pub struct wlr_scene_timer {
    pub pre_render_duration: int64_t,
    pub render_timer: *mut wlr_render_timer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_output_state_options {
    pub timer: *mut wlr_scene_timer,
    pub color_transform: *mut wlr_color_transform,
    pub swapchain: *mut wlr_swapchain,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_root {
    pub node: sway_node,
    pub output_layout: *mut wlr_output_layout,
    pub root_scene: *mut wlr_scene,
    pub staging: *mut wlr_scene_tree,
    pub layer_tree: *mut wlr_scene_tree,
    pub layers: C2Rust_Unnamed_95,
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
    pub events: C2Rust_Unnamed_44,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_44 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_47,
    pub border: C2Rust_Unnamed_46,
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
    pub events: C2Rust_Unnamed_45,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_45 {
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
pub struct C2Rust_Unnamed_46 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_49,
    pub events: C2Rust_Unnamed_48,
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
pub struct C2Rust_Unnamed_48 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_49 {
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
    pub events: C2Rust_Unnamed_51,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_50,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_53,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_52,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_52 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_53 {
    pub sway_root: *mut sway_root,
    pub sway_output: *mut sway_output,
    pub sway_workspace: *mut sway_workspace,
    pub sway_container: *mut sway_container,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_output {
    pub node: sway_node,
    pub layers: C2Rust_Unnamed_94,
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
    pub session_lock: C2Rust_Unnamed_60,
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
    pub events: C2Rust_Unnamed_55,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_54,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_54 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
    pub new_object: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_text_input_manager_v3 {
    pub global: *mut wl_global,
    pub text_inputs: wl_list,
    pub events: C2Rust_Unnamed_57,
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
    pub new_text_input: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_manager_v2 {
    pub global: *mut wl_global,
    pub input_methods: wl_list,
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
    pub new_input_method: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_60 {
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
    pub events: C2Rust_Unnamed_62,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_61,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_61 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
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
    pub events: C2Rust_Unnamed_65,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_63,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_64,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
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
pub struct C2Rust_Unnamed_65 {
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
    pub events: C2Rust_Unnamed_67,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_66,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
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
    pub events: C2Rust_Unnamed_69,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_68,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
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
    pub events: C2Rust_Unnamed_71,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_70,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
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
pub struct C2Rust_Unnamed_71 {
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
    pub events: C2Rust_Unnamed_73,
    pub WLR_PRIVATE: C2Rust_Unnamed_72,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_73 {
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
    pub repeat_info: C2Rust_Unnamed_75,
    pub events: C2Rust_Unnamed_74,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_74 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_75 {
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
    pub events: C2Rust_Unnamed_77,
    pub WLR_PRIVATE: C2Rust_Unnamed_76,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_76 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_77 {
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
    pub events: C2Rust_Unnamed_79,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_78,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_78 {
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_79 {
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
    pub events: C2Rust_Unnamed_83,
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
    pub create_seat: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_gestures_v1 {
    pub global: *mut wl_global,
    pub swipes: wl_list,
    pub pinches: wl_list,
    pub holds: wl_list,
    pub events: C2Rust_Unnamed_85,
    pub data: *mut ::core::ffi::c_void,
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
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
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
    pub events: C2Rust_Unnamed_93,
    pub WLR_PRIVATE: C2Rust_Unnamed_92,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_92 {
    pub event_loop_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_93 {
    pub active: wl_signal,
    pub add_drm_card: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_94 {
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
pub struct C2Rust_Unnamed_95 {
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
    pub events: C2Rust_Unnamed_97,
    pub WLR_PRIVATE: C2Rust_Unnamed_96,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_96 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_97 {
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
    pub events: C2Rust_Unnamed_99,
    pub WLR_PRIVATE: C2Rust_Unnamed_98,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_98 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_99 {
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
    pub events: C2Rust_Unnamed_100,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_100 {
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
    pub events: C2Rust_Unnamed_101,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_101 {
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
    pub previous: C2Rust_Unnamed_105,
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
    pub events: C2Rust_Unnamed_103,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_102,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_102 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub synced: wlr_surface_synced,
    pub destroying: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_103 {
    pub set_region: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_constraint_v1_state {
    pub committed: uint32_t,
    pub region: pixman_region32_t,
    pub cursor_hint: C2Rust_Unnamed_104,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_104 {
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
pub struct C2Rust_Unnamed_105 {
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
    pub xcursor_theme: C2Rust_Unnamed_106,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_106 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct side_gaps {
    pub top: ::core::ffi::c_int,
    pub right: ::core::ffi::c_int,
    pub bottom: ::core::ffi::c_int,
    pub left: ::core::ffi::c_int,
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
    pub colors: C2Rust_Unnamed_107,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_107 {
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
    pub border_colors: C2Rust_Unnamed_110,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_108,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_108 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_109,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_109 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_110 {
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
pub struct matched_output_config {
    pub output: *mut sway_output,
    pub config: *mut output_config,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_output_state {
    pub color_transform: *mut wlr_color_transform,
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
pub struct search_context {
    pub swapchain_mgr: *mut wlr_output_swapchain_manager,
    pub states: *mut wlr_backend_output_state,
    pub configs: *mut matched_output_config,
    pub configs_len: size_t,
    pub degrade_to_off: bool,
}
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<
        [u8; 25],
        [::core::ffi::c_char; 25],
    >(*b"_Bool spawn_swaybg(void)\0")
};
pub const DRM_FORMAT_INVALID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DRM_FORMAT_RGB565: __u32 = 'R' as ::core::ffi::c_int as __u32
    | ('G' as ::core::ffi::c_int as __u32) << 8 as ::core::ffi::c_int
    | ('1' as ::core::ffi::c_int as __u32) << 16 as ::core::ffi::c_int
    | ('6' as ::core::ffi::c_int as __u32) << 24 as ::core::ffi::c_int;
pub const DRM_FORMAT_XRGB8888: __u32 = 'X' as ::core::ffi::c_int as __u32
    | ('R' as ::core::ffi::c_int as __u32) << 8 as ::core::ffi::c_int
    | ('2' as ::core::ffi::c_int as __u32) << 16 as ::core::ffi::c_int
    | ('4' as ::core::ffi::c_int as __u32) << 24 as ::core::ffi::c_int;
pub const DRM_FORMAT_XBGR8888: __u32 = 'X' as ::core::ffi::c_int as __u32
    | ('B' as ::core::ffi::c_int as __u32) << 8 as ::core::ffi::c_int
    | ('2' as ::core::ffi::c_int as __u32) << 16 as ::core::ffi::c_int
    | ('4' as ::core::ffi::c_int as __u32) << 24 as ::core::ffi::c_int;
pub const DRM_FORMAT_ARGB8888: __u32 = 'A' as ::core::ffi::c_int as __u32
    | ('R' as ::core::ffi::c_int as __u32) << 8 as ::core::ffi::c_int
    | ('2' as ::core::ffi::c_int as __u32) << 16 as ::core::ffi::c_int
    | ('4' as ::core::ffi::c_int as __u32) << 24 as ::core::ffi::c_int;
pub const DRM_FORMAT_XRGB2101010: __u32 = 'X' as ::core::ffi::c_int as __u32
    | ('R' as ::core::ffi::c_int as __u32) << 8 as ::core::ffi::c_int
    | ('3' as ::core::ffi::c_int as __u32) << 16 as ::core::ffi::c_int
    | ('0' as ::core::ffi::c_int as __u32) << 24 as ::core::ffi::c_int;
pub const DRM_FORMAT_XBGR2101010: __u32 = 'X' as ::core::ffi::c_int as __u32
    | ('B' as ::core::ffi::c_int as __u32) << 8 as ::core::ffi::c_int
    | ('3' as ::core::ffi::c_int as __u32) << 16 as ::core::ffi::c_int
    | ('0' as ::core::ffi::c_int as __u32) << 24 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn output_get_identifier(
    mut identifier: *mut ::core::ffi::c_char,
    mut len: size_t,
    mut output: *mut sway_output,
) {
    let mut wlr_output: *mut wlr_output = (*output).wlr_output;
    snprintf(
        identifier,
        len,
        b"%s %s %s\0".as_ptr() as *const ::core::ffi::c_char,
        if !(*wlr_output).make.is_null() {
            (*wlr_output).make
        } else {
            b"Unknown\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !(*wlr_output).model.is_null() {
            (*wlr_output).model
        } else {
            b"Unknown\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !(*wlr_output).serial.is_null() {
            (*wlr_output).serial
        } else {
            b"Unknown\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn sway_output_scale_filter_to_string(
    mut scale_filter: scale_filter_mode,
) -> *const ::core::ffi::c_char {
    match scale_filter {
        scale_filter_mode::SCALE_FILTER_DEFAULT => {
            return b"smart\0".as_ptr() as *const ::core::ffi::c_char;
        }
        scale_filter_mode::SCALE_FILTER_LINEAR => {
            return b"linear\0".as_ptr() as *const ::core::ffi::c_char;
        }
        scale_filter_mode::SCALE_FILTER_NEAREST => {
            return b"nearest\0".as_ptr() as *const ::core::ffi::c_char;
        }
        scale_filter_mode::SCALE_FILTER_SMART => {
            return b"smart\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    _sway_assert(
        false,
        b"[%s:%d] %s:Unknown value for scale_filter.\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
        51 as ::core::ffi::c_int,
        b"const char *sway_output_scale_filter_to_string(enum scale_filter_mode)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn new_output_config(
    mut name: *const ::core::ffi::c_char,
) -> *mut output_config {
    let mut oc: *mut output_config = calloc(
        1 as size_t,
        ::core::mem::size_of::<output_config>(),
    ) as *mut output_config;
    if oc.is_null() {
        return ::core::ptr::null_mut::<output_config>();
    }
    (*oc).name = strdup(name);
    if (*oc).name.is_null() {
        free(oc as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<output_config>();
    }
    (*oc).enabled = -1 as ::core::ffi::c_int;
    (*oc).height = -1 as ::core::ffi::c_int;
    (*oc).width = (*oc).height;
    (*oc).refresh_rate = -1 as ::core::ffi::c_int as ::core::ffi::c_float;
    (*oc).custom_mode = -1 as ::core::ffi::c_int;
    (*oc).drm_mode.r#type = -1 as ::core::ffi::c_int as uint32_t;
    (*oc).y = INT_MAX;
    (*oc).x = (*oc).y;
    (*oc).scale = -1 as ::core::ffi::c_int as ::core::ffi::c_float;
    (*oc).scale_filter = scale_filter_mode::SCALE_FILTER_DEFAULT;
    (*oc).transform = -1 as ::core::ffi::c_int as int32_t;
    (*oc).subpixel = wl_output_subpixel::WL_OUTPUT_SUBPIXEL_UNKNOWN;
    (*oc).max_render_time = -1 as ::core::ffi::c_int;
    (*oc).adaptive_sync = -1 as ::core::ffi::c_int;
    (*oc).render_bit_depth = render_bit_depth::RENDER_BIT_DEPTH_DEFAULT;
    (*oc).color_profile = color_profile::COLOR_PROFILE_DEFAULT;
    (*oc).color_transform = ::core::ptr::null_mut::<wlr_color_transform>();
    (*oc).power = -1 as ::core::ffi::c_int;
    (*oc).allow_tearing = -1 as ::core::ffi::c_int;
    (*oc).hdr = -1 as ::core::ffi::c_int;
    return oc;
}
unsafe extern "C" fn supersede_output_config(
    mut dst: *mut output_config,
    mut src: *mut output_config,
) {
    if (*src).enabled != -1 as ::core::ffi::c_int {
        (*dst).enabled = -1 as ::core::ffi::c_int;
    }
    if (*src).width != -1 as ::core::ffi::c_int {
        (*dst).width = -1 as ::core::ffi::c_int;
    }
    if (*src).height != -1 as ::core::ffi::c_int {
        (*dst).height = -1 as ::core::ffi::c_int;
    }
    if (*src).x != INT_MAX {
        (*dst).x = INT_MAX;
    }
    if (*src).y != INT_MAX {
        (*dst).y = INT_MAX;
    }
    if (*src).scale != -1 as ::core::ffi::c_int as ::core::ffi::c_float {
        (*dst).scale = -1 as ::core::ffi::c_int as ::core::ffi::c_float;
    }
    if (*src).scale_filter.0 != scale_filter_mode::SCALE_FILTER_DEFAULT.0 {
        (*dst).scale_filter = scale_filter_mode::SCALE_FILTER_DEFAULT;
    }
    if (*src).subpixel.0 != wl_output_subpixel::WL_OUTPUT_SUBPIXEL_UNKNOWN.0 {
        (*dst).subpixel = wl_output_subpixel::WL_OUTPUT_SUBPIXEL_UNKNOWN;
    }
    if (*src).refresh_rate != -1 as ::core::ffi::c_int as ::core::ffi::c_float {
        (*dst).refresh_rate = -1 as ::core::ffi::c_int as ::core::ffi::c_float;
    }
    if (*src).custom_mode != -1 as ::core::ffi::c_int {
        (*dst).custom_mode = -1 as ::core::ffi::c_int;
    }
    if (*src).drm_mode.r#type != -1 as ::core::ffi::c_int as uint32_t {
        (*dst).drm_mode.r#type = -1 as ::core::ffi::c_int as uint32_t;
    }
    if (*src).transform != -1 as int32_t {
        (*dst).transform = -1 as ::core::ffi::c_int as int32_t;
    }
    if (*src).max_render_time != -1 as ::core::ffi::c_int {
        (*dst).max_render_time = -1 as ::core::ffi::c_int;
    }
    if (*src).adaptive_sync != -1 as ::core::ffi::c_int {
        (*dst).adaptive_sync = -1 as ::core::ffi::c_int;
    }
    if (*src).render_bit_depth.0 != render_bit_depth::RENDER_BIT_DEPTH_DEFAULT.0 {
        (*dst).render_bit_depth = render_bit_depth::RENDER_BIT_DEPTH_DEFAULT;
    }
    if (*src).color_profile.0 != color_profile::COLOR_PROFILE_DEFAULT.0 {
        if !(*dst).color_transform.is_null() {
            wlr_color_transform_unref((*dst).color_transform);
            (*dst).color_transform = ::core::ptr::null_mut::<wlr_color_transform>();
        }
        (*dst).color_profile = color_profile::COLOR_PROFILE_DEFAULT;
    }
    if !(*src).background.is_null() {
        free((*dst).background as *mut ::core::ffi::c_void);
        (*dst).background = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*src).background_option.is_null() {
        free((*dst).background_option as *mut ::core::ffi::c_void);
        (*dst).background_option = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*src).background_fallback.is_null() {
        free((*dst).background_fallback as *mut ::core::ffi::c_void);
        (*dst).background_fallback = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*src).power != -1 as ::core::ffi::c_int {
        (*dst).power = -1 as ::core::ffi::c_int;
    }
    if (*src).allow_tearing != -1 as ::core::ffi::c_int {
        (*dst).allow_tearing = -1 as ::core::ffi::c_int;
    }
    if (*src).hdr != -1 as ::core::ffi::c_int {
        (*dst).hdr = -1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn merge_output_config(
    mut dst: *mut output_config,
    mut src: *mut output_config,
) {
    if (*src).enabled != -1 as ::core::ffi::c_int {
        (*dst).enabled = (*src).enabled;
    }
    if (*src).width != -1 as ::core::ffi::c_int {
        (*dst).width = (*src).width;
    }
    if (*src).height != -1 as ::core::ffi::c_int {
        (*dst).height = (*src).height;
    }
    if (*src).x != INT_MAX {
        (*dst).x = (*src).x;
    }
    if (*src).y != INT_MAX {
        (*dst).y = (*src).y;
    }
    if (*src).scale != -1 as ::core::ffi::c_int as ::core::ffi::c_float {
        (*dst).scale = (*src).scale;
    }
    if (*src).scale_filter.0 != scale_filter_mode::SCALE_FILTER_DEFAULT.0 {
        (*dst).scale_filter = (*src).scale_filter;
    }
    if (*src).subpixel.0 != wl_output_subpixel::WL_OUTPUT_SUBPIXEL_UNKNOWN.0 {
        (*dst).subpixel = (*src).subpixel;
    }
    if (*src).refresh_rate != -1 as ::core::ffi::c_int as ::core::ffi::c_float {
        (*dst).refresh_rate = (*src).refresh_rate;
    }
    if (*src).custom_mode != -1 as ::core::ffi::c_int {
        (*dst).custom_mode = (*src).custom_mode;
    }
    if (*src).drm_mode.r#type != -1 as ::core::ffi::c_int as uint32_t {
        memcpy(
            &raw mut (*dst).drm_mode as *mut ::core::ffi::c_void,
            &raw mut (*src).drm_mode as *const ::core::ffi::c_void,
            ::core::mem::size_of::<drmModeModeInfo>(),
        );
    }
    if (*src).transform != -1 as int32_t {
        (*dst).transform = (*src).transform;
    }
    if (*src).max_render_time != -1 as ::core::ffi::c_int {
        (*dst).max_render_time = (*src).max_render_time;
    }
    if (*src).adaptive_sync != -1 as ::core::ffi::c_int {
        (*dst).adaptive_sync = (*src).adaptive_sync;
    }
    if (*src).render_bit_depth.0 != render_bit_depth::RENDER_BIT_DEPTH_DEFAULT.0 {
        (*dst).render_bit_depth = (*src).render_bit_depth;
    }
    if (*src).color_profile.0 != color_profile::COLOR_PROFILE_DEFAULT.0 {
        if !(*src).color_transform.is_null() {
            wlr_color_transform_ref((*src).color_transform);
        }
        wlr_color_transform_unref((*dst).color_transform);
        (*dst).color_profile = (*src).color_profile;
        (*dst).color_transform = (*src).color_transform;
    }
    if !(*src).background.is_null() {
        free((*dst).background as *mut ::core::ffi::c_void);
        (*dst).background = strdup((*src).background);
    }
    if !(*src).background_option.is_null() {
        free((*dst).background_option as *mut ::core::ffi::c_void);
        (*dst).background_option = strdup((*src).background_option);
    }
    if !(*src).background_fallback.is_null() {
        free((*dst).background_fallback as *mut ::core::ffi::c_void);
        (*dst).background_fallback = strdup((*src).background_fallback);
    }
    if (*src).power != -1 as ::core::ffi::c_int {
        (*dst).power = (*src).power;
    }
    if (*src).allow_tearing != -1 as ::core::ffi::c_int {
        (*dst).allow_tearing = (*src).allow_tearing;
    }
    if (*src).hdr != -1 as ::core::ffi::c_int {
        (*dst).hdr = (*src).hdr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn store_output_config(mut oc: *mut output_config) {
    let mut merged: bool = r#false != 0;
    let mut wildcard: bool = strcmp(
        (*oc).name,
        b"*\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    let mut output: *mut sway_output = if wildcard as ::core::ffi::c_int != 0 {
        ::core::ptr::null_mut::<sway_output>()
    } else {
        all_output_by_name_or_id((*oc).name)
    };
    let mut id: [::core::ffi::c_char; 128] = [0; 128];
    if !output.is_null() {
        output_get_identifier(
            &raw mut id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            output,
        );
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*config).output_configs).length {
        let mut old: *mut output_config = *(*(*config).output_configs)
            .items
            .offset(i as isize) as *mut output_config;
        if strcmp((*old).name, (*oc).name) == 0 as ::core::ffi::c_int {
            merge_output_config(old, oc);
            merged = r#true != 0;
        } else if wildcard {
            supersede_output_config(old, oc);
        } else if !output.is_null()
            && strcmp((*old).name, &raw mut id as *mut ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            && strcmp((*oc).name, (*(*output).wlr_output).name)
                == 0 as ::core::ffi::c_int
        {
            supersede_output_config(old, oc);
        }
        i += 1;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Config stored for output %s (enabled: %d) (%dx%d@%fHz position %d,%d scale %f subpixel %s transform %d) (bg %s %s) (power %d) (max render time: %d) (allow tearing: %d) (hdr: %d)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
        279 as ::core::ffi::c_int,
        (*oc).name,
        (*oc).enabled,
        (*oc).width,
        (*oc).height,
        (*oc).refresh_rate as ::core::ffi::c_double,
        (*oc).x,
        (*oc).y,
        (*oc).scale as ::core::ffi::c_double,
        sway_wl_output_subpixel_to_string((*oc).subpixel),
        (*oc).transform,
        (*oc).background,
        (*oc).background_option,
        (*oc).power,
        (*oc).max_render_time,
        (*oc).allow_tearing,
        (*oc).hdr,
    );
    if !merged {
        list_add((*config).output_configs, oc as *mut ::core::ffi::c_void);
    } else {
        free_output_config(oc);
    };
}
unsafe extern "C" fn set_mode(
    mut output: *mut wlr_output,
    mut pending: *mut wlr_output_state,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut refresh_rate: ::core::ffi::c_float,
    mut custom: bool,
) {
    let mut mhz: ::core::ffi::c_int = roundf(
        refresh_rate * 1000 as ::core::ffi::c_int as ::core::ffi::c_float,
    ) as ::core::ffi::c_int;
    mhz = if mhz <= 0 as ::core::ffi::c_int { INT_MAX } else { mhz };
    if wl_list_empty(&raw mut (*output).modes) != 0 || custom as ::core::ffi::c_int != 0
    {
        wlr_output_state_set_custom_mode(
            pending,
            width as int32_t,
            height as int32_t,
            if refresh_rate > 0 as ::core::ffi::c_int as ::core::ffi::c_float {
                mhz as int32_t
            } else {
                0 as int32_t
            },
        );
        return;
    }
    let mut mode: *mut wlr_output_mode = ::core::ptr::null_mut::<wlr_output_mode>();
    let mut best: *mut wlr_output_mode = ::core::ptr::null_mut::<wlr_output_mode>();
    let mut best_diff_mhz: ::core::ffi::c_int = INT_MAX;
    mode = ((*output).modes.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut wlr_output_mode
        as *mut wlr_output_mode;
    while &raw mut (*mode).link != &raw mut (*output).modes {
        if (*mode).width == width as int32_t && (*mode).height == height as int32_t {
            let mut diff_mhz: ::core::ffi::c_int = abs(
                (*mode).refresh as ::core::ffi::c_int - mhz,
            );
            if diff_mhz < best_diff_mhz {
                best_diff_mhz = diff_mhz;
                best = mode;
                if best_diff_mhz == 0 as ::core::ffi::c_int {
                    break;
                }
            }
        }
        mode = ((*mode).link.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut wlr_output_mode
            as *mut wlr_output_mode;
    }
    if best.is_null() {
        best = wlr_output_preferred_mode(output);
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Configured mode (%dx%d@%.3fHz) not available, applying preferred mode (%dx%d@%.3fHz)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            327 as ::core::ffi::c_int,
            width,
            height,
            refresh_rate as ::core::ffi::c_double,
            (*best).width,
            (*best).height,
            ((*best).refresh as ::core::ffi::c_float / 1000.0f32)
                as ::core::ffi::c_double,
        );
    }
    wlr_output_state_set_mode(pending, best);
}
unsafe extern "C" fn set_modeline(
    mut output: *mut wlr_output,
    mut pending: *mut wlr_output_state,
    mut drm_mode: *mut drmModeModeInfo,
) {
    if !wlr_output_is_drm(output) {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Modeline can only be set to DRM output\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            339 as ::core::ffi::c_int,
        );
        return;
    }
    let mut mode: *mut wlr_output_mode = wlr_drm_connector_add_mode(output, drm_mode);
    if !mode.is_null() {
        wlr_output_state_set_mode(pending, mode);
    }
}
#[no_mangle]
pub unsafe extern "C" fn output_supports_hdr(
    mut output: *mut wlr_output,
    mut unsupported_reason_ptr: *mut *const ::core::ffi::c_char,
) -> bool {
    let mut unsupported_reason: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    if (*output).supported_primaries
        & wlr_color_named_primaries::WLR_COLOR_NAMED_PRIMARIES_BT2020.0
            as ::core::ffi::c_int as uint32_t == 0
    {
        unsupported_reason = b"BT2020 primaries not supported by output\0".as_ptr()
            as *const ::core::ffi::c_char;
    } else if (*output).supported_transfer_functions
        & wlr_color_transfer_function::WLR_COLOR_TRANSFER_FUNCTION_ST2084_PQ.0
            as ::core::ffi::c_int as uint32_t == 0
    {
        unsupported_reason = b"PQ transfer function not supported by output\0".as_ptr()
            as *const ::core::ffi::c_char;
    } else if !(*server.renderer).features.output_color_transform {
        unsupported_reason = b"renderer doesn't support output color transforms\0"
            .as_ptr() as *const ::core::ffi::c_char;
    }
    if !unsupported_reason_ptr.is_null() {
        *unsupported_reason_ptr = unsupported_reason;
    }
    return unsupported_reason.is_null();
}
unsafe extern "C" fn set_hdr(
    mut output: *mut wlr_output,
    mut pending: *mut wlr_output_state,
    mut enabled: bool,
) {
    let mut unsupported_reason: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    if enabled as ::core::ffi::c_int != 0
        && !output_supports_hdr(output, &raw mut unsupported_reason)
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Cannot enable HDR on output %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            369 as ::core::ffi::c_int,
            (*output).name,
            unsupported_reason,
        );
        enabled = r#false != 0;
    }
    if !enabled {
        if (*output).supported_primaries != 0 as uint32_t
            || (*output).supported_transfer_functions != 0 as uint32_t
        {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Disabling HDR on output %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                376 as ::core::ffi::c_int,
                (*output).name,
            );
            wlr_output_state_set_image_description(
                pending,
                ::core::ptr::null::<wlr_output_image_description>(),
            );
        }
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Enabling HDR on output %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
        382 as ::core::ffi::c_int,
        (*output).name,
    );
    let image_desc: wlr_output_image_description = wlr_output_image_description {
        primaries: wlr_color_named_primaries::WLR_COLOR_NAMED_PRIMARIES_BT2020,
        transfer_function: wlr_color_transfer_function::WLR_COLOR_TRANSFER_FUNCTION_ST2084_PQ,
        mastering_display_primaries: wlr_color_primaries {
            red: wlr_color_cie1931_xy {
                x: 0.,
                y: 0.,
            },
            green: wlr_color_cie1931_xy {
                x: 0.,
                y: 0.,
            },
            blue: wlr_color_cie1931_xy {
                x: 0.,
                y: 0.,
            },
            white: wlr_color_cie1931_xy {
                x: 0.,
                y: 0.,
            },
        },
        mastering_luminance: C2Rust_Unnamed_12 {
            min: 0.,
            max: 0.,
        },
        max_cll: 0.,
        max_fall: 0.,
    };
    wlr_output_state_set_image_description(pending, &raw const image_desc);
}
unsafe extern "C" fn phys_size_is_aspect_ratio(mut output: *mut wlr_output) -> bool {
    return (*output).phys_width == 1600 as int32_t
        && (*output).phys_height == 900 as int32_t
        || (*output).phys_width == 1600 as int32_t
            && (*output).phys_height == 1000 as int32_t
        || (*output).phys_width == 160 as int32_t
            && (*output).phys_height == 90 as int32_t
        || (*output).phys_width == 160 as int32_t
            && (*output).phys_height == 100 as int32_t
        || (*output).phys_width == 16 as int32_t && (*output).phys_height == 9 as int32_t
        || (*output).phys_width == 16 as int32_t
            && (*output).phys_height == 10 as int32_t;
}
pub const HIDPI_DPI_LIMIT: ::core::ffi::c_int = 2 as ::core::ffi::c_int
    * 96 as ::core::ffi::c_int;
pub const HIDPI_MIN_HEIGHT: ::core::ffi::c_int = 1200 as ::core::ffi::c_int;
pub const MM_PER_INCH: ::core::ffi::c_double = 25.4f64;
unsafe extern "C" fn compute_default_scale(
    mut output: *mut wlr_output,
    mut pending: *mut wlr_output_state,
) -> ::core::ffi::c_int {
    let mut r#box: wlr_box = wlr_box {
        x: 0,
        y: 0,
        width: (*output).width as ::core::ffi::c_int,
        height: (*output).height as ::core::ffi::c_int,
    };
    if (*pending).committed
        & wlr_output_state_field::WLR_OUTPUT_STATE_MODE.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        match (*pending).mode_type {
            wlr_output_state_mode_type::WLR_OUTPUT_STATE_MODE_FIXED => {
                r#box.width = (*(*pending).mode).width as ::core::ffi::c_int;
                r#box.height = (*(*pending).mode).height as ::core::ffi::c_int;
            }
            wlr_output_state_mode_type::WLR_OUTPUT_STATE_MODE_CUSTOM => {
                r#box.width = (*pending).custom_mode.width as ::core::ffi::c_int;
                r#box.height = (*pending).custom_mode.height as ::core::ffi::c_int;
            }
            _ => {}
        }
    }
    let mut transform: wl_output_transform = (*output).transform;
    if (*pending).committed
        & wlr_output_state_field::WLR_OUTPUT_STATE_TRANSFORM.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        transform = (*pending).transform;
    }
    wlr_box_transform(
        &raw mut r#box,
        &raw mut r#box,
        transform,
        r#box.width,
        r#box.height,
    );
    let mut width: ::core::ffi::c_int = r#box.width;
    let mut height: ::core::ffi::c_int = r#box.height;
    if height < HIDPI_MIN_HEIGHT {
        return 1 as ::core::ffi::c_int;
    }
    if (*output).phys_width == 0 as int32_t || (*output).phys_height == 0 as int32_t {
        return 1 as ::core::ffi::c_int;
    }
    if phys_size_is_aspect_ratio(output) {
        return 1 as ::core::ffi::c_int;
    }
    let mut dpi_x: ::core::ffi::c_double = width as ::core::ffi::c_double
        / ((*output).phys_width as ::core::ffi::c_double / MM_PER_INCH);
    let mut dpi_y: ::core::ffi::c_double = height as ::core::ffi::c_double
        / ((*output).phys_height as ::core::ffi::c_double / MM_PER_INCH);
    if dpi_x <= HIDPI_DPI_LIMIT as ::core::ffi::c_double
        || dpi_y <= HIDPI_DPI_LIMIT as ::core::ffi::c_double
    {
        return 1 as ::core::ffi::c_int;
    }
    return 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn bit_depth_from_format(
    mut render_format: uint32_t,
) -> render_bit_depth {
    if render_format == DRM_FORMAT_XRGB2101010 as uint32_t
        || render_format == DRM_FORMAT_XBGR2101010 as uint32_t
    {
        return render_bit_depth::RENDER_BIT_DEPTH_10
    } else if render_format == DRM_FORMAT_XRGB8888 as uint32_t
        || render_format == DRM_FORMAT_ARGB8888 as uint32_t
    {
        return render_bit_depth::RENDER_BIT_DEPTH_8
    } else if render_format == DRM_FORMAT_RGB565 as uint32_t {
        return render_bit_depth::RENDER_BIT_DEPTH_6
    }
    return render_bit_depth::RENDER_BIT_DEPTH_DEFAULT;
}
unsafe extern "C" fn get_config_render_bit_depth(
    mut oc: *const output_config,
) -> render_bit_depth {
    if !oc.is_null()
        && (*oc).render_bit_depth.0 != render_bit_depth::RENDER_BIT_DEPTH_DEFAULT.0
    {
        return (*oc).render_bit_depth;
    }
    if !oc.is_null() && (*oc).hdr == 1 as ::core::ffi::c_int {
        return render_bit_depth::RENDER_BIT_DEPTH_10;
    }
    return render_bit_depth::RENDER_BIT_DEPTH_8;
}
unsafe extern "C" fn render_format_is_bgr(mut fmt: uint32_t) -> bool {
    return fmt == DRM_FORMAT_XBGR2101010 as uint32_t
        || fmt == DRM_FORMAT_XBGR8888 as uint32_t;
}
unsafe extern "C" fn output_config_is_disabling(mut oc: *mut output_config) -> bool {
    return !oc.is_null()
        && ((*oc).enabled == 0 || (*oc).power == 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn queue_output_config(
    mut oc: *mut output_config,
    mut output: *mut sway_output,
    mut pending: *mut wlr_output_state,
) {
    if output == (*root).fallback_output {
        return;
    }
    let mut wlr_output: *mut wlr_output = (*output).wlr_output;
    if output_config_is_disabling(oc) {
        wlr_output_state_set_enabled(pending, r#false != 0);
        return;
    }
    wlr_output_state_set_enabled(pending, r#true != 0);
    if !oc.is_null() && (*oc).drm_mode.r#type != 0 as uint32_t
        && (*oc).drm_mode.r#type != -1 as ::core::ffi::c_int as uint32_t
    {
        set_modeline(wlr_output, pending, &raw mut (*oc).drm_mode);
    } else if !oc.is_null() && (*oc).width > 0 as ::core::ffi::c_int
        && (*oc).height > 0 as ::core::ffi::c_int
    {
        set_mode(
            wlr_output,
            pending,
            (*oc).width,
            (*oc).height,
            (*oc).refresh_rate,
            (*oc).custom_mode == 1 as ::core::ffi::c_int,
        );
    } else if wl_list_empty(&raw mut (*wlr_output).modes) == 0 {
        let mut preferred_mode: *mut wlr_output_mode = wlr_output_preferred_mode(
            wlr_output,
        );
        wlr_output_state_set_mode(pending, preferred_mode);
    }
    if !oc.is_null()
        && (*oc).subpixel.0 != wl_output_subpixel::WL_OUTPUT_SUBPIXEL_UNKNOWN.0
    {
        wlr_output_state_set_subpixel(pending, (*oc).subpixel);
    } else {
        wlr_output_state_set_subpixel(pending, (*output).detected_subpixel);
    }
    if !oc.is_null() && (*oc).transform >= 0 as int32_t {
        wlr_output_state_set_transform(
            pending,
            wl_output_transform((*oc).transform as ::core::ffi::c_uint),
        );
    } else if wlr_output_is_drm(wlr_output) {
        wlr_output_state_set_transform(
            pending,
            wlr_drm_connector_get_panel_orientation(wlr_output),
        );
    } else {
        wlr_output_state_set_transform(
            pending,
            wl_output_transform::WL_OUTPUT_TRANSFORM_NORMAL,
        );
    }
    if !oc.is_null() && (*oc).scale > 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        wlr_output_state_set_scale(
            pending,
            (round(
                ((*oc).scale * 120 as ::core::ffi::c_int as ::core::ffi::c_float)
                    as ::core::ffi::c_double,
            ) / 120 as ::core::ffi::c_int as ::core::ffi::c_double)
                as ::core::ffi::c_float,
        );
    } else {
        wlr_output_state_set_scale(
            pending,
            compute_default_scale(wlr_output, pending) as ::core::ffi::c_float,
        );
    }
    if (*wlr_output).adaptive_sync_supported {
        if !oc.is_null() && (*oc).adaptive_sync != -1 as ::core::ffi::c_int {
            wlr_output_state_set_adaptive_sync_enabled(
                pending,
                (*oc).adaptive_sync == 1 as ::core::ffi::c_int,
            );
        } else {
            wlr_output_state_set_adaptive_sync_enabled(pending, r#false != 0);
        }
    }
    let mut render_bit_depth_0: render_bit_depth = get_config_render_bit_depth(oc);
    if render_bit_depth_0.0 == render_bit_depth::RENDER_BIT_DEPTH_10.0
        && bit_depth_from_format((*(*output).wlr_output).render_format).0
            == render_bit_depth_0.0
    {
        wlr_output_state_set_render_format(
            pending,
            (*(*output).wlr_output).render_format,
        );
    } else if render_bit_depth_0.0 == render_bit_depth::RENDER_BIT_DEPTH_10.0 {
        wlr_output_state_set_render_format(pending, DRM_FORMAT_XRGB2101010 as uint32_t);
    } else if render_bit_depth_0.0 == render_bit_depth::RENDER_BIT_DEPTH_6.0 {
        wlr_output_state_set_render_format(pending, DRM_FORMAT_RGB565 as uint32_t);
    } else {
        wlr_output_state_set_render_format(pending, DRM_FORMAT_XRGB8888 as uint32_t);
    }
    let mut hdr: bool = !oc.is_null() && (*oc).hdr == 1 as ::core::ffi::c_int;
    let mut color_profile_0: bool = !oc.is_null()
        && (!(*oc).color_transform.is_null()
            || (*oc).color_profile.0
                == color_profile::COLOR_PROFILE_TRANSFORM_WITH_DEVICE_PRIMARIES.0);
    if hdr as ::core::ffi::c_int != 0 && color_profile_0 as ::core::ffi::c_int != 0 {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Cannot use HDR on output %s: output has a color profile set\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            561 as ::core::ffi::c_int,
            (*wlr_output).name,
        );
        hdr = r#false != 0;
    }
    set_hdr(wlr_output, pending, hdr);
}
unsafe extern "C" fn config_output_state_finish(mut state: *mut config_output_state) {
    wlr_color_transform_unref((*state).color_transform);
}
unsafe extern "C" fn color_profile_from_device(
    mut wlr_output: *mut wlr_output,
    mut transfer_function: *mut wlr_color_transform,
) -> *mut wlr_color_transform {
    let mut srgb_primaries: wlr_color_primaries = wlr_color_primaries {
        red: wlr_color_cie1931_xy {
            x: 0.,
            y: 0.,
        },
        green: wlr_color_cie1931_xy {
            x: 0.,
            y: 0.,
        },
        blue: wlr_color_cie1931_xy {
            x: 0.,
            y: 0.,
        },
        white: wlr_color_cie1931_xy {
            x: 0.,
            y: 0.,
        },
    };
    wlr_color_primaries_from_named(
        &raw mut srgb_primaries,
        wlr_color_named_primaries::WLR_COLOR_NAMED_PRIMARIES_SRGB,
    );
    let mut primaries: *const wlr_color_primaries = (*wlr_output).default_primaries;
    if primaries.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] output has no reported color information\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            582 as ::core::ffi::c_int,
        );
        if !transfer_function.is_null() {
            wlr_color_transform_ref(transfer_function);
        }
        return transfer_function;
    } else if memcmp(
        primaries as *const ::core::ffi::c_void,
        &raw mut srgb_primaries as *const ::core::ffi::c_void,
        ::core::mem::size_of::<wlr_color_primaries>(),
    ) == 0 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] output reports sRGB colors, no correction needed\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            588 as ::core::ffi::c_int,
        );
        if !transfer_function.is_null() {
            wlr_color_transform_ref(transfer_function);
        }
        return transfer_function;
    } else {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Creating color profile from reported color primaries: R(%f, %f) G(%f, %f) B(%f, %f) W(%f, %f)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            594 as ::core::ffi::c_int,
            (*primaries).red.x as ::core::ffi::c_double,
            (*primaries).red.y as ::core::ffi::c_double,
            (*primaries).green.x as ::core::ffi::c_double,
            (*primaries).green.y as ::core::ffi::c_double,
            (*primaries).blue.x as ::core::ffi::c_double,
            (*primaries).blue.y as ::core::ffi::c_double,
            (*primaries).white.x as ::core::ffi::c_double,
            (*primaries).white.y as ::core::ffi::c_double,
        );
        let mut matrix: [::core::ffi::c_float; 9] = [0.; 9];
        wlr_color_primaries_transform_absolute_colorimetric(
            &raw mut srgb_primaries,
            primaries,
            &raw mut matrix as *mut ::core::ffi::c_float,
        );
        let mut matrix_transform: *mut wlr_color_transform = wlr_color_transform_init_matrix(
            &raw mut matrix as *mut ::core::ffi::c_float,
        );
        if matrix_transform.is_null() {
            return ::core::ptr::null_mut::<wlr_color_transform>();
        }
        let mut resolved_tf: *mut wlr_color_transform = if !transfer_function.is_null() {
            wlr_color_transform_ref(transfer_function)
        } else {
            wlr_color_transform_init_linear_to_inverse_eotf(
                wlr_color_transfer_function::WLR_COLOR_TRANSFER_FUNCTION_GAMMA22,
            )
        };
        if resolved_tf.is_null() {
            wlr_color_transform_unref(matrix_transform);
            return ::core::ptr::null_mut::<wlr_color_transform>();
        }
        let mut transforms: [*mut wlr_color_transform; 2] = [
            matrix_transform,
            resolved_tf,
        ];
        let mut transforms_len: size_t = ::core::mem::size_of::<
            [*mut wlr_color_transform; 2],
        >()
            .wrapping_div(::core::mem::size_of::<*mut wlr_color_transform>());
        let mut result: *mut wlr_color_transform = wlr_color_transform_init_pipeline(
            &raw mut transforms as *mut *mut wlr_color_transform,
            transforms_len,
        );
        wlr_color_transform_unref(matrix_transform);
        wlr_color_transform_unref(resolved_tf);
        return result;
    };
}
unsafe extern "C" fn get_color_profile(
    mut output: *mut wlr_output,
    mut oc: *mut output_config,
) -> *mut wlr_color_transform {
    if !oc.is_null() && (*oc).color_profile.0 == color_profile::COLOR_PROFILE_TRANSFORM.0
    {
        if !(*oc).color_transform.is_null() {
            wlr_color_transform_ref((*oc).color_transform);
        }
        return (*oc).color_transform;
    } else if !oc.is_null()
        && (*oc).color_profile.0
            == color_profile::COLOR_PROFILE_TRANSFORM_WITH_DEVICE_PRIMARIES.0
    {
        return color_profile_from_device(output, (*oc).color_transform)
    } else {
        return ::core::ptr::null_mut::<wlr_color_transform>()
    };
}
unsafe extern "C" fn finalize_output_config(
    mut oc: *mut output_config,
    mut output: *mut sway_output,
    mut applied: *const wlr_output_state,
    mut config_applied: *const config_output_state,
) -> bool {
    if output == (*root).fallback_output {
        return r#false != 0;
    }
    let mut wlr_output: *mut wlr_output = (*output).wlr_output;
    if !oc.is_null() && (*oc).enabled == 0 {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Disabling output %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            642 as ::core::ffi::c_int,
            (*oc).name,
        );
        if (*output).enabled {
            output_disable(output);
            wlr_output_layout_remove((*root).output_layout, wlr_output);
        }
        return r#true != 0;
    }
    let mut scale_filter_old: scale_filter_mode = (*output).scale_filter;
    let mut scale_filter_new: scale_filter_mode = scale_filter_mode(
        if !oc.is_null() {
            (*oc).scale_filter.0
        } else {
            scale_filter_mode::SCALE_FILTER_DEFAULT.0
        },
    );
    match scale_filter_new {
        scale_filter_mode::SCALE_FILTER_DEFAULT
        | scale_filter_mode::SCALE_FILTER_SMART => {
            (*output).scale_filter = scale_filter_mode(
                (if ceilf((*wlr_output).scale) == (*wlr_output).scale {
                    scale_filter_mode::SCALE_FILTER_NEAREST.0 as ::core::ffi::c_int
                } else {
                    scale_filter_mode::SCALE_FILTER_LINEAR.0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            );
        }
        scale_filter_mode::SCALE_FILTER_LINEAR
        | scale_filter_mode::SCALE_FILTER_NEAREST => {
            (*output).scale_filter = scale_filter_new;
        }
        _ => {}
    }
    if scale_filter_old.0 != (*output).scale_filter.0 {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Set %s scale_filter to %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            664 as ::core::ffi::c_int,
            (*oc).name,
            sway_output_scale_filter_to_string((*output).scale_filter),
        );
        wlr_damage_ring_add_whole(&raw mut (*(*output).scene_output).damage_ring);
    }
    if !oc.is_null() && (*oc).x != INT_MAX && (*oc).y != INT_MAX {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Set %s position to %d, %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            671 as ::core::ffi::c_int,
            (*oc).name,
            (*oc).x,
            (*oc).y,
        );
        wlr_output_layout_add((*root).output_layout, wlr_output, (*oc).x, (*oc).y);
    } else {
        wlr_output_layout_add_auto((*root).output_layout, wlr_output);
    }
    if !(*output).enabled {
        output_enable(output);
    }
    wlr_color_transform_unref((*output).color_transform);
    if !(*config_applied).color_transform.is_null() {
        wlr_color_transform_ref((*config_applied).color_transform);
    }
    (*output).color_transform = (*config_applied).color_transform;
    (*output).max_render_time = if !oc.is_null()
        && (*oc).max_render_time > 0 as ::core::ffi::c_int
    {
        (*oc).max_render_time
    } else {
        0 as ::core::ffi::c_int
    };
    (*output).allow_tearing = !oc.is_null()
        && (*oc).allow_tearing > 0 as ::core::ffi::c_int;
    (*output).hdr = !(*applied).image_description.is_null();
    return r#true != 0;
}
unsafe extern "C" fn output_update_position(mut output: *mut sway_output) {
    let mut output_box: wlr_box = wlr_box {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    wlr_output_layout_get_box(
        (*root).output_layout,
        (*output).wlr_output,
        &raw mut output_box,
    );
    (*output).lx = output_box.x;
    (*output).ly = output_box.y;
    (*output).width = output_box.width;
    (*output).height = output_box.height;
}
unsafe extern "C" fn find_output_config_from_list(
    mut configs: *mut *mut output_config,
    mut configs_len: size_t,
    mut sway_output: *mut sway_output,
) -> *mut output_config {
    let mut name: *const ::core::ffi::c_char = (*(*sway_output).wlr_output).name;
    let mut result: *mut output_config = new_output_config(name);
    if result.is_null() {
        return ::core::ptr::null_mut::<output_config>();
    }
    let mut id: [::core::ffi::c_char; 128] = [0; 128];
    output_get_identifier(
        &raw mut id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        sway_output,
    );
    let mut oc: *mut output_config = ::core::ptr::null_mut::<output_config>();
    let mut names: [*const ::core::ffi::c_char; 4] = [
        b"*\0".as_ptr() as *const ::core::ffi::c_char,
        name,
        &raw mut id as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut name_0: *mut *const ::core::ffi::c_char = (&raw mut names
        as *mut *const ::core::ffi::c_char)
        .offset(0isize);
    while !(*name_0).is_null() {
        let mut idx: size_t = 0 as size_t;
        while idx < configs_len {
            oc = *configs.offset(idx as isize);
            if strcmp((*oc).name, *name_0) == 0 as ::core::ffi::c_int {
                merge_output_config(result, oc);
            }
            idx = idx.wrapping_add(1);
        }
        name_0 = name_0.offset(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn find_output_config(
    mut sway_output: *mut sway_output,
) -> *mut output_config {
    return find_output_config_from_list(
        (*(*config).output_configs).items as *mut *mut output_config,
        (*(*config).output_configs).length as size_t,
        sway_output,
    );
}
unsafe extern "C" fn config_has_manual_mode(mut oc: *mut output_config) -> bool {
    if oc.is_null() {
        return r#false != 0;
    }
    if (*oc).drm_mode.r#type != 0 as uint32_t
        && (*oc).drm_mode.r#type != -1 as ::core::ffi::c_int as uint32_t
    {
        return r#true != 0
    } else if (*oc).width > 0 as ::core::ffi::c_int
        && (*oc).height > 0 as ::core::ffi::c_int
    {
        return r#true != 0
    }
    return r#false != 0;
}
unsafe extern "C" fn dump_output_state(
    mut wlr_output: *mut wlr_output,
    mut state: *mut wlr_output_state,
) {
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Output state for %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
        769 as ::core::ffi::c_int,
        (*wlr_output).name,
    );
    if (*state).committed
        & wlr_output_state_field::WLR_OUTPUT_STATE_ENABLED.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d]     enabled:       %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            771 as ::core::ffi::c_int,
            if (*state).enabled as ::core::ffi::c_int != 0 {
                b"yes\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"no\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    if (*state).committed
        & wlr_output_state_field::WLR_OUTPUT_STATE_RENDER_FORMAT.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        let mut format_name: *mut ::core::ffi::c_char = drmGetFormatName(
            (*state).render_format,
        );
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d]     render_format: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            775 as ::core::ffi::c_int,
            format_name,
        );
        free(format_name as *mut ::core::ffi::c_void);
    }
    if (*state).committed
        & wlr_output_state_field::WLR_OUTPUT_STATE_MODE.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        if (*state).mode_type.0
            == wlr_output_state_mode_type::WLR_OUTPUT_STATE_MODE_CUSTOM.0
        {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d]     custom mode:   %dx%d@%dmHz\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                780 as ::core::ffi::c_int,
                (*state).custom_mode.width,
                (*state).custom_mode.height,
                (*state).custom_mode.refresh,
            );
        } else {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d]     mode:          %dx%d@%dmHz%s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                783 as ::core::ffi::c_int,
                (*(*state).mode).width,
                (*(*state).mode).height,
                (*(*state).mode).refresh,
                if (*(*state).mode).preferred as ::core::ffi::c_int != 0 {
                    b" (preferred)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
    }
    if (*state).committed
        & wlr_output_state_field::WLR_OUTPUT_STATE_ADAPTIVE_SYNC_ENABLED.0
            as ::core::ffi::c_int as uint32_t != 0
    {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d]     adaptive_sync: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            789 as ::core::ffi::c_int,
            if (*state).adaptive_sync_enabled as ::core::ffi::c_int != 0 {
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disabled\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    if (*state).committed
        & wlr_output_state_field::WLR_OUTPUT_STATE_SCALE.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d]     scale:         %f\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            793 as ::core::ffi::c_int,
            (*state).scale as ::core::ffi::c_double,
        );
    }
    if (*state).committed
        & wlr_output_state_field::WLR_OUTPUT_STATE_SUBPIXEL.0 as ::core::ffi::c_int
            as uint32_t != 0
    {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d]     subpixel:      %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            796 as ::core::ffi::c_int,
            sway_wl_output_subpixel_to_string((*state).subpixel),
        );
    }
}
unsafe extern "C" fn reset_output_state(mut state: *mut wlr_output_state) {
    wlr_output_state_finish(state);
    wlr_output_state_init(state);
    (*state).committed = 0 as uint32_t;
}
unsafe extern "C" fn clear_later_output_states(
    mut states: *mut wlr_backend_output_state,
    mut configs_len: size_t,
    mut output_idx: size_t,
) {
    let mut idx: size_t = output_idx.wrapping_add(1 as size_t);
    while idx < configs_len {
        let mut backend_state: *mut wlr_backend_output_state = states
            .offset(idx as isize);
        let mut state: *mut wlr_output_state = &raw mut (*backend_state).base;
        reset_output_state(state);
        wlr_output_state_set_enabled(state, r#false != 0);
        idx = idx.wrapping_add(1);
    }
}
unsafe extern "C" fn search_finish(
    mut ctx: *mut search_context,
    mut output_idx: size_t,
) -> bool {
    let mut backend_state: *mut wlr_backend_output_state = (*ctx)
        .states
        .offset(output_idx as isize);
    let mut state: *mut wlr_output_state = &raw mut (*backend_state).base;
    let mut wlr_output: *mut wlr_output = (*backend_state).output;
    clear_later_output_states((*ctx).states, (*ctx).configs_len, output_idx);
    dump_output_state(wlr_output, state);
    return wlr_output_swapchain_manager_prepare(
        (*ctx).swapchain_mgr,
        (*ctx).states,
        (*ctx).configs_len,
    ) as ::core::ffi::c_int != 0
        && search_valid_config(ctx, output_idx.wrapping_add(1 as size_t))
            as ::core::ffi::c_int != 0;
}
unsafe extern "C" fn search_adaptive_sync(
    mut ctx: *mut search_context,
    mut output_idx: size_t,
) -> bool {
    let mut cfg: *mut matched_output_config = (*ctx).configs.offset(output_idx as isize);
    let mut backend_state: *mut wlr_backend_output_state = (*ctx)
        .states
        .offset(output_idx as isize);
    let mut state: *mut wlr_output_state = &raw mut (*backend_state).base;
    if !(*(*backend_state).output).adaptive_sync_supported {
        return search_finish(ctx, output_idx);
    }
    if !(*cfg).config.is_null()
        && (*(*cfg).config).adaptive_sync == 1 as ::core::ffi::c_int
    {
        wlr_output_state_set_adaptive_sync_enabled(state, r#true != 0);
        if search_finish(ctx, output_idx) {
            return r#true != 0;
        }
    }
    wlr_output_state_set_adaptive_sync_enabled(state, r#false != 0);
    return search_finish(ctx, output_idx);
}
unsafe extern "C" fn search_mode(
    mut ctx: *mut search_context,
    mut output_idx: size_t,
) -> bool {
    let mut cfg: *mut matched_output_config = (*ctx).configs.offset(output_idx as isize);
    let mut backend_state: *mut wlr_backend_output_state = (*ctx)
        .states
        .offset(output_idx as isize);
    let mut state: *mut wlr_output_state = &raw mut (*backend_state).base;
    let mut wlr_output: *mut wlr_output = (*backend_state).output;
    if config_has_manual_mode((*cfg).config) {
        return search_adaptive_sync(ctx, output_idx);
    }
    let mut preferred_mode: *mut wlr_output_mode = wlr_output_preferred_mode(wlr_output);
    if !preferred_mode.is_null() {
        wlr_output_state_set_mode(state, preferred_mode);
        if search_adaptive_sync(ctx, output_idx) {
            return r#true != 0;
        }
    }
    if wl_list_empty(&raw mut (*wlr_output).modes) != 0 {
        (*state).committed
            &= !(wlr_output_state_field::WLR_OUTPUT_STATE_MODE.0 as ::core::ffi::c_int)
                as uint32_t;
        return search_adaptive_sync(ctx, output_idx);
    }
    let mut mode: *mut wlr_output_mode = ::core::ptr::null_mut::<wlr_output_mode>();
    mode = ((*(*backend_state).output).modes.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut wlr_output_mode
        as *mut wlr_output_mode;
    while &raw mut (*mode).link != &raw mut (*(*backend_state).output).modes {
        if mode != preferred_mode {
            wlr_output_state_set_mode(state, mode);
            if search_adaptive_sync(ctx, output_idx) {
                return r#true != 0;
            }
        }
        mode = ((*mode).link.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut wlr_output_mode
            as *mut wlr_output_mode;
    }
    return r#false != 0;
}
unsafe extern "C" fn search_render_format(
    mut ctx: *mut search_context,
    mut output_idx: size_t,
) -> bool {
    let mut cfg: *mut matched_output_config = (*ctx).configs.offset(output_idx as isize);
    let mut backend_state: *mut wlr_backend_output_state = (*ctx)
        .states
        .offset(output_idx as isize);
    let mut state: *mut wlr_output_state = &raw mut (*backend_state).base;
    let mut wlr_output: *mut wlr_output = (*backend_state).output;
    let mut fmts: [uint32_t; 6] = [
        DRM_FORMAT_XRGB2101010 as uint32_t,
        DRM_FORMAT_XBGR2101010 as uint32_t,
        DRM_FORMAT_XRGB8888 as uint32_t,
        DRM_FORMAT_ARGB8888 as uint32_t,
        DRM_FORMAT_RGB565 as uint32_t,
        DRM_FORMAT_INVALID as uint32_t,
    ];
    if render_format_is_bgr((*wlr_output).render_format) {
        fmts[0usize] = DRM_FORMAT_XBGR2101010 as uint32_t;
        fmts[1usize] = DRM_FORMAT_XRGB2101010 as uint32_t;
    }
    let mut primary_formats: *const wlr_drm_format_set = wlr_output_get_primary_formats(
        wlr_output,
        (*server.allocator).buffer_caps,
    );
    let mut needed_bits: render_bit_depth = get_config_render_bit_depth((*cfg).config);
    let mut idx: size_t = 0 as size_t;
    while fmts[idx] != DRM_FORMAT_INVALID as uint32_t {
        let mut format_bits: render_bit_depth = bit_depth_from_format(fmts[idx]);
        if needed_bits.0 >= format_bits.0 {
            if !(!primary_formats.is_null()
                && wlr_drm_format_set_get(primary_formats, fmts[idx]).is_null())
            {
                wlr_output_state_set_render_format(state, fmts[idx]);
                if search_mode(ctx, output_idx) {
                    return r#true != 0;
                }
            }
        }
        idx = idx.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn search_valid_config(
    mut ctx: *mut search_context,
    mut output_idx: size_t,
) -> bool {
    if output_idx >= (*ctx).configs_len {
        return r#true != 0;
    }
    let mut cfg: *mut matched_output_config = (*ctx).configs.offset(output_idx as isize);
    let mut backend_state: *mut wlr_backend_output_state = (*ctx)
        .states
        .offset(output_idx as isize);
    let mut state: *mut wlr_output_state = &raw mut (*backend_state).base;
    let mut wlr_output: *mut wlr_output = (*backend_state).output;
    if !output_config_is_disabling((*cfg).config) {
        queue_output_config(
            (*cfg).config,
            (*cfg).output,
            &raw mut (*backend_state).base,
        );
        if search_render_format(ctx, output_idx) {
            return r#true != 0
        } else if !(*ctx).degrade_to_off {
            return r#false != 0
        }
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Unable to find valid config with output %s, disabling\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            956 as ::core::ffi::c_int,
            (*wlr_output).name,
        );
        reset_output_state(state);
    }
    wlr_output_state_set_enabled(state, r#false != 0);
    return search_finish(ctx, output_idx);
}
unsafe extern "C" fn compare_matched_output_config_priority(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut amc: *const matched_output_config = a as *const matched_output_config;
    let mut bmc: *const matched_output_config = b as *const matched_output_config;
    let mut a_disabling: bool = output_config_is_disabling((*amc).config);
    let mut b_disabling: bool = output_config_is_disabling((*bmc).config);
    let mut a_enabled: bool = (*(*amc).output).enabled;
    let mut b_enabled: bool = (*(*bmc).output).enabled;
    if a_enabled as ::core::ffi::c_int != 0 && !a_disabling {
        return -1 as ::core::ffi::c_int
    } else if b_enabled as ::core::ffi::c_int != 0 && !b_disabling {
        return 1 as ::core::ffi::c_int
    } else if b_disabling as ::core::ffi::c_int != 0 && !a_disabling {
        return -1 as ::core::ffi::c_int
    } else if a_disabling as ::core::ffi::c_int != 0 && !b_disabling {
        return 1 as ::core::ffi::c_int
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn sort_output_configs_by_priority(
    mut configs: *mut matched_output_config,
    mut configs_len: size_t,
) {
    qsort(
        configs as *mut ::core::ffi::c_void,
        configs_len,
        ::core::mem::size_of::<matched_output_config>(),
        Some(
            compare_matched_output_config_priority
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
}
unsafe extern "C" fn apply_resolved_output_configs(
    mut configs: *mut matched_output_config,
    mut configs_len: size_t,
    mut test_only: bool,
    mut degrade_to_off: bool,
) -> bool {
    let mut states: *mut wlr_backend_output_state = calloc(
        configs_len,
        ::core::mem::size_of::<wlr_backend_output_state>(),
    ) as *mut wlr_backend_output_state;
    if states.is_null() {
        return r#false != 0;
    }
    let mut config_states: *mut config_output_state = calloc(
        configs_len,
        ::core::mem::size_of::<config_output_state>(),
    ) as *mut config_output_state;
    if config_states.is_null() {
        free(states as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Committing %zd outputs\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
        1008 as ::core::ffi::c_int,
        configs_len,
    );
    let mut idx: size_t = 0 as size_t;
    while idx < configs_len {
        let mut cfg: *mut matched_output_config = configs.offset(idx as isize);
        let mut backend_state: *mut wlr_backend_output_state = states
            .offset(idx as isize);
        let mut config_state: *mut config_output_state = config_states
            .offset(idx as isize);
        (*backend_state).output = (*(*cfg).output).wlr_output;
        wlr_output_state_init(&raw mut (*backend_state).base);
        queue_output_config(
            (*cfg).config,
            (*cfg).output,
            &raw mut (*backend_state).base,
        );
        dump_output_state((*(*cfg).output).wlr_output, &raw mut (*backend_state).base);
        (*config_state).color_transform = get_color_profile(
            (*(*cfg).output).wlr_output,
            (*cfg).config,
        );
        idx = idx.wrapping_add(1);
    }
    let mut swapchain_mgr: wlr_output_swapchain_manager = wlr_output_swapchain_manager {
        backend: ::core::ptr::null_mut::<wlr_backend>(),
        WLR_PRIVATE: C2Rust_Unnamed_27 {
            outputs: wl_array {
                size: 0,
                alloc: 0,
                data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            },
        },
    };
    wlr_output_swapchain_manager_init(&raw mut swapchain_mgr, server.backend);
    let mut ok: bool = wlr_output_swapchain_manager_prepare(
        &raw mut swapchain_mgr,
        states,
        configs_len,
    );
    '_out: {
        if !ok {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Requested backend configuration failed, searching for valid fallbacks\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                1028 as ::core::ffi::c_int,
            );
            let mut ctx: search_context = search_context {
                swapchain_mgr: &raw mut swapchain_mgr,
                states: states,
                configs: configs,
                configs_len: configs_len,
                degrade_to_off: degrade_to_off,
            };
            if !search_valid_config(&raw mut ctx, 0 as size_t) {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] Search for valid config failed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1037 as ::core::ffi::c_int,
                );
                break '_out;
            }
        }
        if !test_only {
            let mut idx_0: size_t = 0 as size_t;
            while idx_0 < configs_len {
                let mut cfg_0: *mut matched_output_config = configs
                    .offset(idx_0 as isize);
                let mut backend_state_0: *mut wlr_backend_output_state = states
                    .offset(idx_0 as isize);
                let mut config_state_0: *mut config_output_state = config_states
                    .offset(idx_0 as isize);
                let mut opts: wlr_scene_output_state_options = wlr_scene_output_state_options {
                    timer: ::core::ptr::null_mut::<wlr_scene_timer>(),
                    color_transform: (*config_state_0).color_transform,
                    swapchain: wlr_output_swapchain_manager_get_swapchain(
                        &raw mut swapchain_mgr,
                        (*backend_state_0).output,
                    ),
                };
                let mut scene_output: *mut wlr_scene_output = (*(*cfg_0).output)
                    .scene_output;
                let mut state: *mut wlr_output_state = &raw mut (*backend_state_0).base;
                if !wlr_scene_output_build_state(scene_output, state, &raw mut opts) {
                    _sway_log(
                        sway_log_importance_t::SWAY_ERROR,
                        b"[%s:%d] Building output state for '%s' failed\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1060 as ::core::ffi::c_int,
                        (*(*backend_state_0).output).name,
                    );
                    break '_out;
                } else {
                    idx_0 = idx_0.wrapping_add(1);
                }
            }
            ok = wlr_backend_commit(server.backend, states, configs_len);
            if !ok {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] Backend commit failed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1068 as ::core::ffi::c_int,
                );
            } else {
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Commit of %zd outputs succeeded\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1072 as ::core::ffi::c_int,
                    configs_len,
                );
                wlr_output_swapchain_manager_apply(&raw mut swapchain_mgr);
                let mut idx_1: size_t = 0 as size_t;
                while idx_1 < configs_len {
                    let mut cfg_1: *mut matched_output_config = configs
                        .offset(idx_1 as isize);
                    let mut backend_state_1: *mut wlr_backend_output_state = states
                        .offset(idx_1 as isize);
                    let mut config_state_1: *mut config_output_state = config_states
                        .offset(idx_1 as isize);
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Finalizing config for %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1080 as ::core::ffi::c_int,
                        (*(*(*cfg_1).output).wlr_output).name,
                    );
                    finalize_output_config(
                        (*cfg_1).config,
                        (*cfg_1).output,
                        &raw mut (*backend_state_1).base,
                        config_state_1,
                    );
                    idx_1 = idx_1.wrapping_add(1);
                }
                let mut idx_2: size_t = 0 as size_t;
                while idx_2 < configs_len {
                    let mut cfg_2: *mut matched_output_config = configs
                        .offset(idx_2 as isize);
                    output_update_position((*cfg_2).output);
                    arrange_layers((*cfg_2).output);
                    idx_2 = idx_2.wrapping_add(1);
                }
                arrange_root();
                arrange_locks();
                update_output_manager_config(&raw mut server);
                transaction_commit_dirty();
            }
        }
    }
    wlr_output_swapchain_manager_finish(&raw mut swapchain_mgr);
    let mut idx_3: size_t = 0 as size_t;
    while idx_3 < configs_len {
        let mut backend_state_2: *mut wlr_backend_output_state = states
            .offset(idx_3 as isize);
        wlr_output_state_finish(&raw mut (*backend_state_2).base);
        config_output_state_finish(config_states.offset(idx_3 as isize));
        idx_3 = idx_3.wrapping_add(1);
    }
    free(states as *mut ::core::ffi::c_void);
    free(config_states as *mut ::core::ffi::c_void);
    input_manager_configure_all_input_mappings();
    input_manager_configure_xcursor();
    let mut seat: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
    seat = ((*server.input).seats.next as *mut ::core::ffi::c_char)
        .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
        as *mut sway_seat;
    while &raw mut (*seat).link != &raw mut (*server.input).seats {
        wlr_seat_pointer_notify_clear_focus((*seat).wlr_seat);
        cursor_rebase((*seat).cursor);
        seat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
            as *mut sway_seat;
    }
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn apply_output_configs(
    mut ocs: *mut *mut output_config,
    mut ocs_len: size_t,
    mut test_only: bool,
    mut degrade_to_off: bool,
) -> bool {
    let mut configs_len: size_t = wl_list_length(&raw mut (*root).all_outputs) as size_t;
    let mut configs: *mut matched_output_config = calloc(
        configs_len,
        ::core::mem::size_of::<matched_output_config>(),
    ) as *mut matched_output_config;
    if configs.is_null() {
        return r#false != 0;
    }
    let mut config_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sway_output: *mut sway_output = ::core::ptr::null_mut::<sway_output>();
    sway_output = ((*root).all_outputs.next as *mut ::core::ffi::c_char)
        .offset(-(152 as ::core::ffi::c_ulong as isize)) as *mut sway_output
        as *mut sway_output;
    while &raw mut (*sway_output).link != &raw mut (*root).all_outputs {
        if sway_output == (*root).fallback_output {
            configs_len = configs_len.wrapping_sub(1);
        } else {
            let c2rust_fresh0 = config_idx;
            config_idx += 1;
            let mut config_0: *mut matched_output_config = configs
                .offset(c2rust_fresh0 as isize);
            (*config_0).output = sway_output;
            (*config_0).config = find_output_config_from_list(ocs, ocs_len, sway_output);
        }
        sway_output = ((*sway_output).link.next as *mut ::core::ffi::c_char)
            .offset(-(152 as ::core::ffi::c_ulong as isize)) as *mut sway_output
            as *mut sway_output;
    }
    sort_output_configs_by_priority(configs, configs_len);
    let mut ok: bool = apply_resolved_output_configs(
        configs,
        configs_len,
        test_only,
        degrade_to_off,
    );
    let mut idx: size_t = 0 as size_t;
    while idx < configs_len {
        let mut cfg: *mut matched_output_config = configs.offset(idx as isize);
        free_output_config((*cfg).config);
        idx = idx.wrapping_add(1);
    }
    free(configs as *mut ::core::ffi::c_void);
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn apply_stored_output_configs() {
    apply_output_configs(
        (*(*config).output_configs).items as *mut *mut output_config,
        (*(*config).output_configs).length as size_t,
        r#false != 0,
        r#true != 0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn free_output_config(mut oc: *mut output_config) {
    if oc.is_null() {
        return;
    }
    free((*oc).name as *mut ::core::ffi::c_void);
    free((*oc).background as *mut ::core::ffi::c_void);
    free((*oc).background_option as *mut ::core::ffi::c_void);
    free((*oc).background_fallback as *mut ::core::ffi::c_void);
    wlr_color_transform_unref((*oc).color_transform);
    free(oc as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn handle_swaybg_client_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut sway_config: *mut sway_config = (listener as *mut ::core::ffi::c_char)
        .offset(-(312 as ::core::ffi::c_ulong as isize)) as *mut sway_config;
    wl_list_remove(&raw mut (*sway_config).swaybg_client_destroy.link);
    wl_list_init(&raw mut (*sway_config).swaybg_client_destroy.link);
    (*sway_config).swaybg_client = ::core::ptr::null_mut::<wl_client>();
}
unsafe extern "C" fn _spawn_swaybg(mut command: *mut *mut ::core::ffi::c_char) -> bool {
    if !(*config).swaybg_client.is_null() {
        wl_client_destroy((*config).swaybg_client);
    }
    let mut sockets: [::core::ffi::c_int; 2] = [0; 2];
    if socketpair(
        AF_UNIX,
        __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        &raw mut sockets as *mut ::core::ffi::c_int,
    ) != 0 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] socketpair failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            1187 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        return r#false != 0;
    }
    if !sway_set_cloexec(sockets[0usize], r#true != 0)
        || !sway_set_cloexec(sockets[1usize], r#true != 0)
    {
        return r#false != 0;
    }
    (*config).swaybg_client = wl_client_create(server.wl_display, sockets[0usize]);
    if (*config).swaybg_client.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] wl_client_create failed: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            1196 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        return r#false != 0;
    }
    (*config).swaybg_client_destroy.notify = Some(
        handle_swaybg_client_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_client_add_destroy_listener(
        (*config).swaybg_client,
        &raw mut (*config).swaybg_client_destroy,
    );
    let mut pid: pid_t = fork();
    if pid < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] fork failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            1206 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        return r#false != 0;
    } else if pid == 0 as ::core::ffi::c_int {
        if !sway_set_cloexec(sockets[1usize], r#false != 0) {
            _exit(EXIT_FAILURE);
        }
        let mut wayland_socket_str: [::core::ffi::c_char; 16] = [0; 16];
        snprintf(
            &raw mut wayland_socket_str as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            sockets[1usize],
        );
        setenv(
            b"WAYLAND_SOCKET\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut wayland_socket_str as *mut ::core::ffi::c_char,
            r#true,
        );
        execvp(*command.offset(0isize), command);
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] failed to execute '%s' (background configuration probably not applied): %s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            1219 as ::core::ffi::c_int,
            *command.offset(0isize),
            strerror(*__errno_location()),
        );
        _exit(EXIT_FAILURE);
    }
    if close(sockets[1usize]) != 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] close failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            1226 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        return r#false != 0;
    }
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn spawn_swaybg() -> bool {
    if (*config).swaybg_command.is_null() {
        return r#true != 0;
    }
    let mut length: size_t = 2 as size_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*config).output_configs).length {
        let mut oc: *mut output_config = *(*(*config).output_configs)
            .items
            .offset(i as isize) as *mut output_config;
        if !(*oc).background.is_null() {
            if strcmp(
                (*oc).background_option,
                b"solid_color\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                length = length.wrapping_add(4 as size_t);
            } else if !(*oc).background_fallback.is_null() {
                length = length.wrapping_add(8 as size_t);
            } else {
                length = length.wrapping_add(6 as size_t);
            }
        }
        i += 1;
    }
    let mut cmd: *mut *mut ::core::ffi::c_char = calloc(
        length,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
    ) as *mut *mut ::core::ffi::c_char;
    if cmd.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to allocate spawn_swaybg command\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            1254 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    let mut i_0: size_t = 0 as size_t;
    let c2rust_fresh1 = i_0;
    i_0 = i_0.wrapping_add(1);
    *cmd.offset(c2rust_fresh1 as isize) = (*config).swaybg_command;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j < (*(*config).output_configs).length {
        let mut oc_0: *mut output_config = *(*(*config).output_configs)
            .items
            .offset(j as isize) as *mut output_config;
        if !(*oc_0).background.is_null() {
            if strcmp(
                (*oc_0).background_option,
                b"solid_color\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let c2rust_fresh2 = i_0;
                i_0 = i_0.wrapping_add(1);
                *cmd.offset(c2rust_fresh2 as isize) = b"-o\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                let c2rust_fresh3 = i_0;
                i_0 = i_0.wrapping_add(1);
                *cmd.offset(c2rust_fresh3 as isize) = (*oc_0).name;
                let c2rust_fresh4 = i_0;
                i_0 = i_0.wrapping_add(1);
                *cmd.offset(c2rust_fresh4 as isize) = b"-c\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                let c2rust_fresh5 = i_0;
                i_0 = i_0.wrapping_add(1);
                *cmd.offset(c2rust_fresh5 as isize) = (*oc_0).background;
            } else {
                let c2rust_fresh6 = i_0;
                i_0 = i_0.wrapping_add(1);
                *cmd.offset(c2rust_fresh6 as isize) = b"-o\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                let c2rust_fresh7 = i_0;
                i_0 = i_0.wrapping_add(1);
                *cmd.offset(c2rust_fresh7 as isize) = (*oc_0).name;
                let c2rust_fresh8 = i_0;
                i_0 = i_0.wrapping_add(1);
                *cmd.offset(c2rust_fresh8 as isize) = b"-i\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                let c2rust_fresh9 = i_0;
                i_0 = i_0.wrapping_add(1);
                *cmd.offset(c2rust_fresh9 as isize) = (*oc_0).background;
                let c2rust_fresh10 = i_0;
                i_0 = i_0.wrapping_add(1);
                *cmd.offset(c2rust_fresh10 as isize) = b"-m\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                let c2rust_fresh11 = i_0;
                i_0 = i_0.wrapping_add(1);
                *cmd.offset(c2rust_fresh11 as isize) = (*oc_0).background_option;
                if !(*oc_0).background_fallback.is_null() {
                    let c2rust_fresh12 = i_0;
                    i_0 = i_0.wrapping_add(1);
                    *cmd.offset(c2rust_fresh12 as isize) = b"-c\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                    let c2rust_fresh13 = i_0;
                    i_0 = i_0.wrapping_add(1);
                    *cmd.offset(c2rust_fresh13 as isize) = (*oc_0).background_fallback;
                }
            }
            if i_0 <= length {} else {
                __assert_fail(
                    b"i <= length\0".as_ptr() as *const ::core::ffi::c_char,
                    b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1282 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            };
        }
        j += 1;
    }
    let mut k: size_t = 0 as size_t;
    while k < i_0 {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] spawn_swaybg cmd[%zd] = %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config/output.c\0".as_ptr() as *const ::core::ffi::c_char,
            1286 as ::core::ffi::c_int,
            k,
            *cmd.offset(k as isize),
        );
        k = k.wrapping_add(1);
    }
    let mut result: bool = _spawn_swaybg(cmd);
    free(cmd as *mut ::core::ffi::c_void);
    return result;
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
