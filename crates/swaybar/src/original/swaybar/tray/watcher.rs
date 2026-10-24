use ::c2rust_bitfields;
extern "C" {
    pub type sd_bus;
    pub type sd_bus_message;
    pub type sd_bus_slot;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn create_list() -> *mut list_t;
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn list_del(list: *mut list_t, index: ::core::ffi::c_int);
    fn list_seq_find(
        list: *mut list_t,
        compare: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        cmp_to: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn list_free_items_and_destroy(list: *mut list_t);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn format_str(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn has_prefix(
        str: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    ) -> bool;
    static sd_bus_object_vtable_format: ::core::ffi::c_uint;
    fn sd_bus_add_object_vtable(
        bus: *mut sd_bus,
        ret_slot: *mut *mut sd_bus_slot,
        path: *const ::core::ffi::c_char,
        interface: *const ::core::ffi::c_char,
        vtable: *const sd_bus_vtable,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sd_bus_slot_unref(p: *mut sd_bus_slot) -> *mut sd_bus_slot;
    fn sd_bus_slot_set_floating(
        slot: *mut sd_bus_slot,
        b: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sd_bus_message_get_sender(m: *mut sd_bus_message) -> *const ::core::ffi::c_char;
    fn sd_bus_message_append_basic(
        m: *mut sd_bus_message,
        r#type: ::core::ffi::c_char,
        p: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sd_bus_message_append_strv(
        m: *mut sd_bus_message,
        l: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sd_bus_message_read(
        m: *mut sd_bus_message,
        types: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sd_bus_request_name(
        bus: *mut sd_bus,
        name: *const ::core::ffi::c_char,
        flags: uint64_t,
    ) -> ::core::ffi::c_int;
    fn sd_bus_reply_method_return(
        call: *mut sd_bus_message,
        types: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sd_bus_emit_signal(
        bus: *mut sd_bus,
        path: *const ::core::ffi::c_char,
        interface: *const ::core::ffi::c_char,
        member: *const ::core::ffi::c_char,
        types: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sd_bus_emit_properties_changed(
        bus: *mut sd_bus,
        path: *const ::core::ffi::c_char,
        interface: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sd_bus_match_signal(
        bus: *mut sd_bus,
        ret: *mut *mut sd_bus_slot,
        sender: *const ::core::ffi::c_char,
        path: *const ::core::ffi::c_char,
        interface: *const ::core::ffi::c_char,
        member: *const ::core::ffi::c_char,
        callback: sd_bus_message_handler_t,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
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
pub type uint8_t = u8;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sd_bus_error {
    pub name: *const ::core::ffi::c_char,
    pub message: *const ::core::ffi::c_char,
    pub _need_free: ::core::ffi::c_int,
}
pub type sd_bus_message_handler_t = Option<
    unsafe extern "C" fn(
        *mut sd_bus_message,
        *mut ::core::ffi::c_void,
        *mut sd_bus_error,
    ) -> ::core::ffi::c_int,
>;
pub type sd_bus_property_get_t = Option<
    unsafe extern "C" fn(
        *mut sd_bus,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut sd_bus_message,
        *mut ::core::ffi::c_void,
        *mut sd_bus_error,
    ) -> ::core::ffi::c_int,
>;
pub type sd_bus_property_set_t = Option<
    unsafe extern "C" fn(
        *mut sd_bus,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut sd_bus_message,
        *mut ::core::ffi::c_void,
        *mut sd_bus_error,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct sd_bus_vtable {
    #[bitfield(name = "type_value", ty = "uint8_t", bits = "0..=7")]
    #[bitfield(name = "flags", ty = "uint64_t", bits = "8..=63")]
    pub r#type_flags: [u8; 8],
    pub x: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub start: C2Rust_Unnamed_4,
    pub end: C2Rust_Unnamed_3,
    pub method: C2Rust_Unnamed_2,
    pub signal: C2Rust_Unnamed_1,
    pub property: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub member: *const ::core::ffi::c_char,
    pub signature: *const ::core::ffi::c_char,
    pub get: sd_bus_property_get_t,
    pub set: sd_bus_property_set_t,
    pub offset: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub member: *const ::core::ffi::c_char,
    pub signature: *const ::core::ffi::c_char,
    pub names: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub member: *const ::core::ffi::c_char,
    pub signature: *const ::core::ffi::c_char,
    pub result: *const ::core::ffi::c_char,
    pub handler: sd_bus_message_handler_t,
    pub offset: size_t,
    pub names: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub _reserved: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub element_size: size_t,
    pub features: uint64_t,
    pub vtable_format_reference: *const ::core::ffi::c_uint,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const _SD_BUS_VTABLE_START: Self = Self(60);
    pub const _SD_BUS_VTABLE_END: Self = Self(62);
    pub const _SD_BUS_VTABLE_METHOD: Self = Self(77);
    pub const _SD_BUS_VTABLE_SIGNAL: Self = Self(83);
    pub const _SD_BUS_VTABLE_PROPERTY: Self = Self(80);
    pub const _SD_BUS_VTABLE_WRITABLE_PROPERTY: Self = Self(87);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_ulong);
impl C2Rust_Unnamed_6 {
    pub const SD_BUS_VTABLE_DEPRECATED: Self = Self(1);
    pub const SD_BUS_VTABLE_HIDDEN: Self = Self(2);
    pub const SD_BUS_VTABLE_UNPRIVILEGED: Self = Self(4);
    pub const SD_BUS_VTABLE_METHOD_NO_REPLY: Self = Self(8);
    pub const SD_BUS_VTABLE_PROPERTY_CONST: Self = Self(16);
    pub const SD_BUS_VTABLE_PROPERTY_EMITS_CHANGE: Self = Self(32);
    pub const SD_BUS_VTABLE_PROPERTY_EMITS_INVALIDATION: Self = Self(64);
    pub const SD_BUS_VTABLE_PROPERTY_EXPLICIT: Self = Self(128);
    pub const SD_BUS_VTABLE_SENSITIVE: Self = Self(256);
    pub const SD_BUS_VTABLE_ABSOLUTE_OFFSET: Self = Self(512);
    pub const _SD_BUS_VTABLE_CAPABILITY_MASK: Self = Self(72056494526300160);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
    pub const _SD_BUS_VTABLE_PARAM_NAMES: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_watcher {
    pub interface: *mut ::core::ffi::c_char,
    pub bus: *mut sd_bus,
    pub hosts: *mut list_t,
    pub items: *mut list_t,
    pub version: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
static mut obj_path: *const ::core::ffi::c_char = b"/StatusNotifierWatcher\0".as_ptr()
    as *const ::core::ffi::c_char;
unsafe extern "C" fn using_standard_protocol(mut watcher: *mut swaybar_watcher) -> bool {
    return *(*watcher)
        .interface
        .offset(strlen(b"org.\0".as_ptr() as *const ::core::ffi::c_char) as isize)
        as ::core::ffi::c_int == 'f' as ::core::ffi::c_int;
}
unsafe extern "C" fn cmp_id(
    mut item: *const ::core::ffi::c_void,
    mut cmp_to: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        item as *const ::core::ffi::c_char,
        cmp_to as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn handle_lost_service(
    mut msg: *mut sd_bus_message,
    mut data: *mut ::core::ffi::c_void,
    mut error: *mut sd_bus_error,
) -> ::core::ffi::c_int {
    let mut service: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut old_owner: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut new_owner: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut ret: ::core::ffi::c_int = sd_bus_message_read(
        msg,
        b"sss\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut service,
        &raw mut old_owner,
        &raw mut new_owner,
    );
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to parse owner change message: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/watcher.c\0".as_ptr() as *const ::core::ffi::c_char,
            26 as ::core::ffi::c_int,
            strerror(-ret),
        );
        return ret;
    }
    if *new_owner == 0 {
        let mut watcher: *mut swaybar_watcher = data as *mut swaybar_watcher;
        let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while idx < (*(*watcher).items).length {
            let mut id: *mut ::core::ffi::c_char = *(*(*watcher).items)
                .items
                .offset(idx as isize) as *mut ::core::ffi::c_char;
            let mut cmp_res: bool = if using_standard_protocol(watcher)
                as ::core::ffi::c_int != 0
            {
                (cmp_id(
                    id as *const ::core::ffi::c_void,
                    service as *const ::core::ffi::c_void,
                ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            } else {
                has_prefix(id, service) as ::core::ffi::c_int
            } != 0;
            if cmp_res {
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Unregistering Status Notifier Item '%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaybar/tray/watcher.c\0".as_ptr() as *const ::core::ffi::c_char,
                    37 as ::core::ffi::c_int,
                    id,
                );
                let c2rust_fresh0 = idx;
                idx -= 1;
                list_del((*watcher).items, c2rust_fresh0);
                sd_bus_emit_signal(
                    (*watcher).bus,
                    obj_path,
                    (*watcher).interface,
                    b"StatusNotifierItemUnregistered\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"s\0".as_ptr() as *const ::core::ffi::c_char,
                    id,
                );
                sd_bus_emit_properties_changed(
                    (*watcher).bus,
                    obj_path,
                    (*watcher).interface,
                    b"RegisteredStatusNotifierItems\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    NULL,
                );
                free(id as *mut ::core::ffi::c_void);
                if using_standard_protocol(watcher) {
                    break;
                }
            }
            idx += 1;
        }
        let mut idx_0: ::core::ffi::c_int = list_seq_find(
            (*watcher).hosts,
            Some(
                cmp_id
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            service as *const ::core::ffi::c_void,
        );
        if idx_0 != -1 as ::core::ffi::c_int {
            _sway_log(
                sway_log_importance_t::SWAY_DEBUG,
                b"[%s:%d] Unregistering Status Notifier Host '%s'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaybar/tray/watcher.c\0".as_ptr() as *const ::core::ffi::c_char,
                52 as ::core::ffi::c_int,
                service,
            );
            free(*(*(*watcher).hosts).items.offset(idx_0 as isize));
            list_del((*watcher).hosts, idx_0);
            if (*(*watcher).hosts).length == 0 as ::core::ffi::c_int {
                sd_bus_emit_properties_changed(
                    (*watcher).bus,
                    obj_path,
                    (*watcher).interface,
                    b"IsStatusNotifierHostRegistered\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    NULL,
                );
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn register_sni(
    mut msg: *mut sd_bus_message,
    mut data: *mut ::core::ffi::c_void,
    mut error: *mut sd_bus_error,
) -> ::core::ffi::c_int {
    let mut service_or_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut ret: ::core::ffi::c_int = sd_bus_message_read(
        msg,
        b"s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut service_or_path,
    );
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to parse register SNI message: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/watcher.c\0".as_ptr() as *const ::core::ffi::c_char,
            69 as ::core::ffi::c_int,
            strerror(-ret),
        );
        return ret;
    }
    let mut watcher: *mut swaybar_watcher = data as *mut swaybar_watcher;
    if using_standard_protocol(watcher) {
        id = strdup(service_or_path);
    } else {
        let mut service: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut path: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if *service_or_path.offset(0isize) as ::core::ffi::c_int
            == '/' as ::core::ffi::c_int
        {
            service = sd_bus_message_get_sender(msg);
            path = service_or_path;
        } else {
            service = service_or_path;
            path = b"/StatusNotifierItem\0".as_ptr() as *const ::core::ffi::c_char;
        }
        id = format_str(b"%s%s\0".as_ptr() as *const ::core::ffi::c_char, service, path);
    }
    if list_seq_find(
        (*watcher).items,
        Some(
            cmp_id
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        id as *const ::core::ffi::c_void,
    ) == -1 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Registering Status Notifier Item '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/watcher.c\0".as_ptr() as *const ::core::ffi::c_char,
            89 as ::core::ffi::c_int,
            id,
        );
        list_add((*watcher).items, id as *mut ::core::ffi::c_void);
        sd_bus_emit_properties_changed(
            (*watcher).bus,
            obj_path,
            (*watcher).interface,
            b"RegisteredStatusNotifierItems\0".as_ptr() as *const ::core::ffi::c_char,
            NULL,
        );
        sd_bus_emit_signal(
            (*watcher).bus,
            obj_path,
            (*watcher).interface,
            b"StatusNotifierItemRegistered\0".as_ptr() as *const ::core::ffi::c_char,
            b"s\0".as_ptr() as *const ::core::ffi::c_char,
            id,
        );
    } else {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Status Notifier Item '%s' already registered\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/watcher.c\0".as_ptr() as *const ::core::ffi::c_char,
            96 as ::core::ffi::c_int,
            id,
        );
        free(id as *mut ::core::ffi::c_void);
    }
    return sd_bus_reply_method_return(msg, b"\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn register_host(
    mut msg: *mut sd_bus_message,
    mut data: *mut ::core::ffi::c_void,
    mut error: *mut sd_bus_error,
) -> ::core::ffi::c_int {
    let mut service: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut ret: ::core::ffi::c_int = sd_bus_message_read(
        msg,
        b"s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut service,
    );
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to parse register host message: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/watcher.c\0".as_ptr() as *const ::core::ffi::c_char,
            107 as ::core::ffi::c_int,
            strerror(-ret),
        );
        return ret;
    }
    let mut watcher: *mut swaybar_watcher = data as *mut swaybar_watcher;
    if list_seq_find(
        (*watcher).hosts,
        Some(
            cmp_id
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        service as *const ::core::ffi::c_void,
    ) == -1 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Registering Status Notifier Host '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/watcher.c\0".as_ptr() as *const ::core::ffi::c_char,
            113 as ::core::ffi::c_int,
            service,
        );
        list_add((*watcher).hosts, strdup(service) as *mut ::core::ffi::c_void);
        if (*(*watcher).hosts).length == 1 as ::core::ffi::c_int {
            sd_bus_emit_properties_changed(
                (*watcher).bus,
                obj_path,
                (*watcher).interface,
                b"IsStatusNotifierHostRegistered\0".as_ptr()
                    as *const ::core::ffi::c_char,
                NULL,
            );
        }
        sd_bus_emit_signal(
            (*watcher).bus,
            obj_path,
            (*watcher).interface,
            b"StatusNotifierHostRegistered\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Status Notifier Host '%s' already registered\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/watcher.c\0".as_ptr() as *const ::core::ffi::c_char,
            122 as ::core::ffi::c_int,
            service,
        );
    }
    return sd_bus_reply_method_return(msg, b"\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn get_registered_snis(
    mut bus: *mut sd_bus,
    mut obj_path_0: *const ::core::ffi::c_char,
    mut interface: *const ::core::ffi::c_char,
    mut property: *const ::core::ffi::c_char,
    mut reply: *mut sd_bus_message,
    mut data: *mut ::core::ffi::c_void,
    mut error: *mut sd_bus_error,
) -> ::core::ffi::c_int {
    let mut watcher: *mut swaybar_watcher = data as *mut swaybar_watcher;
    list_add((*watcher).items, NULL);
    let mut ret: ::core::ffi::c_int = sd_bus_message_append_strv(
        reply,
        (*(*watcher).items).items as *mut *mut ::core::ffi::c_char,
    );
    list_del((*watcher).items, (*(*watcher).items).length - 1 as ::core::ffi::c_int);
    return ret;
}
unsafe extern "C" fn is_host_registered(
    mut bus: *mut sd_bus,
    mut obj_path_0: *const ::core::ffi::c_char,
    mut interface: *const ::core::ffi::c_char,
    mut property: *const ::core::ffi::c_char,
    mut reply: *mut sd_bus_message,
    mut data: *mut ::core::ffi::c_void,
    mut error: *mut sd_bus_error,
) -> ::core::ffi::c_int {
    let mut watcher: *mut swaybar_watcher = data as *mut swaybar_watcher;
    let mut val: ::core::ffi::c_int = ((*(*watcher).hosts).length
        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    return sd_bus_message_append_basic(
        reply,
        'b' as ::core::ffi::c_char,
        &raw mut val as *const ::core::ffi::c_void,
    );
}
static mut watcher_vtable: [sd_bus_vtable; 10] = [sd_bus_vtable {
    r#type_flags: [0; 8],
    x: C2Rust_Unnamed {
        start: C2Rust_Unnamed_4 {
            element_size: 0,
            features: 0,
            vtable_format_reference: ::core::ptr::null::<::core::ffi::c_uint>(),
        },
    },
}; 10];
#[no_mangle]
pub unsafe extern "C" fn create_watcher(
    mut protocol: *mut ::core::ffi::c_char,
    mut bus: *mut sd_bus,
) -> *mut swaybar_watcher {
    let mut watcher: *mut swaybar_watcher = calloc(
        1 as size_t,
        ::core::mem::size_of::<swaybar_watcher>(),
    ) as *mut swaybar_watcher;
    if watcher.is_null() {
        return ::core::ptr::null_mut::<swaybar_watcher>();
    }
    (*watcher).interface = format_str(
        b"org.%s.StatusNotifierWatcher\0".as_ptr() as *const ::core::ffi::c_char,
        protocol,
    );
    let mut signal_slot: *mut sd_bus_slot = ::core::ptr::null_mut::<sd_bus_slot>();
    let mut vtable_slot: *mut sd_bus_slot = ::core::ptr::null_mut::<sd_bus_slot>();
    let mut ret: ::core::ffi::c_int = sd_bus_add_object_vtable(
        bus,
        &raw mut vtable_slot,
        obj_path,
        (*watcher).interface,
        &raw const watcher_vtable as *const sd_bus_vtable,
        watcher as *mut ::core::ffi::c_void,
    );
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to add object vtable: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/watcher.c\0".as_ptr() as *const ::core::ffi::c_char,
            178 as ::core::ffi::c_int,
            strerror(-ret),
        );
    } else {
        ret = sd_bus_match_signal(
            bus,
            &raw mut signal_slot,
            b"org.freedesktop.DBus\0".as_ptr() as *const ::core::ffi::c_char,
            b"/org/freedesktop/DBus\0".as_ptr() as *const ::core::ffi::c_char,
            b"org.freedesktop.DBus\0".as_ptr() as *const ::core::ffi::c_char,
            b"NameOwnerChanged\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                handle_lost_service
                    as unsafe extern "C" fn(
                        *mut sd_bus_message,
                        *mut ::core::ffi::c_void,
                        *mut sd_bus_error,
                    ) -> ::core::ffi::c_int,
            ),
            watcher as *mut ::core::ffi::c_void,
        );
        if ret < 0 as ::core::ffi::c_int {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Failed to subscribe to unregistering events: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaybar/tray/watcher.c\0".as_ptr() as *const ::core::ffi::c_char,
                186 as ::core::ffi::c_int,
                strerror(-ret),
            );
        } else {
            ret = sd_bus_request_name(bus, (*watcher).interface, 0 as uint64_t);
            if ret < 0 as ::core::ffi::c_int {
                if -ret == EEXIST {
                    _sway_log(
                        sway_log_importance_t::SWAY_DEBUG,
                        b"[%s:%d] Failed to acquire service name '%s':another tray is already running\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"swaybar/tray/watcher.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        194 as ::core::ffi::c_int,
                        (*watcher).interface,
                    );
                } else {
                    _sway_log(
                        sway_log_importance_t::SWAY_ERROR,
                        b"[%s:%d] Failed to acquire service name '%s': %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"swaybar/tray/watcher.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        197 as ::core::ffi::c_int,
                        (*watcher).interface,
                        strerror(-ret),
                    );
                }
            } else {
                sd_bus_slot_set_floating(signal_slot, 0 as ::core::ffi::c_int);
                sd_bus_slot_set_floating(vtable_slot, 0 as ::core::ffi::c_int);
                (*watcher).bus = bus;
                (*watcher).hosts = create_list();
                (*watcher).items = create_list();
                (*watcher).version = 0 as ::core::ffi::c_int;
                _sway_log(
                    sway_log_importance_t::SWAY_DEBUG,
                    b"[%s:%d] Registered %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"swaybar/tray/watcher.c\0".as_ptr() as *const ::core::ffi::c_char,
                    210 as ::core::ffi::c_int,
                    (*watcher).interface,
                );
                return watcher;
            }
        }
    }
    sd_bus_slot_unref(signal_slot);
    sd_bus_slot_unref(vtable_slot);
    destroy_watcher(watcher);
    return ::core::ptr::null_mut::<swaybar_watcher>();
}
#[no_mangle]
pub unsafe extern "C" fn destroy_watcher(mut watcher: *mut swaybar_watcher) {
    if watcher.is_null() {
        return;
    }
    list_free_items_and_destroy((*watcher).hosts);
    list_free_items_and_destroy((*watcher).items);
    free((*watcher).interface as *mut ::core::ffi::c_void);
    free(watcher as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    watcher_vtable = [
        {
            let mut init = sd_bus_vtable {
                r#type_flags: [0; 8],
                x: C2Rust_Unnamed {
                    start: C2Rust_Unnamed_4 {
                        element_size: ::core::mem::size_of::<sd_bus_vtable>(),
                        features: C2Rust_Unnamed_7::_SD_BUS_VTABLE_PARAM_NAMES.0
                            as ::core::ffi::c_int as uint64_t,
                        vtable_format_reference: &raw const sd_bus_object_vtable_format,
                    },
                },
            };
            init.set_type_value(
                C2Rust_Unnamed_5::_SD_BUS_VTABLE_START.0 as ::core::ffi::c_int as uint8_t,
            );
            init.set_flags(0 as uint64_t);
            init
        },
        {
            let mut init = sd_bus_vtable {
                r#type_flags: [0; 8],
                x: C2Rust_Unnamed {
                    method: C2Rust_Unnamed_2 {
                        member: b"RegisterStatusNotifierItem\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        signature: b"s\0".as_ptr() as *const ::core::ffi::c_char,
                        result: b"\0".as_ptr() as *const ::core::ffi::c_char,
                        handler: Some(
                            register_sni
                                as unsafe extern "C" fn(
                                    *mut sd_bus_message,
                                    *mut ::core::ffi::c_void,
                                    *mut sd_bus_error,
                                ) -> ::core::ffi::c_int,
                        ),
                        offset: 0 as size_t,
                        names: b"\0".as_ptr() as *const ::core::ffi::c_char,
                    },
                },
            };
            init.set_type_value(
                C2Rust_Unnamed_5::_SD_BUS_VTABLE_METHOD.0 as ::core::ffi::c_int
                    as uint8_t,
            );
            init.set_flags(C2Rust_Unnamed_6::SD_BUS_VTABLE_UNPRIVILEGED.0 as uint64_t);
            init
        },
        {
            let mut init = sd_bus_vtable {
                r#type_flags: [0; 8],
                x: C2Rust_Unnamed {
                    method: C2Rust_Unnamed_2 {
                        member: b"RegisterStatusNotifierHost\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        signature: b"s\0".as_ptr() as *const ::core::ffi::c_char,
                        result: b"\0".as_ptr() as *const ::core::ffi::c_char,
                        handler: Some(
                            register_host
                                as unsafe extern "C" fn(
                                    *mut sd_bus_message,
                                    *mut ::core::ffi::c_void,
                                    *mut sd_bus_error,
                                ) -> ::core::ffi::c_int,
                        ),
                        offset: 0 as size_t,
                        names: b"\0".as_ptr() as *const ::core::ffi::c_char,
                    },
                },
            };
            init.set_type_value(
                C2Rust_Unnamed_5::_SD_BUS_VTABLE_METHOD.0 as ::core::ffi::c_int
                    as uint8_t,
            );
            init.set_flags(C2Rust_Unnamed_6::SD_BUS_VTABLE_UNPRIVILEGED.0 as uint64_t);
            init
        },
        {
            let mut init = sd_bus_vtable {
                r#type_flags: [0; 8],
                x: C2Rust_Unnamed {
                    property: C2Rust_Unnamed_0 {
                        member: b"RegisteredStatusNotifierItems\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        signature: b"as\0".as_ptr() as *const ::core::ffi::c_char,
                        get: Some(
                            get_registered_snis
                                as unsafe extern "C" fn(
                                    *mut sd_bus,
                                    *const ::core::ffi::c_char,
                                    *const ::core::ffi::c_char,
                                    *const ::core::ffi::c_char,
                                    *mut sd_bus_message,
                                    *mut ::core::ffi::c_void,
                                    *mut sd_bus_error,
                                ) -> ::core::ffi::c_int,
                        ),
                        set: None,
                        offset: 0 as size_t,
                    },
                },
            };
            init.set_type_value(
                C2Rust_Unnamed_5::_SD_BUS_VTABLE_PROPERTY.0 as ::core::ffi::c_int
                    as uint8_t,
            );
            init.set_flags(
                C2Rust_Unnamed_6::SD_BUS_VTABLE_PROPERTY_EMITS_CHANGE.0 as uint64_t,
            );
            init
        },
        {
            let mut init = sd_bus_vtable {
                r#type_flags: [0; 8],
                x: C2Rust_Unnamed {
                    property: C2Rust_Unnamed_0 {
                        member: b"IsStatusNotifierHostRegistered\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        signature: b"b\0".as_ptr() as *const ::core::ffi::c_char,
                        get: Some(
                            is_host_registered
                                as unsafe extern "C" fn(
                                    *mut sd_bus,
                                    *const ::core::ffi::c_char,
                                    *const ::core::ffi::c_char,
                                    *const ::core::ffi::c_char,
                                    *mut sd_bus_message,
                                    *mut ::core::ffi::c_void,
                                    *mut sd_bus_error,
                                ) -> ::core::ffi::c_int,
                        ),
                        set: None,
                        offset: 0 as size_t,
                    },
                },
            };
            init.set_type_value(
                C2Rust_Unnamed_5::_SD_BUS_VTABLE_PROPERTY.0 as ::core::ffi::c_int
                    as uint8_t,
            );
            init.set_flags(
                C2Rust_Unnamed_6::SD_BUS_VTABLE_PROPERTY_EMITS_CHANGE.0 as uint64_t,
            );
            init
        },
        {
            let mut init = sd_bus_vtable {
                r#type_flags: [0; 8],
                x: C2Rust_Unnamed {
                    property: C2Rust_Unnamed_0 {
                        member: b"ProtocolVersion\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        signature: b"i\0".as_ptr() as *const ::core::ffi::c_char,
                        get: None,
                        set: None,
                        offset: 32 as size_t,
                    },
                },
            };
            init.set_type_value(
                C2Rust_Unnamed_5::_SD_BUS_VTABLE_PROPERTY.0 as ::core::ffi::c_int
                    as uint8_t,
            );
            init.set_flags(C2Rust_Unnamed_6::SD_BUS_VTABLE_PROPERTY_CONST.0 as uint64_t);
            init
        },
        {
            let mut init = sd_bus_vtable {
                r#type_flags: [0; 8],
                x: C2Rust_Unnamed {
                    signal: C2Rust_Unnamed_1 {
                        member: b"StatusNotifierItemRegistered\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        signature: b"s\0".as_ptr() as *const ::core::ffi::c_char,
                        names: b"\0".as_ptr() as *const ::core::ffi::c_char,
                    },
                },
            };
            init.set_type_value(
                C2Rust_Unnamed_5::_SD_BUS_VTABLE_SIGNAL.0 as ::core::ffi::c_int
                    as uint8_t,
            );
            init.set_flags(0 as uint64_t);
            init
        },
        {
            let mut init = sd_bus_vtable {
                r#type_flags: [0; 8],
                x: C2Rust_Unnamed {
                    signal: C2Rust_Unnamed_1 {
                        member: b"StatusNotifierItemUnregistered\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        signature: b"s\0".as_ptr() as *const ::core::ffi::c_char,
                        names: b"\0".as_ptr() as *const ::core::ffi::c_char,
                    },
                },
            };
            init.set_type_value(
                C2Rust_Unnamed_5::_SD_BUS_VTABLE_SIGNAL.0 as ::core::ffi::c_int
                    as uint8_t,
            );
            init.set_flags(0 as uint64_t);
            init
        },
        {
            let mut init = sd_bus_vtable {
                r#type_flags: [0; 8],
                x: C2Rust_Unnamed {
                    signal: C2Rust_Unnamed_1 {
                        member: b"StatusNotifierHostRegistered\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        signature: ::core::ptr::null::<::core::ffi::c_char>(),
                        names: b"\0".as_ptr() as *const ::core::ffi::c_char,
                    },
                },
            };
            init.set_type_value(
                C2Rust_Unnamed_5::_SD_BUS_VTABLE_SIGNAL.0 as ::core::ffi::c_int
                    as uint8_t,
            );
            init.set_flags(0 as uint64_t);
            init
        },
        {
            let mut init = sd_bus_vtable {
                r#type_flags: [0; 8],
                x: C2Rust_Unnamed {
                    end: C2Rust_Unnamed_3 {
                        _reserved: 0 as size_t,
                    },
                },
            };
            init.set_type_value(
                C2Rust_Unnamed_5::_SD_BUS_VTABLE_END.0 as ::core::ffi::c_int as uint8_t,
            );
            init.set_flags(0 as uint64_t);
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
