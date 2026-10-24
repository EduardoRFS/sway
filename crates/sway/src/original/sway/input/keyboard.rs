use ::c2rust_bitfields;
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
    pub type libseat;
    pub type udev_monitor;
    pub type udev;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type xkb_context;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_xcursor_manager;
    pub type wlr_cursor_state;
    pub type wlr_linux_dmabuf_feedback_v1_compiled;
    pub type wlr_color_manager_v1;
    pub type wlr_gamma_control_manager_v1;
    pub type sway_workspace;
    pub type sway_text_node;
    pub type sway_view;
    pub type sway_transaction_instruction;
    pub type sway_output;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_primary_selection_source;
    pub type wlr_tablet_v2_tablet_tool;
    pub type wlr_tablet_v2_tablet;
    pub type wlr_tablet_v2_tablet_pad;
    pub type wlr_tablet_pad;
    pub type sway_switch;
    pub type wlr_touch_impl;
    pub type _PangoFontDescription;
    pub type sway_transaction;
    pub type wlr_ext_workspace_manager_v1;
    pub type wlr_tearing_control_manager_v1;
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
    pub type wlr_tablet_manager_v2;
    pub type wlr_xdg_shell;
    pub type wlr_idle_notifier_v1;
    pub type wlr_data_device_manager;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    fn wl_event_source_remove(source: *mut wl_event_source) -> ::core::ffi::c_int;
    fn wl_event_loop_add_idle(
        r#loop: *mut wl_event_loop,
        func: wl_event_loop_idle_func_t,
        data: *mut ::core::ffi::c_void,
    ) -> *mut wl_event_source;
    fn wl_resource_get_client(resource: *mut wl_resource) -> *mut wl_client;
    fn wlr_session_change_vt(session: *mut wlr_session, vt: ::core::ffi::c_uint) -> bool;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn xkb_context_new(flags: xkb_context_flags) -> *mut xkb_context;
    fn xkb_context_unref(context: *mut xkb_context);
    fn xkb_context_set_user_data(
        context: *mut xkb_context,
        user_data: *mut ::core::ffi::c_void,
    );
    fn xkb_context_get_user_data(context: *mut xkb_context) -> *mut ::core::ffi::c_void;
    fn xkb_context_set_log_fn(
        context: *mut xkb_context,
        log_fn: Option<
            unsafe extern "C" fn(
                *mut xkb_context,
                xkb_log_level,
                *const ::core::ffi::c_char,
                ::core::ffi::VaList,
            ) -> (),
        >,
    );
    fn xkb_keymap_new_from_names(
        context: *mut xkb_context,
        names: *const xkb_rule_names,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;
    fn xkb_keymap_new_from_file(
        context: *mut xkb_context,
        file: *mut FILE,
        format: xkb_keymap_format,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;
    fn xkb_keymap_unref(keymap: *mut xkb_keymap);
    fn xkb_keymap_mod_get_index(
        keymap: *mut xkb_keymap,
        name: *const ::core::ffi::c_char,
    ) -> xkb_mod_index_t;
    fn xkb_keymap_key_get_syms_by_level(
        keymap: *mut xkb_keymap,
        key: xkb_keycode_t,
        layout: xkb_layout_index_t,
        level: xkb_level_index_t,
        syms_out: *mut *const xkb_keysym_t,
    ) -> ::core::ffi::c_int;
    fn xkb_state_key_get_syms(
        state: *mut xkb_state,
        key: xkb_keycode_t,
        syms_out: *mut *const xkb_keysym_t,
    ) -> ::core::ffi::c_int;
    fn xkb_state_key_get_layout(
        state: *mut xkb_state,
        key: xkb_keycode_t,
    ) -> xkb_layout_index_t;
    fn xkb_state_key_get_consumed_mods2(
        state: *mut xkb_state,
        key: xkb_keycode_t,
        mode: xkb_consumed_mode,
    ) -> xkb_mod_mask_t;
    fn xkb_state_led_index_is_active(
        state: *mut xkb_state,
        idx: xkb_led_index_t,
    ) -> ::core::ffi::c_int;
    fn wlr_keyboard_from_input_device(
        input_device: *mut wlr_input_device,
    ) -> *mut wlr_keyboard;
    fn wlr_keyboard_set_keymap(kb: *mut wlr_keyboard, keymap: *mut xkb_keymap) -> bool;
    fn wlr_keyboard_keymaps_match(km1: *mut xkb_keymap, km2: *mut xkb_keymap) -> bool;
    fn wlr_keyboard_keysym_to_pointer_button(keysym: xkb_keysym_t) -> uint32_t;
    fn wlr_keyboard_keysym_to_pointer_motion(
        keysym: xkb_keysym_t,
        dx: *mut ::core::ffi::c_int,
        dy: *mut ::core::ffi::c_int,
    );
    fn wlr_keyboard_set_repeat_info(
        kb: *mut wlr_keyboard,
        rate_hz: int32_t,
        delay_ms: int32_t,
    );
    fn wlr_keyboard_led_update(keyboard: *mut wlr_keyboard, leds: uint32_t);
    fn wlr_keyboard_get_modifiers(keyboard: *mut wlr_keyboard) -> uint32_t;
    fn wlr_keyboard_group_create() -> *mut wlr_keyboard_group;
    fn wlr_keyboard_group_from_wlr_keyboard(
        keyboard: *mut wlr_keyboard,
    ) -> *mut wlr_keyboard_group;
    fn wlr_keyboard_group_add_keyboard(
        group: *mut wlr_keyboard_group,
        keyboard: *mut wlr_keyboard,
    ) -> bool;
    fn wlr_keyboard_group_remove_keyboard(
        group: *mut wlr_keyboard_group,
        keyboard: *mut wlr_keyboard,
    );
    fn wlr_keyboard_group_destroy(group: *mut wlr_keyboard_group);
    fn wlr_keyboard_notify_modifiers(
        keyboard: *mut wlr_keyboard,
        mods_depressed: uint32_t,
        mods_latched: uint32_t,
        mods_locked: uint32_t,
        group: uint32_t,
    );
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn vformat_str(
        fmt: *const ::core::ffi::c_char,
        args: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn format_str(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn wlr_seat_pointer_notify_frame(wlr_seat: *mut wlr_seat);
    fn wlr_seat_set_keyboard(seat: *mut wlr_seat, keyboard: *mut wlr_keyboard);
    fn wlr_seat_get_keyboard(seat: *mut wlr_seat) -> *mut wlr_keyboard;
    fn wlr_seat_keyboard_notify_key(
        seat: *mut wlr_seat,
        time_msec: uint32_t,
        key: uint32_t,
        state: uint32_t,
    );
    fn wlr_seat_keyboard_notify_modifiers(
        seat: *mut wlr_seat,
        modifiers_0: *const wlr_keyboard_modifiers,
    );
    fn cursor_notify_key_press(cursor: *mut sway_cursor);
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
    fn seat_set_focus(seat: *mut sway_seat, node: *mut sway_node);
    fn seat_set_focus_surface(
        seat: *mut sway_seat,
        surface: *mut wlr_surface,
        unfocus: bool,
    );
    fn seat_set_focus_layer(seat: *mut sway_seat, layer: *mut wlr_layer_surface_v1);
    fn seat_get_focus(seat: *mut sway_seat) -> *mut sway_node;
    fn seat_get_config(seat: *mut sway_seat) -> *mut seat_config;
    fn seat_get_config_by_name(name: *const ::core::ffi::c_char) -> *mut seat_config;
    fn seat_idle_notify_activity(seat: *mut sway_seat, source: sway_input_idle_source);
    fn keyboard_shortcuts_inhibitor_get_for_focused_surface(
        seat: *const sway_seat,
    ) -> *mut sway_keyboard_shortcuts_inhibitor;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn wlr_input_device_get_virtual_keyboard(
        wlr_dev: *mut wlr_input_device,
    ) -> *mut wlr_virtual_keyboard_v1;
    fn wlr_input_method_keyboard_grab_v2_send_key(
        keyboard_grab: *mut wlr_input_method_keyboard_grab_v2,
        time: uint32_t,
        key: uint32_t,
        state: uint32_t,
    );
    fn wlr_input_method_keyboard_grab_v2_send_modifiers(
        keyboard_grab: *mut wlr_input_method_keyboard_grab_v2,
        modifiers_0: *mut wlr_keyboard_modifiers,
    );
    fn wlr_input_method_keyboard_grab_v2_set_keyboard(
        keyboard_grab: *mut wlr_input_method_keyboard_grab_v2,
        keyboard: *mut wlr_keyboard,
    );
    fn input_config_fill_rule_names(ic: *mut input_config, rules: *mut xkb_rule_names);
    fn seat_execute_command(seat: *mut sway_seat, binding: *mut sway_binding);
    static mut config: *mut sway_config;
    static mut server: sway_server;
    fn input_device_get_config(device: *mut sway_input_device) -> *mut input_config;
    fn input_device_get_identifier(
        device: *mut wlr_input_device,
    ) -> *mut ::core::ffi::c_char;
    fn ipc_event_bar_state_update(bar: *mut bar_config);
    fn ipc_event_input(
        change: *const ::core::ffi::c_char,
        device: *mut sway_input_device,
    );
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut ::core::ffi::c_void,
    pub __gr_top: *mut ::core::ffi::c_void,
    pub __vr_top: *mut ::core::ffi::c_void,
    pub __gr_offs: ::core::ffi::c_int,
    pub __vr_offs: ::core::ffi::c_int,
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type va_list = __builtin_va_list;
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
pub struct wl_keyboard_key_state(pub ::core::ffi::c_uint);
impl wl_keyboard_key_state {
    pub const WL_KEYBOARD_KEY_STATE_RELEASED: Self = Self(0);
    pub const WL_KEYBOARD_KEY_STATE_PRESSED: Self = Self(1);
    pub const WL_KEYBOARD_KEY_STATE_REPEATED: Self = Self(2);
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
    pub WLR_PRIVATE: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
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
    pub events: C2Rust_Unnamed_3,
    pub features: C2Rust_Unnamed_2,
    pub WLR_PRIVATE: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub r#impl: *const wlr_renderer_impl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub input_color_transform: bool,
    pub output_color_transform: bool,
    pub timeline: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
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
    pub events: C2Rust_Unnamed_4,
    pub addons: wlr_addon_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub destroy: wl_signal,
    pub release: wl_signal,
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
    pub features: C2Rust_Unnamed_6,
    pub events: C2Rust_Unnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
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
    pub events: C2Rust_Unnamed_11,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
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
    pub mastering_luminance: C2Rust_Unnamed_10,
    pub max_cll: ::core::ffi::c_double,
    pub max_fall: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub min: ::core::ffi::c_double,
    pub max: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
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
    pub events: C2Rust_Unnamed_14,
    pub WLR_PRIVATE: C2Rust_Unnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub event_loop_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub active: wl_signal,
    pub add_drm_card: wl_signal,
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
pub struct wlr_input_device {
    pub r#type: wlr_input_device_type,
    pub name: *mut ::core::ffi::c_char,
    pub events: C2Rust_Unnamed_15,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type xkb_keycode_t = uint32_t;
pub type xkb_keysym_t = uint32_t;
pub type xkb_layout_index_t = uint32_t;
pub type xkb_level_index_t = uint32_t;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_rule_names {
    pub rules: *const ::core::ffi::c_char,
    pub model: *const ::core::ffi::c_char,
    pub layout: *const ::core::ffi::c_char,
    pub variant: *const ::core::ffi::c_char,
    pub options: *const ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xkb_context_flags(pub ::core::ffi::c_uint);
impl xkb_context_flags {
    pub const XKB_CONTEXT_NO_FLAGS: Self = Self(0);
    pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: Self = Self(1);
    pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: Self = Self(2);
    pub const XKB_CONTEXT_NO_SECURE_GETENV: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xkb_log_level(pub ::core::ffi::c_uint);
impl xkb_log_level {
    pub const XKB_LOG_LEVEL_CRITICAL: Self = Self(10);
    pub const XKB_LOG_LEVEL_ERROR: Self = Self(20);
    pub const XKB_LOG_LEVEL_WARNING: Self = Self(30);
    pub const XKB_LOG_LEVEL_INFO: Self = Self(40);
    pub const XKB_LOG_LEVEL_DEBUG: Self = Self(50);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xkb_keymap_compile_flags(pub ::core::ffi::c_uint);
impl xkb_keymap_compile_flags {
    pub const XKB_KEYMAP_COMPILE_NO_FLAGS: Self = Self(0);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xkb_keymap_format(pub ::core::ffi::c_uint);
impl xkb_keymap_format {
    pub const XKB_KEYMAP_FORMAT_TEXT_V1: Self = Self(1);
    pub const XKB_KEYMAP_FORMAT_TEXT_V2: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xkb_consumed_mode(pub ::core::ffi::c_uint);
impl xkb_consumed_mode {
    pub const XKB_CONSUMED_MODE_XKB: Self = Self(0);
    pub const XKB_CONSUMED_MODE_GTK: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_keyboard_modifier(pub ::core::ffi::c_uint);
impl wlr_keyboard_modifier {
    pub const WLR_MODIFIER_SHIFT: Self = Self(1);
    pub const WLR_MODIFIER_CAPS: Self = Self(2);
    pub const WLR_MODIFIER_CTRL: Self = Self(4);
    pub const WLR_MODIFIER_ALT: Self = Self(8);
    pub const WLR_MODIFIER_MOD2: Self = Self(16);
    pub const WLR_MODIFIER_MOD3: Self = Self(32);
    pub const WLR_MODIFIER_LOGO: Self = Self(64);
    pub const WLR_MODIFIER_MOD5: Self = Self(128);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_impl {
    pub name: *const ::core::ffi::c_char,
    pub led_update: Option<unsafe extern "C" fn(*mut wlr_keyboard, uint32_t) -> ()>,
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
    pub repeat_info: C2Rust_Unnamed_17,
    pub events: C2Rust_Unnamed_16,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub rate: int32_t,
    pub delay: int32_t,
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
pub struct wlr_keyboard_group {
    pub keyboard: wlr_keyboard,
    pub devices: wl_list,
    pub keys: wl_list,
    pub events: C2Rust_Unnamed_18,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub enter: wl_signal,
    pub leave: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_keyboard_key_event {
    pub time_msec: uint32_t,
    pub keycode: uint32_t,
    pub update_state: bool,
    pub state: wl_keyboard_key_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_output_layout {
    pub outputs: wl_list,
    pub display: *mut wl_display,
    pub events: C2Rust_Unnamed_20,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub add: wl_signal,
    pub change: wl_signal,
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
    pub events: C2Rust_Unnamed_23,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_22,
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
pub struct C2Rust_Unnamed_22 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
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
    pub viewport: C2Rust_Unnamed_24,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
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
    pub events: C2Rust_Unnamed_26,
    pub WLR_PRIVATE: C2Rust_Unnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_cursor {
    pub state: *mut wlr_cursor_state,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
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
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_29,
    pub WLR_PRIVATE: C2Rust_Unnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
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
pub struct C2Rust_Unnamed_29 {
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
    pub events: C2Rust_Unnamed_31,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_32,
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
pub struct C2Rust_Unnamed_32 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_34,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
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
    pub events: C2Rust_Unnamed_36,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_37 {
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
pub struct wlr_scene {
    pub tree: wlr_scene_tree,
    pub outputs: wl_list,
    pub linux_dmabuf_v1: *mut wlr_linux_dmabuf_v1,
    pub gamma_control_manager_v1: *mut wlr_gamma_control_manager_v1,
    pub color_manager_v1: *mut wlr_color_manager_v1,
    pub restack_xwayland_surfaces: bool,
    pub WLR_PRIVATE: C2Rust_Unnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
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
    pub layers: C2Rust_Unnamed_45,
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
    pub events: C2Rust_Unnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_42,
    pub border: C2Rust_Unnamed_41,
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
    pub events: C2Rust_Unnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_40 {
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
pub struct C2Rust_Unnamed_41 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
    pub tree: *mut wlr_scene_tree,
    pub border: *mut wlr_scene_tree,
    pub background: *mut wlr_scene_tree,
    pub title_text: *mut sway_text_node,
    pub marks_text: *mut sway_text_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_node {
    pub r#type: sway_node_type,
    pub c2rust_unnamed: C2Rust_Unnamed_44,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_43,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_44 {
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
pub struct C2Rust_Unnamed_45 {
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
    pub events: C2Rust_Unnamed_47,
    pub WLR_PRIVATE: C2Rust_Unnamed_46,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
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
    pub events: C2Rust_Unnamed_49,
    pub WLR_PRIVATE: C2Rust_Unnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
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
pub struct wlr_seat_client {
    pub client: *mut wl_client,
    pub seat: *mut wlr_seat,
    pub link: wl_list,
    pub resources: wl_list,
    pub pointers: wl_list,
    pub keyboards: wl_list,
    pub touches: wl_list,
    pub data_devices: wl_list,
    pub events: C2Rust_Unnamed_51,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_50,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
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
pub struct C2Rust_Unnamed_51 {
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
    pub events: C2Rust_Unnamed_53,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_52,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_52 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_53 {
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
    pub events: C2Rust_Unnamed_55,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_54,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_54 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
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
    pub events: C2Rust_Unnamed_57,
    pub WLR_PRIVATE: C2Rust_Unnamed_56,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_56 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_57 {
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
    pub events: C2Rust_Unnamed_59,
    pub WLR_PRIVATE: C2Rust_Unnamed_58,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_58 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_59 {
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
    pub events: C2Rust_Unnamed_60,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_60 {
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
    pub events: C2Rust_Unnamed_61,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_61 {
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
    pub previous: C2Rust_Unnamed_67,
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
    pub events: C2Rust_Unnamed_63,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_62,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub synced: wlr_surface_synced,
    pub destroying: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
    pub set_region: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_constraint_v1_state {
    pub committed: uint32_t,
    pub region: pixman_region32_t,
    pub cursor_hint: C2Rust_Unnamed_64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
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
pub struct wlr_pointer_constraints_v1 {
    pub global: *mut wl_global,
    pub constraints: wl_list,
    pub events: C2Rust_Unnamed_66,
    pub data: *mut ::core::ffi::c_void,
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
    pub new_constraint: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub node: *mut sway_node,
}
pub type PangoFontDescription = _PangoFontDescription;
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
pub struct seat_config_shortcuts_inhibit(pub ::core::ffi::c_uint);
impl seat_config_shortcuts_inhibit {
    pub const SHORTCUTS_INHIBIT_DEFAULT: Self = Self(0);
    pub const SHORTCUTS_INHIBIT_ENABLE: Self = Self(1);
    pub const SHORTCUTS_INHIBIT_DISABLE: Self = Self(2);
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
    pub xcursor_theme: C2Rust_Unnamed_68,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
    pub name: *mut ::core::ffi::c_char,
    pub size: ::core::ffi::c_int,
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
    pub colors: C2Rust_Unnamed_69,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
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
    pub border_colors: C2Rust_Unnamed_72,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_70,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_71,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_71 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
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
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
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
pub struct wlr_keyboard_shortcuts_inhibitor_v1 {
    pub surface: *mut wlr_surface,
    pub seat: *mut wlr_seat,
    pub active: bool,
    pub resource: *mut wl_resource,
    pub link: wl_list,
    pub events: C2Rust_Unnamed_76,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_75,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_75 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_76 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_keyboards: wl_list,
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
    pub new_virtual_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_v1 {
    pub keyboard: wlr_keyboard,
    pub resource: *mut wl_resource,
    pub seat: *mut wlr_seat,
    pub has_keymap: bool,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_79,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_79 {
    pub seat_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_pointers: wl_list,
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
    pub new_virtual_pointer: wl_signal,
    pub destroy: wl_signal,
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
    pub session_lock: C2Rust_Unnamed_88,
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
pub struct wlr_text_input_manager_v3 {
    pub global: *mut wl_global,
    pub text_inputs: wl_list,
    pub events: C2Rust_Unnamed_85,
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
    pub new_text_input: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_manager_v2 {
    pub global: *mut wl_global,
    pub input_methods: wl_list,
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
    pub new_input_method: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_88 {
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
    pub events: C2Rust_Unnamed_91,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_89,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_89 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_90,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_90 {
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
pub struct C2Rust_Unnamed_91 {
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
    pub events: C2Rust_Unnamed_93,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_92,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_92 {
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_93 {
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
pub struct wlr_pointer_gestures_v1 {
    pub global: *mut wl_global,
    pub swipes: wl_list,
    pub pinches: wl_list,
    pub holds: wl_list,
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
    pub destroy: wl_signal,
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
pub struct modifier_key {
    pub name: *mut ::core::ffi::c_char,
    pub r#mod: uint32_t,
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
pub struct key_info {
    pub keycode: uint32_t,
    pub code_modifiers: uint32_t,
    pub raw_keysyms: *const xkb_keysym_t,
    pub raw_modifiers: uint32_t,
    pub raw_keysyms_len: size_t,
    pub translated_keysyms: *const xkb_keysym_t,
    pub translated_modifiers: uint32_t,
    pub translated_keysyms_len: size_t,
}
pub const XKB_MOD_NAME_SHIFT: [::core::ffi::c_char; 6] = unsafe {
    ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"Shift\0")
};
pub const XKB_MOD_NAME_CAPS: [::core::ffi::c_char; 5] = unsafe {
    ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Lock\0")
};
pub const XKB_MOD_NAME_CTRL: [::core::ffi::c_char; 8] = unsafe {
    ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"Control\0")
};
pub const XKB_MOD_NAME_ALT: [::core::ffi::c_char; 5] = unsafe {
    ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Mod1\0")
};
pub const XKB_MOD_NAME_LOGO: [::core::ffi::c_char; 5] = unsafe {
    ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Mod4\0")
};
pub const XKB_MOD_NAME_NUM: [::core::ffi::c_char; 5] = unsafe {
    ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Mod2\0")
};
pub const XKB_KEY_XF86Switch_VT_1: ::core::ffi::c_int = 0x1008fe01 as ::core::ffi::c_int;
pub const XKB_KEY_XF86Switch_VT_12: ::core::ffi::c_int = 0x1008fe0c
    as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_signal_add(
    mut signal: *mut wl_signal,
    mut listener: *mut wl_listener,
) {
    wl_list_insert((*signal).listener_list.prev, &raw mut (*listener).link);
}
pub const XKB_LAYOUT_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const XKB_MOD_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const WLR_LED_COUNT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SWAY_KEYBOARD_PRESSED_KEYS_CAP: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
static mut modifiers: [modifier_key; 11] = [
    modifier_key {
        name: XKB_MOD_NAME_SHIFT.as_ptr() as *mut ::core::ffi::c_char,
        r#mod: wlr_keyboard_modifier::WLR_MODIFIER_SHIFT.0 as ::core::ffi::c_int
            as uint32_t,
    },
    modifier_key {
        name: XKB_MOD_NAME_CAPS.as_ptr() as *mut ::core::ffi::c_char,
        r#mod: wlr_keyboard_modifier::WLR_MODIFIER_CAPS.0 as ::core::ffi::c_int
            as uint32_t,
    },
    modifier_key {
        name: XKB_MOD_NAME_CTRL.as_ptr() as *mut ::core::ffi::c_char,
        r#mod: wlr_keyboard_modifier::WLR_MODIFIER_CTRL.0 as ::core::ffi::c_int
            as uint32_t,
    },
    modifier_key {
        name: b"Ctrl\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        r#mod: wlr_keyboard_modifier::WLR_MODIFIER_CTRL.0 as ::core::ffi::c_int
            as uint32_t,
    },
    modifier_key {
        name: XKB_MOD_NAME_ALT.as_ptr() as *mut ::core::ffi::c_char,
        r#mod: wlr_keyboard_modifier::WLR_MODIFIER_ALT.0 as ::core::ffi::c_int
            as uint32_t,
    },
    modifier_key {
        name: b"Alt\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        r#mod: wlr_keyboard_modifier::WLR_MODIFIER_ALT.0 as ::core::ffi::c_int
            as uint32_t,
    },
    modifier_key {
        name: XKB_MOD_NAME_NUM.as_ptr() as *mut ::core::ffi::c_char,
        r#mod: wlr_keyboard_modifier::WLR_MODIFIER_MOD2.0 as ::core::ffi::c_int
            as uint32_t,
    },
    modifier_key {
        name: b"Mod3\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        r#mod: wlr_keyboard_modifier::WLR_MODIFIER_MOD3.0 as ::core::ffi::c_int
            as uint32_t,
    },
    modifier_key {
        name: XKB_MOD_NAME_LOGO.as_ptr() as *mut ::core::ffi::c_char,
        r#mod: wlr_keyboard_modifier::WLR_MODIFIER_LOGO.0 as ::core::ffi::c_int
            as uint32_t,
    },
    modifier_key {
        name: b"Super\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        r#mod: wlr_keyboard_modifier::WLR_MODIFIER_LOGO.0 as ::core::ffi::c_int
            as uint32_t,
    },
    modifier_key {
        name: b"Mod5\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        r#mod: wlr_keyboard_modifier::WLR_MODIFIER_MOD5.0 as ::core::ffi::c_int
            as uint32_t,
    },
];
#[no_mangle]
pub unsafe extern "C" fn get_modifier_mask_by_name(
    mut name: *const ::core::ffi::c_char,
) -> uint32_t {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        < ::core::mem::size_of::<[modifier_key; 11]>()
            .wrapping_div(::core::mem::size_of::<modifier_key>()) as ::core::ffi::c_int
    {
        if strcasecmp(modifiers[i as usize].name, name) == 0 as ::core::ffi::c_int {
            return modifiers[i as usize].r#mod;
        }
        i += 1;
    }
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn get_modifier_name_by_mask(
    mut modifier: uint32_t,
) -> *const ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        < ::core::mem::size_of::<[modifier_key; 11]>()
            .wrapping_div(::core::mem::size_of::<modifier_key>()) as ::core::ffi::c_int
    {
        if modifiers[i as usize].r#mod == modifier {
            return modifiers[i as usize].name;
        }
        i += 1;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn get_modifier_names(
    mut names: *mut *const ::core::ffi::c_char,
    mut modifier_masks: uint32_t,
) -> ::core::ffi::c_int {
    let mut length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        < ::core::mem::size_of::<[modifier_key; 11]>()
            .wrapping_div(::core::mem::size_of::<modifier_key>()) as ::core::ffi::c_int
    {
        if modifier_masks & modifiers[i as usize].r#mod != 0 as uint32_t {
            *names.offset(length as isize) = modifiers[i as usize].name;
            length += 1;
            modifier_masks ^= modifiers[i as usize].r#mod;
        }
        i += 1;
    }
    return length;
}
unsafe extern "C" fn state_erase_key(
    mut state: *mut sway_shortcut_state,
    mut keycode: uint32_t,
) -> bool {
    let mut found: bool = r#false != 0;
    let mut j: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < (*state).npressed {
        if i > j {
            (*state).pressed_keys[j] = (*state).pressed_keys[i];
            (*state).pressed_keycodes[j] = (*state).pressed_keycodes[i];
        }
        if (*state).pressed_keycodes[i] != keycode {
            j = j.wrapping_add(1);
        } else {
            found = r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    while (*state).npressed > j {
        (*state).npressed = (*state).npressed.wrapping_sub(1);
        (*state).pressed_keys[(*state).npressed] = 0 as uint32_t;
        (*state).pressed_keycodes[(*state).npressed] = 0 as uint32_t;
    }
    (*state).current_key = 0 as uint32_t;
    return found;
}
unsafe extern "C" fn state_add_key(
    mut state: *mut sway_shortcut_state,
    mut keycode: uint32_t,
    mut key_id: uint32_t,
) {
    if (*state).npressed >= SWAY_KEYBOARD_PRESSED_KEYS_CAP as size_t {
        return;
    }
    let mut i: size_t = 0 as size_t;
    while i < (*state).npressed && (*state).pressed_keys[i] < key_id {
        i = i.wrapping_add(1);
    }
    let mut j: size_t = (*state).npressed;
    while j > i {
        (*state).pressed_keys[j] = (*state).pressed_keys[j.wrapping_sub(1 as size_t)];
        (*state).pressed_keycodes[j] = (*state)
            .pressed_keycodes[j.wrapping_sub(1 as size_t)];
        j = j.wrapping_sub(1);
    }
    (*state).pressed_keys[i] = key_id;
    (*state).pressed_keycodes[i] = keycode;
    (*state).npressed = (*state).npressed.wrapping_add(1);
    (*state).current_key = key_id;
}
unsafe extern "C" fn update_shortcut_state(
    mut state: *mut sway_shortcut_state,
    mut keycode: uint32_t,
    mut keystate: wl_keyboard_key_state,
    mut new_key: uint32_t,
    mut raw_modifiers: uint32_t,
) -> bool {
    let mut last_key_was_a_modifier: bool = raw_modifiers != (*state).last_raw_modifiers;
    (*state).last_raw_modifiers = raw_modifiers;
    if last_key_was_a_modifier as ::core::ffi::c_int != 0 && (*state).last_keycode != 0 {
        state_erase_key(state, (*state).last_keycode);
    }
    if keystate.0 == wl_keyboard_key_state::WL_KEYBOARD_KEY_STATE_PRESSED.0 {
        state_add_key(state, keycode, new_key);
        (*state).last_keycode = keycode;
    } else {
        return state_erase_key(state, keycode)
    }
    return r#false != 0;
}
unsafe extern "C" fn get_active_binding(
    mut state: *const sway_shortcut_state,
    mut bindings: *mut list_t,
    mut current_binding: *mut *mut sway_binding,
    mut modifiers_0: uint32_t,
    mut release: bool,
    mut locked: bool,
    mut inhibited: bool,
    mut input: *const ::core::ffi::c_char,
    mut exact_input: bool,
    mut group: xkb_layout_index_t,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*bindings).length {
        let mut binding: *mut sway_binding = *(*bindings).items.offset(i as isize)
            as *mut sway_binding;
        let mut binding_locked: bool = (*binding).flags
            & binding_flags::BINDING_LOCKED.0 as ::core::ffi::c_int as uint32_t
            != 0 as uint32_t;
        let mut binding_inhibited: bool = (*binding).flags
            & binding_flags::BINDING_INHIBITED.0 as ::core::ffi::c_int as uint32_t
            != 0 as uint32_t;
        let mut binding_release: bool = (*binding).flags
            & binding_flags::BINDING_RELEASE.0 as ::core::ffi::c_int as uint32_t != 0;
        's_4: {
            if !(modifiers_0 ^ (*binding).modifiers != 0
                || release as ::core::ffi::c_int != binding_release as ::core::ffi::c_int
                || locked as ::core::ffi::c_int > binding_locked as ::core::ffi::c_int
                || inhibited as ::core::ffi::c_int
                    > binding_inhibited as ::core::ffi::c_int
                || (*binding).group != XKB_LAYOUT_INVALID as xkb_layout_index_t
                    && (*binding).group != group
                || strcmp((*binding).input, input) != 0 as ::core::ffi::c_int
                    && (strcmp(
                        (*binding).input,
                        b"*\0".as_ptr() as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                        || exact_input as ::core::ffi::c_int != 0))
            {
                let mut r#match: bool = r#false != 0;
                if (*state).npressed == (*(*binding).keys).length as size_t {
                    r#match = r#true != 0;
                    let mut j: size_t = 0 as size_t;
                    while j < (*state).npressed {
                        let mut key: uint32_t = *(*(*(*binding).keys)
                            .items
                            .offset(j as isize) as *mut uint32_t);
                        if key != (*state).pressed_keys[j] {
                            r#match = r#false != 0;
                            break;
                        } else {
                            j = j.wrapping_add(1);
                        }
                    }
                } else if (*(*binding).keys).length == 1 as ::core::ffi::c_int {
                    r#match = (*state).current_key
                        == *(*(*(*binding).keys).items.offset(0isize) as *mut uint32_t);
                }
                if r#match {
                    if !(*current_binding).is_null() {
                        if *current_binding == binding {
                            break 's_4;
                        } else {
                            let mut current_locked: bool = (**current_binding).flags
                                & binding_flags::BINDING_LOCKED.0 as ::core::ffi::c_int
                                    as uint32_t != 0 as uint32_t;
                            let mut current_inhibited: bool = (**current_binding).flags
                                & binding_flags::BINDING_INHIBITED.0 as ::core::ffi::c_int
                                    as uint32_t != 0 as uint32_t;
                            let mut current_input: bool = strcmp(
                                (**current_binding).input,
                                input,
                            ) == 0 as ::core::ffi::c_int;
                            let mut current_group_set: bool = (**current_binding).group
                                != XKB_LAYOUT_INVALID as xkb_layout_index_t;
                            let mut binding_input: bool = strcmp((*binding).input, input)
                                == 0 as ::core::ffi::c_int;
                            let mut binding_group_set: bool = (*binding).group
                                != XKB_LAYOUT_INVALID as xkb_layout_index_t;
                            if current_input as ::core::ffi::c_int
                                == binding_input as ::core::ffi::c_int
                                && current_locked as ::core::ffi::c_int
                                    == binding_locked as ::core::ffi::c_int
                                && current_inhibited as ::core::ffi::c_int
                                    == binding_inhibited as ::core::ffi::c_int
                                && current_group_set as ::core::ffi::c_int
                                    == binding_group_set as ::core::ffi::c_int
                            {
                                _sway_log(
                                    sway_log_importance_t::SWAY_DEBUG,
                                    b"[%s:%d] Encountered conflicting bindings %d and %d\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    b"sway/input/keyboard.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    219 as ::core::ffi::c_int,
                                    (**current_binding).order,
                                    (*binding).order,
                                );
                                break 's_4;
                            } else if current_input as ::core::ffi::c_int != 0
                                && !binding_input
                            {
                                break 's_4;
                            } else if current_input as ::core::ffi::c_int
                                == binding_input as ::core::ffi::c_int
                                && (**current_binding).group == group
                            {
                                break 's_4;
                            } else if current_input as ::core::ffi::c_int
                                == binding_input as ::core::ffi::c_int
                                && current_group_set as ::core::ffi::c_int
                                    == binding_group_set as ::core::ffi::c_int
                                && current_locked as ::core::ffi::c_int
                                    == locked as ::core::ffi::c_int
                            {
                                break 's_4;
                            } else if current_input as ::core::ffi::c_int
                                == binding_input as ::core::ffi::c_int
                                && current_group_set as ::core::ffi::c_int
                                    == binding_group_set as ::core::ffi::c_int
                                && current_locked as ::core::ffi::c_int
                                    == binding_locked as ::core::ffi::c_int
                                && current_inhibited as ::core::ffi::c_int
                                    == inhibited as ::core::ffi::c_int
                            {
                                break 's_4;
                            }
                        }
                    }
                    *current_binding = binding;
                    if strcmp((**current_binding).input, input)
                        == 0 as ::core::ffi::c_int
                        && (**current_binding).flags
                            & binding_flags::BINDING_LOCKED.0 as ::core::ffi::c_int
                                as uint32_t == locked as uint32_t
                        && (**current_binding).flags
                            & binding_flags::BINDING_INHIBITED.0 as ::core::ffi::c_int
                                as uint32_t == inhibited as uint32_t
                        && (**current_binding).group == group
                    {
                        return;
                    }
                }
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn keyboard_execute_compositor_binding(
    mut keyboard: *mut sway_keyboard,
    mut pressed_keysyms: *const xkb_keysym_t,
    mut modifiers_0: uint32_t,
    mut keysyms_len: size_t,
) -> bool {
    let mut i: size_t = 0 as size_t;
    while i < keysyms_len {
        let mut keysym: xkb_keysym_t = *pressed_keysyms.offset(i as isize);
        if keysym >= XKB_KEY_XF86Switch_VT_1 as xkb_keysym_t
            && keysym <= XKB_KEY_XF86Switch_VT_12 as xkb_keysym_t
        {
            if !server.session.is_null() {
                let mut vt: ::core::ffi::c_uint = (keysym as ::core::ffi::c_uint)
                    .wrapping_sub(XKB_KEY_XF86Switch_VT_1 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint);
                wlr_session_change_vt(server.session, vt);
            }
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn keyboard_execute_pointer_keysyms(
    mut keyboard: *mut sway_keyboard,
    mut time: uint32_t,
    mut pressed_keysyms: *const xkb_keysym_t,
    mut keysyms_len: size_t,
    mut state: wl_keyboard_key_state,
) -> bool {
    let mut cursor: *mut sway_cursor = (*(*(*keyboard).seat_device).sway_seat).cursor;
    let mut i: size_t = 0 as size_t;
    while i < keysyms_len {
        let mut keysym: xkb_keysym_t = *pressed_keysyms.offset(i as isize);
        let mut button: uint32_t = wlr_keyboard_keysym_to_pointer_button(keysym);
        if button != 0 as uint32_t {
            dispatch_cursor_button(
                cursor,
                &raw mut (*(*keyboard).wlr).base,
                time,
                button,
                wl_pointer_button_state(state.0),
            );
            wlr_seat_pointer_notify_frame((*(*cursor).seat).wlr_seat);
            return r#true != 0;
        }
        let mut dx: ::core::ffi::c_int = 0;
        let mut dy: ::core::ffi::c_int = 0;
        wlr_keyboard_keysym_to_pointer_motion(keysym, &raw mut dx, &raw mut dy);
        if state.0 == wl_keyboard_key_state::WL_KEYBOARD_KEY_STATE_PRESSED.0
            && (dx != 0 as ::core::ffi::c_int || dy != 0 as ::core::ffi::c_int)
        {
            dx *= 10 as ::core::ffi::c_int;
            dy *= 10 as ::core::ffi::c_int;
            pointer_motion(
                cursor,
                time,
                &raw mut (*(*keyboard).wlr).base,
                dx as ::core::ffi::c_double,
                dy as ::core::ffi::c_double,
                dx as ::core::ffi::c_double,
                dy as ::core::ffi::c_double,
            );
            wlr_seat_pointer_notify_frame((*(*cursor).seat).wlr_seat);
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn keyboard_keysyms_translated(
    mut keyboard: *mut sway_keyboard,
    mut keycode: xkb_keycode_t,
    mut keysyms: *mut *const xkb_keysym_t,
    mut modifiers_0: *mut uint32_t,
) -> size_t {
    *modifiers_0 = wlr_keyboard_get_modifiers((*keyboard).wlr);
    let mut consumed: xkb_mod_mask_t = xkb_state_key_get_consumed_mods2(
        (*(*keyboard).wlr).xkb_state,
        keycode,
        xkb_consumed_mode::XKB_CONSUMED_MODE_XKB,
    );
    *modifiers_0 = *modifiers_0 & !consumed;
    return xkb_state_key_get_syms((*(*keyboard).wlr).xkb_state, keycode, keysyms)
        as size_t;
}
unsafe extern "C" fn keyboard_keysyms_raw(
    mut keyboard: *mut sway_keyboard,
    mut keycode: xkb_keycode_t,
    mut keysyms: *mut *const xkb_keysym_t,
    mut modifiers_0: *mut uint32_t,
) -> size_t {
    *modifiers_0 = wlr_keyboard_get_modifiers((*keyboard).wlr);
    let mut layout_index: xkb_layout_index_t = xkb_state_key_get_layout(
        (*(*keyboard).wlr).xkb_state,
        keycode,
    );
    return xkb_keymap_key_get_syms_by_level(
        (*(*keyboard).wlr).keymap,
        keycode,
        layout_index,
        0 as xkb_level_index_t,
        keysyms,
    ) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn sway_keyboard_disarm_key_repeat(
    mut keyboard: *mut sway_keyboard,
) {
    if keyboard.is_null() {
        return;
    }
    (*keyboard).repeat_binding = ::core::ptr::null_mut::<sway_binding>();
    if wl_event_source_timer_update(
        (*keyboard).key_repeat_source,
        0 as ::core::ffi::c_int,
    ) < 0 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] failed to disarm key repeat timer\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
            364 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn update_keyboard_state(
    mut keyboard: *mut sway_keyboard,
    mut raw_keycode: uint32_t,
    mut keystate: wl_keyboard_key_state,
    mut keyinfo: *mut key_info,
) {
    (*keyinfo).keycode = raw_keycode.wrapping_add(8 as uint32_t);
    (*keyinfo).raw_keysyms_len = keyboard_keysyms_raw(
        keyboard,
        (*keyinfo).keycode,
        &raw mut (*keyinfo).raw_keysyms,
        &raw mut (*keyinfo).raw_modifiers,
    );
    (*keyinfo).translated_keysyms_len = keyboard_keysyms_translated(
        keyboard,
        (*keyinfo).keycode,
        &raw mut (*keyinfo).translated_keysyms,
        &raw mut (*keyinfo).translated_modifiers,
    );
    (*keyinfo).code_modifiers = wlr_keyboard_get_modifiers((*keyboard).wlr);
    update_shortcut_state(
        &raw mut (*keyboard).state_keycodes,
        raw_keycode,
        keystate,
        (*keyinfo).keycode,
        (*keyinfo).code_modifiers,
    );
    let mut i: size_t = 0 as size_t;
    while i < (*keyinfo).raw_keysyms_len {
        update_shortcut_state(
            &raw mut (*keyboard).state_keysyms_raw,
            raw_keycode,
            keystate,
            *(*keyinfo).raw_keysyms.offset(i as isize),
            (*keyinfo).code_modifiers,
        );
        i = i.wrapping_add(1);
    }
    let mut i_0: size_t = 0 as size_t;
    while i_0 < (*keyinfo).translated_keysyms_len {
        update_shortcut_state(
            &raw mut (*keyboard).state_keysyms_translated,
            raw_keycode,
            keystate,
            *(*keyinfo).translated_keysyms.offset(i_0 as isize),
            (*keyinfo).code_modifiers,
        );
        i_0 = i_0.wrapping_add(1);
    }
}
unsafe extern "C" fn keyboard_get_im_grab(
    mut keyboard: *mut sway_keyboard,
) -> *mut wlr_input_method_keyboard_grab_v2 {
    let mut input_method: *mut wlr_input_method_v2 = (*(*(*keyboard).seat_device)
        .sway_seat)
        .im_relay
        .input_method;
    let mut virtual_keyboard: *mut wlr_virtual_keyboard_v1 = wlr_input_device_get_virtual_keyboard(
        (*(*(*keyboard).seat_device).input_device).wlr_device,
    );
    if input_method.is_null() || (*input_method).keyboard_grab.is_null()
        || !virtual_keyboard.is_null()
            && wl_resource_get_client((*virtual_keyboard).resource)
                == wl_resource_get_client((*(*input_method).keyboard_grab).resource)
    {
        return ::core::ptr::null_mut::<wlr_input_method_keyboard_grab_v2>();
    }
    return (*input_method).keyboard_grab;
}
unsafe extern "C" fn handle_key_event(
    mut keyboard: *mut sway_keyboard,
    mut event: *mut wlr_keyboard_key_event,
) {
    let mut seat: *mut sway_seat = (*(*keyboard).seat_device).sway_seat;
    let mut wlr_seat: *mut wlr_seat = (*seat).wlr_seat;
    let mut wlr_device: *mut wlr_input_device = (*(*(*keyboard).seat_device)
        .input_device)
        .wlr_device;
    let mut device_identifier: *mut ::core::ffi::c_char = input_device_get_identifier(
        wlr_device,
    );
    let mut exact_identifier: bool = !(*(*keyboard).wlr).group.is_null();
    seat_idle_notify_activity(seat, sway_input_idle_source::IDLE_SOURCE_KEYBOARD);
    let mut locked: bool = !server.session_lock.lock.is_null();
    let mut sway_inhibitor: *mut sway_keyboard_shortcuts_inhibitor = keyboard_shortcuts_inhibitor_get_for_focused_surface(
        seat,
    );
    let mut shortcuts_inhibited: bool = !sway_inhibitor.is_null()
        && (*(*sway_inhibitor).inhibitor).active as ::core::ffi::c_int != 0;
    if (*event).state.0 == wl_keyboard_key_state::WL_KEYBOARD_KEY_STATE_PRESSED.0 {
        cursor_notify_key_press((*seat).cursor);
    }
    let mut keyinfo: key_info = key_info {
        keycode: 0,
        code_modifiers: 0,
        raw_keysyms: ::core::ptr::null::<xkb_keysym_t>(),
        raw_modifiers: 0,
        raw_keysyms_len: 0,
        translated_keysyms: ::core::ptr::null::<xkb_keysym_t>(),
        translated_modifiers: 0,
        translated_keysyms_len: 0,
    };
    update_keyboard_state(keyboard, (*event).keycode, (*event).state, &raw mut keyinfo);
    let mut handled: bool = r#false != 0;
    let mut binding_released: *mut sway_binding = ::core::ptr::null_mut::<
        sway_binding,
    >();
    get_active_binding(
        &raw mut (*keyboard).state_keycodes,
        (*(*config).current_mode).keycode_bindings,
        &raw mut binding_released,
        keyinfo.code_modifiers,
        r#true != 0,
        locked,
        shortcuts_inhibited,
        device_identifier,
        exact_identifier,
        (*keyboard).effective_layout,
    );
    get_active_binding(
        &raw mut (*keyboard).state_keysyms_raw,
        (*(*config).current_mode).keysym_bindings,
        &raw mut binding_released,
        keyinfo.raw_modifiers,
        r#true != 0,
        locked,
        shortcuts_inhibited,
        device_identifier,
        exact_identifier,
        (*keyboard).effective_layout,
    );
    get_active_binding(
        &raw mut (*keyboard).state_keysyms_translated,
        (*(*config).current_mode).keysym_bindings,
        &raw mut binding_released,
        keyinfo.translated_modifiers,
        r#true != 0,
        locked,
        shortcuts_inhibited,
        device_identifier,
        exact_identifier,
        (*keyboard).effective_layout,
    );
    if !(*keyboard).held_binding.is_null()
        && binding_released != (*keyboard).held_binding
        && (*event).state.0 == wl_keyboard_key_state::WL_KEYBOARD_KEY_STATE_RELEASED.0
    {
        seat_execute_command(seat, (*keyboard).held_binding);
        handled = r#true != 0;
    }
    if binding_released != (*keyboard).held_binding {
        (*keyboard).held_binding = ::core::ptr::null_mut::<sway_binding>();
    }
    if !binding_released.is_null()
        && (*event).state.0 == wl_keyboard_key_state::WL_KEYBOARD_KEY_STATE_PRESSED.0
    {
        (*keyboard).held_binding = binding_released;
    }
    let mut binding: *mut sway_binding = ::core::ptr::null_mut::<sway_binding>();
    if (*event).state.0 == wl_keyboard_key_state::WL_KEYBOARD_KEY_STATE_PRESSED.0 {
        get_active_binding(
            &raw mut (*keyboard).state_keycodes,
            (*(*config).current_mode).keycode_bindings,
            &raw mut binding,
            keyinfo.code_modifiers,
            r#false != 0,
            locked,
            shortcuts_inhibited,
            device_identifier,
            exact_identifier,
            (*keyboard).effective_layout,
        );
        get_active_binding(
            &raw mut (*keyboard).state_keysyms_raw,
            (*(*config).current_mode).keysym_bindings,
            &raw mut binding,
            keyinfo.raw_modifiers,
            r#false != 0,
            locked,
            shortcuts_inhibited,
            device_identifier,
            exact_identifier,
            (*keyboard).effective_layout,
        );
        get_active_binding(
            &raw mut (*keyboard).state_keysyms_translated,
            (*(*config).current_mode).keysym_bindings,
            &raw mut binding,
            keyinfo.translated_modifiers,
            r#false != 0,
            locked,
            shortcuts_inhibited,
            device_identifier,
            exact_identifier,
            (*keyboard).effective_layout,
        );
    }
    if !binding.is_null()
        && (*binding).flags
            & binding_flags::BINDING_NOREPEAT.0 as ::core::ffi::c_int as uint32_t == 0
        && (*(*keyboard).wlr).repeat_info.delay > 0 as int32_t
    {
        (*keyboard).repeat_binding = binding;
        if wl_event_source_timer_update(
            (*keyboard).key_repeat_source,
            (*(*keyboard).wlr).repeat_info.delay as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] failed to set key repeat timer\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
                514 as ::core::ffi::c_int,
            );
        }
    } else if !(*keyboard).repeat_binding.is_null() {
        sway_keyboard_disarm_key_repeat(keyboard);
    }
    if !binding.is_null() {
        seat_execute_command(seat, binding);
        handled = r#true != 0;
    }
    if !handled && !(*(*keyboard).wlr).group.is_null() {
        free(device_identifier as *mut ::core::ffi::c_void);
        return;
    }
    if !handled
        && (*event).state.0 == wl_keyboard_key_state::WL_KEYBOARD_KEY_STATE_PRESSED.0
    {
        handled = keyboard_execute_compositor_binding(
            keyboard,
            keyinfo.translated_keysyms,
            keyinfo.translated_modifiers,
            keyinfo.translated_keysyms_len,
        );
    }
    if !handled
        && (*event).state.0 == wl_keyboard_key_state::WL_KEYBOARD_KEY_STATE_PRESSED.0
    {
        handled = keyboard_execute_compositor_binding(
            keyboard,
            keyinfo.raw_keysyms,
            keyinfo.raw_modifiers,
            keyinfo.raw_keysyms_len,
        );
    }
    if !handled {
        handled = keyboard_execute_pointer_keysyms(
            keyboard,
            (*event).time_msec,
            keyinfo.translated_keysyms,
            keyinfo.translated_keysyms_len,
            (*event).state,
        );
    }
    if (*event).state.0 == wl_keyboard_key_state::WL_KEYBOARD_KEY_STATE_RELEASED.0 {
        let mut pressed_sent: bool = update_shortcut_state(
            &raw mut (*keyboard).state_pressed_sent,
            (*event).keycode,
            (*event).state,
            keyinfo.keycode,
            0 as uint32_t,
        );
        if pressed_sent as ::core::ffi::c_int != 0
            && !(*(*seat).wlr_seat).keyboard_state.focused_surface.is_null()
        {
            wlr_seat_set_keyboard(wlr_seat, (*keyboard).wlr);
            wlr_seat_keyboard_notify_key(
                wlr_seat,
                (*event).time_msec,
                (*event).keycode,
                (*event).state.0 as uint32_t,
            );
            handled = r#true != 0;
        }
    }
    if !handled {
        let mut kb_grab: *mut wlr_input_method_keyboard_grab_v2 = keyboard_get_im_grab(
            keyboard,
        );
        if !kb_grab.is_null() {
            wlr_input_method_keyboard_grab_v2_set_keyboard(kb_grab, (*keyboard).wlr);
            wlr_input_method_keyboard_grab_v2_send_key(
                kb_grab,
                (*event).time_msec,
                (*event).keycode,
                (*event).state.0 as uint32_t,
            );
            handled = r#true != 0;
        }
    }
    if !handled
        && (*event).state.0 != wl_keyboard_key_state::WL_KEYBOARD_KEY_STATE_RELEASED.0
    {
        update_shortcut_state(
            &raw mut (*keyboard).state_pressed_sent,
            (*event).keycode,
            (*event).state,
            keyinfo.keycode,
            0 as uint32_t,
        );
        wlr_seat_set_keyboard(wlr_seat, (*keyboard).wlr);
        wlr_seat_keyboard_notify_key(
            wlr_seat,
            (*event).time_msec,
            (*event).keycode,
            (*event).state.0 as uint32_t,
        );
    }
    free(device_identifier as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn handle_keyboard_key(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut keyboard: *mut sway_keyboard = (listener as *mut ::core::ffi::c_char)
        .offset(-(40 as ::core::ffi::c_ulong as isize)) as *mut sway_keyboard;
    handle_key_event(keyboard, data as *mut wlr_keyboard_key_event);
}
unsafe extern "C" fn handle_keyboard_group_key(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut sway_group: *mut sway_keyboard_group = (listener as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut sway_keyboard_group;
    handle_key_event(
        (*(*sway_group).seat_device).keyboard,
        data as *mut wlr_keyboard_key_event,
    );
}
unsafe extern "C" fn handle_keyboard_group_enter(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut sway_group: *mut sway_keyboard_group = (listener as *mut ::core::ffi::c_char)
        .offset(-(64 as ::core::ffi::c_ulong as isize)) as *mut sway_keyboard_group;
    let mut keyboard: *mut sway_keyboard = (*(*sway_group).seat_device).keyboard;
    let mut keycodes: *mut wl_array = data as *mut wl_array;
    let mut keycode: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    keycode = (*keycodes).data as *mut uint32_t;
    while (*keycodes).size != 0 as size_t
        && (keycode as *const ::core::ffi::c_char)
            < ((*keycodes).data as *const ::core::ffi::c_char)
                .offset((*keycodes).size as isize)
    {
        let mut keyinfo: key_info = key_info {
            keycode: 0,
            code_modifiers: 0,
            raw_keysyms: ::core::ptr::null::<xkb_keysym_t>(),
            raw_modifiers: 0,
            raw_keysyms_len: 0,
            translated_keysyms: ::core::ptr::null::<xkb_keysym_t>(),
            translated_modifiers: 0,
            translated_keysyms_len: 0,
        };
        update_keyboard_state(
            keyboard,
            *keycode,
            wl_keyboard_key_state::WL_KEYBOARD_KEY_STATE_PRESSED,
            &raw mut keyinfo,
        );
        keycode = keycode.offset(1);
    }
}
unsafe extern "C" fn handle_keyboard_group_leave(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut sway_group: *mut sway_keyboard_group = (listener as *mut ::core::ffi::c_char)
        .offset(-(88 as ::core::ffi::c_ulong as isize)) as *mut sway_keyboard_group;
    let mut keyboard: *mut sway_keyboard = (*(*sway_group).seat_device).keyboard;
    let mut keycodes: *mut wl_array = data as *mut wl_array;
    let mut pressed_sent: bool = r#false != 0;
    let mut keycode: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    keycode = (*keycodes).data as *mut uint32_t;
    while (*keycodes).size != 0 as size_t
        && (keycode as *const ::core::ffi::c_char)
            < ((*keycodes).data as *const ::core::ffi::c_char)
                .offset((*keycodes).size as isize)
    {
        let mut keyinfo: key_info = key_info {
            keycode: 0,
            code_modifiers: 0,
            raw_keysyms: ::core::ptr::null::<xkb_keysym_t>(),
            raw_modifiers: 0,
            raw_keysyms_len: 0,
            translated_keysyms: ::core::ptr::null::<xkb_keysym_t>(),
            translated_modifiers: 0,
            translated_keysyms_len: 0,
        };
        update_keyboard_state(
            keyboard,
            *keycode,
            wl_keyboard_key_state::WL_KEYBOARD_KEY_STATE_RELEASED,
            &raw mut keyinfo,
        );
        pressed_sent = pressed_sent as ::core::ffi::c_int
            | update_shortcut_state(
                &raw mut (*keyboard).state_pressed_sent,
                *keycode,
                wl_keyboard_key_state::WL_KEYBOARD_KEY_STATE_RELEASED,
                keyinfo.keycode,
                0 as uint32_t,
            ) as ::core::ffi::c_int != 0;
        keycode = keycode.offset(1);
    }
    if !pressed_sent {
        return;
    }
    let mut seat: *mut sway_seat = (*(*keyboard).seat_device).sway_seat;
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
    };
}
unsafe extern "C" fn handle_keyboard_repeat(
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut keyboard: *mut sway_keyboard = data as *mut sway_keyboard;
    if !(*keyboard).repeat_binding.is_null() {
        if (*(*keyboard).wlr).repeat_info.rate > 0 as int32_t {
            if wl_event_source_timer_update(
                (*keyboard).key_repeat_source,
                1000 as ::core::ffi::c_int
                    / (*(*keyboard).wlr).repeat_info.rate as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] failed to update key repeat timer\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
                    662 as ::core::ffi::c_int,
                );
            }
        }
        seat_execute_command(
            (*(*keyboard).seat_device).sway_seat,
            (*keyboard).repeat_binding,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn determine_bar_visibility(mut modifiers_0: uint32_t) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*config).bars).length {
        let mut bar: *mut bar_config = *(*(*config).bars).items.offset(i as isize)
            as *mut bar_config;
        if (*bar).modifier != 0 as uint32_t {
            let mut vis_by_mod: bool = !modifiers_0 & (*bar).modifier == 0 as uint32_t;
            if (*bar).visible_by_modifier as ::core::ffi::c_int
                != vis_by_mod as ::core::ffi::c_int
            {
                if (*bar).visible_by_modifier as ::core::ffi::c_int != 0
                    || strcmp((*bar).mode, (*bar).hidden_state)
                        == 0 as ::core::ffi::c_int
                {
                    (*bar).visible_by_modifier = vis_by_mod;
                    ipc_event_bar_state_update(bar);
                }
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn handle_modifier_event(mut keyboard: *mut sway_keyboard) {
    if (*(*keyboard).wlr).group.is_null() {
        let mut kb_grab: *mut wlr_input_method_keyboard_grab_v2 = keyboard_get_im_grab(
            keyboard,
        );
        if !kb_grab.is_null() {
            wlr_input_method_keyboard_grab_v2_set_keyboard(kb_grab, (*keyboard).wlr);
            wlr_input_method_keyboard_grab_v2_send_modifiers(
                kb_grab,
                &raw mut (*(*keyboard).wlr).modifiers,
            );
        } else {
            let mut wlr_seat: *mut wlr_seat = (*(*(*keyboard).seat_device).sway_seat)
                .wlr_seat;
            wlr_seat_set_keyboard(wlr_seat, (*keyboard).wlr);
            wlr_seat_keyboard_notify_modifiers(
                wlr_seat,
                &raw mut (*(*keyboard).wlr).modifiers,
            );
        }
        let mut modifiers_0: uint32_t = wlr_keyboard_get_modifiers((*keyboard).wlr);
        determine_bar_visibility(modifiers_0);
    }
    if (*(*keyboard).wlr).modifiers.group != (*keyboard).effective_layout {
        (*keyboard).effective_layout = (*(*keyboard).wlr).modifiers.group;
        if wlr_keyboard_group_from_wlr_keyboard((*keyboard).wlr).is_null() {
            ipc_event_input(
                b"xkb_layout\0".as_ptr() as *const ::core::ffi::c_char,
                (*(*keyboard).seat_device).input_device,
            );
        }
    }
}
unsafe extern "C" fn handle_keyboard_modifiers(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut keyboard: *mut sway_keyboard = (listener as *mut ::core::ffi::c_char)
        .offset(-(64 as ::core::ffi::c_ulong as isize)) as *mut sway_keyboard;
    handle_modifier_event(keyboard);
}
unsafe extern "C" fn handle_keyboard_group_modifiers(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut group: *mut sway_keyboard_group = (listener as *mut ::core::ffi::c_char)
        .offset(-(40 as ::core::ffi::c_ulong as isize)) as *mut sway_keyboard_group;
    handle_modifier_event((*(*group).seat_device).keyboard);
}
#[no_mangle]
pub unsafe extern "C" fn sway_keyboard_create(
    mut seat: *mut sway_seat,
    mut device: *mut sway_seat_device,
) -> *mut sway_keyboard {
    let mut keyboard: *mut sway_keyboard = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_keyboard>(),
    ) as *mut sway_keyboard;
    if !_sway_assert(
        !keyboard.is_null(),
        b"[%s:%d] %s:could not allocate sway keyboard\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
        739 as ::core::ffi::c_int,
        b"struct sway_keyboard *sway_keyboard_create(struct sway_seat *, struct sway_seat_device *)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return ::core::ptr::null_mut::<sway_keyboard>();
    }
    (*keyboard).seat_device = device;
    (*keyboard).wlr = wlr_keyboard_from_input_device(
        (*(*device).input_device).wlr_device,
    );
    (*device).keyboard = keyboard as *mut sway_keyboard;
    wl_list_init(&raw mut (*keyboard).keyboard_key.link);
    wl_list_init(&raw mut (*keyboard).keyboard_modifiers.link);
    (*keyboard).key_repeat_source = wl_event_loop_add_timer(
        server.wl_event_loop,
        Some(
            handle_keyboard_repeat
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        keyboard as *mut ::core::ffi::c_void,
    );
    return keyboard;
}
unsafe extern "C" fn handle_xkb_context_log(
    mut context: *mut xkb_context,
    mut level: xkb_log_level,
    mut format: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
) {
    let mut error: *mut ::core::ffi::c_char = vformat_str(format, args.clone());
    let mut len: size_t = strlen(error);
    if *error.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        == '\n' as ::core::ffi::c_int
    {
        *error.offset(len.wrapping_sub(1 as size_t) as isize) = '\0'
            as ::core::ffi::c_char;
    }
    let mut importance: sway_log_importance_t = sway_log_importance_t::SWAY_DEBUG;
    if level.0 <= xkb_log_level::XKB_LOG_LEVEL_ERROR.0 {
        importance = sway_log_importance_t::SWAY_ERROR;
    } else if level.0 <= xkb_log_level::XKB_LOG_LEVEL_INFO.0 {
        importance = sway_log_importance_t::SWAY_INFO;
    }
    _sway_log(
        importance,
        b"[%s:%d] [xkbcommon] %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
        771 as ::core::ffi::c_int,
        error,
    );
    let mut data: *mut *mut ::core::ffi::c_char = xkb_context_get_user_data(context)
        as *mut *mut ::core::ffi::c_char;
    if importance.0 == sway_log_importance_t::SWAY_ERROR.0 && !data.is_null()
        && (*data).is_null()
    {
        *data = error;
    } else {
        free(error as *mut ::core::ffi::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn sway_keyboard_compile_keymap(
    mut ic: *mut input_config,
    mut error: *mut *mut ::core::ffi::c_char,
) -> *mut xkb_keymap {
    let mut context: *mut xkb_context = xkb_context_new(
        xkb_context_flags::XKB_CONTEXT_NO_SECURE_GETENV,
    );
    if !_sway_assert(
        !context.is_null(),
        b"[%s:%d] %s:cannot create XKB context\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
        784 as ::core::ffi::c_int,
        b"struct xkb_keymap *sway_keyboard_compile_keymap(struct input_config *, char **)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) {
        return ::core::ptr::null_mut::<xkb_keymap>();
    }
    xkb_context_set_user_data(context, error as *mut ::core::ffi::c_void);
    xkb_context_set_log_fn(
        context,
        Some(
            handle_xkb_context_log
                as unsafe extern "C" fn(
                    *mut xkb_context,
                    xkb_log_level,
                    *const ::core::ffi::c_char,
                    ::core::ffi::VaList,
                ) -> (),
        ),
    );
    let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
    if !ic.is_null() && !(*ic).xkb_file.is_null() {
        let mut keymap_file: *mut FILE = fopen(
            (*ic).xkb_file,
            b"r\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if keymap_file.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] cannot read xkb file %s: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
                795 as ::core::ffi::c_int,
                (*ic).xkb_file,
                strerror(*__errno_location()),
            );
            if !error.is_null() {
                *error = format_str(
                    b"cannot read xkb file %s: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*ic).xkb_file,
                    strerror(*__errno_location()),
                );
            }
        } else {
            keymap = xkb_keymap_new_from_file(
                context,
                keymap_file,
                xkb_keymap_format::XKB_KEYMAP_FORMAT_TEXT_V1,
                xkb_keymap_compile_flags::XKB_KEYMAP_COMPILE_NO_FLAGS,
            );
            if fclose(keymap_file) != 0 as ::core::ffi::c_int {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] Failed to close xkb file %s: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
                    807 as ::core::ffi::c_int,
                    (*ic).xkb_file,
                    strerror(*__errno_location()),
                );
            }
        }
    } else {
        let mut rules: xkb_rule_names = xkb_rule_names {
            rules: ::core::ptr::null::<::core::ffi::c_char>(),
            model: ::core::ptr::null::<::core::ffi::c_char>(),
            layout: ::core::ptr::null::<::core::ffi::c_char>(),
            variant: ::core::ptr::null::<::core::ffi::c_char>(),
            options: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        if !ic.is_null() {
            input_config_fill_rule_names(ic, &raw mut rules);
        }
        keymap = xkb_keymap_new_from_names(
            context,
            &raw mut rules,
            xkb_keymap_compile_flags::XKB_KEYMAP_COMPILE_NO_FLAGS,
        );
    }
    xkb_context_set_user_data(context, NULL);
    xkb_context_unref(context);
    return keymap;
}
unsafe extern "C" fn repeat_info_match(
    mut a: *mut sway_keyboard,
    mut b: *mut wlr_keyboard,
) -> bool {
    return (*a).repeat_rate == (*b).repeat_info.rate
        && (*a).repeat_delay == (*b).repeat_info.delay;
}
unsafe extern "C" fn destroy_empty_wlr_keyboard_group(
    mut data: *mut ::core::ffi::c_void,
) {
    wlr_keyboard_group_destroy(data as *mut wlr_keyboard_group);
}
unsafe extern "C" fn sway_keyboard_group_remove(mut keyboard: *mut sway_keyboard) {
    let mut device: *mut sway_input_device = (*(*keyboard).seat_device).input_device;
    let mut wlr_group: *mut wlr_keyboard_group = (*(*keyboard).wlr).group;
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Removing keyboard %s from group %p\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
        838 as ::core::ffi::c_int,
        (*device).identifier,
        wlr_group,
    );
    wlr_keyboard_group_remove_keyboard((*(*keyboard).wlr).group, (*keyboard).wlr);
    if wl_list_empty(&raw mut (*wlr_group).devices) != 0 {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Destroying empty keyboard group %p\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
            844 as ::core::ffi::c_int,
            wlr_group,
        );
        let mut sway_group: *mut sway_keyboard_group = (*wlr_group).data
            as *mut sway_keyboard_group;
        (*wlr_group).data = NULL;
        wl_list_remove(&raw mut (*sway_group).link);
        wl_list_remove(&raw mut (*sway_group).keyboard_key.link);
        wl_list_remove(&raw mut (*sway_group).keyboard_modifiers.link);
        wl_list_remove(&raw mut (*sway_group).enter.link);
        wl_list_remove(&raw mut (*sway_group).leave.link);
        sway_keyboard_destroy((*(*sway_group).seat_device).keyboard);
        free((*(*sway_group).seat_device).input_device as *mut ::core::ffi::c_void);
        free((*sway_group).seat_device as *mut ::core::ffi::c_void);
        free(sway_group as *mut ::core::ffi::c_void);
        wl_event_loop_add_idle(
            server.wl_event_loop,
            Some(
                destroy_empty_wlr_keyboard_group
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
            wlr_group as *mut ::core::ffi::c_void,
        );
    }
}
unsafe extern "C" fn sway_keyboard_group_remove_invalid(
    mut keyboard: *mut sway_keyboard,
) {
    if (*(*keyboard).wlr).group.is_null() {
        return;
    }
    let mut seat: *mut sway_seat = (*(*keyboard).seat_device).sway_seat;
    let mut sc: *mut seat_config = seat_get_config(seat);
    if sc.is_null() {
        sc = seat_get_config_by_name(b"*\0".as_ptr() as *const ::core::ffi::c_char);
    }
    match if !sc.is_null() {
        (*sc).keyboard_grouping.0
    } else {
        seat_keyboard_grouping::KEYBOARD_GROUP_DEFAULT.0
    } {
        1 => {
            sway_keyboard_group_remove(keyboard);
        }
        0 | 2 => {
            let mut group: *mut wlr_keyboard_group = (*(*keyboard).wlr).group;
            if !wlr_keyboard_keymaps_match((*keyboard).keymap, (*group).keyboard.keymap)
                || !repeat_info_match(keyboard, &raw mut (*group).keyboard)
            {
                sway_keyboard_group_remove(keyboard);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn sway_keyboard_group_add(mut keyboard: *mut sway_keyboard) {
    let mut device: *mut sway_input_device = (*(*keyboard).seat_device).input_device;
    let mut seat: *mut sway_seat = (*(*keyboard).seat_device).sway_seat;
    let mut sc: *mut seat_config = seat_get_config(seat);
    if (*device).is_virtual {
        return;
    }
    if sc.is_null() {
        sc = seat_get_config_by_name(b"*\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if !sc.is_null()
        && (*sc).keyboard_grouping.0 == seat_keyboard_grouping::KEYBOARD_GROUP_NONE.0
    {
        return;
    }
    let mut group: *mut sway_keyboard_group = ::core::ptr::null_mut::<
        sway_keyboard_group,
    >();
    group = ((*seat).keyboard_groups.next as *mut ::core::ffi::c_char)
        .offset(-(112 as ::core::ffi::c_ulong as isize)) as *mut sway_keyboard_group
        as *mut sway_keyboard_group;
    while &raw mut (*group).link != &raw mut (*seat).keyboard_groups {
        match if !sc.is_null() {
            (*sc).keyboard_grouping.0
        } else {
            seat_keyboard_grouping::KEYBOARD_GROUP_DEFAULT.0
        } {
            1 => return,
            0 | 2 => {
                let mut wlr_group: *mut wlr_keyboard_group = (*group).wlr_group;
                if wlr_keyboard_keymaps_match(
                    (*keyboard).keymap,
                    (*wlr_group).keyboard.keymap,
                ) as ::core::ffi::c_int != 0
                    && repeat_info_match(keyboard, &raw mut (*wlr_group).keyboard)
                        as ::core::ffi::c_int != 0
                {
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Adding keyboard %s to group %p\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/input/keyboard.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        922 as ::core::ffi::c_int,
                        (*device).identifier,
                        wlr_group,
                    );
                    wlr_keyboard_group_add_keyboard(wlr_group, (*keyboard).wlr);
                    return;
                }
            }
            _ => {}
        }
        group = ((*group).link.next as *mut ::core::ffi::c_char)
            .offset(-(112 as ::core::ffi::c_ulong as isize)) as *mut sway_keyboard_group
            as *mut sway_keyboard_group;
    }
    let mut sway_group: *mut sway_keyboard_group = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_keyboard_group>(),
    ) as *mut sway_keyboard_group;
    if sway_group.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to allocate sway_keyboard_group\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
            934 as ::core::ffi::c_int,
        );
        return;
    }
    (*sway_group).wlr_group = wlr_keyboard_group_create();
    if (*sway_group).wlr_group.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create keyboard group\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
            940 as ::core::ffi::c_int,
        );
    } else {
        (*(*sway_group).wlr_group).data = sway_group as *mut ::core::ffi::c_void;
        wlr_keyboard_set_keymap(
            &raw mut (*(*sway_group).wlr_group).keyboard,
            (*keyboard).keymap,
        );
        wlr_keyboard_set_repeat_info(
            &raw mut (*(*sway_group).wlr_group).keyboard,
            (*keyboard).repeat_rate,
            (*keyboard).repeat_delay,
        );
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Created keyboard group %p\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
            947 as ::core::ffi::c_int,
            (*sway_group).wlr_group,
        );
        (*sway_group).seat_device = calloc(
            1 as size_t,
            ::core::mem::size_of::<sway_seat_device>(),
        ) as *mut sway_seat_device;
        if (*sway_group).seat_device.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Failed to allocate sway_seat_device for group\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
                951 as ::core::ffi::c_int,
            );
        } else {
            (*(*sway_group).seat_device).sway_seat = seat;
            (*(*sway_group).seat_device).input_device = calloc(
                1 as size_t,
                ::core::mem::size_of::<sway_input_device>(),
            ) as *mut sway_input_device;
            if (*(*sway_group).seat_device).input_device.is_null() {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] Failed to allocate sway_input_device for group\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
                    959 as ::core::ffi::c_int,
                );
            } else {
                (*(*(*sway_group).seat_device).input_device).wlr_device = &raw mut (*(*sway_group)
                    .wlr_group)
                    .keyboard
                    .base;
                if sway_keyboard_create(seat, (*sway_group).seat_device).is_null() {
                    _sway_log(
                        sway_log_importance_t::SWAY_ERROR,
                        b"[%s:%d] Failed to allocate sway_keyboard for group\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/input/keyboard.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        966 as ::core::ffi::c_int,
                    );
                } else {
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Adding keyboard %s to group %p\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/input/keyboard.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        970 as ::core::ffi::c_int,
                        (*device).identifier,
                        (*sway_group).wlr_group,
                    );
                    wlr_keyboard_group_add_keyboard(
                        (*sway_group).wlr_group,
                        (*keyboard).wlr,
                    );
                    wl_list_insert(
                        &raw mut (*seat).keyboard_groups,
                        &raw mut (*sway_group).link,
                    );
                    wl_signal_add(
                        &raw mut (*(*sway_group).wlr_group).keyboard.events.key,
                        &raw mut (*sway_group).keyboard_key,
                    );
                    (*sway_group).keyboard_key.notify = Some(
                        handle_keyboard_group_key
                            as unsafe extern "C" fn(
                                *mut wl_listener,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ) as wl_notify_func_t;
                    wl_signal_add(
                        &raw mut (*(*sway_group).wlr_group).keyboard.events.modifiers,
                        &raw mut (*sway_group).keyboard_modifiers,
                    );
                    (*sway_group).keyboard_modifiers.notify = Some(
                        handle_keyboard_group_modifiers
                            as unsafe extern "C" fn(
                                *mut wl_listener,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ) as wl_notify_func_t;
                    wl_signal_add(
                        &raw mut (*(*sway_group).wlr_group).events.enter,
                        &raw mut (*sway_group).enter,
                    );
                    (*sway_group).enter.notify = Some(
                        handle_keyboard_group_enter
                            as unsafe extern "C" fn(
                                *mut wl_listener,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ) as wl_notify_func_t;
                    wl_signal_add(
                        &raw mut (*(*sway_group).wlr_group).events.leave,
                        &raw mut (*sway_group).leave,
                    );
                    (*sway_group).leave.notify = Some(
                        handle_keyboard_group_leave
                            as unsafe extern "C" fn(
                                *mut wl_listener,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ) as wl_notify_func_t;
                    return;
                }
            }
        }
    }
    if !sway_group.is_null() && !(*sway_group).wlr_group.is_null() {
        wlr_keyboard_group_destroy((*sway_group).wlr_group);
    }
    free((*(*sway_group).seat_device).keyboard as *mut ::core::ffi::c_void);
    free((*(*sway_group).seat_device).input_device as *mut ::core::ffi::c_void);
    free((*sway_group).seat_device as *mut ::core::ffi::c_void);
    free(sway_group as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn sway_keyboard_set_layout(
    mut keyboard: *mut sway_keyboard,
    mut input_config: *mut input_config,
) {
    let mut keymap: *mut xkb_keymap = sway_keyboard_compile_keymap(
        input_config,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    if keymap.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to compile keymap. Attempting defaults\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
            1005 as ::core::ffi::c_int,
        );
        keymap = sway_keyboard_compile_keymap(
            ::core::ptr::null_mut::<input_config>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        if keymap.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Failed to compile default keymap. Aborting configure\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
                1008 as ::core::ffi::c_int,
            );
            return;
        }
    }
    let mut keymap_changed: bool = if !(*keyboard).keymap.is_null() {
        !wlr_keyboard_keymaps_match((*keyboard).keymap, keymap) as ::core::ffi::c_int
    } else {
        r#true
    } != 0;
    let mut effective_layout_changed: bool = (*keyboard).effective_layout
        != 0 as xkb_layout_index_t;
    if keymap_changed as ::core::ffi::c_int != 0
        || (*config).reloading as ::core::ffi::c_int != 0
    {
        xkb_keymap_unref((*keyboard).keymap);
        (*keyboard).keymap = keymap;
        (*keyboard).effective_layout = 0 as xkb_layout_index_t;
        sway_keyboard_group_remove_invalid(keyboard);
        wlr_keyboard_set_keymap((*keyboard).wlr, (*keyboard).keymap);
        if (*(*keyboard).wlr).group.is_null() {
            sway_keyboard_group_add(keyboard);
        }
        let mut locked_mods: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        if !input_config.is_null()
            && (*input_config).xkb_numlock > 0 as ::core::ffi::c_int
        {
            let mut mod_index: xkb_mod_index_t = xkb_keymap_mod_get_index(
                keymap,
                b"Mod2\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if mod_index != XKB_MOD_INVALID as xkb_mod_index_t {
                locked_mods |= (1 as ::core::ffi::c_int as uint32_t) << mod_index;
            }
        }
        if !input_config.is_null()
            && (*input_config).xkb_capslock > 0 as ::core::ffi::c_int
        {
            let mut mod_index_0: xkb_mod_index_t = xkb_keymap_mod_get_index(
                keymap,
                b"Lock\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if mod_index_0 != XKB_MOD_INVALID as xkb_mod_index_t {
                locked_mods |= (1 as ::core::ffi::c_int as uint32_t) << mod_index_0;
            }
        }
        if locked_mods != 0 {
            wlr_keyboard_notify_modifiers(
                (*keyboard).wlr,
                0 as uint32_t,
                0 as uint32_t,
                locked_mods,
                0 as uint32_t,
            );
            let mut leds: uint32_t = 0 as uint32_t;
            let mut i: uint32_t = 0 as uint32_t;
            while i < WLR_LED_COUNT as uint32_t {
                if xkb_state_led_index_is_active(
                    (*(*keyboard).wlr).xkb_state,
                    (*(*keyboard).wlr).led_indexes[i as usize],
                ) != 0
                {
                    leds |= ((1 as ::core::ffi::c_int) << i) as uint32_t;
                }
                i = i.wrapping_add(1);
            }
            if !(*(*keyboard).wlr).group.is_null() {
                wlr_keyboard_led_update(
                    &raw mut (*(*(*keyboard).wlr).group).keyboard,
                    leds,
                );
            } else {
                wlr_keyboard_led_update((*keyboard).wlr, leds);
            }
        }
    } else {
        xkb_keymap_unref(keymap);
        sway_keyboard_group_remove_invalid(keyboard);
        if (*(*keyboard).wlr).group.is_null() {
            sway_keyboard_group_add(keyboard);
        }
    }
    if keymap_changed {
        ipc_event_input(
            b"xkb_keymap\0".as_ptr() as *const ::core::ffi::c_char,
            (*(*keyboard).seat_device).input_device,
        );
    } else if effective_layout_changed {
        ipc_event_input(
            b"xkb_layout\0".as_ptr() as *const ::core::ffi::c_char,
            (*(*keyboard).seat_device).input_device,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sway_keyboard_configure(mut keyboard: *mut sway_keyboard) {
    let mut input_config: *mut input_config = input_device_get_config(
        (*(*keyboard).seat_device).input_device,
    );
    if !_sway_assert(
        wlr_keyboard_group_from_wlr_keyboard((*keyboard).wlr).is_null(),
        b"[%s:%d] %s:sway_keyboard_configure should not be called with a keyboard group's keyboard\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"sway/input/keyboard.c\0".as_ptr() as *const ::core::ffi::c_char,
        1081 as ::core::ffi::c_int,
        b"void sway_keyboard_configure(struct sway_keyboard *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    let mut repeat_rate: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    if !input_config.is_null() && (*input_config).repeat_rate != INT_MIN {
        repeat_rate = (*input_config).repeat_rate;
    }
    let mut repeat_delay: ::core::ffi::c_int = 600 as ::core::ffi::c_int;
    if !input_config.is_null() && (*input_config).repeat_delay != INT_MIN {
        repeat_delay = (*input_config).repeat_delay;
    }
    let mut repeat_info_changed: bool = (*keyboard).repeat_rate != repeat_rate as int32_t
        || (*keyboard).repeat_delay != repeat_delay as int32_t;
    if repeat_info_changed as ::core::ffi::c_int != 0
        || (*config).reloading as ::core::ffi::c_int != 0
    {
        (*keyboard).repeat_rate = repeat_rate as int32_t;
        (*keyboard).repeat_delay = repeat_delay as int32_t;
        wlr_keyboard_set_repeat_info(
            (*keyboard).wlr,
            (*keyboard).repeat_rate,
            (*keyboard).repeat_delay,
        );
    }
    if !(*(*(*keyboard).seat_device).input_device).is_virtual {
        sway_keyboard_set_layout(keyboard, input_config);
    }
    let mut seat: *mut wlr_seat = (*(*(*keyboard).seat_device).sway_seat).wlr_seat;
    let mut current_keyboard: *mut wlr_keyboard = (*seat).keyboard_state.keyboard;
    if current_keyboard.is_null() {
        wlr_seat_set_keyboard(seat, (*keyboard).wlr);
    }
    wl_list_remove(&raw mut (*keyboard).keyboard_key.link);
    wl_signal_add(
        &raw mut (*(*keyboard).wlr).events.key,
        &raw mut (*keyboard).keyboard_key,
    );
    (*keyboard).keyboard_key.notify = Some(
        handle_keyboard_key
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_list_remove(&raw mut (*keyboard).keyboard_modifiers.link);
    wl_signal_add(
        &raw mut (*(*keyboard).wlr).events.modifiers,
        &raw mut (*keyboard).keyboard_modifiers,
    );
    (*keyboard).keyboard_modifiers.notify = Some(
        handle_keyboard_modifiers
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
}
#[no_mangle]
pub unsafe extern "C" fn sway_keyboard_destroy(mut keyboard: *mut sway_keyboard) {
    if keyboard.is_null() {
        return;
    }
    if !(*(*keyboard).wlr).group.is_null() {
        sway_keyboard_group_remove(keyboard);
    }
    let mut wlr_seat: *mut wlr_seat = (*(*(*keyboard).seat_device).sway_seat).wlr_seat;
    if wlr_seat_get_keyboard(wlr_seat) == (*keyboard).wlr {
        wlr_seat_set_keyboard(wlr_seat, ::core::ptr::null_mut::<wlr_keyboard>());
    }
    if !(*keyboard).keymap.is_null() {
        xkb_keymap_unref((*keyboard).keymap);
    }
    wl_list_remove(&raw mut (*keyboard).keyboard_key.link);
    wl_list_remove(&raw mut (*keyboard).keyboard_modifiers.link);
    sway_keyboard_disarm_key_repeat(keyboard);
    wl_event_source_remove((*keyboard).key_repeat_source);
    free(keyboard as *mut ::core::ffi::c_void);
}
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
