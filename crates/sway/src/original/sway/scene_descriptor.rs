extern "C" {
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn wlr_addon_init(
        addon: *mut wlr_addon,
        set: *mut wlr_addon_set,
        owner: *const ::core::ffi::c_void,
        r#impl: *const wlr_addon_interface,
    );
    fn wlr_addon_finish(addon: *mut wlr_addon);
    fn wlr_addon_find(
        set: *mut wlr_addon_set,
        owner: *const ::core::ffi::c_void,
        r#impl: *const wlr_addon_interface,
    ) -> *mut wlr_addon;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
}
pub type size_t = usize;
pub type int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_signal {
    pub listener_list: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon_set {
    pub WLR_PRIVATE: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub addons: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon {
    pub r#impl: *const wlr_addon_interface,
    pub WLR_PRIVATE: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub owner: *const ::core::ffi::c_void,
    pub link: wl_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_addon_interface {
    pub name: *const ::core::ffi::c_char,
    pub destroy: Option<unsafe extern "C" fn(*mut wlr_addon) -> ()>,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: ::core::ffi::c_long,
    pub numRects: ::core::ffi::c_long,
}
pub type pixman_region32_data_t = pixman_region32_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
pub type pixman_box32_t = pixman_box32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_t = pixman_region32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_box {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_node {
    pub r#type: wlr_scene_node_type,
    pub parent: *mut wlr_scene_tree,
    pub link: wl_list,
    pub enabled: bool,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub events: C2Rust_Unnamed_2,
    pub data: *mut ::core::ffi::c_void,
    pub addons: wlr_addon_set,
    pub WLR_PRIVATE: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub visible: pixman_region32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub destroy: wl_signal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlr_scene_tree {
    pub node: wlr_scene_node,
    pub children: wl_list,
    pub WLR_PRIVATE: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub bounding_box: wlr_box,
    pub bounding_box_valid: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wlr_scene_node_type(pub ::core::ffi::c_uint);
impl wlr_scene_node_type {
    pub const WLR_SCENE_NODE_TREE: Self = Self(0);
    pub const WLR_SCENE_NODE_RECT: Self = Self(1);
    pub const WLR_SCENE_NODE_BUFFER: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sway_scene_descriptor_type(pub ::core::ffi::c_uint);
impl sway_scene_descriptor_type {
    pub const SWAY_SCENE_DESC_BUFFER_TIMER: Self = Self(0);
    pub const SWAY_SCENE_DESC_NON_INTERACTIVE: Self = Self(1);
    pub const SWAY_SCENE_DESC_CONTAINER: Self = Self(2);
    pub const SWAY_SCENE_DESC_VIEW: Self = Self(3);
    pub const SWAY_SCENE_DESC_LAYER_SHELL: Self = Self(4);
    pub const SWAY_SCENE_DESC_XWAYLAND_UNMANAGED: Self = Self(5);
    pub const SWAY_SCENE_DESC_POPUP: Self = Self(6);
    pub const SWAY_SCENE_DESC_DRAG_ICON: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scene_descriptor {
    pub data: *mut ::core::ffi::c_void,
    pub addon: wlr_addon,
}
unsafe extern "C" fn scene_node_get_descriptor(
    mut node: *mut wlr_scene_node,
    mut r#type: sway_scene_descriptor_type,
) -> *mut scene_descriptor {
    let mut addon: *mut wlr_addon = wlr_addon_find(
        &raw mut (*node).addons,
        ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(r#type.0 as usize),
        &raw const addon_interface,
    );
    if addon.is_null() {
        return ::core::ptr::null_mut::<scene_descriptor>();
    }
    let mut desc: *mut scene_descriptor = (addon as *mut ::core::ffi::c_char)
        .offset(-(8 as ::core::ffi::c_ulong as isize)) as *mut scene_descriptor;
    return desc;
}
unsafe extern "C" fn descriptor_destroy(mut desc: *mut scene_descriptor) {
    wlr_addon_finish(&raw mut (*desc).addon);
    free(desc as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn scene_descriptor_try_get(
    mut node: *mut wlr_scene_node,
    mut r#type: sway_scene_descriptor_type,
) -> *mut ::core::ffi::c_void {
    let mut desc: *mut scene_descriptor = scene_node_get_descriptor(node, r#type);
    if desc.is_null() {
        return NULL;
    }
    return (*desc).data;
}
#[no_mangle]
pub unsafe extern "C" fn scene_descriptor_destroy(
    mut node: *mut wlr_scene_node,
    mut r#type: sway_scene_descriptor_type,
) {
    let mut desc: *mut scene_descriptor = scene_node_get_descriptor(node, r#type);
    if desc.is_null() {
        return;
    }
    descriptor_destroy(desc);
}
unsafe extern "C" fn addon_handle_destroy(mut addon: *mut wlr_addon) {
    let mut desc: *mut scene_descriptor = (addon as *mut ::core::ffi::c_char)
        .offset(-(8 as ::core::ffi::c_ulong as isize)) as *mut scene_descriptor;
    descriptor_destroy(desc);
}
static mut addon_interface: wlr_addon_interface = wlr_addon_interface {
    name: b"sway_scene_descriptor\0".as_ptr() as *const ::core::ffi::c_char,
    destroy: Some(addon_handle_destroy as unsafe extern "C" fn(*mut wlr_addon) -> ()),
};
#[no_mangle]
pub unsafe extern "C" fn scene_descriptor_assign(
    mut node: *mut wlr_scene_node,
    mut r#type: sway_scene_descriptor_type,
    mut data: *mut ::core::ffi::c_void,
) -> bool {
    let mut desc: *mut scene_descriptor = calloc(
        1 as size_t,
        ::core::mem::size_of::<scene_descriptor>(),
    ) as *mut scene_descriptor;
    if desc.is_null() {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Could not allocate a scene descriptor\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"sway/scene_descriptor.c\0".as_ptr() as *const ::core::ffi::c_char,
            62 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    wlr_addon_init(
        &raw mut (*desc).addon,
        &raw mut (*node).addons,
        ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(r#type.0 as usize),
        &raw const addon_interface,
    );
    (*desc).data = data;
    return r#true != 0;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
