extern "C" {
    pub type wl_event_loop;
    pub type wl_event_source;
    pub type wl_display;
    pub type wl_client;
    pub type wl_global;
    pub type wlr_renderer_impl;
    pub type wlr_texture_impl;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _cairo_font_options;
    pub type _PangoContext;
    pub type _PangoFontDescription;
    pub type xkb_state;
    pub type wlr_swapchain;
    pub type wlr_allocator;
    pub type wlr_drm_syncobj_timeline;
    pub type wlr_backend;
    pub type wlr_output_impl;
    pub type wlr_linux_dmabuf_feedback_v1_compiled;
    pub type wlr_output_layer_feedback_event;
    pub type wlr_gamma_control_v1;
    pub type wlr_color_manager_v1;
    pub type wlr_gamma_control_manager_v1;
    pub type sway_workspace;
    pub type sway_view;
    pub type sway_transaction_instruction;
    pub type sway_output;
    pub type sway_seat;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    fn pixman_region32_init(region: *mut pixman_region32_t);
    fn pixman_region32_fini(region: *mut pixman_region32_t);
    fn pixman_region32_union_rect(
        dest: *mut pixman_region32_t,
        source: *const pixman_region32_t,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_uint,
        height: ::core::ffi::c_uint,
    ) -> pixman_bool_t;
    fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn wlr_buffer_drop(buffer: *mut wlr_buffer);
    fn wlr_buffer_init(
        buffer: *mut wlr_buffer,
        r#impl: *const wlr_buffer_impl,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    fn cairo_destroy(cr: *mut cairo_t);
    fn cairo_set_source_rgba(
        cr: *mut cairo_t,
        red: ::core::ffi::c_double,
        green: ::core::ffi::c_double,
        blue: ::core::ffi::c_double,
        alpha: ::core::ffi::c_double,
    );
    fn cairo_set_antialias(cr: *mut cairo_t, antialias: cairo_antialias_t);
    fn cairo_move_to(
        cr: *mut cairo_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    );
    fn cairo_rectangle(
        cr: *mut cairo_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        width: ::core::ffi::c_double,
        height: ::core::ffi::c_double,
    );
    fn cairo_fill(cr: *mut cairo_t);
    fn cairo_font_options_create() -> *mut cairo_font_options_t;
    fn cairo_font_options_destroy(options: *mut cairo_font_options_t);
    fn cairo_font_options_set_antialias(
        options: *mut cairo_font_options_t,
        antialias: cairo_antialias_t,
    );
    fn cairo_font_options_set_subpixel_order(
        options: *mut cairo_font_options_t,
        subpixel_order: cairo_subpixel_order_t,
    );
    fn cairo_font_options_set_hint_style(
        options: *mut cairo_font_options_t,
        hint_style: cairo_hint_style_t,
    );
    fn cairo_set_font_options(cr: *mut cairo_t, options: *const cairo_font_options_t);
    fn cairo_status(cr: *mut cairo_t) -> cairo_status_t;
    fn cairo_status_to_string(status: cairo_status_t) -> *const ::core::ffi::c_char;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_status(surface: *mut cairo_surface_t) -> cairo_status_t;
    fn cairo_surface_flush(surface: *mut cairo_surface_t);
    fn cairo_image_surface_create(
        format: cairo_format_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_image_surface_get_data(
        surface: *mut cairo_surface_t,
    ) -> *mut ::core::ffi::c_uchar;
    fn cairo_image_surface_get_stride(
        surface: *mut cairo_surface_t,
    ) -> ::core::ffi::c_int;
    fn cairo_recording_surface_create(
        content: cairo_content_t,
        extents: *const cairo_rectangle_t,
    ) -> *mut cairo_surface_t;
    fn to_cairo_subpixel_order(subpixel: wl_output_subpixel) -> cairo_subpixel_order_t;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn g_object_unref(object: gpointer);
    fn pango_cairo_create_context(cr: *mut cairo_t) -> *mut PangoContext;
    fn get_text_size(
        cairo: *mut cairo_t,
        desc: *const PangoFontDescription,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
        baseline: *mut ::core::ffi::c_int,
        scale: ::core::ffi::c_double,
        markup: bool,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn render_text(
        cairo: *mut cairo_t,
        desc: *mut PangoFontDescription,
        scale: ::core::ffi::c_double,
        markup: bool,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn wlr_scene_node_destroy(node: *mut wlr_scene_node);
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
    fn wlr_scene_buffer_set_dest_size(
        scene_buffer: *mut wlr_scene_buffer,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    static mut config: *mut sway_config;
}
pub type __u32 = ::core::ffi::c_uint;
pub type size_t = usize;
pub type __off64_t = ::core::ffi::c_long;
pub type off_t = __off64_t;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub struct wl_signal {
    pub listener_list: wl_list,
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
pub struct C2Rust_Unnamed_1 {
    pub destroy: wl_signal,
    pub release: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_buffer_impl {
    pub destroy: Option<unsafe extern "C" fn(*mut wlr_buffer) -> ()>,
    pub get_dmabuf: Option<
        unsafe extern "C" fn(*mut wlr_buffer, *mut wlr_dmabuf_attributes) -> bool,
    >,
    pub get_shm: Option<
        unsafe extern "C" fn(*mut wlr_buffer, *mut wlr_shm_attributes) -> bool,
    >,
    pub begin_data_ptr_access: Option<
        unsafe extern "C" fn(
            *mut wlr_buffer,
            uint32_t,
            *mut *mut ::core::ffi::c_void,
            *mut uint32_t,
            *mut size_t,
        ) -> bool,
    >,
    pub end_data_ptr_access: Option<unsafe extern "C" fn(*mut wlr_buffer) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_shm_attributes {
    pub fd: ::core::ffi::c_int,
    pub format: uint32_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub stride: ::core::ffi::c_int,
    pub offset: off_t,
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
pub struct wlr_texture {
    pub r#impl: *const wlr_texture_impl,
    pub width: uint32_t,
    pub height: uint32_t,
    pub renderer: *mut wlr_renderer,
}
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct _cairo_status(pub ::core::ffi::c_uint);
impl _cairo_status {
    pub const CAIRO_STATUS_SUCCESS: Self = Self(0);
    pub const CAIRO_STATUS_NO_MEMORY: Self = Self(1);
    pub const CAIRO_STATUS_INVALID_RESTORE: Self = Self(2);
    pub const CAIRO_STATUS_INVALID_POP_GROUP: Self = Self(3);
    pub const CAIRO_STATUS_NO_CURRENT_POINT: Self = Self(4);
    pub const CAIRO_STATUS_INVALID_MATRIX: Self = Self(5);
    pub const CAIRO_STATUS_INVALID_STATUS: Self = Self(6);
    pub const CAIRO_STATUS_NULL_POINTER: Self = Self(7);
    pub const CAIRO_STATUS_INVALID_STRING: Self = Self(8);
    pub const CAIRO_STATUS_INVALID_PATH_DATA: Self = Self(9);
    pub const CAIRO_STATUS_READ_ERROR: Self = Self(10);
    pub const CAIRO_STATUS_WRITE_ERROR: Self = Self(11);
    pub const CAIRO_STATUS_SURFACE_FINISHED: Self = Self(12);
    pub const CAIRO_STATUS_SURFACE_TYPE_MISMATCH: Self = Self(13);
    pub const CAIRO_STATUS_PATTERN_TYPE_MISMATCH: Self = Self(14);
    pub const CAIRO_STATUS_INVALID_CONTENT: Self = Self(15);
    pub const CAIRO_STATUS_INVALID_FORMAT: Self = Self(16);
    pub const CAIRO_STATUS_INVALID_VISUAL: Self = Self(17);
    pub const CAIRO_STATUS_FILE_NOT_FOUND: Self = Self(18);
    pub const CAIRO_STATUS_INVALID_DASH: Self = Self(19);
    pub const CAIRO_STATUS_INVALID_DSC_COMMENT: Self = Self(20);
    pub const CAIRO_STATUS_INVALID_INDEX: Self = Self(21);
    pub const CAIRO_STATUS_CLIP_NOT_REPRESENTABLE: Self = Self(22);
    pub const CAIRO_STATUS_TEMP_FILE_ERROR: Self = Self(23);
    pub const CAIRO_STATUS_INVALID_STRIDE: Self = Self(24);
    pub const CAIRO_STATUS_FONT_TYPE_MISMATCH: Self = Self(25);
    pub const CAIRO_STATUS_USER_FONT_IMMUTABLE: Self = Self(26);
    pub const CAIRO_STATUS_USER_FONT_ERROR: Self = Self(27);
    pub const CAIRO_STATUS_NEGATIVE_COUNT: Self = Self(28);
    pub const CAIRO_STATUS_INVALID_CLUSTERS: Self = Self(29);
    pub const CAIRO_STATUS_INVALID_SLANT: Self = Self(30);
    pub const CAIRO_STATUS_INVALID_WEIGHT: Self = Self(31);
    pub const CAIRO_STATUS_INVALID_SIZE: Self = Self(32);
    pub const CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED: Self = Self(33);
    pub const CAIRO_STATUS_DEVICE_TYPE_MISMATCH: Self = Self(34);
    pub const CAIRO_STATUS_DEVICE_ERROR: Self = Self(35);
    pub const CAIRO_STATUS_INVALID_MESH_CONSTRUCTION: Self = Self(36);
    pub const CAIRO_STATUS_DEVICE_FINISHED: Self = Self(37);
    pub const CAIRO_STATUS_JBIG2_GLOBAL_MISSING: Self = Self(38);
    pub const CAIRO_STATUS_PNG_ERROR: Self = Self(39);
    pub const CAIRO_STATUS_FREETYPE_ERROR: Self = Self(40);
    pub const CAIRO_STATUS_WIN32_GDI_ERROR: Self = Self(41);
    pub const CAIRO_STATUS_TAG_ERROR: Self = Self(42);
    pub const CAIRO_STATUS_DWRITE_ERROR: Self = Self(43);
    pub const CAIRO_STATUS_SVG_FONT_ERROR: Self = Self(44);
    pub const CAIRO_STATUS_LAST_STATUS: Self = Self(45);
}
pub type cairo_status_t = _cairo_status;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct _cairo_content(pub ::core::ffi::c_uint);
impl _cairo_content {
    pub const CAIRO_CONTENT_COLOR: Self = Self(4096);
    pub const CAIRO_CONTENT_ALPHA: Self = Self(8192);
    pub const CAIRO_CONTENT_COLOR_ALPHA: Self = Self(12288);
}
pub type cairo_content_t = _cairo_content;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct _cairo_format(pub ::core::ffi::c_int);
impl _cairo_format {
    pub const CAIRO_FORMAT_INVALID: Self = Self(-1);
    pub const CAIRO_FORMAT_ARGB32: Self = Self(0);
    pub const CAIRO_FORMAT_RGB24: Self = Self(1);
    pub const CAIRO_FORMAT_A8: Self = Self(2);
    pub const CAIRO_FORMAT_A1: Self = Self(3);
    pub const CAIRO_FORMAT_RGB16_565: Self = Self(4);
    pub const CAIRO_FORMAT_RGB30: Self = Self(5);
    pub const CAIRO_FORMAT_RGB96F: Self = Self(6);
    pub const CAIRO_FORMAT_RGBA128F: Self = Self(7);
}
pub type cairo_format_t = _cairo_format;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct _cairo_antialias(pub ::core::ffi::c_uint);
impl _cairo_antialias {
    pub const CAIRO_ANTIALIAS_DEFAULT: Self = Self(0);
    pub const CAIRO_ANTIALIAS_NONE: Self = Self(1);
    pub const CAIRO_ANTIALIAS_GRAY: Self = Self(2);
    pub const CAIRO_ANTIALIAS_SUBPIXEL: Self = Self(3);
    pub const CAIRO_ANTIALIAS_FAST: Self = Self(4);
    pub const CAIRO_ANTIALIAS_GOOD: Self = Self(5);
    pub const CAIRO_ANTIALIAS_BEST: Self = Self(6);
}
pub type cairo_antialias_t = _cairo_antialias;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
}
pub type cairo_rectangle_t = _cairo_rectangle;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct _cairo_subpixel_order(pub ::core::ffi::c_uint);
impl _cairo_subpixel_order {
    pub const CAIRO_SUBPIXEL_ORDER_DEFAULT: Self = Self(0);
    pub const CAIRO_SUBPIXEL_ORDER_RGB: Self = Self(1);
    pub const CAIRO_SUBPIXEL_ORDER_BGR: Self = Self(2);
    pub const CAIRO_SUBPIXEL_ORDER_VRGB: Self = Self(3);
    pub const CAIRO_SUBPIXEL_ORDER_VBGR: Self = Self(4);
}
pub type cairo_subpixel_order_t = _cairo_subpixel_order;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct _cairo_hint_style(pub ::core::ffi::c_uint);
impl _cairo_hint_style {
    pub const CAIRO_HINT_STYLE_DEFAULT: Self = Self(0);
    pub const CAIRO_HINT_STYLE_NONE: Self = Self(1);
    pub const CAIRO_HINT_STYLE_SLIGHT: Self = Self(2);
    pub const CAIRO_HINT_STYLE_MEDIUM: Self = Self(3);
    pub const CAIRO_HINT_STYLE_FULL: Self = Self(4);
}
pub type cairo_hint_style_t = _cairo_hint_style;
pub type cairo_font_options_t = _cairo_font_options;
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
pub type gpointer = *mut ::core::ffi::c_void;
pub type PangoContext = _PangoContext;
pub type PangoFontDescription = _PangoFontDescription;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub ref_count: ::core::ffi::c_int,
    pub addons: wlr_addon_set,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
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
    pub mastering_luminance: C2Rust_Unnamed_7,
    pub max_cll: ::core::ffi::c_double,
    pub max_fall: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_scale_filter_mode(pub ::core::ffi::c_uint);
impl wlr_scale_filter_mode {
    pub const WLR_SCALE_FILTER_BILINEAR: Self = Self(0);
    pub const WLR_SCALE_FILTER_NEAREST: Self = Self(1);
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
pub struct wlr_damage_ring {
    pub current: pixman_region32_t,
    pub WLR_PRIVATE: C2Rust_Unnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub buffers: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_linux_dmabuf_v1 {
    pub global: *mut wl_global,
    pub events: C2Rust_Unnamed_12,
    pub WLR_PRIVATE: C2Rust_Unnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
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
pub struct C2Rust_Unnamed_12 {
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
pub struct wlr_scene_node {
    pub r#type: wlr_scene_node_type,
    pub parent: *mut wlr_scene_tree,
    pub link: wl_list,
    pub enabled: bool,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub events: C2Rust_Unnamed_16,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
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
    pub events: C2Rust_Unnamed_22,
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
    pub WLR_PRIVATE: C2Rust_Unnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
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
    pub events: C2Rust_Unnamed_20,
    pub WLR_PRIVATE: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
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
pub struct C2Rust_Unnamed_20 {
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
    pub WLR_PRIVATE: C2Rust_Unnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
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
pub struct C2Rust_Unnamed_22 {
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
    pub layers: C2Rust_Unnamed_29,
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
    pub events: C2Rust_Unnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub new_node: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sway_container {
    pub node: sway_node,
    pub view: *mut sway_view,
    pub scene_tree: *mut wlr_scene_tree,
    pub title_bar: C2Rust_Unnamed_26,
    pub border: C2Rust_Unnamed_25,
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
    pub events: C2Rust_Unnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
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
pub struct C2Rust_Unnamed_25 {
    pub tree: *mut wlr_scene_tree,
    pub top: *mut wlr_scene_rect,
    pub bottom: *mut wlr_scene_rect,
    pub left: *mut wlr_scene_rect,
    pub right: *mut wlr_scene_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
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
pub struct sway_node {
    pub r#type: sway_node_type,
    pub c2rust_unnamed: C2Rust_Unnamed_28,
    pub id: size_t,
    pub instruction: *mut sway_transaction_instruction,
    pub ntxnrefs: size_t,
    pub destroying: bool,
    pub dirty: bool,
    pub events: C2Rust_Unnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_28 {
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
pub struct C2Rust_Unnamed_29 {
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
    pub xcursor_theme: C2Rust_Unnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
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
    pub colors: C2Rust_Unnamed_31,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
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
    pub border_colors: C2Rust_Unnamed_34,
    pub has_focused_tab_title: bool,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub keysym_translation_state: *mut xkb_state,
    pub handler_context: C2Rust_Unnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
    pub input_config: *mut input_config,
    pub output_config: *mut output_config,
    pub seat_config: *mut seat_config,
    pub seat: *mut sway_seat,
    pub node: *mut sway_node,
    pub container: *mut sway_container,
    pub workspace: *mut sway_workspace,
    pub node_overridden: bool,
    pub leftovers: C2Rust_Unnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {
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
pub struct text_buffer {
    pub buffer_node: *mut wlr_scene_buffer,
    pub text: *mut ::core::ffi::c_char,
    pub props: sway_text_node,
    pub visible: bool,
    pub scale: ::core::ffi::c_float,
    pub subpixel: wl_output_subpixel,
    pub outputs_update: wl_listener,
    pub destroy: wl_listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_buffer {
    pub base: wlr_buffer,
    pub surface: *mut cairo_surface_t,
    pub cairo: *mut cairo_t,
}
pub const DRM_FORMAT_ARGB8888: __u32 = 'A' as ::core::ffi::c_int as __u32
    | ('R' as ::core::ffi::c_int as __u32) << 8 as ::core::ffi::c_int
    | ('2' as ::core::ffi::c_int as __u32) << 16 as ::core::ffi::c_int
    | ('4' as ::core::ffi::c_int as __u32) << 24 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_signal_add(
    mut signal: *mut wl_signal,
    mut listener: *mut wl_listener,
) {
    wl_list_insert((*signal).listener_list.prev, &raw mut (*listener).link);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
unsafe extern "C" fn cairo_buffer_handle_destroy(mut wlr_buffer: *mut wlr_buffer) {
    let mut buffer: *mut cairo_buffer = (wlr_buffer as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut cairo_buffer;
    cairo_surface_destroy((*buffer).surface);
    cairo_destroy((*buffer).cairo);
    free(buffer as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn cairo_buffer_handle_begin_data_ptr_access(
    mut wlr_buffer: *mut wlr_buffer,
    mut flags: uint32_t,
    mut data: *mut *mut ::core::ffi::c_void,
    mut format: *mut uint32_t,
    mut stride: *mut size_t,
) -> bool {
    let mut buffer: *mut cairo_buffer = (wlr_buffer as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut cairo_buffer;
    *data = cairo_image_surface_get_data((*buffer).surface) as *mut ::core::ffi::c_void;
    *stride = cairo_image_surface_get_stride((*buffer).surface) as size_t;
    *format = DRM_FORMAT_ARGB8888 as uint32_t;
    return r#true != 0;
}
unsafe extern "C" fn cairo_buffer_handle_end_data_ptr_access(
    mut wlr_buffer: *mut wlr_buffer,
) {}
static mut cairo_buffer_impl: wlr_buffer_impl = wlr_buffer_impl {
    destroy: Some(
        cairo_buffer_handle_destroy as unsafe extern "C" fn(*mut wlr_buffer) -> (),
    ),
    get_dmabuf: None,
    get_shm: None,
    begin_data_ptr_access: Some(
        cairo_buffer_handle_begin_data_ptr_access
            as unsafe extern "C" fn(
                *mut wlr_buffer,
                uint32_t,
                *mut *mut ::core::ffi::c_void,
                *mut uint32_t,
                *mut size_t,
            ) -> bool,
    ),
    end_data_ptr_access: Some(
        cairo_buffer_handle_end_data_ptr_access
            as unsafe extern "C" fn(*mut wlr_buffer) -> (),
    ),
};
unsafe extern "C" fn get_text_width(
    mut props: *mut sway_text_node,
) -> ::core::ffi::c_int {
    let mut width: ::core::ffi::c_int = (*props).width;
    if (*props).max_width >= 0 as ::core::ffi::c_int {
        width = if width < (*props).max_width { width } else { (*props).max_width };
    }
    return if width > 0 as ::core::ffi::c_int { width } else { 0 as ::core::ffi::c_int };
}
unsafe extern "C" fn render_backing_buffer(mut buffer: *mut text_buffer) {
    let mut cairo_buffer: *mut cairo_buffer = ::core::ptr::null_mut::<cairo_buffer>();
    let mut cairo: *mut cairo_t = ::core::ptr::null_mut::<cairo_t>();
    let mut opaque: pixman_region32_t = pixman_region32_t {
        extents: pixman_box32_t {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        },
        data: ::core::ptr::null_mut::<pixman_region32_data_t>(),
    };
    if !(*buffer).visible {
        return;
    }
    if (*buffer).props.max_width == 0 as ::core::ffi::c_int {
        wlr_scene_buffer_set_buffer(
            (*buffer).buffer_node,
            ::core::ptr::null_mut::<wlr_buffer>(),
        );
        return;
    }
    let mut scale: ::core::ffi::c_float = (*buffer).scale;
    let mut width: ::core::ffi::c_int = ceil(
        (get_text_width(&raw mut (*buffer).props) as ::core::ffi::c_float * scale)
            as ::core::ffi::c_double,
    ) as ::core::ffi::c_int;
    let mut height: ::core::ffi::c_int = ceil(
        ((*buffer).props.height as ::core::ffi::c_float * scale) as ::core::ffi::c_double,
    ) as ::core::ffi::c_int;
    let mut color: *mut ::core::ffi::c_float = &raw mut (*buffer).props.color
        as *mut ::core::ffi::c_float;
    let mut background: *mut ::core::ffi::c_float = &raw mut (*buffer).props.background
        as *mut ::core::ffi::c_float;
    let mut pango: *mut PangoContext = ::core::ptr::null_mut::<PangoContext>();
    let mut fo: *mut cairo_font_options_t = cairo_font_options_create();
    cairo_font_options_set_hint_style(fo, _cairo_hint_style::CAIRO_HINT_STYLE_FULL);
    let mut subpixel: wl_output_subpixel = (*buffer).subpixel;
    if subpixel.0 == wl_output_subpixel::WL_OUTPUT_SUBPIXEL_NONE.0
        || subpixel.0 == wl_output_subpixel::WL_OUTPUT_SUBPIXEL_UNKNOWN.0
    {
        cairo_font_options_set_antialias(fo, _cairo_antialias::CAIRO_ANTIALIAS_GRAY);
    } else {
        cairo_font_options_set_antialias(fo, _cairo_antialias::CAIRO_ANTIALIAS_SUBPIXEL);
        cairo_font_options_set_subpixel_order(fo, to_cairo_subpixel_order(subpixel));
    }
    let mut surface: *mut cairo_surface_t = cairo_image_surface_create(
        _cairo_format::CAIRO_FORMAT_ARGB32,
        width,
        height,
    );
    let mut status: cairo_status_t = cairo_surface_status(surface);
    if status.0 != _cairo_status::CAIRO_STATUS_SUCCESS.0 {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] cairo_image_surface_create failed: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/sway_text_node.c\0".as_ptr() as *const ::core::ffi::c_char,
            98 as ::core::ffi::c_int,
            cairo_status_to_string(status),
        );
    } else {
        cairo_buffer = calloc(1 as size_t, ::core::mem::size_of::<cairo_buffer>())
            as *mut cairo_buffer;
        if cairo_buffer.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] cairo_buffer allocation failed\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"sway/sway_text_node.c\0".as_ptr() as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_int,
            );
        } else {
            cairo = cairo_create(surface);
            if cairo.is_null() {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] cairo_create failed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"sway/sway_text_node.c\0".as_ptr() as *const ::core::ffi::c_char,
                    111 as ::core::ffi::c_int,
                );
                free(cairo_buffer as *mut ::core::ffi::c_void);
            } else {
                cairo_set_antialias(cairo, _cairo_antialias::CAIRO_ANTIALIAS_BEST);
                cairo_set_font_options(cairo, fo);
                pango = pango_cairo_create_context(cairo);
                cairo_set_source_rgba(
                    cairo,
                    *background.offset(0isize) as ::core::ffi::c_double,
                    *background.offset(1isize) as ::core::ffi::c_double,
                    *background.offset(2isize) as ::core::ffi::c_double,
                    *background.offset(3isize) as ::core::ffi::c_double,
                );
                cairo_rectangle(
                    cairo,
                    0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    width as ::core::ffi::c_double,
                    height as ::core::ffi::c_double,
                );
                cairo_fill(cairo);
                cairo_set_source_rgba(
                    cairo,
                    *color.offset(0isize) as ::core::ffi::c_double,
                    *color.offset(1isize) as ::core::ffi::c_double,
                    *color.offset(2isize) as ::core::ffi::c_double,
                    *color.offset(3isize) as ::core::ffi::c_double,
                );
                cairo_move_to(
                    cairo,
                    0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    (((*config).font_baseline - (*buffer).props.baseline)
                        as ::core::ffi::c_float * scale) as ::core::ffi::c_double,
                );
                render_text(
                    cairo,
                    (*config).font_description,
                    scale as ::core::ffi::c_double,
                    (*buffer).props.pango_markup,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*buffer).text,
                );
                cairo_surface_flush(surface);
                wlr_buffer_init(
                    &raw mut (*cairo_buffer).base,
                    &raw const cairo_buffer_impl,
                    width,
                    height,
                );
                (*cairo_buffer).surface = surface;
                (*cairo_buffer).cairo = cairo;
                wlr_scene_buffer_set_buffer(
                    (*buffer).buffer_node,
                    &raw mut (*cairo_buffer).base,
                );
                wlr_buffer_drop(&raw mut (*cairo_buffer).base);
                opaque = pixman_region32_t {
                    extents: pixman_box32_t {
                        x1: 0,
                        y1: 0,
                        x2: 0,
                        y2: 0,
                    },
                    data: ::core::ptr::null_mut::<pixman_region32_data_t>(),
                };
                pixman_region32_init(&raw mut opaque);
                if *background.offset(3isize)
                    == 1 as ::core::ffi::c_int as ::core::ffi::c_float
                {
                    pixman_region32_union_rect(
                        &raw mut opaque,
                        &raw mut opaque,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        get_text_width(&raw mut (*buffer).props) as ::core::ffi::c_uint,
                        (*buffer).props.height as ::core::ffi::c_uint,
                    );
                }
                wlr_scene_buffer_set_opaque_region(
                    (*buffer).buffer_node,
                    &raw mut opaque,
                );
                pixman_region32_fini(&raw mut opaque);
            }
        }
    }
    if !pango.is_null() {
        g_object_unref(pango as gpointer);
    }
    cairo_font_options_destroy(fo);
}
unsafe extern "C" fn handle_outputs_update(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut buffer: *mut text_buffer = (listener as *mut ::core::ffi::c_char)
        .offset(-(96 as ::core::ffi::c_ulong as isize)) as *mut text_buffer;
    let mut event: *mut wlr_scene_outputs_update_event = data
        as *mut wlr_scene_outputs_update_event;
    let mut scale: ::core::ffi::c_float = 0 as ::core::ffi::c_int
        as ::core::ffi::c_float;
    let mut subpixel: wl_output_subpixel = wl_output_subpixel::WL_OUTPUT_SUBPIXEL_UNKNOWN;
    let mut i: size_t = 0 as size_t;
    while i < (*event).size {
        let mut output: *mut wlr_scene_output = *(*event).active.offset(i as isize);
        if subpixel.0 == wl_output_subpixel::WL_OUTPUT_SUBPIXEL_UNKNOWN.0 {
            subpixel = (*(*output).output).subpixel;
        } else if subpixel.0 != (*(*output).output).subpixel.0 {
            subpixel = wl_output_subpixel::WL_OUTPUT_SUBPIXEL_NONE;
        }
        if scale != 0 as ::core::ffi::c_int as ::core::ffi::c_float
            && scale != (*(*output).output).scale
        {
            subpixel = wl_output_subpixel::WL_OUTPUT_SUBPIXEL_NONE;
        }
        if scale < (*(*output).output).scale {
            scale = (*(*output).output).scale;
        }
        i = i.wrapping_add(1);
    }
    (*buffer).visible = (*event).size > 0 as size_t;
    if scale != (*buffer).scale || subpixel.0 != (*buffer).subpixel.0 {
        (*buffer).scale = scale;
        (*buffer).subpixel = subpixel;
        render_backing_buffer(buffer);
    }
}
unsafe extern "C" fn handle_destroy(
    mut listener: *mut wl_listener,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut buffer: *mut text_buffer = (listener as *mut ::core::ffi::c_char)
        .offset(-(120 as ::core::ffi::c_ulong as isize)) as *mut text_buffer;
    wl_list_remove(&raw mut (*buffer).outputs_update.link);
    wl_list_remove(&raw mut (*buffer).destroy.link);
    free((*buffer).text as *mut ::core::ffi::c_void);
    free(buffer as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn text_calc_size(mut buffer: *mut text_buffer) {
    let mut props: *mut sway_text_node = &raw mut (*buffer).props;
    let mut recorder: *mut cairo_surface_t = cairo_recording_surface_create(
        _cairo_content::CAIRO_CONTENT_COLOR_ALPHA,
        ::core::ptr::null::<cairo_rectangle_t>(),
    );
    let mut c: *mut cairo_t = cairo_create(recorder);
    cairo_surface_destroy(recorder);
    if cairo_status(c).0 != _cairo_status::CAIRO_STATUS_SUCCESS.0 {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] cairo_t allocation failed: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/sway_text_node.c\0".as_ptr() as *const ::core::ffi::c_char,
            206 as ::core::ffi::c_int,
            cairo_status_to_string(cairo_status(c)),
        );
    } else {
        cairo_set_antialias(c, _cairo_antialias::CAIRO_ANTIALIAS_BEST);
        get_text_size(
            c,
            (*config).font_description,
            &raw mut (*props).width,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            &raw mut (*props).baseline,
            1 as ::core::ffi::c_int as ::core::ffi::c_double,
            (*props).pango_markup,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            (*buffer).text,
        );
        wlr_scene_buffer_set_dest_size(
            (*buffer).buffer_node,
            get_text_width(props),
            (*props).height,
        );
    }
    cairo_destroy(c);
}
#[no_mangle]
pub unsafe extern "C" fn sway_text_node_create(
    mut parent: *mut wlr_scene_tree,
    mut text: *mut ::core::ffi::c_char,
    mut color: *mut ::core::ffi::c_float,
    mut pango_markup: bool,
) -> *mut sway_text_node {
    let mut buffer: *mut text_buffer = calloc(
        1 as size_t,
        ::core::mem::size_of::<text_buffer>(),
    ) as *mut text_buffer;
    if buffer.is_null() {
        return ::core::ptr::null_mut::<sway_text_node>();
    }
    let mut node: *mut wlr_scene_buffer = wlr_scene_buffer_create(
        parent,
        ::core::ptr::null_mut::<wlr_buffer>(),
    );
    if node.is_null() {
        free(buffer as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<sway_text_node>();
    }
    (*buffer).buffer_node = node;
    (*buffer).props.node = &raw mut (*node).node;
    (*buffer).props.max_width = -1 as ::core::ffi::c_int;
    (*buffer).text = strdup(text);
    if (*buffer).text.is_null() {
        free(buffer as *mut ::core::ffi::c_void);
        wlr_scene_node_destroy(&raw mut (*node).node);
        return ::core::ptr::null_mut::<sway_text_node>();
    }
    (*buffer).props.height = (*config).font_height;
    (*buffer).props.pango_markup = pango_markup;
    memcpy(
        &raw mut (*buffer).props.color as *mut ::core::ffi::c_void,
        color as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>().wrapping_mul(4 as size_t),
    );
    (*buffer).destroy.notify = Some(
        handle_destroy
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(&raw mut (*node).node.events.destroy, &raw mut (*buffer).destroy);
    (*buffer).outputs_update.notify = Some(
        handle_outputs_update
            as unsafe extern "C" fn(*mut wl_listener, *mut ::core::ffi::c_void) -> (),
    ) as wl_notify_func_t;
    wl_signal_add(
        &raw mut (*node).events.outputs_update,
        &raw mut (*buffer).outputs_update,
    );
    text_calc_size(buffer);
    return &raw mut (*buffer).props;
}
#[no_mangle]
pub unsafe extern "C" fn sway_text_node_set_color(
    mut node: *mut sway_text_node,
    mut color: *mut ::core::ffi::c_float,
) {
    if memcmp(
        &raw mut (*node).color as *const ::core::ffi::c_void,
        color as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>().wrapping_mul(4 as size_t),
    ) == 0 as ::core::ffi::c_int
    {
        return;
    }
    memcpy(
        &raw mut (*node).color as *mut ::core::ffi::c_void,
        color as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>().wrapping_mul(4 as size_t),
    );
    let mut buffer: *mut text_buffer = (node as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut text_buffer;
    render_backing_buffer(buffer);
}
#[no_mangle]
pub unsafe extern "C" fn sway_text_node_set_text(
    mut node: *mut sway_text_node,
    mut text: *mut ::core::ffi::c_char,
) {
    let mut buffer: *mut text_buffer = (node as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut text_buffer;
    if strcmp((*buffer).text, text) == 0 as ::core::ffi::c_int {
        return;
    }
    let mut new_text: *mut ::core::ffi::c_char = strdup(text);
    if new_text.is_null() {
        return;
    }
    free((*buffer).text as *mut ::core::ffi::c_void);
    (*buffer).text = new_text;
    text_calc_size(buffer);
    render_backing_buffer(buffer);
}
#[no_mangle]
pub unsafe extern "C" fn sway_text_node_set_max_width(
    mut node: *mut sway_text_node,
    mut max_width: ::core::ffi::c_int,
) {
    let mut buffer: *mut text_buffer = (node as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut text_buffer;
    if max_width == (*buffer).props.max_width {
        return;
    }
    (*buffer).props.max_width = max_width;
    wlr_scene_buffer_set_dest_size(
        (*buffer).buffer_node,
        get_text_width(&raw mut (*buffer).props),
        (*buffer).props.height,
    );
    render_backing_buffer(buffer);
}
#[no_mangle]
pub unsafe extern "C" fn sway_text_node_set_background(
    mut node: *mut sway_text_node,
    mut background: *mut ::core::ffi::c_float,
) {
    let mut buffer: *mut text_buffer = (node as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut text_buffer;
    if memcmp(
        &raw mut (*node).background as *const ::core::ffi::c_void,
        background as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>().wrapping_mul(4 as size_t),
    ) == 0 as ::core::ffi::c_int
    {
        return;
    }
    memcpy(
        &raw mut (*node).background as *mut ::core::ffi::c_void,
        background as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_float>().wrapping_mul(4 as size_t),
    );
    render_backing_buffer(buffer);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
