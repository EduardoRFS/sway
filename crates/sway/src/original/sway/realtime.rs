extern "C" {
    fn sched_get_priority_min(__algorithm: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_setschedparam(
        __target_thread: pthread_t,
        __policy: ::core::ffi::c_int,
        __param: *const sched_param,
    ) -> ::core::ffi::c_int;
    fn pthread_getschedparam(
        __target_thread: pthread_t,
        __policy: *mut ::core::ffi::c_int,
        __param: *mut sched_param,
    ) -> ::core::ffi::c_int;
    fn pthread_atfork(
        __prepare: Option<unsafe extern "C" fn() -> ()>,
        __parent: Option<unsafe extern "C" fn() -> ()>,
        __child: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: ::core::ffi::c_int,
}
pub type pthread_t = ::core::ffi::c_ulong;
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
unsafe extern "C" fn child_fork_callback() {
    let mut param: sched_param = sched_param { sched_priority: 0 };
    param.sched_priority = 0 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = pthread_setschedparam(
        pthread_self(),
        SCHED_OTHER,
        &raw mut param,
    );
    if ret != 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to reset scheduler policy on fork\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/realtime.c\0".as_ptr() as *const ::core::ffi::c_char,
            15 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_rr_scheduling() {
    let mut prio: ::core::ffi::c_int = sched_get_priority_min(SCHED_RR);
    let mut old_policy: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut param: sched_param = sched_param { sched_priority: 0 };
    ret = pthread_getschedparam(pthread_self(), &raw mut old_policy, &raw mut param);
    if ret != 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Failed to get old scheduling priority\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/realtime.c\0".as_ptr() as *const ::core::ffi::c_char,
            27 as ::core::ffi::c_int,
        );
        return;
    }
    param.sched_priority = prio;
    ret = pthread_setschedparam(pthread_self(), SCHED_RR, &raw mut param);
    if ret != 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Failed to set scheduling priority to %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/realtime.c\0".as_ptr() as *const ::core::ffi::c_char,
            35 as ::core::ffi::c_int,
            prio,
        );
        return;
    }
    pthread_atfork(
        None,
        None,
        Some(child_fork_callback as unsafe extern "C" fn() -> ()),
    );
}
pub const SCHED_OTHER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SCHED_RR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
