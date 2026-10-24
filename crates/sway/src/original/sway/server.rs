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
    pub type wlr_output_layer_state;
    pub type wlr_session;
    pub type wlr_tablet_client_v2;
    pub type wlr_tablet_impl;
    pub type wlr_tablet_tool_client_v2;
    pub type xkb_state;
    pub type xkb_keymap;
    pub type wlr_keyboard_group;
    pub type wlr_keyboard_impl;
    pub type wlr_primary_selection_source;
    pub type wlr_ext_image_capture_source_v1_interface;
    pub type wlr_linux_dmabuf_feedback_v1_compiled;
    pub type wlr_output_layer_feedback_event;
    pub type wlr_tablet_pad_client_v2;
    pub type wlr_tablet_pad;
    pub type sway_transaction;
    pub type wlr_xwm;
    pub type sway_transaction_instruction;
    pub type sway_text_node;
    pub type launcher_ctx;
    pub type sway_xdg_decoration;
    pub type sway_switch;
    pub type sway_keyboard;
    pub type wlr_touch_impl;
    pub type wlr_cursor;
    pub type _PangoFontDescription;
    pub type wlr_drm_lease;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn unsetenv(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn wl_list_init(list: *mut wl_list);
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn wl_event_loop_add_idle(
        r#loop: *mut wl_event_loop,
        func: wl_event_loop_idle_func_t,
        data: *mut ::core::ffi::c_void,
    ) -> *mut wl_event_source;
    fn wl_display_create() -> *mut wl_display;
    fn wl_display_destroy(display: *mut wl_display);
    fn wl_display_get_event_loop(display: *mut wl_display) -> *mut wl_event_loop;
    fn wl_display_add_socket(
        display: *mut wl_display,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn wl_display_run(display: *mut wl_display);
    fn wl_display_destroy_clients(display: *mut wl_display);
    fn wl_display_set_default_max_buffer_size(
        display: *mut wl_display,
        max_buffer_size: size_t,
    );
    fn wl_display_set_global_filter(
        display: *mut wl_display,
        filter: wl_display_global_filter_func_t,
        data: *mut ::core::ffi::c_void,
    );
    fn wlr_renderer_autocreate(backend: *mut wlr_backend) -> *mut wlr_renderer;
    fn wlr_renderer_get_texture_formats(
        r: *mut wlr_renderer,
        buffer_caps: uint32_t,
    ) -> *const wlr_drm_format_set;
    fn wlr_renderer_init_wl_shm(
        r: *mut wlr_renderer,
        wl_display: *mut wl_display,
    ) -> bool;
    fn wlr_renderer_get_drm_fd(r: *mut wlr_renderer) -> ::core::ffi::c_int;
    fn wlr_renderer_destroy(renderer: *mut wlr_renderer);
    fn wlr_output_init_render(
        output: *mut wlr_output,
        allocator: *mut wlr_allocator,
        renderer: *mut wlr_renderer,
    ) -> bool;
    fn wlr_output_set_name(output: *mut wlr_output, name: *const ::core::ffi::c_char);
    fn wlr_backend_autocreate(
        r#loop: *mut wl_event_loop,
        session_ptr: *mut *mut wlr_session,
    ) -> *mut wlr_backend;
    fn wlr_backend_start(backend: *mut wlr_backend) -> bool;
    fn wlr_backend_destroy(backend: *mut wlr_backend);
    fn wlr_backend_get_drm_fd(backend: *mut wlr_backend) -> ::core::ffi::c_int;
    fn wlr_multi_backend_add(multi: *mut wlr_backend, backend: *mut wlr_backend) -> bool;
    fn wlr_multi_for_each_backend(
        backend: *mut wlr_backend,
        callback: Option<
            unsafe extern "C" fn(*mut wlr_backend, *mut ::core::ffi::c_void) -> (),
        >,
        data: *mut ::core::ffi::c_void,
    );
    fn wlr_allocator_autocreate(
        backend: *mut wlr_backend,
        renderer: *mut wlr_renderer,
    ) -> *mut wlr_allocator;
    fn wlr_allocator_destroy(alloc: *mut wlr_allocator);
    fn wlr_color_manager_v1_create(
        display: *mut wl_display,
        version: uint32_t,
        options: *const wlr_color_manager_v1_options,
    ) -> *mut wlr_color_manager_v1;
    fn wlr_color_manager_v1_transfer_function_list_from_renderer(
        renderer: *mut wlr_renderer,
        len: *mut size_t,
    ) -> *mut wp_color_manager_v1_transfer_function;
    fn wlr_color_manager_v1_primaries_list_from_renderer(
        renderer: *mut wlr_renderer,
        len: *mut size_t,
    ) -> *mut wp_color_manager_v1_primaries;
    fn wlr_alpha_modifier_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_alpha_modifier_v1;
    fn wlr_color_representation_manager_v1_create_with_renderer(
        display: *mut wl_display,
        version: uint32_t,
        renderer: *mut wlr_renderer,
    ) -> *mut wlr_color_representation_manager_v1;
    fn wlr_content_type_manager_v1_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_content_type_manager_v1;
    fn wlr_cursor_shape_manager_v1_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_cursor_shape_manager_v1;
    fn wlr_data_device_manager_create(
        display: *mut wl_display,
    ) -> *mut wlr_data_device_manager;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn wlr_ext_data_control_manager_v1_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_ext_data_control_manager_v1;
    fn wlr_data_control_manager_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_data_control_manager_v1;
    fn wlr_ext_foreign_toplevel_list_v1_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_ext_foreign_toplevel_list_v1;
    fn wlr_fixes_create(display: *mut wl_display, version: uint32_t) -> *mut wlr_fixes;
    fn wlr_foreign_toplevel_manager_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_foreign_toplevel_manager_v1;
    fn wlr_tablet_v2_create(display: *mut wl_display) -> *mut wlr_tablet_manager_v2;
    fn wlr_scene_set_gamma_control_manager_v1(
        scene: *mut wlr_scene,
        gamma_control: *mut wlr_gamma_control_manager_v1,
    );
    fn wlr_scene_set_color_manager_v1(
        scene: *mut wlr_scene,
        manager: *mut wlr_color_manager_v1,
    );
    fn wlr_ext_output_image_capture_source_manager_v1_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_ext_output_image_capture_source_manager_v1;
    fn wlr_ext_foreign_toplevel_image_capture_source_manager_v1_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_ext_foreign_toplevel_image_capture_source_manager_v1;
    fn wlr_ext_foreign_toplevel_image_capture_source_manager_v1_request_accept(
        request: *mut wlr_ext_foreign_toplevel_image_capture_source_manager_v1_request_event,
        source: *mut wlr_ext_image_capture_source_v1,
    ) -> bool;
    fn wlr_ext_image_capture_source_v1_create_with_scene_node(
        node: *mut wlr_scene_node,
        event_loop: *mut wl_event_loop,
        allocator: *mut wlr_allocator,
        renderer: *mut wlr_renderer,
    ) -> *mut wlr_ext_image_capture_source_v1;
    fn wlr_ext_image_copy_capture_manager_v1_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_ext_image_copy_capture_manager_v1;
    fn wlr_headless_backend_create(r#loop: *mut wl_event_loop) -> *mut wlr_backend;
    fn wlr_headless_add_output(
        backend: *mut wlr_backend,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
    ) -> *mut wlr_output;
    fn wlr_gamma_control_manager_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_gamma_control_manager_v1;
    fn wlr_idle_notifier_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_idle_notifier_v1;
    fn wlr_layer_shell_v1_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_layer_shell_v1;
    fn wlr_linux_dmabuf_v1_create_with_renderer(
        display: *mut wl_display,
        version: uint32_t,
        renderer: *mut wlr_renderer,
    ) -> *mut wlr_linux_dmabuf_v1;
    fn wlr_linux_drm_syncobj_manager_v1_create(
        display: *mut wl_display,
        version: uint32_t,
        drm_fd: ::core::ffi::c_int,
    ) -> *mut wlr_linux_drm_syncobj_manager_v1;
    fn wlr_output_manager_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_output_manager_v1;
    fn wlr_output_power_manager_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_output_power_manager_v1;
    fn wlr_pointer_constraints_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_pointer_constraints_v1;
    fn wlr_presentation_create(
        display: *mut wl_display,
        backend: *mut wlr_backend,
        version: uint32_t,
    ) -> *mut wlr_presentation;
    fn wlr_primary_selection_v1_device_manager_create(
        display: *mut wl_display,
    ) -> *mut wlr_primary_selection_v1_device_manager;
    fn wlr_relative_pointer_manager_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_relative_pointer_manager_v1;
    fn wlr_screencopy_manager_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_screencopy_manager_v1;
    fn wlr_security_context_manager_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_security_context_manager_v1;
    fn wlr_security_context_manager_v1_lookup_client(
        manager: *mut wlr_security_context_manager_v1,
        client: *const wl_client,
    ) -> *const wlr_security_context_v1_state;
    fn wlr_server_decoration_manager_create(
        display: *mut wl_display,
    ) -> *mut wlr_server_decoration_manager;
    fn wlr_server_decoration_manager_set_default_mode(
        manager: *mut wlr_server_decoration_manager,
        default_mode: uint32_t,
    );
    fn wlr_single_pixel_buffer_manager_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_single_pixel_buffer_manager_v1;
    fn wlr_subcompositor_create(display: *mut wl_display) -> *mut wlr_subcompositor;
    fn wlr_viewporter_create(display: *mut wl_display) -> *mut wlr_viewporter;
    fn wlr_xdg_activation_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_xdg_activation_v1;
    fn wlr_xdg_shell_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_xdg_shell;
    fn wlr_xdg_decoration_manager_v1_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_xdg_decoration_manager_v1;
    fn wlr_xdg_foreign_registry_create(
        display: *mut wl_display,
    ) -> *mut wlr_xdg_foreign_registry;
    fn wlr_xdg_foreign_v1_create(
        display: *mut wl_display,
        registry: *mut wlr_xdg_foreign_registry,
    ) -> *mut wlr_xdg_foreign_v1;
    fn wlr_xdg_foreign_v2_create(
        display: *mut wl_display,
        registry: *mut wlr_xdg_foreign_registry,
    ) -> *mut wlr_xdg_foreign_v2;
    fn wlr_xdg_output_manager_v1_create(
        display: *mut wl_display,
        layout: *mut wlr_output_layout,
    ) -> *mut wlr_xdg_output_manager_v1;
    fn wlr_xdg_toplevel_tag_manager_v1_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_xdg_toplevel_tag_manager_v1;
    fn drmGetVersion(fd: ::core::ffi::c_int) -> drmVersionPtr;
    fn drmFreeVersion(_: drmVersionPtr);
    fn create_list() -> *mut list_t;
    fn list_free(list: *mut list_t);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    static mut root: *mut sway_root;
    fn root_create(display: *mut wl_display) -> *mut sway_root;
    fn output_create(wlr_output: *mut wlr_output) -> *mut sway_output;
    fn handle_output_manager_apply(
        listener: *mut wl_listener,
        data: *mut ::core::ffi::c_void,
    );
    fn handle_output_manager_test(
        listener: *mut wl_listener,
        data: *mut ::core::ffi::c_void,
    );
    fn handle_output_power_manager_set_mode(
        listener: *mut wl_listener,
        data: *mut ::core::ffi::c_void,
    );
    fn sway_ext_workspace_init() -> bool;
    fn wlr_fractional_scale_manager_v1_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_fractional_scale_manager_v1;
    fn sway_idle_inhibit_manager_v1_init() -> bool;
    fn wlr_export_dmabuf_manager_v1_create(
        display: *mut wl_display,
    ) -> *mut wlr_export_dmabuf_manager_v1;
    fn wlr_compositor_create(
        display: *mut wl_display,
        version: uint32_t,
        renderer: *mut wlr_renderer,
    ) -> *mut wlr_compositor;
    fn wlr_compositor_set_renderer(
        compositor: *mut wlr_compositor,
        renderer: *mut wlr_renderer,
    );
    fn wlr_xwayland_create(
        wl_display: *mut wl_display,
        compositor: *mut wlr_compositor,
        lazy: bool,
    ) -> *mut wlr_xwayland;
    fn wlr_xwayland_destroy(wlr_xwayland: *mut wlr_xwayland);
    fn wlr_drm_lease_v1_manager_create(
        display: *mut wl_display,
        backend: *mut wlr_backend,
    ) -> *mut wlr_drm_lease_v1_manager;
    fn wlr_drm_lease_request_v1_grant(
        request: *mut wlr_drm_lease_request_v1,
    ) -> *mut wlr_drm_lease_v1;
    fn wlr_drm_lease_request_v1_reject(request: *mut wlr_drm_lease_request_v1);
    fn wlr_tearing_control_manager_v1_create(
        display: *mut wl_display,
        version: uint32_t,
    ) -> *mut wlr_tearing_control_manager_v1;
    fn wlr_text_input_manager_v3_create(
        wl_display: *mut wl_display,
    ) -> *mut wlr_text_input_manager_v3;
    fn handle_request_set_cursor_shape(
        listener: *mut wl_listener,
        data: *mut ::core::ffi::c_void,
    );
    fn xdg_toplevel_tag_manager_v1_handle_set_tag(
        listener: *mut wl_listener,
        data: *mut ::core::ffi::c_void,
    );
    static mut config: *mut sway_config;
    static mut server: sway_server;
    fn handle_new_output(listener: *mut wl_listener, data: *mut ::core::ffi::c_void);
    fn handle_layer_shell_surface(
        listener: *mut wl_listener,
        data: *mut ::core::ffi::c_void,
    );
    fn sway_session_lock_init() -> bool;
    fn handle_xdg_shell_toplevel(
        listener: *mut wl_listener,
        data: *mut ::core::ffi::c_void,
    );
    fn handle_xwayland_surface(
        listener: *mut wl_listener,
        data: *mut ::core::ffi::c_void,
    );
    fn handle_server_decoration(
        listener: *mut wl_listener,
        data: *mut ::core::ffi::c_void,
    );
    fn handle_xdg_decoration(listener: *mut wl_listener, data: *mut ::core::ffi::c_void);
    fn handle_pointer_constraint(
        listener: *mut wl_listener,
        data: *mut ::core::ffi::c_void,
    );
    fn xdg_activation_v1_handle_request_activate(
        listener: *mut wl_listener,
        data: *mut ::core::ffi::c_void,
    );
    fn xdg_activation_v1_handle_new_token(
        listener: *mut wl_listener,
        data: *mut ::core::ffi::c_void,
    );
    fn handle_new_tearing_hint(
        listener: *mut wl_listener,
        data: *mut ::core::ffi::c_void,
    );
    fn handle_xwayland_ready(listener: *mut wl_listener, data: *mut ::core::ffi::c_void);
    fn input_manager_create(server_0: *mut sway_server) -> *mut sway_input_manager;
    fn input_manager_finish(input: *mut sway_input_manager);
    fn input_manager_get_default_seat() -> *mut sway_seat;
    fn wlr_input_method_manager_v2_create(
        display: *mut wl_display,
    ) -> *mut wlr_input_method_manager_v2;
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type dev_t = __dev_t;
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
pub type wl_event_loop_idle_func_t = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
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
pub type wl_display_global_filter_func_t = Option<
    unsafe extern "C" fn(
        *const wl_client,
        *const wl_global,
        *mut ::core::ffi::c_void,
    ) -> bool,
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
pub struct wl_data_device_manager_dnd_action(pub ::core::ffi::c_uint);
impl wl_data_device_manager_dnd_action {
    pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_NONE: Self = Self(0);
    pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY: Self = Self(1);
    pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE: Self = Self(2);
    pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_ASK: Self = Self(4);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_buffer_cap(pub ::core::ffi::c_uint);
impl wlr_buffer_cap {
    pub const WLR_BUFFER_CAP_DATA_PTR: Self = Self(1);
    pub const WLR_BUFFER_CAP_DMABUF: Self = Self(2);
    pub const WLR_BUFFER_CAP_SHM: Self = Self(4);
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
pub struct wlr_allocator {
    pub r#impl: *const wlr_allocator_interface,
    pub buffer_caps: uint32_t,
    pub events: C2Rust_Unnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
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
    pub events: C2Rust_Unnamed_17,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_16,
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
pub struct C2Rust_Unnamed_16 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
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
    pub viewport: C2Rust_Unnamed_18,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
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
    pub events: C2Rust_Unnamed_20,
    pub WLR_PRIVATE: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_alpha_modifier_v1 {
    pub global: *mut wl_global,
    pub WLR_PRIVATE: C2Rust_Unnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub display_destroy: wl_listener,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wp_color_manager_v1_render_intent(pub ::core::ffi::c_uint);
impl wp_color_manager_v1_render_intent {
    pub const WP_COLOR_MANAGER_V1_RENDER_INTENT_PERCEPTUAL: Self = Self(0);
    pub const WP_COLOR_MANAGER_V1_RENDER_INTENT_RELATIVE: Self = Self(1);
    pub const WP_COLOR_MANAGER_V1_RENDER_INTENT_SATURATION: Self = Self(2);
    pub const WP_COLOR_MANAGER_V1_RENDER_INTENT_ABSOLUTE: Self = Self(3);
    pub const WP_COLOR_MANAGER_V1_RENDER_INTENT_RELATIVE_BPC: Self = Self(4);
    pub const WP_COLOR_MANAGER_V1_RENDER_INTENT_ABSOLUTE_NO_ADAPTATION: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wp_color_manager_v1_primaries(pub ::core::ffi::c_uint);
impl wp_color_manager_v1_primaries {
    pub const WP_COLOR_MANAGER_V1_PRIMARIES_SRGB: Self = Self(1);
    pub const WP_COLOR_MANAGER_V1_PRIMARIES_PAL_M: Self = Self(2);
    pub const WP_COLOR_MANAGER_V1_PRIMARIES_PAL: Self = Self(3);
    pub const WP_COLOR_MANAGER_V1_PRIMARIES_NTSC: Self = Self(4);
    pub const WP_COLOR_MANAGER_V1_PRIMARIES_GENERIC_FILM: Self = Self(5);
    pub const WP_COLOR_MANAGER_V1_PRIMARIES_BT2020: Self = Self(6);
    pub const WP_COLOR_MANAGER_V1_PRIMARIES_CIE1931_XYZ: Self = Self(7);
    pub const WP_COLOR_MANAGER_V1_PRIMARIES_DCI_P3: Self = Self(8);
    pub const WP_COLOR_MANAGER_V1_PRIMARIES_DISPLAY_P3: Self = Self(9);
    pub const WP_COLOR_MANAGER_V1_PRIMARIES_ADOBE_RGB: Self = Self(10);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wp_color_manager_v1_transfer_function(pub ::core::ffi::c_uint);
impl wp_color_manager_v1_transfer_function {
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_BT1886: Self = Self(1);
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_GAMMA22: Self = Self(2);
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_GAMMA28: Self = Self(3);
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_ST240: Self = Self(4);
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_EXT_LINEAR: Self = Self(5);
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_LOG_100: Self = Self(6);
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_LOG_316: Self = Self(7);
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_XVYCC: Self = Self(8);
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_SRGB: Self = Self(9);
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_EXT_SRGB: Self = Self(10);
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_ST2084_PQ: Self = Self(11);
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_ST428: Self = Self(12);
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_HLG: Self = Self(13);
    pub const WP_COLOR_MANAGER_V1_TRANSFER_FUNCTION_COMPOUND_POWER_2_4: Self = Self(14);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_color_manager_v1_features {
    pub icc_v2_v4: bool,
    pub parametric: bool,
    pub set_primaries: bool,
    pub set_tf_power: bool,
    pub set_luminances: bool,
    pub set_mastering_display_primaries: bool,
    pub extended_target_volume: bool,
    pub windows_scrgb: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_color_manager_v1_options {
    pub features: wlr_color_manager_v1_features,
    pub render_intents: *const wp_color_manager_v1_render_intent,
    pub render_intents_len: size_t,
    pub transfer_functions: *const wp_color_manager_v1_transfer_function,
    pub transfer_functions_len: size_t,
    pub primaries: *const wp_color_manager_v1_primaries,
    pub primaries_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_color_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_23,
    pub WLR_PRIVATE: C2Rust_Unnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub features: wlr_color_manager_v1_features,
    pub render_intents: *mut wp_color_manager_v1_render_intent,
    pub render_intents_len: size_t,
    pub transfer_functions: *mut wp_color_manager_v1_transfer_function,
    pub transfer_functions_len: size_t,
    pub primaries: *mut wp_color_manager_v1_primaries,
    pub primaries_len: size_t,
    pub outputs: wl_list,
    pub surface_feedbacks: wl_list,
    pub last_image_desc_identity: uint64_t,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub destroy: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wp_color_representation_surface_v1_alpha_mode(pub ::core::ffi::c_uint);
impl wp_color_representation_surface_v1_alpha_mode {
    pub const WP_COLOR_REPRESENTATION_SURFACE_V1_ALPHA_MODE_PREMULTIPLIED_ELECTRICAL: Self = Self(
        0,
    );
    pub const WP_COLOR_REPRESENTATION_SURFACE_V1_ALPHA_MODE_PREMULTIPLIED_OPTICAL: Self = Self(
        1,
    );
    pub const WP_COLOR_REPRESENTATION_SURFACE_V1_ALPHA_MODE_STRAIGHT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wp_color_representation_surface_v1_coefficients(pub ::core::ffi::c_uint);
impl wp_color_representation_surface_v1_coefficients {
    pub const WP_COLOR_REPRESENTATION_SURFACE_V1_COEFFICIENTS_IDENTITY: Self = Self(1);
    pub const WP_COLOR_REPRESENTATION_SURFACE_V1_COEFFICIENTS_BT709: Self = Self(2);
    pub const WP_COLOR_REPRESENTATION_SURFACE_V1_COEFFICIENTS_FCC: Self = Self(3);
    pub const WP_COLOR_REPRESENTATION_SURFACE_V1_COEFFICIENTS_BT601: Self = Self(4);
    pub const WP_COLOR_REPRESENTATION_SURFACE_V1_COEFFICIENTS_SMPTE240: Self = Self(5);
    pub const WP_COLOR_REPRESENTATION_SURFACE_V1_COEFFICIENTS_BT2020: Self = Self(6);
    pub const WP_COLOR_REPRESENTATION_SURFACE_V1_COEFFICIENTS_BT2020_CL: Self = Self(7);
    pub const WP_COLOR_REPRESENTATION_SURFACE_V1_COEFFICIENTS_ICTCP: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wp_color_representation_surface_v1_range(pub ::core::ffi::c_uint);
impl wp_color_representation_surface_v1_range {
    pub const WP_COLOR_REPRESENTATION_SURFACE_V1_RANGE_FULL: Self = Self(1);
    pub const WP_COLOR_REPRESENTATION_SURFACE_V1_RANGE_LIMITED: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_color_representation_v1_coeffs_and_range {
    pub coeffs: wp_color_representation_surface_v1_coefficients,
    pub range: wp_color_representation_surface_v1_range,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_color_representation_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_25,
    pub WLR_PRIVATE: C2Rust_Unnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub supported_alpha_modes: *mut wp_color_representation_surface_v1_alpha_mode,
    pub supported_alpha_modes_len: size_t,
    pub supported_coeffs_and_ranges: *mut wlr_color_representation_v1_coeffs_and_range,
    pub supported_coeffs_and_ranges_len: size_t,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
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
pub struct wlr_content_type_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_27,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_cursor_shape_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_29,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
    pub request_set_shape: wl_signal,
    pub destroy: wl_signal,
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
    pub events: C2Rust_Unnamed_31,
    pub WLR_PRIVATE: C2Rust_Unnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
    pub surface_destroy: wl_listener,
    pub tool_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
    pub tablet_destroy: wl_listener,
}
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
pub struct wlr_tablet {
    pub base: wlr_input_device,
    pub r#impl: *const wlr_tablet_impl,
    pub usb_vendor_id: uint16_t,
    pub usb_product_id: uint16_t,
    pub width_mm: ::core::ffi::c_double,
    pub height_mm: ::core::ffi::c_double,
    pub events: C2Rust_Unnamed_34,
    pub paths: wl_array,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
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
    pub events: C2Rust_Unnamed_35,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
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
    pub events: C2Rust_Unnamed_37,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
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
pub struct C2Rust_Unnamed_37 {
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
    pub events: C2Rust_Unnamed_39,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
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
    pub events: C2Rust_Unnamed_41,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_40 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_41 {
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
    pub events: C2Rust_Unnamed_43,
    pub WLR_PRIVATE: C2Rust_Unnamed_42,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
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
    pub repeat_info: C2Rust_Unnamed_45,
    pub events: C2Rust_Unnamed_44,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_44 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_45 {
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
    pub events: C2Rust_Unnamed_47,
    pub WLR_PRIVATE: C2Rust_Unnamed_46,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
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
    pub events: C2Rust_Unnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
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
    pub events: C2Rust_Unnamed_50,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_49,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
    pub source_destroy: wl_listener,
    pub seat_client_destroy: wl_listener,
    pub focus_destroy: wl_listener,
    pub icon_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
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
    pub events: C2Rust_Unnamed_52,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_51,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_52 {
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
    pub events: C2Rust_Unnamed_53,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_53 {
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
pub struct wlr_data_control_manager_v1 {
    pub global: *mut wl_global,
    pub devices: wl_list,
    pub events: C2Rust_Unnamed_55,
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
    pub destroy: wl_signal,
    pub new_device: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_data_control_manager_v1 {
    pub global: *mut wl_global,
    pub devices: wl_list,
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
    pub destroy: wl_signal,
    pub new_device: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_data_device_manager {
    pub global: *mut wl_global,
    pub data_sources: wl_list,
    pub events: C2Rust_Unnamed_59,
    pub data: *mut ::core::ffi::c_void,
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
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_export_dmabuf_manager_v1 {
    pub global: *mut wl_global,
    pub frames: wl_list,
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
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_foreign_toplevel_list_v1 {
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub toplevels: wl_list,
    pub events: C2Rust_Unnamed_63,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_62,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
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
    pub events: C2Rust_Unnamed_64,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_fixes {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_66,
    pub WLR_PRIVATE: C2Rust_Unnamed_65,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_65 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_foreign_toplevel_manager_v1 {
    pub event_loop: *mut wl_event_loop,
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub toplevels: wl_list,
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
    pub events: C2Rust_Unnamed_69,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
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
pub struct wlr_scene_node {
    pub r#type: wlr_scene_node_type,
    pub parent: *mut wlr_scene_tree,
    pub link: wl_list,
    pub enabled: bool,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub events: C2Rust_Unnamed_71,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_70,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_71 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_72,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
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
pub struct wlr_ext_image_capture_source_v1 {
    pub r#impl: *const wlr_ext_image_capture_source_v1_interface,
    pub resources: wl_list,
    pub width: uint32_t,
    pub height: uint32_t,
    pub shm_formats: *mut uint32_t,
    pub shm_formats_len: size_t,
    pub dmabuf_device: dev_t,
    pub dmabuf_formats: wlr_drm_format_set,
    pub events: C2Rust_Unnamed_73,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_73 {
    pub constraints_update: wl_signal,
    pub frame: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_output_image_capture_source_manager_v1 {
    pub global: *mut wl_global,
    pub WLR_PRIVATE: C2Rust_Unnamed_74,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_74 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_foreign_toplevel_image_capture_source_manager_v1 {
    pub global: *mut wl_global,
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
    pub destroy: wl_signal,
    pub capture_request: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_foreign_toplevel_image_capture_source_manager_v1_request_event {
    pub toplevel_handle: *mut wlr_ext_foreign_toplevel_handle_v1,
    pub client: *mut wl_client,
    pub WLR_PRIVATE: C2Rust_Unnamed_77,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_77 {
    pub new_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_image_copy_capture_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_79,
    pub WLR_PRIVATE: C2Rust_Unnamed_78,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_78 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_79 {
    pub new_session: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_fractional_scale_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_81,
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_gamma_control_manager_v1 {
    pub global: *mut wl_global,
    pub controls: wl_list,
    pub fallback_gamma_size: size_t,
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
    pub set_gamma: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_gamma_control_v1 {
    pub resource: *mut wl_resource,
    pub output: *mut wlr_output,
    pub manager: *mut wlr_gamma_control_manager_v1,
    pub link: wl_list,
    pub table: *mut uint16_t,
    pub ramp_size: size_t,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_84,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_84 {
    pub output_destroy_listener: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_idle_notifier_v1 {
    pub global: *mut wl_global,
    pub WLR_PRIVATE: C2Rust_Unnamed_85,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_85 {
    pub inhibited: bool,
    pub notifications: wl_list,
    pub display_destroy: wl_listener,
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
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_88,
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
pub struct C2Rust_Unnamed_88 {
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
    pub events: C2Rust_Unnamed_90,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_89,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_89 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_90 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_92,
    pub WLR_PRIVATE: C2Rust_Unnamed_91,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_91 {
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
pub struct C2Rust_Unnamed_92 {
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
pub struct wlr_linux_drm_syncobj_manager_v1 {
    pub global: *mut wl_global,
    pub WLR_PRIVATE: C2Rust_Unnamed_93,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_93 {
    pub drm_fd: ::core::ffi::c_int,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_manager_v1 {
    pub display: *mut wl_display,
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub heads: wl_list,
    pub serial: uint32_t,
    pub current_configuration_dirty: bool,
    pub events: C2Rust_Unnamed_95,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_94,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_94 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_95 {
    pub apply: wl_signal,
    pub test: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_power_manager_v1 {
    pub global: *mut wl_global,
    pub output_powers: wl_list,
    pub events: C2Rust_Unnamed_97,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_96,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_96 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_97 {
    pub set_mode: wl_signal,
    pub destroy: wl_signal,
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
    pub cursor_hint: C2Rust_Unnamed_98,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_98 {
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
    pub events: C2Rust_Unnamed_100,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_99,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_99 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub synced: wlr_surface_synced,
    pub destroying: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_100 {
    pub set_region: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_constraints_v1 {
    pub global: *mut wl_global,
    pub constraints: wl_list,
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
    pub new_constraint: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_presentation {
    pub global: *mut wl_global,
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
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_primary_selection_v1_device_manager {
    pub global: *mut wl_global,
    pub devices: wl_list,
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
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_relative_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub relative_pointers: wl_list,
    pub events: C2Rust_Unnamed_108,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_107,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_107 {
    pub display_destroy_listener: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_108 {
    pub destroy: wl_signal,
    pub new_relative_pointer: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_screencopy_manager_v1 {
    pub global: *mut wl_global,
    pub frames: wl_list,
    pub events: C2Rust_Unnamed_110,
    pub data: *mut ::core::ffi::c_void,
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
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_security_context_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_112,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_111,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_111 {
    pub contexts: wl_list,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_112 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_server_decoration_manager_mode(pub ::core::ffi::c_uint);
impl wlr_server_decoration_manager_mode {
    pub const WLR_SERVER_DECORATION_MANAGER_MODE_NONE: Self = Self(0);
    pub const WLR_SERVER_DECORATION_MANAGER_MODE_CLIENT: Self = Self(1);
    pub const WLR_SERVER_DECORATION_MANAGER_MODE_SERVER: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_server_decoration_manager {
    pub global: *mut wl_global,
    pub resources: wl_list,
    pub decorations: wl_list,
    pub default_mode: uint32_t,
    pub events: C2Rust_Unnamed_114,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_113,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_113 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_114 {
    pub new_decoration: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_session_lock_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_116,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_115,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_115 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_116 {
    pub new_lock: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_session_lock_v1 {
    pub resource: *mut wl_resource,
    pub surfaces: wl_list,
    pub events: C2Rust_Unnamed_118,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_117,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_117 {
    pub locked_sent: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_118 {
    pub new_surface: wl_signal,
    pub unlock: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_single_pixel_buffer_manager_v1 {
    pub global: *mut wl_global,
    pub WLR_PRIVATE: C2Rust_Unnamed_119,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_119 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_subcompositor {
    pub global: *mut wl_global,
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
    pub destroy: wl_signal,
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
    pub events: C2Rust_Unnamed_123,
    pub WLR_PRIVATE: C2Rust_Unnamed_122,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_122 {
    pub pad_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_123 {
    pub button_feedback: wl_signal,
    pub strip_feedback: wl_signal,
    pub ring_feedback: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_tablet_manager_v2 {
    pub wl_global: *mut wl_global,
    pub clients: wl_list,
    pub seats: wl_list,
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
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_viewporter {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_127,
    pub WLR_PRIVATE: C2Rust_Unnamed_126,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_126 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_127 {
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
pub struct wlr_xdg_activation_v1 {
    pub global: *mut wl_global,
    pub token_timeout_msec: uint32_t,
    pub tokens: wl_list,
    pub events: C2Rust_Unnamed_129,
    pub WLR_PRIVATE: C2Rust_Unnamed_128,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_128 {
    pub display: *mut wl_display,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_129 {
    pub destroy: wl_signal,
    pub request_activate: wl_signal,
    pub new_token: wl_signal,
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
pub struct wlr_xdg_shell {
    pub global: *mut wl_global,
    pub version: uint32_t,
    pub clients: wl_list,
    pub popup_grabs: wl_list,
    pub ping_timeout: uint32_t,
    pub events: C2Rust_Unnamed_131,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_130,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_130 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_131 {
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
    pub size: C2Rust_Unnamed_133,
    pub parent_size: C2Rust_Unnamed_133,
    pub offset: C2Rust_Unnamed_132,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_132 {
    pub x: int32_t,
    pub y: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_133 {
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
    pub events: C2Rust_Unnamed_135,
    pub grab_link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_134,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_134 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_135 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_138,
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
    pub events: C2Rust_Unnamed_137,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_136,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_136 {
    pub synced: wlr_surface_synced,
    pub role_resource_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_137 {
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
pub union C2Rust_Unnamed_138 {
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
    pub events: C2Rust_Unnamed_140,
    pub WLR_PRIVATE: C2Rust_Unnamed_139,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_139 {
    pub synced: wlr_surface_synced,
    pub parent_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_140 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_141,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_141 {
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
    pub bounds: C2Rust_Unnamed_142,
    pub wm_capabilities: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_142 {
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
pub struct wlr_xdg_decoration_manager_v1 {
    pub global: *mut wl_global,
    pub decorations: wl_list,
    pub events: C2Rust_Unnamed_144,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_143,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_143 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_144 {
    pub new_toplevel_decoration: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_foreign_registry {
    pub exported_surfaces: wl_list,
    pub events: C2Rust_Unnamed_146,
    pub WLR_PRIVATE: C2Rust_Unnamed_145,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_145 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_146 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_foreign_v1 {
    pub exporter: C2Rust_Unnamed_149,
    pub importer: C2Rust_Unnamed_149,
    pub registry: *mut wlr_xdg_foreign_registry,
    pub events: C2Rust_Unnamed_148,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_147,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_147 {
    pub foreign_registry_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_148 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_149 {
    pub global: *mut wl_global,
    pub objects: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_foreign_v2 {
    pub exporter: C2Rust_Unnamed_152,
    pub importer: C2Rust_Unnamed_152,
    pub registry: *mut wlr_xdg_foreign_registry,
    pub events: C2Rust_Unnamed_151,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_150,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_150 {
    pub foreign_registry_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_151 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_152 {
    pub global: *mut wl_global,
    pub objects: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_layout {
    pub outputs: wl_list,
    pub display: *mut wl_display,
    pub events: C2Rust_Unnamed_154,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_153,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_153 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_154 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_output_manager_v1 {
    pub global: *mut wl_global,
    pub layout: *mut wlr_output_layout,
    pub outputs: wl_list,
    pub events: C2Rust_Unnamed_156,
    pub WLR_PRIVATE: C2Rust_Unnamed_155,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_155 {
    pub display_destroy: wl_listener,
    pub layout_add: wl_listener,
    pub layout_change: wl_listener,
    pub layout_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_156 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xdg_toplevel_tag_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_158,
    pub WLR_PRIVATE: C2Rust_Unnamed_157,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_157 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_158 {
    pub set_tag: wl_signal,
    pub set_description: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmVersion {
    pub version_major: ::core::ffi::c_int,
    pub version_minor: ::core::ffi::c_int,
    pub version_patchlevel: ::core::ffi::c_int,
    pub name_len: ::core::ffi::c_int,
    pub name: *mut ::core::ffi::c_char,
    pub date_len: ::core::ffi::c_int,
    pub date: *mut ::core::ffi::c_char,
    pub desc_len: ::core::ffi::c_int,
    pub desc: *mut ::core::ffi::c_char,
}
pub type drmVersion = _drmVersion;
pub type drmVersionPtr = *mut _drmVersion;
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
    pub WLR_PRIVATE: C2Rust_Unnamed_159,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_159 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_buffer {
    pub node: wlr_scene_node,
    pub buffer: *mut wlr_buffer,
    pub events: C2Rust_Unnamed_164,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_160,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_160 {
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
    pub events: C2Rust_Unnamed_162,
    pub WLR_PRIVATE: C2Rust_Unnamed_161,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_161 {
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
pub struct C2Rust_Unnamed_162 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_163,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_163 {
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
pub struct C2Rust_Unnamed_164 {
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
    pub layers: C2Rust_Unnamed_211,
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
    pub events: C2Rust_Unnamed_165,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_165 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_206,
    pub border: C2Rust_Unnamed_205,
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
    pub events: C2Rust_Unnamed_166,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_166 {
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
    pub layers: C2Rust_Unnamed_204,
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
    pub events: C2Rust_Unnamed_168,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_167,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_167 {
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_168 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_group_handle_v1 {
    pub manager: *mut wlr_ext_workspace_manager_v1,
    pub caps: uint32_t,
    pub events: C2Rust_Unnamed_170,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_169,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_169 {
    pub outputs: wl_list,
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_170 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_manager_v1 {
    pub global: *mut wl_global,
    pub groups: wl_list,
    pub workspaces: wl_list,
    pub events: C2Rust_Unnamed_172,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_171,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_171 {
    pub resources: wl_list,
    pub idle_source: *mut wl_event_source,
    pub event_loop: *mut wl_event_loop,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_172 {
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
    pub layers: C2Rust_Unnamed_201,
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
    pub session_lock: C2Rust_Unnamed_179,
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
    pub events: C2Rust_Unnamed_174,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_173,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_173 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_174 {
    pub new_object: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_text_input_manager_v3 {
    pub global: *mut wl_global,
    pub text_inputs: wl_list,
    pub events: C2Rust_Unnamed_176,
    pub WLR_PRIVATE: C2Rust_Unnamed_175,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_175 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_176 {
    pub new_text_input: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_manager_v2 {
    pub global: *mut wl_global,
    pub input_methods: wl_list,
    pub events: C2Rust_Unnamed_178,
    pub WLR_PRIVATE: C2Rust_Unnamed_177,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_177 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_178 {
    pub new_input_method: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_179 {
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
pub struct wlr_drm_lease_v1_manager {
    pub devices: wl_list,
    pub display: *mut wl_display,
    pub events: C2Rust_Unnamed_181,
    pub WLR_PRIVATE: C2Rust_Unnamed_180,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_180 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_181 {
    pub destroy: wl_signal,
    pub request: wl_signal,
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
    pub events: C2Rust_Unnamed_184,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_182,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_182 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_183,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_183 {
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
pub struct C2Rust_Unnamed_184 {
    pub destroy: wl_signal,
    pub ready: wl_signal,
    pub new_surface: wl_signal,
    pub remove_startup_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_xwayland_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_186,
    pub WLR_PRIVATE: C2Rust_Unnamed_185,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_185 {
    pub client: *mut wl_client,
    pub surfaces: wl_list,
    pub display_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_186 {
    pub destroy: wl_signal,
    pub new_surface: wl_signal,
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
    pub events: C2Rust_Unnamed_188,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_187,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_187 {
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_188 {
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
    pub events: C2Rust_Unnamed_190,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_189,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_189 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_190 {
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
    pub events: C2Rust_Unnamed_192,
    pub WLR_PRIVATE: C2Rust_Unnamed_191,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_191 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_192 {
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
    pub events: C2Rust_Unnamed_194,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_193,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_193 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_194 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_pointers: wl_list,
    pub events: C2Rust_Unnamed_196,
    pub WLR_PRIVATE: C2Rust_Unnamed_195,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_195 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_196 {
    pub new_virtual_pointer: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_keyboards: wl_list,
    pub events: C2Rust_Unnamed_198,
    pub WLR_PRIVATE: C2Rust_Unnamed_197,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_197 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_198 {
    pub new_virtual_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_200,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_199,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_199 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_200 {
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_201 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_203,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_202,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_202 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_203 {
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
pub struct C2Rust_Unnamed_204 {
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
pub struct C2Rust_Unnamed_205 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_206 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_208,
    pub events: C2Rust_Unnamed_207,
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
pub struct C2Rust_Unnamed_207 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_208 {
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
    pub events: C2Rust_Unnamed_210,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_209,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_209 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_210 {
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
pub struct C2Rust_Unnamed_211 {
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
    pub events: C2Rust_Unnamed_213,
    pub WLR_PRIVATE: C2Rust_Unnamed_212,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_212 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_213 {
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
    pub events: C2Rust_Unnamed_215,
    pub WLR_PRIVATE: C2Rust_Unnamed_214,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_214 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_215 {
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
    pub events: C2Rust_Unnamed_216,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_216 {
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
    pub previous: C2Rust_Unnamed_217,
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
pub struct C2Rust_Unnamed_217 {
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
    pub xcursor_theme: C2Rust_Unnamed_218,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_218 {
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
    pub colors: C2Rust_Unnamed_219,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_219 {
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
    pub border_colors: C2Rust_Unnamed_222,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_220,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_220 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_221,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_221 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_222 {
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
pub struct wlr_drm_lease_request_v1 {
    pub resource: *mut wl_resource,
    pub device: *mut wlr_drm_lease_device_v1,
    pub connectors: *mut *mut wlr_drm_lease_connector_v1,
    pub n_connectors: size_t,
    pub lease_resource: *mut wl_resource,
    pub invalid: bool,
    pub link: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_drm_lease_connector_v1 {
    pub resources: wl_list,
    pub output: *mut wlr_output,
    pub device: *mut wlr_drm_lease_device_v1,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_223,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_223 {
    pub destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_drm_lease_device_v1 {
    pub resources: wl_list,
    pub global: *mut wl_global,
    pub manager: *mut wlr_drm_lease_v1_manager,
    pub backend: *mut wlr_backend,
    pub connectors: wl_list,
    pub leases: wl_list,
    pub requests: wl_list,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_224,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_224 {
    pub backend_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_drm_lease_v1 {
    pub resource: *mut wl_resource,
    pub drm_lease: *mut wlr_drm_lease,
    pub device: *mut wlr_drm_lease_device_v1,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_225,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_225 {
    pub destroy: wl_listener,
}
#[inline]
unsafe extern "C" fn wl_signal_add(
    mut signal: *mut wl_signal,
    mut listener: *mut wl_listener,
) {
    wl_list_insert((*signal).listener_list.prev, &raw mut (*listener).link);
}
pub const SWAY_XDG_SHELL_VERSION: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SWAY_LAYER_SHELL_VERSION: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SWAY_FOREIGN_TOPLEVEL_LIST_VERSION: ::core::ffi::c_int = 1
    as ::core::ffi::c_int;
pub const SWAY_PRESENTATION_VERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SWAY_XDG_DECORATION_VERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[no_mangle]
pub static mut unsupported_gpu_detected: bool = r#false != 0;
unsafe extern "C" fn handle_drm_lease_request(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut req: *mut wlr_drm_lease_request_v1 = data as *mut wlr_drm_lease_request_v1;
    let mut lease: *mut wlr_drm_lease_v1 = wlr_drm_lease_request_v1_grant(req);
    if lease.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to grant lease request\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            93 as ::core::ffi::c_int,
        );
        wlr_drm_lease_request_v1_reject(req);
    }
}
unsafe extern "C" fn is_privileged(mut global: *const wl_global) -> bool {
    if !server.drm_lease_manager.is_null() {
        let mut drm_lease_dev: *mut wlr_drm_lease_device_v1 = ::core::ptr::null_mut::<
            wlr_drm_lease_device_v1,
        >();
        drm_lease_dev = ((*server.drm_lease_manager).devices.next
            as *mut ::core::ffi::c_char)
            .offset(-(88 as ::core::ffi::c_ulong as isize))
            as *mut wlr_drm_lease_device_v1 as *mut wlr_drm_lease_device_v1;
        while &raw mut (*drm_lease_dev).link
            != &raw mut (*server.drm_lease_manager).devices
        {
            if (*drm_lease_dev).global == global as *mut wl_global {
                return r#true != 0;
            }
            drm_lease_dev = ((*drm_lease_dev).link.next as *mut ::core::ffi::c_char)
                .offset(-(88 as ::core::ffi::c_ulong as isize))
                as *mut wlr_drm_lease_device_v1 as *mut wlr_drm_lease_device_v1;
        }
    }
    return global == (*server.output_manager_v1).global
        || global == (*server.output_power_manager_v1).global
        || global == (*server.input_method).global
        || global == (*server.foreign_toplevel_list).global
        || global == (*server.foreign_toplevel_manager).global
        || global == (*server.wlr_data_control_manager_v1).global
        || global == (*server.ext_data_control_manager_v1).global
        || global == (*server.screencopy_manager_v1).global
        || global == (*server.ext_image_copy_capture_manager_v1).global
        || global == (*server.export_dmabuf_manager_v1).global
        || global == (*server.security_context_manager_v1).global
        || global == (*server.gamma_control_manager_v1).global
        || global == (*server.layer_shell).global
        || global == (*server.session_lock.manager).global
        || global == (*(*server.input).keyboard_shortcuts_inhibit).global
        || global == (*(*server.input).virtual_keyboard).global
        || global == (*(*server.input).virtual_pointer).global
        || global == (*(*server.input).transient_seat_manager).global
        || global == (*server.xdg_output_manager_v1).global
        || global == (*server.workspace_manager_v1).global;
}
unsafe extern "C" fn filter_global(
    mut client: *const wl_client,
    mut global: *const wl_global,
    mut data: *mut ::core::ffi::c_void,
) -> bool {
    let mut xwayland: *mut wlr_xwayland = server.xwayland.wlr_xwayland;
    if !xwayland.is_null() && global == (*(*xwayland).shell_v1).global {
        return !(*xwayland).server.is_null() && client == (*(*xwayland).server).client;
    }
    let mut security_context: *const wlr_security_context_v1_state = wlr_security_context_manager_v1_lookup_client(
        server.security_context_manager_v1,
        client as *mut wl_client,
    );
    if is_privileged(global) {
        return security_context.is_null();
    }
    return r#true != 0;
}
unsafe extern "C" fn detect_proprietary(
    mut backend: *mut wlr_backend,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut drm_fd: ::core::ffi::c_int = wlr_backend_get_drm_fd(backend);
    if drm_fd < 0 as ::core::ffi::c_int {
        return;
    }
    let mut version: *mut drmVersion = drmGetVersion(drm_fd);
    if version.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] drmGetVersion() failed\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            163 as ::core::ffi::c_int,
        );
        return;
    }
    if strcmp((*version).name, b"nvidia-drm\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        unsupported_gpu_detected = r#true != 0;
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] !!! Proprietary Nvidia drivers are in use !!!\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            169 as ::core::ffi::c_int,
        );
    }
    if strcmp((*version).name, b"evdi\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        unsupported_gpu_detected = r#true != 0;
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] !!! Proprietary DisplayLink drivers are in use !!!\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            174 as ::core::ffi::c_int,
        );
    }
    drmFreeVersion(version);
}
unsafe extern "C" fn do_renderer_recreate(mut data: *mut ::core::ffi::c_void) {
    let mut server_0: *mut sway_server = data as *mut sway_server;
    (*server_0).recreating_renderer = ::core::ptr::null_mut::<wl_event_source>();
    _sway_log(
        sway_log_importance_t::SWAY_INFO,
        b"[%s:%d] Re-creating renderer after GPU reset\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
        184 as ::core::ffi::c_int,
    );
    let mut renderer: *mut wlr_renderer = wlr_renderer_autocreate((*server_0).backend);
    if renderer.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to create renderer\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            187 as ::core::ffi::c_int,
        );
        return;
    }
    let mut allocator: *mut wlr_allocator = wlr_allocator_autocreate(
        (*server_0).backend,
        renderer,
    );
    if allocator.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to create allocator\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            194 as ::core::ffi::c_int,
        );
        wlr_renderer_destroy(renderer);
        return;
    }
    let mut old_renderer: *mut wlr_renderer = (*server_0).renderer;
    let mut old_allocator: *mut wlr_allocator = (*server_0).allocator;
    (*server_0).renderer = renderer;
    (*server_0).allocator = allocator;
    wl_list_remove(&raw mut (*server_0).renderer_lost.link);
    wl_signal_add(
        &raw mut (*(*server_0).renderer).events.lost,
        &raw mut (*server_0).renderer_lost,
    );
    wlr_compositor_set_renderer((*server_0).compositor, renderer);
    let mut output: *mut sway_output = ::core::ptr::null_mut::<sway_output>();
    output = ((*root).all_outputs.next as *mut ::core::ffi::c_char)
        .offset(-(152 as ::core::ffi::c_ulong as isize)) as *mut sway_output
        as *mut sway_output;
    while &raw mut (*output).link != &raw mut (*root).all_outputs {
        wlr_output_init_render(
            (*output).wlr_output,
            (*server_0).allocator,
            (*server_0).renderer,
        );
        output = ((*output).link.next as *mut ::core::ffi::c_char)
            .offset(-(152 as ::core::ffi::c_ulong as isize)) as *mut sway_output
            as *mut sway_output;
    }
    wlr_allocator_destroy(old_allocator);
    wlr_renderer_destroy(old_renderer);
}
unsafe extern "C" fn handle_renderer_lost(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut server_0: *mut sway_server = (listener as *mut ::core::ffi::c_char)
        .offset(-(120 as ::core::ffi::c_ulong as isize)) as *mut sway_server;
    if !(*server_0).recreating_renderer.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Re-creation of renderer already scheduled\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            223 as ::core::ffi::c_int,
        );
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_INFO,
        b"[%s:%d] Scheduling re-creation of renderer after GPU reset\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
        227 as ::core::ffi::c_int,
    );
    (*server_0).recreating_renderer = wl_event_loop_add_idle(
        (*server_0).wl_event_loop,
        Some(
            do_renderer_recreate as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        server_0 as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn handle_new_foreign_toplevel_capture_request(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut request: *mut wlr_ext_foreign_toplevel_image_capture_source_manager_v1_request_event = data
        as *mut wlr_ext_foreign_toplevel_image_capture_source_manager_v1_request_event;
    let mut view: *mut sway_view = (*(*request).toplevel_handle).data as *mut sway_view;
    if (*view).image_capture_source.is_null() {
        (*view).image_capture_source = wlr_ext_image_capture_source_v1_create_with_scene_node(
            &raw mut (*(*view).image_capture_scene).tree.node,
            server.wl_event_loop,
            server.allocator,
            server.renderer,
        );
        if (*view).image_capture_source.is_null() {
            return;
        }
    }
    wlr_ext_foreign_toplevel_image_capture_source_manager_v1_request_accept(
        request,
        (*view).image_capture_source,
    );
}
#[no_mangle]
pub unsafe extern "C" fn server_init(mut server_0: *mut sway_server) -> bool {
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Initializing Wayland server\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
        247 as ::core::ffi::c_int,
    );
    (*server_0).wl_display = wl_display_create() as *mut wl_display;
    if (*server_0).wl_display.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create wl_display\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            250 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).wl_event_loop = wl_display_get_event_loop((*server_0).wl_display);
    wl_display_set_global_filter(
        (*server_0).wl_display,
        Some(
            filter_global
                as unsafe extern "C" fn(
                    *const wl_client,
                    *const wl_global,
                    *mut ::core::ffi::c_void,
                ) -> bool,
        ),
        NULL,
    );
    wl_display_set_default_max_buffer_size(
        (*server_0).wl_display,
        (1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t,
    );
    if wlr_fixes_create((*server_0).wl_display, 1 as uint32_t).is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create wp_fixes global\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            259 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    root = root_create((*server_0).wl_display);
    if root.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create root\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            264 as ::core::ffi::c_int,
        );
        wl_display_destroy((*server_0).wl_display);
        return r#false != 0;
    }
    (*server_0).backend = wlr_backend_autocreate(
        (*server_0).wl_event_loop,
        &raw mut (*server_0).session,
    );
    if (*server_0).backend.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to create backend\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            271 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    wlr_multi_for_each_backend(
        (*server_0).backend,
        Some(
            detect_proprietary
                as unsafe extern "C" fn(*mut wlr_backend, *mut ::core::ffi::c_void) -> (),
        ),
        NULL,
    );
    (*server_0).renderer = wlr_renderer_autocreate((*server_0).backend);
    if (*server_0).renderer.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create renderer\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            279 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).renderer_lost.notify = Some(
        handle_renderer_lost
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*server_0).renderer).events.lost,
        &raw mut (*server_0).renderer_lost,
    );
    wlr_renderer_init_wl_shm((*server_0).renderer, (*server_0).wl_display);
    if wlr_renderer_get_drm_fd((*server_0).renderer) >= 0 as ::core::ffi::c_int
        && !wlr_renderer_get_texture_formats(
                (*server_0).renderer,
                wlr_buffer_cap::WLR_BUFFER_CAP_DMABUF.0 as ::core::ffi::c_int as uint32_t,
            )
            .is_null()
    {
        (*server_0).linux_dmabuf_v1 = wlr_linux_dmabuf_v1_create_with_renderer(
            (*server_0).wl_display,
            5 as uint32_t,
            (*server_0).renderer,
        );
        if (*server_0).linux_dmabuf_v1.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Failed to create linux-dmabuf v1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
                293 as ::core::ffi::c_int,
            );
            return r#false != 0;
        }
    }
    if wlr_renderer_get_drm_fd((*server_0).renderer) >= 0 as ::core::ffi::c_int
        && (*(*server_0).renderer).features.timeline as ::core::ffi::c_int != 0
        && (*(*server_0).backend).features.timeline as ::core::ffi::c_int != 0
    {
        wlr_linux_drm_syncobj_manager_v1_create(
            (*server_0).wl_display,
            1 as uint32_t,
            wlr_renderer_get_drm_fd((*server_0).renderer),
        );
    }
    (*server_0).allocator = wlr_allocator_autocreate(
        (*server_0).backend,
        (*server_0).renderer,
    );
    if (*server_0).allocator.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create allocator\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            307 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).compositor = wlr_compositor_create(
        (*server_0).wl_display,
        6 as uint32_t,
        (*server_0).renderer,
    );
    if (*server_0).compositor.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create compositor\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            314 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    if wlr_subcompositor_create((*server_0).wl_display).is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create subcompositor\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            319 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).data_device_manager = wlr_data_device_manager_create(
        (*server_0).wl_display,
    );
    if (*server_0).data_device_manager.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create data device manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            326 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).gamma_control_manager_v1 = wlr_gamma_control_manager_v1_create(
        (*server_0).wl_display,
    );
    if (*server_0).gamma_control_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create gamma control manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            333 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*(*server_0).gamma_control_manager_v1).fallback_gamma_size = 512 as size_t;
    wlr_scene_set_gamma_control_manager_v1(
        (*root).root_scene,
        (*server_0).gamma_control_manager_v1,
    );
    (*server_0).new_output.notify = Some(
        handle_new_output
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*server_0).backend).events.new_output,
        &raw mut (*server_0).new_output,
    );
    (*server_0).xdg_output_manager_v1 = wlr_xdg_output_manager_v1_create(
        (*server_0).wl_display,
        (*root).output_layout,
    );
    if (*server_0).xdg_output_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create XDG output manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            347 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).idle_notifier_v1 = wlr_idle_notifier_v1_create((*server_0).wl_display);
    if (*server_0).idle_notifier_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create idle notifier\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            353 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    if !sway_idle_inhibit_manager_v1_init() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to init idle inhibit manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            357 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).layer_shell = wlr_layer_shell_v1_create(
        (*server_0).wl_display,
        SWAY_LAYER_SHELL_VERSION as uint32_t,
    );
    if (*server_0).layer_shell.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create layer shell\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            364 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    wl_signal_add(
        &raw mut (*(*server_0).layer_shell).events.new_surface,
        &raw mut (*server_0).layer_shell_surface,
    );
    (*server_0).layer_shell_surface.notify = Some(
        handle_layer_shell_surface
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    (*server_0).xdg_shell = wlr_xdg_shell_create(
        (*server_0).wl_display,
        SWAY_XDG_SHELL_VERSION as uint32_t,
    );
    if (*server_0).xdg_shell.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create XDG shell\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            374 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    wl_signal_add(
        &raw mut (*(*server_0).xdg_shell).events.new_toplevel,
        &raw mut (*server_0).xdg_shell_toplevel,
    );
    (*server_0).xdg_shell_toplevel.notify = Some(
        handle_xdg_shell_toplevel
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    (*server_0).tablet_v2 = wlr_tablet_v2_create((*server_0).wl_display);
    if (*server_0).tablet_v2.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create tablet manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            383 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).server_decoration_manager = wlr_server_decoration_manager_create(
        (*server_0).wl_display,
    );
    if (*server_0).server_decoration_manager.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create server decoration manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            390 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    wlr_server_decoration_manager_set_default_mode(
        (*server_0).server_decoration_manager,
        wlr_server_decoration_manager_mode::WLR_SERVER_DECORATION_MANAGER_MODE_SERVER.0
            as ::core::ffi::c_int as uint32_t,
    );
    wl_signal_add(
        &raw mut (*(*server_0).server_decoration_manager).events.new_decoration,
        &raw mut (*server_0).server_decoration,
    );
    (*server_0).server_decoration.notify = Some(
        handle_server_decoration
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_list_init(&raw mut (*server_0).decorations);
    (*server_0).xdg_decoration_manager = wlr_xdg_decoration_manager_v1_create(
        (*server_0).wl_display,
        SWAY_XDG_DECORATION_VERSION as uint32_t,
    );
    if (*server_0).xdg_decoration_manager.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create XDG decoration manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            405 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    wl_signal_add(
        &raw mut (*(*server_0).xdg_decoration_manager).events.new_toplevel_decoration,
        &raw mut (*server_0).xdg_decoration,
    );
    (*server_0).xdg_decoration.notify = Some(
        handle_xdg_decoration
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_list_init(&raw mut (*server_0).xdg_decorations);
    (*server_0).relative_pointer_manager = wlr_relative_pointer_manager_v1_create(
        (*server_0).wl_display,
    );
    if (*server_0).relative_pointer_manager.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create relative pointer manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            417 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).pointer_constraints = wlr_pointer_constraints_v1_create(
        (*server_0).wl_display,
    );
    if (*server_0).pointer_constraints.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create pointer constraints\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            424 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).pointer_constraint.notify = Some(
        handle_pointer_constraint
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*server_0).pointer_constraints).events.new_constraint,
        &raw mut (*server_0).pointer_constraint,
    );
    if wlr_presentation_create(
            (*server_0).wl_display,
            (*server_0).backend,
            SWAY_PRESENTATION_VERSION as uint32_t,
        )
        .is_null()
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create presentation\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            432 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    if wlr_alpha_modifier_v1_create((*server_0).wl_display).is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create alpha modifier\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            436 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).output_manager_v1 = wlr_output_manager_v1_create((*server_0).wl_display);
    if (*server_0).output_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create output manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            443 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).output_manager_apply.notify = Some(
        handle_output_manager_apply
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*server_0).output_manager_v1).events.apply,
        &raw mut (*server_0).output_manager_apply,
    );
    (*server_0).output_manager_test.notify = Some(
        handle_output_manager_test
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*server_0).output_manager_v1).events.test,
        &raw mut (*server_0).output_manager_test,
    );
    (*server_0).output_power_manager_v1 = wlr_output_power_manager_v1_create(
        (*server_0).wl_display,
    );
    if (*server_0).output_power_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create output power manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            456 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).output_power_manager_set_mode.notify = Some(
        handle_output_power_manager_set_mode
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*server_0).output_power_manager_v1).events.set_mode,
        &raw mut (*server_0).output_power_manager_set_mode,
    );
    (*server_0).input_method = wlr_input_method_manager_v2_create(
        (*server_0).wl_display,
    );
    if (*server_0).input_method.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create input method manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            465 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).text_input = wlr_text_input_manager_v3_create((*server_0).wl_display);
    if (*server_0).text_input.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create text input manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            470 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).foreign_toplevel_list = wlr_ext_foreign_toplevel_list_v1_create(
        (*server_0).wl_display,
        SWAY_FOREIGN_TOPLEVEL_LIST_VERSION as uint32_t,
    );
    if (*server_0).foreign_toplevel_list.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create foreign toplevel list\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            476 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).foreign_toplevel_manager = wlr_foreign_toplevel_manager_v1_create(
        (*server_0).wl_display,
    );
    if (*server_0).foreign_toplevel_manager.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create foreign toplevel manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            482 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    if !sway_session_lock_init() {
        return r#false != 0;
    }
    if !sway_ext_workspace_init() {
        return r#false != 0;
    }
    (*server_0).drm_lease_manager = wlr_drm_lease_v1_manager_create(
        (*server_0).wl_display,
        (*server_0).backend,
    ) as *mut wlr_drm_lease_v1_manager;
    if !(*server_0).drm_lease_manager.is_null() {
        (*server_0).drm_lease_request.notify = Some(
            handle_drm_lease_request
                as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
        ) as wl_notify_func_t;
        wl_signal_add(
            &raw mut (*(*server_0).drm_lease_manager).events.request,
            &raw mut (*server_0).drm_lease_request,
        );
    } else {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Failed to create wlr_drm_lease_device_v1\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            501 as ::core::ffi::c_int,
        );
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] VR will not be available\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            502 as ::core::ffi::c_int,
        );
    }
    (*server_0).export_dmabuf_manager_v1 = wlr_export_dmabuf_manager_v1_create(
        (*server_0).wl_display,
    );
    if (*server_0).export_dmabuf_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create export dmabuf manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            508 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).screencopy_manager_v1 = wlr_screencopy_manager_v1_create(
        (*server_0).wl_display,
    );
    if (*server_0).screencopy_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create screencopy manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            513 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).ext_image_copy_capture_manager_v1 = wlr_ext_image_copy_capture_manager_v1_create(
        (*server_0).wl_display,
        1 as uint32_t,
    );
    if (*server_0).ext_image_copy_capture_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create ext image copy capture manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            518 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    if wlr_ext_output_image_capture_source_manager_v1_create(
            (*server_0).wl_display,
            1 as uint32_t,
        )
        .is_null()
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create ext output image capture source manager\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            522 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).wlr_data_control_manager_v1 = wlr_data_control_manager_v1_create(
        (*server_0).wl_display,
    );
    if (*server_0).wlr_data_control_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create data control manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            527 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).ext_data_control_manager_v1 = wlr_ext_data_control_manager_v1_create(
        (*server_0).wl_display,
        1 as uint32_t,
    );
    if (*server_0).ext_data_control_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create ext data control manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            532 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).security_context_manager_v1 = wlr_security_context_manager_v1_create(
        (*server_0).wl_display,
    );
    if (*server_0).security_context_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create security context manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            537 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    if wlr_viewporter_create((*server_0).wl_display).is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create viewporter\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            541 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    if wlr_single_pixel_buffer_manager_v1_create((*server_0).wl_display).is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create single pixel buffer manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            545 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).content_type_manager_v1 = wlr_content_type_manager_v1_create(
        (*server_0).wl_display,
        1 as uint32_t,
    );
    if (*server_0).content_type_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create content type manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            551 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    if wlr_fractional_scale_manager_v1_create((*server_0).wl_display, 1 as uint32_t)
        .is_null()
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create fractional scale manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            555 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).ext_foreign_toplevel_image_capture_source_manager_v1 = wlr_ext_foreign_toplevel_image_capture_source_manager_v1_create(
        (*server_0).wl_display,
        1 as uint32_t,
    );
    if (*server_0).ext_foreign_toplevel_image_capture_source_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create ext foreign toplevel image capture source manager\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            562 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).new_foreign_toplevel_capture_request.notify = Some(
        handle_new_foreign_toplevel_capture_request
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*server_0).ext_foreign_toplevel_image_capture_source_manager_v1)
            .events
            .capture_request,
        &raw mut (*server_0).new_foreign_toplevel_capture_request,
    );
    (*server_0).tearing_control_v1 = wlr_tearing_control_manager_v1_create(
        (*server_0).wl_display,
        1 as uint32_t,
    );
    if (*server_0).tearing_control_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create tearing control manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            572 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).tearing_control_new_object.notify = Some(
        handle_new_tearing_hint
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*server_0).tearing_control_v1).events.new_object,
        &raw mut (*server_0).tearing_control_new_object,
    );
    wl_list_init(&raw mut (*server_0).tearing_controllers);
    let mut foreign_registry: *mut wlr_xdg_foreign_registry = wlr_xdg_foreign_registry_create(
        (*server_0).wl_display,
    );
    if foreign_registry.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create XDG foreign registry\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            583 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    if wlr_xdg_foreign_v1_create((*server_0).wl_display, foreign_registry).is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create XDG foreign v1\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            587 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    if wlr_xdg_foreign_v2_create((*server_0).wl_display, foreign_registry).is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create XDG foreign v2\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            591 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).xdg_activation_v1 = wlr_xdg_activation_v1_create((*server_0).wl_display);
    if (*server_0).xdg_activation_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create XDG activation\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            597 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).xdg_activation_v1_request_activate.notify = Some(
        xdg_activation_v1_handle_request_activate
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*server_0).xdg_activation_v1).events.request_activate,
        &raw mut (*server_0).xdg_activation_v1_request_activate,
    );
    (*server_0).xdg_activation_v1_new_token.notify = Some(
        xdg_activation_v1_handle_new_token
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*(*server_0).xdg_activation_v1).events.new_token,
        &raw mut (*server_0).xdg_activation_v1_new_token,
    );
    let mut xdg_toplevel_tag_manager_v1: *mut wlr_xdg_toplevel_tag_manager_v1 = wlr_xdg_toplevel_tag_manager_v1_create(
        (*server_0).wl_display,
        1 as uint32_t,
    );
    if xdg_toplevel_tag_manager_v1.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create XDG toplevel tag manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            612 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).xdg_toplevel_tag_manager_v1_set_tag.notify = Some(
        xdg_toplevel_tag_manager_v1_handle_set_tag
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*xdg_toplevel_tag_manager_v1).events.set_tag,
        &raw mut (*server_0).xdg_toplevel_tag_manager_v1_set_tag,
    );
    let mut cursor_shape_manager: *mut wlr_cursor_shape_manager_v1 = wlr_cursor_shape_manager_v1_create(
        (*server_0).wl_display,
        2 as uint32_t,
    );
    if cursor_shape_manager.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create cursor shape manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            623 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).request_set_cursor_shape.notify = Some(
        handle_request_set_cursor_shape
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*cursor_shape_manager).events.request_set_shape,
        &raw mut (*server_0).request_set_cursor_shape,
    );
    if (*(*server_0).renderer).features.input_color_transform {
        let render_intents: [wp_color_manager_v1_render_intent; 1] = [
            wp_color_manager_v1_render_intent::WP_COLOR_MANAGER_V1_RENDER_INTENT_PERCEPTUAL,
        ];
        let mut transfer_functions_len: size_t = 0 as size_t;
        let mut transfer_functions: *mut wp_color_manager_v1_transfer_function = wlr_color_manager_v1_transfer_function_list_from_renderer(
            (*server_0).renderer,
            &raw mut transfer_functions_len,
        );
        let mut primaries_len: size_t = 0 as size_t;
        let mut primaries: *mut wp_color_manager_v1_primaries = wlr_color_manager_v1_primaries_list_from_renderer(
            (*server_0).renderer,
            &raw mut primaries_len,
        );
        let mut c2rust_lvalue: wlr_color_manager_v1_options = wlr_color_manager_v1_options {
            features: wlr_color_manager_v1_features {
                icc_v2_v4: false,
                parametric: r#true != 0,
                set_primaries: false,
                set_tf_power: false,
                set_luminances: false,
                set_mastering_display_primaries: r#true != 0,
                extended_target_volume: false,
                windows_scrgb: false,
            },
            render_intents: &raw const render_intents
                as *const wp_color_manager_v1_render_intent,
            render_intents_len: ::core::mem::size_of::<
                [wp_color_manager_v1_render_intent; 1],
            >()
                .wrapping_div(
                    ::core::mem::size_of::<wp_color_manager_v1_render_intent>(),
                ),
            transfer_functions: transfer_functions,
            transfer_functions_len: transfer_functions_len,
            primaries: primaries,
            primaries_len: primaries_len,
        };
        let mut cm: *mut wlr_color_manager_v1 = wlr_color_manager_v1_create(
            (*server_0).wl_display,
            2 as uint32_t,
            &raw mut c2rust_lvalue,
        );
        free(transfer_functions as *mut ::core::ffi::c_void);
        free(primaries as *mut ::core::ffi::c_void);
        if cm.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Failed to create color manager\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
                655 as ::core::ffi::c_int,
            );
            return r#false != 0;
        }
        wlr_scene_set_color_manager_v1((*root).root_scene, cm);
    }
    if wlr_color_representation_manager_v1_create_with_renderer(
            (*server_0).wl_display,
            1 as uint32_t,
            (*server_0).renderer,
        )
        .is_null()
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create color representation manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            663 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    wl_list_init(&raw mut (*server_0).pending_launcher_ctxs);
    let mut name_candidate: [::core::ffi::c_char; 16] = [0; 16];
    let mut i: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
    while i <= 32 as ::core::ffi::c_uint {
        snprintf(
            &raw mut name_candidate as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            b"wayland-%u\0".as_ptr() as *const ::core::ffi::c_char,
            i,
        );
        if wl_display_add_socket(
            (*server_0).wl_display,
            &raw mut name_candidate as *mut ::core::ffi::c_char,
        ) >= 0 as ::core::ffi::c_int
        {
            (*server_0).socket = strdup(
                &raw mut name_candidate as *mut ::core::ffi::c_char,
            );
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if (*server_0).socket.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to open wayland socket\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            680 as ::core::ffi::c_int,
        );
        wlr_backend_destroy((*server_0).backend);
        return r#false != 0;
    }
    (*server_0).headless_backend = wlr_headless_backend_create(
        (*server_0).wl_event_loop,
    );
    if (*server_0).headless_backend.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create secondary headless backend\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            687 as ::core::ffi::c_int,
        );
        wlr_backend_destroy((*server_0).backend);
        return r#false != 0;
    } else {
        wlr_multi_backend_add((*server_0).backend, (*server_0).headless_backend);
    }
    let mut wlr_output: *mut wlr_output = wlr_headless_add_output(
        (*server_0).headless_backend,
        800 as ::core::ffi::c_uint,
        600 as ::core::ffi::c_uint,
    );
    wlr_output_set_name(
        wlr_output,
        b"FALLBACK\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*root).fallback_output = output_create(wlr_output);
    if (*server_0).txn_timeout_ms == 0 {
        (*server_0).txn_timeout_ms = 200 as size_t;
    }
    (*server_0).dirty_nodes = create_list();
    if (*server_0).dirty_nodes.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create dirty nodes list\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            706 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    (*server_0).input = input_manager_create(server_0);
    if (*server_0).input.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create input manager\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            712 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    input_manager_get_default_seat();
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn server_fini(mut server_0: *mut sway_server) {
    wl_list_remove(&raw mut (*server_0).renderer_lost.link);
    wl_list_remove(&raw mut (*server_0).new_output.link);
    wl_list_remove(&raw mut (*server_0).layer_shell_surface.link);
    wl_list_remove(&raw mut (*server_0).xdg_shell_toplevel.link);
    wl_list_remove(&raw mut (*server_0).server_decoration.link);
    wl_list_remove(&raw mut (*server_0).xdg_decoration.link);
    wl_list_remove(&raw mut (*server_0).pointer_constraint.link);
    wl_list_remove(&raw mut (*server_0).output_manager_apply.link);
    wl_list_remove(&raw mut (*server_0).output_manager_test.link);
    wl_list_remove(&raw mut (*server_0).output_power_manager_set_mode.link);
    if !(*server_0).drm_lease_manager.is_null() {
        wl_list_remove(&raw mut (*server_0).drm_lease_request.link);
    }
    wl_list_remove(&raw mut (*server_0).tearing_control_new_object.link);
    wl_list_remove(&raw mut (*server_0).xdg_activation_v1_request_activate.link);
    wl_list_remove(&raw mut (*server_0).xdg_activation_v1_new_token.link);
    wl_list_remove(&raw mut (*server_0).xdg_toplevel_tag_manager_v1_set_tag.link);
    wl_list_remove(&raw mut (*server_0).request_set_cursor_shape.link);
    wl_list_remove(&raw mut (*server_0).new_foreign_toplevel_capture_request.link);
    wl_list_remove(&raw mut (*server_0).workspace_manager_v1_commit.link);
    input_manager_finish((*server_0).input);
    if !(*server_0).xwayland.wlr_xwayland.is_null() {
        wl_list_remove(&raw mut (*server_0).xwayland_surface.link);
        wl_list_remove(&raw mut (*server_0).xwayland_ready.link);
        wlr_xwayland_destroy((*server_0).xwayland.wlr_xwayland);
    }
    wl_display_destroy_clients((*server_0).wl_display);
    wlr_backend_destroy((*server_0).backend);
    wl_display_destroy((*server_0).wl_display);
    list_free((*server_0).dirty_nodes);
    free((*server_0).socket as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn server_start(mut server_0: *mut sway_server) -> bool {
    if (*config).xwayland.0 != xwayland_mode::XWAYLAND_MODE_DISABLED.0 {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Initializing Xwayland (lazy=%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            764 as ::core::ffi::c_int,
            ((*config).xwayland.0 == xwayland_mode::XWAYLAND_MODE_LAZY.0)
                as ::core::ffi::c_int,
        );
        (*server_0).xwayland.wlr_xwayland = wlr_xwayland_create(
            (*server_0).wl_display,
            (*server_0).compositor,
            (*config).xwayland.0 == xwayland_mode::XWAYLAND_MODE_LAZY.0,
        );
        if (*server_0).xwayland.wlr_xwayland.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Failed to start Xwayland\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
                770 as ::core::ffi::c_int,
            );
            unsetenv(b"DISPLAY\0".as_ptr() as *const ::core::ffi::c_char);
        } else {
            wl_signal_add(
                &raw mut (*(*server_0).xwayland.wlr_xwayland).events.new_surface,
                &raw mut (*server_0).xwayland_surface,
            );
            (*server_0).xwayland_surface.notify = Some(
                handle_xwayland_surface
                    as unsafe extern "C" fn(
                        *mut wl_listener,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ) as wl_notify_func_t;
            wl_signal_add(
                &raw mut (*(*server_0).xwayland.wlr_xwayland).events.ready,
                &raw mut (*server_0).xwayland_ready,
            );
            (*server_0).xwayland_ready.notify = Some(
                handle_xwayland_ready
                    as unsafe extern "C" fn(
                        *mut wl_listener,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ) as wl_notify_func_t;
            setenv(
                b"DISPLAY\0".as_ptr() as *const ::core::ffi::c_char,
                (*(*server_0).xwayland.wlr_xwayland).display_name,
                r#true,
            );
        }
    }
    if (*config).primary_selection {
        wlr_primary_selection_v1_device_manager_create((*server_0).wl_display);
    }
    _sway_log(
        sway_log_importance_t::SWAY_INFO,
        b"[%s:%d] Starting backend on wayland display '%s'\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
        791 as ::core::ffi::c_int,
        (*server_0).socket,
    );
    if !wlr_backend_start((*server_0).backend) {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to start backend\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
            794 as ::core::ffi::c_int,
        );
        wlr_backend_destroy((*server_0).backend);
        return r#false != 0;
    }
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn server_run(mut server_0: *mut sway_server) {
    _sway_log(
        sway_log_importance_t::SWAY_INFO,
        b"[%s:%d] Running compositor on wayland display '%s'\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/server.c\0".as_ptr() as *const ::core::ffi::c_char,
        803 as ::core::ffi::c_int,
        (*server_0).socket,
    );
    wl_display_run((*server_0).wl_display);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
