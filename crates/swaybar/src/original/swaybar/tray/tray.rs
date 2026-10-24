extern "C" {
    pub type _cairo;
    pub type _cairo_surface;
    pub type wl_display;
    pub type wl_buffer;
    pub type wl_compositor;
    pub type wl_output;
    pub type wl_shm;
    pub type wl_surface;
    pub type _PangoContext;
    pub type _PangoFontDescription;
    pub type sd_bus;
    pub type r#loop;
    pub type status_line;
    pub type wp_cursor_shape_manager_v1;
    pub type zxdg_output_manager_v1;
    pub type zwlr_layer_shell_v1;
    pub type zwlr_layer_surface_v1;
    pub type zxdg_output_v1;
    pub type sd_bus_message;
    pub type sd_bus_slot;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn create_list() -> *mut list_t;
    fn list_free(list: *mut list_t);
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
    fn create_watcher(
        protocol: *mut ::core::ffi::c_char,
        bus: *mut sd_bus,
    ) -> *mut swaybar_watcher;
    fn destroy_watcher(watcher: *mut swaybar_watcher);
    fn init_host(
        host: *mut swaybar_host,
        protocol: *mut ::core::ffi::c_char,
        tray: *mut swaybar_tray,
    ) -> bool;
    fn finish_host(host: *mut swaybar_host);
    fn init_themes(themes: *mut *mut list_t, basedirs: *mut *mut list_t);
    fn finish_themes(themes: *mut list_t, basedirs: *mut list_t);
    fn sd_bus_open_user(ret: *mut *mut sd_bus) -> ::core::ffi::c_int;
    fn sd_bus_flush_close_unref(bus: *mut sd_bus) -> *mut sd_bus;
    fn sd_bus_get_fd(bus: *mut sd_bus) -> ::core::ffi::c_int;
    fn sd_bus_process(
        bus: *mut sd_bus,
        ret: *mut *mut sd_bus_message,
    ) -> ::core::ffi::c_int;
    fn sd_bus_message_read(
        m: *mut sd_bus_message,
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
    fn destroy_sni(sni: *mut swaybar_sni);
    fn render_sni(
        cairo: *mut cairo_t,
        output: *mut swaybar_output,
        x: *mut ::core::ffi::c_double,
        sni: *mut swaybar_sni,
    ) -> uint32_t;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
}
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_output_subpixel(pub ::core::ffi::c_uint);
impl wl_output_subpixel {
    pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: Self = Self(0);
    pub const WL_OUTPUT_SUBPIXEL_NONE: Self = Self(1);
    pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: Self = Self(2);
    pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: Self = Self(3);
    pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: Self = Self(4);
    pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
pub type PangoContext = _PangoContext;
pub type PangoFontDescription = _PangoFontDescription;
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
pub struct swaybar_watcher {
    pub interface: *mut ::core::ffi::c_char,
    pub bus: *mut sd_bus,
    pub hosts: *mut list_t,
    pub items: *mut list_t,
    pub version: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_host {
    pub tray: *mut swaybar_tray,
    pub service: *mut ::core::ffi::c_char,
    pub watcher_interface: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_output {
    pub link: wl_list,
    pub bar: *mut swaybar,
    pub output: *mut wl_output,
    pub xdg_output: *mut zxdg_output_v1,
    pub surface: *mut wl_surface,
    pub layer_surface: *mut zwlr_layer_surface_v1,
    pub wl_name: uint32_t,
    pub workspaces: wl_list,
    pub hotspots: wl_list,
    pub name: *mut ::core::ffi::c_char,
    pub identifier: *mut ::core::ffi::c_char,
    pub focused: bool,
    pub width: uint32_t,
    pub height: uint32_t,
    pub scale: int32_t,
    pub subpixel: wl_output_subpixel,
    pub buffers: [pool_buffer; 2],
    pub current_buffer: *mut pool_buffer,
    pub dirty: bool,
    pub frame_scheduled: bool,
    pub output_height: uint32_t,
    pub output_width: uint32_t,
    pub output_x: uint32_t,
    pub output_y: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool_buffer {
    pub buffer: *mut wl_buffer,
    pub surface: *mut cairo_surface_t,
    pub cairo: *mut cairo_t,
    pub pango: *mut PangoContext,
    pub width: uint32_t,
    pub height: uint32_t,
    pub data: *mut ::core::ffi::c_void,
    pub size: size_t,
    pub busy: bool,
}
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
unsafe extern "C" fn handle_lost_watcher(
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
            b"swaybar/tray/tray.c\0".as_ptr() as *const ::core::ffi::c_char,
            21 as ::core::ffi::c_int,
            strerror(-ret),
        );
        return ret;
    }
    if *new_owner == 0 {
        let mut tray: *mut swaybar_tray = data as *mut swaybar_tray;
        if strcmp(
            service,
            b"org.freedesktop.StatusNotifierWatcher\0".as_ptr()
                as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*tray).watcher_xdg = create_watcher(
                b"freedesktop\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                (*tray).bus,
            );
        } else if strcmp(
            service,
            b"org.kde.StatusNotifierWatcher\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*tray).watcher_kde = create_watcher(
                b"kde\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                (*tray).bus,
            );
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn create_tray(mut bar: *mut swaybar) -> *mut swaybar_tray {
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Initializing tray\0".as_ptr() as *const ::core::ffi::c_char,
        b"swaybar/tray/tray.c\0".as_ptr() as *const ::core::ffi::c_char,
        38 as ::core::ffi::c_int,
    );
    let mut bus: *mut sd_bus = ::core::ptr::null_mut::<sd_bus>();
    let mut ret: ::core::ffi::c_int = sd_bus_open_user(&raw mut bus);
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to connect to user bus: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/tray.c\0".as_ptr() as *const ::core::ffi::c_char,
            43 as ::core::ffi::c_int,
            strerror(-ret),
        );
        return ::core::ptr::null_mut::<swaybar_tray>();
    }
    let mut tray: *mut swaybar_tray = calloc(
        1 as size_t,
        ::core::mem::size_of::<swaybar_tray>(),
    ) as *mut swaybar_tray;
    if tray.is_null() {
        return ::core::ptr::null_mut::<swaybar_tray>();
    }
    (*tray).bar = bar;
    (*tray).bus = bus;
    (*tray).fd = sd_bus_get_fd((*tray).bus);
    (*tray).watcher_xdg = create_watcher(
        b"freedesktop\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        (*tray).bus,
    );
    (*tray).watcher_kde = create_watcher(
        b"kde\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*tray).bus,
    );
    ret = sd_bus_match_signal(
        bus,
        ::core::ptr::null_mut::<*mut sd_bus_slot>(),
        b"org.freedesktop.DBus\0".as_ptr() as *const ::core::ffi::c_char,
        b"/org/freedesktop/DBus\0".as_ptr() as *const ::core::ffi::c_char,
        b"org.freedesktop.DBus\0".as_ptr() as *const ::core::ffi::c_char,
        b"NameOwnerChanged\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            handle_lost_watcher
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
            b"swaybar/tray/tray.c\0".as_ptr() as *const ::core::ffi::c_char,
            62 as ::core::ffi::c_int,
            strerror(-ret),
        );
    }
    (*tray).items = create_list();
    init_host(
        &raw mut (*tray).host_xdg,
        b"freedesktop\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        tray,
    );
    init_host(
        &raw mut (*tray).host_kde,
        b"kde\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        tray,
    );
    init_themes(&raw mut (*tray).themes, &raw mut (*tray).basedirs);
    return tray;
}
#[no_mangle]
pub unsafe extern "C" fn destroy_tray(mut tray: *mut swaybar_tray) {
    if tray.is_null() {
        return;
    }
    finish_host(&raw mut (*tray).host_xdg);
    finish_host(&raw mut (*tray).host_kde);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*tray).items).length {
        destroy_sni(*(*(*tray).items).items.offset(i as isize) as *mut swaybar_sni);
        i += 1;
    }
    list_free((*tray).items);
    destroy_watcher((*tray).watcher_xdg);
    destroy_watcher((*tray).watcher_kde);
    sd_bus_flush_close_unref((*tray).bus);
    finish_themes((*tray).themes, (*tray).basedirs);
    free(tray as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tray_in(
    mut fd: ::core::ffi::c_int,
    mut mask: ::core::ffi::c_short,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut bar: *mut swaybar = data as *mut swaybar;
    let mut ret: ::core::ffi::c_int = 0;
    if mask as ::core::ffi::c_int & (POLLHUP | POLLERR) != 0 {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] D-Bus connection closed unexpectedly\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/tray.c\0".as_ptr() as *const ::core::ffi::c_char,
            98 as ::core::ffi::c_int,
        );
        (*bar).running = r#false != 0;
        return;
    }
    loop {
        ret = sd_bus_process(
            (*(*bar).tray).bus,
            ::core::ptr::null_mut::<*mut sd_bus_message>(),
        );
        if ret <= 0 as ::core::ffi::c_int {
            break;
        }
    }
    if ret < 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to process bus: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/tray.c\0".as_ptr() as *const ::core::ffi::c_char,
            107 as ::core::ffi::c_int,
            strerror(-ret),
        );
    }
}
unsafe extern "C" fn cmp_output(
    mut item: *const ::core::ffi::c_void,
    mut cmp_to: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut output: *const swaybar_output = cmp_to as *const swaybar_output;
    if !(*output).identifier.is_null()
        && strcmp(item as *const ::core::ffi::c_char, (*output).identifier)
            == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    return strcmp(item as *const ::core::ffi::c_char, (*output).name);
}
#[no_mangle]
pub unsafe extern "C" fn render_tray(
    mut cairo: *mut cairo_t,
    mut output: *mut swaybar_output,
    mut x: *mut ::core::ffi::c_double,
) -> uint32_t {
    let mut config: *mut swaybar_config = (*(*output).bar).config;
    if !(*config).tray_outputs.is_null() {
        if list_seq_find(
            (*config).tray_outputs,
            Some(
                cmp_output
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            output as *const ::core::ffi::c_void,
        ) == -1 as ::core::ffi::c_int
        {
            return 0 as uint32_t;
        }
    }
    if (*output).height.wrapping_mul((*output).scale as uint32_t) as ::core::ffi::c_int
        <= 2 as ::core::ffi::c_int * (*config).tray_padding
    {
        return ((2 as int32_t * (*config).tray_padding as int32_t + 1 as int32_t)
            / (*output).scale) as uint32_t;
    }
    let mut max_height: uint32_t = 0 as uint32_t;
    let mut tray: *mut swaybar_tray = (*(*output).bar).tray;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*tray).items).length {
        let mut h: uint32_t = render_sni(
            cairo,
            output,
            x,
            *(*(*tray).items).items.offset(i as isize) as *mut swaybar_sni,
        );
        if h > max_height {
            max_height = h;
        }
        i += 1;
    }
    return max_height;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
