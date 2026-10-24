use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    pub type xkb_context;
    pub type xkb_keymap;
    pub type xkb_state;
    pub type wlr_keyboard_impl;
    pub type wlr_keyboard_group;
    pub type wlr_data_source;
    pub type wlr_drag;
    pub type wlr_primary_selection_source;
    pub type wlr_session;
    pub type wlr_linux_dmabuf_feedback_v1_compiled;
    pub type wlr_output_layer_feedback_event;
    pub type wlr_gamma_control_v1;
    pub type wlr_color_manager_v1;
    pub type wlr_gamma_control_manager_v1;
    pub type sway_output;
    pub type sway_transaction_instruction;
    pub type sway_text_node;
    pub type wlr_xwm;
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
    pub type sway_keyboard;
    pub type wlr_touch_impl;
    pub type sway_cursor;
    pub type _PangoFontDescription;
    pub type pcre2_real_code_8;
    pub type wlr_pointer_gestures_v1;
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
    pub type wlr_xwayland_shell_v1;
    pub type wlr_tablet_manager_v2;
    pub type wlr_xdg_shell;
    pub type wlr_idle_notifier_v1;
    pub type wlr_data_device_manager;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn getline(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    fn rewind(__stream: *mut FILE);
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn realpath(
        __name: *const ::core::ffi::c_char,
        __resolved: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn chdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn getcwd(
        __buf: *mut ::core::ffi::c_char,
        __size: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn dirname(__path: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn wordexp(
        __words: *const ::core::ffi::c_char,
        __pwordexp: *mut wordexp_t,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn wordfree(__wordexp: *mut wordexp_t);
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn wl_client_destroy(client: *mut wl_client);
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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strip_whitespace(str: *mut ::core::ffi::c_char);
    fn lenient_strcmp(
        a: *const ::core::ffi::c_char,
        b: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn vformat_str(
        fmt: *const ::core::ffi::c_char,
        args: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn format_str(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
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
    fn list_free_items_and_destroy(list: *mut list_t);
    fn swaynag_log(
        swaynag_command: *const ::core::ffi::c_char,
        swaynag: *mut swaynag_instance,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn swaynag_show(swaynag: *mut swaynag_instance);
    fn xkb_context_new(flags: xkb_context_flags) -> *mut xkb_context;
    fn xkb_context_unref(context: *mut xkb_context);
    fn xkb_keymap_new_from_names(
        context: *mut xkb_context,
        names: *const xkb_rule_names,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;
    fn xkb_keymap_unref(keymap: *mut xkb_keymap);
    fn xkb_state_new(keymap: *mut xkb_keymap) -> *mut xkb_state;
    fn xkb_state_unref(state: *mut xkb_state);
    fn xkb_state_get_keymap(state: *mut xkb_state) -> *mut xkb_keymap;
    fn input_manager_apply_input_config(input_config: *mut input_config);
    fn input_manager_reset_all_inputs();
    fn input_manager_apply_seat_config(seat_config: *mut seat_config);
    fn input_manager_get_seat(
        seat_name: *const ::core::ffi::c_char,
        create: bool,
    ) -> *mut sway_seat;
    fn input_manager_verify_fallback_seat();
    fn pango_font_description_from_string(
        str: *const ::core::ffi::c_char,
    ) -> *mut PangoFontDescription;
    static mut server: sway_server;
    fn sway_switch_retrigger_bindings_for_all();
    fn criteria_destroy(criteria: *mut criteria);
    fn execute_command(
        command: *mut ::core::ffi::c_char,
        seat: *mut sway_seat,
        con: *mut sway_container,
    ) -> *mut list_t;
    fn config_command(
        command: *mut ::core::ffi::c_char,
        new_block: *mut *mut ::core::ffi::c_char,
    ) -> *mut cmd_results;
    fn config_commands_command(exec: *mut ::core::ffi::c_char) -> *mut cmd_results;
    fn free_cmd_results(results: *mut cmd_results);
    fn arrange_root();
    fn seat_destroy(seat: *mut sway_seat);
    fn free_sway_variable(var: *mut sway_variable);
    fn input_config_fill_rule_names(ic: *mut input_config, rules: *mut xkb_rule_names);
    fn free_input_config(ic: *mut input_config);
    fn seat_name_cmp(
        item: *const ::core::ffi::c_void,
        data: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn free_seat_config(ic: *mut seat_config);
    fn free_output_config(oc: *mut output_config);
    fn request_modeset();
    fn spawn_swaybg() -> bool;
    fn free_sway_binding(sb: *mut sway_binding);
    fn free_switch_binding(binding: *mut sway_switch_binding);
    fn free_gesture_binding(binding: *mut sway_gesture_binding);
    fn seat_execute_command(seat: *mut sway_seat, binding: *mut sway_binding);
    fn free_bar_config(bar: *mut bar_config);
    fn free_workspace_config(wsc: *mut workspace_config);
    fn translate_binding(binding: *mut sway_binding) -> bool;
    fn binding_add_translated(binding: *mut sway_binding, bindings: *mut list_t);
    fn get_text_metrics(
        desc: *const PangoFontDescription,
        height: *mut ::core::ffi::c_int,
        baseline: *mut ::core::ffi::c_int,
    );
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_abort(filename: *const ::core::ffi::c_char, ...);
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    fn color_to_rgba(dest: *mut ::core::ffi::c_float, color: uint32_t);
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
pub type __gnuc_va_list = __builtin_va_list;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino64_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_int;
pub type __blkcnt64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
pub type va_list = __gnuc_va_list;
pub type ssize_t = isize;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type int16_t = i16;
pub type int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordexp_t {
    pub we_wordc: size_t,
    pub we_wordv: *mut *mut ::core::ffi::c_char,
    pub we_offs: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino64_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,
    pub __pad1: __dev_t,
    pub st_size: __off64_t,
    pub st_blksize: __blksize_t,
    pub __pad2: ::core::ffi::c_int,
    pub st_blocks: __blkcnt64_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [::core::ffi::c_int; 2],
}
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
    pub events: C2Rust_Unnamed_14,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub destroy: wl_signal,
}
pub type xkb_layout_index_t = uint32_t;
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
pub struct xkb_keymap_compile_flags(pub ::core::ffi::c_uint);
impl xkb_keymap_compile_flags {
    pub const XKB_KEYMAP_COMPILE_NO_FLAGS: Self = Self(0);
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
    pub repeat_info: C2Rust_Unnamed_16,
    pub events: C2Rust_Unnamed_15,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub rate: int32_t,
    pub delay: int32_t,
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
    pub events: C2Rust_Unnamed_17,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
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
    pub events: C2Rust_Unnamed_25,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub acc_discrete: [int32_t; 2],
    pub last_discrete: [int32_t; 2],
    pub acc_axis: [::core::ffi::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
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
    pub events: C2Rust_Unnamed_27,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
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
    pub events: C2Rust_Unnamed_29,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
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
    pub events: C2Rust_Unnamed_31,
    pub WLR_PRIVATE: C2Rust_Unnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
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
    pub events: C2Rust_Unnamed_33,
    pub WLR_PRIVATE: C2Rust_Unnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
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
pub struct wlr_keyboard_shortcuts_inhibit_manager_v1 {
    pub inhibitors: wl_list,
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
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_keyboard_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_keyboards: wl_list,
    pub events: C2Rust_Unnamed_37,
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
    pub new_virtual_keyboard: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_virtual_pointer_manager_v1 {
    pub global: *mut wl_global,
    pub virtual_pointers: wl_list,
    pub events: C2Rust_Unnamed_39,
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
    pub new_virtual_pointer: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_transient_seat_manager_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_41,
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
    pub destroy: wl_signal,
    pub create_seat: wl_signal,
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
    pub events: C2Rust_Unnamed_42,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct gesture_type(pub ::core::ffi::c_uint);
impl gesture_type {
    pub const GESTURE_TYPE_NONE: Self = Self(0);
    pub const GESTURE_TYPE_HOLD: Self = Self(1);
    pub const GESTURE_TYPE_PINCH: Self = Self(2);
    pub const GESTURE_TYPE_SWIPE: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gesture {
    pub r#type: gesture_type,
    pub fingers: uint8_t,
    pub directions: uint32_t,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_43,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_45,
    pub WLR_PRIVATE: C2Rust_Unnamed_44,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_44 {
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
pub struct C2Rust_Unnamed_45 {
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
    pub events: C2Rust_Unnamed_47,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_46,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
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
    pub events: C2Rust_Unnamed_49,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_50,
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
pub struct C2Rust_Unnamed_50 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_52,
    pub data: *mut ::core::ffi::c_void,
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
    pub events: C2Rust_Unnamed_54,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_53,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_53 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_54 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_55,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
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
    pub events: C2Rust_Unnamed_60,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_56,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_56 {
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
    pub events: C2Rust_Unnamed_58,
    pub WLR_PRIVATE: C2Rust_Unnamed_57,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_57 {
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
pub struct C2Rust_Unnamed_58 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_59,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_59 {
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
pub struct C2Rust_Unnamed_60 {
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
    pub layers: C2Rust_Unnamed_78,
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
    pub events: C2Rust_Unnamed_61,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_61 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_73,
    pub border: C2Rust_Unnamed_72,
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
    pub events: C2Rust_Unnamed_62,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_62 {
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
    pub layers: C2Rust_Unnamed_69,
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
    pub events: C2Rust_Unnamed_64,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_63,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_group_handle_v1 {
    pub manager: *mut wlr_ext_workspace_manager_v1,
    pub caps: uint32_t,
    pub events: C2Rust_Unnamed_66,
    pub link: wl_list,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_65,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_65 {
    pub outputs: wl_list,
    pub resources: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_ext_workspace_manager_v1 {
    pub global: *mut wl_global,
    pub groups: wl_list,
    pub workspaces: wl_list,
    pub events: C2Rust_Unnamed_68,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_67,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
    pub resources: wl_list,
    pub idle_source: *mut wl_event_source,
    pub event_loop: *mut wl_event_loop,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
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
pub struct side_gaps {
    pub top: ::core::ffi::c_int,
    pub right: ::core::ffi::c_int,
    pub bottom: ::core::ffi::c_int,
    pub left: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
    pub tiling: *mut wlr_scene_tree,
    pub fullscreen: *mut wlr_scene_tree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_node {
    pub r#type: sway_node_type,
    pub c2rust_unnamed: C2Rust_Unnamed_71,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_70,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_71 {
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
pub struct C2Rust_Unnamed_72 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_73 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_75,
    pub events: C2Rust_Unnamed_74,
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
pub struct C2Rust_Unnamed_74 {
    pub unmap: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_75 {
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
    pub events: C2Rust_Unnamed_77,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_76,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_76 {
    pub wm_name: *mut ::core::ffi::c_char,
    pub net_wm_name: *mut ::core::ffi::c_char,
    pub surface_commit: wl_listener,
    pub surface_map: wl_listener,
    pub surface_unmap: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_77 {
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
pub struct C2Rust_Unnamed_78 {
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
    pub events: C2Rust_Unnamed_80,
    pub WLR_PRIVATE: C2Rust_Unnamed_79,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_79 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_80 {
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
    pub events: C2Rust_Unnamed_82,
    pub WLR_PRIVATE: C2Rust_Unnamed_81,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_81 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_82 {
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
    pub events: C2Rust_Unnamed_83,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_83 {
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
pub type PangoFontDescription = _PangoFontDescription;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_variable {
    pub name: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
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
pub struct sway_switch_trigger(pub ::core::ffi::c_uint);
impl sway_switch_trigger {
    pub const SWAY_SWITCH_TRIGGER_OFF: Self = Self(0);
    pub const SWAY_SWITCH_TRIGGER_ON: Self = Self(1);
    pub const SWAY_SWITCH_TRIGGER_TOGGLE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_switch_binding {
    pub r#type: wlr_switch_type,
    pub trigger: sway_switch_trigger,
    pub flags: uint32_t,
    pub command: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_gesture_binding {
    pub input: *mut ::core::ffi::c_char,
    pub flags: uint32_t,
    pub gesture: gesture,
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
    pub xcursor_theme: C2Rust_Unnamed_84,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_84 {
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
    pub colors: C2Rust_Unnamed_85,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_85 {
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
    pub border_colors: C2Rust_Unnamed_88,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_86,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_86 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_87,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_87 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_88 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_path {
    pub prefix: *const ::core::ffi::c_char,
    pub config_folder: *const ::core::ffi::c_char,
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
    pub session_lock: C2Rust_Unnamed_95,
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
    pub new_object: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_text_input_manager_v3 {
    pub global: *mut wl_global,
    pub text_inputs: wl_list,
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
    pub new_text_input: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_input_method_manager_v2 {
    pub global: *mut wl_global,
    pub input_methods: wl_list,
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
    pub new_input_method: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_95 {
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
    pub events: C2Rust_Unnamed_98,
    pub user_event_handler: Option<
        unsafe extern "C" fn(*mut wlr_xwayland, *mut xcb_generic_event_t) -> bool,
    >,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_96,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_96 {
    pub cursor_buffer: *mut wlr_buffer,
    pub cursor_hotspot: C2Rust_Unnamed_97,
    pub server_start: wl_listener,
    pub server_ready: wl_listener,
    pub server_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub shell_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_97 {
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
pub struct C2Rust_Unnamed_98 {
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
    pub events: C2Rust_Unnamed_100,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_99,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_99 {
    pub client_destroy: wl_listener,
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_100 {
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
    pub new_inhibitor: wl_signal,
    pub destroy: wl_signal,
}
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BTN_LEFT: ::core::ffi::c_int = 0x110 as ::core::ffi::c_int;
pub const BTN_RIGHT: ::core::ffi::c_int = 0x111 as ::core::ffi::c_int;
pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub static mut config: *mut sway_config = ::core::ptr::null_mut::<sway_config>();
unsafe extern "C" fn keysym_translation_state_create(
    mut rules: xkb_rule_names,
    mut context_flags: uint32_t,
) -> *mut xkb_state {
    let mut context: *mut xkb_context = xkb_context_new(
        xkb_context_flags(
            (context_flags
                | xkb_context_flags::XKB_CONTEXT_NO_SECURE_GETENV.0 as ::core::ffi::c_int
                    as uint32_t) as ::core::ffi::c_uint,
        ),
    );
    let mut xkb_keymap: *mut xkb_keymap = xkb_keymap_new_from_names(
        context,
        &raw mut rules,
        xkb_keymap_compile_flags::XKB_KEYMAP_COMPILE_NO_FLAGS,
    );
    xkb_context_unref(context);
    if xkb_keymap.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to compile keysym translation XKB keymap\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            49 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<xkb_state>();
    }
    return xkb_state_new(xkb_keymap);
}
unsafe extern "C" fn keysym_translation_state_destroy(mut state: *mut xkb_state) {
    if state.is_null() {
        return;
    }
    xkb_keymap_unref(xkb_state_get_keymap(state));
    xkb_state_unref(state);
}
unsafe extern "C" fn free_mode(mut mode: *mut sway_mode) {
    if mode.is_null() {
        return;
    }
    free((*mode).name as *mut ::core::ffi::c_void);
    if !(*mode).keysym_bindings.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*mode).keysym_bindings).length {
            free_sway_binding(
                *(*(*mode).keysym_bindings).items.offset(i as isize) as *mut sway_binding,
            );
            i += 1;
        }
        list_free((*mode).keysym_bindings);
    }
    if !(*mode).keycode_bindings.is_null() {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*(*mode).keycode_bindings).length {
            free_sway_binding(
                *(*(*mode).keycode_bindings).items.offset(i_0 as isize)
                    as *mut sway_binding,
            );
            i_0 += 1;
        }
        list_free((*mode).keycode_bindings);
    }
    if !(*mode).mouse_bindings.is_null() {
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < (*(*mode).mouse_bindings).length {
            free_sway_binding(
                *(*(*mode).mouse_bindings).items.offset(i_1 as isize)
                    as *mut sway_binding,
            );
            i_1 += 1;
        }
        list_free((*mode).mouse_bindings);
    }
    if !(*mode).switch_bindings.is_null() {
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 < (*(*mode).switch_bindings).length {
            free_switch_binding(
                *(*(*mode).switch_bindings).items.offset(i_2 as isize)
                    as *mut sway_switch_binding,
            );
            i_2 += 1;
        }
        list_free((*mode).switch_bindings);
    }
    if !(*mode).gesture_bindings.is_null() {
        let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_3 < (*(*mode).gesture_bindings).length {
            free_gesture_binding(
                *(*(*mode).gesture_bindings).items.offset(i_3 as isize)
                    as *mut sway_gesture_binding,
            );
            i_3 += 1;
        }
        list_free((*mode).gesture_bindings);
    }
    free(mode as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn free_config(mut config_0: *mut sway_config) {
    if config_0.is_null() {
        return;
    }
    memset(
        &raw mut (*config_0).handler_context as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<C2Rust_Unnamed_86>(),
    );
    if !(*config_0).symbols.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*config_0).symbols).length {
            free_sway_variable(
                *(*(*config_0).symbols).items.offset(i as isize) as *mut sway_variable,
            );
            i += 1;
        }
        list_free((*config_0).symbols);
    }
    if !(*config_0).modes.is_null() {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*(*config_0).modes).length {
            free_mode(
                *(*(*config_0).modes).items.offset(i_0 as isize) as *mut sway_mode,
            );
            i_0 += 1;
        }
        list_free((*config_0).modes);
    }
    if !(*config_0).bars.is_null() {
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < (*(*config_0).bars).length {
            free_bar_config(
                *(*(*config_0).bars).items.offset(i_1 as isize) as *mut bar_config,
            );
            i_1 += 1;
        }
        list_free((*config_0).bars);
    }
    list_free((*config_0).cmd_queue);
    if !(*config_0).workspace_configs.is_null() {
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 < (*(*config_0).workspace_configs).length {
            free_workspace_config(
                *(*(*config_0).workspace_configs).items.offset(i_2 as isize)
                    as *mut workspace_config,
            );
            i_2 += 1;
        }
        list_free((*config_0).workspace_configs);
    }
    if !(*config_0).output_configs.is_null() {
        let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_3 < (*(*config_0).output_configs).length {
            free_output_config(
                *(*(*config_0).output_configs).items.offset(i_3 as isize)
                    as *mut output_config,
            );
            i_3 += 1;
        }
        list_free((*config_0).output_configs);
    }
    if !(*config_0).swaybg_client.is_null() {
        wl_client_destroy((*config_0).swaybg_client);
    }
    if !(*config_0).input_configs.is_null() {
        let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_4 < (*(*config_0).input_configs).length {
            free_input_config(
                *(*(*config_0).input_configs).items.offset(i_4 as isize)
                    as *mut input_config,
            );
            i_4 += 1;
        }
        list_free((*config_0).input_configs);
    }
    if !(*config_0).input_type_configs.is_null() {
        let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_5 < (*(*config_0).input_type_configs).length {
            free_input_config(
                *(*(*config_0).input_type_configs).items.offset(i_5 as isize)
                    as *mut input_config,
            );
            i_5 += 1;
        }
        list_free((*config_0).input_type_configs);
    }
    if !(*config_0).seat_configs.is_null() {
        let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_6 < (*(*config_0).seat_configs).length {
            free_seat_config(
                *(*(*config_0).seat_configs).items.offset(i_6 as isize)
                    as *mut seat_config,
            );
            i_6 += 1;
        }
        list_free((*config_0).seat_configs);
    }
    if !(*config_0).criteria.is_null() {
        let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_7 < (*(*config_0).criteria).length {
            criteria_destroy(
                *(*(*config_0).criteria).items.offset(i_7 as isize) as *mut criteria,
            );
            i_7 += 1;
        }
        list_free((*config_0).criteria);
    }
    list_free((*config_0).no_focus);
    list_free((*config_0).active_bar_modifiers);
    list_free_items_and_destroy((*config_0).config_chain);
    free((*config_0).floating_scroll_up_cmd as *mut ::core::ffi::c_void);
    free((*config_0).floating_scroll_down_cmd as *mut ::core::ffi::c_void);
    free((*config_0).floating_scroll_left_cmd as *mut ::core::ffi::c_void);
    free((*config_0).floating_scroll_right_cmd as *mut ::core::ffi::c_void);
    free((*config_0).font as *mut ::core::ffi::c_void);
    free((*config_0).swaybg_command as *mut ::core::ffi::c_void);
    free((*config_0).swaynag_command as *mut ::core::ffi::c_void);
    free(
        (*config_0).current_config_path as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
    );
    free(
        (*config_0).current_config as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
    );
    keysym_translation_state_destroy((*config_0).keysym_translation_state);
    free(config_0 as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn destroy_removed_seats(
    mut old_config: *mut sway_config,
    mut new_config: *mut sway_config,
) {
    let mut seat_config: *mut seat_config = ::core::ptr::null_mut::<seat_config>();
    let mut seat: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*(*old_config).seat_configs).length {
        seat_config = *(*(*old_config).seat_configs).items.offset(i as isize)
            as *mut seat_config;
        if strcmp((*seat_config).name, b"*\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            if !new_config.is_null()
                && list_seq_find(
                    (*new_config).seat_configs,
                    Some(
                        seat_name_cmp
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_void,
                                *const ::core::ffi::c_void,
                            ) -> ::core::ffi::c_int,
                    ),
                    (*seat_config).name as *const ::core::ffi::c_void,
                ) < 0 as ::core::ffi::c_int
            {
                seat = input_manager_get_seat((*seat_config).name, r#false != 0);
                if !seat.is_null() {
                    seat_destroy(seat);
                }
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn config_defaults(mut config_0: *mut sway_config) {
    let mut rules: xkb_rule_names = xkb_rule_names {
        rules: ::core::ptr::null::<::core::ffi::c_char>(),
        model: ::core::ptr::null::<::core::ffi::c_char>(),
        layout: ::core::ptr::null::<::core::ffi::c_char>(),
        variant: ::core::ptr::null::<::core::ffi::c_char>(),
        options: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    (*config_0).swaynag_command = strdup(
        b"swaynag\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !(*config_0).swaynag_command.is_null() {
        (*config_0).swaynag_config_errors = swaynag_instance {
            client: ::core::ptr::null_mut::<wl_client>(),
            client_destroy: wl_listener {
                link: wl_list {
                    prev: ::core::ptr::null_mut::<wl_list>(),
                    next: ::core::ptr::null_mut::<wl_list>(),
                },
                notify: None,
            },
            args: ::core::ptr::null::<::core::ffi::c_char>(),
            fd: [0; 2],
            detailed: false,
        };
        (*config_0).swaynag_config_errors.args = b"--type error --message 'There are errors in your config file' --detailed-message --button-no-terminal 'Exit sway' 'swaymsg exit' --button-no-terminal 'Reload sway' 'swaymsg reload'\0"
            .as_ptr() as *const ::core::ffi::c_char;
        (*config_0).swaynag_config_errors.detailed = r#true != 0;
        (*config_0).symbols = create_list();
        if !(*config_0).symbols.is_null() {
            (*config_0).modes = create_list();
            if !(*config_0).modes.is_null() {
                (*config_0).bars = create_list();
                if !(*config_0).bars.is_null() {
                    (*config_0).workspace_configs = create_list();
                    if !(*config_0).workspace_configs.is_null() {
                        (*config_0).criteria = create_list();
                        if !(*config_0).criteria.is_null() {
                            (*config_0).no_focus = create_list();
                            if !(*config_0).no_focus.is_null() {
                                (*config_0).seat_configs = create_list();
                                if !(*config_0).seat_configs.is_null() {
                                    (*config_0).output_configs = create_list();
                                    if !(*config_0).output_configs.is_null() {
                                        (*config_0).input_type_configs = create_list();
                                        if !(*config_0).input_type_configs.is_null() {
                                            (*config_0).input_configs = create_list();
                                            if !(*config_0).input_configs.is_null() {
                                                (*config_0).cmd_queue = create_list();
                                                if !(*config_0).cmd_queue.is_null() {
                                                    (*config_0).current_mode = malloc(
                                                        ::core::mem::size_of::<sway_mode>(),
                                                    ) as *mut sway_mode;
                                                    if !(*config_0).current_mode.is_null() {
                                                        (*(*config_0).current_mode).name = malloc(
                                                            ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
                                                        ) as *mut ::core::ffi::c_char;
                                                        if !(*(*config_0).current_mode).name.is_null() {
                                                            strcpy(
                                                                (*(*config_0).current_mode).name,
                                                                b"default\0".as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                            (*(*config_0).current_mode).keysym_bindings = create_list();
                                                            if !(*(*config_0).current_mode).keysym_bindings.is_null() {
                                                                (*(*config_0).current_mode).keycode_bindings = create_list();
                                                                if !(*(*config_0).current_mode).keycode_bindings.is_null() {
                                                                    (*(*config_0).current_mode).mouse_bindings = create_list();
                                                                    if !(*(*config_0).current_mode).mouse_bindings.is_null() {
                                                                        (*(*config_0).current_mode).switch_bindings = create_list();
                                                                        if !(*(*config_0).current_mode).switch_bindings.is_null() {
                                                                            (*(*config_0).current_mode).gesture_bindings = create_list();
                                                                            if !(*(*config_0).current_mode).gesture_bindings.is_null() {
                                                                                list_add(
                                                                                    (*config_0).modes,
                                                                                    (*config_0).current_mode as *mut ::core::ffi::c_void,
                                                                                );
                                                                                (*config_0).floating_mod = 0 as uint32_t;
                                                                                (*config_0).floating_mod_inverse = r#false != 0;
                                                                                (*config_0).dragging_key = BTN_LEFT as uint32_t;
                                                                                (*config_0).resizing_key = BTN_RIGHT as uint32_t;
                                                                                (*config_0).floating_scroll_up_cmd = strdup(
                                                                                    b"\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                );
                                                                                if !(*config_0).floating_scroll_up_cmd.is_null() {
                                                                                    (*config_0).floating_scroll_down_cmd = strdup(
                                                                                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                    );
                                                                                    if !(*config_0).floating_scroll_down_cmd.is_null() {
                                                                                        (*config_0).floating_scroll_left_cmd = strdup(
                                                                                            b"\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                        );
                                                                                        if !(*config_0).floating_scroll_left_cmd.is_null() {
                                                                                            (*config_0).floating_scroll_right_cmd = strdup(
                                                                                                b"\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                            );
                                                                                            if !(*config_0).floating_scroll_right_cmd.is_null() {
                                                                                                (*config_0).default_layout = sway_container_layout(
                                                                                                    sway_container_layout_0::L_NONE.0 as ::core::ffi::c_int
                                                                                                        as ::core::ffi::c_uint,
                                                                                                );
                                                                                                (*config_0).default_orientation = sway_container_layout(
                                                                                                    sway_container_layout_0::L_NONE.0 as ::core::ffi::c_int
                                                                                                        as ::core::ffi::c_uint,
                                                                                                );
                                                                                                (*config_0).font = strdup(
                                                                                                    b"monospace 10\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                );
                                                                                                if !(*config_0).font.is_null() {
                                                                                                    (*config_0).font_description = pango_font_description_from_string(
                                                                                                        (*config_0).font,
                                                                                                    );
                                                                                                    (*config_0).urgent_timeout = 500 as size_t;
                                                                                                    (*config_0).focus_on_window_activation = sway_fowa::FOWA_URGENT;
                                                                                                    (*config_0).popup_during_fullscreen = sway_popup_during_fullscreen::POPUP_SMART;
                                                                                                    (*config_0).xwayland = xwayland_mode::XWAYLAND_MODE_LAZY;
                                                                                                    (*config_0).titlebar_border_thickness = 1
                                                                                                        as ::core::ffi::c_int;
                                                                                                    (*config_0).titlebar_h_padding = 5 as ::core::ffi::c_int;
                                                                                                    (*config_0).titlebar_v_padding = 4 as ::core::ffi::c_int;
                                                                                                    (*config_0).floating_maximum_width = 0 as ::core::ffi::c_int
                                                                                                        as int32_t;
                                                                                                    (*config_0).floating_maximum_height = 0
                                                                                                        as ::core::ffi::c_int as int32_t;
                                                                                                    (*config_0).floating_minimum_width = 75
                                                                                                        as ::core::ffi::c_int as int32_t;
                                                                                                    (*config_0).floating_minimum_height = 50
                                                                                                        as ::core::ffi::c_int as int32_t;
                                                                                                    (*config_0).focus_follows_mouse = focus_follows_mouse_mode::FOLLOWS_YES;
                                                                                                    (*config_0).mouse_warping = mouse_warping_mode::WARP_OUTPUT;
                                                                                                    (*config_0).focus_wrapping = focus_wrapping_mode::WRAP_YES;
                                                                                                    (*config_0).validating = r#false != 0;
                                                                                                    (*config_0).reloading = r#false != 0;
                                                                                                    (*config_0).active = r#false != 0;
                                                                                                    (*config_0).failed = r#false != 0;
                                                                                                    (*config_0).auto_back_and_forth = r#false != 0;
                                                                                                    (*config_0).reading = r#false != 0;
                                                                                                    (*config_0).show_marks = r#true != 0;
                                                                                                    (*config_0).title_align = alignment::ALIGN_LEFT;
                                                                                                    (*config_0).tiling_drag = r#true != 0;
                                                                                                    (*config_0).tiling_drag_threshold = 9 as ::core::ffi::c_int;
                                                                                                    (*config_0).primary_selection = r#true != 0;
                                                                                                    (*config_0).smart_gaps = smart_gaps_mode::SMART_GAPS_OFF;
                                                                                                    (*config_0).gaps_inner = 0 as ::core::ffi::c_int;
                                                                                                    (*config_0).gaps_outer.top = 0 as ::core::ffi::c_int;
                                                                                                    (*config_0).gaps_outer.right = 0 as ::core::ffi::c_int;
                                                                                                    (*config_0).gaps_outer.bottom = 0 as ::core::ffi::c_int;
                                                                                                    (*config_0).gaps_outer.left = 0 as ::core::ffi::c_int;
                                                                                                    (*config_0).active_bar_modifiers = create_list();
                                                                                                    if !(*config_0).active_bar_modifiers.is_null() {
                                                                                                        (*config_0).swaybg_command = strdup(
                                                                                                            b"swaybg\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                        );
                                                                                                        if !(*config_0).swaybg_command.is_null() {
                                                                                                            (*config_0).config_chain = create_list();
                                                                                                            if !(*config_0).config_chain.is_null() {
                                                                                                                (*config_0).current_config_path = ::core::ptr::null::<
                                                                                                                    ::core::ffi::c_char,
                                                                                                                >();
                                                                                                                (*config_0).current_config = ::core::ptr::null::<
                                                                                                                    ::core::ffi::c_char,
                                                                                                                >();
                                                                                                                (*config_0).border = sway_container_border::B_NORMAL;
                                                                                                                (*config_0).floating_border = sway_container_border::B_NORMAL;
                                                                                                                (*config_0).border_thickness = 2 as ::core::ffi::c_int;
                                                                                                                (*config_0).floating_border_thickness = 2
                                                                                                                    as ::core::ffi::c_int;
                                                                                                                (*config_0).hide_edge_borders = edge_border_types::E_NONE;
                                                                                                                (*config_0).hide_edge_borders_smart = edge_border_smart_types::ESMART_OFF;
                                                                                                                (*config_0).hide_lone_tab = r#false != 0;
                                                                                                                (*config_0).has_focused_tab_title = r#false != 0;
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.focused.border
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x4c7899ff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.focused.background
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x285577ff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.focused.text
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0xffffffff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.focused.indicator
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x2e9ef4ff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.focused.child_border
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x285577ff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.focused_inactive.border
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x333333ff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0)
                                                                                                                        .border_colors
                                                                                                                        .focused_inactive
                                                                                                                        .background as *mut ::core::ffi::c_float,
                                                                                                                    0x5f676aff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.focused_inactive.text
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0xffffffff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0)
                                                                                                                        .border_colors
                                                                                                                        .focused_inactive
                                                                                                                        .indicator as *mut ::core::ffi::c_float,
                                                                                                                    0x484e50ff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0)
                                                                                                                        .border_colors
                                                                                                                        .focused_inactive
                                                                                                                        .child_border as *mut ::core::ffi::c_float,
                                                                                                                    0x5f676aff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.unfocused.border
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x333333ff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.unfocused.background
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x222222ff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.unfocused.text
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x888888ff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.unfocused.indicator
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x292d2eff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.unfocused.child_border
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x222222ff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.urgent.border
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x2f343aff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.urgent.background
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x900000ff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.urgent.text
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0xffffffff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.urgent.indicator
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x900000ff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.urgent.child_border
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0x900000ff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.placeholder.border
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0xff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.placeholder.background
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0xc0c0cff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.placeholder.text
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0xffffffff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.placeholder.indicator
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0xff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.placeholder.child_border
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0xc0c0cff as uint32_t,
                                                                                                                );
                                                                                                                color_to_rgba(
                                                                                                                    &raw mut (*config_0).border_colors.background
                                                                                                                        as *mut ::core::ffi::c_float,
                                                                                                                    0xffffffff as uint32_t,
                                                                                                                );
                                                                                                                rules = xkb_rule_names {
                                                                                                                    rules: ::core::ptr::null::<::core::ffi::c_char>(),
                                                                                                                    model: ::core::ptr::null::<::core::ffi::c_char>(),
                                                                                                                    layout: ::core::ptr::null::<::core::ffi::c_char>(),
                                                                                                                    variant: ::core::ptr::null::<::core::ffi::c_char>(),
                                                                                                                    options: ::core::ptr::null::<::core::ffi::c_char>(),
                                                                                                                };
                                                                                                                (*config_0).keysym_translation_state = keysym_translation_state_create(
                                                                                                                    rules,
                                                                                                                    0 as uint32_t,
                                                                                                                );
                                                                                                                if (*config_0).keysym_translation_state.is_null() {
                                                                                                                    (*config_0).keysym_translation_state = keysym_translation_state_create(
                                                                                                                        rules,
                                                                                                                        xkb_context_flags::XKB_CONTEXT_NO_ENVIRONMENT_NAMES.0
                                                                                                                            as ::core::ffi::c_int as uint32_t,
                                                                                                                    );
                                                                                                                }
                                                                                                                if !(*config_0).keysym_translation_state.is_null() {
                                                                                                                    return;
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    _sway_abort(
        b"[%s:%d] Unable to allocate config structures\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
        359 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn file_exists(mut path: *const ::core::ffi::c_char) -> bool {
    return !path.is_null() && access(path, R_OK) != -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn config_path(
    mut prefix: *const ::core::ffi::c_char,
    mut config_folder: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if prefix.is_null() || *prefix.offset(0isize) == 0 || config_folder.is_null()
        || *config_folder.offset(0isize) == 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return format_str(
        b"%s/%s/config\0".as_ptr() as *const ::core::ffi::c_char,
        prefix,
        config_folder,
    );
}
unsafe extern "C" fn get_config_path() -> *mut ::core::ffi::c_char {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut home: *const ::core::ffi::c_char = getenv(
        b"HOME\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut config_home_fallback: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut config_home: *const ::core::ffi::c_char = getenv(
        b"XDG_CONFIG_HOME\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if (config_home.is_null()
        || *config_home.offset(0isize) as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int) && !home.is_null()
    {
        config_home_fallback = format_str(
            b"%s/.config\0".as_ptr() as *const ::core::ffi::c_char,
            home,
        );
        config_home = config_home_fallback;
    }
    let mut config_paths: [config_path; 6] = [
        config_path {
            prefix: home,
            config_folder: b".sway\0".as_ptr() as *const ::core::ffi::c_char,
        },
        config_path {
            prefix: config_home,
            config_folder: b"sway\0".as_ptr() as *const ::core::ffi::c_char,
        },
        config_path {
            prefix: home,
            config_folder: b".i3\0".as_ptr() as *const ::core::ffi::c_char,
        },
        config_path {
            prefix: config_home,
            config_folder: b"i3\0".as_ptr() as *const ::core::ffi::c_char,
        },
        config_path {
            prefix: SYSCONFDIR.as_ptr() as *const ::core::ffi::c_char,
            config_folder: b"sway\0".as_ptr() as *const ::core::ffi::c_char,
        },
        config_path {
            prefix: SYSCONFDIR.as_ptr() as *const ::core::ffi::c_char,
            config_folder: b"i3\0".as_ptr() as *const ::core::ffi::c_char,
        },
    ];
    let mut num_config_paths: size_t = ::core::mem::size_of::<[config_path; 6]>()
        .wrapping_div(::core::mem::size_of::<config_path>());
    let mut i: size_t = 0 as size_t;
    while i < num_config_paths {
        path = config_path(config_paths[i].prefix, config_paths[i].config_folder);
        if !path.is_null() {
            if file_exists(path) {
                break;
            }
            free(path as *mut ::core::ffi::c_void);
            path = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        i = i.wrapping_add(1);
    }
    free(config_home_fallback as *mut ::core::ffi::c_void);
    return path;
}
unsafe extern "C" fn load_config(
    mut path: *const ::core::ffi::c_char,
    mut config_0: *mut sway_config,
    mut swaynag: *mut swaynag_instance,
) -> bool {
    if path.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to find a config file!\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            418 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    _sway_log(
        sway_log_importance_t::SWAY_INFO,
        b"[%s:%d] Loading config from %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
        422 as ::core::ffi::c_int,
        path,
    );
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        __pad1: 0,
        st_size: 0,
        st_blksize: 0,
        __pad2: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 2],
    };
    if stat(path, &raw mut sb) == 0 as ::core::ffi::c_int
        && sb.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] %s is a directory not a config file\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            426 as ::core::ffi::c_int,
            path,
        );
        return r#false != 0;
    }
    let mut f: *mut FILE = fopen(path, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    if f.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to open %s for reading\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            432 as ::core::ffi::c_int,
            path,
        );
        return r#false != 0;
    }
    let mut config_load_success: bool = read_config(f, config_0, swaynag);
    fclose(f);
    if !config_load_success {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Error(s) loading config!\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            440 as ::core::ffi::c_int,
        );
    }
    return (*config_0).active as ::core::ffi::c_int != 0 || !(*config_0).validating
        || config_load_success as ::core::ffi::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn load_main_config(
    mut file: *const ::core::ffi::c_char,
    mut is_active: bool,
    mut validating: bool,
) -> bool {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if !file.is_null() {
        path = strdup(file);
    } else {
        path = get_config_path();
    }
    if path.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Cannot find config file\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            454 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    let mut real_path: *mut ::core::ffi::c_char = realpath(
        path,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    if real_path.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] %s not found\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            460 as ::core::ffi::c_int,
            path,
        );
        free(path as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    let mut old_config: *mut sway_config = config;
    config = calloc(1 as size_t, ::core::mem::size_of::<sway_config>())
        as *mut sway_config;
    if config.is_null() {
        _sway_abort(
            b"[%s:%d] Unable to allocate config\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            468 as ::core::ffi::c_int,
        );
    }
    config_defaults(config);
    (*config).validating = validating;
    if is_active {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Performing configuration file %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            474 as ::core::ffi::c_int,
            if validating as ::core::ffi::c_int != 0 {
                b"validation\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"reload\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        (*config).reloading = r#true != 0;
        (*config).active = r#true != 0;
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] xwayland will remain %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            480 as ::core::ffi::c_int,
            if (*old_config).xwayland.0 != 0 {
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disabled\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        (*config).xwayland = (*old_config).xwayland;
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] primary_selection will remain %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            485 as ::core::ffi::c_int,
            if (*old_config).primary_selection as ::core::ffi::c_int != 0 {
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disabled\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        (*config).primary_selection = (*old_config).primary_selection;
        if !(*config).validating {
            if !(*old_config).swaybg_client.is_null() {
                wl_client_destroy((*old_config).swaybg_client);
            }
            if !(*old_config).swaynag_config_errors.client.is_null() {
                wl_client_destroy((*old_config).swaynag_config_errors.client);
            }
            input_manager_reset_all_inputs();
        }
    }
    (*config).user_config_path = if !file.is_null() { r#true } else { r#false } != 0;
    (*config).current_config_path = path;
    list_add((*config).config_chain, real_path as *mut ::core::ffi::c_void);
    (*config).reading = r#true != 0;
    let mut success: bool = load_config(
        path,
        config,
        &raw mut (*config).swaynag_config_errors,
    );
    if validating {
        free_config(config);
        config = old_config;
        return success;
    }
    config_update_font_height();
    if !validating {
        input_manager_verify_fallback_seat();
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*config).input_configs).length {
            input_manager_apply_input_config(
                *(*(*config).input_configs).items.offset(i as isize) as *mut input_config,
            );
            i += 1;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*(*config).input_type_configs).length {
            input_manager_apply_input_config(
                *(*(*config).input_type_configs).items.offset(i_0 as isize)
                    as *mut input_config,
            );
            i_0 += 1;
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < (*(*config).seat_configs).length {
            input_manager_apply_seat_config(
                *(*(*config).seat_configs).items.offset(i_1 as isize) as *mut seat_config,
            );
            i_1 += 1;
        }
        sway_switch_retrigger_bindings_for_all();
        spawn_swaybg();
        (*config).reloading = r#false != 0;
        if is_active {
            request_modeset();
            if !(*config).swaynag_config_errors.client.is_null() {
                swaynag_show(&raw mut (*config).swaynag_config_errors);
            }
        }
    }
    if !old_config.is_null() {
        destroy_removed_seats(old_config, config);
        free_config(old_config);
    }
    (*config).reading = r#false != 0;
    return success;
}
unsafe extern "C" fn load_include_config(
    mut path: *const ::core::ffi::c_char,
    mut config_0: *mut sway_config,
    mut swaynag: *mut swaynag_instance,
) -> bool {
    let mut parent_config: *const ::core::ffi::c_char = (*config_0).current_config_path;
    let mut real_path: *mut ::core::ffi::c_char = realpath(
        path,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    if real_path.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] %s not found.\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            563 as ::core::ffi::c_int,
            path,
        );
        return r#false != 0;
    }
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < (*(*config_0).config_chain).length {
        let mut old_path: *mut ::core::ffi::c_char = *(*(*config_0).config_chain)
            .items
            .offset(j as isize) as *mut ::core::ffi::c_char;
        if strcmp(real_path, old_path) == 0 as ::core::ffi::c_int {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] %s already included once, won't be included again.\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                572 as ::core::ffi::c_int,
                real_path,
            );
            free(real_path as *mut ::core::ffi::c_void);
            return r#false != 0;
        }
        j += 1;
    }
    (*config_0).current_config_path = real_path;
    list_add((*config_0).config_chain, real_path as *mut ::core::ffi::c_void);
    let mut index: ::core::ffi::c_int = (*(*config_0).config_chain).length
        - 1 as ::core::ffi::c_int;
    if !load_config(real_path, config_0, swaynag) {
        free(real_path as *mut ::core::ffi::c_void);
        (*config_0).current_config_path = parent_config;
        list_del((*config_0).config_chain, index);
        return r#false != 0;
    }
    (*config_0).current_config_path = parent_config;
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn load_include_configs(
    mut path: *const ::core::ffi::c_char,
    mut config_0: *mut sway_config,
    mut swaynag: *mut swaynag_instance,
) {
    let mut p: wordexp_t = wordexp_t {
        we_wordc: 0,
        we_wordv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        we_offs: 0,
    };
    let mut wd: *mut ::core::ffi::c_char = getcwd(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
    );
    let mut parent_path: *mut ::core::ffi::c_char = strdup(
        (*config_0).current_config_path,
    );
    let mut parent_dir: *const ::core::ffi::c_char = dirname(parent_path);
    if chdir(parent_dir) < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] failed to change working directory\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            603 as ::core::ffi::c_int,
        );
    } else {
        p = wordexp_t {
            we_wordc: 0,
            we_wordv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            we_offs: 0,
        };
        if wordexp(path, &raw mut p, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int
        {
            let mut w: *mut *mut ::core::ffi::c_char = p.we_wordv;
            let mut i: size_t = 0;
            i = 0 as size_t;
            while i < p.we_wordc {
                load_include_config(*w.offset(i as isize), config_0, swaynag);
                i = i.wrapping_add(1);
            }
            wordfree(&raw mut p);
        }
        if chdir(wd) < 0 as ::core::ffi::c_int {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] failed to change working directory\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                619 as ::core::ffi::c_int,
            );
        }
    }
    free(parent_path as *mut ::core::ffi::c_void);
    free(wd as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn run_deferred_commands() {
    if (*(*config).cmd_queue).length == 0 {
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Running deferred commands\0".as_ptr() as *const ::core::ffi::c_char,
        b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
        630 as ::core::ffi::c_int,
    );
    while (*(*config).cmd_queue).length != 0 {
        let mut line: *mut ::core::ffi::c_char = *(*(*config).cmd_queue)
            .items
            .offset(0isize) as *mut ::core::ffi::c_char;
        let mut res_list: *mut list_t = execute_command(
            line,
            ::core::ptr::null_mut::<sway_seat>(),
            ::core::ptr::null_mut::<sway_container>(),
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*res_list).length {
            let mut res: *mut cmd_results = *(*res_list).items.offset(i as isize)
                as *mut cmd_results;
            if (*res).status.0 != cmd_status::CMD_SUCCESS.0 {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] Error on line '%s': %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                    637 as ::core::ffi::c_int,
                    line,
                    (*res).error,
                );
            }
            free_cmd_results(res);
            i += 1;
        }
        list_del((*config).cmd_queue, 0 as ::core::ffi::c_int);
        list_free(res_list);
        free(line as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn run_deferred_bindings() {
    let mut seat: *mut sway_seat = ::core::ptr::null_mut::<sway_seat>();
    seat = ((*server.input).seats.next as *mut ::core::ffi::c_char)
        .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
        as *mut sway_seat;
    while &raw mut (*seat).link != &raw mut (*server.input).seats {
        if (*(*seat).deferred_bindings).length != 0 {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Running deferred bindings for seat %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                654 as ::core::ffi::c_int,
                (*(*seat).wlr_seat).name,
            );
            while (*(*seat).deferred_bindings).length != 0 {
                let mut binding: *mut sway_binding = *(*(*seat).deferred_bindings)
                    .items
                    .offset(0isize) as *mut sway_binding;
                seat_execute_command(seat, binding);
                list_del((*seat).deferred_bindings, 0 as ::core::ffi::c_int);
                free_sway_binding(binding);
            }
        }
        seat = ((*seat).link.next as *mut ::core::ffi::c_char)
            .offset(-(624 as ::core::ffi::c_ulong as isize)) as *mut sway_seat
            as *mut sway_seat;
    }
}
unsafe extern "C" fn getline_with_cont(
    mut lineptr: *mut *mut ::core::ffi::c_char,
    mut line_size: *mut size_t,
    mut file: *mut FILE,
    mut nlines: *mut ::core::ffi::c_int,
) -> ssize_t {
    let mut next_line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut next_line_size: size_t = 0 as size_t;
    let mut nread: ssize_t = getline(lineptr, line_size, file) as ssize_t;
    *nlines = if nread == -1 as ssize_t {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    while nread >= 2 as ssize_t
        && strcmp(
            (*lineptr).offset(nread - 2 as ssize_t),
            b"\\\n\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && *(*lineptr).offset(0isize) as ::core::ffi::c_int != '#' as ::core::ffi::c_int
    {
        let mut next_nread: ssize_t = getline(
            &raw mut next_line,
            &raw mut next_line_size,
            file,
        ) as ssize_t;
        if next_nread == -1 as ssize_t {
            break;
        }
        *nlines += 1;
        nread += next_nread - 2 as ssize_t;
        if (*line_size as ssize_t) < nread + 1 as ssize_t {
            *line_size = (nread + 1 as ssize_t) as size_t;
            let mut old_ptr: *mut ::core::ffi::c_char = *lineptr;
            *lineptr = realloc(*lineptr as *mut ::core::ffi::c_void, *line_size)
                as *mut ::core::ffi::c_char;
            if (*lineptr).is_null() {
                free(old_ptr as *mut ::core::ffi::c_void);
                nread = -1 as ssize_t;
                break;
            }
        }
        strcpy((*lineptr).offset(nread - next_nread), next_line);
    }
    free(next_line as *mut ::core::ffi::c_void);
    return nread;
}
unsafe extern "C" fn detect_brace(mut file: *mut FILE) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut lines: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pos: ::core::ffi::c_long = ftell(file);
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut line_size: size_t = 0 as size_t;
    while getline(&raw mut line, &raw mut line_size, file) != -1 as __ssize_t {
        lines += 1;
        strip_whitespace(line);
        if *line == 0 {
            continue;
        }
        if strcmp(line, b"{\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            ret = lines;
        }
        break;
    }
    free(line as *mut ::core::ffi::c_void);
    if ret == 0 as ::core::ffi::c_int {
        fseek(file, pos, SEEK_SET);
    }
    return ret;
}
unsafe extern "C" fn expand_line(
    mut block: *const ::core::ffi::c_char,
    mut line: *const ::core::ffi::c_char,
    mut add_brace: bool,
) -> *mut ::core::ffi::c_char {
    let mut size: ::core::ffi::c_int = if !block.is_null() {
        strlen(block).wrapping_add(1 as size_t)
    } else {
        0 as size_t
    }
        .wrapping_add(strlen(line))
        .wrapping_add(
            (if add_brace as ::core::ffi::c_int != 0 {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as size_t,
        )
        .wrapping_add(1 as size_t) as ::core::ffi::c_int;
    let mut expanded: *mut ::core::ffi::c_char = calloc(1 as size_t, size as size_t)
        as *mut ::core::ffi::c_char;
    if expanded.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Cannot allocate expanded line buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            724 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    snprintf(
        expanded,
        size as size_t,
        b"%s%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        if !block.is_null() {
            block
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !block.is_null() {
            b" \0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        line,
        if add_brace as ::core::ffi::c_int != 0 {
            b" {\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return expanded;
}
#[no_mangle]
pub unsafe extern "C" fn read_config(
    mut file: *mut FILE,
    mut config_0: *mut sway_config,
    mut swaynag: *mut swaynag_instance,
) -> bool {
    let mut reading_main_config: bool = r#false != 0;
    let mut this_config: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut config_size: size_t = 0 as size_t;
    if (*config_0).current_config.is_null() {
        reading_main_config = r#true != 0;
        let mut ret_seek: ::core::ffi::c_int = fseek(
            file,
            0 as ::core::ffi::c_long,
            SEEK_END,
        );
        let mut ret_tell: ::core::ffi::c_long = ftell(file);
        if ret_seek == -1 as ::core::ffi::c_int || ret_tell == -1 as ::core::ffi::c_long
        {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Unable to get size of config file\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                743 as ::core::ffi::c_int,
            );
            return r#false != 0;
        }
        config_size = ret_tell as size_t;
        rewind(file);
        this_config = calloc(1 as size_t, config_size.wrapping_add(1 as size_t))
            as *mut ::core::ffi::c_char;
        (*config_0).current_config = this_config;
        if this_config.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Unable to allocate buffer for config contents\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                751 as ::core::ffi::c_int,
            );
            return r#false != 0;
        }
    }
    let mut success: bool = r#true != 0;
    let mut line_number: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut line_size: size_t = 0 as size_t;
    let mut nread: ssize_t = 0;
    let mut stack: *mut list_t = create_list();
    let mut read: size_t = 0 as size_t;
    let mut nlines: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        nread = getline_with_cont(
            &raw mut line,
            &raw mut line_size,
            file,
            &raw mut nlines,
        );
        if nread == -1 as ssize_t {
            break;
        }
        if reading_main_config {
            if read.wrapping_add(nread as size_t) > config_size {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] Config file changed during reading\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                    767 as ::core::ffi::c_int,
                );
                success = r#false != 0;
                break;
            } else {
                strcpy(this_config.offset(read as isize), line);
                read = read.wrapping_add(nread as size_t);
            }
        }
        if *line.offset(nread - 1 as ssize_t) as ::core::ffi::c_int
            == '\n' as ::core::ffi::c_int
        {
            *line.offset(nread - 1 as ssize_t) = '\0' as ::core::ffi::c_char;
        }
        line_number += nlines;
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Read line %d: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            781 as ::core::ffi::c_int,
            line_number,
            line,
        );
        strip_whitespace(line);
        if *line == 0
            || *line.offset(0isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int
        {
            continue;
        }
        let mut brace_detected: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize)
            as ::core::ffi::c_int != '{' as ::core::ffi::c_int
            && *line.offset(strlen(line).wrapping_sub(1 as size_t) as isize)
                as ::core::ffi::c_int != '}' as ::core::ffi::c_int
        {
            brace_detected = detect_brace(file);
            if brace_detected > 0 as ::core::ffi::c_int {
                line_number += brace_detected;
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Detected open brace on line %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                    792 as ::core::ffi::c_int,
                    line_number,
                );
            }
        }
        let mut block: *mut ::core::ffi::c_char = (if (*stack).length != 0 {
            *(*stack).items.offset(0isize)
        } else {
            NULL
        }) as *mut ::core::ffi::c_char;
        let mut expanded: *mut ::core::ffi::c_char = expand_line(
            block,
            line,
            brace_detected > 0 as ::core::ffi::c_int,
        );
        if expanded.is_null() {
            success = r#false != 0;
            break;
        } else {
            (*config_0).current_config_line_number = line_number;
            (*config_0).current_config_line = line;
            let mut res: *mut cmd_results = ::core::ptr::null_mut::<cmd_results>();
            let mut new_block: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            if !block.is_null()
                && strcmp(block, b"<commands>\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                res = config_commands_command(expanded);
            } else {
                res = config_command(expanded, &raw mut new_block);
            }
            match (*res).status {
                cmd_status::CMD_FAILURE | cmd_status::CMD_INVALID => {
                    _sway_log(
                        sway_log_importance_t::SWAY_ERROR,
                        b"[%s:%d] Error on line %i '%s': %s (%s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                        814 as ::core::ffi::c_int,
                        line_number,
                        line,
                        (*res).error,
                        (*config_0).current_config_path,
                    );
                    if !(*config_0).validating {
                        swaynag_log(
                            (*config_0).swaynag_command,
                            swaynag,
                            b"Error on line %i (%s) '%s': %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            line_number,
                            (*config_0).current_config_path,
                            line,
                            (*res).error,
                        );
                    }
                    success = r#false != 0;
                }
                cmd_status::CMD_DEFER => {
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Deferring command `%s'\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                        825 as ::core::ffi::c_int,
                        line,
                    );
                    list_add(
                        (*config_0).cmd_queue,
                        strdup(expanded) as *mut ::core::ffi::c_void,
                    );
                }
                cmd_status::CMD_BLOCK_COMMANDS => {
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Entering commands block\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                        830 as ::core::ffi::c_int,
                    );
                    list_insert(
                        stack,
                        0 as ::core::ffi::c_int,
                        b"<commands>\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                    );
                }
                cmd_status::CMD_BLOCK => {
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Entering block '%s'\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                        835 as ::core::ffi::c_int,
                        new_block,
                    );
                    list_insert(
                        stack,
                        0 as ::core::ffi::c_int,
                        strdup(new_block) as *mut ::core::ffi::c_void,
                    );
                    if strcmp(new_block, b"bar\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        (*config_0).current_bar = ::core::ptr::null_mut::<bar_config>();
                    }
                }
                cmd_status::CMD_BLOCK_END => {
                    if block.is_null() {
                        _sway_log(
                            sway_log_importance_t::SWAY_DEBUG,
                            b"[%s:%d] Unmatched '}' on line %i\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                            844 as ::core::ffi::c_int,
                            line_number,
                        );
                        success = r#false != 0;
                    } else {
                        if strcmp(block, b"bar\0".as_ptr() as *const ::core::ffi::c_char)
                            == 0 as ::core::ffi::c_int
                        {
                            (*config_0).current_bar = ::core::ptr::null_mut::<
                                bar_config,
                            >();
                        }
                        _sway_log(
                            sway_log_importance_t::SWAY_DEBUG,
                            b"[%s:%d] Exiting block '%s'\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                            852 as ::core::ffi::c_int,
                            block,
                        );
                        list_del(stack, 0 as ::core::ffi::c_int);
                        free(block as *mut ::core::ffi::c_void);
                        memset(
                            &raw mut (*config_0).handler_context
                                as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<C2Rust_Unnamed_86>(),
                        );
                    }
                }
                _ => {}
            }
            free(new_block as *mut ::core::ffi::c_void);
            free(expanded as *mut ::core::ffi::c_void);
            free_cmd_results(res);
        }
    }
    free(line as *mut ::core::ffi::c_void);
    list_free_items_and_destroy(stack);
    (*config_0).current_config_line_number = 0 as ::core::ffi::c_int;
    (*config_0).current_config_line = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn config_add_swaynag_warning(
    mut fmt: *mut ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    if (*config).reading as ::core::ffi::c_int != 0 && !(*config).validating {
        let mut args: ::core::ffi::VaList<'_>;
        args = c2rust_args.clone();
        let mut str: *mut ::core::ffi::c_char = vformat_str(fmt, args.clone());
        if str.is_null() {
            return;
        }
        swaynag_log(
            (*config).swaynag_command,
            &raw mut (*config).swaynag_config_errors,
            b"Warning on line %i (%s) '%s': %s\0".as_ptr() as *const ::core::ffi::c_char,
            (*config).current_config_line_number,
            (*config).current_config_path,
            (*config).current_config_line,
            str,
        );
        free(str as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_var_replacement(
    mut str: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    let mut find: *mut ::core::ffi::c_char = str;
    loop {
        find = strchr(find, '$' as ::core::ffi::c_int);
        if find.is_null() {
            break;
        }
        if find > str
            && *find.offset(-1isize) as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
        {
            if find == str.offset(1 as ::core::ffi::c_int as isize)
                || !(find > str.offset(1 as ::core::ffi::c_int as isize)
                    && *find.offset(-2isize) as ::core::ffi::c_int
                        == '\\' as ::core::ffi::c_int)
            {
                find = find.offset(1);
                continue;
            }
        }
        if *find.offset(1isize) as ::core::ffi::c_int == '$' as ::core::ffi::c_int {
            let mut length: size_t = strlen(
                find.offset(1 as ::core::ffi::c_int as isize),
            );
            memmove(
                find as *mut ::core::ffi::c_void,
                find.offset(1 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                length,
            );
            *find.offset(length as isize) = '\0' as ::core::ffi::c_char;
            find = find.offset(1);
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < (*(*config).symbols).length {
                let mut var: *mut sway_variable = *(*(*config).symbols)
                    .items
                    .offset(i as isize) as *mut sway_variable;
                if has_prefix(find, (*var).name) {
                    let mut vnlen: ::core::ffi::c_int = strlen((*var).name)
                        as ::core::ffi::c_int;
                    let mut vvlen: ::core::ffi::c_int = strlen((*var).value)
                        as ::core::ffi::c_int;
                    let mut newstr: *mut ::core::ffi::c_char = malloc(
                        strlen(str)
                            .wrapping_sub(vnlen as size_t)
                            .wrapping_add(vvlen as size_t)
                            .wrapping_add(1 as size_t),
                    ) as *mut ::core::ffi::c_char;
                    if newstr.is_null() {
                        _sway_log(
                            sway_log_importance_t::SWAY_ERROR,
                            b"[%s:%d] Unable to allocate replacement during variable expansion\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                            917 as ::core::ffi::c_int,
                        );
                        break;
                    } else {
                        let mut newptr: *mut ::core::ffi::c_char = newstr;
                        let mut offset: ::core::ffi::c_int = find.offset_from(str)
                            as ::core::ffi::c_int;
                        strncpy(newptr, str, offset as size_t);
                        newptr = newptr.offset(offset as isize);
                        memcpy(
                            newptr as *mut ::core::ffi::c_void,
                            (*var).value as *const ::core::ffi::c_void,
                            vvlen as size_t,
                        );
                        newptr = newptr.offset(vvlen as isize);
                        strcpy(newptr, find.offset(vnlen as isize));
                        free(str as *mut ::core::ffi::c_void);
                        str = newstr;
                        find = str.offset(offset as isize).offset(vvlen as isize);
                        break;
                    }
                } else {
                    i += 1;
                }
            }
            if i == (*(*config).symbols).length {
                find = find.offset(1);
            }
        }
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn workspace_output_cmp_workspace(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut wsa: *const workspace_config = a as *const workspace_config;
    let mut wsb: *const workspace_config = b as *const workspace_config;
    return lenient_strcmp((*wsa).workspace, (*wsb).workspace);
}
#[no_mangle]
pub unsafe extern "C" fn config_update_font_height() {
    let mut prev_max_height: ::core::ffi::c_int = (*config).font_height;
    get_text_metrics(
        (*config).font_description,
        &raw mut (*config).font_height,
        &raw mut (*config).font_baseline,
    );
    if (*config).font_height != prev_max_height {
        arrange_root();
    }
}
unsafe extern "C" fn translate_binding_list(
    mut bindings: *mut list_t,
    mut bindsyms: *mut list_t,
    mut bindcodes: *mut list_t,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*bindings).length {
        let mut binding: *mut sway_binding = *(*bindings).items.offset(i as isize)
            as *mut sway_binding;
        translate_binding(binding);
        match (*binding).r#type {
            binding_input_type::BINDING_KEYSYM => {
                binding_add_translated(binding, bindsyms);
            }
            binding_input_type::BINDING_KEYCODE => {
                binding_add_translated(binding, bindcodes);
            }
            _ => {
                _sway_assert(
                    false,
                    b"[%s:%d] %s:unexpected translated binding type: %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
                    975 as ::core::ffi::c_int,
                    b"void translate_binding_list(list_t *, list_t *, list_t *)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*binding).r#type.0,
                );
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn translate_keysyms(mut input_config: *mut input_config) {
    keysym_translation_state_destroy((*config).keysym_translation_state);
    let mut rules: xkb_rule_names = xkb_rule_names {
        rules: ::core::ptr::null::<::core::ffi::c_char>(),
        model: ::core::ptr::null::<::core::ffi::c_char>(),
        layout: ::core::ptr::null::<::core::ffi::c_char>(),
        variant: ::core::ptr::null::<::core::ffi::c_char>(),
        options: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    input_config_fill_rule_names(input_config, &raw mut rules);
    (*config).keysym_translation_state = keysym_translation_state_create(
        rules,
        0 as uint32_t,
    );
    if (*config).keysym_translation_state.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to create keysym translation XKB state for device '%s'\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            990 as ::core::ffi::c_int,
            (*input_config).identifier,
        );
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*config).modes).length {
        let mut mode: *mut sway_mode = *(*(*config).modes).items.offset(i as isize)
            as *mut sway_mode;
        let mut bindsyms: *mut list_t = create_list();
        let mut bindcodes: *mut list_t = create_list();
        translate_binding_list((*mode).keysym_bindings, bindsyms, bindcodes);
        translate_binding_list((*mode).keycode_bindings, bindsyms, bindcodes);
        list_free((*mode).keysym_bindings);
        list_free((*mode).keycode_bindings);
        (*mode).keysym_bindings = bindsyms;
        (*mode).keycode_bindings = bindcodes;
        i += 1;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Translated keysyms using config for device '%s'\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/config.c\0".as_ptr() as *const ::core::ffi::c_char,
        1011 as ::core::ffi::c_int,
        (*input_config).identifier,
    );
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const SYSCONFDIR: &str = concat!(env!("SWAY_SYSCONFDIR"), "\0");
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
