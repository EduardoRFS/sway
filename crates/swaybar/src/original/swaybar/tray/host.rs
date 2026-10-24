extern "C" {
    pub type wl_display;
    pub type wl_compositor;
    pub type wl_shm;
    pub type _PangoFontDescription;
    pub type _cairo_surface;
    pub type swaybar_watcher;
    pub type sd_bus;
    pub type r#loop;
    pub type status_line;
    pub type wp_cursor_shape_manager_v1;
    pub type zxdg_output_manager_v1;
    pub type zwlr_layer_shell_v1;
    pub type sd_bus_slot;
    pub type sd_bus_message;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn getpid() -> __pid_t;
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
    fn set_bar_dirty(bar: *mut swaybar);
    fn sd_bus_slot_unref(p: *mut sd_bus_slot) -> *mut sd_bus_slot;
    fn sd_bus_slot_set_floating(
        slot: *mut sd_bus_slot,
        b: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sd_bus_message_get_error(m: *mut sd_bus_message) -> *const sd_bus_error;
    fn sd_bus_message_is_method_error(
        m: *mut sd_bus_message,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sd_bus_message_read(
        m: *mut sd_bus_message,
        types: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sd_bus_message_read_strv(
        m: *mut sd_bus_message,
        ret: *mut *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sd_bus_message_enter_container(
        m: *mut sd_bus_message,
        r#type: ::core::ffi::c_char,
        contents: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sd_bus_request_name(
        bus: *mut sd_bus,
        name: *const ::core::ffi::c_char,
        flags: uint64_t,
    ) -> ::core::ffi::c_int;
    fn sd_bus_release_name(
        bus: *mut sd_bus,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sd_bus_call_method_async(
        bus: *mut sd_bus,
        ret_slot: *mut *mut sd_bus_slot,
        destination: *const ::core::ffi::c_char,
        path: *const ::core::ffi::c_char,
        interface: *const ::core::ffi::c_char,
        member: *const ::core::ffi::c_char,
        callback: sd_bus_message_handler_t,
        userdata: *mut ::core::ffi::c_void,
        types: *const ::core::ffi::c_char,
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
    fn sd_bus_error_get_errno(e: *const sd_bus_error) -> ::core::ffi::c_int;
    fn create_sni(
        id: *mut ::core::ffi::c_char,
        tray: *mut swaybar_tray,
    ) -> *mut swaybar_sni;
    fn destroy_sni(sni: *mut swaybar_sni);
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn format_str(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
}
pub type __pid_t = ::core::ffi::c_int;
pub type pid_t = __pid_t;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
pub type PangoFontDescription = _PangoFontDescription;
pub type cairo_surface_t = _cairo_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct box_colors {
    pub border: uint32_t,
    pub background: uint32_t,
    pub text: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_config {
    pub status_command: *mut ::core::ffi::c_char,
    pub pango_markup: bool,
    pub position: uint32_t,
    pub font_description: *mut PangoFontDescription,
    pub sep_symbol: *mut ::core::ffi::c_char,
    pub mode: *mut ::core::ffi::c_char,
    pub hidden_state: *mut ::core::ffi::c_char,
    pub modifier: *mut ::core::ffi::c_char,
    pub strip_workspace_numbers: bool,
    pub strip_workspace_name: bool,
    pub binding_mode_indicator: bool,
    pub wrap_scroll: bool,
    pub workspace_buttons: bool,
    pub workspace_min_width: uint32_t,
    pub bindings: *mut list_t,
    pub outputs: wl_list,
    pub height: ::core::ffi::c_int,
    pub status_padding: ::core::ffi::c_int,
    pub status_edge_padding: ::core::ffi::c_int,
    pub gaps: C2Rust_Unnamed_0,
    pub colors: C2Rust_Unnamed,
    pub icon_theme: *mut ::core::ffi::c_char,
    pub tray_bindings: wl_list,
    pub tray_hidden: bool,
    pub tray_outputs: *mut list_t,
    pub tray_padding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub background: uint32_t,
    pub statusline: uint32_t,
    pub separator: uint32_t,
    pub focused_background: uint32_t,
    pub focused_statusline: uint32_t,
    pub focused_separator: uint32_t,
    pub focused_workspace: box_colors,
    pub active_workspace: box_colors,
    pub inactive_workspace: box_colors,
    pub urgent_workspace: box_colors,
    pub binding_mode: box_colors,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub top: ::core::ffi::c_int,
    pub right: ::core::ffi::c_int,
    pub bottom: ::core::ffi::c_int,
    pub left: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar {
    pub id: *mut ::core::ffi::c_char,
    pub mode: *mut ::core::ffi::c_char,
    pub mode_pango_markup: bool,
    pub visible_by_modifier: bool,
    pub visible_by_urgency: bool,
    pub visible_by_mode: bool,
    pub visible: bool,
    pub display: *mut wl_display,
    pub compositor: *mut wl_compositor,
    pub layer_shell: *mut zwlr_layer_shell_v1,
    pub xdg_output_manager: *mut zxdg_output_manager_v1,
    pub cursor_shape_manager: *mut wp_cursor_shape_manager_v1,
    pub shm: *mut wl_shm,
    pub config: *mut swaybar_config,
    pub status: *mut status_line,
    pub eventloop: *mut r#loop,
    pub ipc_event_socketfd: ::core::ffi::c_int,
    pub ipc_socketfd: ::core::ffi::c_int,
    pub outputs: wl_list,
    pub unused_outputs: wl_list,
    pub seats: wl_list,
    pub tray: *mut swaybar_tray,
    pub running: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_tray {
    pub bar: *mut swaybar,
    pub fd: ::core::ffi::c_int,
    pub bus: *mut sd_bus,
    pub host_xdg: swaybar_host,
    pub host_kde: swaybar_host,
    pub items: *mut list_t,
    pub watcher_xdg: *mut swaybar_watcher,
    pub watcher_kde: *mut swaybar_watcher,
    pub basedirs: *mut list_t,
    pub themes: *mut list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_host {
    pub tray: *mut swaybar_tray,
    pub service: *mut ::core::ffi::c_char,
    pub watcher_interface: *mut ::core::ffi::c_char,
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
pub type sd_bus_message_handler_t = Option<
    unsafe extern "C" fn(
        *mut sd_bus_message,
        *mut ::core::ffi::c_void,
        *mut sd_bus_error,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sd_bus_error {
    pub name: *const ::core::ffi::c_char,
    pub message: *const ::core::ffi::c_char,
    pub _need_free: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_sni {
    pub tray: *mut swaybar_tray,
    pub icon: *mut cairo_surface_t,
    pub min_size: ::core::ffi::c_int,
    pub max_size: ::core::ffi::c_int,
    pub target_size: ::core::ffi::c_int,
    pub watcher_id: *mut ::core::ffi::c_char,
    pub service: *mut ::core::ffi::c_char,
    pub path: *mut ::core::ffi::c_char,
    pub interface: *mut ::core::ffi::c_char,
    pub status: *mut ::core::ffi::c_char,
    pub icon_name: *mut ::core::ffi::c_char,
    pub icon_pixmap: *mut list_t,
    pub attention_icon_name: *mut ::core::ffi::c_char,
    pub attention_icon_pixmap: *mut list_t,
    pub item_is_menu: bool,
    pub menu: *mut ::core::ffi::c_char,
    pub icon_theme_path: *mut ::core::ffi::c_char,
    pub slots: wl_list,
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut watcher_path: *const ::core::ffi::c_char = b"/StatusNotifierWatcher\0"
    .as_ptr() as *const ::core::ffi::c_char;
unsafe extern "C" fn cmp_sni_id(
    mut item: *const ::core::ffi::c_void,
    mut cmp_to: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut sni: *const swaybar_sni = item as *const swaybar_sni;
    return strcmp((*sni).watcher_id, cmp_to as *const ::core::ffi::c_char);
}
unsafe extern "C" fn add_sni(
    mut tray: *mut swaybar_tray,
    mut id: *mut ::core::ffi::c_char,
) {
    let mut idx: ::core::ffi::c_int = list_seq_find(
        (*tray).items,
        Some(
            cmp_sni_id
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        id as *const ::core::ffi::c_void,
    );
    if idx == -1 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Registering Status Notifier Item '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/host.c\0".as_ptr() as *const ::core::ffi::c_char,
            24 as ::core::ffi::c_int,
            id,
        );
        let mut sni: *mut swaybar_sni = create_sni(id, tray);
        if !sni.is_null() {
            list_add((*tray).items, sni as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn handle_sni_registered(
    mut msg: *mut sd_bus_message,
    mut data: *mut ::core::ffi::c_void,
    mut error: *mut sd_bus_error,
) -> ::core::ffi::c_int {
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut ret: ::core::ffi::c_int = sd_bus_message_read(
        msg,
        b"s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut id,
    );
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to parse register SNI message: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/host.c\0".as_ptr() as *const ::core::ffi::c_char,
            37 as ::core::ffi::c_int,
            strerror(-ret),
        );
    }
    let mut tray: *mut swaybar_tray = data as *mut swaybar_tray;
    add_sni(tray, id);
    return ret;
}
unsafe extern "C" fn handle_sni_unregistered(
    mut msg: *mut sd_bus_message,
    mut data: *mut ::core::ffi::c_void,
    mut error: *mut sd_bus_error,
) -> ::core::ffi::c_int {
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut ret: ::core::ffi::c_int = sd_bus_message_read(
        msg,
        b"s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut id,
    );
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to parse unregister SNI message: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/host.c\0".as_ptr() as *const ::core::ffi::c_char,
            51 as ::core::ffi::c_int,
            strerror(-ret),
        );
    }
    let mut tray: *mut swaybar_tray = data as *mut swaybar_tray;
    let mut idx: ::core::ffi::c_int = list_seq_find(
        (*tray).items,
        Some(
            cmp_sni_id
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        id as *const ::core::ffi::c_void,
    );
    if idx != -1 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Unregistering Status Notifier Item '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/host.c\0".as_ptr() as *const ::core::ffi::c_char,
            57 as ::core::ffi::c_int,
            id,
        );
        destroy_sni(*(*(*tray).items).items.offset(idx as isize) as *mut swaybar_sni);
        list_del((*tray).items, idx);
        set_bar_dirty((*tray).bar);
    }
    return ret;
}
unsafe extern "C" fn get_registered_snis_callback(
    mut msg: *mut sd_bus_message,
    mut data: *mut ::core::ffi::c_void,
    mut error: *mut sd_bus_error,
) -> ::core::ffi::c_int {
    if sd_bus_message_is_method_error(msg, ::core::ptr::null::<::core::ffi::c_char>())
        != 0
    {
        let mut err: *const sd_bus_error = sd_bus_message_get_error(msg);
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to get registered SNIs: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/host.c\0".as_ptr() as *const ::core::ffi::c_char,
            69 as ::core::ffi::c_int,
            (*err).message,
        );
        return -sd_bus_error_get_errno(err);
    }
    let mut ret: ::core::ffi::c_int = sd_bus_message_enter_container(
        msg,
        'v' as ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to read registered SNIs: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/host.c\0".as_ptr() as *const ::core::ffi::c_char,
            75 as ::core::ffi::c_int,
            strerror(-ret),
        );
        return ret;
    }
    let mut ids: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_char,
    >();
    ret = sd_bus_message_read_strv(msg, &raw mut ids);
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to read registered SNIs: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/host.c\0".as_ptr() as *const ::core::ffi::c_char,
            82 as ::core::ffi::c_int,
            strerror(-ret),
        );
        return ret;
    }
    if !ids.is_null() {
        let mut tray: *mut swaybar_tray = data as *mut swaybar_tray;
        let mut id: *mut *mut ::core::ffi::c_char = ids;
        while !(*id).is_null() {
            add_sni(tray, *id);
            free(*id as *mut ::core::ffi::c_void);
            id = id.offset(1);
        }
    }
    free(ids as *mut ::core::ffi::c_void);
    return ret;
}
unsafe extern "C" fn register_to_watcher(mut host: *mut swaybar_host) -> bool {
    let mut ret: ::core::ffi::c_int = sd_bus_call_method_async(
        (*(*host).tray).bus,
        ::core::ptr::null_mut::<*mut sd_bus_slot>(),
        (*host).watcher_interface,
        watcher_path,
        (*host).watcher_interface,
        b"RegisterStatusNotifierHost\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        b"s\0".as_ptr() as *const ::core::ffi::c_char,
        (*host).service,
    );
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to send register call: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/host.c\0".as_ptr() as *const ::core::ffi::c_char,
            104 as ::core::ffi::c_int,
            strerror(-ret),
        );
        return r#false != 0;
    }
    ret = sd_bus_call_method_async(
        (*(*host).tray).bus,
        ::core::ptr::null_mut::<*mut sd_bus_slot>(),
        (*host).watcher_interface,
        watcher_path,
        b"org.freedesktop.DBus.Properties\0".as_ptr() as *const ::core::ffi::c_char,
        b"Get\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            get_registered_snis_callback
                as unsafe extern "C" fn(
                    *mut sd_bus_message,
                    *mut ::core::ffi::c_void,
                    *mut sd_bus_error,
                ) -> ::core::ffi::c_int,
        ),
        (*host).tray as *mut ::core::ffi::c_void,
        b"ss\0".as_ptr() as *const ::core::ffi::c_char,
        (*host).watcher_interface,
        b"RegisteredStatusNotifierItems\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to get registered SNIs: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/host.c\0".as_ptr() as *const ::core::ffi::c_char,
            114 as ::core::ffi::c_int,
            strerror(-ret),
        );
    }
    return ret >= 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn handle_new_watcher(
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
            b"swaybar/tray/host.c\0".as_ptr() as *const ::core::ffi::c_char,
            125 as ::core::ffi::c_int,
            strerror(-ret),
        );
        return ret;
    }
    if *old_owner == 0 {
        let mut host: *mut swaybar_host = data as *mut swaybar_host;
        if strcmp(service, (*host).watcher_interface) == 0 as ::core::ffi::c_int {
            register_to_watcher(host);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn init_host(
    mut host: *mut swaybar_host,
    mut protocol: *mut ::core::ffi::c_char,
    mut tray: *mut swaybar_tray,
) -> bool {
    let mut pid: pid_t = 0;
    (*host).watcher_interface = format_str(
        b"org.%s.StatusNotifierWatcher\0".as_ptr() as *const ::core::ffi::c_char,
        protocol,
    );
    if (*host).watcher_interface.is_null() {
        return r#false != 0;
    }
    let mut reg_slot: *mut sd_bus_slot = ::core::ptr::null_mut::<sd_bus_slot>();
    let mut unreg_slot: *mut sd_bus_slot = ::core::ptr::null_mut::<sd_bus_slot>();
    let mut watcher_slot: *mut sd_bus_slot = ::core::ptr::null_mut::<sd_bus_slot>();
    let mut ret: ::core::ffi::c_int = sd_bus_match_signal(
        (*tray).bus,
        &raw mut reg_slot,
        (*host).watcher_interface,
        watcher_path,
        (*host).watcher_interface,
        b"StatusNotifierItemRegistered\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_sni_registered
                as unsafe extern "C" fn(
                    *mut sd_bus_message,
                    *mut ::core::ffi::c_void,
                    *mut sd_bus_error,
                ) -> ::core::ffi::c_int,
        ),
        tray as *mut ::core::ffi::c_void,
    );
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to subscribe to registering events: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/host.c\0".as_ptr() as *const ::core::ffi::c_char,
            151 as ::core::ffi::c_int,
            strerror(-ret),
        );
    } else {
        ret = sd_bus_match_signal(
            (*tray).bus,
            &raw mut unreg_slot,
            (*host).watcher_interface,
            watcher_path,
            (*host).watcher_interface,
            b"StatusNotifierItemUnregistered\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                handle_sni_unregistered
                    as unsafe extern "C" fn(
                        *mut sd_bus_message,
                        *mut ::core::ffi::c_void,
                        *mut sd_bus_error,
                    ) -> ::core::ffi::c_int,
            ),
            tray as *mut ::core::ffi::c_void,
        );
        if ret < 0 as ::core::ffi::c_int {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                b"[%s:%d] Failed to subscribe to unregistering events: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"swaybar/tray/host.c\0".as_ptr() as *const ::core::ffi::c_char,
                159 as ::core::ffi::c_int,
                strerror(-ret),
            );
        } else {
            ret = sd_bus_match_signal(
                (*tray).bus,
                &raw mut watcher_slot,
                b"org.freedesktop.DBus\0".as_ptr() as *const ::core::ffi::c_char,
                b"/org/freedesktop/DBus\0".as_ptr() as *const ::core::ffi::c_char,
                b"org.freedesktop.DBus\0".as_ptr() as *const ::core::ffi::c_char,
                b"NameOwnerChanged\0".as_ptr() as *const ::core::ffi::c_char,
                Some(
                    handle_new_watcher
                        as unsafe extern "C" fn(
                            *mut sd_bus_message,
                            *mut ::core::ffi::c_void,
                            *mut sd_bus_error,
                        ) -> ::core::ffi::c_int,
                ),
                host as *mut ::core::ffi::c_void,
            );
            if ret < 0 as ::core::ffi::c_int {
                _sway_log(
                    sway_log_importance_t::SWAY_ERROR,
                    b"[%s:%d] Failed to subscribe to unregistering events: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"swaybar/tray/host.c\0".as_ptr() as *const ::core::ffi::c_char,
                    168 as ::core::ffi::c_int,
                    strerror(-ret),
                );
            } else {
                pid = getpid();
                (*host).service = format_str(
                    b"org.%s.StatusNotifierHost-%d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    protocol,
                    pid,
                );
                if !(*host).service.is_null() {
                    ret = sd_bus_request_name(
                        (*tray).bus,
                        (*host).service,
                        0 as uint64_t,
                    );
                    if ret < 0 as ::core::ffi::c_int {
                        _sway_log(
                            sway_log_importance_t::SWAY_DEBUG,
                            b"[%s:%d] Failed to acquire service name: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"swaybar/tray/host.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            180 as ::core::ffi::c_int,
                            strerror(-ret),
                        );
                    } else {
                        (*host).tray = tray;
                        if register_to_watcher(host) {
                            sd_bus_slot_set_floating(reg_slot, 0 as ::core::ffi::c_int);
                            sd_bus_slot_set_floating(
                                unreg_slot,
                                0 as ::core::ffi::c_int,
                            );
                            sd_bus_slot_set_floating(
                                watcher_slot,
                                0 as ::core::ffi::c_int,
                            );
                            _sway_log(
                                sway_log_importance_t::SWAY_DEBUG,
                                b"[%s:%d] Registered %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"swaybar/tray/host.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                193 as ::core::ffi::c_int,
                                (*host).service,
                            );
                            return r#true != 0;
                        }
                    }
                }
            }
        }
    }
    sd_bus_slot_unref(reg_slot);
    sd_bus_slot_unref(unreg_slot);
    sd_bus_slot_unref(watcher_slot);
    finish_host(host);
    return r#false != 0;
}
#[no_mangle]
pub unsafe extern "C" fn finish_host(mut host: *mut swaybar_host) {
    sd_bus_release_name((*(*host).tray).bus, (*host).service);
    free((*host).service as *mut ::core::ffi::c_void);
    free((*host).watcher_interface as *mut ::core::ffi::c_void);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
