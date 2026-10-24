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
    pub type sway_output;
    pub type wlr_keyboard_group;
    pub type wlr_keyboard_impl;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_primary_selection_source;
    pub type wlr_tablet_v2_tablet_tool;
    pub type wlr_tablet_v2_tablet;
    pub type wlr_tablet_v2_tablet_pad;
    pub type wlr_tablet_pad;
    pub type sway_switch;
    pub type sway_keyboard;
    pub type wlr_touch_impl;
    pub type sway_cursor;
    pub type _PangoFontDescription;
    pub type pcre2_real_code_8;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
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
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const ::core::ffi::c_char,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_new_array() -> *mut json_object;
    fn json_object_array_add(
        obj: *mut json_object,
        val: *mut json_object,
    ) -> ::core::ffi::c_int;
    fn json_object_new_boolean(b: json_bool) -> *mut json_object;
    fn json_object_new_string(s: *const ::core::ffi::c_char) -> *mut json_object;
    fn strip_quotes(str: *mut ::core::ffi::c_char);
    fn split_args(
        str: *const ::core::ffi::c_char,
        argc: *mut ::core::ffi::c_int,
    ) -> *mut *mut ::core::ffi::c_char;
    fn free_argv(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char);
    fn unescape_string(string: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn join_args(
        argv: *mut *mut ::core::ffi::c_char,
        argc: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn argsep(
        stringp: *mut *mut ::core::ffi::c_char,
        delim: *const ::core::ffi::c_char,
        matched_delim: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn vformat_str(
        fmt: *const ::core::ffi::c_char,
        args: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn create_list() -> *mut list_t;
    fn list_free(list: *mut list_t);
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    static mut root: *mut sway_root;
    fn cmd_allow_tearing(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_assign(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_bar(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_bindcode(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_bindgesture(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_bindswitch(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_bindsym(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_border(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_client_noop(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_client_focused(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_client_focused_inactive(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_client_focused_tab_title(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_client_unfocused(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_client_urgent(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_create_output(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_default_border(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_default_floating_border(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_default_orientation(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_exec(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_exec_always(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_exit(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_floating(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_floating_maximum_size(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_floating_minimum_size(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_floating_modifier(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_focus(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_focus_follows_mouse(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_focus_on_window_activation(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_focus_wrapping(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_font(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_for_window(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_force_display_urgency_hint(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_force_focus_wrapping(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_fullscreen(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_gaps(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_hide_edge_borders(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_include(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_inhibit_idle(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_input(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_seat(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_kill(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_layout(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_mark(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_max_render_time(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_mode(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_mouse_warping(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_move(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_new_float(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_new_window(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_nop(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_opacity(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_no_focus(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_output(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_popup_during_fullscreen(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_primary_selection(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_reload(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_rename(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_resize(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_scratchpad(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_set(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_shortcuts_inhibitor(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_show_marks(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_smart_borders(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_smart_gaps(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_split(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_splith(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_splitt(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_splitv(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_sticky(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_swaybg_command(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_swaynag_command(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_swap(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_tiling_drag(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_tiling_drag_threshold(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_title_align(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_title_format(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_titlebar_border_thickness(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_titlebar_padding(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_unbindcode(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_unbindswitch(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_unbindgesture(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_unbindsym(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_unmark(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_urgent(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_workspace(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_workspace_layout(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_ws_auto_back_and_forth(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn cmd_xwayland(
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn seat_get_focus_inactive(
        seat: *mut sway_seat,
        node: *mut sway_node,
    ) -> *mut sway_node;
    fn do_var_replacement(str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut config: *mut sway_config;
    fn input_manager_get_default_seat() -> *mut sway_seat;
    fn criteria_destroy(criteria: *mut criteria);
    fn criteria_parse(
        raw: *mut ::core::ffi::c_char,
        error: *mut *mut ::core::ffi::c_char,
    ) -> *mut criteria;
    fn criteria_get_containers(criteria: *mut criteria) -> *mut list_t;
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
pub type va_list = __builtin_va_list;
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type json_bool = ::core::ffi::c_int;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
    pub events: C2Rust_Unnamed_0,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub addons: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon {
    pub r#impl: *const wlr_addon_interface,
    pub WLR_PRIVATE: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub ref_count: ::core::ffi::c_int,
    pub addons: wlr_addon_set,
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
    pub events: C2Rust_Unnamed_6,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_5,
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
pub struct C2Rust_Unnamed_5 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
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
    pub viewport: C2Rust_Unnamed_7,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
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
    pub events: C2Rust_Unnamed_8,
    pub addons: wlr_addon_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub destroy: wl_signal,
    pub release: wl_signal,
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
    pub events: C2Rust_Unnamed_12,
    pub features: C2Rust_Unnamed_11,
    pub WLR_PRIVATE: C2Rust_Unnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub r#impl: *const wlr_renderer_impl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub input_color_transform: bool,
    pub output_color_transform: bool,
    pub timeline: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
    pub destroy: wl_signal,
    pub lost: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_compositor {
    pub global: *mut wl_global,
    pub renderer: *mut wlr_renderer,
    pub events: C2Rust_Unnamed_14,
    pub WLR_PRIVATE: C2Rust_Unnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
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
    pub events: C2Rust_Unnamed_18,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
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
    pub mastering_luminance: C2Rust_Unnamed_16,
    pub max_cll: ::core::ffi::c_double,
    pub max_fall: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub renderer_destroy: wl_listener,
    pub color_transform: *mut wlr_color_transform,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
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
    pub features: C2Rust_Unnamed_20,
    pub events: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub destroy: wl_signal,
    pub new_input: wl_signal,
    pub new_output: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
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
    pub custom_mode: C2Rust_Unnamed_21,
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
pub struct C2Rust_Unnamed_21 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_24,
    pub WLR_PRIVATE: C2Rust_Unnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
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
pub struct C2Rust_Unnamed_24 {
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
    pub events: C2Rust_Unnamed_26,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
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
    pub events: C2Rust_Unnamed_28,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_29,
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
pub struct C2Rust_Unnamed_29 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_31,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
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
    pub events: C2Rust_Unnamed_33,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
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
    pub events: C2Rust_Unnamed_39,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
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
    pub events: C2Rust_Unnamed_37,
    pub WLR_PRIVATE: C2Rust_Unnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
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
pub struct C2Rust_Unnamed_37 {
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
pub type wlr_scene_buffer_point_accepts_input_func_t = Option<
    unsafe extern "C" fn(
        *mut wlr_scene_buffer,
        *mut ::core::ffi::c_double,
        *mut ::core::ffi::c_double,
    ) -> bool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
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
    pub layers: C2Rust_Unnamed_50,
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
    pub events: C2Rust_Unnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_40 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_43,
    pub border: C2Rust_Unnamed_42,
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
    pub events: C2Rust_Unnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_41 {
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
pub struct C2Rust_Unnamed_42 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_45,
    pub events: C2Rust_Unnamed_44,
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
pub struct C2Rust_Unnamed_44 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_45 {
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
    pub events: C2Rust_Unnamed_47,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_46,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_49,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_49 {
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
pub struct C2Rust_Unnamed_50 {
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
    pub events: C2Rust_Unnamed_52,
    pub WLR_PRIVATE: C2Rust_Unnamed_51,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_52 {
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
    pub events: C2Rust_Unnamed_54,
    pub WLR_PRIVATE: C2Rust_Unnamed_53,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_53 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_54 {
    pub destroy: wl_signal,
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
    pub repeat_info: C2Rust_Unnamed_56,
    pub events: C2Rust_Unnamed_55,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_56 {
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
    pub events: C2Rust_Unnamed_58,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_57,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_57 {
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
pub struct C2Rust_Unnamed_58 {
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
pub struct wlr_seat_keyboard_state {
    pub seat: *mut wlr_seat,
    pub keyboard: *mut wlr_keyboard,
    pub focused_client: *mut wlr_seat_client,
    pub focused_surface: *mut wlr_surface,
    pub grab: *mut wlr_seat_keyboard_grab,
    pub default_grab: *mut wlr_seat_keyboard_grab,
    pub events: C2Rust_Unnamed_64,
    pub WLR_PRIVATE: C2Rust_Unnamed_63,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
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
    pub events: C2Rust_Unnamed_66,
    pub WLR_PRIVATE: C2Rust_Unnamed_65,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_65 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
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
    pub events: C2Rust_Unnamed_67,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
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
    pub events: C2Rust_Unnamed_68,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
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
    pub xcursor_theme: C2Rust_Unnamed_69,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
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
    pub colors: C2Rust_Unnamed_70,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
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
    pub border_colors: C2Rust_Unnamed_73,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_71,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_71 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_72,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_73 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_handler {
    pub command: *const ::core::ffi::c_char,
    pub handle: Option<sway_cmd>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct expected_args(pub ::core::ffi::c_uint);
impl expected_args {
    pub const EXPECTED_AT_LEAST: Self = Self(0);
    pub const EXPECTED_AT_MOST: Self = Self(1);
    pub const EXPECTED_EQUAL_TO: Self = Self(2);
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
pub type pcre2_code_8 = pcre2_real_code_8;
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
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const ::core::ffi::c_void,
    mut __base: *const ::core::ffi::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut ::core::ffi::c_void {
    let mut __p: *const ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>();
    let mut __comparison: ::core::ffi::c_int = 0;
    while __nmemb != 0 {
        __p = (__base as *const ::core::ffi::c_char)
            .offset((__nmemb >> 1 as ::core::ffi::c_int).wrapping_mul(__size) as isize)
            as *const ::core::ffi::c_void;
        __comparison = Some(__compar.expect("non-null function pointer"))
            .expect("non-null function pointer")(__key, __p);
        if __comparison == 0 as ::core::ffi::c_int {
            return __p as *mut ::core::ffi::c_void;
        }
        if __comparison > 0 as ::core::ffi::c_int {
            __base = (__p as *const ::core::ffi::c_char).offset(__size as isize)
                as *const ::core::ffi::c_void;
            __nmemb = __nmemb.wrapping_sub(1);
        }
        __nmemb >>= 1 as ::core::ffi::c_int;
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn checkarg(
    mut argc: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut r#type: expected_args,
    mut val: ::core::ffi::c_int,
) -> *mut cmd_results {
    let mut error_name: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    match r#type {
        expected_args::EXPECTED_AT_LEAST => {
            if argc < val {
                error_name = b"at least \0".as_ptr() as *const ::core::ffi::c_char;
            }
        }
        expected_args::EXPECTED_AT_MOST => {
            if argc > val {
                error_name = b"at most \0".as_ptr() as *const ::core::ffi::c_char;
            }
        }
        expected_args::EXPECTED_EQUAL_TO => {
            if argc != val {
                error_name = b"\0".as_ptr() as *const ::core::ffi::c_char;
            }
        }
        _ => {}
    }
    return if !error_name.is_null() {
        cmd_results_new(
            cmd_status::CMD_INVALID,
            b"Invalid %s command (expected %s%d argument%s, got %d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            error_name,
            val,
            if val != 1 as ::core::ffi::c_int {
                b"s\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            argc,
        )
    } else {
        ::core::ptr::null_mut::<cmd_results>()
    };
}
static mut handlers: [cmd_handler; 55] = [
    cmd_handler {
        command: b"assign\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_assign
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"bar\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_bar
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"bindcode\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_bindcode
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"bindgesture\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_bindgesture
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"bindswitch\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_bindswitch
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"bindsym\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_bindsym
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"client.background\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_client_noop
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"client.focused\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_client_focused
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"client.focused_inactive\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_client_focused_inactive
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"client.focused_tab_title\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_client_focused_tab_title
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"client.placeholder\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_client_noop
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"client.unfocused\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_client_unfocused
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"client.urgent\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_client_urgent
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"default_border\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_default_border
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"default_floating_border\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_default_floating_border
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"exec\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_exec
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"exec_always\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_exec_always
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"floating_maximum_size\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_floating_maximum_size
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"floating_minimum_size\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_floating_minimum_size
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"floating_modifier\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_floating_modifier
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"focus\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_focus
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"focus_follows_mouse\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_focus_follows_mouse
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"focus_on_window_activation\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_focus_on_window_activation
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"focus_wrapping\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_focus_wrapping
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"font\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_font
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"for_window\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_for_window
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"force_display_urgency_hint\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_force_display_urgency_hint
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"force_focus_wrapping\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_force_focus_wrapping
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"fullscreen\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_fullscreen
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"gaps\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_gaps
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"hide_edge_borders\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_hide_edge_borders
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"input\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_input
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"mode\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_mode
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"mouse_warping\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_mouse_warping
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"new_float\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_new_float
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"new_window\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_new_window
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"no_focus\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_no_focus
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"output\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_output
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"popup_during_fullscreen\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_popup_during_fullscreen
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"seat\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_seat
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"set\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_set
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"show_marks\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_show_marks
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"smart_borders\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_smart_borders
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"smart_gaps\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_smart_gaps
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"tiling_drag\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_tiling_drag
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"tiling_drag_threshold\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_tiling_drag_threshold
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"title_align\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_title_align
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"titlebar_border_thickness\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_titlebar_border_thickness
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"titlebar_padding\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_titlebar_padding
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"unbindcode\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_unbindcode
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"unbindgesture\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_unbindgesture
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"unbindswitch\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_unbindswitch
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"unbindsym\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_unbindsym
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"workspace\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_workspace
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"workspace_auto_back_and_forth\0".as_ptr()
            as *const ::core::ffi::c_char,
        handle: Some(
            cmd_ws_auto_back_and_forth
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
];
static mut config_handlers: [cmd_handler; 7] = [
    cmd_handler {
        command: b"default_orientation\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_default_orientation
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"include\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_include
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"primary_selection\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_primary_selection
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"swaybg_command\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_swaybg_command
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"swaynag_command\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_swaynag_command
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"workspace_layout\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_workspace_layout
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"xwayland\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_xwayland
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
];
static mut command_handlers: [cmd_handler; 28] = [
    cmd_handler {
        command: b"allow_tearing\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_allow_tearing
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"border\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_border
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"create_output\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_create_output
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"exit\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_exit
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"floating\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_floating
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"fullscreen\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_fullscreen
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"inhibit_idle\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_inhibit_idle
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"kill\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_kill
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"layout\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_layout
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"mark\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_mark
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"max_render_time\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_max_render_time
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"move\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_move
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"nop\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_nop
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"opacity\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_opacity
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"reload\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_reload
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"rename\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_rename
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"resize\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_resize
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"scratchpad\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_scratchpad
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"shortcuts_inhibitor\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_shortcuts_inhibitor
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"split\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_split
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"splith\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_splith
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"splitt\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_splitt
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"splitv\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_splitv
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"sticky\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_sticky
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"swap\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_swap
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"title_format\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_title_format
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"unmark\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_unmark
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
    cmd_handler {
        command: b"urgent\0".as_ptr() as *const ::core::ffi::c_char,
        handle: Some(
            cmd_urgent
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> *mut cmd_results,
        ),
    },
];
unsafe extern "C" fn handler_compare(
    mut _a: *const ::core::ffi::c_void,
    mut _b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const cmd_handler = _a as *const cmd_handler;
    let mut b: *const cmd_handler = _b as *const cmd_handler;
    return strcasecmp((*a).command, (*b).command);
}
#[no_mangle]
pub unsafe extern "C" fn find_handler(
    mut line: *const ::core::ffi::c_char,
    mut handlers_0: *const cmd_handler,
    mut handlers_size: size_t,
) -> *const cmd_handler {
    if handlers_0.is_null() || handlers_size == 0 {
        return ::core::ptr::null::<cmd_handler>();
    }
    let query: cmd_handler = cmd_handler {
        command: line,
        handle: None,
    };
    return bsearch(
        &raw const query as *const ::core::ffi::c_void,
        handlers_0 as *const ::core::ffi::c_void,
        handlers_size.wrapping_div(::core::mem::size_of::<cmd_handler>()),
        ::core::mem::size_of::<cmd_handler>(),
        Some(
            handler_compare
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    ) as *const cmd_handler;
}
unsafe extern "C" fn find_handler_ex(
    mut line: *mut ::core::ffi::c_char,
    mut config_handlers_0: *const cmd_handler,
    mut config_handlers_size: size_t,
    mut command_handlers_0: *const cmd_handler,
    mut command_handlers_size: size_t,
    mut handlers_0: *const cmd_handler,
    mut handlers_size: size_t,
) -> *const cmd_handler {
    let mut handler: *const cmd_handler = ::core::ptr::null::<cmd_handler>();
    if (*config).reading {
        handler = find_handler(line, config_handlers_0, config_handlers_size);
    } else if (*config).active {
        handler = find_handler(line, command_handlers_0, command_handlers_size);
    }
    return if !handler.is_null() {
        handler
    } else {
        find_handler(line, handlers_0, handlers_size)
    };
}
unsafe extern "C" fn find_core_handler(
    mut line: *mut ::core::ffi::c_char,
) -> *const cmd_handler {
    return find_handler_ex(
        line,
        &raw const config_handlers as *const cmd_handler,
        ::core::mem::size_of::<[cmd_handler; 7]>(),
        &raw const command_handlers as *const cmd_handler,
        ::core::mem::size_of::<[cmd_handler; 28]>(),
        &raw const handlers as *const cmd_handler,
        ::core::mem::size_of::<[cmd_handler; 55]>(),
    );
}
unsafe extern "C" fn set_config_node(
    mut node: *mut sway_node,
    mut node_overridden: bool,
) {
    (*config).handler_context.node = node;
    (*config).handler_context.container = ::core::ptr::null_mut::<sway_container>();
    (*config).handler_context.workspace = ::core::ptr::null_mut::<sway_workspace>();
    (*config).handler_context.node_overridden = node_overridden;
    if node.is_null() {
        return;
    }
    match (*node).r#type {
        sway_node_type::N_CONTAINER => {
            (*config).handler_context.container = (*node).c2rust_unnamed.sway_container;
            (*config).handler_context.workspace = (*(*node)
                .c2rust_unnamed
                .sway_container)
                .pending
                .workspace;
        }
        sway_node_type::N_WORKSPACE => {
            (*config).handler_context.workspace = (*node).c2rust_unnamed.sway_workspace;
        }
        sway_node_type::N_ROOT | sway_node_type::N_OUTPUT | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn execute_command(
    mut _exec: *mut ::core::ffi::c_char,
    mut seat: *mut sway_seat,
    mut con: *mut sway_container,
) -> *mut list_t {
    let mut cmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut matched_delim: ::core::ffi::c_char = ';' as ::core::ffi::c_char;
    let mut containers: *mut list_t = ::core::ptr::null_mut::<list_t>();
    let mut using_criteria: bool = r#false != 0;
    if seat.is_null() {
        seat = input_manager_get_default_seat();
        if !_sway_assert(
            !seat.is_null(),
            b"[%s:%d] %s:could not find a seat to run the command on\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/commands.c\0".as_ptr() as *const ::core::ffi::c_char,
            215 as ::core::ffi::c_int,
            b"list_t *execute_command(char *, struct sway_seat *, struct sway_container *)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        ) {
            return ::core::ptr::null_mut::<list_t>();
        }
    }
    let mut exec: *mut ::core::ffi::c_char = strdup(_exec);
    let mut head: *mut ::core::ffi::c_char = exec;
    let mut res_list: *mut list_t = create_list();
    if res_list.is_null() || exec.is_null() {
        return ::core::ptr::null_mut::<list_t>();
    }
    (*config).handler_context.seat = seat;
    '_cleanup: loop {
        while *(*__ctype_b_loc()).offset(*head as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
        {
            head = head.offset(1);
        }
        if matched_delim as ::core::ffi::c_int == ';' as ::core::ffi::c_int {
            using_criteria = r#false != 0;
            if *head as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
                let mut error: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut criteria: *mut criteria = criteria_parse(head, &raw mut error);
                if criteria.is_null() {
                    list_add(
                        res_list,
                        cmd_results_new(
                            cmd_status::CMD_INVALID,
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            error,
                        ) as *mut ::core::ffi::c_void,
                    );
                    free(error as *mut ::core::ffi::c_void);
                    break;
                } else {
                    list_free(containers);
                    containers = criteria_get_containers(criteria);
                    head = head.offset(strlen((*criteria).raw) as isize);
                    criteria_destroy(criteria);
                    using_criteria = r#true != 0;
                    while *(*__ctype_b_loc())
                        .offset(*head as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int
                            as ::core::ffi::c_ushort as ::core::ffi::c_int != 0
                    {
                        head = head.offset(1);
                    }
                }
            }
        }
        cmd = argsep(
            &raw mut head,
            b";,\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut matched_delim,
        );
        while *(*__ctype_b_loc()).offset(*cmd as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
        {
            cmd = cmd.offset(1);
        }
        if strcmp(cmd, b"\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            _sway_log(
                sway_log_importance_t::SWAY_INFO,
                b"[%s:%d] Ignoring empty command.\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/commands.c\0".as_ptr() as *const ::core::ffi::c_char,
                258 as ::core::ffi::c_int,
            );
        } else {
            _sway_log(
                sway_log_importance_t::SWAY_INFO,
                b"[%s:%d] Handling command '%s'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/commands.c\0".as_ptr() as *const ::core::ffi::c_char,
                261 as ::core::ffi::c_int,
                cmd,
            );
            let mut argc: ::core::ffi::c_int = 0;
            let mut argv: *mut *mut ::core::ffi::c_char = split_args(cmd, &raw mut argc);
            if strcmp(
                *argv.offset(0isize),
                b"exec\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
                && strcmp(
                    *argv.offset(0isize),
                    b"exec_always\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                && strcmp(
                    *argv.offset(0isize),
                    b"mode\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
            {
                let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while i < argc {
                    if **argv.offset(i as isize) as ::core::ffi::c_int
                        == '"' as ::core::ffi::c_int
                        || **argv.offset(i as isize) as ::core::ffi::c_int
                            == '\'' as ::core::ffi::c_int
                    {
                        strip_quotes(*argv.offset(i as isize));
                    }
                    i += 1;
                }
            }
            let mut handler: *const cmd_handler = find_core_handler(
                *argv.offset(0isize),
            );
            if handler.is_null() {
                list_add(
                    res_list,
                    cmd_results_new(
                        cmd_status::CMD_INVALID,
                        b"Unknown/invalid command '%s'\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        *argv.offset(0isize),
                    ) as *mut ::core::ffi::c_void,
                );
                free_argv(argc, argv);
                break;
            } else {
                let mut i_0: ::core::ffi::c_int = if (*handler).handle
                    == Some(
                        cmd_set
                            as unsafe extern "C" fn(
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> *mut cmd_results,
                    )
                {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                };
                while i_0 < argc {
                    *argv.offset(i_0 as isize) = do_var_replacement(
                        *argv.offset(i_0 as isize),
                    );
                    i_0 += 1;
                }
                if !using_criteria {
                    if !con.is_null() {
                        set_config_node(&raw mut (*con).node, r#true != 0);
                    } else {
                        set_config_node(
                            seat_get_focus_inactive(seat, &raw mut (*root).node),
                            r#false != 0,
                        );
                    }
                    let mut res: *mut cmd_results = (*handler)
                        .handle
                        .expect(
                            "non-null function pointer",
                        )(
                        argc - 1 as ::core::ffi::c_int,
                        argv.offset(1 as ::core::ffi::c_int as isize),
                    );
                    list_add(res_list, res as *mut ::core::ffi::c_void);
                    if (*res).status.0 == cmd_status::CMD_INVALID.0 {
                        free_argv(argc, argv);
                        break;
                    }
                } else if (*containers).length == 0 as ::core::ffi::c_int {
                    list_add(
                        res_list,
                        cmd_results_new(
                            cmd_status::CMD_FAILURE,
                            b"No matching node.\0".as_ptr() as *const ::core::ffi::c_char,
                        ) as *mut ::core::ffi::c_void,
                    );
                } else {
                    let mut fail_res: *mut cmd_results = ::core::ptr::null_mut::<
                        cmd_results,
                    >();
                    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_1 < (*containers).length {
                        let mut container: *mut sway_container = *(*containers)
                            .items
                            .offset(i_1 as isize) as *mut sway_container;
                        set_config_node(&raw mut (*container).node, r#true != 0);
                        let mut res_0: *mut cmd_results = (*handler)
                            .handle
                            .expect(
                                "non-null function pointer",
                            )(
                            argc - 1 as ::core::ffi::c_int,
                            argv.offset(1 as ::core::ffi::c_int as isize),
                        );
                        if (*res_0).status.0 == cmd_status::CMD_SUCCESS.0 {
                            free_cmd_results(res_0);
                        } else {
                            if !fail_res.is_null() {
                                free_cmd_results(fail_res);
                            }
                            fail_res = res_0;
                            if (*res_0).status.0 == cmd_status::CMD_INVALID.0 {
                                list_add(res_list, fail_res as *mut ::core::ffi::c_void);
                                free_argv(argc, argv);
                                break '_cleanup;
                            }
                        }
                        i_1 += 1;
                    }
                    list_add(
                        res_list,
                        (if !fail_res.is_null() {
                            fail_res
                        } else {
                            cmd_results_new(
                                cmd_status::CMD_SUCCESS,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                            )
                        }) as *mut ::core::ffi::c_void,
                    );
                }
                free_argv(argc, argv);
            }
        }
        if head.is_null() {
            break;
        }
    }
    free(exec as *mut ::core::ffi::c_void);
    list_free(containers);
    return res_list;
}
#[no_mangle]
pub unsafe extern "C" fn config_command(
    mut exec: *mut ::core::ffi::c_char,
    mut new_block: *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    let mut handler: *const cmd_handler = ::core::ptr::null::<cmd_handler>();
    let mut command: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut results: *mut cmd_results = ::core::ptr::null_mut::<cmd_results>();
    let mut argc: ::core::ffi::c_int = 0;
    let mut argv: *mut *mut ::core::ffi::c_char = split_args(exec, &raw mut argc);
    '_cleanup: {
        if argc == 0 {
            results = cmd_results_new(
                cmd_status::CMD_SUCCESS,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        } else if argc > 1 as ::core::ffi::c_int
            && strcmp(
                *argv.offset((argc - 1 as ::core::ffi::c_int) as isize),
                b"{\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            *new_block = join_args(argv, argc - 1 as ::core::ffi::c_int);
            results = cmd_results_new(
                cmd_status::CMD_BLOCK,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        } else if strcmp(
            *argv.offset((argc - 1 as ::core::ffi::c_int) as isize),
            b"}\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            results = cmd_results_new(
                cmd_status::CMD_BLOCK_END,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        } else {
            if **argv.offset(0isize) as ::core::ffi::c_int == '$' as ::core::ffi::c_int {
                *argv.offset(0isize) = do_var_replacement(*argv.offset(0isize));
                let mut temp: *mut ::core::ffi::c_char = join_args(argv, argc);
                free_argv(argc, argv);
                argv = split_args(temp, &raw mut argc);
                free(temp as *mut ::core::ffi::c_void);
                if argc == 0 {
                    results = cmd_results_new(
                        cmd_status::CMD_SUCCESS,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                    break '_cleanup;
                }
            }
            _sway_log(
                sway_log_importance_t::SWAY_INFO,
                b"[%s:%d] Config command: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"sway/commands.c\0".as_ptr() as *const ::core::ffi::c_char,
                381 as ::core::ffi::c_int,
                exec,
            );
            handler = find_core_handler(*argv.offset(0isize));
            if handler.is_null() || (*handler).handle.is_none() {
                if !handler.is_null() {
                    results = cmd_results_new(
                        cmd_status::CMD_INVALID,
                        b"Command '%s' is shimmed, but unimplemented\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        *argv.offset(0isize),
                    );
                } else {
                    results = cmd_results_new(
                        cmd_status::CMD_INVALID,
                        b"Unknown/invalid command '%s'\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        *argv.offset(0isize),
                    );
                }
            } else {
                if (*handler).handle
                    == Some(
                        cmd_set
                            as unsafe extern "C" fn(
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> *mut cmd_results,
                    ) && argc > 1 as ::core::ffi::c_int
                    && **argv.offset(1isize) as ::core::ffi::c_int
                        == '$' as ::core::ffi::c_int
                {
                    let mut temp_0: *mut ::core::ffi::c_char = calloc(
                        1 as size_t,
                        strlen(*argv.offset(1isize)).wrapping_add(2 as size_t),
                    ) as *mut ::core::ffi::c_char;
                    *temp_0.offset(0isize) = '$' as ::core::ffi::c_char;
                    strcpy(temp_0.offset(1isize), *argv.offset(1isize));
                    free(*argv.offset(1isize) as *mut ::core::ffi::c_void);
                    *argv.offset(1isize) = temp_0;
                }
                command = do_var_replacement(join_args(argv, argc));
                _sway_log(
                    sway_log_importance_t::SWAY_INFO,
                    b"[%s:%d] After replacement: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/commands.c\0".as_ptr() as *const ::core::ffi::c_char,
                    404 as ::core::ffi::c_int,
                    command,
                );
                free_argv(argc, argv);
                argv = split_args(command, &raw mut argc);
                free(command as *mut ::core::ffi::c_void);
                let mut i: ::core::ffi::c_int = if (*handler).handle
                    == Some(
                        cmd_set
                            as unsafe extern "C" fn(
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> *mut cmd_results,
                    )
                {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                };
                while i < argc {
                    if (*handler).handle
                        != Some(
                            cmd_exec
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> *mut cmd_results,
                        )
                        && (*handler).handle
                            != Some(
                                cmd_exec_always
                                    as unsafe extern "C" fn(
                                        ::core::ffi::c_int,
                                        *mut *mut ::core::ffi::c_char,
                                    ) -> *mut cmd_results,
                            )
                        && (*handler).handle
                            != Some(
                                cmd_mode
                                    as unsafe extern "C" fn(
                                        ::core::ffi::c_int,
                                        *mut *mut ::core::ffi::c_char,
                                    ) -> *mut cmd_results,
                            )
                        && (*handler).handle
                            != Some(
                                cmd_bindsym
                                    as unsafe extern "C" fn(
                                        ::core::ffi::c_int,
                                        *mut *mut ::core::ffi::c_char,
                                    ) -> *mut cmd_results,
                            )
                        && (*handler).handle
                            != Some(
                                cmd_bindcode
                                    as unsafe extern "C" fn(
                                        ::core::ffi::c_int,
                                        *mut *mut ::core::ffi::c_char,
                                    ) -> *mut cmd_results,
                            )
                        && (*handler).handle
                            != Some(
                                cmd_bindswitch
                                    as unsafe extern "C" fn(
                                        ::core::ffi::c_int,
                                        *mut *mut ::core::ffi::c_char,
                                    ) -> *mut cmd_results,
                            )
                        && (*handler).handle
                            != Some(
                                cmd_bindgesture
                                    as unsafe extern "C" fn(
                                        ::core::ffi::c_int,
                                        *mut *mut ::core::ffi::c_char,
                                    ) -> *mut cmd_results,
                            )
                        && (*handler).handle
                            != Some(
                                cmd_set
                                    as unsafe extern "C" fn(
                                        ::core::ffi::c_int,
                                        *mut *mut ::core::ffi::c_char,
                                    ) -> *mut cmd_results,
                            )
                        && (*handler).handle
                            != Some(
                                cmd_for_window
                                    as unsafe extern "C" fn(
                                        ::core::ffi::c_int,
                                        *mut *mut ::core::ffi::c_char,
                                    ) -> *mut cmd_results,
                            )
                        && (**argv.offset(i as isize) as ::core::ffi::c_int
                            == '"' as ::core::ffi::c_int
                            || **argv.offset(i as isize) as ::core::ffi::c_int
                                == '\'' as ::core::ffi::c_int)
                    {
                        strip_quotes(*argv.offset(i as isize));
                    }
                    unescape_string(*argv.offset(i as isize));
                    i += 1;
                }
                results = (*handler)
                    .handle
                    .expect(
                        "non-null function pointer",
                    )(
                    argc - 1 as ::core::ffi::c_int,
                    argv.offset(1 as ::core::ffi::c_int as isize),
                ) as *mut cmd_results;
            }
        }
    }
    free_argv(argc, argv);
    return results;
}
#[no_mangle]
pub unsafe extern "C" fn config_subcommand(
    mut argv: *mut *mut ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut handlers_0: *const cmd_handler,
    mut handlers_size: size_t,
) -> *mut cmd_results {
    let mut command: *mut ::core::ffi::c_char = join_args(argv, argc);
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Subcommand: %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/commands.c\0".as_ptr() as *const ::core::ffi::c_char,
        436 as ::core::ffi::c_int,
        command,
    );
    free(command as *mut ::core::ffi::c_void);
    let mut handler: *const cmd_handler = find_handler(
        *argv.offset(0isize),
        handlers_0,
        handlers_size,
    );
    if handler.is_null() {
        return cmd_results_new(
            cmd_status::CMD_INVALID,
            b"Unknown/invalid command '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(0isize),
        );
    }
    if (*handler).handle.is_some() {
        return (*handler)
            .handle
            .expect(
                "non-null function pointer",
            )(
            argc - 1 as ::core::ffi::c_int,
            argv.offset(1 as ::core::ffi::c_int as isize),
        );
    }
    return cmd_results_new(
        cmd_status::CMD_INVALID,
        b"The command '%s' is shimmed, but unimplemented\0".as_ptr()
            as *const ::core::ffi::c_char,
        *argv.offset(0isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn config_commands_command(
    mut exec: *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    let mut cmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut handler: *const cmd_handler = ::core::ptr::null::<cmd_handler>();
    let mut results: *mut cmd_results = ::core::ptr::null_mut::<cmd_results>();
    let mut argc: ::core::ffi::c_int = 0;
    let mut argv: *mut *mut ::core::ffi::c_char = split_args(exec, &raw mut argc);
    if argc == 0 {
        results = cmd_results_new(
            cmd_status::CMD_SUCCESS,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    } else {
        cmd = *argv.offset(0isize);
        if strcmp(cmd, b"}\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            results = cmd_results_new(
                cmd_status::CMD_BLOCK_END,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        } else {
            handler = find_handler(cmd, ::core::ptr::null::<cmd_handler>(), 0 as size_t);
            if handler.is_null()
                && strcmp(cmd, b"*\0".as_ptr() as *const ::core::ffi::c_char)
                    != 0 as ::core::ffi::c_int
            {
                results = cmd_results_new(
                    cmd_status::CMD_INVALID,
                    b"Unknown/invalid command '%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cmd,
                );
            } else {
                results = cmd_results_new(
                    cmd_status::CMD_SUCCESS,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
    }
    free_argv(argc, argv);
    return results;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_results_new(
    mut status: cmd_status,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut cmd_results {
    let mut results: *mut cmd_results = malloc(::core::mem::size_of::<cmd_results>())
        as *mut cmd_results;
    if results.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to allocate command results\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/commands.c\0".as_ptr() as *const ::core::ffi::c_char,
            487 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<cmd_results>();
    }
    (*results).status = status;
    if !format.is_null() {
        let mut args: ::core::ffi::VaList<'_>;
        args = c2rust_args.clone();
        (*results).error = vformat_str(format, args.clone());
    } else {
        (*results).error = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return results;
}
#[no_mangle]
pub unsafe extern "C" fn free_cmd_results(mut results: *mut cmd_results) {
    if !(*results).error.is_null() {
        free((*results).error as *mut ::core::ffi::c_void);
    }
    free(results as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_results_to_json(
    mut res_list: *mut list_t,
) -> *mut ::core::ffi::c_char {
    let mut result_array: *mut json_object = json_object_new_array();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*res_list).length {
        let mut results: *mut cmd_results = *(*res_list).items.offset(i as isize)
            as *mut cmd_results;
        let mut root_0: *mut json_object = json_object_new_object();
        json_object_object_add(
            root_0,
            b"success\0".as_ptr() as *const ::core::ffi::c_char,
            json_object_new_boolean(
                ((*results).status.0 == cmd_status::CMD_SUCCESS.0) as ::core::ffi::c_int,
            ),
        );
        if !(*results).error.is_null() {
            json_object_object_add(
                root_0,
                b"parse_error\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_new_boolean(
                    ((*results).status.0 == cmd_status::CMD_INVALID.0)
                        as ::core::ffi::c_int,
                ),
            );
            json_object_object_add(
                root_0,
                b"error\0".as_ptr() as *const ::core::ffi::c_char,
                json_object_new_string((*results).error),
            );
        }
        json_object_array_add(result_array, root_0);
        i += 1;
    }
    let mut json: *const ::core::ffi::c_char = json_object_to_json_string(result_array);
    let mut res: *mut ::core::ffi::c_char = strdup(json);
    json_object_put(result_array);
    return res;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
