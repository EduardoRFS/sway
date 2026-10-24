extern "C" {
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn list_free_items_and_destroy(list: *mut list_t);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    fn split_string(
        str: *const ::core::ffi::c_char,
        delims: *const ::core::ffi::c_char,
    ) -> *mut list_t;
    fn format_str(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
}
pub type int8_t = i8;
pub type uint8_t = u8;
pub type uint32_t = u32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct gesture_type(pub ::core::ffi::c_uint);
impl gesture_type {
    pub const GESTURE_TYPE_NONE: Self = Self(0);
    pub const GESTURE_TYPE_HOLD: Self = Self(1);
    pub const GESTURE_TYPE_PINCH: Self = Self(2);
    pub const GESTURE_TYPE_SWIPE: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct gesture_direction(pub ::core::ffi::c_uint);
impl gesture_direction {
    pub const GESTURE_DIRECTION_NONE: Self = Self(0);
    pub const GESTURE_DIRECTION_UP: Self = Self(1);
    pub const GESTURE_DIRECTION_DOWN: Self = Self(2);
    pub const GESTURE_DIRECTION_LEFT: Self = Self(4);
    pub const GESTURE_DIRECTION_RIGHT: Self = Self(8);
    pub const GESTURE_DIRECTION_INWARD: Self = Self(16);
    pub const GESTURE_DIRECTION_OUTWARD: Self = Self(32);
    pub const GESTURE_DIRECTION_CLOCKWISE: Self = Self(64);
    pub const GESTURE_DIRECTION_COUNTERCLOCKWISE: Self = Self(128);
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gesture_tracker {
    pub r#type: gesture_type,
    pub fingers: uint8_t,
    pub dx: ::core::ffi::c_double,
    pub dy: ::core::ffi::c_double,
    pub scale: ::core::ffi::c_double,
    pub rotation: ::core::ffi::c_double,
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL_0 as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[no_mangle]
pub static mut GESTURE_FINGERS_ANY: uint8_t = 0 as uint8_t;
#[no_mangle]
pub unsafe extern "C" fn gesture_parse(
    mut input: *const ::core::ffi::c_char,
    mut output: *mut gesture,
) -> *mut ::core::ffi::c_char {
    (*output).r#type = gesture_type::GESTURE_TYPE_NONE;
    (*output).fingers = GESTURE_FINGERS_ANY;
    (*output).directions = gesture_direction::GESTURE_DIRECTION_NONE.0
        as ::core::ffi::c_int as uint32_t;
    let mut split: *mut list_t = split_string(
        input,
        b":\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if (*split).length < 1 as ::core::ffi::c_int
        || (*split).length > 3 as ::core::ffi::c_int
    {
        return format_str(
            b"expected <gesture>[:<fingers>][:direction], got %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            input,
        );
    }
    if strcmp(
        *(*split).items.offset(0isize) as *const ::core::ffi::c_char,
        b"hold\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*output).r#type = gesture_type::GESTURE_TYPE_HOLD;
    } else if strcmp(
        *(*split).items.offset(0isize) as *const ::core::ffi::c_char,
        b"pinch\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*output).r#type = gesture_type::GESTURE_TYPE_PINCH;
    } else if strcmp(
        *(*split).items.offset(0isize) as *const ::core::ffi::c_char,
        b"swipe\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*output).r#type = gesture_type::GESTURE_TYPE_SWIPE;
    } else {
        return format_str(
            b"expected hold|pinch|swipe, got %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            *(*split).items.offset(0isize) as *const ::core::ffi::c_char,
        )
    }
    if (*split).length > 1 as ::core::ffi::c_int {
        let mut next: *mut ::core::ffi::c_char = *(*split).items.offset(1isize)
            as *mut ::core::ffi::c_char;
        if strlen(next) == 1 as size_t
            && '1' as ::core::ffi::c_int <= *next.offset(0isize) as ::core::ffi::c_int
            && *next.offset(0isize) as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            (*output).fingers = atoi(next) as uint8_t;
            next = (if (*split).length == 3 as ::core::ffi::c_int {
                *(*split).items.offset(2isize)
            } else {
                NULL
            }) as *mut ::core::ffi::c_char;
        } else if (*split).length == 3 as ::core::ffi::c_int {
            return format_str(
                b"expected 1-9, got %s\0".as_ptr() as *const ::core::ffi::c_char,
                next,
            )
        }
        if !next.is_null() {
            let mut directions: *mut list_t = split_string(
                next,
                b"+\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*directions).length {
                let mut item: *const ::core::ffi::c_char = *(*directions)
                    .items
                    .offset(i as isize) as *const ::core::ffi::c_char;
                if strcmp(item, b"any\0".as_ptr() as *const ::core::ffi::c_char)
                    != 0 as ::core::ffi::c_int
                {
                    if strcmp(item, b"up\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        (*output).directions
                            |= gesture_direction::GESTURE_DIRECTION_UP.0
                                as ::core::ffi::c_int as uint32_t;
                    } else if strcmp(
                        item,
                        b"down\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*output).directions
                            |= gesture_direction::GESTURE_DIRECTION_DOWN.0
                                as ::core::ffi::c_int as uint32_t;
                    } else if strcmp(
                        item,
                        b"left\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*output).directions
                            |= gesture_direction::GESTURE_DIRECTION_LEFT.0
                                as ::core::ffi::c_int as uint32_t;
                    } else if strcmp(
                        item,
                        b"right\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*output).directions
                            |= gesture_direction::GESTURE_DIRECTION_RIGHT.0
                                as ::core::ffi::c_int as uint32_t;
                    } else if strcmp(
                        item,
                        b"inward\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*output).directions
                            |= gesture_direction::GESTURE_DIRECTION_INWARD.0
                                as ::core::ffi::c_int as uint32_t;
                    } else if strcmp(
                        item,
                        b"outward\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*output).directions
                            |= gesture_direction::GESTURE_DIRECTION_OUTWARD.0
                                as ::core::ffi::c_int as uint32_t;
                    } else if strcmp(
                        item,
                        b"clockwise\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*output).directions
                            |= gesture_direction::GESTURE_DIRECTION_CLOCKWISE.0
                                as ::core::ffi::c_int as uint32_t;
                    } else if strcmp(
                        item,
                        b"counterclockwise\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*output).directions
                            |= gesture_direction::GESTURE_DIRECTION_COUNTERCLOCKWISE.0
                                as ::core::ffi::c_int as uint32_t;
                    } else {
                        return format_str(
                            b"expected direction, got %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            item,
                        )
                    }
                }
                i += 1;
            }
            list_free_items_and_destroy(directions);
        }
    }
    list_free_items_and_destroy(split);
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn gesture_type_string(
    mut r#type: gesture_type,
) -> *const ::core::ffi::c_char {
    match r#type {
        gesture_type::GESTURE_TYPE_NONE => {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        gesture_type::GESTURE_TYPE_HOLD => {
            return b"hold\0".as_ptr() as *const ::core::ffi::c_char;
        }
        gesture_type::GESTURE_TYPE_PINCH => {
            return b"pinch\0".as_ptr() as *const ::core::ffi::c_char;
        }
        gesture_type::GESTURE_TYPE_SWIPE => {
            return b"swipe\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn gesture_direction_string(
    mut direction: gesture_direction,
) -> *const ::core::ffi::c_char {
    match direction {
        gesture_direction::GESTURE_DIRECTION_NONE => {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        gesture_direction::GESTURE_DIRECTION_UP => {
            return b"up\0".as_ptr() as *const ::core::ffi::c_char;
        }
        gesture_direction::GESTURE_DIRECTION_DOWN => {
            return b"down\0".as_ptr() as *const ::core::ffi::c_char;
        }
        gesture_direction::GESTURE_DIRECTION_LEFT => {
            return b"left\0".as_ptr() as *const ::core::ffi::c_char;
        }
        gesture_direction::GESTURE_DIRECTION_RIGHT => {
            return b"right\0".as_ptr() as *const ::core::ffi::c_char;
        }
        gesture_direction::GESTURE_DIRECTION_INWARD => {
            return b"inward\0".as_ptr() as *const ::core::ffi::c_char;
        }
        gesture_direction::GESTURE_DIRECTION_OUTWARD => {
            return b"outward\0".as_ptr() as *const ::core::ffi::c_char;
        }
        gesture_direction::GESTURE_DIRECTION_CLOCKWISE => {
            return b"clockwise\0".as_ptr() as *const ::core::ffi::c_char;
        }
        gesture_direction::GESTURE_DIRECTION_COUNTERCLOCKWISE => {
            return b"counterclockwise\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn gesture_directions_to_string(
    mut directions: uint32_t,
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut bit: uint8_t = 0 as uint8_t;
    while (bit as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
        let mut masked: uint32_t = directions
            & ((1 as ::core::ffi::c_int) << bit as ::core::ffi::c_int) as uint32_t;
        if masked != 0 {
            let mut name: *const ::core::ffi::c_char = gesture_direction_string(
                gesture_direction(masked as ::core::ffi::c_uint),
            );
            if name.is_null() {
                name = b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
            }
            if result.is_null() {
                result = strdup(name);
            } else {
                let mut new: *mut ::core::ffi::c_char = format_str(
                    b"%s+%s\0".as_ptr() as *const ::core::ffi::c_char,
                    result,
                    name,
                );
                free(result as *mut ::core::ffi::c_void);
                result = new;
            }
        }
        bit = bit.wrapping_add(1);
    }
    if result.is_null() {
        return strdup(b"any\0".as_ptr() as *const ::core::ffi::c_char);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn gesture_to_string(
    mut gesture: *mut gesture,
) -> *mut ::core::ffi::c_char {
    let mut directions: *mut ::core::ffi::c_char = gesture_directions_to_string(
        (*gesture).directions,
    );
    let mut result: *mut ::core::ffi::c_char = format_str(
        b"%s:%u:%s\0".as_ptr() as *const ::core::ffi::c_char,
        gesture_type_string((*gesture).r#type),
        (*gesture).fingers as ::core::ffi::c_int,
        directions,
    );
    free(directions as *mut ::core::ffi::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn gesture_check(
    mut target: *mut gesture,
    mut r#type: gesture_type,
    mut fingers: uint8_t,
) -> bool {
    if (*target).r#type.0 != r#type.0 {
        return r#false != 0;
    }
    if (*target).fingers as ::core::ffi::c_int
        != GESTURE_FINGERS_ANY as ::core::ffi::c_int
        && (*target).fingers as ::core::ffi::c_int != fingers as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn gesture_match(
    mut target: *mut gesture,
    mut to_match: *mut gesture,
    mut exact: bool,
) -> bool {
    if !gesture_check(target, (*to_match).r#type, (*to_match).fingers) {
        return r#false != 0;
    }
    if exact as ::core::ffi::c_int != 0 && (*target).directions != (*to_match).directions
    {
        return r#false != 0;
    }
    return (*target).directions & (*to_match).directions == (*target).directions;
}
#[no_mangle]
pub unsafe extern "C" fn gesture_equal(
    mut a: *mut gesture,
    mut b: *mut gesture,
) -> bool {
    return (*a).r#type.0 == (*b).r#type.0
        && (*a).fingers as ::core::ffi::c_int == (*b).fingers as ::core::ffi::c_int
        && (*a).directions == (*b).directions;
}
unsafe extern "C" fn gesture_directions_count(mut directions: uint32_t) -> uint8_t {
    let mut count: uint8_t = 0 as uint8_t;
    while directions != 0 {
        count = (count as uint32_t).wrapping_add(directions & 1 as uint32_t) as uint8_t;
        directions >>= 1 as ::core::ffi::c_int;
    }
    return count;
}
unsafe extern "C" fn gesture_directions_compare(
    mut a: uint32_t,
    mut b: uint32_t,
) -> int8_t {
    return (gesture_directions_count(a) as ::core::ffi::c_int
        - gesture_directions_count(b) as ::core::ffi::c_int) as int8_t;
}
#[no_mangle]
pub unsafe extern "C" fn gesture_compare(
    mut a: *mut gesture,
    mut b: *mut gesture,
) -> int8_t {
    return gesture_directions_compare((*a).directions, (*b).directions);
}
#[no_mangle]
pub unsafe extern "C" fn gesture_tracker_begin(
    mut tracker: *mut gesture_tracker,
    mut r#type: gesture_type,
    mut fingers: uint8_t,
) {
    (*tracker).r#type = r#type;
    (*tracker).fingers = fingers;
    (*tracker).dx = 0.0f64;
    (*tracker).dy = 0.0f64;
    (*tracker).scale = 1.0f64;
    (*tracker).rotation = 0.0f64;
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] begin tracking %s:%u:? gesture\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"common/gesture.c\0".as_ptr() as *const ::core::ffi::c_char,
        235 as ::core::ffi::c_int,
        gesture_type_string(r#type),
        fingers as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gesture_tracker_check(
    mut tracker: *mut gesture_tracker,
    mut r#type: gesture_type,
) -> bool {
    return (*tracker).r#type.0 == r#type.0;
}
#[no_mangle]
pub unsafe extern "C" fn gesture_tracker_update(
    mut tracker: *mut gesture_tracker,
    mut dx: ::core::ffi::c_double,
    mut dy: ::core::ffi::c_double,
    mut scale: ::core::ffi::c_double,
    mut rotation: ::core::ffi::c_double,
) {
    if (*tracker).r#type.0 == gesture_type::GESTURE_TYPE_HOLD.0 {
        _sway_assert(
            false,
            b"[%s:%d] %s:hold does not update.\0".as_ptr() as *const ::core::ffi::c_char,
            b"common/gesture.c\0".as_ptr() as *const ::core::ffi::c_char,
            246 as ::core::ffi::c_int,
            b"void gesture_tracker_update(struct gesture_tracker *, double, double, double, double)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    (*tracker).dx += dx;
    (*tracker).dy += dy;
    if (*tracker).r#type.0 == gesture_type::GESTURE_TYPE_PINCH.0 {
        (*tracker).scale = scale;
        (*tracker).rotation += rotation;
    }
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] update tracking %s:%u:? gesture: %f %f %f %f\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"common/gesture.c\0".as_ptr() as *const ::core::ffi::c_char,
        258 as ::core::ffi::c_int,
        gesture_type_string((*tracker).r#type),
        (*tracker).fingers as ::core::ffi::c_int,
        (*tracker).dx,
        (*tracker).dy,
        (*tracker).scale,
        (*tracker).rotation,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gesture_tracker_cancel(mut tracker: *mut gesture_tracker) {
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] cancel tracking %s:%u:? gesture\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"common/gesture.c\0".as_ptr() as *const ::core::ffi::c_char,
        266 as ::core::ffi::c_int,
        gesture_type_string((*tracker).r#type),
        (*tracker).fingers as ::core::ffi::c_int,
    );
    (*tracker).r#type = gesture_type::GESTURE_TYPE_NONE;
}
#[no_mangle]
pub unsafe extern "C" fn gesture_tracker_end(
    mut tracker: *mut gesture_tracker,
) -> *mut gesture {
    let mut result: *mut gesture = calloc(1 as size_t, ::core::mem::size_of::<gesture>())
        as *mut gesture;
    let min_rotation: ::core::ffi::c_double = 5 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    let min_scale_delta: ::core::ffi::c_double = 0.1f64;
    's_130: {
        match (*tracker).r#type {
            gesture_type::GESTURE_TYPE_PINCH => {
                if (*tracker).rotation > min_rotation {
                    (*result).directions
                        |= gesture_direction::GESTURE_DIRECTION_CLOCKWISE.0
                            as ::core::ffi::c_int as uint32_t;
                }
                if (*tracker).rotation < -min_rotation {
                    (*result).directions
                        |= gesture_direction::GESTURE_DIRECTION_COUNTERCLOCKWISE.0
                            as ::core::ffi::c_int as uint32_t;
                }
                if (*tracker).scale > 1.0f64 + min_scale_delta {
                    (*result).directions
                        |= gesture_direction::GESTURE_DIRECTION_OUTWARD.0
                            as ::core::ffi::c_int as uint32_t;
                }
                if (*tracker).scale < 1.0f64 - min_scale_delta {
                    (*result).directions
                        |= gesture_direction::GESTURE_DIRECTION_INWARD.0
                            as ::core::ffi::c_int as uint32_t;
                }
            }
            gesture_type::GESTURE_TYPE_SWIPE => {}
            gesture_type::GESTURE_TYPE_NONE => {
                _sway_assert(
                    false,
                    b"[%s:%d] %s:Not tracking any gesture.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"common/gesture.c\0".as_ptr() as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int,
                    b"struct gesture *gesture_tracker_end(struct gesture_tracker *)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                return result;
            }
            gesture_type::GESTURE_TYPE_HOLD | _ => {
                break 's_130;
            }
        }
        if fabs((*tracker).dx) > fabs((*tracker).dy) {
            if (*tracker).dx > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                (*result).directions
                    |= gesture_direction::GESTURE_DIRECTION_RIGHT.0 as ::core::ffi::c_int
                        as uint32_t;
            } else {
                (*result).directions
                    |= gesture_direction::GESTURE_DIRECTION_LEFT.0 as ::core::ffi::c_int
                        as uint32_t;
            }
        } else if (*tracker).dy > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            (*result).directions
                |= gesture_direction::GESTURE_DIRECTION_DOWN.0 as ::core::ffi::c_int
                    as uint32_t;
        } else {
            (*result).directions
                |= gesture_direction::GESTURE_DIRECTION_UP.0 as ::core::ffi::c_int
                    as uint32_t;
        }
    }
    (*result).r#type = (*tracker).r#type;
    (*result).fingers = (*tracker).fingers;
    let mut description: *mut ::core::ffi::c_char = gesture_to_string(result);
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] end tracking gesture: %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"common/gesture.c\0".as_ptr() as *const ::core::ffi::c_char,
        326 as ::core::ffi::c_int,
        description,
    );
    free(description as *mut ::core::ffi::c_void);
    (*tracker).r#type = gesture_type::GESTURE_TYPE_NONE;
    return result;
}
