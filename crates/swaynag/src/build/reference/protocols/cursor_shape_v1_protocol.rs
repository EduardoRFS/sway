extern "C" {
    static wl_pointer_interface: wl_interface;
    static zwp_tablet_tool_v2_interface: wl_interface;
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
static mut cursor_shape_v1_types: [*const wl_interface; 6] = unsafe {
    [
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        &raw const wp_cursor_shape_device_v1_interface,
        &raw const wl_pointer_interface,
        &raw const wp_cursor_shape_device_v1_interface,
        &raw const zwp_tablet_tool_v2_interface,
    ]
};
static mut wp_cursor_shape_manager_v1_requests: [wl_message; 3] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 3];
#[no_mangle]
pub static mut wp_cursor_shape_manager_v1_interface: wl_interface = unsafe {
    wl_interface {
        name: b"wp_cursor_shape_manager_v1\0".as_ptr() as *const ::core::ffi::c_char,
        version: 2 as ::core::ffi::c_int,
        method_count: 3 as ::core::ffi::c_int,
        methods: &raw const wp_cursor_shape_manager_v1_requests as *const wl_message,
        event_count: 0 as ::core::ffi::c_int,
        events: ::core::ptr::null::<wl_message>(),
    }
};
static mut wp_cursor_shape_device_v1_requests: [wl_message; 2] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 2];
#[no_mangle]
pub static mut wp_cursor_shape_device_v1_interface: wl_interface = unsafe {
    wl_interface {
        name: b"wp_cursor_shape_device_v1\0".as_ptr() as *const ::core::ffi::c_char,
        version: 2 as ::core::ffi::c_int,
        method_count: 2 as ::core::ffi::c_int,
        methods: &raw const wp_cursor_shape_device_v1_requests as *const wl_message,
        event_count: 0 as ::core::ffi::c_int,
        events: ::core::ptr::null::<wl_message>(),
    }
};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
unsafe extern "C" fn c2rust_run_static_initializers() {
    wp_cursor_shape_device_v1_requests = [
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut cursor_shape_v1_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_shape\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"uu\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut cursor_shape_v1_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    wp_cursor_shape_manager_v1_requests = [
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut cursor_shape_v1_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"get_pointer\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"no\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut cursor_shape_v1_types as *mut *const wl_interface)
                .offset(2 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"get_tablet_tool_v2\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"no\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut cursor_shape_v1_types as *mut *const wl_interface)
                .offset(4 as ::core::ffi::c_int as isize),
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
