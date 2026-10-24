extern "C" {
    pub type _PangoFontDescription;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn list_free(list: *mut list_t);
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn _sway_abort(filename: *const ::core::ffi::c_char, ...);
    fn pango_font_description_copy(
        desc: *const PangoFontDescription,
    ) -> *mut PangoFontDescription;
    fn pango_font_description_free(desc: *mut PangoFontDescription);
    fn pango_font_description_from_string(
        str: *const ::core::ffi::c_char,
    ) -> *mut PangoFontDescription;
}
pub type size_t = usize;
pub type int32_t = i32;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
pub type ssize_t = isize;
pub type PangoFontDescription = _PangoFontDescription;
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
pub struct zwlr_layer_surface_v1_anchor(pub ::core::ffi::c_uint);
impl zwlr_layer_surface_v1_anchor {
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_TOP: Self = Self(1);
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_BOTTOM: Self = Self(2);
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_LEFT: Self = Self(4);
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_RIGHT: Self = Self(8);
}
#[no_mangle]
pub unsafe extern "C" fn swaynag_type_new(
    mut name: *const ::core::ffi::c_char,
) -> *mut swaynag_type {
    let mut r#type: *mut swaynag_type = calloc(
        1 as size_t,
        ::core::mem::size_of::<swaynag_type>(),
    ) as *mut swaynag_type;
    if r#type.is_null() {
        _sway_abort(
            b"[%s:%d] Failed to allocate type: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaynag/types.c\0".as_ptr() as *const ::core::ffi::c_char,
            17 as ::core::ffi::c_int,
            name,
        );
    }
    (*r#type).name = strdup(name);
    (*r#type).bar_border_thickness = -1 as ssize_t;
    (*r#type).message_padding = -1 as ssize_t;
    (*r#type).details_border_thickness = -1 as ssize_t;
    (*r#type).button_border_thickness = -1 as ssize_t;
    (*r#type).button_gap = -1 as ssize_t;
    (*r#type).button_gap_close = -1 as ssize_t;
    (*r#type).button_margin_right = -1 as ssize_t;
    (*r#type).button_padding = -1 as ssize_t;
    (*r#type).layer = -1 as ::core::ffi::c_int as int32_t;
    return r#type;
}
#[no_mangle]
pub unsafe extern "C" fn swaynag_types_add_default(mut types: *mut list_t) {
    let mut type_defaults: *mut swaynag_type = swaynag_type_new(
        b"<defaults>\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*type_defaults).font_description = pango_font_description_from_string(
        b"pango:Monospace 10\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*type_defaults).anchors = (zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_TOP
        .0 as ::core::ffi::c_int
        | zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_LEFT.0
            as ::core::ffi::c_int
        | zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_RIGHT.0
            as ::core::ffi::c_int) as uint32_t;
    (*type_defaults).layer = zwlr_layer_shell_v1_layer::ZWLR_LAYER_SHELL_V1_LAYER_TOP.0
        as ::core::ffi::c_int as int32_t;
    (*type_defaults).button_background = 0x333333ff as uint32_t;
    (*type_defaults).details_background = 0x333333ff as uint32_t;
    (*type_defaults).background = 0x323232ff as uint32_t;
    (*type_defaults).text = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    (*type_defaults).button_text = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    (*type_defaults).border = 0x222222ff as uint32_t;
    (*type_defaults).border_bottom = 0x444444ff as uint32_t;
    (*type_defaults).bar_border_thickness = 2 as ssize_t;
    (*type_defaults).message_padding = 8 as ssize_t;
    (*type_defaults).details_border_thickness = 3 as ssize_t;
    (*type_defaults).button_border_thickness = 3 as ssize_t;
    (*type_defaults).button_gap = 20 as ssize_t;
    (*type_defaults).button_gap_close = 15 as ssize_t;
    (*type_defaults).button_margin_right = 2 as ssize_t;
    (*type_defaults).button_padding = 3 as ssize_t;
    list_add(types, type_defaults as *mut ::core::ffi::c_void);
    let mut type_error: *mut swaynag_type = swaynag_type_new(
        b"error\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*type_error).button_background = 0x680a0aff as uint32_t;
    (*type_error).details_background = 0x680a0aff as uint32_t;
    (*type_error).background = 0x900000ff as ::core::ffi::c_uint as uint32_t;
    (*type_error).text = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    (*type_error).button_text = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    (*type_error).border = 0xd92424ff as ::core::ffi::c_uint as uint32_t;
    (*type_error).border_bottom = 0x470909ff as uint32_t;
    list_add(types, type_error as *mut ::core::ffi::c_void);
    let mut type_warning: *mut swaynag_type = swaynag_type_new(
        b"warning\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*type_warning).button_background = 0xffc100ff as ::core::ffi::c_uint as uint32_t;
    (*type_warning).details_background = 0xffc100ff as ::core::ffi::c_uint as uint32_t;
    (*type_warning).background = 0xffa800ff as ::core::ffi::c_uint as uint32_t;
    (*type_warning).text = 0xff as uint32_t;
    (*type_warning).button_text = 0xff as uint32_t;
    (*type_warning).border = 0xab7100ff as ::core::ffi::c_uint as uint32_t;
    (*type_warning).border_bottom = 0xab7100ff as ::core::ffi::c_uint as uint32_t;
    list_add(types, type_warning as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn swaynag_type_get(
    mut types: *mut list_t,
    mut name: *mut ::core::ffi::c_char,
) -> *mut swaynag_type {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*types).length {
        let mut r#type: *mut swaynag_type = *(*types).items.offset(i as isize)
            as *mut swaynag_type;
        if strcasecmp((*r#type).name, name) == 0 as ::core::ffi::c_int {
            return r#type;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<swaynag_type>();
}
#[no_mangle]
pub unsafe extern "C" fn swaynag_type_merge(
    mut dest: *mut swaynag_type,
    mut src: *mut swaynag_type,
) {
    if dest.is_null() || src.is_null() {
        return;
    }
    if !(*src).font_description.is_null() {
        (*dest).font_description = pango_font_description_copy((*src).font_description);
    }
    if !(*src).output.is_null() {
        (*dest).output = strdup((*src).output);
    }
    if (*src).anchors > 0 as uint32_t {
        (*dest).anchors = (*src).anchors;
    }
    if (*src).layer >= 0 as int32_t {
        (*dest).layer = (*src).layer;
    }
    if (*src).button_background > 0 as uint32_t {
        (*dest).button_background = (*src).button_background;
    }
    if (*src).details_background > 0 as uint32_t {
        (*dest).details_background = (*src).details_background;
    }
    if (*src).background > 0 as uint32_t {
        (*dest).background = (*src).background;
    }
    if (*src).text > 0 as uint32_t {
        (*dest).text = (*src).text;
    }
    if (*src).button_text > 0 as uint32_t {
        (*dest).button_text = (*src).button_text;
    }
    if (*src).border > 0 as uint32_t {
        (*dest).border = (*src).border;
    }
    if (*src).border_bottom > 0 as uint32_t {
        (*dest).border_bottom = (*src).border_bottom;
    }
    if (*src).bar_border_thickness > -1 as ssize_t {
        (*dest).bar_border_thickness = (*src).bar_border_thickness;
    }
    if (*src).message_padding > -1 as ssize_t {
        (*dest).message_padding = (*src).message_padding;
    }
    if (*src).details_border_thickness > -1 as ssize_t {
        (*dest).details_border_thickness = (*src).details_border_thickness;
    }
    if (*src).button_border_thickness > -1 as ssize_t {
        (*dest).button_border_thickness = (*src).button_border_thickness;
    }
    if (*src).button_gap > -1 as ssize_t {
        (*dest).button_gap = (*src).button_gap;
    }
    if (*src).button_gap_close > -1 as ssize_t {
        (*dest).button_gap_close = (*src).button_gap_close;
    }
    if (*src).button_margin_right > -1 as ssize_t {
        (*dest).button_margin_right = (*src).button_margin_right;
    }
    if (*src).button_padding > -1 as ssize_t {
        (*dest).button_padding = (*src).button_padding;
    }
}
#[no_mangle]
pub unsafe extern "C" fn swaynag_type_free(mut r#type: *mut swaynag_type) {
    free((*r#type).name as *mut ::core::ffi::c_void);
    pango_font_description_free((*r#type).font_description);
    free((*r#type).output as *mut ::core::ffi::c_void);
    free(r#type as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn swaynag_types_free(mut types: *mut list_t) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*types).length {
        swaynag_type_free(*(*types).items.offset(i as isize) as *mut swaynag_type);
        i += 1;
    }
    list_free(types);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
