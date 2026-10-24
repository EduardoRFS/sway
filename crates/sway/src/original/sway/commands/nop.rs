extern "C" {
    fn cmd_results_new(
        status: cmd_status,
        error: *const ::core::ffi::c_char,
        ...
    ) -> *mut cmd_results;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_results {
    pub status: cmd_status,
    pub error: *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct cmd_status(pub ::core::ffi::c_uint);
impl cmd_status {
    pub const CMD_SUCCESS: Self = Self(0);
    pub const CMD_FAILURE: Self = Self(1);
    pub const CMD_INVALID: Self = Self(2);
    pub const CMD_DEFER: Self = Self(3);
    pub const CMD_BLOCK: Self = Self(4);
    pub const CMD_BLOCK_COMMANDS: Self = Self(5);
    pub const CMD_BLOCK_END: Self = Self(6);
}
pub type sway_cmd = unsafe extern "C" fn(
    ::core::ffi::c_int,
    *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results;
#[no_mangle]
pub unsafe extern "C" fn cmd_nop(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut cmd_results {
    return cmd_results_new(
        cmd_status::CMD_SUCCESS,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
