extern "C" {
    pub type json_object;
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_buffer_impl;
    pub type wlr_renderer_impl;
    pub type wlr_texture_impl;
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
    pub type wlr_pointer_constraints_v1;
    pub type wlr_drm_lease_v1_manager;
    pub type wlr_xdg_decoration_manager_v1;
    pub type wlr_server_decoration_manager;
    pub type wlr_relative_pointer_manager_v1;
    pub type wlr_xcursor_manager;
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
    pub type wlr_pointer_gestures_v1;
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
    pub type wlr_touch_impl;
    pub type sway_cursor;
    pub type _PangoFontDescription;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fcntl(
        __fd: ::core::ffi::c_int,
        __cmd: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn json_object_get(obj: *mut json_object) -> *mut json_object;
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_is_type(
        obj: *const json_object,
        r#type: json_type,
    ) -> ::core::ffi::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const ::core::ffi::c_char,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_object_del(obj: *mut json_object, key: *const ::core::ffi::c_char);
    fn json_object_new_array() -> *mut json_object;
    fn json_object_array_length(obj: *const json_object) -> size_t;
    fn json_object_array_add(
        obj: *mut json_object,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_array_get_idx(
        obj: *const json_object,
        idx: size_t,
    ) -> *mut json_object;
    fn json_object_new_boolean(b: json_bool) -> *mut json_object;
    fn json_object_new_int(i: int32_t) -> *mut json_object;
    fn json_object_new_string(s: *const ::core::ffi::c_char) -> *mut json_object;
    fn json_object_get_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_tokener_parse(str: *const ::core::ffi::c_char) -> *mut json_object;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
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
    fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn bind(
        __fd: ::core::ffi::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn recv(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn listen(__fd: ::core::ffi::c_int, __n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn accept(
        __fd: ::core::ffi::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn shutdown(
        __fd: ::core::ffi::c_int,
        __how: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ioctl(
        __fd: ::core::ffi::c_int,
        __request: ::core::ffi::c_ulong,
        ...
    ) -> ::core::ffi::c_int;
    fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    fn getpid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn wl_list_remove(elm: *mut wl_list);
    fn wl_event_loop_add_fd(
        r#loop: *mut wl_event_loop,
        fd: ::core::ffi::c_int,
        mask: uint32_t,
        func: wl_event_loop_fd_func_t,
        data: *mut ::core::ffi::c_void,
    ) -> *mut wl_event_source;
    fn wl_event_source_remove(source: *mut wl_event_source) -> ::core::ffi::c_int;
    fn wl_display_add_destroy_listener(
        display: *mut wl_display,
        listener: *mut wl_listener,
    );
    fn xkb_keysym_get_name(
        keysym: xkb_keysym_t,
        buffer: *mut ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    fn create_list() -> *mut list_t;
    fn list_free(list: *mut list_t);
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn list_del(list: *mut list_t, index: ::core::ffi::c_int);
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
    fn output_get_active_workspace(output: *mut sway_output) -> *mut sway_workspace;
    fn execute_command(
        command: *mut ::core::ffi::c_char,
        seat: *mut sway_seat,
        con: *mut sway_container,
    ) -> *mut list_t;
    fn free_cmd_results(results: *mut cmd_results);
    fn cmd_results_to_json(res_list: *mut list_t) -> *mut ::core::ffi::c_char;
    fn transaction_commit_dirty();
    fn ipc_json_get_version() -> *mut json_object;
    fn ipc_json_get_binding_mode() -> *mut json_object;
    fn ipc_json_describe_disabled_output(o: *mut sway_output) -> *mut json_object;
    fn ipc_json_describe_non_desktop_output(
        o: *mut sway_output_non_desktop,
    ) -> *mut json_object;
    fn ipc_json_describe_node(node: *mut sway_node) -> *mut json_object;
    fn ipc_json_describe_node_recursive(node: *mut sway_node) -> *mut json_object;
    fn ipc_json_describe_input(device: *mut sway_input_device) -> *mut json_object;
    fn ipc_json_describe_seat(seat: *mut sway_seat) -> *mut json_object;
    fn ipc_json_describe_bar_config(bar: *mut bar_config) -> *mut json_object;
    fn get_modifier_names(
        names: *mut *const ::core::ffi::c_char,
        modifier_masks: uint32_t,
    ) -> ::core::ffi::c_int;
    fn force_modeset();
    fn modeset_is_pending() -> bool;
    static mut config: *mut sway_config;
    fn seat_get_focused_workspace(seat: *mut sway_seat) -> *mut sway_workspace;
    static mut server: sway_server;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_abort(filename: *const ::core::ffi::c_char, ...);
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    fn sway_wl_output_subpixel_to_string(
        subpixel: wl_output_subpixel,
    ) -> *const ::core::ffi::c_char;
    fn input_manager_get_default_seat() -> *mut sway_seat;
}
pub type __uid_t = ::core::ffi::c_uint;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type size_t = usize;
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type json_bool = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct json_type(pub ::core::ffi::c_uint);
impl json_type {
    pub const json_type_null: Self = Self(0);
    pub const json_type_boolean: Self = Self(1);
    pub const json_type_double: Self = Self(2);
    pub const json_type_int: Self = Self(3);
    pub const json_type_object: Self = Self(4);
    pub const json_type_array: Self = Self(5);
    pub const json_type_string: Self = Self(6);
}
pub type ssize_t = isize;
pub type socklen_t = __socklen_t;
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
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const SHUT_RD: Self = Self(0);
    pub const SHUT_WR: Self = Self(1);
    pub const SHUT_RDWR: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108],
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const WL_EVENT_READABLE: Self = Self(1);
    pub const WL_EVENT_WRITABLE: Self = Self(2);
    pub const WL_EVENT_HANGUP: Self = Self(4);
    pub const WL_EVENT_ERROR: Self = Self(8);
}
pub type wl_event_loop_fd_func_t = Option<
    unsafe extern "C" fn(
        ::core::ffi::c_int,
        uint32_t,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
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
    pub events: C2Rust_Unnamed_1,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub destroy: wl_signal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_tablet_tool_tip_state(pub ::core::ffi::c_uint);
impl wlr_tablet_tool_tip_state {
    pub const WLR_TABLET_TOOL_TIP_UP: Self = Self(0);
    pub const WLR_TABLET_TOOL_TIP_DOWN: Self = Self(1);
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
pub struct wlr_addon_set {
    pub WLR_PRIVATE: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub addons: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon {
    pub r#impl: *const wlr_addon_interface,
    pub WLR_PRIVATE: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub ref_count: ::core::ffi::c_int,
    pub addons: wlr_addon_set,
}
pub type xkb_keysym_t = uint32_t;
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
    pub events: C2Rust_Unnamed_7,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_6,
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
pub struct C2Rust_Unnamed_6 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
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
    pub viewport: C2Rust_Unnamed_8,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub has_src: bool,
    pub has_dst: bool,
    pub src: wlr_fbox,
    pub dst_width: ::core::ffi::c_int,
    pub dst_height: ::core::ffi::c_int,
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
    pub events: C2Rust_Unnamed_9,
    pub addons: wlr_addon_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub destroy: wl_signal,
    pub release: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_client_buffer {
    pub base: wlr_buffer,
    pub texture: *mut wlr_texture,
    pub source: *mut wlr_buffer,
    pub WLR_PRIVATE: C2Rust_Unnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
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
    pub events: C2Rust_Unnamed_13,
    pub features: C2Rust_Unnamed_12,
    pub WLR_PRIVATE: C2Rust_Unnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub r#impl: *const wlr_renderer_impl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
    pub input_color_transform: bool,
    pub output_color_transform: bool,
    pub timeline: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub destroy: wl_signal,
    pub lost: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_compositor {
    pub global: *mut wl_global,
    pub renderer: *mut wlr_renderer,
    pub events: C2Rust_Unnamed_15,
    pub WLR_PRIVATE: C2Rust_Unnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
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
    pub events: C2Rust_Unnamed_19,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
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
    pub mastering_luminance: C2Rust_Unnamed_17,
    pub max_cll: ::core::ffi::c_double,
    pub max_fall: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub min: ::core::ffi::c_double,
    pub max: ::core::ffi::c_double,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub renderer_destroy: wl_listener,
    pub color_transform: *mut wlr_color_transform,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
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
    pub features: C2Rust_Unnamed_21,
    pub events: C2Rust_Unnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
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
    pub custom_mode: C2Rust_Unnamed_22,
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
pub struct C2Rust_Unnamed_22 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_25,
    pub WLR_PRIVATE: C2Rust_Unnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
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
pub struct C2Rust_Unnamed_25 {
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
    pub layers: C2Rust_Unnamed_93,
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
    pub title_bar: C2Rust_Unnamed_88,
    pub border: C2Rust_Unnamed_87,
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
    pub layers: C2Rust_Unnamed_86,
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
    pub layers: C2Rust_Unnamed_83,
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
    pub events: C2Rust_Unnamed_58,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_56,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_56 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_57,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_57 {
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
pub struct C2Rust_Unnamed_58 {
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
    pub events: C2Rust_Unnamed_60,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_59,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_59 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_60 {
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
    pub events: C2Rust_Unnamed_62,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_61,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_61 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
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
    pub events: C2Rust_Unnamed_64,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_63,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
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
pub struct C2Rust_Unnamed_64 {
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
    pub events: C2Rust_Unnamed_66,
    pub WLR_PRIVATE: C2Rust_Unnamed_65,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_65 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
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
    pub repeat_info: C2Rust_Unnamed_68,
    pub events: C2Rust_Unnamed_67,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
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
    pub events: C2Rust_Unnamed_70,
    pub WLR_PRIVATE: C2Rust_Unnamed_69,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
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
    pub events: C2Rust_Unnamed_72,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_71,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_71 {
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
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
    pub create_seat: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_pointers: wl_list,
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
    pub new_virtual_pointer: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_keyboards: wl_list,
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
    pub new_virtual_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
    pub inhibitors: wl_list,
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_82,
    pub data: *mut ::core::ffi::c_void,
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
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_83 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_85,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_84,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_84 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_85 {
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
pub struct C2Rust_Unnamed_86 {
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
pub struct C2Rust_Unnamed_87 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_88 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_90,
    pub events: C2Rust_Unnamed_89,
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
pub struct C2Rust_Unnamed_89 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_90 {
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
    pub events: C2Rust_Unnamed_92,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_91,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_91 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_92 {
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
pub struct C2Rust_Unnamed_93 {
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
    pub events: C2Rust_Unnamed_95,
    pub WLR_PRIVATE: C2Rust_Unnamed_94,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_94 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_95 {
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
    pub events: C2Rust_Unnamed_97,
    pub WLR_PRIVATE: C2Rust_Unnamed_96,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_96 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_97 {
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
    pub events: C2Rust_Unnamed_98,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_98 {
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
    pub events: C2Rust_Unnamed_99,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_99 {
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
pub struct seat_config_hide_cursor_when_typing(pub ::core::ffi::c_uint);
impl seat_config_hide_cursor_when_typing {
    pub const HIDE_WHEN_TYPING_DEFAULT: Self = Self(0);
    pub const HIDE_WHEN_TYPING_ENABLE: Self = Self(1);
    pub const HIDE_WHEN_TYPING_DISABLE: Self = Self(2);
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
    pub xcursor_theme: C2Rust_Unnamed_100,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_100 {
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
    pub colors: C2Rust_Unnamed_101,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_101 {
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
    pub border_colors: C2Rust_Unnamed_104,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_102,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_102 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_103,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_103 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_104 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_output_non_desktop {
    pub wlr_output: *mut wlr_output,
    pub destroy: wl_listener,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ipc_command_type(pub ::core::ffi::c_int);
impl ipc_command_type {
    pub const IPC_COMMAND: Self = Self(0);
    pub const IPC_GET_WORKSPACES: Self = Self(1);
    pub const IPC_SUBSCRIBE: Self = Self(2);
    pub const IPC_GET_OUTPUTS: Self = Self(3);
    pub const IPC_GET_TREE: Self = Self(4);
    pub const IPC_GET_MARKS: Self = Self(5);
    pub const IPC_GET_BAR_CONFIG: Self = Self(6);
    pub const IPC_GET_VERSION: Self = Self(7);
    pub const IPC_GET_BINDING_MODES: Self = Self(8);
    pub const IPC_GET_CONFIG: Self = Self(9);
    pub const IPC_SEND_TICK: Self = Self(10);
    pub const IPC_SYNC: Self = Self(11);
    pub const IPC_GET_BINDING_STATE: Self = Self(12);
    pub const IPC_GET_INPUTS: Self = Self(100);
    pub const IPC_GET_SEATS: Self = Self(101);
    pub const IPC_EVENT_WORKSPACE: Self = Self(-2147483648);
    pub const IPC_EVENT_OUTPUT: Self = Self(-2147483647);
    pub const IPC_EVENT_MODE: Self = Self(-2147483646);
    pub const IPC_EVENT_WINDOW: Self = Self(-2147483645);
    pub const IPC_EVENT_BARCONFIG_UPDATE: Self = Self(-2147483644);
    pub const IPC_EVENT_BINDING: Self = Self(-2147483643);
    pub const IPC_EVENT_SHUTDOWN: Self = Self(-2147483642);
    pub const IPC_EVENT_TICK: Self = Self(-2147483641);
    pub const IPC_EVENT_BAR_STATE_UPDATE: Self = Self(-2147483628);
    pub const IPC_EVENT_INPUT: Self = Self(-2147483627);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_client {
    pub event_source: *mut wl_event_source,
    pub writable_event_source: *mut wl_event_source,
    pub server: *mut sway_server,
    pub fd: ::core::ffi::c_int,
    pub subscribed_events: ipc_command_type,
    pub write_buffer_len: size_t,
    pub write_buffer_size: size_t,
    pub write_buffer: *mut ::core::ffi::c_char,
    pub pending_length: uint32_t,
    pub pending_type: ipc_command_type,
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FIONREAD: ::core::ffi::c_int = 0x541b as ::core::ffi::c_int;
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
pub const F_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut ipc_socket: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
static mut ipc_event_source: *mut wl_event_source = ::core::ptr::null_mut::<
    wl_event_source,
>();
static mut ipc_sockaddr: *mut sockaddr_un = ::core::ptr::null_mut::<sockaddr_un>();
static mut ipc_client_list: *mut list_t = ::core::ptr::null_mut::<list_t>();
static mut ipc_display_destroy: wl_listener = wl_listener {
    link: wl_list {
        prev: ::core::ptr::null_mut::<wl_list>(),
        next: ::core::ptr::null_mut::<wl_list>(),
    },
    notify: None,
};
static mut ipc_magic: [::core::ffi::c_char; 6] = [
    'i' as ::core::ffi::c_char,
    '3' as ::core::ffi::c_char,
    '-' as ::core::ffi::c_char,
    'i' as ::core::ffi::c_char,
    'p' as ::core::ffi::c_char,
    'c' as ::core::ffi::c_char,
];
pub const IPC_HEADER_SIZE: usize = ::core::mem::size_of::<[::core::ffi::c_char; 6]>()
    .wrapping_add(8usize);
unsafe extern "C" fn handle_display_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    if !ipc_event_source.is_null() {
        wl_event_source_remove(ipc_event_source);
    }
    close(ipc_socket);
    unlink(&raw mut (*ipc_sockaddr).sun_path as *mut ::core::ffi::c_char);
    while (*ipc_client_list).length != 0 {
        ipc_client_disconnect(
            *(*ipc_client_list)
                .items
                .offset(((*ipc_client_list).length - 1 as ::core::ffi::c_int) as isize)
                as *mut ipc_client,
        );
    }
    list_free(ipc_client_list);
    free(ipc_sockaddr as *mut ::core::ffi::c_void);
    wl_list_remove(&raw mut ipc_display_destroy.link);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_init(mut server_0: *mut sway_server) {
    ipc_socket = socket(
        AF_UNIX,
        __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if ipc_socket == -1 as ::core::ffi::c_int {
        _sway_abort(
            b"[%s:%d] Unable to create IPC socket\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            87 as ::core::ffi::c_int,
        );
    }
    if fcntl(ipc_socket, F_SETFD, FD_CLOEXEC) == -1 as ::core::ffi::c_int {
        _sway_abort(
            b"[%s:%d] Unable to set CLOEXEC on IPC socket\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            90 as ::core::ffi::c_int,
        );
    }
    if fcntl(ipc_socket, F_SETFL, O_NONBLOCK) == -1 as ::core::ffi::c_int {
        _sway_abort(
            b"[%s:%d] Unable to set NONBLOCK on IPC socket\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            93 as ::core::ffi::c_int,
        );
    }
    ipc_sockaddr = ipc_user_sockaddr();
    if !getenv(b"SWAYSOCK\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        && access(getenv(b"SWAYSOCK\0".as_ptr() as *const ::core::ffi::c_char), F_OK)
            == -1 as ::core::ffi::c_int
    {
        strncpy(
            &raw mut (*ipc_sockaddr).sun_path as *mut ::core::ffi::c_char,
            getenv(b"SWAYSOCK\0".as_ptr() as *const ::core::ffi::c_char),
            ::core::mem::size_of::<[::core::ffi::c_char; 108]>()
                .wrapping_sub(1 as size_t),
        );
        (*ipc_sockaddr)
            .sun_path[::core::mem::size_of::<[::core::ffi::c_char; 108]>()
            .wrapping_sub(1usize)] = 0 as ::core::ffi::c_char;
    }
    unlink(&raw mut (*ipc_sockaddr).sun_path as *mut ::core::ffi::c_char);
    if bind(
        ipc_socket,
        ipc_sockaddr as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_un>() as socklen_t,
    ) == -1 as ::core::ffi::c_int
    {
        _sway_abort(
            b"[%s:%d] Unable to bind IPC socket\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            106 as ::core::ffi::c_int,
        );
    }
    if listen(ipc_socket, 3 as ::core::ffi::c_int) == -1 as ::core::ffi::c_int {
        _sway_abort(
            b"[%s:%d] Unable to listen on IPC socket\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            110 as ::core::ffi::c_int,
        );
    }
    setenv(
        b"I3SOCK\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ipc_sockaddr).sun_path as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    setenv(
        b"SWAYSOCK\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ipc_sockaddr).sun_path as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    ipc_client_list = create_list();
    ipc_display_destroy.notify = Some(
        handle_display_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_display_add_destroy_listener(
        (*server_0).wl_display,
        &raw mut ipc_display_destroy,
    );
    ipc_event_source = wl_event_loop_add_fd(
        (*server_0).wl_event_loop,
        ipc_socket,
        C2Rust_Unnamed_0::WL_EVENT_READABLE.0 as ::core::ffi::c_int as uint32_t,
        Some(
            ipc_handle_connection
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    uint32_t,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        server_0 as *mut ::core::ffi::c_void,
    ) as *mut wl_event_source;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_user_sockaddr() -> *mut sockaddr_un {
    let mut ipc_sockaddr_0: *mut sockaddr_un = malloc(
        ::core::mem::size_of::<sockaddr_un>(),
    ) as *mut sockaddr_un;
    if ipc_sockaddr_0.is_null() {
        _sway_abort(
            b"[%s:%d] Can't allocate ipc_sockaddr\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            129 as ::core::ffi::c_int,
        );
    }
    (*ipc_sockaddr_0).sun_family = AF_UNIX as sa_family_t;
    let mut path_size: ::core::ffi::c_int = ::core::mem::size_of::<
        [::core::ffi::c_char; 108],
    >() as ::core::ffi::c_int;
    let mut dir: *const ::core::ffi::c_char = getenv(
        b"XDG_RUNTIME_DIR\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if dir.is_null() {
        dir = b"/tmp\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if path_size
        <= snprintf(
            &raw mut (*ipc_sockaddr_0).sun_path as *mut ::core::ffi::c_char,
            path_size as size_t,
            b"%s/sway-ipc.%u.%i.sock\0".as_ptr() as *const ::core::ffi::c_char,
            dir,
            getuid(),
            getpid(),
        )
    {
        _sway_abort(
            b"[%s:%d] Socket path won't fit into ipc_sockaddr->sun_path\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            142 as ::core::ffi::c_int,
        );
    }
    return ipc_sockaddr_0;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_handle_connection(
    mut fd: ::core::ffi::c_int,
    mut mask: uint32_t,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut server_0: *mut sway_server = data as *mut sway_server;
    if mask == C2Rust_Unnamed_0::WL_EVENT_READABLE.0 as ::core::ffi::c_int as uint32_t
    {} else {
        __assert_fail(
            b"mask == WL_EVENT_READABLE\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            151 as ::core::ffi::c_uint,
            b"int ipc_handle_connection(int, uint32_t, void *)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    let mut client_fd: ::core::ffi::c_int = accept(
        ipc_socket,
        ::core::ptr::null_mut::<sockaddr>(),
        ::core::ptr::null_mut::<socklen_t>(),
    );
    if client_fd == -1 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to accept IPC client connection: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            155 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut flags: ::core::ffi::c_int = 0;
    flags = fcntl(client_fd, F_GETFD);
    if flags == -1 as ::core::ffi::c_int
        || fcntl(client_fd, F_SETFD, flags | FD_CLOEXEC) == -1 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to set CLOEXEC on IPC client socket: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            162 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        close(client_fd);
        return 0 as ::core::ffi::c_int;
    }
    flags = fcntl(client_fd, F_GETFL);
    if flags == -1 as ::core::ffi::c_int
        || fcntl(client_fd, F_SETFL, flags | O_NONBLOCK) == -1 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to set NONBLOCK on IPC client socket: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            168 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        close(client_fd);
        return 0 as ::core::ffi::c_int;
    }
    let mut client: *mut ipc_client = malloc(::core::mem::size_of::<ipc_client>())
        as *mut ipc_client;
    if client.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to allocate ipc client\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            175 as ::core::ffi::c_int,
        );
        close(client_fd);
        return 0 as ::core::ffi::c_int;
    }
    (*client).server = server_0;
    (*client).pending_length = 0 as uint32_t;
    (*client).fd = client_fd;
    (*client).subscribed_events = ipc_command_type(0 as ::core::ffi::c_int);
    (*client).event_source = wl_event_loop_add_fd(
        (*server_0).wl_event_loop,
        client_fd,
        C2Rust_Unnamed_0::WL_EVENT_READABLE.0 as ::core::ffi::c_int as uint32_t,
        Some(
            ipc_client_handle_readable
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    uint32_t,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        client as *mut ::core::ffi::c_void,
    ) as *mut wl_event_source;
    (*client).writable_event_source = ::core::ptr::null_mut::<wl_event_source>();
    (*client).write_buffer_size = 128 as size_t;
    (*client).write_buffer_len = 0 as size_t;
    (*client).write_buffer = malloc((*client).write_buffer_size)
        as *mut ::core::ffi::c_char;
    if (*client).write_buffer.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to allocate ipc client write buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            191 as ::core::ffi::c_int,
        );
        close(client_fd);
        return 0 as ::core::ffi::c_int;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] New client: fd %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        196 as ::core::ffi::c_int,
        client_fd,
    );
    list_add(ipc_client_list, client as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_client_handle_readable(
    mut client_fd: ::core::ffi::c_int,
    mut mask: uint32_t,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut client: *mut ipc_client = data as *mut ipc_client;
    if mask & C2Rust_Unnamed_0::WL_EVENT_ERROR.0 as ::core::ffi::c_int as uint32_t != 0 {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] IPC Client socket error, removing client\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            205 as ::core::ffi::c_int,
        );
        ipc_client_disconnect(client);
        return 0 as ::core::ffi::c_int;
    }
    if mask & C2Rust_Unnamed_0::WL_EVENT_HANGUP.0 as ::core::ffi::c_int as uint32_t != 0
    {
        ipc_client_disconnect(client);
        return 0 as ::core::ffi::c_int;
    }
    let mut read_available: ::core::ffi::c_int = 0;
    if ioctl(client_fd, FIONREAD as ::core::ffi::c_ulong, &raw mut read_available)
        == -1 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Unable to read IPC socket buffer size: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            217 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        ipc_client_disconnect(client);
        return 0 as ::core::ffi::c_int;
    }
    if (*client).pending_length > 0 as uint32_t {
        if read_available as uint32_t >= (*client).pending_length {
            let mut pending_length: uint32_t = (*client).pending_length;
            let mut pending_type: ipc_command_type = (*client).pending_type;
            (*client).pending_length = 0 as uint32_t;
            ipc_client_handle_command(client, pending_length, pending_type);
        }
        return 0 as ::core::ffi::c_int;
    }
    if read_available < IPC_HEADER_SIZE as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut buf: [uint8_t; 14] = [0; 14];
    let mut received: ssize_t = recv(
        client_fd,
        &raw mut buf as *mut uint8_t as *mut ::core::ffi::c_void,
        IPC_HEADER_SIZE,
        0 as ::core::ffi::c_int,
    );
    if received == -1 as ssize_t {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Unable to receive header from IPC client: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            242 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        ipc_client_disconnect(client);
        return 0 as ::core::ffi::c_int;
    }
    if memcmp(
        &raw mut buf as *mut uint8_t as *const ::core::ffi::c_void,
        &raw const ipc_magic as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 6]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] IPC header check failed\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            248 as ::core::ffi::c_int,
        );
        ipc_client_disconnect(client);
        return 0 as ::core::ffi::c_int;
    }
    memcpy(
        &raw mut (*client).pending_length as *mut ::core::ffi::c_void,
        (&raw mut buf as *mut uint8_t)
            .offset(::core::mem::size_of::<[::core::ffi::c_char; 6]>() as isize)
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>(),
    );
    memcpy(
        &raw mut (*client).pending_type as *mut ::core::ffi::c_void,
        (&raw mut buf as *mut uint8_t)
            .offset(::core::mem::size_of::<[::core::ffi::c_char; 6]>() as isize)
            .offset(::core::mem::size_of::<uint32_t>() as isize)
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>(),
    );
    if read_available as ssize_t - received >= (*client).pending_length as ssize_t {
        let mut pending_length_0: uint32_t = (*client).pending_length;
        let mut pending_type_0: ipc_command_type = (*client).pending_type;
        (*client).pending_length = 0 as uint32_t;
        ipc_client_handle_command(client, pending_length_0, pending_type_0);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ipc_has_event_listeners(mut event: ipc_command_type) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*ipc_client_list).length {
        let mut client: *mut ipc_client = *(*ipc_client_list).items.offset(i as isize)
            as *mut ipc_client;
        if (*client).subscribed_events.0
            & (1 as ::core::ffi::c_int) << (event.0 & 0x7f as ::core::ffi::c_int)
            != 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn ipc_send_event(
    mut json_string: *const ::core::ffi::c_char,
    mut event: ipc_command_type,
) {
    let mut client: *mut ipc_client = ::core::ptr::null_mut::<ipc_client>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*ipc_client_list).length {
        client = *(*ipc_client_list).items.offset(i as isize) as *mut ipc_client;
        if (*client).subscribed_events.0
            & (1 as ::core::ffi::c_int) << (event.0 & 0x7f as ::core::ffi::c_int)
            != 0 as ::core::ffi::c_int
        {
            if !ipc_send_reply(
                client,
                event,
                json_string,
                strlen(json_string) as uint32_t,
            ) {
                _sway_log(
                    sway_log_importance_t::SWAY_INFO,
                    b"[%s:%d] Unable to send reply to IPC client: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
                    286 as ::core::ffi::c_int,
                    strerror(*__errno_location()),
                );
                i -= 1;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ipc_event_workspace(
    mut old: *mut sway_workspace,
    mut new: *mut sway_workspace,
    mut change: *const ::core::ffi::c_char,
) {
    if !ipc_has_event_listeners(ipc_command_type::IPC_EVENT_WORKSPACE) {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Sending workspace::%s event\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        300 as ::core::ffi::c_int,
        change,
    );
    let mut obj: *mut json_object = json_object_new_object();
    json_object_object_add(
        obj,
        b"change\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(change),
    );
    if !old.is_null() {
        json_object_object_add(
            obj,
            b"old\0".as_ptr() as *const ::core::ffi::c_char,
            ipc_json_describe_node_recursive(&raw mut (*old).node),
        );
    } else {
        json_object_object_add(
            obj,
            b"old\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<json_object>(),
        );
    }
    if !new.is_null() {
        json_object_object_add(
            obj,
            b"current\0".as_ptr() as *const ::core::ffi::c_char,
            ipc_json_describe_node_recursive(&raw mut (*new).node),
        );
    } else {
        json_object_object_add(
            obj,
            b"current\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<json_object>(),
        );
    }
    let mut json_string: *const ::core::ffi::c_char = json_object_to_json_string(obj);
    ipc_send_event(json_string, ipc_command_type::IPC_EVENT_WORKSPACE);
    json_object_put(obj);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_event_window(
    mut window: *mut sway_container,
    mut change: *const ::core::ffi::c_char,
) {
    if !ipc_has_event_listeners(ipc_command_type::IPC_EVENT_WINDOW) {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Sending window::%s event\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        326 as ::core::ffi::c_int,
        change,
    );
    let mut obj: *mut json_object = json_object_new_object();
    json_object_object_add(
        obj,
        b"change\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(change),
    );
    json_object_object_add(
        obj,
        b"container\0".as_ptr() as *const ::core::ffi::c_char,
        ipc_json_describe_node_recursive(&raw mut (*window).node),
    );
    let mut json_string: *const ::core::ffi::c_char = json_object_to_json_string(obj);
    ipc_send_event(json_string, ipc_command_type::IPC_EVENT_WINDOW);
    json_object_put(obj);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_event_barconfig_update(mut bar: *mut bar_config) {
    if !ipc_has_event_listeners(ipc_command_type::IPC_EVENT_BARCONFIG_UPDATE) {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Sending barconfig_update event\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        341 as ::core::ffi::c_int,
    );
    let mut json: *mut json_object = ipc_json_describe_bar_config(bar);
    let mut json_string: *const ::core::ffi::c_char = json_object_to_json_string(json);
    ipc_send_event(json_string, ipc_command_type::IPC_EVENT_BARCONFIG_UPDATE);
    json_object_put(json);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_event_bar_state_update(mut bar: *mut bar_config) {
    if !ipc_has_event_listeners(ipc_command_type::IPC_EVENT_BAR_STATE_UPDATE) {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Sending bar_state_update event\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        353 as ::core::ffi::c_int,
    );
    let mut json: *mut json_object = json_object_new_object();
    json_object_object_add(
        json,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*bar).id),
    );
    json_object_object_add(
        json,
        b"visible_by_modifier\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean((*bar).visible_by_modifier as json_bool),
    );
    let mut json_string: *const ::core::ffi::c_char = json_object_to_json_string(json);
    ipc_send_event(json_string, ipc_command_type::IPC_EVENT_BAR_STATE_UPDATE);
    json_object_put(json);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_event_mode(
    mut mode: *const ::core::ffi::c_char,
    mut pango: bool,
) {
    if !ipc_has_event_listeners(ipc_command_type::IPC_EVENT_MODE) {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Sending mode::%s event\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        369 as ::core::ffi::c_int,
        mode,
    );
    let mut obj: *mut json_object = json_object_new_object();
    json_object_object_add(
        obj,
        b"change\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(mode),
    );
    json_object_object_add(
        obj,
        b"pango_markup\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(pango as json_bool),
    );
    let mut json_string: *const ::core::ffi::c_char = json_object_to_json_string(obj);
    ipc_send_event(json_string, ipc_command_type::IPC_EVENT_MODE);
    json_object_put(obj);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_event_shutdown(mut reason: *const ::core::ffi::c_char) {
    if !ipc_has_event_listeners(ipc_command_type::IPC_EVENT_SHUTDOWN) {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Sending shutdown::%s event\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        384 as ::core::ffi::c_int,
        reason,
    );
    let mut json: *mut json_object = json_object_new_object();
    json_object_object_add(
        json,
        b"change\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(reason),
    );
    let mut json_string: *const ::core::ffi::c_char = json_object_to_json_string(json);
    ipc_send_event(json_string, ipc_command_type::IPC_EVENT_SHUTDOWN);
    json_object_put(json);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_event_binding(mut binding: *mut sway_binding) {
    if !ipc_has_event_listeners(ipc_command_type::IPC_EVENT_BINDING) {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Sending binding event\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        398 as ::core::ffi::c_int,
    );
    let mut json_binding: *mut json_object = json_object_new_object();
    json_object_object_add(
        json_binding,
        b"command\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string((*binding).command),
    );
    let mut names: [*const ::core::ffi::c_char; 10] = [::core::ptr::null::<
        ::core::ffi::c_char,
    >(); 10];
    let mut len: ::core::ffi::c_int = get_modifier_names(
        &raw mut names as *mut *const ::core::ffi::c_char,
        (*binding).modifiers,
    );
    let mut modifiers: *mut json_object = json_object_new_array();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < len {
        json_object_array_add(modifiers, json_object_new_string(names[i as usize]));
        i += 1;
    }
    json_object_object_add(
        json_binding,
        b"event_state_mask\0".as_ptr() as *const ::core::ffi::c_char,
        modifiers,
    );
    let mut input_codes: *mut json_object = json_object_new_array();
    let mut input_code: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut symbols: *mut json_object = json_object_new_array();
    let mut symbol: *mut json_object = ::core::ptr::null_mut::<json_object>();
    let mut keycode: uint32_t = 0;
    let mut keysym: uint32_t = 0;
    let mut buffer: [::core::ffi::c_char; 64] = [0; 64];
    match (*binding).r#type {
        binding_input_type::BINDING_KEYCODE => {
            keycode = 0;
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < (*(*binding).keys).length {
                keycode = *(*(*(*binding).keys).items.offset(i_0 as isize)
                    as *mut uint32_t);
                json_object_array_add(
                    input_codes,
                    json_object_new_int(keycode as int32_t),
                );
                if i_0 == 0 as ::core::ffi::c_int {
                    input_code = keycode as ::core::ffi::c_int;
                }
                i_0 += 1;
            }
        }
        binding_input_type::BINDING_KEYSYM
        | binding_input_type::BINDING_MOUSESYM
        | binding_input_type::BINDING_MOUSECODE => {
            keysym = 0;
            buffer = [0; 64];
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < (*(*binding).keys).length {
                keysym = *(*(*(*binding).keys).items.offset(i_1 as isize)
                    as *mut uint32_t);
                's_113: {
                    if keysym >= BTN_LEFT as uint32_t
                        && keysym <= (BTN_LEFT + 8 as ::core::ffi::c_int) as uint32_t
                    {
                        snprintf(
                            &raw mut buffer as *mut ::core::ffi::c_char,
                            64 as size_t,
                            b"button%u\0".as_ptr() as *const ::core::ffi::c_char,
                            keysym
                                .wrapping_sub(BTN_LEFT as uint32_t)
                                .wrapping_add(1 as uint32_t),
                        );
                    } else if xkb_keysym_get_name(
                        keysym,
                        &raw mut buffer as *mut ::core::ffi::c_char,
                        64 as size_t,
                    ) < 0 as ::core::ffi::c_int
                    {
                        break 's_113;
                    }
                    let mut str: *mut json_object = json_object_new_string(
                        &raw mut buffer as *mut ::core::ffi::c_char,
                    );
                    if i_1 == 0 as ::core::ffi::c_int {
                        json_object_array_add(symbols, json_object_get(str));
                        symbol = str;
                    } else {
                        json_object_array_add(symbols, str);
                    }
                }
                i_1 += 1;
            }
        }
        _ => {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Unsupported ipc binding event\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
                454 as ::core::ffi::c_int,
            );
            json_object_put(input_codes);
            json_object_put(symbols);
            json_object_put(json_binding);
            return;
        }
    }
    json_object_object_add(
        json_binding,
        b"input_codes\0".as_ptr() as *const ::core::ffi::c_char,
        input_codes,
    );
    json_object_object_add(
        json_binding,
        b"input_code\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_int(input_code as int32_t),
    );
    json_object_object_add(
        json_binding,
        b"symbols\0".as_ptr() as *const ::core::ffi::c_char,
        symbols,
    );
    json_object_object_add(
        json_binding,
        b"symbol\0".as_ptr() as *const ::core::ffi::c_char,
        symbol,
    );
    let mut mouse: bool = (*binding).r#type.0 == binding_input_type::BINDING_MOUSECODE.0
        || (*binding).r#type.0 == binding_input_type::BINDING_MOUSESYM.0;
    json_object_object_add(
        json_binding,
        b"input_type\0".as_ptr() as *const ::core::ffi::c_char,
        if mouse as ::core::ffi::c_int != 0 {
            json_object_new_string(b"mouse\0".as_ptr() as *const ::core::ffi::c_char)
        } else {
            json_object_new_string(b"keyboard\0".as_ptr() as *const ::core::ffi::c_char)
        },
    );
    let mut json: *mut json_object = json_object_new_object();
    json_object_object_add(
        json,
        b"change\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(b"run\0".as_ptr() as *const ::core::ffi::c_char),
    );
    json_object_object_add(
        json,
        b"binding\0".as_ptr() as *const ::core::ffi::c_char,
        json_binding,
    );
    let mut json_string: *const ::core::ffi::c_char = json_object_to_json_string(json);
    ipc_send_event(json_string, ipc_command_type::IPC_EVENT_BINDING);
    json_object_put(json);
}
unsafe extern "C" fn ipc_event_tick(mut payload: *const ::core::ffi::c_char) {
    if !ipc_has_event_listeners(ipc_command_type::IPC_EVENT_TICK) {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Sending tick event\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        484 as ::core::ffi::c_int,
    );
    let mut json: *mut json_object = json_object_new_object();
    json_object_object_add(
        json,
        b"first\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(r#false),
    );
    json_object_object_add(
        json,
        b"payload\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(payload),
    );
    let mut json_string: *const ::core::ffi::c_char = json_object_to_json_string(json);
    ipc_send_event(json_string, ipc_command_type::IPC_EVENT_TICK);
    json_object_put(json);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_event_input(
    mut change: *const ::core::ffi::c_char,
    mut device: *mut sway_input_device,
) {
    if !ipc_has_event_listeners(ipc_command_type::IPC_EVENT_INPUT) {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Sending input event\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        499 as ::core::ffi::c_int,
    );
    let mut json: *mut json_object = json_object_new_object();
    json_object_object_add(
        json,
        b"change\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(change),
    );
    json_object_object_add(
        json,
        b"input\0".as_ptr() as *const ::core::ffi::c_char,
        ipc_json_describe_input(device),
    );
    let mut json_string: *const ::core::ffi::c_char = json_object_to_json_string(json);
    ipc_send_event(json_string, ipc_command_type::IPC_EVENT_INPUT);
    json_object_put(json);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_event_output() {
    if !ipc_has_event_listeners(ipc_command_type::IPC_EVENT_OUTPUT) {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Sending output event\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        514 as ::core::ffi::c_int,
    );
    let mut json: *mut json_object = json_object_new_object();
    json_object_object_add(
        json,
        b"change\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_string(b"unspecified\0".as_ptr() as *const ::core::ffi::c_char),
    );
    let mut json_string: *const ::core::ffi::c_char = json_object_to_json_string(json);
    ipc_send_event(json_string, ipc_command_type::IPC_EVENT_OUTPUT);
    json_object_put(json);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_client_handle_writable(
    mut client_fd: ::core::ffi::c_int,
    mut mask: uint32_t,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut client: *mut ipc_client = data as *mut ipc_client;
    if mask & C2Rust_Unnamed_0::WL_EVENT_ERROR.0 as ::core::ffi::c_int as uint32_t != 0 {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] IPC Client socket error, removing client\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            528 as ::core::ffi::c_int,
        );
        ipc_client_disconnect(client);
        return 0 as ::core::ffi::c_int;
    }
    if mask & C2Rust_Unnamed_0::WL_EVENT_HANGUP.0 as ::core::ffi::c_int as uint32_t != 0
    {
        ipc_client_disconnect(client);
        return 0 as ::core::ffi::c_int;
    }
    if (*client).write_buffer_len <= 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    let mut written: ssize_t = write(
        (*client).fd,
        (*client).write_buffer as *const ::core::ffi::c_void,
        (*client).write_buffer_len,
    );
    if written == -1 as ssize_t && *__errno_location() == EAGAIN {
        return 0 as ::core::ffi::c_int
    } else if written == -1 as ssize_t {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Unable to send data from queue to IPC client: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            547 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        ipc_client_disconnect(client);
        return 0 as ::core::ffi::c_int;
    }
    memmove(
        (*client).write_buffer as *mut ::core::ffi::c_void,
        (*client).write_buffer.offset(written as isize) as *const ::core::ffi::c_void,
        (*client).write_buffer_len.wrapping_sub(written as size_t),
    );
    (*client).write_buffer_len = (*client)
        .write_buffer_len
        .wrapping_sub(written as size_t);
    if (*client).write_buffer_len == 0 as size_t
        && !(*client).writable_event_source.is_null()
    {
        wl_event_source_remove((*client).writable_event_source);
        (*client).writable_event_source = ::core::ptr::null_mut::<wl_event_source>();
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_client_disconnect(mut client: *mut ipc_client) {
    if !_sway_assert(
        !client.is_null(),
        b"[%s:%d] %s:client != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        564 as ::core::ffi::c_int,
        b"void ipc_client_disconnect(struct ipc_client *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    shutdown((*client).fd, C2Rust_Unnamed::SHUT_RDWR.0 as ::core::ffi::c_int);
    _sway_log(
        sway_log_importance_t::SWAY_INFO,
        b"[%s:%d] IPC Client %d disconnected\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        570 as ::core::ffi::c_int,
        (*client).fd,
    );
    wl_event_source_remove((*client).event_source);
    if !(*client).writable_event_source.is_null() {
        wl_event_source_remove((*client).writable_event_source);
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*ipc_client_list).length
        && *(*ipc_client_list).items.offset(i as isize)
            != client as *mut ::core::ffi::c_void
    {
        i += 1;
    }
    list_del(ipc_client_list, i);
    free((*client).write_buffer as *mut ::core::ffi::c_void);
    close((*client).fd);
    free(client as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn ipc_get_workspaces_callback(
    mut workspace: *mut sway_workspace,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut workspace_json: *mut json_object = ipc_json_describe_node(
        &raw mut (*workspace).node,
    );
    let mut seat: *mut sway_seat = input_manager_get_default_seat();
    let mut focused_ws: *mut sway_workspace = seat_get_focused_workspace(seat);
    let mut focused: bool = workspace == focused_ws;
    json_object_object_del(
        workspace_json,
        b"focused\0".as_ptr() as *const ::core::ffi::c_char,
    );
    json_object_object_add(
        workspace_json,
        b"focused\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(focused as json_bool),
    );
    json_object_array_add(data as *mut json_object, workspace_json);
    focused_ws = output_get_active_workspace((*workspace).output);
    let mut visible: bool = workspace == focused_ws;
    json_object_object_add(
        workspace_json,
        b"visible\0".as_ptr() as *const ::core::ffi::c_char,
        json_object_new_boolean(visible as json_bool),
    );
}
unsafe extern "C" fn ipc_get_marks_callback(
    mut con: *mut sway_container,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut marks: *mut json_object = data as *mut json_object;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*con).marks).length {
        let mut mark: *mut ::core::ffi::c_char = *(*(*con).marks)
            .items
            .offset(i as isize) as *mut ::core::ffi::c_char;
        json_object_array_add(marks, json_object_new_string(mark));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ipc_client_handle_command(
    mut client: *mut ipc_client,
    mut payload_length: uint32_t,
    mut payload_type: ipc_command_type,
) {
    if !_sway_assert(
        !client.is_null(),
        b"[%s:%d] %s:client != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
        614 as ::core::ffi::c_int,
        b"void ipc_client_handle_command(struct ipc_client *, uint32_t, enum ipc_command_type)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return;
    }
    let mut buf: *mut ::core::ffi::c_char = malloc(
        payload_length.wrapping_add(1 as uint32_t) as size_t,
    ) as *mut ::core::ffi::c_char;
    if buf.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Unable to allocate IPC payload: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            620 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        ipc_client_disconnect(client);
        return;
    }
    if payload_length > 0 as uint32_t {
        let mut received: ssize_t = recv(
            (*client).fd,
            buf as *mut ::core::ffi::c_void,
            payload_length as size_t,
            0 as ::core::ffi::c_int,
        );
        if received == -1 as ssize_t {
            _sway_log(
                sway_log_importance_t::SWAY_INFO,
                b"[%s:%d] Unable to receive payload from IPC client: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
                629 as ::core::ffi::c_int,
                strerror(*__errno_location()),
            );
            ipc_client_disconnect(client);
            free(buf as *mut ::core::ffi::c_void);
            return;
        }
    }
    *buf.offset(payload_length as isize) = '\0' as ::core::ffi::c_char;
    '_exit_cleanup: {
        match payload_type {
            ipc_command_type::IPC_COMMAND => {
                let mut line: *mut ::core::ffi::c_char = strtok(
                    buf,
                    b"\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                while !line.is_null() {
                    let mut line_length: size_t = strlen(line);
                    if line.offset(line_length as isize)
                        >= buf.offset(payload_length as isize)
                    {
                        break;
                    }
                    *line.offset(line_length as isize) = ';' as ::core::ffi::c_char;
                    line = strtok(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                let mut res_list: *mut list_t = execute_command(
                    buf,
                    ::core::ptr::null_mut::<sway_seat>(),
                    ::core::ptr::null_mut::<sway_container>(),
                );
                if modeset_is_pending() {
                    force_modeset();
                }
                transaction_commit_dirty();
                let mut json: *mut ::core::ffi::c_char = cmd_results_to_json(res_list);
                let mut length: ::core::ffi::c_int = strlen(json) as ::core::ffi::c_int;
                ipc_send_reply(client, payload_type, json, length as uint32_t);
                free(json as *mut ::core::ffi::c_void);
                while (*res_list).length != 0 {
                    let mut results: *mut cmd_results = *(*res_list).items.offset(0isize)
                        as *mut cmd_results;
                    free_cmd_results(results);
                    list_del(res_list, 0 as ::core::ffi::c_int);
                }
                list_free(res_list);
            }
            ipc_command_type::IPC_SEND_TICK => {
                ipc_event_tick(buf);
                ipc_send_reply(
                    client,
                    payload_type,
                    b"{\"success\": true}\0".as_ptr() as *const ::core::ffi::c_char,
                    17 as uint32_t,
                );
            }
            ipc_command_type::IPC_GET_OUTPUTS => {
                let mut outputs: *mut json_object = json_object_new_array();
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < (*(*root).outputs).length {
                    let mut output: *mut sway_output = *(*(*root).outputs)
                        .items
                        .offset(i as isize) as *mut sway_output;
                    let mut output_json: *mut json_object = ipc_json_describe_node(
                        &raw mut (*output).node,
                    );
                    let mut seat: *mut sway_seat = input_manager_get_default_seat();
                    let mut focused_ws: *mut sway_workspace = seat_get_focused_workspace(
                        seat,
                    );
                    let mut focused: bool = !focused_ws.is_null()
                        && output == (*focused_ws).output;
                    json_object_object_del(
                        output_json,
                        b"focused\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    json_object_object_add(
                        output_json,
                        b"focused\0".as_ptr() as *const ::core::ffi::c_char,
                        json_object_new_boolean(focused as json_bool),
                    );
                    let mut subpixel: *const ::core::ffi::c_char = sway_wl_output_subpixel_to_string(
                        (*(*output).wlr_output).subpixel,
                    );
                    json_object_object_add(
                        output_json,
                        b"subpixel_hinting\0".as_ptr() as *const ::core::ffi::c_char,
                        json_object_new_string(subpixel),
                    );
                    json_object_array_add(outputs, output_json);
                    i += 1;
                }
                let mut output_0: *mut sway_output = ::core::ptr::null_mut::<
                    sway_output,
                >();
                output_0 = ((*root).all_outputs.next as *mut ::core::ffi::c_char)
                    .offset(-(152 as ::core::ffi::c_ulong as isize)) as *mut sway_output
                    as *mut sway_output;
                while &raw mut (*output_0).link != &raw mut (*root).all_outputs {
                    if !(*output_0).enabled && output_0 != (*root).fallback_output {
                        json_object_array_add(
                            outputs,
                            ipc_json_describe_disabled_output(output_0),
                        );
                    }
                    output_0 = ((*output_0).link.next as *mut ::core::ffi::c_char)
                        .offset(-(152 as ::core::ffi::c_ulong as isize))
                        as *mut sway_output as *mut sway_output;
                }
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < (*(*root).non_desktop_outputs).length {
                    let mut non_desktop_output: *mut sway_output_non_desktop = *(*(*root)
                        .non_desktop_outputs)
                        .items
                        .offset(i_0 as isize) as *mut sway_output_non_desktop;
                    json_object_array_add(
                        outputs,
                        ipc_json_describe_non_desktop_output(non_desktop_output),
                    );
                    i_0 += 1;
                }
                let mut json_string: *const ::core::ffi::c_char = json_object_to_json_string(
                    outputs,
                );
                ipc_send_reply(
                    client,
                    payload_type,
                    json_string,
                    strlen(json_string) as uint32_t,
                );
                json_object_put(outputs);
            }
            ipc_command_type::IPC_GET_WORKSPACES => {
                let mut workspaces: *mut json_object = json_object_new_array();
                root_for_each_workspace(
                    Some(
                        ipc_get_workspaces_callback
                            as unsafe extern "C" fn(
                                *mut sway_workspace,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                    workspaces as *mut ::core::ffi::c_void,
                );
                let mut json_string_0: *const ::core::ffi::c_char = json_object_to_json_string(
                    workspaces,
                );
                ipc_send_reply(
                    client,
                    payload_type,
                    json_string_0,
                    strlen(json_string_0) as uint32_t,
                );
                json_object_put(workspaces);
            }
            ipc_command_type::IPC_SUBSCRIBE => {
                let mut request: *mut json_object = json_tokener_parse(buf);
                if request.is_null()
                    || json_object_is_type(request, json_type::json_type_array) == 0
                {
                    let msg: [::core::ffi::c_char; 19] = ::core::mem::transmute::<
                        [u8; 19],
                        [::core::ffi::c_char; 19],
                    >(*b"{\"success\": false}\0");
                    ipc_send_reply(
                        client,
                        payload_type,
                        &raw const msg as *const ::core::ffi::c_char,
                        strlen(&raw const msg as *const ::core::ffi::c_char) as uint32_t,
                    );
                    json_object_put(request);
                    _sway_log(
                        sway_log_importance_t::SWAY_INFO,
                        b"[%s:%d] Failed to parse subscribe request\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
                        738 as ::core::ffi::c_int,
                    );
                } else {
                    let mut is_tick: bool = r#false != 0;
                    let mut i_1: size_t = 0 as size_t;
                    while i_1 < json_object_array_length(request) {
                        let mut event_type: *const ::core::ffi::c_char = json_object_get_string(
                            json_object_array_get_idx(request, i_1),
                        );
                        if strcmp(
                            event_type,
                            b"workspace\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*client).subscribed_events = ipc_command_type(
                                (*client).subscribed_events.0
                                    | (1 as ::core::ffi::c_int)
                                        << (ipc_command_type::IPC_EVENT_WORKSPACE.0
                                            & 0x7f as ::core::ffi::c_int),
                            );
                        } else if strcmp(
                            event_type,
                            b"output\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*client).subscribed_events = ipc_command_type(
                                (*client).subscribed_events.0
                                    | (1 as ::core::ffi::c_int)
                                        << (ipc_command_type::IPC_EVENT_OUTPUT.0
                                            & 0x7f as ::core::ffi::c_int),
                            );
                        } else if strcmp(
                            event_type,
                            b"barconfig_update\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*client).subscribed_events = ipc_command_type(
                                (*client).subscribed_events.0
                                    | (1 as ::core::ffi::c_int)
                                        << (ipc_command_type::IPC_EVENT_BARCONFIG_UPDATE.0
                                            & 0x7f as ::core::ffi::c_int),
                            );
                        } else if strcmp(
                            event_type,
                            b"bar_state_update\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*client).subscribed_events = ipc_command_type(
                                (*client).subscribed_events.0
                                    | (1 as ::core::ffi::c_int)
                                        << (ipc_command_type::IPC_EVENT_BAR_STATE_UPDATE.0
                                            & 0x7f as ::core::ffi::c_int),
                            );
                        } else if strcmp(
                            event_type,
                            b"mode\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*client).subscribed_events = ipc_command_type(
                                (*client).subscribed_events.0
                                    | (1 as ::core::ffi::c_int)
                                        << (ipc_command_type::IPC_EVENT_MODE.0
                                            & 0x7f as ::core::ffi::c_int),
                            );
                        } else if strcmp(
                            event_type,
                            b"shutdown\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*client).subscribed_events = ipc_command_type(
                                (*client).subscribed_events.0
                                    | (1 as ::core::ffi::c_int)
                                        << (ipc_command_type::IPC_EVENT_SHUTDOWN.0
                                            & 0x7f as ::core::ffi::c_int),
                            );
                        } else if strcmp(
                            event_type,
                            b"window\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*client).subscribed_events = ipc_command_type(
                                (*client).subscribed_events.0
                                    | (1 as ::core::ffi::c_int)
                                        << (ipc_command_type::IPC_EVENT_WINDOW.0
                                            & 0x7f as ::core::ffi::c_int),
                            );
                        } else if strcmp(
                            event_type,
                            b"binding\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*client).subscribed_events = ipc_command_type(
                                (*client).subscribed_events.0
                                    | (1 as ::core::ffi::c_int)
                                        << (ipc_command_type::IPC_EVENT_BINDING.0
                                            & 0x7f as ::core::ffi::c_int),
                            );
                        } else if strcmp(
                            event_type,
                            b"tick\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*client).subscribed_events = ipc_command_type(
                                (*client).subscribed_events.0
                                    | (1 as ::core::ffi::c_int)
                                        << (ipc_command_type::IPC_EVENT_TICK.0
                                            & 0x7f as ::core::ffi::c_int),
                            );
                            is_tick = r#true != 0;
                        } else if strcmp(
                            event_type,
                            b"input\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            (*client).subscribed_events = ipc_command_type(
                                (*client).subscribed_events.0
                                    | (1 as ::core::ffi::c_int)
                                        << (ipc_command_type::IPC_EVENT_INPUT.0
                                            & 0x7f as ::core::ffi::c_int),
                            );
                        } else {
                            let msg_0: [::core::ffi::c_char; 19] = ::core::mem::transmute::<
                                [u8; 19],
                                [::core::ffi::c_char; 19],
                            >(*b"{\"success\": false}\0");
                            ipc_send_reply(
                                client,
                                payload_type,
                                &raw const msg_0 as *const ::core::ffi::c_char,
                                strlen(&raw const msg_0 as *const ::core::ffi::c_char)
                                    as uint32_t,
                            );
                            json_object_put(request);
                            _sway_log(
                                sway_log_importance_t::SWAY_INFO,
                                b"[%s:%d] Unsupported event type in subscribe request\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"sway/ipc-server.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                771 as ::core::ffi::c_int,
                            );
                            break '_exit_cleanup;
                        }
                        i_1 = i_1.wrapping_add(1);
                    }
                    json_object_put(request);
                    let msg_1: [::core::ffi::c_char; 18] = ::core::mem::transmute::<
                        [u8; 18],
                        [::core::ffi::c_char; 18],
                    >(*b"{\"success\": true}\0");
                    ipc_send_reply(
                        client,
                        payload_type,
                        &raw const msg_1 as *const ::core::ffi::c_char,
                        strlen(&raw const msg_1 as *const ::core::ffi::c_char)
                            as uint32_t,
                    );
                    if is_tick {
                        let tickmsg: [::core::ffi::c_char; 31] = ::core::mem::transmute::<
                            [u8; 31],
                            [::core::ffi::c_char; 31],
                        >(*b"{\"first\": true, \"payload\": \"\"}\0");
                        ipc_send_reply(
                            client,
                            ipc_command_type::IPC_EVENT_TICK,
                            &raw const tickmsg as *const ::core::ffi::c_char,
                            strlen(&raw const tickmsg as *const ::core::ffi::c_char)
                                as uint32_t,
                        );
                    }
                }
            }
            ipc_command_type::IPC_GET_INPUTS => {
                let mut inputs: *mut json_object = json_object_new_array();
                let mut device: *mut sway_input_device = ::core::ptr::null_mut::<
                    sway_input_device,
                >();
                device = ((*server.input).devices.next as *mut ::core::ffi::c_char)
                    .offset(-(16 as ::core::ffi::c_ulong as isize))
                    as *mut sway_input_device as *mut sway_input_device;
                while &raw mut (*device).link != &raw mut (*server.input).devices {
                    json_object_array_add(inputs, ipc_json_describe_input(device));
                    device = ((*device).link.next as *mut ::core::ffi::c_char)
                        .offset(-(16 as ::core::ffi::c_ulong as isize))
                        as *mut sway_input_device as *mut sway_input_device;
                }
                let mut json_string_1: *const ::core::ffi::c_char = json_object_to_json_string(
                    inputs,
                );
                ipc_send_reply(
                    client,
                    payload_type,
                    json_string_1,
                    strlen(json_string_1) as uint32_t,
                );
                json_object_put(inputs);
            }
            ipc_command_type::IPC_GET_SEATS => {
                let mut seats: *mut json_object = json_object_new_array();
                let mut seat_0: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
                seat_0 = ((*server.input).seats.next as *mut ::core::ffi::c_char)
                    .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
                    as *mut sway_seat;
                while &raw mut (*seat_0).link != &raw mut (*server.input).seats {
                    json_object_array_add(seats, ipc_json_describe_seat(seat_0));
                    seat_0 = ((*seat_0).link.next as *mut ::core::ffi::c_char)
                        .offset(-(624 as ::core::ffi::c_ulong as isize))
                        as *mut sway_seat as *mut sway_seat;
                }
                let mut json_string_2: *const ::core::ffi::c_char = json_object_to_json_string(
                    seats,
                );
                ipc_send_reply(
                    client,
                    payload_type,
                    json_string_2,
                    strlen(json_string_2) as uint32_t,
                );
                json_object_put(seats);
            }
            ipc_command_type::IPC_GET_TREE => {
                let mut tree: *mut json_object = ipc_json_describe_node_recursive(
                    &raw mut (*root).node,
                );
                let mut json_string_3: *const ::core::ffi::c_char = json_object_to_json_string(
                    tree,
                );
                ipc_send_reply(
                    client,
                    payload_type,
                    json_string_3,
                    strlen(json_string_3) as uint32_t,
                );
                json_object_put(tree);
            }
            ipc_command_type::IPC_GET_MARKS => {
                let mut marks: *mut json_object = json_object_new_array();
                root_for_each_container(
                    Some(
                        ipc_get_marks_callback
                            as unsafe extern "C" fn(
                                *mut sway_container,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                    marks as *mut ::core::ffi::c_void,
                );
                let mut json_string_4: *const ::core::ffi::c_char = json_object_to_json_string(
                    marks,
                );
                ipc_send_reply(
                    client,
                    payload_type,
                    json_string_4,
                    strlen(json_string_4) as uint32_t,
                );
                json_object_put(marks);
            }
            ipc_command_type::IPC_GET_VERSION => {
                let mut version: *mut json_object = ipc_json_get_version();
                let mut json_string_5: *const ::core::ffi::c_char = json_object_to_json_string(
                    version,
                );
                ipc_send_reply(
                    client,
                    payload_type,
                    json_string_5,
                    strlen(json_string_5) as uint32_t,
                );
                json_object_put(version);
            }
            ipc_command_type::IPC_GET_BAR_CONFIG => {
                if *buf.offset(0isize) == 0 {
                    let mut bars: *mut json_object = json_object_new_array();
                    let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_2 < (*(*config).bars).length {
                        let mut bar: *mut bar_config = *(*(*config).bars)
                            .items
                            .offset(i_2 as isize) as *mut bar_config;
                        json_object_array_add(bars, json_object_new_string((*bar).id));
                        i_2 += 1;
                    }
                    let mut json_string_6: *const ::core::ffi::c_char = json_object_to_json_string(
                        bars,
                    );
                    ipc_send_reply(
                        client,
                        payload_type,
                        json_string_6,
                        strlen(json_string_6) as uint32_t,
                    );
                    json_object_put(bars);
                } else {
                    let mut bar_0: *mut bar_config = ::core::ptr::null_mut::<
                        bar_config,
                    >();
                    let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_3 < (*(*config).bars).length {
                        bar_0 = *(*(*config).bars).items.offset(i_3 as isize)
                            as *mut bar_config;
                        if strcmp(buf, (*bar_0).id) == 0 as ::core::ffi::c_int {
                            break;
                        }
                        bar_0 = ::core::ptr::null_mut::<bar_config>();
                        i_3 += 1;
                    }
                    if bar_0.is_null() {
                        let mut error: *const ::core::ffi::c_char = b"{ \"success\": false, \"error\": \"No bar with that ID\" }\0"
                            .as_ptr() as *const ::core::ffi::c_char;
                        ipc_send_reply(
                            client,
                            payload_type,
                            error,
                            strlen(error) as uint32_t,
                        );
                    } else {
                        let mut json_0: *mut json_object = ipc_json_describe_bar_config(
                            bar_0,
                        );
                        let mut json_string_7: *const ::core::ffi::c_char = json_object_to_json_string(
                            json_0,
                        );
                        ipc_send_reply(
                            client,
                            payload_type,
                            json_string_7,
                            strlen(json_string_7) as uint32_t,
                        );
                        json_object_put(json_0);
                    }
                }
            }
            ipc_command_type::IPC_GET_BINDING_MODES => {
                let mut modes: *mut json_object = json_object_new_array();
                let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_4 < (*(*config).modes).length {
                    let mut mode: *mut sway_mode = *(*(*config).modes)
                        .items
                        .offset(i_4 as isize) as *mut sway_mode;
                    json_object_array_add(modes, json_object_new_string((*mode).name));
                    i_4 += 1;
                }
                let mut json_string_8: *const ::core::ffi::c_char = json_object_to_json_string(
                    modes,
                );
                ipc_send_reply(
                    client,
                    payload_type,
                    json_string_8,
                    strlen(json_string_8) as uint32_t,
                );
                json_object_put(modes);
            }
            ipc_command_type::IPC_GET_BINDING_STATE => {
                let mut current_mode: *mut json_object = ipc_json_get_binding_mode();
                let mut json_string_9: *const ::core::ffi::c_char = json_object_to_json_string(
                    current_mode,
                );
                ipc_send_reply(
                    client,
                    payload_type,
                    json_string_9,
                    strlen(json_string_9) as uint32_t,
                );
                json_object_put(current_mode);
            }
            ipc_command_type::IPC_GET_CONFIG => {
                let mut json_1: *mut json_object = json_object_new_object();
                json_object_object_add(
                    json_1,
                    b"config\0".as_ptr() as *const ::core::ffi::c_char,
                    json_object_new_string((*config).current_config),
                );
                let mut json_string_10: *const ::core::ffi::c_char = json_object_to_json_string(
                    json_1,
                );
                ipc_send_reply(
                    client,
                    payload_type,
                    json_string_10,
                    strlen(json_string_10) as uint32_t,
                );
                json_object_put(json_1);
            }
            ipc_command_type::IPC_SYNC => {
                let msg_2: [::core::ffi::c_char; 19] = ::core::mem::transmute::<
                    [u8; 19],
                    [::core::ffi::c_char; 19],
                >(*b"{\"success\": false}\0");
                ipc_send_reply(
                    client,
                    payload_type,
                    &raw const msg_2 as *const ::core::ffi::c_char,
                    strlen(&raw const msg_2 as *const ::core::ffi::c_char) as uint32_t,
                );
            }
            _ => {
                _sway_log(
                    sway_log_importance_t::SWAY_INFO,
                    b"[%s:%d] Unknown IPC command type %x\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
                    928 as ::core::ffi::c_int,
                    payload_type.0,
                );
            }
        }
    }
    free(buf as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_send_reply(
    mut client: *mut ipc_client,
    mut payload_type: ipc_command_type,
    mut payload: *const ::core::ffi::c_char,
    mut payload_length: uint32_t,
) -> bool {
    if !payload.is_null() {} else {
        __assert_fail(
            b"payload\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            938 as ::core::ffi::c_uint,
            b"_Bool ipc_send_reply(struct ipc_client *, enum ipc_command_type, const char *, uint32_t)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    let mut data: [::core::ffi::c_char; 14] = [0; 14];
    memcpy(
        &raw mut data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        &raw const ipc_magic as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 6]>(),
    );
    memcpy(
        (&raw mut data as *mut ::core::ffi::c_char)
            .offset(::core::mem::size_of::<[::core::ffi::c_char; 6]>() as isize)
            as *mut ::core::ffi::c_void,
        &raw mut payload_length as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>(),
    );
    memcpy(
        (&raw mut data as *mut ::core::ffi::c_char)
            .offset(::core::mem::size_of::<[::core::ffi::c_char; 6]>() as isize)
            .offset(::core::mem::size_of::<uint32_t>() as isize)
            as *mut ::core::ffi::c_void,
        &raw mut payload_type as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ipc_command_type>(),
    );
    while (*client)
        .write_buffer_len
        .wrapping_add(IPC_HEADER_SIZE)
        .wrapping_add(payload_length as size_t) >= (*client).write_buffer_size
    {
        (*client).write_buffer_size = (*client)
            .write_buffer_size
            .wrapping_mul(2 as size_t);
    }
    if (*client).write_buffer_size as ::core::ffi::c_double > 4e6f64 {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Client write buffer too big (%zu), disconnecting client\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            952 as ::core::ffi::c_int,
            (*client).write_buffer_size,
        );
        ipc_client_disconnect(client);
        return r#false != 0;
    }
    let mut new_buffer: *mut ::core::ffi::c_char = realloc(
        (*client).write_buffer as *mut ::core::ffi::c_void,
        (*client).write_buffer_size,
    ) as *mut ::core::ffi::c_char;
    if new_buffer.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to reallocate ipc client write buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/ipc-server.c\0".as_ptr() as *const ::core::ffi::c_char,
            960 as ::core::ffi::c_int,
        );
        ipc_client_disconnect(client);
        return r#false != 0;
    }
    (*client).write_buffer = new_buffer;
    memcpy(
        (*client).write_buffer.offset((*client).write_buffer_len as isize)
            as *mut ::core::ffi::c_void,
        &raw mut data as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        IPC_HEADER_SIZE,
    );
    (*client).write_buffer_len = ((*client).write_buffer_len as ::core::ffi::c_ulong)
        .wrapping_add(IPC_HEADER_SIZE as ::core::ffi::c_ulong) as size_t;
    memcpy(
        (*client).write_buffer.offset((*client).write_buffer_len as isize)
            as *mut ::core::ffi::c_void,
        payload as *const ::core::ffi::c_void,
        payload_length as size_t,
    );
    (*client).write_buffer_len = (*client)
        .write_buffer_len
        .wrapping_add(payload_length as size_t);
    if (*client).writable_event_source.is_null() {
        (*client).writable_event_source = wl_event_loop_add_fd(
            server.wl_event_loop,
            (*client).fd,
            C2Rust_Unnamed_0::WL_EVENT_WRITABLE.0 as ::core::ffi::c_int as uint32_t,
            Some(
                ipc_client_handle_writable
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        uint32_t,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            client as *mut ::core::ffi::c_void,
        ) as *mut wl_event_source;
    }
    return r#true != 0;
}
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const F_GETFD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_GETFL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BTN_LEFT: ::core::ffi::c_int = 0x110 as ::core::ffi::c_int;
