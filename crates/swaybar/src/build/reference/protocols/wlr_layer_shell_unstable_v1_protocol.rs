extern "C" {
    static wl_output_interface: wl_interface;
    static wl_surface_interface: wl_interface;
    static xdg_popup_interface: wl_interface;
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
static mut wlr_layer_shell_unstable_v1_types: [*const wl_interface; 10] = unsafe {
    [
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        &raw const zwlr_layer_surface_v1_interface,
        &raw const wl_surface_interface,
        &raw const wl_output_interface,
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        &raw const xdg_popup_interface,
    ]
};
static mut zwlr_layer_shell_v1_requests: [wl_message; 2] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 2];
#[no_mangle]
pub static mut zwlr_layer_shell_v1_interface: wl_interface = unsafe {
    wl_interface {
        name: b"zwlr_layer_shell_v1\0".as_ptr() as *const ::core::ffi::c_char,
        version: 5 as ::core::ffi::c_int,
        method_count: 2 as ::core::ffi::c_int,
        methods: &raw const zwlr_layer_shell_v1_requests as *const wl_message,
        event_count: 0 as ::core::ffi::c_int,
        events: ::core::ptr::null::<wl_message>(),
    }
};
static mut zwlr_layer_surface_v1_requests: [wl_message; 10] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 10];
static mut zwlr_layer_surface_v1_events: [wl_message; 2] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 2];
#[no_mangle]
pub static mut zwlr_layer_surface_v1_interface: wl_interface = unsafe {
    wl_interface {
        name: b"zwlr_layer_surface_v1\0".as_ptr() as *const ::core::ffi::c_char,
        version: 5 as ::core::ffi::c_int,
        method_count: 10 as ::core::ffi::c_int,
        methods: &raw const zwlr_layer_surface_v1_requests as *const wl_message,
        event_count: 2 as ::core::ffi::c_int,
        events: &raw const zwlr_layer_surface_v1_events as *const wl_message,
    }
};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
unsafe extern "C" fn c2rust_run_static_initializers() {
    zwlr_layer_surface_v1_events = [
        wl_message {
            name: b"configure\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"uuu\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"closed\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwlr_layer_surface_v1_requests = [
        wl_message {
            name: b"set_size\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"uu\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_anchor\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_exclusive_zone\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"i\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_margin\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"iiii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_keyboard_interactivity\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"get_popup\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"o\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(9 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"ack_configure\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_layer\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"2u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_exclusive_edge\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"5u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwlr_layer_shell_v1_requests = [
        wl_message {
            name: b"get_layer_surface\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"no?ous\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(4 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"3\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut wlr_layer_shell_unstable_v1_types
                as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
