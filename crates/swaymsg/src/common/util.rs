extern "C" {
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn fcntl(
        __fd: ::core::ffi::c_int,
        __cmd: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn strtof(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_float;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
    fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type size_t = usize;
pub type clockid_t = __clockid_t;
pub type uint32_t = u32;
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
pub struct sway_log_importance_t(pub ::core::ffi::c_uint);
impl sway_log_importance_t {
    pub const SWAY_SILENT: Self = Self(0);
    pub const SWAY_ERROR: Self = Self(1);
    pub const SWAY_INFO: Self = Self(2);
    pub const SWAY_DEBUG: Self = Self(3);
    pub const SWAY_LOG_IMPORTANCE_LAST: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct movement_unit(pub ::core::ffi::c_uint);
impl movement_unit {
    pub const MOVEMENT_UNIT_PX: Self = Self(0);
    pub const MOVEMENT_UNIT_PPT: Self = Self(1);
    pub const MOVEMENT_UNIT_DEFAULT: Self = Self(2);
    pub const MOVEMENT_UNIT_INVALID: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct movement_amount {
    pub amount: ::core::ffi::c_int,
    pub unit: movement_unit,
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn wrap(
    mut i: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (i % max + max) % max;
}
#[no_mangle]
pub unsafe extern "C" fn parse_color(
    mut color: *const ::core::ffi::c_char,
    mut result: *mut uint32_t,
) -> bool {
    if *color.offset(0isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int {
        color = color.offset(1);
    }
    let mut len: ::core::ffi::c_int = strlen(color) as ::core::ffi::c_int;
    if len != 6 as ::core::ffi::c_int && len != 8 as ::core::ffi::c_int
        || *(*__ctype_b_loc())
            .offset(*color.offset(0isize) as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISxdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int == 0
        || *(*__ctype_b_loc())
            .offset(*color.offset(1isize) as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISxdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int == 0
    {
        return r#false != 0;
    }
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut parsed: uint32_t = strtoul(color, &raw mut ptr, 16 as ::core::ffi::c_int)
        as uint32_t;
    if *ptr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        return r#false != 0;
    }
    *result = if len == 6 as ::core::ffi::c_int {
        parsed << 8 as ::core::ffi::c_int | 0xff as uint32_t
    } else {
        parsed
    };
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn color_to_rgba(
    mut dest: *mut ::core::ffi::c_float,
    mut color: uint32_t,
) {
    *dest.offset(0isize) = ((color >> 24 as ::core::ffi::c_int & 0xff as uint32_t)
        as ::core::ffi::c_double / 255.0f64) as ::core::ffi::c_float;
    *dest.offset(1isize) = ((color >> 16 as ::core::ffi::c_int & 0xff as uint32_t)
        as ::core::ffi::c_double / 255.0f64) as ::core::ffi::c_float;
    *dest.offset(2isize) = ((color >> 8 as ::core::ffi::c_int & 0xff as uint32_t)
        as ::core::ffi::c_double / 255.0f64) as ::core::ffi::c_float;
    *dest.offset(3isize) = ((color & 0xff as uint32_t) as ::core::ffi::c_double
        / 255.0f64) as ::core::ffi::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn parse_boolean(
    mut boolean: *const ::core::ffi::c_char,
    mut current: bool,
) -> bool {
    if strcasecmp(boolean, b"1\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcasecmp(boolean, b"yes\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcasecmp(boolean, b"on\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcasecmp(boolean, b"true\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcasecmp(boolean, b"enable\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcasecmp(boolean, b"enabled\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcasecmp(boolean, b"active\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return r#true != 0
    } else if strcasecmp(boolean, b"toggle\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return !current
    }
    return r#false != 0;
}
#[no_mangle]
pub unsafe extern "C" fn parse_float(
    mut value: *const ::core::ffi::c_char,
) -> ::core::ffi::c_float {
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut flt: ::core::ffi::c_float = strtof(value, &raw mut end);
    if *end as ::core::ffi::c_int != 0 || *__errno_location() != 0 {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Invalid float value '%s', defaulting to NAN\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"common/util.c\0".as_ptr() as *const ::core::ffi::c_char,
            61 as ::core::ffi::c_int,
            value,
        );
        return ::core::f32::NAN;
    }
    return flt;
}
#[no_mangle]
pub unsafe extern "C" fn parse_movement_unit(
    mut unit: *const ::core::ffi::c_char,
) -> movement_unit {
    if strcasecmp(unit, b"px\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return movement_unit::MOVEMENT_UNIT_PX;
    }
    if strcasecmp(unit, b"ppt\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return movement_unit::MOVEMENT_UNIT_PPT;
    }
    if strcasecmp(unit, b"default\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return movement_unit::MOVEMENT_UNIT_DEFAULT;
    }
    return movement_unit::MOVEMENT_UNIT_INVALID;
}
#[no_mangle]
pub unsafe extern "C" fn parse_movement_amount(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut amount: *mut movement_amount,
) -> ::core::ffi::c_int {
    if !_sway_assert(
        argc > 0 as ::core::ffi::c_int,
        b"[%s:%d] %s:Expected args in parse_movement_amount\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"common/util.c\0".as_ptr() as *const ::core::ffi::c_char,
        82 as ::core::ffi::c_int,
        b"int parse_movement_amount(int, char **, struct movement_amount *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        (*amount).amount = 0 as ::core::ffi::c_int;
        (*amount).unit = movement_unit::MOVEMENT_UNIT_INVALID;
        return 0 as ::core::ffi::c_int;
    }
    let mut err: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    (*amount).amount = strtol(
        *argv.offset(0isize),
        &raw mut err,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if *err != 0 {
        (*amount).unit = parse_movement_unit(err);
        return 1 as ::core::ffi::c_int;
    }
    if argc == 1 as ::core::ffi::c_int {
        (*amount).unit = movement_unit::MOVEMENT_UNIT_DEFAULT;
        return 1 as ::core::ffi::c_int;
    }
    (*amount).unit = parse_movement_unit(*argv.offset(1isize));
    if (*amount).unit.0 == movement_unit::MOVEMENT_UNIT_INVALID.0 {
        (*amount).unit = movement_unit::MOVEMENT_UNIT_DEFAULT;
        return 1 as ::core::ffi::c_int;
    }
    return 2 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sway_wl_output_subpixel_to_string(
    mut subpixel: wl_output_subpixel,
) -> *const ::core::ffi::c_char {
    match subpixel {
        wl_output_subpixel::WL_OUTPUT_SUBPIXEL_UNKNOWN => {
            return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wl_output_subpixel::WL_OUTPUT_SUBPIXEL_NONE => {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wl_output_subpixel::WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB => {
            return b"rgb\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wl_output_subpixel::WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR => {
            return b"bgr\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wl_output_subpixel::WL_OUTPUT_SUBPIXEL_VERTICAL_RGB => {
            return b"vrgb\0".as_ptr() as *const ::core::ffi::c_char;
        }
        wl_output_subpixel::WL_OUTPUT_SUBPIXEL_VERTICAL_BGR => {
            return b"vbgr\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    _sway_assert(
        false,
        b"[%s:%d] %s:Unknown value for wl_output_subpixel.\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"common/util.c\0".as_ptr() as *const ::core::ffi::c_char,
        123 as ::core::ffi::c_int,
        b"const char *sway_wl_output_subpixel_to_string(enum wl_output_subpixel)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn sway_set_cloexec(
    mut fd: ::core::ffi::c_int,
    mut cloexec: bool,
) -> bool {
    let mut flags: ::core::ffi::c_int = fcntl(fd, F_GETFD);
    if flags == -1 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] fcntl failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"common/util.c\0".as_ptr() as *const ::core::ffi::c_char,
            130 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        return r#false != 0;
    }
    if cloexec {
        flags = flags | FD_CLOEXEC;
    } else {
        flags = flags & !FD_CLOEXEC;
    }
    if fcntl(fd, F_SETFD, flags) == -1 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] fcntl failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"common/util.c\0".as_ptr() as *const ::core::ffi::c_char,
            139 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        return r#false != 0;
    }
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn get_current_time_in_msec() -> uint32_t {
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(CLOCK_MONOTONIC, &raw mut now);
    return (now.tv_sec * 1000 as __syscall_slong_t
        + now.tv_nsec / 1000000 as __syscall_slong_t) as uint32_t;
}
pub const F_GETFD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
