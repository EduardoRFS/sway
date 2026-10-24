extern "C" {
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
static mut tablet_v2_types: [*const wl_interface; 24] = unsafe {
    [
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        &raw const zwp_tablet_seat_v2_interface,
        &raw const wl_seat_interface,
        &raw const zwp_tablet_v2_interface,
        &raw const zwp_tablet_tool_v2_interface,
        &raw const zwp_tablet_pad_v2_interface,
        ::core::ptr::null::<wl_interface>(),
        &raw const wl_surface_interface,
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        ::core::ptr::null::<wl_interface>(),
        &raw const zwp_tablet_v2_interface,
        &raw const wl_surface_interface,
        &raw const zwp_tablet_pad_ring_v2_interface,
        &raw const zwp_tablet_pad_strip_v2_interface,
        &raw const zwp_tablet_pad_dial_v2_interface,
        &raw const zwp_tablet_pad_group_v2_interface,
        ::core::ptr::null::<wl_interface>(),
        &raw const zwp_tablet_v2_interface,
        &raw const wl_surface_interface,
        ::core::ptr::null::<wl_interface>(),
        &raw const wl_surface_interface,
    ]
};
static mut zwp_tablet_manager_v2_requests: [wl_message; 2] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 2];
#[no_mangle]
pub static mut zwp_tablet_manager_v2_interface: wl_interface = unsafe {
    wl_interface {
        name: b"zwp_tablet_manager_v2\0".as_ptr() as *const ::core::ffi::c_char,
        version: 2 as ::core::ffi::c_int,
        method_count: 2 as ::core::ffi::c_int,
        methods: &raw const zwp_tablet_manager_v2_requests as *const wl_message,
        event_count: 0 as ::core::ffi::c_int,
        events: ::core::ptr::null::<wl_message>(),
    }
};
static mut zwp_tablet_seat_v2_requests: [wl_message; 1] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 1];
static mut zwp_tablet_seat_v2_events: [wl_message; 3] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 3];
#[no_mangle]
pub static mut zwp_tablet_seat_v2_interface: wl_interface = unsafe {
    wl_interface {
        name: b"zwp_tablet_seat_v2\0".as_ptr() as *const ::core::ffi::c_char,
        version: 2 as ::core::ffi::c_int,
        method_count: 1 as ::core::ffi::c_int,
        methods: &raw const zwp_tablet_seat_v2_requests as *const wl_message,
        event_count: 3 as ::core::ffi::c_int,
        events: &raw const zwp_tablet_seat_v2_events as *const wl_message,
    }
};
static mut zwp_tablet_tool_v2_requests: [wl_message; 2] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 2];
static mut zwp_tablet_tool_v2_events: [wl_message; 19] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 19];
#[no_mangle]
pub static mut zwp_tablet_tool_v2_interface: wl_interface = unsafe {
    wl_interface {
        name: b"zwp_tablet_tool_v2\0".as_ptr() as *const ::core::ffi::c_char,
        version: 2 as ::core::ffi::c_int,
        method_count: 2 as ::core::ffi::c_int,
        methods: &raw const zwp_tablet_tool_v2_requests as *const wl_message,
        event_count: 19 as ::core::ffi::c_int,
        events: &raw const zwp_tablet_tool_v2_events as *const wl_message,
    }
};
static mut zwp_tablet_v2_requests: [wl_message; 1] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 1];
static mut zwp_tablet_v2_events: [wl_message; 6] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 6];
#[no_mangle]
pub static mut zwp_tablet_v2_interface: wl_interface = unsafe {
    wl_interface {
        name: b"zwp_tablet_v2\0".as_ptr() as *const ::core::ffi::c_char,
        version: 2 as ::core::ffi::c_int,
        method_count: 1 as ::core::ffi::c_int,
        methods: &raw const zwp_tablet_v2_requests as *const wl_message,
        event_count: 6 as ::core::ffi::c_int,
        events: &raw const zwp_tablet_v2_events as *const wl_message,
    }
};
static mut zwp_tablet_pad_ring_v2_requests: [wl_message; 2] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 2];
static mut zwp_tablet_pad_ring_v2_events: [wl_message; 4] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 4];
#[no_mangle]
pub static mut zwp_tablet_pad_ring_v2_interface: wl_interface = unsafe {
    wl_interface {
        name: b"zwp_tablet_pad_ring_v2\0".as_ptr() as *const ::core::ffi::c_char,
        version: 2 as ::core::ffi::c_int,
        method_count: 2 as ::core::ffi::c_int,
        methods: &raw const zwp_tablet_pad_ring_v2_requests as *const wl_message,
        event_count: 4 as ::core::ffi::c_int,
        events: &raw const zwp_tablet_pad_ring_v2_events as *const wl_message,
    }
};
static mut zwp_tablet_pad_strip_v2_requests: [wl_message; 2] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 2];
static mut zwp_tablet_pad_strip_v2_events: [wl_message; 4] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 4];
#[no_mangle]
pub static mut zwp_tablet_pad_strip_v2_interface: wl_interface = unsafe {
    wl_interface {
        name: b"zwp_tablet_pad_strip_v2\0".as_ptr() as *const ::core::ffi::c_char,
        version: 2 as ::core::ffi::c_int,
        method_count: 2 as ::core::ffi::c_int,
        methods: &raw const zwp_tablet_pad_strip_v2_requests as *const wl_message,
        event_count: 4 as ::core::ffi::c_int,
        events: &raw const zwp_tablet_pad_strip_v2_events as *const wl_message,
    }
};
static mut zwp_tablet_pad_group_v2_requests: [wl_message; 1] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 1];
static mut zwp_tablet_pad_group_v2_events: [wl_message; 7] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 7];
#[no_mangle]
pub static mut zwp_tablet_pad_group_v2_interface: wl_interface = unsafe {
    wl_interface {
        name: b"zwp_tablet_pad_group_v2\0".as_ptr() as *const ::core::ffi::c_char,
        version: 2 as ::core::ffi::c_int,
        method_count: 1 as ::core::ffi::c_int,
        methods: &raw const zwp_tablet_pad_group_v2_requests as *const wl_message,
        event_count: 7 as ::core::ffi::c_int,
        events: &raw const zwp_tablet_pad_group_v2_events as *const wl_message,
    }
};
static mut zwp_tablet_pad_v2_requests: [wl_message; 2] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 2];
static mut zwp_tablet_pad_v2_events: [wl_message; 8] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 8];
#[no_mangle]
pub static mut zwp_tablet_pad_v2_interface: wl_interface = unsafe {
    wl_interface {
        name: b"zwp_tablet_pad_v2\0".as_ptr() as *const ::core::ffi::c_char,
        version: 2 as ::core::ffi::c_int,
        method_count: 2 as ::core::ffi::c_int,
        methods: &raw const zwp_tablet_pad_v2_requests as *const wl_message,
        event_count: 8 as ::core::ffi::c_int,
        events: &raw const zwp_tablet_pad_v2_events as *const wl_message,
    }
};
static mut zwp_tablet_pad_dial_v2_requests: [wl_message; 2] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 2];
static mut zwp_tablet_pad_dial_v2_events: [wl_message; 2] = [wl_message {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    signature: ::core::ptr::null::<::core::ffi::c_char>(),
    types: ::core::ptr::null_mut::<*const wl_interface>(),
}; 2];
#[no_mangle]
pub static mut zwp_tablet_pad_dial_v2_interface: wl_interface = unsafe {
    wl_interface {
        name: b"zwp_tablet_pad_dial_v2\0".as_ptr() as *const ::core::ffi::c_char,
        version: 2 as ::core::ffi::c_int,
        method_count: 2 as ::core::ffi::c_int,
        methods: &raw const zwp_tablet_pad_dial_v2_requests as *const wl_message,
        event_count: 2 as ::core::ffi::c_int,
        events: &raw const zwp_tablet_pad_dial_v2_events as *const wl_message,
    }
};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
unsafe extern "C" fn c2rust_run_static_initializers() {
    zwp_tablet_pad_dial_v2_events = [
        wl_message {
            name: b"delta\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"i\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"frame\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_v2_events = [
        wl_message {
            name: b"name\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"s\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"id\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"uu\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"path\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"s\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"done\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"removed\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"bustype\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"2u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_v2_requests = [
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_pad_group_v2_events = [
        wl_message {
            name: b"buttons\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"a\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"ring\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"n\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(15 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"strip\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"n\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(16 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"modes\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"done\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"mode_switch\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"uuu\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"dial\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"2n\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(17 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_pad_group_v2_requests = [
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_pad_strip_v2_events = [
        wl_message {
            name: b"source\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"position\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"stop\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"frame\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_pad_strip_v2_requests = [
        wl_message {
            name: b"set_feedback\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"su\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_pad_ring_v2_events = [
        wl_message {
            name: b"source\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"angle\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"f\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"stop\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"frame\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_pad_ring_v2_requests = [
        wl_message {
            name: b"set_feedback\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"su\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_pad_v2_events = [
        wl_message {
            name: b"group\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"n\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(18 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"path\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"s\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"buttons\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"done\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"button\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"uuu\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"enter\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"uoo\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(19 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"leave\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"uo\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(22 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"removed\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_pad_v2_requests = [
        wl_message {
            name: b"set_feedback\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"usu\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_tool_v2_events = [
        wl_message {
            name: b"type\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"hardware_serial\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"uu\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"hardware_id_wacom\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"uu\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"capability\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"done\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"removed\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"proximity_in\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"uoo\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(12 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"proximity_out\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"down\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"up\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"motion\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"ff\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"pressure\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"distance\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"tilt\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"ff\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"rotation\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"f\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"slider\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"i\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"wheel\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"fi\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"button\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"uuu\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"frame\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_tool_v2_requests = [
        wl_message {
            name: b"set_cursor\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"u?oii\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(8 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_seat_v2_events = [
        wl_message {
            name: b"tablet_added\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"n\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(5 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"tool_added\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"n\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(6 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"pad_added\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"n\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(7 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_seat_v2_requests = [
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_pad_dial_v2_requests = [
        wl_message {
            name: b"set_feedback\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"su\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
    zwp_tablet_manager_v2_requests = [
        wl_message {
            name: b"get_tablet_seat\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"no\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(3 as ::core::ffi::c_int as isize),
        },
        wl_message {
            name: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
            signature: b"\0".as_ptr() as *const ::core::ffi::c_char,
            types: (&raw mut tablet_v2_types as *mut *const wl_interface)
                .offset(0 as ::core::ffi::c_int as isize),
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
