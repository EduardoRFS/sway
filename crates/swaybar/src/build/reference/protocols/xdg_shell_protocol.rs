extern "C" {
    static wl_output_interface: wl_interface;
    static wl_seat_interface: wl_interface;
    static wl_surface_interface: wl_interface;
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
static mut xdg_shell_types: [*const wl_interface; 26] = unsafe {
    [
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        &raw const xdg_positioner_interface,
        &raw const xdg_surface_interface,
        &raw const wl_surface_interface,
        &raw const xdg_toplevel_interface,
        &raw const xdg_popup_interface,
        &raw const xdg_surface_interface,
        &raw const xdg_positioner_interface,
        &raw const xdg_toplevel_interface,
        &raw const wl_seat_interface,
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        &raw const wl_seat_interface,
        ::core::ptr::null::<wl_interface>(),
        &raw const wl_seat_interface,
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        &raw const wl_output_interface,
        &raw const wl_seat_interface,
        ::core::ptr::null::<wl_interface>(),
        &raw const xdg_positioner_interface,
        ::core::ptr::null::<wl_interface>(),
    ]
};
static mut xdg_wm_base_requests: [wl_message; 4] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 4];
static mut xdg_wm_base_events: [wl_message; 1] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 1];
#[no_mangle]
pub static mut xdg_wm_base_interface: wl_interface = unsafe {
    wl_interface {
        name: b"xdg_wm_base\0".as_ptr() as *const ::core::ffi::c_char,
        version: 7 as ::core::ffi::c_int,
        method_count: 4 as ::core::ffi::c_int,
        methods: &raw const xdg_wm_base_requests as *const wl_message,
        event_count: 1 as ::core::ffi::c_int,
        events: &raw const xdg_wm_base_events as *const wl_message,
    }
};
static mut xdg_positioner_requests: [wl_message; 10] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 10];
#[no_mangle]
pub static mut xdg_positioner_interface: wl_interface = unsafe {
    wl_interface {
        name: b"xdg_positioner\0".as_ptr() as *const ::core::ffi::c_char,
        version: 7 as ::core::ffi::c_int,
        method_count: 10 as ::core::ffi::c_int,
        methods: &raw const xdg_positioner_requests as *const wl_message,
        event_count: 0 as ::core::ffi::c_int,
        events: ::core::ptr::null::<wl_message>(),
    }
};
static mut xdg_surface_requests: [wl_message; 5] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 5];
static mut xdg_surface_events: [wl_message; 1] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 1];
#[no_mangle]
pub static mut xdg_surface_interface: wl_interface = unsafe {
    wl_interface {
        name: b"xdg_surface\0".as_ptr() as *const ::core::ffi::c_char,
        version: 7 as ::core::ffi::c_int,
        method_count: 5 as ::core::ffi::c_int,
        methods: &raw const xdg_surface_requests as *const wl_message,
        event_count: 1 as ::core::ffi::c_int,
        events: &raw const xdg_surface_events as *const wl_message,
    }
};
static mut xdg_toplevel_requests: [wl_message; 14] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 14];
static mut xdg_toplevel_events: [wl_message; 4] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 4];
#[no_mangle]
pub static mut xdg_toplevel_interface: wl_interface = unsafe {
    wl_interface {
        name: b"xdg_toplevel\0".as_ptr() as *const ::core::ffi::c_char,
        version: 7 as ::core::ffi::c_int,
        method_count: 14 as ::core::ffi::c_int,
        methods: &raw const xdg_toplevel_requests as *const wl_message,
        event_count: 4 as ::core::ffi::c_int,
        events: &raw const xdg_toplevel_events as *const wl_message,
    }
};
static mut xdg_popup_requests: [wl_message; 3] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 3];
static mut xdg_popup_events: [wl_message; 3] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 3];
#[no_mangle]
pub static mut xdg_popup_interface: wl_interface = unsafe {
    wl_interface {
        name: b"xdg_popup\0".as_ptr() as *const ::core::ffi::c_char,
        version: 7 as ::core::ffi::c_int,
        method_count: 3 as ::core::ffi::c_int,
        methods: &raw const xdg_popup_requests as *const wl_message,
        event_count: 3 as ::core::ffi::c_int,
        events: &raw const xdg_popup_events as *const wl_message,
    }
};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
unsafe extern "C" fn c2rust_run_static_initializers() {
    xdg_popup_events = [
        wl_message {
            name: b"configure\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"iiii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"popup_done\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"repositioned\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"3u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    xdg_positioner_requests = [
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_size\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"ii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_anchor_rect\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"iiii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_anchor\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_gravity\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_constraint_adjustment\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_offset\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"ii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_reactive\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"3\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_parent_size\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"3ii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_parent_configure\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"3u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    xdg_toplevel_events = [
        wl_message {
            name: b"configure\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"iia\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"close\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"configure_bounds\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"4ii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"wm_capabilities\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"5a\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    xdg_toplevel_requests = [
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_parent\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"?o\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(11 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_title\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"s\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_app_id\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"s\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"show_window_menu\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"ouii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(12 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"move\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"ou\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(16 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"resize\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"ouu\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(18 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_max_size\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"ii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_min_size\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"ii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_maximized\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"unset_maximized\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_fullscreen\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"?o\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(21 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"unset_fullscreen\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_minimized\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    xdg_surface_events = [
        wl_message {
            name: b"configure\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    xdg_surface_requests = [
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"get_toplevel\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"n\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(7 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"get_popup\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"n?oo\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(8 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"set_window_geometry\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"iiii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"ack_configure\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    xdg_popup_requests = [
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"grab\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"ou\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(22 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"reposition\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"3ou\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(24 as ::core::ffi::c_int as isize),
        },
    ];
    xdg_wm_base_requests = [
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"create_positioner\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"n\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(4 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"get_xdg_surface\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"no\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(5 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"pong\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    xdg_wm_base_events = [
        wl_message {
            name: b"ping\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut xdg_shell_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
