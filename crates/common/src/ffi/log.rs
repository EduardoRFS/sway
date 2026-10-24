use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn raise(__sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type clockid_t = __clockid_t;
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
pub type terminate_callback_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub const SIGABRT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut log_terminate: terminate_callback_t = unsafe {
    ::core::mem::transmute::<
        Option<unsafe extern "C" fn(::core::ffi::c_int) -> !>,
        terminate_callback_t,
    >(Some(exit as unsafe extern "C" fn(::core::ffi::c_int) -> !))
};
#[no_mangle]
pub unsafe extern "C" fn _sway_abort(
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut args: ::core::ffi::VaList<'_>;
    args = c2rust_args.clone();
    _sway_vlog(sway_log_importance_t::SWAY_ERROR, format, args.clone());
    log_terminate.expect("non-null function pointer")(EXIT_FAILURE);
}
#[no_mangle]
pub unsafe extern "C" fn _sway_assert(
    mut condition: bool,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> bool {
    if condition {
        return r#true != 0;
    }
    let mut args: ::core::ffi::VaList<'_>;
    args = c2rust_args.clone();
    _sway_vlog(sway_log_importance_t::SWAY_ERROR, format, args.clone());
    raise(SIGABRT);
    return r#false != 0;
}
static mut colored: bool = r#true != 0;
static mut log_importance: sway_log_importance_t = sway_log_importance_t::SWAY_ERROR;
static mut start_time: timespec = timespec {
    tv_sec: -1 as __time_t,
    tv_nsec: -1 as __syscall_slong_t,
};
static mut verbosity_colors: [*const ::core::ffi::c_char; 4] = [
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[1;31m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[1;34m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[1;90m\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut verbosity_headers: [*const ::core::ffi::c_char; 4] = [
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"[ERROR]\0".as_ptr() as *const ::core::ffi::c_char,
    b"[INFO]\0".as_ptr() as *const ::core::ffi::c_char,
    b"[DEBUG]\0".as_ptr() as *const ::core::ffi::c_char,
];
unsafe extern "C" fn timespec_sub(
    mut r: *mut timespec,
    mut a: *const timespec,
    mut b: *const timespec,
) {
    let NSEC_PER_SEC: ::core::ffi::c_long = 1000000000 as ::core::ffi::c_long;
    (*r).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*r).tv_nsec = (*a).tv_nsec - (*b).tv_nsec;
    if (*r).tv_nsec < 0 as __syscall_slong_t {
        (*r).tv_sec -= 1;
        (*r).tv_nsec += NSEC_PER_SEC;
    }
}
unsafe extern "C" fn init_start_time() {
    if start_time.tv_sec >= 0 as __time_t {
        return;
    }
    clock_gettime(CLOCK_MONOTONIC, &raw mut start_time);
}
unsafe extern "C" fn sway_log_stderr(
    mut verbosity: sway_log_importance_t,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
) {
    init_start_time();
    if verbosity.0 > log_importance.0 {
        return;
    }
    let mut ts: timespec = timespec {
        tv_sec: 0 as __time_t,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut ts);
    timespec_sub(&raw mut ts, &raw mut ts, &raw mut start_time);
    fprintf(
        stderr,
        b"%02d:%02d:%02d.%03ld \0".as_ptr() as *const ::core::ffi::c_char,
        (ts.tv_sec / 60 as __time_t / 60 as __time_t) as ::core::ffi::c_int,
        (ts.tv_sec / 60 as __time_t % 60 as __time_t) as ::core::ffi::c_int,
        (ts.tv_sec % 60 as __time_t) as ::core::ffi::c_int,
        ts.tv_nsec / 1000000 as __syscall_slong_t,
    );
    let mut c: ::core::ffi::c_uint = if verbosity.0
        < sway_log_importance_t::SWAY_LOG_IMPORTANCE_LAST.0
    {
        verbosity.0
    } else {
        (sway_log_importance_t::SWAY_LOG_IMPORTANCE_LAST.0 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
    };
    if colored as ::core::ffi::c_int != 0 && isatty(STDERR_FILENO) != 0 {
        fprintf(
            stderr,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            verbosity_colors[c as usize],
        );
    } else {
        fprintf(
            stderr,
            b"%s \0".as_ptr() as *const ::core::ffi::c_char,
            verbosity_headers[c as usize],
        );
    }
    vfprintf(stderr, fmt, args.clone());
    if colored as ::core::ffi::c_int != 0 && isatty(STDERR_FILENO) != 0 {
        fprintf(stderr, b"\x1B[0m\0".as_ptr() as *const ::core::ffi::c_char);
    }
    fprintf(stderr, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn sway_log_init(
    mut verbosity: sway_log_importance_t,
    mut callback: terminate_callback_t,
) {
    init_start_time();
    if verbosity.0 < sway_log_importance_t::SWAY_LOG_IMPORTANCE_LAST.0 {
        log_importance = verbosity;
    }
    if callback.is_some() {
        log_terminate = callback;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _sway_vlog(
    mut verbosity: sway_log_importance_t,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
) {
    sway_log_stderr(verbosity, fmt, args.clone());
}
#[no_mangle]
pub unsafe extern "C" fn _sway_log(
    mut verbosity: sway_log_importance_t,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut args: ::core::ffi::VaList<'_>;
    args = c2rust_args.clone();
    sway_log_stderr(verbosity, fmt, args.clone());
}
