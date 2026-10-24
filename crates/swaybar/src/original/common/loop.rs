extern "C" {
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn create_list() -> *mut list_t;
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn list_del(list: *mut list_t, index: ::core::ffi::c_int);
    fn list_free_items_and_destroy(list: *mut list_t);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct r#loop {
    pub fds: *mut pollfd,
    pub fd_length: ::core::ffi::c_int,
    pub fd_capacity: ::core::ffi::c_int,
    pub fd_events: *mut list_t,
    pub timers: *mut list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_timer {
    pub callback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub data: *mut ::core::ffi::c_void,
    pub expiry: timespec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_fd_event {
    pub callback: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_short,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub data: *mut ::core::ffi::c_void,
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn loop_create() -> *mut r#loop {
    let mut r#loop: *mut r#loop = calloc(1 as size_t, ::core::mem::size_of::<r#loop>())
        as *mut r#loop;
    if r#loop.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to allocate memory for loop\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"common/loop.c\0".as_ptr() as *const ::core::ffi::c_char,
            36 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<r#loop>();
    }
    (*r#loop).fd_capacity = 10 as ::core::ffi::c_int;
    (*r#loop).fds = malloc(
        ::core::mem::size_of::<pollfd>().wrapping_mul((*r#loop).fd_capacity as size_t),
    ) as *mut pollfd;
    (*r#loop).fd_events = create_list();
    (*r#loop).timers = create_list();
    return r#loop;
}
#[no_mangle]
pub unsafe extern "C" fn loop_destroy(mut r#loop: *mut r#loop) {
    list_free_items_and_destroy((*r#loop).fd_events);
    list_free_items_and_destroy((*r#loop).timers);
    free((*r#loop).fds as *mut ::core::ffi::c_void);
    free(r#loop as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn loop_poll(mut r#loop: *mut r#loop) {
    let mut ms: ::core::ffi::c_int = INT_MAX;
    if (*(*r#loop).timers).length != 0 {
        let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        clock_gettime(CLOCK_MONOTONIC, &raw mut now);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*r#loop).timers).length {
            let mut timer: *mut loop_timer = *(*(*r#loop).timers)
                .items
                .offset(i as isize) as *mut loop_timer;
            let mut timer_ms: ::core::ffi::c_int = (((*timer).expiry.tv_sec - now.tv_sec)
                * 1000 as __time_t) as ::core::ffi::c_int;
            timer_ms = (timer_ms as __syscall_slong_t
                + ((*timer).expiry.tv_nsec - now.tv_nsec) / 1000000 as __syscall_slong_t)
                as ::core::ffi::c_int;
            if timer_ms < ms {
                ms = timer_ms;
            }
            i += 1;
        }
    }
    if ms < 0 as ::core::ffi::c_int {
        ms = 0 as ::core::ffi::c_int;
    }
    poll((*r#loop).fds, (*r#loop).fd_length as nfds_t, ms);
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*r#loop).fd_length {
        let mut pfd: pollfd = *(*r#loop).fds.offset(i_0 as isize);
        let mut event: *mut loop_fd_event = *(*(*r#loop).fd_events)
            .items
            .offset(i_0 as isize) as *mut loop_fd_event;
        let mut events: ::core::ffi::c_uint = (pfd.events as ::core::ffi::c_int | POLLHUP
            | POLLERR) as ::core::ffi::c_uint;
        if pfd.revents as ::core::ffi::c_uint & events != 0 {
            (*event)
                .callback
                .expect("non-null function pointer")(pfd.fd, pfd.revents, (*event).data);
        }
        i_0 += 1;
    }
    if (*(*r#loop).timers).length != 0 {
        let mut now_0: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        clock_gettime(CLOCK_MONOTONIC, &raw mut now_0);
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < (*(*r#loop).timers).length {
            let mut timer_0: *mut loop_timer = *(*(*r#loop).timers)
                .items
                .offset(i_1 as isize) as *mut loop_timer;
            let mut expired: bool = (*timer_0).expiry.tv_sec < now_0.tv_sec
                || (*timer_0).expiry.tv_sec == now_0.tv_sec
                    && (*timer_0).expiry.tv_nsec < now_0.tv_nsec;
            if expired {
                (*timer_0).callback.expect("non-null function pointer")((*timer_0).data);
                loop_remove_timer(r#loop, timer_0);
                i_1 -= 1;
            }
            i_1 += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn loop_add_fd(
    mut r#loop: *mut r#loop,
    mut fd: ::core::ffi::c_int,
    mut mask: ::core::ffi::c_short,
    mut callback: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_short,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut event: *mut loop_fd_event = calloc(
        1 as size_t,
        ::core::mem::size_of::<loop_fd_event>(),
    ) as *mut loop_fd_event;
    if event.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to allocate memory for event\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"common/loop.c\0".as_ptr() as *const ::core::ffi::c_char,
            109 as ::core::ffi::c_int,
        );
        return;
    }
    (*event).callback = callback;
    (*event).data = data;
    list_add((*r#loop).fd_events, event as *mut ::core::ffi::c_void);
    let mut pfd: pollfd = pollfd {
        fd: fd,
        events: mask,
        revents: 0 as ::core::ffi::c_short,
    };
    if (*r#loop).fd_length == (*r#loop).fd_capacity {
        let mut capacity: ::core::ffi::c_int = (*r#loop).fd_capacity
            + 10 as ::core::ffi::c_int;
        let mut tmp: *mut pollfd = realloc(
            (*r#loop).fds as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<pollfd>().wrapping_mul(capacity as size_t),
        ) as *mut pollfd;
        if tmp.is_null() {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Unable to allocate memory for pollfd\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"common/loop.c\0".as_ptr() as *const ::core::ffi::c_char,
                123 as ::core::ffi::c_int,
            );
            return;
        }
        (*r#loop).fds = tmp;
        (*r#loop).fd_capacity = capacity;
    }
    let c2rust_fresh0 = (*r#loop).fd_length;
    (*r#loop).fd_length += 1;
    *(*r#loop).fds.offset(c2rust_fresh0 as isize) = pfd;
}
#[no_mangle]
pub unsafe extern "C" fn loop_add_timer(
    mut r#loop: *mut r#loop,
    mut ms: ::core::ffi::c_int,
    mut callback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    mut data: *mut ::core::ffi::c_void,
) -> *mut loop_timer {
    let mut timer: *mut loop_timer = calloc(
        1 as size_t,
        ::core::mem::size_of::<loop_timer>(),
    ) as *mut loop_timer;
    if timer.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Unable to allocate memory for timer\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"common/loop.c\0".as_ptr() as *const ::core::ffi::c_char,
            137 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<loop_timer>();
    }
    (*timer).callback = callback;
    (*timer).data = data;
    clock_gettime(CLOCK_MONOTONIC, &raw mut (*timer).expiry);
    (*timer).expiry.tv_sec += (ms / 1000 as ::core::ffi::c_int) as __time_t;
    let mut nsec: ::core::ffi::c_long = (ms % 1000 as ::core::ffi::c_int
        * 1000000 as ::core::ffi::c_int) as ::core::ffi::c_long;
    if (*timer).expiry.tv_nsec + nsec >= 1000000000 as ::core::ffi::c_long {
        (*timer).expiry.tv_sec += 1;
        nsec -= 1000000000 as ::core::ffi::c_long;
    }
    (*timer).expiry.tv_nsec += nsec;
    list_add((*r#loop).timers, timer as *mut ::core::ffi::c_void);
    return timer;
}
#[no_mangle]
pub unsafe extern "C" fn loop_remove_fd(
    mut r#loop: *mut r#loop,
    mut fd: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*r#loop).fd_length {
        if (*(*r#loop).fds.offset(i as isize)).fd == fd {
            free(*(*(*r#loop).fd_events).items.offset(i as isize));
            list_del((*r#loop).fd_events, i);
            (*r#loop).fd_length -= 1;
            memmove(
                (*r#loop).fds.offset(i as isize) as *mut ::core::ffi::c_void,
                (*r#loop).fds.offset((i + 1 as ::core::ffi::c_int) as isize)
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<pollfd>()
                    .wrapping_mul(((*r#loop).fd_length - i) as size_t),
            );
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
#[no_mangle]
pub unsafe extern "C" fn loop_remove_timer(
    mut r#loop: *mut r#loop,
    mut timer: *mut loop_timer,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*r#loop).timers).length {
        if *(*(*r#loop).timers).items.offset(i as isize)
            == timer as *mut ::core::ffi::c_void
        {
            list_del((*r#loop).timers, i);
            free(timer as *mut ::core::ffi::c_void);
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
