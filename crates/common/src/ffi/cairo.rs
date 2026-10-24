extern "C" {
    pub type _cairo;
    pub type _cairo_surface;
    fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    fn cairo_destroy(cr: *mut cairo_t);
    fn cairo_set_source_rgba(
        cr: *mut cairo_t,
        red: ::core::ffi::c_double,
        green: ::core::ffi::c_double,
        blue: ::core::ffi::c_double,
        alpha: ::core::ffi::c_double,
    );
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
    fn cairo_paint(cr: *mut cairo_t);
    fn cairo_image_surface_create(
        format: cairo_format_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_image_surface_get_width(
        surface: *mut cairo_surface_t,
    ) -> ::core::ffi::c_int;
    fn cairo_image_surface_get_height(
        surface: *mut cairo_surface_t,
    ) -> ::core::ffi::c_int;
}
pub type uint32_t = u32;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
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
pub struct wl_output_subpixel(pub ::core::ffi::c_uint);
impl wl_output_subpixel {
    pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: Self = Self(0);
    pub const WL_OUTPUT_SUBPIXEL_NONE: Self = Self(1);
    pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: Self = Self(2);
    pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: Self = Self(3);
    pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: Self = Self(4);
    pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: Self = Self(5);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_set_source_u32(
    mut cairo: *mut cairo_t,
    mut color: uint32_t,
) {
    cairo_set_source_rgba(
        cairo,
        (color >> 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int & 0xff as uint32_t)
            as ::core::ffi::c_double / 255.0f64,
        (color >> 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int & 0xff as uint32_t)
            as ::core::ffi::c_double / 255.0f64,
        (color >> 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int & 0xff as uint32_t)
            as ::core::ffi::c_double / 255.0f64,
        (color >> 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int & 0xff as uint32_t)
            as ::core::ffi::c_double / 255.0f64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn to_cairo_subpixel_order(
    mut subpixel: wl_output_subpixel,
) -> cairo_subpixel_order_t {
    match subpixel {
        wl_output_subpixel::WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB => {
            return _cairo_subpixel_order::CAIRO_SUBPIXEL_ORDER_RGB;
        }
        wl_output_subpixel::WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR => {
            return _cairo_subpixel_order::CAIRO_SUBPIXEL_ORDER_BGR;
        }
        wl_output_subpixel::WL_OUTPUT_SUBPIXEL_VERTICAL_RGB => {
            return _cairo_subpixel_order::CAIRO_SUBPIXEL_ORDER_VRGB;
        }
        wl_output_subpixel::WL_OUTPUT_SUBPIXEL_VERTICAL_BGR => {
            return _cairo_subpixel_order::CAIRO_SUBPIXEL_ORDER_VBGR;
        }
        _ => return _cairo_subpixel_order::CAIRO_SUBPIXEL_ORDER_DEFAULT,
    };
}
#[no_mangle]
pub unsafe extern "C" fn cairo_image_surface_scale(
    mut image: *mut cairo_surface_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> *mut cairo_surface_t {
    let mut image_width: ::core::ffi::c_int = cairo_image_surface_get_width(image);
    let mut image_height: ::core::ffi::c_int = cairo_image_surface_get_height(image);
    let mut new: *mut cairo_surface_t = cairo_image_surface_create(
        _cairo_format::CAIRO_FORMAT_ARGB32,
        width,
        height,
    );
    let mut cairo: *mut cairo_t = cairo_create(new);
    cairo_scale(
        cairo,
        width as ::core::ffi::c_double / image_width as ::core::ffi::c_double,
        height as ::core::ffi::c_double / image_height as ::core::ffi::c_double,
    );
    cairo_set_source_surface(
        cairo,
        image,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    cairo_paint(cairo);
    cairo_destroy(cairo);
    return new;
}
