use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_surface;
    pub type wl_proxy;
    pub type wl_display;
    pub type wl_buffer;
    pub type wl_compositor;
    pub type wl_output;
    pub type wl_seat;
    pub type wl_shm;
    pub type wl_surface;
    pub type _PangoContext;
    pub type _PangoFontDescription;
    pub type _PangoLayout;
    pub type wp_cursor_shape_manager_v1;
    pub type zwlr_layer_surface_v1;
    pub type zwlr_layer_shell_v1;
    fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    fn cairo_destroy(cr: *mut cairo_t);
    fn cairo_save(cr: *mut cairo_t);
    fn cairo_restore(cr: *mut cairo_t);
    fn cairo_set_operator(cr: *mut cairo_t, op: cairo_operator_t);
    fn cairo_set_source_surface(
        cr: *mut cairo_t,
        surface: *mut cairo_surface_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    );
    fn cairo_scale(
        cr: *mut cairo_t,
        sx: ::core::ffi::c_double,
        sy: ::core::ffi::c_double,
    );
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
    fn cairo_paint(cr: *mut cairo_t);
    fn cairo_fill(cr: *mut cairo_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_recording_surface_create(
        content: cairo_content_t,
        extents: *const cairo_rectangle_t,
    ) -> *mut cairo_surface_t;
    fn wl_proxy_marshal_flags(
        proxy: *mut wl_proxy,
        opcode: uint32_t,
        interface: *const wl_interface,
        version: uint32_t,
        flags: uint32_t,
        ...
    ) -> *mut wl_proxy;
    fn wl_proxy_get_version(proxy: *mut wl_proxy) -> uint32_t;
    fn wl_display_roundtrip(display: *mut wl_display) -> ::core::ffi::c_int;
    fn cairo_set_source_u32(cairo: *mut cairo_t, color: uint32_t);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_object_unref(object: gpointer);
    fn pango_layout_set_text(
        layout: *mut PangoLayout,
        text: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    );
    fn pango_layout_get_text(layout: *mut PangoLayout) -> *const ::core::ffi::c_char;
    fn pango_layout_set_width(layout: *mut PangoLayout, width: ::core::ffi::c_int);
    fn pango_layout_set_height(layout: *mut PangoLayout, height: ::core::ffi::c_int);
    fn pango_layout_set_wrap(layout: *mut PangoLayout, wrap: PangoWrapMode);
    fn pango_layout_set_single_paragraph_mode(
        layout: *mut PangoLayout,
        setting: gboolean,
    );
    fn pango_layout_set_ellipsize(
        layout: *mut PangoLayout,
        ellipsize: PangoEllipsizeMode,
    );
    fn pango_layout_get_pixel_size(
        layout: *mut PangoLayout,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
    );
    fn pango_layout_get_line_count(layout: *mut PangoLayout) -> ::core::ffi::c_int;
    fn pango_layout_get_line_readonly(
        layout: *mut PangoLayout,
        line: ::core::ffi::c_int,
    ) -> *mut PangoLayoutLine;
    fn pango_cairo_update_layout(cr: *mut cairo_t, layout: *mut PangoLayout);
    fn pango_cairo_show_layout(cr: *mut cairo_t, layout: *mut PangoLayout);
    fn get_pango_layout(
        cairo: *mut cairo_t,
        desc: *const PangoFontDescription,
        text: *const ::core::ffi::c_char,
        scale: ::core::ffi::c_double,
        markup: bool,
    ) -> *mut PangoLayout;
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
    fn get_next_buffer(
        shm: *mut wl_shm,
        pool: *mut pool_buffer,
        width: uint32_t,
        height: uint32_t,
    ) -> *mut pool_buffer;
}
pub type int32_t = i32;
pub type uint32_t = u32;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
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
pub struct _cairo_operator(pub ::core::ffi::c_uint);
impl _cairo_operator {
    pub const CAIRO_OPERATOR_CLEAR: Self = Self(0);
    pub const CAIRO_OPERATOR_SOURCE: Self = Self(1);
    pub const CAIRO_OPERATOR_OVER: Self = Self(2);
    pub const CAIRO_OPERATOR_IN: Self = Self(3);
    pub const CAIRO_OPERATOR_OUT: Self = Self(4);
    pub const CAIRO_OPERATOR_ATOP: Self = Self(5);
    pub const CAIRO_OPERATOR_DEST: Self = Self(6);
    pub const CAIRO_OPERATOR_DEST_OVER: Self = Self(7);
    pub const CAIRO_OPERATOR_DEST_IN: Self = Self(8);
    pub const CAIRO_OPERATOR_DEST_OUT: Self = Self(9);
    pub const CAIRO_OPERATOR_DEST_ATOP: Self = Self(10);
    pub const CAIRO_OPERATOR_XOR: Self = Self(11);
    pub const CAIRO_OPERATOR_ADD: Self = Self(12);
    pub const CAIRO_OPERATOR_SATURATE: Self = Self(13);
    pub const CAIRO_OPERATOR_MULTIPLY: Self = Self(14);
    pub const CAIRO_OPERATOR_SCREEN: Self = Self(15);
    pub const CAIRO_OPERATOR_OVERLAY: Self = Self(16);
    pub const CAIRO_OPERATOR_DARKEN: Self = Self(17);
    pub const CAIRO_OPERATOR_LIGHTEN: Self = Self(18);
    pub const CAIRO_OPERATOR_COLOR_DODGE: Self = Self(19);
    pub const CAIRO_OPERATOR_COLOR_BURN: Self = Self(20);
    pub const CAIRO_OPERATOR_HARD_LIGHT: Self = Self(21);
    pub const CAIRO_OPERATOR_SOFT_LIGHT: Self = Self(22);
    pub const CAIRO_OPERATOR_DIFFERENCE: Self = Self(23);
    pub const CAIRO_OPERATOR_EXCLUSION: Self = Self(24);
    pub const CAIRO_OPERATOR_HSL_HUE: Self = Self(25);
    pub const CAIRO_OPERATOR_HSL_SATURATION: Self = Self(26);
    pub const CAIRO_OPERATOR_HSL_COLOR: Self = Self(27);
    pub const CAIRO_OPERATOR_HSL_LUMINOSITY: Self = Self(28);
}
pub type cairo_operator_t = _cairo_operator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
}
pub type cairo_rectangle_t = _cairo_rectangle;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_message {
    pub name: *const ::core::ffi::c_char,
    pub signature: *const ::core::ffi::c_char,
    pub types: *mut *const wl_interface,
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
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
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
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type ssize_t = isize;
pub type PangoContext = _PangoContext;
pub type PangoFontDescription = _PangoFontDescription;
pub type PangoLayout = _PangoLayout;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _PangoLayoutLine {
    pub layout: *mut PangoLayout,
    pub start_index: gint,
    pub length: gint,
    pub runs: *mut GSList,
    #[bitfield(name = "is_paragraph_start", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "resolved_dir", ty = "guint", bits = "1..=3")]
    pub is_paragraph_start_resolved_dir: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type PangoLayoutLine = _PangoLayoutLine;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PangoWrapMode(pub ::core::ffi::c_uint);
impl PangoWrapMode {
    pub const PANGO_WRAP_WORD: Self = Self(0);
    pub const PANGO_WRAP_CHAR: Self = Self(1);
    pub const PANGO_WRAP_WORD_CHAR: Self = Self(2);
    pub const PANGO_WRAP_NONE: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PangoEllipsizeMode(pub ::core::ffi::c_uint);
impl PangoEllipsizeMode {
    pub const PANGO_ELLIPSIZE_NONE: Self = Self(0);
    pub const PANGO_ELLIPSIZE_START: Self = Self(1);
    pub const PANGO_ELLIPSIZE_MIDDLE: Self = Self(2);
    pub const PANGO_ELLIPSIZE_END: Self = Self(3);
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
pub struct pool_buffer {
    pub buffer: *mut wl_buffer,
    pub surface: *mut cairo_surface_t,
    pub cairo: *mut cairo_t,
    pub pango: *mut PangoContext,
    pub width: uint32_t,
    pub height: uint32_t,
    pub data: *mut ::core::ffi::c_void,
    pub size: size_t,
    pub busy: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag_type {
    pub name: *mut ::core::ffi::c_char,
    pub font_description: *mut PangoFontDescription,
    pub output: *mut ::core::ffi::c_char,
    pub anchors: uint32_t,
    pub layer: int32_t,
    pub button_text: uint32_t,
    pub button_background: uint32_t,
    pub details_background: uint32_t,
    pub background: uint32_t,
    pub text: uint32_t,
    pub border: uint32_t,
    pub border_bottom: uint32_t,
    pub bar_border_thickness: ssize_t,
    pub message_padding: ssize_t,
    pub details_border_thickness: ssize_t,
    pub button_border_thickness: ssize_t,
    pub button_gap: ssize_t,
    pub button_gap_close: ssize_t,
    pub button_margin_right: ssize_t,
    pub button_padding: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag {
    pub run_display: bool,
    pub display: *mut wl_display,
    pub compositor: *mut wl_compositor,
    pub seat: *mut wl_seat,
    pub shm: *mut wl_shm,
    pub outputs: wl_list,
    pub seats: wl_list,
    pub output: *mut swaynag_output,
    pub layer_shell: *mut zwlr_layer_shell_v1,
    pub layer_surface: *mut zwlr_layer_surface_v1,
    pub cursor_shape_manager: *mut wp_cursor_shape_manager_v1,
    pub surface: *mut wl_surface,
    pub width: uint32_t,
    pub height: uint32_t,
    pub scale: int32_t,
    pub buffers: [pool_buffer; 2],
    pub current_buffer: *mut pool_buffer,
    pub r#type: *mut swaynag_type,
    pub message: *mut ::core::ffi::c_char,
    pub buttons: *mut list_t,
    pub details: swaynag_details,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag_details {
    pub visible: bool,
    pub message: *mut ::core::ffi::c_char,
    pub details_text: *mut ::core::ffi::c_char,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub offset: ::core::ffi::c_int,
    pub visible_lines: ::core::ffi::c_int,
    pub total_lines: ::core::ffi::c_int,
    pub button_details: *mut swaynag_button,
    pub button_up: swaynag_button,
    pub button_down: swaynag_button,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag_button {
    pub text: *mut ::core::ffi::c_char,
    pub r#type: swaynag_action_type,
    pub action: *mut ::core::ffi::c_char,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub terminal: bool,
    pub dismiss: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct swaynag_action_type(pub ::core::ffi::c_uint);
impl swaynag_action_type {
    pub const SWAYNAG_ACTION_DISMISS: Self = Self(0);
    pub const SWAYNAG_ACTION_EXPAND: Self = Self(1);
    pub const SWAYNAG_ACTION_COMMAND: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaynag_output {
    pub name: *mut ::core::ffi::c_char,
    pub wl_output: *mut wl_output,
    pub wl_name: uint32_t,
    pub scale: uint32_t,
    pub swaynag: *mut swaynag,
    pub link: wl_list,
}
pub const WL_SURFACE_ATTACH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WL_SURFACE_DAMAGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WL_SURFACE_COMMIT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const WL_SURFACE_SET_BUFFER_SCALE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_surface_attach(
    mut wl_surface: *mut wl_surface,
    mut buffer: *mut wl_buffer,
    mut x: int32_t,
    mut y: int32_t,
) {
    wl_proxy_marshal_flags(
        wl_surface as *mut wl_proxy,
        WL_SURFACE_ATTACH as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_surface as *mut wl_proxy),
        0 as uint32_t,
        buffer,
        x,
        y,
    );
}
#[inline]
unsafe extern "C" fn wl_surface_damage(
    mut wl_surface: *mut wl_surface,
    mut x: int32_t,
    mut y: int32_t,
    mut width: int32_t,
    mut height: int32_t,
) {
    wl_proxy_marshal_flags(
        wl_surface as *mut wl_proxy,
        WL_SURFACE_DAMAGE as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_surface as *mut wl_proxy),
        0 as uint32_t,
        x,
        y,
        width,
        height,
    );
}
#[inline]
unsafe extern "C" fn wl_surface_commit(mut wl_surface: *mut wl_surface) {
    wl_proxy_marshal_flags(
        wl_surface as *mut wl_proxy,
        WL_SURFACE_COMMIT as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_surface as *mut wl_proxy),
        0 as uint32_t,
    );
}
#[inline]
unsafe extern "C" fn wl_surface_set_buffer_scale(
    mut wl_surface: *mut wl_surface,
    mut scale: int32_t,
) {
    wl_proxy_marshal_flags(
        wl_surface as *mut wl_proxy,
        WL_SURFACE_SET_BUFFER_SCALE as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_surface as *mut wl_proxy),
        0 as uint32_t,
        scale,
    );
}
pub const PANGO_SCALE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const SWAYNAG_MAX_HEIGHT: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const ZWLR_LAYER_SURFACE_V1_SET_SIZE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ZWLR_LAYER_SURFACE_V1_SET_EXCLUSIVE_ZONE: ::core::ffi::c_int = 2
    as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn zwlr_layer_surface_v1_set_size(
    mut zwlr_layer_surface_v1: *mut zwlr_layer_surface_v1,
    mut width: uint32_t,
    mut height: uint32_t,
) {
    wl_proxy_marshal_flags(
        zwlr_layer_surface_v1 as *mut wl_proxy,
        ZWLR_LAYER_SURFACE_V1_SET_SIZE as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(zwlr_layer_surface_v1 as *mut wl_proxy),
        0 as uint32_t,
        width,
        height,
    );
}
#[inline]
unsafe extern "C" fn zwlr_layer_surface_v1_set_exclusive_zone(
    mut zwlr_layer_surface_v1: *mut zwlr_layer_surface_v1,
    mut zone: int32_t,
) {
    wl_proxy_marshal_flags(
        zwlr_layer_surface_v1 as *mut wl_proxy,
        ZWLR_LAYER_SURFACE_V1_SET_EXCLUSIVE_ZONE as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(zwlr_layer_surface_v1 as *mut wl_proxy),
        0 as uint32_t,
        zone,
    );
}
unsafe extern "C" fn render_message(
    mut cairo: *mut cairo_t,
    mut swaynag: *mut swaynag,
) -> uint32_t {
    let mut text_width: ::core::ffi::c_int = 0;
    let mut text_height: ::core::ffi::c_int = 0;
    get_text_size(
        cairo,
        (*(*swaynag).r#type).font_description,
        &raw mut text_width,
        &raw mut text_height,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        r#true != 0,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*swaynag).message,
    );
    let mut padding: ::core::ffi::c_int = (*(*swaynag).r#type).message_padding
        as ::core::ffi::c_int;
    let mut ideal_height: uint32_t = (text_height + padding * 2 as ::core::ffi::c_int)
        as uint32_t;
    let mut ideal_surface_height: uint32_t = ideal_height;
    if (*swaynag).height < ideal_surface_height {
        return ideal_surface_height;
    }
    cairo_set_source_u32(cairo, (*(*swaynag).r#type).text);
    cairo_move_to(
        cairo,
        padding as ::core::ffi::c_double,
        (ideal_height.wrapping_sub(text_height as uint32_t) as ::core::ffi::c_int
            / 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
    );
    render_text(
        cairo,
        (*(*swaynag).r#type).font_description,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        r#false != 0,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*swaynag).message,
    );
    return ideal_surface_height;
}
unsafe extern "C" fn render_details_scroll_button(
    mut cairo: *mut cairo_t,
    mut swaynag: *mut swaynag,
    mut button: *mut swaynag_button,
) {
    let mut text_width: ::core::ffi::c_int = 0;
    let mut text_height: ::core::ffi::c_int = 0;
    get_text_size(
        cairo,
        (*(*swaynag).r#type).font_description,
        &raw mut text_width,
        &raw mut text_height,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        r#true != 0,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*button).text,
    );
    let mut border: ::core::ffi::c_int = (*(*swaynag).r#type).button_border_thickness
        as ::core::ffi::c_int;
    let mut padding: ::core::ffi::c_int = (*(*swaynag).r#type).button_padding
        as ::core::ffi::c_int;
    cairo_set_source_u32(cairo, (*(*swaynag).r#type).details_background);
    cairo_rectangle(
        cairo,
        (*button).x as ::core::ffi::c_double,
        (*button).y as ::core::ffi::c_double,
        (*button).width as ::core::ffi::c_double,
        (*button).height as ::core::ffi::c_double,
    );
    cairo_fill(cairo);
    cairo_set_source_u32(cairo, (*(*swaynag).r#type).button_background);
    cairo_rectangle(
        cairo,
        ((*button).x + border) as ::core::ffi::c_double,
        ((*button).y + border) as ::core::ffi::c_double,
        ((*button).width - border * 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
        ((*button).height - border * 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
    );
    cairo_fill(cairo);
    cairo_set_source_u32(cairo, (*(*swaynag).r#type).button_text);
    cairo_move_to(
        cairo,
        ((*button).x + border + padding) as ::core::ffi::c_double,
        ((*button).y + border
            + ((*button).height - text_height) / 2 as ::core::ffi::c_int)
            as ::core::ffi::c_double,
    );
    render_text(
        cairo,
        (*(*swaynag).r#type).font_description,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        r#true != 0,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*button).text,
    );
}
unsafe extern "C" fn get_detailed_scroll_button_width(
    mut cairo: *mut cairo_t,
    mut swaynag: *mut swaynag,
) -> ::core::ffi::c_int {
    let mut up_width: ::core::ffi::c_int = 0;
    let mut down_width: ::core::ffi::c_int = 0;
    let mut temp_height: ::core::ffi::c_int = 0;
    get_text_size(
        cairo,
        (*(*swaynag).r#type).font_description,
        &raw mut up_width,
        &raw mut temp_height,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        r#true != 0,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*swaynag).details.button_up.text,
    );
    get_text_size(
        cairo,
        (*(*swaynag).r#type).font_description,
        &raw mut down_width,
        &raw mut temp_height,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        r#true != 0,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*swaynag).details.button_down.text,
    );
    let mut text_width: ::core::ffi::c_int = if up_width > down_width {
        up_width
    } else {
        down_width
    };
    let mut border: ::core::ffi::c_int = (*(*swaynag).r#type).button_border_thickness
        as ::core::ffi::c_int;
    let mut padding: ::core::ffi::c_int = (*(*swaynag).r#type).button_padding
        as ::core::ffi::c_int;
    return text_width + border * 2 as ::core::ffi::c_int
        + padding * 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn render_detailed(
    mut cairo: *mut cairo_t,
    mut swaynag: *mut swaynag,
    mut y: uint32_t,
) -> uint32_t {
    let mut width: uint32_t = (*swaynag).width;
    let mut border: ::core::ffi::c_int = (*(*swaynag).r#type).details_border_thickness
        as ::core::ffi::c_int;
    let mut padding: ::core::ffi::c_int = (*(*swaynag).r#type).message_padding
        as ::core::ffi::c_int;
    let mut decor: ::core::ffi::c_int = padding + border;
    (*swaynag).details.x = decor;
    (*swaynag).details.y = y.wrapping_add(decor as uint32_t) as ::core::ffi::c_int;
    (*swaynag).details.width = width
        .wrapping_sub((decor * 2 as ::core::ffi::c_int) as uint32_t)
        as ::core::ffi::c_int;
    let mut layout: *mut PangoLayout = get_pango_layout(
        cairo,
        (*(*swaynag).r#type).font_description,
        (*swaynag).details.message,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        r#false != 0,
    );
    pango_layout_set_width(
        layout,
        ((*swaynag).details.width - padding * 2 as ::core::ffi::c_int) * PANGO_SCALE,
    );
    pango_layout_set_wrap(layout, PangoWrapMode::PANGO_WRAP_WORD_CHAR);
    pango_layout_set_single_paragraph_mode(layout, r#false);
    pango_cairo_update_layout(cairo, layout);
    (*swaynag).details.total_lines = pango_layout_get_line_count(layout);
    let mut line: *mut PangoLayoutLine = ::core::ptr::null_mut::<PangoLayoutLine>();
    line = pango_layout_get_line_readonly(layout, (*swaynag).details.offset);
    let mut offset: gint = (*line).start_index;
    let mut text: *const ::core::ffi::c_char = pango_layout_get_text(layout);
    pango_layout_set_text(
        layout,
        text.offset(offset as isize),
        strlen(text).wrapping_sub(offset as size_t) as ::core::ffi::c_int,
    );
    let mut text_width: ::core::ffi::c_int = 0;
    let mut text_height: ::core::ffi::c_int = 0;
    pango_cairo_update_layout(cairo, layout);
    pango_layout_get_pixel_size(layout, &raw mut text_width, &raw mut text_height);
    let mut show_buttons: bool = (*swaynag).details.offset > 0 as ::core::ffi::c_int;
    let mut button_width: ::core::ffi::c_int = get_detailed_scroll_button_width(
        cairo,
        swaynag,
    );
    if show_buttons {
        (*swaynag).details.width -= button_width;
        pango_layout_set_width(
            layout,
            ((*swaynag).details.width - padding * 2 as ::core::ffi::c_int) * PANGO_SCALE,
        );
    }
    let mut ideal_height: uint32_t = 0;
    loop {
        ideal_height = ((*swaynag).details.y + text_height + decor
            + padding * 2 as ::core::ffi::c_int) as uint32_t;
        if ideal_height > SWAYNAG_MAX_HEIGHT as uint32_t {
            ideal_height = SWAYNAG_MAX_HEIGHT as uint32_t;
            if !show_buttons {
                show_buttons = r#true != 0;
                (*swaynag).details.width -= button_width;
                pango_layout_set_width(
                    layout,
                    ((*swaynag).details.width - padding * 2 as ::core::ffi::c_int)
                        * PANGO_SCALE,
                );
            }
        }
        (*swaynag).details.height = ideal_height
            .wrapping_sub((*swaynag).details.y as uint32_t)
            .wrapping_sub(decor as uint32_t) as ::core::ffi::c_int;
        pango_layout_set_height(
            layout,
            ((*swaynag).details.height - padding * 2 as ::core::ffi::c_int) * PANGO_SCALE,
        );
        pango_layout_set_ellipsize(layout, PangoEllipsizeMode::PANGO_ELLIPSIZE_END);
        pango_cairo_update_layout(cairo, layout);
        pango_layout_get_pixel_size(layout, &raw mut text_width, &raw mut text_height);
        if text_height == (*swaynag).details.height - padding * 2 as ::core::ffi::c_int {
            break;
        }
    }
    (*swaynag).details.visible_lines = pango_layout_get_line_count(layout);
    if show_buttons {
        (*swaynag).details.button_up.x = (*swaynag).details.x + (*swaynag).details.width;
        (*swaynag).details.button_up.y = (*swaynag).details.y;
        (*swaynag).details.button_up.width = button_width;
        (*swaynag).details.button_up.height = (*swaynag).details.height
            / 2 as ::core::ffi::c_int;
        render_details_scroll_button(
            cairo,
            swaynag,
            &raw mut (*swaynag).details.button_up,
        );
        (*swaynag).details.button_down.x = (*swaynag).details.x
            + (*swaynag).details.width;
        (*swaynag).details.button_down.y = (*swaynag).details.button_up.y
            + (*swaynag).details.button_up.height;
        (*swaynag).details.button_down.width = button_width;
        (*swaynag).details.button_down.height = (*swaynag).details.height
            / 2 as ::core::ffi::c_int;
        render_details_scroll_button(
            cairo,
            swaynag,
            &raw mut (*swaynag).details.button_down,
        );
    }
    cairo_set_source_u32(cairo, (*(*swaynag).r#type).details_background);
    cairo_rectangle(
        cairo,
        (*swaynag).details.x as ::core::ffi::c_double,
        (*swaynag).details.y as ::core::ffi::c_double,
        (*swaynag).details.width as ::core::ffi::c_double,
        (*swaynag).details.height as ::core::ffi::c_double,
    );
    cairo_fill(cairo);
    cairo_move_to(
        cairo,
        ((*swaynag).details.x + padding) as ::core::ffi::c_double,
        ((*swaynag).details.y + padding) as ::core::ffi::c_double,
    );
    cairo_set_source_u32(cairo, (*(*swaynag).r#type).text);
    pango_cairo_show_layout(cairo, layout);
    g_object_unref(layout as gpointer);
    return ideal_height;
}
unsafe extern "C" fn render_button(
    mut cairo: *mut cairo_t,
    mut swaynag: *mut swaynag,
    mut button_index: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_int,
) -> uint32_t {
    let mut button: *mut swaynag_button = *(*(*swaynag).buttons)
        .items
        .offset(button_index as isize) as *mut swaynag_button;
    let mut text_width: ::core::ffi::c_int = 0;
    let mut text_height: ::core::ffi::c_int = 0;
    get_text_size(
        cairo,
        (*(*swaynag).r#type).font_description,
        &raw mut text_width,
        &raw mut text_height,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        r#true != 0,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*button).text,
    );
    let mut border: ::core::ffi::c_int = (*(*swaynag).r#type).button_border_thickness
        as ::core::ffi::c_int;
    let mut padding: ::core::ffi::c_int = (*(*swaynag).r#type).button_padding
        as ::core::ffi::c_int;
    let mut ideal_height: uint32_t = (text_height + padding * 2 as ::core::ffi::c_int
        + border * 2 as ::core::ffi::c_int) as uint32_t;
    let mut ideal_surface_height: uint32_t = ideal_height;
    if (*swaynag).height < ideal_surface_height {
        return ideal_surface_height;
    }
    (*button).x = *x - border - text_width - padding * 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int;
    (*button).y = ideal_height.wrapping_sub(text_height as uint32_t)
        as ::core::ffi::c_int / 2 as ::core::ffi::c_int - padding
        + 1 as ::core::ffi::c_int;
    (*button).width = text_width + padding * 2 as ::core::ffi::c_int;
    (*button).height = text_height + padding * 2 as ::core::ffi::c_int;
    cairo_set_source_u32(cairo, (*(*swaynag).r#type).border);
    cairo_rectangle(
        cairo,
        ((*button).x - border) as ::core::ffi::c_double,
        ((*button).y - border) as ::core::ffi::c_double,
        ((*button).width + border * 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
        ((*button).height + border * 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
    );
    cairo_fill(cairo);
    cairo_set_source_u32(cairo, (*(*swaynag).r#type).button_background);
    cairo_rectangle(
        cairo,
        (*button).x as ::core::ffi::c_double,
        (*button).y as ::core::ffi::c_double,
        (*button).width as ::core::ffi::c_double,
        (*button).height as ::core::ffi::c_double,
    );
    cairo_fill(cairo);
    cairo_set_source_u32(cairo, (*(*swaynag).r#type).button_text);
    cairo_move_to(
        cairo,
        ((*button).x + padding) as ::core::ffi::c_double,
        ((*button).y + padding) as ::core::ffi::c_double,
    );
    render_text(
        cairo,
        (*(*swaynag).r#type).font_description,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        r#true != 0,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*button).text,
    );
    *x = (*button).x - border;
    return ideal_surface_height;
}
unsafe extern "C" fn render_to_cairo(
    mut cairo: *mut cairo_t,
    mut swaynag: *mut swaynag,
) -> uint32_t {
    let mut max_height: uint32_t = 0 as uint32_t;
    cairo_set_operator(cairo, _cairo_operator::CAIRO_OPERATOR_SOURCE);
    cairo_set_source_u32(cairo, (*(*swaynag).r#type).background);
    cairo_paint(cairo);
    let mut h: uint32_t = render_message(cairo, swaynag);
    max_height = if h > max_height { h } else { max_height };
    let mut x: ::core::ffi::c_int = ((*swaynag).width as ssize_t
        - (*(*swaynag).r#type).button_margin_right) as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*swaynag).buttons).length {
        h = render_button(cairo, swaynag, i, &raw mut x);
        max_height = if h > max_height { h } else { max_height };
        x = (x as ssize_t - (*(*swaynag).r#type).button_gap) as ::core::ffi::c_int;
        if i == 0 as ::core::ffi::c_int {
            x = (x as ssize_t - (*(*swaynag).r#type).button_gap_close)
                as ::core::ffi::c_int;
        }
        i += 1;
    }
    if (*swaynag).details.visible {
        h = render_detailed(cairo, swaynag, max_height);
        max_height = if h > max_height { h } else { max_height };
    }
    let mut border: ::core::ffi::c_int = (*(*swaynag).r#type).bar_border_thickness
        as ::core::ffi::c_int;
    if max_height > (*swaynag).height {
        max_height = max_height.wrapping_add(border as uint32_t);
    }
    cairo_set_source_u32(cairo, (*(*swaynag).r#type).border_bottom);
    cairo_rectangle(
        cairo,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*swaynag).height.wrapping_sub(border as uint32_t) as ::core::ffi::c_double,
        (*swaynag).width as ::core::ffi::c_double,
        border as ::core::ffi::c_double,
    );
    cairo_fill(cairo);
    return max_height;
}
#[no_mangle]
pub unsafe extern "C" fn render_frame(mut swaynag: *mut swaynag) {
    if !(*swaynag).run_display {
        return;
    }
    let mut recorder: *mut cairo_surface_t = cairo_recording_surface_create(
        _cairo_content::CAIRO_CONTENT_COLOR_ALPHA,
        ::core::ptr::null::<cairo_rectangle_t>(),
    );
    let mut cairo: *mut cairo_t = cairo_create(recorder);
    cairo_scale(
        cairo,
        (*swaynag).scale as ::core::ffi::c_double,
        (*swaynag).scale as ::core::ffi::c_double,
    );
    cairo_save(cairo);
    cairo_set_operator(cairo, _cairo_operator::CAIRO_OPERATOR_CLEAR);
    cairo_paint(cairo);
    cairo_restore(cairo);
    let mut height: uint32_t = render_to_cairo(cairo, swaynag);
    if height != (*swaynag).height {
        zwlr_layer_surface_v1_set_size((*swaynag).layer_surface, 0 as uint32_t, height);
        zwlr_layer_surface_v1_set_exclusive_zone(
            (*swaynag).layer_surface,
            height as int32_t,
        );
        wl_surface_commit((*swaynag).surface);
        wl_display_roundtrip((*swaynag).display);
    } else {
        (*swaynag).current_buffer = get_next_buffer(
            (*swaynag).shm,
            &raw mut (*swaynag).buffers as *mut pool_buffer,
            (*swaynag).width.wrapping_mul((*swaynag).scale as uint32_t),
            (*swaynag).height.wrapping_mul((*swaynag).scale as uint32_t),
        );
        if (*swaynag).current_buffer.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Failed to get buffer. Skipping frame.\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaynag/render.c\0".as_ptr() as *const ::core::ffi::c_char,
                277 as ::core::ffi::c_int,
            );
        } else {
            let mut shm: *mut cairo_t = (*(*swaynag).current_buffer).cairo;
            cairo_save(shm);
            cairo_set_operator(shm, _cairo_operator::CAIRO_OPERATOR_CLEAR);
            cairo_paint(shm);
            cairo_restore(shm);
            cairo_set_source_surface(shm, recorder, 0.0f64, 0.0f64);
            cairo_paint(shm);
            wl_surface_set_buffer_scale((*swaynag).surface, (*swaynag).scale);
            wl_surface_attach(
                (*swaynag).surface,
                (*(*swaynag).current_buffer).buffer,
                0 as int32_t,
                0 as int32_t,
            );
            wl_surface_damage(
                (*swaynag).surface,
                0 as int32_t,
                0 as int32_t,
                (*swaynag).width as int32_t,
                (*swaynag).height as int32_t,
            );
            wl_surface_commit((*swaynag).surface);
            wl_display_roundtrip((*swaynag).display);
        }
    }
    cairo_surface_destroy(recorder);
    cairo_destroy(cairo);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
