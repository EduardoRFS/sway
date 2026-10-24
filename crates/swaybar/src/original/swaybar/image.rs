extern "C" {
    pub type _cairo_surface;
    pub type _GdkPixbuf;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn cairo_status_to_string(status: cairo_status_t) -> *const ::core::ffi::c_char;
    fn cairo_surface_status(surface: *mut cairo_surface_t) -> cairo_status_t;
    fn cairo_surface_flush(surface: *mut cairo_surface_t);
    fn cairo_surface_mark_dirty(surface: *mut cairo_surface_t);
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
    fn g_object_unref(object: gpointer);
    fn gdk_pixbuf_get_n_channels(pixbuf: *const GdkPixbuf) -> ::core::ffi::c_int;
    fn gdk_pixbuf_get_width(pixbuf: *const GdkPixbuf) -> ::core::ffi::c_int;
    fn gdk_pixbuf_get_height(pixbuf: *const GdkPixbuf) -> ::core::ffi::c_int;
    fn gdk_pixbuf_get_rowstride(pixbuf: *const GdkPixbuf) -> ::core::ffi::c_int;
    fn gdk_pixbuf_read_pixels(pixbuf: *const GdkPixbuf) -> *const guint8;
    fn gdk_pixbuf_new_from_file(
        filename: *const ::core::ffi::c_char,
        error: *mut *mut GError,
    ) -> *mut GdkPixbuf;
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
pub type gpointer = *mut ::core::ffi::c_void;
pub type GdkPixbuf = _GdkPixbuf;
pub type GError = _GError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type GQuark = guint32;
pub type guint32 = ::core::ffi::c_uint;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
unsafe extern "C" fn gdk_cairo_image_surface_create_from_pixbuf(
    mut gdkbuf: *const GdkPixbuf,
) -> *mut cairo_surface_t {
    let mut chan: ::core::ffi::c_int = gdk_pixbuf_get_n_channels(gdkbuf);
    if chan < 3 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cairo_surface_t>();
    }
    let mut gdkpix: *const guint8 = gdk_pixbuf_read_pixels(gdkbuf);
    if gdkpix.is_null() {
        return ::core::ptr::null_mut::<cairo_surface_t>();
    }
    let mut w: gint = gdk_pixbuf_get_width(gdkbuf);
    let mut h: gint = gdk_pixbuf_get_height(gdkbuf);
    let mut stride: ::core::ffi::c_int = gdk_pixbuf_get_rowstride(gdkbuf);
    let mut fmt: cairo_format_t = _cairo_format(
        if chan == 3 as ::core::ffi::c_int {
            _cairo_format::CAIRO_FORMAT_RGB24.0
        } else {
            _cairo_format::CAIRO_FORMAT_ARGB32.0
        },
    );
    let mut cs: *mut cairo_surface_t = cairo_image_surface_create(fmt, w, h);
    cairo_surface_flush(cs);
    if cs.is_null()
        || cairo_surface_status(cs).0 != _cairo_status::CAIRO_STATUS_SUCCESS.0
    {
        return ::core::ptr::null_mut::<cairo_surface_t>();
    }
    let mut cstride: ::core::ffi::c_int = cairo_image_surface_get_stride(cs);
    let mut cpix: *mut ::core::ffi::c_uchar = cairo_image_surface_get_data(cs);
    if chan == 3 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0;
        i = h as ::core::ffi::c_int;
        while i != 0 {
            let mut gp: *const guint8 = gdkpix;
            let mut cp: *mut ::core::ffi::c_uchar = cpix;
            let mut end: *const guint8 = gp.offset((3 as gint * w) as isize);
            while gp < end {
                *cp.offset(0isize) = *gp.offset(2isize) as ::core::ffi::c_uchar;
                *cp.offset(1isize) = *gp.offset(1isize) as ::core::ffi::c_uchar;
                *cp.offset(2isize) = *gp.offset(0isize) as ::core::ffi::c_uchar;
                gp = gp.offset(3 as ::core::ffi::c_int as isize);
                cp = cp.offset(4 as ::core::ffi::c_int as isize);
            }
            gdkpix = gdkpix.offset(stride as isize);
            cpix = cpix.offset(cstride as isize);
            i -= 1;
        }
    } else {
        let mut i_0: ::core::ffi::c_int = 0;
        i_0 = h as ::core::ffi::c_int;
        while i_0 != 0 {
            let mut gp_0: *const guint8 = gdkpix;
            let mut cp_0: *mut ::core::ffi::c_uchar = cpix;
            let mut end_0: *const guint8 = gp_0.offset((4 as gint * w) as isize);
            let mut z1: guint = 0;
            let mut z2: guint = 0;
            let mut z3: guint = 0;
            while gp_0 < end_0 {
                z1 = (*gp_0.offset(2isize) as ::core::ffi::c_int
                    * *gp_0.offset(3isize) as ::core::ffi::c_int
                    + 0x80 as ::core::ffi::c_int) as guint;
                *cp_0.offset(0isize) = (z1.wrapping_add(z1 >> 8 as ::core::ffi::c_int)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                z2 = (*gp_0.offset(1isize) as ::core::ffi::c_int
                    * *gp_0.offset(3isize) as ::core::ffi::c_int
                    + 0x80 as ::core::ffi::c_int) as guint;
                *cp_0.offset(1isize) = (z2.wrapping_add(z2 >> 8 as ::core::ffi::c_int)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                z3 = (*gp_0.offset(0isize) as ::core::ffi::c_int
                    * *gp_0.offset(3isize) as ::core::ffi::c_int
                    + 0x80 as ::core::ffi::c_int) as guint;
                *cp_0.offset(2isize) = (z3.wrapping_add(z3 >> 8 as ::core::ffi::c_int)
                    >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                *cp_0.offset(3isize) = *gp_0.offset(3isize) as ::core::ffi::c_uchar;
                gp_0 = gp_0.offset(4 as ::core::ffi::c_int as isize);
                cp_0 = cp_0.offset(4 as ::core::ffi::c_int as isize);
            }
            gdkpix = gdkpix.offset(stride as isize);
            cpix = cpix.offset(cstride as isize);
            i_0 -= 1;
        }
    }
    cairo_surface_mark_dirty(cs);
    return cs;
}
#[no_mangle]
pub unsafe extern "C" fn load_image(
    mut path: *const ::core::ffi::c_char,
) -> *mut cairo_surface_t {
    let mut image: *mut cairo_surface_t = ::core::ptr::null_mut::<cairo_surface_t>();
    let mut err: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut pixbuf: *mut GdkPixbuf = gdk_pixbuf_new_from_file(path, &raw mut err);
    if pixbuf.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to load background image (%s).\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/image.c\0".as_ptr() as *const ::core::ffi::c_char,
            113 as ::core::ffi::c_int,
            (*err).message,
        );
        return ::core::ptr::null_mut::<cairo_surface_t>();
    }
    image = gdk_cairo_image_surface_create_from_pixbuf(pixbuf);
    g_object_unref(pixbuf as gpointer);
    if image.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to read background image.\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/image.c\0".as_ptr() as *const ::core::ffi::c_char,
            123 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<cairo_surface_t>();
    }
    if cairo_surface_status(image).0 != _cairo_status::CAIRO_STATUS_SUCCESS.0 {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to read background image: %s.\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/image.c\0".as_ptr() as *const ::core::ffi::c_char,
            127 as ::core::ffi::c_int,
            cairo_status_to_string(cairo_surface_status(image)),
        );
        return ::core::ptr::null_mut::<cairo_surface_t>();
    }
    return image;
}
