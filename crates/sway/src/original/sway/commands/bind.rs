extern "C" {
    pub type xkb_keymap;
    pub type xkb_state;
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
    pub type wlr_linux_dmabuf_feedback_v1_compiled;
    pub type wlr_color_manager_v1;
    pub type wlr_gamma_control_manager_v1;
    pub type sway_workspace;
    pub type sway_text_node;
    pub type sway_view;
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
    pub type wlr_touch_impl;
    pub type _PangoFontDescription;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn xkb_keysym_from_name(
        name: *const ::core::ffi::c_char,
        flags: xkb_keysym_flags,
    ) -> xkb_keysym_t;
    fn xkb_keymap_key_for_each(
        keymap: *mut xkb_keymap,
        iter: xkb_keymap_key_iter_t,
        data: *mut ::core::ffi::c_void,
    );
    fn xkb_state_get_keymap(state: *mut xkb_state) -> *mut xkb_keymap;
    fn xkb_state_key_get_one_sym(
        state: *mut xkb_state,
        key: xkb_keycode_t,
    ) -> xkb_keysym_t;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strip_quotes(str: *mut ::core::ffi::c_char);
    fn split_string(
        str: *const ::core::ffi::c_char,
        delims: *const ::core::ffi::c_char,
    ) -> *mut list_t;
    fn join_args(
        argv: *mut *mut ::core::ffi::c_char,
        argc: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn has_prefix(
        str: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    ) -> bool;
    fn create_list() -> *mut list_t;
    fn list_free(list: *mut list_t);
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn list_del(list: *mut list_t, index: ::core::ffi::c_int);
    fn list_qsort(
        list: *mut list_t,
        compare: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
    fn list_free_items_and_destroy(list: *mut list_t);
    fn checkarg(
        argc: ::core::ffi::c_int,
        name: *const ::core::ffi::c_char,
        r#type: expected_args,
        val: ::core::ffi::c_int,
    ) -> *mut cmd_results;
    fn execute_command(
        command: *mut ::core::ffi::c_char,
        seat: *mut sway_seat,
        con: *mut sway_container,
    ) -> *mut list_t;
    fn cmd_results_new(
        status: cmd_status,
        error: *const ::core::ffi::c_char,
        ...
    ) -> *mut cmd_results;
    fn free_cmd_results(results: *mut cmd_results);
    fn transaction_commit_dirty();
    fn config_add_swaynag_warning(fmt: *mut ::core::ffi::c_char, ...);
    static mut config: *mut sway_config;
    fn get_modifier_mask_by_name(name: *const ::core::ffi::c_char) -> uint32_t;
    fn node_at_coords(
        seat: *mut sway_seat,
        lx: ::core::ffi::c_double,
        ly: ::core::ffi::c_double,
        surface: *mut *mut wlr_surface,
        sx: *mut ::core::ffi::c_double,
        sy: *mut ::core::ffi::c_double,
    ) -> *mut sway_node;
    fn get_mouse_bindsym(
        name: *const ::core::ffi::c_char,
        error: *mut *mut ::core::ffi::c_char,
    ) -> uint32_t;
    fn get_mouse_bindcode(
        name: *const ::core::ffi::c_char,
        error: *mut *mut ::core::ffi::c_char,
    ) -> uint32_t;
    fn ipc_event_binding(binding: *mut sway_binding);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
}
pub type size_t = usize;
pub type int32_t = i32;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type xkb_keycode_t = uint32_t;
pub type xkb_keysym_t = uint32_t;
pub type xkb_layout_index_t = uint32_t;
pub type xkb_mod_index_t = uint32_t;
pub type xkb_mod_mask_t = uint32_t;
pub type xkb_led_index_t = uint32_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xkb_keysym_flags(pub ::core::ffi::c_uint);
impl xkb_keysym_flags {
    pub const XKB_KEYSYM_NO_FLAGS: Self = Self(0);
    pub const XKB_KEYSYM_CASE_INSENSITIVE: Self = Self(1);
}
pub type xkb_keymap_key_iter_t = Option<
    unsafe extern "C" fn(*mut xkb_keymap, xkb_keycode_t, *mut ::core::ffi::c_void) -> (),
>;
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
pub struct wlr_output_layout {
    pub outputs: wl_list,
    pub display: *mut wl_display,
    pub events: C2Rust_Unnamed_14,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub display_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub add: wl_signal,
    pub change: wl_signal,
    pub destroy: wl_signal,
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
    pub events: C2Rust_Unnamed_18,
    pub current_outputs: wl_list,
    pub addons: wlr_addon_set,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub role_resource_destroy: wl_listener,
    pub previous: C2Rust_Unnamed_17,
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
pub struct C2Rust_Unnamed_17 {
    pub scale: int32_t,
    pub transform: wl_output_transform,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub buffer_width: ::core::ffi::c_int,
    pub buffer_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
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
    pub viewport: C2Rust_Unnamed_19,
    pub cached_state_locks: size_t,
    pub cached_state_link: wl_list,
    pub synced: wl_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
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
    pub events: C2Rust_Unnamed_21,
    pub WLR_PRIVATE: C2Rust_Unnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub display_destroy: wl_listener,
    pub renderer_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub new_surface: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_cursor {
    pub state: *mut wlr_cursor_state,
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub events: C2Rust_Unnamed_22,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
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
pub struct wlr_switch_type(pub ::core::ffi::c_uint);
impl wlr_switch_type {
    pub const WLR_SWITCH_TYPE_LID: Self = Self(0);
    pub const WLR_SWITCH_TYPE_TABLET_MODE: Self = Self(1);
    pub const WLR_SWITCH_TYPE_KEYPAD_SLIDE: Self = Self(2);
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
    pub events: C2Rust_Unnamed_26,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub synced: wlr_surface_synced,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub destroy: wl_signal,
    pub new_popup: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_surface_v1_state {
    pub committed: uint32_t,
    pub anchor: uint32_t,
    pub exclusive_zone: int32_t,
    pub margin: C2Rust_Unnamed_27,
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
pub struct C2Rust_Unnamed_27 {
    pub top: int32_t,
    pub right: int32_t,
    pub bottom: int32_t,
    pub left: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_layer_shell_v1 {
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
    pub events: C2Rust_Unnamed_31,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
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
    pub layers: C2Rust_Unnamed_40,
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
    pub events: C2Rust_Unnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_37,
    pub border: C2Rust_Unnamed_36,
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
    pub events: C2Rust_Unnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {
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
pub struct C2Rust_Unnamed_36 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_37 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_39,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_39 {
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
pub struct C2Rust_Unnamed_40 {
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
    pub events: C2Rust_Unnamed_42,
    pub WLR_PRIVATE: C2Rust_Unnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_41 {
    pub seat_client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
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
    pub events: C2Rust_Unnamed_44,
    pub WLR_PRIVATE: C2Rust_Unnamed_43,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub keyboard_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_44 {
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
    pub repeat_info: C2Rust_Unnamed_46,
    pub events: C2Rust_Unnamed_45,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_45 {
    pub key: wl_signal,
    pub modifiers: wl_signal,
    pub keymap: wl_signal,
    pub repeat_info: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
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
    pub events: C2Rust_Unnamed_48,
    pub serials: wlr_serial_ringset,
    pub needs_touch_frame: bool,
    pub value120: C2Rust_Unnamed_47,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_47 {
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
pub struct C2Rust_Unnamed_48 {
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
    pub events: C2Rust_Unnamed_50,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_49,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
    pub display_destroy: wl_listener,
    pub selection_source_destroy: wl_listener,
    pub primary_selection_source_destroy: wl_listener,
    pub drag_source_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
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
    pub events: C2Rust_Unnamed_52,
    pub link: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_51,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
    pub surface_destroy: wl_listener,
    pub focus_surface_destroy: wl_listener,
    pub client_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_52 {
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
    pub events: C2Rust_Unnamed_54,
    pub WLR_PRIVATE: C2Rust_Unnamed_53,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_53 {
    pub keyboard_destroy: wl_listener,
    pub keyboard_keymap: wl_listener,
    pub keyboard_repeat_info: wl_listener,
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_54 {
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
    pub events: C2Rust_Unnamed_56,
    pub WLR_PRIVATE: C2Rust_Unnamed_55,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
    pub surface_destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_56 {
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
    pub events: C2Rust_Unnamed_57,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_57 {
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
    pub events: C2Rust_Unnamed_58,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_58 {
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
    pub previous: C2Rust_Unnamed_64,
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
    pub events: C2Rust_Unnamed_60,
    pub data: *mut ::core::ffi::c_void,
    pub WLR_PRIVATE: C2Rust_Unnamed_59,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_59 {
    pub surface_destroy: wl_listener,
    pub seat_destroy: wl_listener,
    pub synced: wlr_surface_synced,
    pub destroying: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_60 {
    pub set_region: wl_signal,
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_pointer_constraint_v1_state {
    pub committed: uint32_t,
    pub region: pixman_region32_t,
    pub cursor_hint: C2Rust_Unnamed_61,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_61 {
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
    pub new_constraint: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_64 {
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
    pub xcursor_theme: C2Rust_Unnamed_65,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_65 {
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
    pub colors: C2Rust_Unnamed_66,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_66 {
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
    pub border_colors: C2Rust_Unnamed_69,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_67,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_68,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
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
pub struct keycode_matches {
    pub keysym: xkb_keysym_t,
    pub keycode: xkb_keycode_t,
    pub count: ::core::ffi::c_int,
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
pub const XKB_KEY_NoSymbol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const XKB_KEYCODE_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const XKB_LAYOUT_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const XKB_KEYCODE_MAX: ::core::ffi::c_uint = (0xffffffff as ::core::ffi::c_uint)
    .wrapping_sub(1 as ::core::ffi::c_uint);
#[no_mangle]
pub static mut binding_order: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn free_sway_binding(mut binding: *mut sway_binding) {
    if binding.is_null() {
        return;
    }
    list_free_items_and_destroy((*binding).keys);
    list_free_items_and_destroy((*binding).syms);
    free((*binding).input as *mut ::core::ffi::c_void);
    free((*binding).command as *mut ::core::ffi::c_void);
    free(binding as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn free_switch_binding(mut binding: *mut sway_switch_binding) {
    if binding.is_null() {
        return;
    }
    free((*binding).command as *mut ::core::ffi::c_void);
    free(binding as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn binding_switch_compare(
    mut binding_a: *mut sway_switch_binding,
    mut binding_b: *mut sway_switch_binding,
) -> bool {
    if (*binding_a).r#type.0 != (*binding_b).r#type.0 {
        return r#false != 0;
    }
    if (*binding_a).trigger.0 != (*binding_b).trigger.0 {
        return r#false != 0;
    }
    if (*binding_a).flags
        & binding_flags::BINDING_LOCKED.0 as ::core::ffi::c_int as uint32_t
        != (*binding_b).flags
            & binding_flags::BINDING_LOCKED.0 as ::core::ffi::c_int as uint32_t
    {
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn binding_key_compare(
    mut binding_a: *mut sway_binding,
    mut binding_b: *mut sway_binding,
) -> bool {
    if strcmp((*binding_a).input, (*binding_b).input) != 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    if (*binding_a).r#type.0 != (*binding_b).r#type.0 {
        return r#false != 0;
    }
    let mut conflict_generating_flags: uint32_t = (binding_flags::BINDING_RELEASE.0
        as ::core::ffi::c_int | binding_flags::BINDING_BORDER.0 as ::core::ffi::c_int
        | binding_flags::BINDING_CONTENTS.0 as ::core::ffi::c_int
        | binding_flags::BINDING_TITLEBAR.0 as ::core::ffi::c_int
        | binding_flags::BINDING_LOCKED.0 as ::core::ffi::c_int
        | binding_flags::BINDING_INHIBITED.0 as ::core::ffi::c_int) as uint32_t;
    if (*binding_a).flags & conflict_generating_flags
        != (*binding_b).flags & conflict_generating_flags
    {
        return r#false != 0;
    }
    if (*binding_a).group != (*binding_b).group {
        return r#false != 0;
    }
    if (*binding_a).modifiers ^ (*binding_b).modifiers != 0 {
        return r#false != 0;
    }
    if (*(*binding_a).keys).length != (*(*binding_b).keys).length {
        return r#false != 0;
    }
    let mut keys_len: ::core::ffi::c_int = (*(*binding_a).keys).length;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < keys_len {
        let mut key_a: uint32_t = *(*(*(*binding_a).keys).items.offset(i as isize)
            as *mut uint32_t);
        let mut key_b: uint32_t = *(*(*(*binding_b).keys).items.offset(i as isize)
            as *mut uint32_t);
        if key_a != key_b {
            return r#false != 0;
        }
        i += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn key_qsort_cmp(
    mut keyp_a: *const ::core::ffi::c_void,
    mut keyp_b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut key_a: uint32_t = **(keyp_a as *mut *mut uint32_t);
    let mut key_b: uint32_t = **(keyp_b as *mut *mut uint32_t);
    return if key_a < key_b {
        -1 as ::core::ffi::c_int
    } else if key_a > key_b {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn identify_key(
    mut name: *const ::core::ffi::c_char,
    mut first_key: bool,
    mut key_val: *mut uint32_t,
    mut r#type: *mut binding_input_type,
) -> *mut cmd_results {
    if (*r#type).0 == binding_input_type::BINDING_MOUSECODE.0 {
        let mut message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut button: uint32_t = get_mouse_bindcode(name, &raw mut message);
        if button == 0 {
            if !message.is_null() {
                let mut error: *mut cmd_results = cmd_results_new(
                    cmd_status::CMD_INVALID,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    message,
                );
                free(message as *mut ::core::ffi::c_void);
                return error;
            } else {
                return cmd_results_new(
                    cmd_status::CMD_INVALID,
                    b"Unknown button code %s\0".as_ptr() as *const ::core::ffi::c_char,
                    name,
                )
            }
        }
        *key_val = button;
    } else if (*r#type).0 == binding_input_type::BINDING_MOUSESYM.0 {
        let mut message_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut button_0: uint32_t = get_mouse_bindsym(name, &raw mut message_0);
        if button_0 == 0 {
            if !message_0.is_null() {
                let mut error_0: *mut cmd_results = cmd_results_new(
                    cmd_status::CMD_INVALID,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    message_0,
                );
                free(message_0 as *mut ::core::ffi::c_void);
                return error_0;
            } else {
                return cmd_results_new(
                    cmd_status::CMD_INVALID,
                    b"Unknown button %s\0".as_ptr() as *const ::core::ffi::c_char,
                    name,
                )
            }
        }
        *key_val = button_0;
    } else if (*r#type).0 == binding_input_type::BINDING_KEYCODE.0 {
        if first_key {
            let mut message_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut button_1: uint32_t = get_mouse_bindcode(name, &raw mut message_1);
            free(message_1 as *mut ::core::ffi::c_void);
            if button_1 != 0 {
                *r#type = binding_input_type::BINDING_MOUSECODE;
                *key_val = button_1;
                return ::core::ptr::null_mut::<cmd_results>();
            }
        }
        let mut keycode: xkb_keycode_t = strtol(
            name,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        ) as xkb_keycode_t;
        if !(keycode <= XKB_KEYCODE_MAX as xkb_keycode_t) {
            if first_key {
                return cmd_results_new(
                    cmd_status::CMD_INVALID,
                    b"Invalid keycode or button code '%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    name,
                )
            } else {
                return cmd_results_new(
                    cmd_status::CMD_INVALID,
                    b"Invalid keycode '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    name,
                )
            }
        }
        *key_val = keycode as uint32_t;
    } else {
        if first_key {
            let mut message_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut button_2: uint32_t = get_mouse_bindsym(name, &raw mut message_2);
            if !message_2.is_null() {
                let mut error_1: *mut cmd_results = cmd_results_new(
                    cmd_status::CMD_INVALID,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    message_2,
                );
                free(message_2 as *mut ::core::ffi::c_void);
                return error_1;
            } else if button_2 != 0 {
                *r#type = binding_input_type::BINDING_MOUSESYM;
                *key_val = button_2;
                return ::core::ptr::null_mut::<cmd_results>();
            }
        }
        let mut keysym: xkb_keysym_t = xkb_keysym_from_name(
            name,
            xkb_keysym_flags::XKB_KEYSYM_CASE_INSENSITIVE,
        );
        if keysym == 0 {
            if first_key {
                return cmd_results_new(
                    cmd_status::CMD_INVALID,
                    b"Unknown key or button '%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    name,
                )
            } else {
                return cmd_results_new(
                    cmd_status::CMD_INVALID,
                    b"Unknown key '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    name,
                )
            }
        }
        *key_val = keysym as uint32_t;
    }
    return ::core::ptr::null_mut::<cmd_results>();
}
unsafe extern "C" fn switch_binding_add(
    mut binding: *mut sway_switch_binding,
    mut bindtype: *const ::core::ffi::c_char,
    mut switchcombo: *const ::core::ffi::c_char,
    mut warn: bool,
) -> *mut cmd_results {
    let mut mode_bindings: *mut list_t = (*(*config).current_mode).switch_bindings;
    let mut overwritten: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*mode_bindings).length {
        let mut config_binding: *mut sway_switch_binding = *(*mode_bindings)
            .items
            .offset(i as isize) as *mut sway_switch_binding;
        if binding_switch_compare(binding, config_binding) {
            _sway_log(
                sway_log_importance_t::SWAY_INFO,
                b"[%s:%d] Overwriting binding '%s' to `%s` from `%s`\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
                218 as ::core::ffi::c_int,
                switchcombo,
                (*binding).command,
                (*config_binding).command,
            );
            if warn {
                config_add_swaynag_warning(
                    b"Overwriting binding'%s' to `%s` from `%s`\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    switchcombo,
                    (*binding).command,
                    (*config_binding).command,
                );
            }
            free_switch_binding(config_binding);
            *(*mode_bindings).items.offset(i as isize) = binding
                as *mut ::core::ffi::c_void;
            overwritten = r#true != 0;
        }
        i += 1;
    }
    if !overwritten {
        list_add(mode_bindings, binding as *mut ::core::ffi::c_void);
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] %s - Bound %s to command `%s`\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
            234 as ::core::ffi::c_int,
            bindtype,
            switchcombo,
            (*binding).command,
        );
    }
    return cmd_results_new(
        cmd_status::CMD_SUCCESS,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn switch_binding_remove(
    mut binding: *mut sway_switch_binding,
    mut bindtype: *const ::core::ffi::c_char,
    mut switchcombo: *const ::core::ffi::c_char,
) -> *mut cmd_results {
    let mut mode_bindings: *mut list_t = (*(*config).current_mode).switch_bindings;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*mode_bindings).length {
        let mut config_binding: *mut sway_switch_binding = *(*mode_bindings)
            .items
            .offset(i as isize) as *mut sway_switch_binding;
        if binding_switch_compare(binding, config_binding) {
            free_switch_binding(config_binding);
            free_switch_binding(binding);
            list_del(mode_bindings, i);
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] %s - Unbound %s switch\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
                251 as ::core::ffi::c_int,
                bindtype,
                switchcombo,
            );
            return cmd_results_new(
                cmd_status::CMD_SUCCESS,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        i += 1;
    }
    free_switch_binding(binding);
    return cmd_results_new(
        cmd_status::CMD_FAILURE,
        b"Could not find switch binding `%s`\0".as_ptr() as *const ::core::ffi::c_char,
        switchcombo,
    );
}
unsafe extern "C" fn binding_upsert(
    mut binding: *mut sway_binding,
    mut mode_bindings: *mut list_t,
) -> *mut sway_binding {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*mode_bindings).length {
        let mut config_binding: *mut sway_binding = *(*mode_bindings)
            .items
            .offset(i as isize) as *mut sway_binding;
        if binding_key_compare(binding, config_binding) {
            *(*mode_bindings).items.offset(i as isize) = binding
                as *mut ::core::ffi::c_void;
            return config_binding;
        }
        i += 1;
    }
    list_add(mode_bindings, binding as *mut ::core::ffi::c_void);
    return ::core::ptr::null_mut::<sway_binding>();
}
unsafe extern "C" fn binding_add(
    mut binding: *mut sway_binding,
    mut mode_bindings: *mut list_t,
    mut bindtype: *const ::core::ffi::c_char,
    mut keycombo: *const ::core::ffi::c_char,
    mut warn: bool,
) -> *mut cmd_results {
    let mut config_binding: *mut sway_binding = binding_upsert(binding, mode_bindings);
    if !config_binding.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Overwriting binding '%s' for device '%s' to `%s` from `%s`\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
            286 as ::core::ffi::c_int,
            keycombo,
            (*binding).input,
            (*binding).command,
            (*config_binding).command,
        );
        if warn {
            config_add_swaynag_warning(
                b"Overwriting binding'%s' for device '%s' to `%s` from `%s`\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                keycombo,
                (*binding).input,
                (*binding).command,
                (*config_binding).command,
            );
        }
        free_sway_binding(config_binding);
    } else {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] %s - Bound %s to command `%s` for device '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
            297 as ::core::ffi::c_int,
            bindtype,
            keycombo,
            (*binding).command,
            (*binding).input,
        );
    }
    return cmd_results_new(
        cmd_status::CMD_SUCCESS,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn binding_remove(
    mut binding: *mut sway_binding,
    mut mode_bindings: *mut list_t,
    mut bindtype: *const ::core::ffi::c_char,
    mut keycombo: *const ::core::ffi::c_char,
) -> *mut cmd_results {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*mode_bindings).length {
        let mut config_binding: *mut sway_binding = *(*mode_bindings)
            .items
            .offset(i as isize) as *mut sway_binding;
        if binding_key_compare(binding, config_binding) {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] %s - Unbound `%s` from device '%s'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
                310 as ::core::ffi::c_int,
                bindtype,
                keycombo,
                (*binding).input,
            );
            free_sway_binding(config_binding);
            free_sway_binding(binding);
            list_del(mode_bindings, i);
            return cmd_results_new(
                cmd_status::CMD_SUCCESS,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        i += 1;
    }
    free_sway_binding(binding);
    return cmd_results_new(
        cmd_status::CMD_FAILURE,
        b"Could not find binding `%s` for the given flags\0".as_ptr()
            as *const ::core::ffi::c_char,
        keycombo,
    );
}
unsafe extern "C" fn cmd_bindsym_or_bindcode(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut bindcode: bool,
    mut unbind: bool,
) -> *mut cmd_results {
    let mut bindtype: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut minargs: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    if unbind {
        bindtype = if bindcode as ::core::ffi::c_int != 0 {
            b"unbindcode\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"unbindsym\0".as_ptr() as *const ::core::ffi::c_char
        };
        minargs -= 1;
    } else {
        bindtype = if bindcode as ::core::ffi::c_int != 0 {
            b"bindcode\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"bindsym\0".as_ptr() as *const ::core::ffi::c_char
        };
    }
    let mut error: *mut cmd_results = ::core::ptr::null_mut::<cmd_results>();
    error = checkarg(argc, bindtype, expected_args::EXPECTED_AT_LEAST, minargs);
    if !error.is_null() {
        return error;
    }
    let mut binding: *mut sway_binding = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_binding>(),
    ) as *mut sway_binding;
    if binding.is_null() {
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"Unable to allocate binding\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    (*binding).input = strdup(b"*\0".as_ptr() as *const ::core::ffi::c_char);
    (*binding).keys = create_list();
    (*binding).group = XKB_LAYOUT_INVALID as xkb_layout_index_t;
    (*binding).modifiers = 0 as uint32_t;
    (*binding).flags = 0 as uint32_t;
    (*binding).r#type = binding_input_type(
        (if bindcode as ::core::ffi::c_int != 0 {
            binding_input_type::BINDING_KEYCODE.0 as ::core::ffi::c_int
        } else {
            binding_input_type::BINDING_KEYSYM.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    let mut exclude_titlebar: bool = r#false != 0;
    let mut warn: bool = r#true != 0;
    while argc > 0 as ::core::ffi::c_int {
        if strcmp(
            b"--release\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(0isize),
        ) == 0 as ::core::ffi::c_int
        {
            (*binding).flags
                |= binding_flags::BINDING_RELEASE.0 as ::core::ffi::c_int as uint32_t;
        } else if strcmp(
            b"--locked\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(0isize),
        ) == 0 as ::core::ffi::c_int
        {
            (*binding).flags
                |= binding_flags::BINDING_LOCKED.0 as ::core::ffi::c_int as uint32_t;
        } else if strcmp(
            b"--inhibited\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(0isize),
        ) == 0 as ::core::ffi::c_int
        {
            (*binding).flags
                |= binding_flags::BINDING_INHIBITED.0 as ::core::ffi::c_int as uint32_t;
        } else if strcmp(
            b"--whole-window\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(0isize),
        ) == 0 as ::core::ffi::c_int
        {
            (*binding).flags
                |= (binding_flags::BINDING_BORDER.0 as ::core::ffi::c_int
                    | binding_flags::BINDING_CONTENTS.0 as ::core::ffi::c_int
                    | binding_flags::BINDING_TITLEBAR.0 as ::core::ffi::c_int)
                    as uint32_t;
        } else if strcmp(
            b"--border\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(0isize),
        ) == 0 as ::core::ffi::c_int
        {
            (*binding).flags
                |= binding_flags::BINDING_BORDER.0 as ::core::ffi::c_int as uint32_t;
        } else if strcmp(
            b"--to-code\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(0isize),
        ) == 0 as ::core::ffi::c_int
        {
            if !bindcode {
                (*binding).flags
                    |= binding_flags::BINDING_CODE.0 as ::core::ffi::c_int as uint32_t;
            }
        } else if strcmp(
            b"--exclude-titlebar\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(0isize),
        ) == 0 as ::core::ffi::c_int
        {
            exclude_titlebar = r#true != 0;
        } else if has_prefix(
            *argv.offset(0isize),
            b"--input-device=\0".as_ptr() as *const ::core::ffi::c_char,
        ) {
            free((*binding).input as *mut ::core::ffi::c_void);
            (*binding).input = strdup(
                (*argv.offset(0isize))
                    .offset(
                        strlen(
                            b"--input-device=\0".as_ptr() as *const ::core::ffi::c_char,
                        ) as isize,
                    ),
            );
            strip_quotes((*binding).input);
        } else if strcmp(
            b"--no-warn\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(0isize),
        ) == 0 as ::core::ffi::c_int
        {
            warn = r#false != 0;
        } else {
            if strcmp(
                b"--no-repeat\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0isize),
            ) != 0 as ::core::ffi::c_int
            {
                break;
            }
            (*binding).flags
                |= binding_flags::BINDING_NOREPEAT.0 as ::core::ffi::c_int as uint32_t;
        }
        argv = argv.offset(1);
        argc -= 1;
    }
    if (*binding).flags
        & (binding_flags::BINDING_BORDER.0 as ::core::ffi::c_int
            | binding_flags::BINDING_CONTENTS.0 as ::core::ffi::c_int
            | binding_flags::BINDING_TITLEBAR.0 as ::core::ffi::c_int) as uint32_t != 0
        || exclude_titlebar as ::core::ffi::c_int != 0
    {
        (*binding).r#type = binding_input_type(
            (if (*binding).r#type.0 == binding_input_type::BINDING_KEYCODE.0 {
                binding_input_type::BINDING_MOUSECODE.0 as ::core::ffi::c_int
            } else {
                binding_input_type::BINDING_MOUSESYM.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
    }
    if argc < minargs {
        free_sway_binding(binding);
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"Invalid %s command (expected at least %d non-option arguments, got %d)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            bindtype,
            minargs,
            argc,
        );
    }
    let mut split: *mut list_t = split_string(
        *argv.offset(0isize),
        b"+\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*split).length {
        if has_prefix(
            *(*split).items.offset(i as isize) as *const ::core::ffi::c_char,
            b"Group\0".as_ptr() as *const ::core::ffi::c_char,
        ) {
            if (*binding).group != XKB_LAYOUT_INVALID as xkb_layout_index_t {
                free_sway_binding(binding);
                list_free_items_and_destroy(split);
                return cmd_results_new(
                    cmd_status::CMD_FAILURE,
                    b"Only one group can be specified\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut group: ::core::ffi::c_int = strtol(
                (*(*split).items.offset(i as isize))
                    .offset(
                        strlen(b"Group\0".as_ptr() as *const ::core::ffi::c_char)
                            as isize,
                    ) as *const ::core::ffi::c_char,
                &raw mut end,
                10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
            if group < 1 as ::core::ffi::c_int || group > 4 as ::core::ffi::c_int
                || *end.offset(0isize) as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
            {
                free_sway_binding(binding);
                list_free_items_and_destroy(split);
                return cmd_results_new(
                    cmd_status::CMD_FAILURE,
                    b"Invalid group\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            (*binding).group = (group - 1 as ::core::ffi::c_int) as xkb_layout_index_t;
        } else {
            if strcmp(
                *(*split).items.offset(i as isize) as *const ::core::ffi::c_char,
                b"Mode_switch\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                if (*binding).group != XKB_LAYOUT_INVALID as xkb_layout_index_t {
                    free_sway_binding(binding);
                    list_free_items_and_destroy(split);
                    return cmd_results_new(
                        cmd_status::CMD_FAILURE,
                        b"Only one group can be specified\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                (*binding).group = 1 as xkb_layout_index_t;
            }
            let mut r#mod: uint32_t = 0;
            r#mod = get_modifier_mask_by_name(
                *(*split).items.offset(i as isize) as *const ::core::ffi::c_char,
            );
            if r#mod > 0 as uint32_t {
                (*binding).modifiers |= r#mod;
            } else {
                let mut key_val: uint32_t = 0 as uint32_t;
                error = identify_key(
                    *(*split).items.offset(i as isize) as *const ::core::ffi::c_char,
                    (*(*binding).keys).length == 0 as ::core::ffi::c_int,
                    &raw mut key_val,
                    &raw mut (*binding).r#type,
                );
                if !error.is_null() {
                    free_sway_binding(binding);
                    list_free(split);
                    return error;
                }
                let mut key: *mut uint32_t = calloc(
                    1 as size_t,
                    ::core::mem::size_of::<uint32_t>(),
                ) as *mut uint32_t;
                if key.is_null() {
                    free_sway_binding(binding);
                    list_free_items_and_destroy(split);
                    return cmd_results_new(
                        cmd_status::CMD_FAILURE,
                        b"Unable to allocate binding key\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                *key = key_val;
                list_add((*binding).keys, key as *mut ::core::ffi::c_void);
            }
        }
        i += 1;
    }
    list_free_items_and_destroy(split);
    if exclude_titlebar {
        (*binding).flags
            &= !(binding_flags::BINDING_TITLEBAR.0 as ::core::ffi::c_int) as uint32_t;
    } else if (*binding).r#type.0 == binding_input_type::BINDING_MOUSECODE.0
        || (*binding).r#type.0 == binding_input_type::BINDING_MOUSESYM.0
    {
        (*binding).flags
            |= binding_flags::BINDING_TITLEBAR.0 as ::core::ffi::c_int as uint32_t;
    }
    list_qsort(
        (*binding).keys,
        Some(
            key_qsort_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    if !translate_binding(binding) {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Unable to translate bindsym into bindcode: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
            471 as ::core::ffi::c_int,
            *argv.offset(0isize),
        );
    }
    let mut mode_bindings: *mut list_t = ::core::ptr::null_mut::<list_t>();
    if (*binding).r#type.0 == binding_input_type::BINDING_KEYCODE.0 {
        mode_bindings = (*(*config).current_mode).keycode_bindings;
    } else if (*binding).r#type.0 == binding_input_type::BINDING_KEYSYM.0 {
        mode_bindings = (*(*config).current_mode).keysym_bindings;
    } else {
        mode_bindings = (*(*config).current_mode).mouse_bindings;
    }
    if unbind {
        return binding_remove(binding, mode_bindings, bindtype, *argv.offset(0isize));
    }
    (*binding).command = join_args(
        argv.offset(1 as ::core::ffi::c_int as isize),
        argc - 1 as ::core::ffi::c_int,
    );
    let c2rust_fresh0 = binding_order;
    binding_order += 1;
    (*binding).order = c2rust_fresh0;
    return binding_add(binding, mode_bindings, bindtype, *argv.offset(0isize), warn);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_bind_or_unbind_switch(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut unbind: bool,
) -> *mut cmd_results {
    let mut minargs: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    let mut bindtype: *mut ::core::ffi::c_char = b"bindswitch\0".as_ptr()
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    if unbind {
        minargs -= 1;
        bindtype = b"unbindswitch\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    }
    let mut error: *mut cmd_results = ::core::ptr::null_mut::<cmd_results>();
    error = checkarg(argc, bindtype, expected_args::EXPECTED_AT_LEAST, minargs);
    if !error.is_null() {
        return error;
    }
    let mut binding: *mut sway_switch_binding = calloc(
        1 as size_t,
        ::core::mem::size_of::<sway_switch_binding>(),
    ) as *mut sway_switch_binding;
    if binding.is_null() {
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"Unable to allocate binding\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut warn: bool = r#true != 0;
    while argc > 0 as ::core::ffi::c_int {
        if strcmp(
            b"--locked\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(0isize),
        ) == 0 as ::core::ffi::c_int
        {
            (*binding).flags
                |= binding_flags::BINDING_LOCKED.0 as ::core::ffi::c_int as uint32_t;
        } else if strcmp(
            b"--no-warn\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(0isize),
        ) == 0 as ::core::ffi::c_int
        {
            warn = r#false != 0;
        } else {
            if strcmp(
                b"--reload\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0isize),
            ) != 0 as ::core::ffi::c_int
            {
                break;
            }
            (*binding).flags
                |= binding_flags::BINDING_RELOAD.0 as ::core::ffi::c_int as uint32_t;
        }
        argv = argv.offset(1);
        argc -= 1;
    }
    if argc < minargs {
        free(binding as *mut ::core::ffi::c_void);
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"Invalid %s command (expected at least %d non-option arguments, got %d)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            bindtype,
            minargs,
            argc,
        );
    }
    let mut split: *mut list_t = split_string(
        *argv.offset(0isize),
        b":\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if (*split).length != 2 as ::core::ffi::c_int {
        free_switch_binding(binding);
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"Invalid %s command (expected binding with the form <switch>:<state>)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            bindtype,
        );
    }
    if strcmp(
        *(*split).items.offset(0isize) as *const ::core::ffi::c_char,
        b"tablet\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*binding).r#type = wlr_switch_type::WLR_SWITCH_TYPE_TABLET_MODE;
    } else if strcmp(
        *(*split).items.offset(0isize) as *const ::core::ffi::c_char,
        b"lid\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*binding).r#type = wlr_switch_type::WLR_SWITCH_TYPE_LID;
    } else if strcmp(
        *(*split).items.offset(0isize) as *const ::core::ffi::c_char,
        b"keypad_slide\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*binding).r#type = wlr_switch_type::WLR_SWITCH_TYPE_KEYPAD_SLIDE;
    } else {
        free_switch_binding(binding);
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"Invalid %s command (expected switch binding: unknown switch %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            bindtype,
            *(*split).items.offset(0isize) as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        *(*split).items.offset(1isize) as *const ::core::ffi::c_char,
        b"on\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*binding).trigger = sway_switch_trigger::SWAY_SWITCH_TRIGGER_ON;
    } else if strcmp(
        *(*split).items.offset(1isize) as *const ::core::ffi::c_char,
        b"off\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*binding).trigger = sway_switch_trigger::SWAY_SWITCH_TRIGGER_OFF;
    } else if strcmp(
        *(*split).items.offset(1isize) as *const ::core::ffi::c_char,
        b"toggle\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*binding).trigger = sway_switch_trigger::SWAY_SWITCH_TRIGGER_TOGGLE;
    } else {
        free_switch_binding(binding);
        return cmd_results_new(
            cmd_status::CMD_FAILURE,
            b"Invalid %s command (expected switch state: unknown state %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            bindtype,
            *(*split).items.offset(1isize) as *const ::core::ffi::c_char,
        );
    }
    list_free_items_and_destroy(split);
    if unbind {
        return switch_binding_remove(binding, bindtype, *argv.offset(0isize));
    }
    (*binding).command = join_args(
        argv.offset(1 as ::core::ffi::c_int as isize),
        argc - 1 as ::core::ffi::c_int,
    );
    return switch_binding_add(binding, bindtype, *argv.offset(0isize), warn);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_bindsym(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    return cmd_bindsym_or_bindcode(argc, argv, r#false != 0, r#false != 0);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_bindcode(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    return cmd_bindsym_or_bindcode(argc, argv, r#true != 0, r#false != 0);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_unbindsym(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    return cmd_bindsym_or_bindcode(argc, argv, r#false != 0, r#true != 0);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_unbindcode(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    return cmd_bindsym_or_bindcode(argc, argv, r#true != 0, r#true != 0);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_bindswitch(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    return cmd_bind_or_unbind_switch(argc, argv, r#false != 0);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_unbindswitch(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    return cmd_bind_or_unbind_switch(argc, argv, r#true != 0);
}
#[no_mangle]
pub unsafe extern "C" fn seat_execute_command(
    mut seat: *mut sway_seat,
    mut binding: *mut sway_binding,
) {
    if !(*config).active {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] deferring command for binding: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
            608 as ::core::ffi::c_int,
            (*binding).command,
        );
        let mut deferred: *mut sway_binding = calloc(
            1 as size_t,
            ::core::mem::size_of::<sway_binding>(),
        ) as *mut sway_binding;
        if deferred.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Failed to allocate deferred binding\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
                612 as ::core::ffi::c_int,
            );
            return;
        }
        memcpy(
            deferred as *mut ::core::ffi::c_void,
            binding as *const ::core::ffi::c_void,
            ::core::mem::size_of::<sway_binding>(),
        );
        (*deferred).command = if !(*binding).command.is_null() {
            strdup((*binding).command)
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        list_add((*seat).deferred_bindings, deferred as *mut ::core::ffi::c_void);
        return;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] running command for binding: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
        621 as ::core::ffi::c_int,
        (*binding).command,
    );
    let mut con: *mut sway_container = ::core::ptr::null_mut::<sway_container>();
    if (*binding).r#type.0 == binding_input_type::BINDING_MOUSESYM.0
        || (*binding).r#type.0 == binding_input_type::BINDING_MOUSECODE.0
    {
        let mut surface: *mut wlr_surface = ::core::ptr::null_mut::<wlr_surface>();
        let mut sx: ::core::ffi::c_double = 0.;
        let mut sy: ::core::ffi::c_double = 0.;
        let mut node: *mut sway_node = node_at_coords(
            seat,
            (*(*(*seat).cursor).cursor).x,
            (*(*(*seat).cursor).cursor).y,
            &raw mut surface,
            &raw mut sx,
            &raw mut sy,
        );
        if !node.is_null() && (*node).r#type.0 == sway_node_type::N_CONTAINER.0 {
            con = (*node).c2rust_unnamed.sway_container;
        }
    }
    let mut res_list: *mut list_t = execute_command((*binding).command, seat, con);
    let mut success: bool = r#true != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*res_list).length {
        let mut results: *mut cmd_results = *(*res_list).items.offset(i as isize)
            as *mut cmd_results;
        if (*results).status.0 != cmd_status::CMD_SUCCESS.0 {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] could not run command for binding: %s (%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
                640 as ::core::ffi::c_int,
                (*binding).command,
                (*results).error,
            );
            success = r#false != 0;
        }
        free_cmd_results(results);
        i += 1;
    }
    list_free(res_list);
    if success {
        ipc_event_binding(binding);
    }
    transaction_commit_dirty();
}
unsafe extern "C" fn find_keycode(
    mut keymap: *mut xkb_keymap,
    mut keycode: xkb_keycode_t,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut keysym: xkb_keysym_t = xkb_state_key_get_one_sym(
        (*config).keysym_translation_state,
        keycode,
    );
    if keysym == XKB_KEY_NoSymbol as xkb_keysym_t {
        return;
    }
    let mut matches: *mut keycode_matches = data as *mut keycode_matches;
    if (*matches).keysym == keysym {
        (*matches).keycode = keycode;
        (*matches).count += 1;
    }
}
unsafe extern "C" fn get_keycode_for_keysym(
    mut keysym: xkb_keysym_t,
) -> keycode_matches {
    let mut matches: keycode_matches = keycode_matches {
        keysym: keysym,
        keycode: XKB_KEYCODE_INVALID as xkb_keycode_t,
        count: 0 as ::core::ffi::c_int,
    };
    xkb_keymap_key_for_each(
        xkb_state_get_keymap((*config).keysym_translation_state),
        Some(
            find_keycode
                as unsafe extern "C" fn(
                    *mut xkb_keymap,
                    xkb_keycode_t,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        &raw mut matches as *mut ::core::ffi::c_void,
    );
    return matches;
}
#[no_mangle]
pub unsafe extern "C" fn translate_binding(mut binding: *mut sway_binding) -> bool {
    if (*binding).flags & binding_flags::BINDING_CODE.0 as ::core::ffi::c_int as uint32_t
        == 0 as uint32_t
    {
        return r#true != 0;
    }
    match (*binding).r#type {
        binding_input_type::BINDING_KEYSYM => {
            (*binding).syms = (*binding).keys;
            (*binding).keys = create_list();
        }
        binding_input_type::BINDING_KEYCODE => {
            list_free_items_and_destroy((*binding).keys);
            (*binding).keys = create_list();
        }
        _ => return r#true != 0,
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    '_error: {
        while i < (*(*binding).syms).length {
            let mut keysym: *mut xkb_keysym_t = *(*(*binding).syms)
                .items
                .offset(i as isize) as *mut xkb_keysym_t;
            let mut matches: keycode_matches = get_keycode_for_keysym(*keysym);
            if matches.count != 1 as ::core::ffi::c_int {
                _sway_log(
                    sway_log_importance_t::SWAY_INFO,
                    b"[%s:%d] Unable to convert keysym %u into a single keycode (found %d matches)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
                    725 as ::core::ffi::c_int,
                    *keysym,
                    matches.count,
                );
                break '_error;
            } else {
                let mut keycode: *mut xkb_keycode_t = malloc(
                    ::core::mem::size_of::<xkb_keycode_t>(),
                ) as *mut xkb_keycode_t;
                if keycode.is_null() {
                    _sway_log(
                        sway_log_importance_t::SWAY_ERROR,
                        b"[%s:%d] Unable to allocate memory for a keycode\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
                        733 as ::core::ffi::c_int,
                    );
                    break '_error;
                } else {
                    *keycode = matches.keycode;
                    list_add((*binding).keys, keycode as *mut ::core::ffi::c_void);
                    i += 1;
                }
            }
        }
        list_qsort(
            (*binding).keys,
            Some(
                key_qsort_cmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        (*binding).r#type = binding_input_type::BINDING_KEYCODE;
        return r#true != 0;
    }
    list_free_items_and_destroy((*binding).keys);
    (*binding).r#type = binding_input_type::BINDING_KEYSYM;
    (*binding).keys = (*binding).syms;
    (*binding).syms = ::core::ptr::null_mut::<list_t>();
    return r#false != 0;
}
#[no_mangle]
pub unsafe extern "C" fn binding_add_translated(
    mut binding: *mut sway_binding,
    mut mode_bindings: *mut list_t,
) {
    let mut config_binding: *mut sway_binding = binding_upsert(binding, mode_bindings);
    if !config_binding.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Overwriting binding for device '%s' to `%s` from `%s`\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/commands/bind.c\0".as_ptr() as *const ::core::ffi::c_char,
            759 as ::core::ffi::c_int,
            (*binding).input,
            (*binding).command,
            (*config_binding).command,
        );
        free_sway_binding(config_binding);
    }
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
