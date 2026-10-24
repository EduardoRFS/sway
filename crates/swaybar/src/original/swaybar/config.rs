extern "C" {
    pub type _PangoFontDescription;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn wl_list_init(list: *mut wl_list);
    fn wl_list_remove(elm: *mut wl_list);
    fn create_list() -> *mut list_t;
    fn list_free(list: *mut list_t);
    fn list_free_items_and_destroy(list: *mut list_t);
    fn pango_font_description_free(desc: *mut PangoFontDescription);
    fn pango_font_description_from_string(
        str: *const ::core::ffi::c_char,
    ) -> *mut PangoFontDescription;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
}
pub type size_t = usize;
pub type uint32_t = u32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct box_colors {
    pub border: uint32_t,
    pub background: uint32_t,
    pub text: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_output {
    pub link: wl_list,
    pub name: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swaybar_binding {
    pub button: uint32_t,
    pub command: *mut ::core::ffi::c_char,
    pub release: bool,
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
pub struct tray_binding {
    pub button: uint32_t,
    pub command: *mut ::core::ffi::c_char,
    pub link: wl_list,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct zwlr_layer_surface_v1_anchor(pub ::core::ffi::c_uint);
impl zwlr_layer_surface_v1_anchor {
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_TOP: Self = Self(1);
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_BOTTOM: Self = Self(2);
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_LEFT: Self = Self(4);
    pub const ZWLR_LAYER_SURFACE_V1_ANCHOR_RIGHT: Self = Self(8);
}
#[no_mangle]
pub unsafe extern "C" fn parse_position(
    mut position: *const ::core::ffi::c_char,
) -> uint32_t {
    let mut horiz: uint32_t = (zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_LEFT
        .0 as ::core::ffi::c_int
        | zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_RIGHT.0
            as ::core::ffi::c_int) as uint32_t;
    if strcmp(b"top\0".as_ptr() as *const ::core::ffi::c_char, position)
        == 0 as ::core::ffi::c_int
    {
        return zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_TOP.0
            as ::core::ffi::c_int as uint32_t | horiz
    } else if strcmp(b"bottom\0".as_ptr() as *const ::core::ffi::c_char, position)
        == 0 as ::core::ffi::c_int
    {
        return zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_BOTTOM.0
            as ::core::ffi::c_int as uint32_t | horiz
    } else {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Invalid position: %s, defaulting to bottom\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/config.c\0".as_ptr() as *const ::core::ffi::c_char,
            18 as ::core::ffi::c_int,
            position,
        );
        return zwlr_layer_surface_v1_anchor::ZWLR_LAYER_SURFACE_V1_ANCHOR_BOTTOM.0
            as ::core::ffi::c_int as uint32_t | horiz;
    };
}
#[no_mangle]
pub unsafe extern "C" fn init_config() -> *mut swaybar_config {
    let mut config: *mut swaybar_config = calloc(
        1 as size_t,
        ::core::mem::size_of::<swaybar_config>(),
    ) as *mut swaybar_config;
    (*config).status_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*config).pango_markup = r#false != 0;
    (*config).position = parse_position(
        b"bottom\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*config).font_description = pango_font_description_from_string(
        b"monospace 10\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*config).mode = strdup(b"dock\0".as_ptr() as *const ::core::ffi::c_char);
    (*config).hidden_state = strdup(b"hide\0".as_ptr() as *const ::core::ffi::c_char);
    (*config).sep_symbol = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*config).strip_workspace_numbers = r#false != 0;
    (*config).strip_workspace_name = r#false != 0;
    (*config).binding_mode_indicator = r#true != 0;
    (*config).wrap_scroll = r#false != 0;
    (*config).workspace_buttons = r#true != 0;
    (*config).workspace_min_width = 0 as uint32_t;
    (*config).bindings = create_list();
    wl_list_init(&raw mut (*config).outputs);
    (*config).status_padding = 1 as ::core::ffi::c_int;
    (*config).status_edge_padding = 3 as ::core::ffi::c_int;
    (*config).height = 0 as ::core::ffi::c_int;
    (*config).gaps.top = 0 as ::core::ffi::c_int;
    (*config).gaps.right = 0 as ::core::ffi::c_int;
    (*config).gaps.bottom = 0 as ::core::ffi::c_int;
    (*config).gaps.left = 0 as ::core::ffi::c_int;
    (*config).colors.background = 0xff as uint32_t;
    (*config).colors.focused_background = 0xff as uint32_t;
    (*config).colors.statusline = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    (*config).colors.focused_statusline = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    (*config).colors.separator = 0x666666ff as uint32_t;
    (*config).colors.focused_workspace.border = 0x4c7899ff as uint32_t;
    (*config).colors.focused_workspace.background = 0x285577ff as uint32_t;
    (*config).colors.focused_workspace.text = 0xffffffff as ::core::ffi::c_uint
        as uint32_t;
    (*config).colors.active_workspace.border = 0x333333ff as uint32_t;
    (*config).colors.active_workspace.background = 0x5f676aff as uint32_t;
    (*config).colors.active_workspace.text = 0xffffffff as ::core::ffi::c_uint
        as uint32_t;
    (*config).colors.inactive_workspace.border = 0x333333ff as uint32_t;
    (*config).colors.inactive_workspace.background = 0x222222ff as uint32_t;
    (*config).colors.inactive_workspace.text = 0x888888ff as ::core::ffi::c_uint
        as uint32_t;
    (*config).colors.urgent_workspace.border = 0x2f343aff as uint32_t;
    (*config).colors.urgent_workspace.background = 0x900000ff as ::core::ffi::c_uint
        as uint32_t;
    (*config).colors.urgent_workspace.text = 0xffffffff as ::core::ffi::c_uint
        as uint32_t;
    (*config).colors.binding_mode.border = 0x2f343aff as uint32_t;
    (*config).colors.binding_mode.background = 0x900000ff as ::core::ffi::c_uint
        as uint32_t;
    (*config).colors.binding_mode.text = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    (*config).tray_padding = 2 as ::core::ffi::c_int;
    wl_list_init(&raw mut (*config).tray_bindings);
    return config;
}
#[no_mangle]
pub unsafe extern "C" fn free_binding(mut binding: *mut swaybar_binding) {
    if binding.is_null() {
        return;
    }
    free((*binding).command as *mut ::core::ffi::c_void);
    free(binding as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn free_tray_binding(mut binding: *mut tray_binding) {
    if binding.is_null() {
        return;
    }
    free((*binding).command as *mut ::core::ffi::c_void);
    free(binding as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn free_config(mut config: *mut swaybar_config) {
    free((*config).status_command as *mut ::core::ffi::c_void);
    pango_font_description_free((*config).font_description);
    free((*config).mode as *mut ::core::ffi::c_void);
    free((*config).hidden_state as *mut ::core::ffi::c_void);
    free((*config).sep_symbol as *mut ::core::ffi::c_void);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*config).bindings).length {
        let mut binding: *mut swaybar_binding = *(*(*config).bindings)
            .items
            .offset(i as isize) as *mut swaybar_binding;
        free_binding(binding);
        i += 1;
    }
    list_free((*config).bindings);
    let mut coutput: *mut config_output = ::core::ptr::null_mut::<config_output>();
    let mut tmp: *mut config_output = ::core::ptr::null_mut::<config_output>();
    coutput = ((*config).outputs.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
        as *mut config_output;
    tmp = ((*coutput).link.next as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
        as *mut config_output;
    while &raw mut (*coutput).link != &raw mut (*config).outputs {
        wl_list_remove(&raw mut (*coutput).link);
        free((*coutput).name as *mut ::core::ffi::c_void);
        free(coutput as *mut ::core::ffi::c_void);
        coutput = tmp;
        tmp = ((*coutput).link.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut config_output
            as *mut config_output;
    }
    list_free_items_and_destroy((*config).tray_outputs);
    let mut tray_bind: *mut tray_binding = ::core::ptr::null_mut::<tray_binding>();
    let mut tmp_tray_bind: *mut tray_binding = ::core::ptr::null_mut::<tray_binding>();
    tray_bind = ((*config).tray_bindings.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut tray_binding
        as *mut tray_binding;
    tmp_tray_bind = ((*tray_bind).link.next as *mut ::core::ffi::c_char)
        .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut tray_binding
        as *mut tray_binding;
    while &raw mut (*tray_bind).link != &raw mut (*config).tray_bindings {
        wl_list_remove(&raw mut (*tray_bind).link);
        free_tray_binding(tray_bind);
        tray_bind = tmp_tray_bind;
        tmp_tray_bind = ((*tray_bind).link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut tray_binding
            as *mut tray_binding;
    }
    free((*config).icon_theme as *mut ::core::ffi::c_void);
    free(config as *mut ::core::ffi::c_void);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
