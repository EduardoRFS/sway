extern "C" {
    pub type _cairo;
    pub type _cairo_font_options;
    pub type _GData;
    pub type _PangoContext;
    pub type _PangoLanguage;
    pub type _PangoFontDescription;
    pub type _PangoAttrList;
    pub type _PangoLayout;
    fn cairo_font_options_create() -> *mut cairo_font_options_t;
    fn cairo_font_options_destroy(options: *mut cairo_font_options_t);
    fn cairo_get_font_options(cr: *mut cairo_t, options: *mut cairo_font_options_t);
    fn cairo_status(cr: *mut cairo_t) -> cairo_status_t;
    fn cairo_status_to_string(status: cairo_status_t) -> *const ::core::ffi::c_char;
    fn g_error_free(error: *mut GError);
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn g_object_unref(object: gpointer);
    fn pango_font_metrics_unref(metrics: *mut PangoFontMetrics);
    fn pango_font_metrics_get_ascent(
        metrics: *mut PangoFontMetrics,
    ) -> ::core::ffi::c_int;
    fn pango_font_metrics_get_descent(
        metrics: *mut PangoFontMetrics,
    ) -> ::core::ffi::c_int;
    fn pango_attr_scale_new(scale_factor: ::core::ffi::c_double) -> *mut PangoAttribute;
    fn pango_attr_list_new() -> *mut PangoAttrList;
    fn pango_attr_list_unref(list: *mut PangoAttrList);
    fn pango_attr_list_insert(list: *mut PangoAttrList, attr: *mut PangoAttribute);
    fn pango_font_map_create_context(fontmap: *mut PangoFontMap) -> *mut PangoContext;
    fn pango_context_get_metrics(
        context: *mut PangoContext,
        desc: *const PangoFontDescription,
        language: *mut PangoLanguage,
    ) -> *mut PangoFontMetrics;
    fn pango_context_set_round_glyph_positions(
        context: *mut PangoContext,
        round_positions: gboolean,
    );
    fn pango_layout_get_context(layout: *mut PangoLayout) -> *mut PangoContext;
    fn pango_layout_set_attributes(layout: *mut PangoLayout, attrs: *mut PangoAttrList);
    fn pango_layout_set_text(
        layout: *mut PangoLayout,
        text: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    );
    fn pango_layout_set_font_description(
        layout: *mut PangoLayout,
        desc: *const PangoFontDescription,
    );
    fn pango_layout_set_single_paragraph_mode(
        layout: *mut PangoLayout,
        setting: gboolean,
    );
    fn pango_layout_get_pixel_size(
        layout: *mut PangoLayout,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
    );
    fn pango_layout_get_baseline(layout: *mut PangoLayout) -> ::core::ffi::c_int;
    fn pango_parse_markup(
        markup_text: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
        accel_marker: gunichar,
        attr_list: *mut *mut PangoAttrList,
        text: *mut *mut ::core::ffi::c_char,
        accel_char: *mut gunichar,
        error: *mut *mut GError,
    ) -> gboolean;
    fn pango_cairo_font_map_get_default() -> *mut PangoFontMap;
    fn pango_cairo_context_set_font_options(
        context: *mut PangoContext,
        options: *const cairo_font_options_t,
    );
    fn pango_cairo_create_layout(cr: *mut cairo_t) -> *mut PangoLayout;
    fn pango_cairo_update_layout(cr: *mut cairo_t, layout: *mut PangoLayout);
    fn pango_cairo_show_layout(cr: *mut cairo_t, layout: *mut PangoLayout);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn lenient_strcat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn vformat_str(
        fmt: *const ::core::ffi::c_char,
        args: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
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
pub type cairo_t = _cairo;
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
pub type cairo_font_options_t = _cairo_font_options;
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type va_list = __builtin_va_list;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GData = _GData;
pub type gunichar = guint32;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoFontMap {
    pub parent_instance: GObject,
}
pub type PangoFontMap = _PangoFontMap;
pub type PangoContext = _PangoContext;
pub type PangoLanguage = _PangoLanguage;
pub type PangoFontDescription = _PangoFontDescription;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoFontMetrics {
    pub ref_count: guint,
    pub ascent: ::core::ffi::c_int,
    pub descent: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub approximate_char_width: ::core::ffi::c_int,
    pub approximate_digit_width: ::core::ffi::c_int,
    pub underline_position: ::core::ffi::c_int,
    pub underline_thickness: ::core::ffi::c_int,
    pub strikethrough_position: ::core::ffi::c_int,
    pub strikethrough_thickness: ::core::ffi::c_int,
}
pub type PangoFontMetrics = _PangoFontMetrics;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoAttribute {
    pub klass: *const PangoAttrClass,
    pub start_index: guint,
    pub end_index: guint,
}
pub type PangoAttrClass = _PangoAttrClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoAttrClass {
    pub r#type: PangoAttrType,
    pub copy: Option<unsafe extern "C" fn(*const PangoAttribute) -> *mut PangoAttribute>,
    pub destroy: Option<unsafe extern "C" fn(*mut PangoAttribute) -> ()>,
    pub equal: Option<
        unsafe extern "C" fn(*const PangoAttribute, *const PangoAttribute) -> gboolean,
    >,
}
pub type PangoAttribute = _PangoAttribute;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PangoAttrType(pub ::core::ffi::c_uint);
impl PangoAttrType {
    pub const PANGO_ATTR_INVALID: Self = Self(0);
    pub const PANGO_ATTR_LANGUAGE: Self = Self(1);
    pub const PANGO_ATTR_FAMILY: Self = Self(2);
    pub const PANGO_ATTR_STYLE: Self = Self(3);
    pub const PANGO_ATTR_WEIGHT: Self = Self(4);
    pub const PANGO_ATTR_VARIANT: Self = Self(5);
    pub const PANGO_ATTR_STRETCH: Self = Self(6);
    pub const PANGO_ATTR_SIZE: Self = Self(7);
    pub const PANGO_ATTR_FONT_DESC: Self = Self(8);
    pub const PANGO_ATTR_FOREGROUND: Self = Self(9);
    pub const PANGO_ATTR_BACKGROUND: Self = Self(10);
    pub const PANGO_ATTR_UNDERLINE: Self = Self(11);
    pub const PANGO_ATTR_STRIKETHROUGH: Self = Self(12);
    pub const PANGO_ATTR_RISE: Self = Self(13);
    pub const PANGO_ATTR_SHAPE: Self = Self(14);
    pub const PANGO_ATTR_SCALE: Self = Self(15);
    pub const PANGO_ATTR_FALLBACK: Self = Self(16);
    pub const PANGO_ATTR_LETTER_SPACING: Self = Self(17);
    pub const PANGO_ATTR_UNDERLINE_COLOR: Self = Self(18);
    pub const PANGO_ATTR_STRIKETHROUGH_COLOR: Self = Self(19);
    pub const PANGO_ATTR_ABSOLUTE_SIZE: Self = Self(20);
    pub const PANGO_ATTR_GRAVITY: Self = Self(21);
    pub const PANGO_ATTR_GRAVITY_HINT: Self = Self(22);
    pub const PANGO_ATTR_FONT_FEATURES: Self = Self(23);
    pub const PANGO_ATTR_FOREGROUND_ALPHA: Self = Self(24);
    pub const PANGO_ATTR_BACKGROUND_ALPHA: Self = Self(25);
    pub const PANGO_ATTR_ALLOW_BREAKS: Self = Self(26);
    pub const PANGO_ATTR_SHOW: Self = Self(27);
    pub const PANGO_ATTR_INSERT_HYPHENS: Self = Self(28);
    pub const PANGO_ATTR_OVERLINE: Self = Self(29);
    pub const PANGO_ATTR_OVERLINE_COLOR: Self = Self(30);
    pub const PANGO_ATTR_LINE_HEIGHT: Self = Self(31);
    pub const PANGO_ATTR_ABSOLUTE_LINE_HEIGHT: Self = Self(32);
    pub const PANGO_ATTR_TEXT_TRANSFORM: Self = Self(33);
    pub const PANGO_ATTR_WORD: Self = Self(34);
    pub const PANGO_ATTR_SENTENCE: Self = Self(35);
    pub const PANGO_ATTR_BASELINE_SHIFT: Self = Self(36);
    pub const PANGO_ATTR_FONT_SCALE: Self = Self(37);
}
pub type PangoAttrList = _PangoAttrList;
pub type PangoLayout = _PangoLayout;
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
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PANGO_SCALE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn escape_markup_text(
    mut src: *const ::core::ffi::c_char,
    mut dest: *mut ::core::ffi::c_char,
) -> size_t {
    let mut length: size_t = 0 as size_t;
    if !dest.is_null() {
        *dest.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    while *src.offset(0isize) != 0 {
        match *src.offset(0isize) as ::core::ffi::c_int {
            38 => {
                length = length.wrapping_add(5 as size_t);
                lenient_strcat(dest, b"&amp;\0".as_ptr() as *const ::core::ffi::c_char);
            }
            60 => {
                length = length.wrapping_add(4 as size_t);
                lenient_strcat(dest, b"&lt;\0".as_ptr() as *const ::core::ffi::c_char);
            }
            62 => {
                length = length.wrapping_add(4 as size_t);
                lenient_strcat(dest, b"&gt;\0".as_ptr() as *const ::core::ffi::c_char);
            }
            39 => {
                length = length.wrapping_add(6 as size_t);
                lenient_strcat(dest, b"&apos;\0".as_ptr() as *const ::core::ffi::c_char);
            }
            34 => {
                length = length.wrapping_add(6 as size_t);
                lenient_strcat(dest, b"&quot;\0".as_ptr() as *const ::core::ffi::c_char);
            }
            _ => {
                if !dest.is_null() {
                    *dest.offset(length as isize) = *src;
                    *dest.offset(length.wrapping_add(1 as size_t) as isize) = '\0'
                        as ::core::ffi::c_char;
                }
                length = length.wrapping_add(1 as size_t);
            }
        }
        src = src.offset(1);
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn get_pango_layout(
    mut cairo: *mut cairo_t,
    mut desc: *const PangoFontDescription,
    mut text: *const ::core::ffi::c_char,
    mut scale: ::core::ffi::c_double,
    mut markup: bool,
) -> *mut PangoLayout {
    let mut layout: *mut PangoLayout = pango_cairo_create_layout(cairo);
    pango_context_set_round_glyph_positions(pango_layout_get_context(layout), r#false);
    let mut attrs: *mut PangoAttrList = ::core::ptr::null_mut::<PangoAttrList>();
    if markup {
        let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        if pango_parse_markup(
            text,
            -1 as ::core::ffi::c_int,
            0 as gunichar,
            &raw mut attrs,
            &raw mut buf,
            ::core::ptr::null_mut::<gunichar>(),
            &raw mut error,
        ) != 0
        {
            pango_layout_set_text(layout, buf, -1 as ::core::ffi::c_int);
            free(buf as *mut ::core::ffi::c_void);
        } else {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] pango_parse_markup '%s' -> error %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"common/pango.c\0".as_ptr() as *const ::core::ffi::c_char,
                66 as ::core::ffi::c_int,
                text,
                (*error).message,
            );
            g_error_free(error);
            markup = r#false != 0;
        }
    }
    if !markup {
        attrs = pango_attr_list_new();
        pango_layout_set_text(layout, text, -1 as ::core::ffi::c_int);
    }
    pango_attr_list_insert(attrs, pango_attr_scale_new(scale));
    pango_layout_set_font_description(layout, desc);
    pango_layout_set_single_paragraph_mode(layout, 1 as gboolean);
    pango_layout_set_attributes(layout, attrs);
    pango_attr_list_unref(attrs);
    return layout;
}
#[no_mangle]
pub unsafe extern "C" fn get_text_size(
    mut cairo: *mut cairo_t,
    mut desc: *const PangoFontDescription,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
    mut baseline: *mut ::core::ffi::c_int,
    mut scale: ::core::ffi::c_double,
    mut markup: bool,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    if !width.is_null() {
        *width = 0 as ::core::ffi::c_int;
    }
    if !height.is_null() {
        *height = 0 as ::core::ffi::c_int;
    }
    if !baseline.is_null() {
        *baseline = 0 as ::core::ffi::c_int;
    }
    let mut args: ::core::ffi::VaList<'_>;
    args = c2rust_args.clone();
    let mut buf: *mut ::core::ffi::c_char = vformat_str(fmt, args.clone());
    if buf.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to format string\0".as_ptr() as *const ::core::ffi::c_char,
            b"common/pango.c\0".as_ptr() as *const ::core::ffi::c_char,
            102 as ::core::ffi::c_int,
        );
        return;
    }
    let mut layout: *mut PangoLayout = get_pango_layout(cairo, desc, buf, scale, markup);
    pango_cairo_update_layout(cairo, layout);
    let mut status: cairo_status_t = cairo_status(cairo);
    if status.0 != _cairo_status::CAIRO_STATUS_SUCCESS.0 {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] pango_cairo_update_layout() failed: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"common/pango.c\0".as_ptr() as *const ::core::ffi::c_char,
            110 as ::core::ffi::c_int,
            cairo_status_to_string(status),
        );
    } else {
        pango_layout_get_pixel_size(layout, width, height);
        if !baseline.is_null() {
            *baseline = pango_layout_get_baseline(layout) / PANGO_SCALE;
        }
    }
    g_object_unref(layout as gpointer);
    free(buf as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn get_text_metrics(
    mut description: *const PangoFontDescription,
    mut height: *mut ::core::ffi::c_int,
    mut baseline: *mut ::core::ffi::c_int,
) {
    let mut fontmap: *mut PangoFontMap = pango_cairo_font_map_get_default();
    let mut pango: *mut PangoContext = pango_font_map_create_context(fontmap);
    pango_context_set_round_glyph_positions(pango, r#false);
    let mut metrics: *mut PangoFontMetrics = pango_context_get_metrics(
        pango,
        description,
        ::core::ptr::null_mut::<PangoLanguage>(),
    );
    *baseline = pango_font_metrics_get_ascent(metrics) / PANGO_SCALE;
    *height = *baseline + pango_font_metrics_get_descent(metrics) / PANGO_SCALE;
    pango_font_metrics_unref(metrics);
    g_object_unref(pango as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn render_text(
    mut cairo: *mut cairo_t,
    mut desc: *const PangoFontDescription,
    mut scale: ::core::ffi::c_double,
    mut markup: bool,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut args: ::core::ffi::VaList<'_>;
    args = c2rust_args.clone();
    let mut buf: *mut ::core::ffi::c_char = vformat_str(fmt, args.clone());
    if buf.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to format string\0".as_ptr() as *const ::core::ffi::c_char,
            b"common/pango.c\0".as_ptr() as *const ::core::ffi::c_char,
            146 as ::core::ffi::c_int,
        );
        return;
    }
    let mut layout: *mut PangoLayout = get_pango_layout(cairo, desc, buf, scale, markup);
    let mut fo: *mut cairo_font_options_t = cairo_font_options_create();
    cairo_get_font_options(cairo, fo);
    pango_cairo_context_set_font_options(pango_layout_get_context(layout), fo);
    cairo_font_options_destroy(fo);
    pango_cairo_update_layout(cairo, layout);
    let mut status: cairo_status_t = cairo_status(cairo);
    if status.0 != _cairo_status::CAIRO_STATUS_SUCCESS.0 {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] pango_cairo_update_layout() failed: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"common/pango.c\0".as_ptr() as *const ::core::ffi::c_char,
            159 as ::core::ffi::c_int,
            cairo_status_to_string(status),
        );
    } else {
        pango_cairo_show_layout(cairo, layout);
    }
    g_object_unref(layout as gpointer);
    free(buf as *mut ::core::ffi::c_void);
}
