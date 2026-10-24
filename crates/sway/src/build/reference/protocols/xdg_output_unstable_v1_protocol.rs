extern "C" {
    static wl_output_interface: wl_interface;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_message {
    pub name: *const ::core::ffi::c_char,
    pub signature: *const ::core::ffi::c_char,
    pub types: *mut *const wl_interface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_interface {
    pub name: *const ::core::ffi::c_char,
    pub version: ::core::ffi::c_int,
    pub method_count: ::core::ffi::c_int,
    pub methods: *const wl_message,
    pub event_count: ::core::ffi::c_int,
    pub events: *const wl_message,
}
static mut xdg_output_unstable_v1_types: [*const wl_interface; 4] = unsafe {
    [
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        &raw const zxdg_output_v1_interface,
        &raw const wl_output_interface,
    ]
};
static mut zxdg_output_manager_v1_requests: [wl_message; 2] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 2];
#[no_mangle]
pub static mut zxdg_output_manager_v1_interface: wl_interface = unsafe {
    wl_interface {
        name: b"zxdg_output_manager_v1\0".as_ptr() as *const ::core::ffi::c_char,
        version: 3 as ::core::ffi::c_int,
        method_count: 2 as ::core::ffi::c_int,
        methods: &raw const zxdg_output_manager_v1_requests as *const wl_message,
        event_count: 0 as ::core::ffi::c_int,
        events: ::core::ptr::null::<wl_message>(),
    }
};
static mut zxdg_output_v1_requests: [wl_message; 1] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 1];
static mut zxdg_output_v1_events: [wl_message; 5] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 5];
#[no_mangle]
pub static mut zxdg_output_v1_interface: wl_interface = unsafe {
    wl_interface {
        name: b"zxdg_output_v1\0".as_ptr() as *const ::core::ffi::c_char,
        version: 3 as ::core::ffi::c_int,
        method_count: 1 as ::core::ffi::c_int,
        methods: &raw const zxdg_output_v1_requests as *const wl_message,
        event_count: 5 as ::core::ffi::c_int,
        events: &raw const zxdg_output_v1_events as *const wl_message,
    }
};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
unsafe extern "C" fn c2rust_run_static_initializers() {
    zxdg_output_v1_events = [
        wl_message {
            name: b"logical_position\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"ii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_output_unstable_v1_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"logical_size\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"ii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_output_unstable_v1_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"done\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_output_unstable_v1_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"name\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"2s\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_output_unstable_v1_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"description\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"2s\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_output_unstable_v1_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zxdg_output_v1_requests = [
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_output_unstable_v1_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zxdg_output_manager_v1_requests = [
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_output_unstable_v1_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"get_xdg_output\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"no\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_output_unstable_v1_types as *mut *const wl_interface)
                .offset(2 as ::core::ffi::c_int as isize),
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
