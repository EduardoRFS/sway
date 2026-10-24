extern "C" {
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strspn(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strpbrk(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn wordexp(
        __words: *const ::core::ffi::c_char,
        __pwordexp: *mut wordexp_t,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn wordfree(__wordexp: *mut wordexp_t);
    fn create_list() -> *mut list_t;
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
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
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __builtin_va_list;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordexp_t {
    pub we_wordc: size_t,
    pub we_wordv: *mut *mut ::core::ffi::c_char,
    pub we_offs: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
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
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
static mut whitespace: [::core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b" \x0C\n\r\t\x0B\0")
};
#[no_mangle]
pub unsafe extern "C" fn strip_whitespace(mut str: *mut ::core::ffi::c_char) {
    let mut len: size_t = strlen(str);
    let mut start: size_t = strspn(
        str,
        &raw const whitespace as *const ::core::ffi::c_char,
    ) as size_t;
    memmove(
        str as *mut ::core::ffi::c_void,
        str.offset(start as isize) as *const ::core::ffi::c_void,
        len.wrapping_add(1 as size_t).wrapping_sub(start),
    );
    if *str != 0 {
        len = len.wrapping_sub(start.wrapping_add(1 as size_t));
        while *(*__ctype_b_loc())
            .offset(*str.offset(len as isize) as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
        {
            len = len.wrapping_sub(1);
        }
        *str.offset(len.wrapping_add(1 as size_t) as isize) = '\0'
            as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn strip_quotes(mut str: *mut ::core::ffi::c_char) {
    let mut in_str: bool = r#false != 0;
    let mut in_chr: bool = r#false != 0;
    let mut escaped: bool = r#false != 0;
    let mut end: *mut ::core::ffi::c_char = strchr(str, 0 as ::core::ffi::c_int);
    while *str != 0 {
        if *str as ::core::ffi::c_int == '\'' as ::core::ffi::c_int && !in_str
            && !escaped
        {
            in_chr = !in_chr;
        } else if *str as ::core::ffi::c_int == '"' as ::core::ffi::c_int && !in_chr
            && !escaped
        {
            in_str = !in_str;
        } else if *str as ::core::ffi::c_int == '\\' as ::core::ffi::c_int {
            escaped = !escaped;
            str = str.offset(1);
            continue;
        } else {
            escaped = r#false != 0;
            str = str.offset(1);
            continue;
        }
        let c2rust_fresh0 = end;
        end = end.offset(-1);
        memmove(
            str as *mut ::core::ffi::c_void,
            str.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            c2rust_fresh0.offset_from(str) as size_t,
        );
    }
    *end = '\0' as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn lenient_strcat(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if !dest.is_null() && !src.is_null() {
        return strcat(dest, src);
    }
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn lenient_strncat(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut len: size_t,
) -> *mut ::core::ffi::c_char {
    if !dest.is_null() && !src.is_null() {
        return strncat(dest, src, len);
    }
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn lenient_strcmp(
    mut a: *const ::core::ffi::c_char,
    mut b: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if a == b {
        return 0 as ::core::ffi::c_int
    } else if a.is_null() {
        return -1 as ::core::ffi::c_int
    } else if b.is_null() {
        return 1 as ::core::ffi::c_int
    } else {
        return strcmp(a, b)
    };
}
#[no_mangle]
pub unsafe extern "C" fn split_string(
    mut str: *const ::core::ffi::c_char,
    mut delims: *const ::core::ffi::c_char,
) -> *mut list_t {
    let mut res: *mut list_t = create_list();
    let mut copy: *mut ::core::ffi::c_char = strdup(str);
    let mut token: *mut ::core::ffi::c_char = strtok(copy, delims);
    while !token.is_null() {
        list_add(res, strdup(token) as *mut ::core::ffi::c_void);
        token = strtok(::core::ptr::null_mut::<::core::ffi::c_char>(), delims);
    }
    free(copy as *mut ::core::ffi::c_void);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn split_args(
    mut start: *const ::core::ffi::c_char,
    mut argc: *mut ::core::ffi::c_int,
) -> *mut *mut ::core::ffi::c_char {
    *argc = 0 as ::core::ffi::c_int;
    let mut alloc: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    let mut argv: *mut *mut ::core::ffi::c_char = malloc(
        ::core::mem::size_of::<*mut ::core::ffi::c_char>().wrapping_mul(alloc as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    let mut in_token: bool = r#false != 0;
    let mut in_string: bool = r#false != 0;
    let mut in_char: bool = r#false != 0;
    let mut in_brackets: bool = r#false != 0;
    let mut escaped: bool = r#false != 0;
    let mut end: *const ::core::ffi::c_char = start;
    if !start.is_null() {
        while *start != 0 {
            if !in_token {
                end = end
                    .offset(
                        strspn(end, &raw const whitespace as *const ::core::ffi::c_char)
                            as isize,
                    );
                start = end;
                in_token = r#true != 0;
            }
            if *end as ::core::ffi::c_int == '"' as ::core::ffi::c_int && !in_char
                && !escaped
            {
                in_string = !in_string;
            } else if *end as ::core::ffi::c_int == '\'' as ::core::ffi::c_int
                && !in_string && !escaped
            {
                in_char = !in_char;
            } else if *end as ::core::ffi::c_int == '[' as ::core::ffi::c_int
                && !in_string && !in_char && !in_brackets && !escaped
            {
                in_brackets = r#true != 0;
            } else if *end as ::core::ffi::c_int == ']' as ::core::ffi::c_int
                && !in_string && !in_char && in_brackets as ::core::ffi::c_int != 0
                && !escaped
            {
                in_brackets = r#false != 0;
            } else if *end as ::core::ffi::c_int == '\\' as ::core::ffi::c_int {
                escaped = !escaped;
            } else if *end as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || !in_string && !in_char && !in_brackets && !escaped
                    && !strchr(
                            &raw const whitespace as *const ::core::ffi::c_char,
                            *end as ::core::ffi::c_int,
                        )
                        .is_null()
            {
                if end.offset_from(start) > 0isize {
                    let mut token: *mut ::core::ffi::c_char = malloc(
                        (end.offset_from(start) + 1isize) as size_t,
                    ) as *mut ::core::ffi::c_char;
                    strncpy(token, start, (end.offset_from(start) + 1isize) as size_t);
                    *token.offset(end.offset_from(start) as isize) = '\0'
                        as ::core::ffi::c_char;
                    *argv.offset(*argc as isize) = token;
                    *argc += 1;
                    if *argc + 1 as ::core::ffi::c_int == alloc {
                        alloc *= 2 as ::core::ffi::c_int;
                        argv = realloc(
                            argv as *mut ::core::ffi::c_void,
                            (alloc as size_t)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
                                ),
                        ) as *mut *mut ::core::ffi::c_char;
                    }
                }
                in_token = r#false != 0;
                escaped = r#false != 0;
                continue;
            }
            if *end as ::core::ffi::c_int != '\\' as ::core::ffi::c_int {
                escaped = r#false != 0;
            }
            end = end.offset(1);
        }
    }
    *argv.offset(*argc as isize) = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return argv;
}
#[no_mangle]
pub unsafe extern "C" fn free_argv(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    loop {
        let c2rust_fresh1 = argc;
        argc -= 1;
        if c2rust_fresh1 <= 0 as ::core::ffi::c_int {
            break;
        }
        free(*argv.offset(argc as isize) as *mut ::core::ffi::c_void);
    }
    free(argv as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn unescape_string(
    mut string: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = strlen(string) as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while *string.offset(i as isize) != 0 {
        if *string.offset(i as isize) as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
        {
            i += 1;
            match *string.offset(i as isize) as ::core::ffi::c_int {
                48 => {
                    *string.offset((i - 1 as ::core::ffi::c_int) as isize) = '\0'
                        as ::core::ffi::c_char;
                    return i - 1 as ::core::ffi::c_int;
                }
                97 => {
                    *string.offset((i - 1 as ::core::ffi::c_int) as isize) = '\u{7}'
                        as ::core::ffi::c_char;
                    *string.offset(i as isize) = '\0' as ::core::ffi::c_char;
                }
                98 => {
                    *string.offset((i - 1 as ::core::ffi::c_int) as isize) = '\u{8}'
                        as ::core::ffi::c_char;
                    *string.offset(i as isize) = '\0' as ::core::ffi::c_char;
                }
                102 => {
                    *string.offset((i - 1 as ::core::ffi::c_int) as isize) = '\u{c}'
                        as ::core::ffi::c_char;
                    *string.offset(i as isize) = '\0' as ::core::ffi::c_char;
                }
                110 => {
                    *string.offset((i - 1 as ::core::ffi::c_int) as isize) = '\n'
                        as ::core::ffi::c_char;
                    *string.offset(i as isize) = '\0' as ::core::ffi::c_char;
                }
                114 => {
                    *string.offset((i - 1 as ::core::ffi::c_int) as isize) = '\r'
                        as ::core::ffi::c_char;
                    *string.offset(i as isize) = '\0' as ::core::ffi::c_char;
                }
                116 => {
                    *string.offset((i - 1 as ::core::ffi::c_int) as isize) = '\t'
                        as ::core::ffi::c_char;
                    *string.offset(i as isize) = '\0' as ::core::ffi::c_char;
                }
                118 => {
                    *string.offset((i - 1 as ::core::ffi::c_int) as isize) = '\u{b}'
                        as ::core::ffi::c_char;
                    *string.offset(i as isize) = '\0' as ::core::ffi::c_char;
                }
                92 => {
                    *string.offset(i as isize) = '\0' as ::core::ffi::c_char;
                }
                39 => {
                    *string.offset((i - 1 as ::core::ffi::c_int) as isize) = '\''
                        as ::core::ffi::c_char;
                    *string.offset(i as isize) = '\0' as ::core::ffi::c_char;
                }
                34 => {
                    *string.offset((i - 1 as ::core::ffi::c_int) as isize) = '"'
                        as ::core::ffi::c_char;
                    *string.offset(i as isize) = '\0' as ::core::ffi::c_char;
                }
                63 => {
                    *string.offset((i - 1 as ::core::ffi::c_int) as isize) = '?'
                        as ::core::ffi::c_char;
                    *string.offset(i as isize) = '\0' as ::core::ffi::c_char;
                }
                120 => {
                    let mut c: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                    if *string.offset((i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                        && *string.offset((i + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    {
                        c = (*string.offset((i + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        if *string.offset((i + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && *string.offset((i + 2 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        {
                            c = (c as ::core::ffi::c_int * 0x10 as ::core::ffi::c_int)
                                as ::core::ffi::c_uchar;
                            c = (c as ::core::ffi::c_int
                                + (*string.offset((i + 2 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int - '0' as ::core::ffi::c_int))
                                as ::core::ffi::c_uchar;
                            *string.offset((i + 2 as ::core::ffi::c_int) as isize) = '\0'
                                as ::core::ffi::c_char;
                        }
                        *string.offset((i + 1 as ::core::ffi::c_int) as isize) = '\0'
                            as ::core::ffi::c_char;
                    }
                    *string.offset(i as isize) = '\0' as ::core::ffi::c_char;
                    *string.offset((i - 1 as ::core::ffi::c_int) as isize) = c
                        as ::core::ffi::c_char;
                }
                _ => {}
            }
        }
        i += 1;
    }
    let mut shift: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < len {
        if *string.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            shift += 1;
        } else {
            *string.offset((i - shift) as isize) = *string.offset(i as isize);
        }
        i += 1;
    }
    *string.offset((len - shift) as isize) = 0 as ::core::ffi::c_char;
    return len - shift;
}
#[no_mangle]
pub unsafe extern "C" fn join_args(
    mut argv: *mut *mut ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    if !_sway_assert(
        argc > 0 as ::core::ffi::c_int,
        b"[%s:%d] %s:argc should be positive\0".as_ptr() as *const ::core::ffi::c_char,
        b"common/stringop.c\0".as_ptr() as *const ::core::ffi::c_char,
        237 as ::core::ffi::c_int,
        b"char *join_args(char **, int)\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < argc {
        len = (len as size_t)
            .wrapping_add(strlen(*argv.offset(i as isize)).wrapping_add(1 as size_t))
            as ::core::ffi::c_int;
        i += 1;
    }
    let mut res: *mut ::core::ffi::c_char = malloc(len as size_t)
        as *mut ::core::ffi::c_char;
    len = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < argc {
        strcpy(res.offset(len as isize), *argv.offset(i as isize));
        len = (len as size_t).wrapping_add(strlen(*argv.offset(i as isize)))
            as ::core::ffi::c_int;
        let c2rust_fresh2 = len;
        len += 1;
        *res.offset(c2rust_fresh2 as isize) = ' ' as ::core::ffi::c_char;
        i += 1;
    }
    *res.offset((len - 1 as ::core::ffi::c_int) as isize) = '\0' as ::core::ffi::c_char;
    return res;
}
#[inline]
unsafe extern "C" fn argsep_next_interesting(
    mut src: *const ::core::ffi::c_char,
    mut delim: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut special: *mut ::core::ffi::c_char = strpbrk(
        src,
        b"\"'\\\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut next_delim: *mut ::core::ffi::c_char = strpbrk(src, delim);
    if special.is_null() {
        return next_delim;
    }
    if next_delim.is_null() {
        return special;
    }
    return if next_delim < special { next_delim } else { special };
}
#[no_mangle]
pub unsafe extern "C" fn argsep(
    mut stringp: *mut *mut ::core::ffi::c_char,
    mut delim: *const ::core::ffi::c_char,
    mut matched: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut start: *mut ::core::ffi::c_char = *stringp;
    let mut end: *mut ::core::ffi::c_char = start;
    let mut in_string: bool = r#false != 0;
    let mut in_char: bool = r#false != 0;
    let mut escaped: bool = r#false != 0;
    let mut interesting: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    loop {
        interesting = argsep_next_interesting(end, delim);
        if interesting.is_null() {
            break;
        }
        if escaped as ::core::ffi::c_int != 0 && interesting != end {
            escaped = r#false != 0;
        }
        if *interesting as ::core::ffi::c_int == '"' as ::core::ffi::c_int && !in_char
            && !escaped
        {
            in_string = !in_string;
            end = interesting.offset(1 as ::core::ffi::c_int as isize);
        } else if *interesting as ::core::ffi::c_int == '\'' as ::core::ffi::c_int
            && !in_string && !escaped
        {
            in_char = !in_char;
            end = interesting.offset(1 as ::core::ffi::c_int as isize);
        } else if *interesting as ::core::ffi::c_int == '\\' as ::core::ffi::c_int {
            escaped = !escaped;
            end = interesting.offset(1 as ::core::ffi::c_int as isize);
        } else if !in_string && !in_char && !escaped {
            end = interesting;
            if !matched.is_null() {
                *matched = *end;
            }
            if end.offset_from(start) != 0 {
                let c2rust_fresh3 = end;
                end = end.offset(1);
                *c2rust_fresh3 = 0 as ::core::ffi::c_char;
                *stringp = end;
                break;
            } else {
                start = start.offset(1);
                end = start;
            }
        } else {
            end = end.offset(1);
        }
    }
    if interesting.is_null() {
        *stringp = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !matched.is_null() {
            *matched = '\0' as ::core::ffi::c_char;
        }
    }
    return start;
}
#[no_mangle]
pub unsafe extern "C" fn expand_path(mut path: *mut *mut ::core::ffi::c_char) -> bool {
    let mut p: wordexp_t = wordexp_t {
        we_wordc: 0 as size_t,
        we_wordv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        we_offs: 0,
    };
    while !strstr(*path, b"  \0".as_ptr() as *const ::core::ffi::c_char).is_null() {
        *path = realloc(
            *path as *mut ::core::ffi::c_void,
            strlen(*path).wrapping_add(2 as size_t),
        ) as *mut ::core::ffi::c_char;
        let mut ptr: *mut ::core::ffi::c_char = strstr(
                *path,
                b"  \0".as_ptr() as *const ::core::ffi::c_char,
            )
            .offset(1 as ::core::ffi::c_int as isize);
        memmove(
            ptr.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            ptr as *const ::core::ffi::c_void,
            strlen(ptr).wrapping_add(1 as size_t),
        );
        *ptr = '\\' as ::core::ffi::c_char;
    }
    if wordexp(*path, &raw mut p, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int
        || (*p.we_wordv.offset(0isize)).is_null()
    {
        wordfree(&raw mut p);
        return r#false != 0;
    }
    free(*path as *mut ::core::ffi::c_void);
    *path = join_args(p.we_wordv, p.we_wordc as ::core::ffi::c_int);
    wordfree(&raw mut p);
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn vformat_str(
    mut fmt: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
) -> *mut ::core::ffi::c_char {
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut args_copy: ::core::ffi::VaList<'_>;
    args_copy = args.clone();
    let mut len: ::core::ffi::c_int = vsnprintf(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
        fmt,
        args.clone(),
    );
    if len < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] vsnprintf(\"%s\") failed: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"common/stringop.c\0".as_ptr() as *const ::core::ffi::c_char,
            339 as ::core::ffi::c_int,
            fmt,
            strerror(*__errno_location()),
        );
    } else {
        str = malloc((len + 1 as ::core::ffi::c_int) as size_t)
            as *mut ::core::ffi::c_char;
        if str.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] malloc() failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"common/stringop.c\0".as_ptr() as *const ::core::ffi::c_char,
                345 as ::core::ffi::c_int,
                strerror(*__errno_location()),
            );
        } else {
            vsnprintf(
                str,
                (len + 1 as ::core::ffi::c_int) as size_t,
                fmt,
                args_copy.clone(),
            );
        }
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn format_str(
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut ::core::ffi::c_char {
    let mut args: ::core::ffi::VaList<'_>;
    args = c2rust_args.clone();
    let mut str: *mut ::core::ffi::c_char = vformat_str(fmt, args.clone());
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn has_prefix(
    mut str: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> bool {
    return strncmp(str, prefix, strlen(prefix)) == 0 as ::core::ffi::c_int;
}
