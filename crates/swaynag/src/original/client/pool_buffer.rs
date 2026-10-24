extern "C" {
    pub type _cairo;
    pub type _cairo_surface;
    pub type _PangoContext;
    pub type wl_proxy;
    pub type wl_buffer;
    pub type wl_shm;
    pub type wl_shm_pool;
    fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    fn cairo_destroy(cr: *mut cairo_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_image_surface_create_for_data(
        data: *mut ::core::ffi::c_uchar,
        format: cairo_format_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        stride: ::core::ffi::c_int,
    ) -> *mut cairo_surface_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_object_unref(object: gpointer);
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn pango_cairo_create_context(cr: *mut cairo_t) -> *mut PangoContext;
    fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
    ) -> *mut ::core::ffi::c_void;
    fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
    fn shm_open(
        __name: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        __mode: mode_t,
    ) -> ::core::ffi::c_int;
    fn shm_unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getpid() -> __pid_t;
    fn ftruncate(__fd: ::core::ffi::c_int, __length: __off64_t) -> ::core::ffi::c_int;
    fn wl_proxy_marshal_flags(
        proxy: *mut wl_proxy,
        opcode: uint32_t,
        interface: *const wl_interface,
        version: uint32_t,
        flags: uint32_t,
        ...
    ) -> *mut wl_proxy;
    fn wl_proxy_add_listener(
        proxy: *mut wl_proxy,
        implementation: *mut Option<unsafe extern "C" fn() -> ()>,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn wl_proxy_get_version(proxy: *mut wl_proxy) -> uint32_t;
    static wl_shm_pool_interface: wl_interface;
    static wl_buffer_interface: wl_interface;
}
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct _cairo_format(pub ::core::ffi::c_int);
impl _cairo_format {
    pub const CAIRO_FORMAT_INVALID: Self = Self(-1);
    pub const CAIRO_FORMAT_ARGB32: Self = Self(0);
    pub const CAIRO_FORMAT_RGB24: Self = Self(1);
    pub const CAIRO_FORMAT_A8: Self = Self(2);
    pub const CAIRO_FORMAT_A1: Self = Self(3);
    pub const CAIRO_FORMAT_RGB16_565: Self = Self(4);
    pub const CAIRO_FORMAT_RGB30: Self = Self(5);
    pub const CAIRO_FORMAT_RGB96F: Self = Self(6);
    pub const CAIRO_FORMAT_RGBA128F: Self = Self(7);
}
pub type cairo_format_t = _cairo_format;
pub type __mode_t = ::core::ffi::c_uint;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type size_t = usize;
pub type clockid_t = __clockid_t;
pub type gpointer = *mut ::core::ffi::c_void;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type PangoContext = _PangoContext;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_message {
    pub name: *const ::core::ffi::c_char,
    pub signature: *const ::core::ffi::c_char,
    pub types: *mut *const wl_interface,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct wl_shm_format(pub ::core::ffi::c_uint);
impl wl_shm_format {
    pub const WL_SHM_FORMAT_ARGB8888: Self = Self(0);
    pub const WL_SHM_FORMAT_XRGB8888: Self = Self(1);
    pub const WL_SHM_FORMAT_C8: Self = Self(538982467);
    pub const WL_SHM_FORMAT_RGB332: Self = Self(943867730);
    pub const WL_SHM_FORMAT_BGR233: Self = Self(944916290);
    pub const WL_SHM_FORMAT_XRGB4444: Self = Self(842093144);
    pub const WL_SHM_FORMAT_XBGR4444: Self = Self(842089048);
    pub const WL_SHM_FORMAT_RGBX4444: Self = Self(842094674);
    pub const WL_SHM_FORMAT_BGRX4444: Self = Self(842094658);
    pub const WL_SHM_FORMAT_ARGB4444: Self = Self(842093121);
    pub const WL_SHM_FORMAT_ABGR4444: Self = Self(842089025);
    pub const WL_SHM_FORMAT_RGBA4444: Self = Self(842088786);
    pub const WL_SHM_FORMAT_BGRA4444: Self = Self(842088770);
    pub const WL_SHM_FORMAT_XRGB1555: Self = Self(892424792);
    pub const WL_SHM_FORMAT_XBGR1555: Self = Self(892420696);
    pub const WL_SHM_FORMAT_RGBX5551: Self = Self(892426322);
    pub const WL_SHM_FORMAT_BGRX5551: Self = Self(892426306);
    pub const WL_SHM_FORMAT_ARGB1555: Self = Self(892424769);
    pub const WL_SHM_FORMAT_ABGR1555: Self = Self(892420673);
    pub const WL_SHM_FORMAT_RGBA5551: Self = Self(892420434);
    pub const WL_SHM_FORMAT_BGRA5551: Self = Self(892420418);
    pub const WL_SHM_FORMAT_RGB565: Self = Self(909199186);
    pub const WL_SHM_FORMAT_BGR565: Self = Self(909199170);
    pub const WL_SHM_FORMAT_RGB888: Self = Self(875710290);
    pub const WL_SHM_FORMAT_BGR888: Self = Self(875710274);
    pub const WL_SHM_FORMAT_XBGR8888: Self = Self(875709016);
    pub const WL_SHM_FORMAT_RGBX8888: Self = Self(875714642);
    pub const WL_SHM_FORMAT_BGRX8888: Self = Self(875714626);
    pub const WL_SHM_FORMAT_ABGR8888: Self = Self(875708993);
    pub const WL_SHM_FORMAT_RGBA8888: Self = Self(875708754);
    pub const WL_SHM_FORMAT_BGRA8888: Self = Self(875708738);
    pub const WL_SHM_FORMAT_XRGB2101010: Self = Self(808669784);
    pub const WL_SHM_FORMAT_XBGR2101010: Self = Self(808665688);
    pub const WL_SHM_FORMAT_RGBX1010102: Self = Self(808671314);
    pub const WL_SHM_FORMAT_BGRX1010102: Self = Self(808671298);
    pub const WL_SHM_FORMAT_ARGB2101010: Self = Self(808669761);
    pub const WL_SHM_FORMAT_ABGR2101010: Self = Self(808665665);
    pub const WL_SHM_FORMAT_RGBA1010102: Self = Self(808665426);
    pub const WL_SHM_FORMAT_BGRA1010102: Self = Self(808665410);
    pub const WL_SHM_FORMAT_YUYV: Self = Self(1448695129);
    pub const WL_SHM_FORMAT_YVYU: Self = Self(1431918169);
    pub const WL_SHM_FORMAT_UYVY: Self = Self(1498831189);
    pub const WL_SHM_FORMAT_VYUY: Self = Self(1498765654);
    pub const WL_SHM_FORMAT_AYUV: Self = Self(1448433985);
    pub const WL_SHM_FORMAT_NV12: Self = Self(842094158);
    pub const WL_SHM_FORMAT_NV21: Self = Self(825382478);
    pub const WL_SHM_FORMAT_NV16: Self = Self(909203022);
    pub const WL_SHM_FORMAT_NV61: Self = Self(825644622);
    pub const WL_SHM_FORMAT_YUV410: Self = Self(961959257);
    pub const WL_SHM_FORMAT_YVU410: Self = Self(961893977);
    pub const WL_SHM_FORMAT_YUV411: Self = Self(825316697);
    pub const WL_SHM_FORMAT_YVU411: Self = Self(825316953);
    pub const WL_SHM_FORMAT_YUV420: Self = Self(842093913);
    pub const WL_SHM_FORMAT_YVU420: Self = Self(842094169);
    pub const WL_SHM_FORMAT_YUV422: Self = Self(909202777);
    pub const WL_SHM_FORMAT_YVU422: Self = Self(909203033);
    pub const WL_SHM_FORMAT_YUV444: Self = Self(875713881);
    pub const WL_SHM_FORMAT_YVU444: Self = Self(875714137);
    pub const WL_SHM_FORMAT_R8: Self = Self(538982482);
    pub const WL_SHM_FORMAT_R16: Self = Self(540422482);
    pub const WL_SHM_FORMAT_RG88: Self = Self(943212370);
    pub const WL_SHM_FORMAT_GR88: Self = Self(943215175);
    pub const WL_SHM_FORMAT_RG1616: Self = Self(842221394);
    pub const WL_SHM_FORMAT_GR1616: Self = Self(842224199);
    pub const WL_SHM_FORMAT_XRGB16161616F: Self = Self(1211388504);
    pub const WL_SHM_FORMAT_XBGR16161616F: Self = Self(1211384408);
    pub const WL_SHM_FORMAT_ARGB16161616F: Self = Self(1211388481);
    pub const WL_SHM_FORMAT_ABGR16161616F: Self = Self(1211384385);
    pub const WL_SHM_FORMAT_XYUV8888: Self = Self(1448434008);
    pub const WL_SHM_FORMAT_VUY888: Self = Self(875713878);
    pub const WL_SHM_FORMAT_VUY101010: Self = Self(808670550);
    pub const WL_SHM_FORMAT_Y210: Self = Self(808530521);
    pub const WL_SHM_FORMAT_Y212: Self = Self(842084953);
    pub const WL_SHM_FORMAT_Y216: Self = Self(909193817);
    pub const WL_SHM_FORMAT_Y410: Self = Self(808531033);
    pub const WL_SHM_FORMAT_Y412: Self = Self(842085465);
    pub const WL_SHM_FORMAT_Y416: Self = Self(909194329);
    pub const WL_SHM_FORMAT_XVYU2101010: Self = Self(808670808);
    pub const WL_SHM_FORMAT_XVYU12_16161616: Self = Self(909334104);
    pub const WL_SHM_FORMAT_XVYU16161616: Self = Self(942954072);
    pub const WL_SHM_FORMAT_Y0L0: Self = Self(810299481);
    pub const WL_SHM_FORMAT_X0L0: Self = Self(810299480);
    pub const WL_SHM_FORMAT_Y0L2: Self = Self(843853913);
    pub const WL_SHM_FORMAT_X0L2: Self = Self(843853912);
    pub const WL_SHM_FORMAT_YUV420_8BIT: Self = Self(942691673);
    pub const WL_SHM_FORMAT_YUV420_10BIT: Self = Self(808539481);
    pub const WL_SHM_FORMAT_XRGB8888_A8: Self = Self(943805016);
    pub const WL_SHM_FORMAT_XBGR8888_A8: Self = Self(943800920);
    pub const WL_SHM_FORMAT_RGBX8888_A8: Self = Self(943806546);
    pub const WL_SHM_FORMAT_BGRX8888_A8: Self = Self(943806530);
    pub const WL_SHM_FORMAT_RGB888_A8: Self = Self(943798354);
    pub const WL_SHM_FORMAT_BGR888_A8: Self = Self(943798338);
    pub const WL_SHM_FORMAT_RGB565_A8: Self = Self(943797586);
    pub const WL_SHM_FORMAT_BGR565_A8: Self = Self(943797570);
    pub const WL_SHM_FORMAT_NV24: Self = Self(875714126);
    pub const WL_SHM_FORMAT_NV42: Self = Self(842290766);
    pub const WL_SHM_FORMAT_P210: Self = Self(808530512);
    pub const WL_SHM_FORMAT_P010: Self = Self(808530000);
    pub const WL_SHM_FORMAT_P012: Self = Self(842084432);
    pub const WL_SHM_FORMAT_P016: Self = Self(909193296);
    pub const WL_SHM_FORMAT_AXBXGXRX106106106106: Self = Self(808534593);
    pub const WL_SHM_FORMAT_NV15: Self = Self(892425806);
    pub const WL_SHM_FORMAT_Q410: Self = Self(808531025);
    pub const WL_SHM_FORMAT_Q401: Self = Self(825242705);
    pub const WL_SHM_FORMAT_XRGB16161616: Self = Self(942953048);
    pub const WL_SHM_FORMAT_XBGR16161616: Self = Self(942948952);
    pub const WL_SHM_FORMAT_ARGB16161616: Self = Self(942953025);
    pub const WL_SHM_FORMAT_ABGR16161616: Self = Self(942948929);
    pub const WL_SHM_FORMAT_C1: Self = Self(538980675);
    pub const WL_SHM_FORMAT_C2: Self = Self(538980931);
    pub const WL_SHM_FORMAT_C4: Self = Self(538981443);
    pub const WL_SHM_FORMAT_D1: Self = Self(538980676);
    pub const WL_SHM_FORMAT_D2: Self = Self(538980932);
    pub const WL_SHM_FORMAT_D4: Self = Self(538981444);
    pub const WL_SHM_FORMAT_D8: Self = Self(538982468);
    pub const WL_SHM_FORMAT_R1: Self = Self(538980690);
    pub const WL_SHM_FORMAT_R2: Self = Self(538980946);
    pub const WL_SHM_FORMAT_R4: Self = Self(538981458);
    pub const WL_SHM_FORMAT_R10: Self = Self(540029266);
    pub const WL_SHM_FORMAT_R12: Self = Self(540160338);
    pub const WL_SHM_FORMAT_AVUY8888: Self = Self(1498764865);
    pub const WL_SHM_FORMAT_XVUY8888: Self = Self(1498764888);
    pub const WL_SHM_FORMAT_P030: Self = Self(808661072);
    pub const WL_SHM_FORMAT_RGB161616: Self = Self(942950226);
    pub const WL_SHM_FORMAT_BGR161616: Self = Self(942950210);
    pub const WL_SHM_FORMAT_R16F: Self = Self(1210064978);
    pub const WL_SHM_FORMAT_GR1616F: Self = Self(1210077767);
    pub const WL_SHM_FORMAT_BGR161616F: Self = Self(1213351746);
    pub const WL_SHM_FORMAT_R32F: Self = Self(1176510546);
    pub const WL_SHM_FORMAT_GR3232F: Self = Self(1176523335);
    pub const WL_SHM_FORMAT_BGR323232F: Self = Self(1179797314);
    pub const WL_SHM_FORMAT_ABGR32323232F: Self = Self(1178092097);
    pub const WL_SHM_FORMAT_NV20: Self = Self(808605262);
    pub const WL_SHM_FORMAT_NV30: Self = Self(808670798);
    pub const WL_SHM_FORMAT_S010: Self = Self(808530003);
    pub const WL_SHM_FORMAT_S210: Self = Self(808530515);
    pub const WL_SHM_FORMAT_S410: Self = Self(808531027);
    pub const WL_SHM_FORMAT_S012: Self = Self(842084435);
    pub const WL_SHM_FORMAT_S212: Self = Self(842084947);
    pub const WL_SHM_FORMAT_S412: Self = Self(842085459);
    pub const WL_SHM_FORMAT_S016: Self = Self(909193299);
    pub const WL_SHM_FORMAT_S216: Self = Self(909193811);
    pub const WL_SHM_FORMAT_S416: Self = Self(909194323);
    pub const WL_SHM_FORMAT_XVUY2101010: Self = Self(808671576);
    pub const WL_SHM_FORMAT_P230: Self = Self(808661584);
    pub const WL_SHM_FORMAT_T430: Self = Self(808662100);
    pub const WL_SHM_FORMAT_Y8: Self = Self(1497715271);
    pub const WL_SHM_FORMAT_XYYY2101010: Self = Self(876695641);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wl_buffer_listener {
    pub release: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_buffer) -> (),
    >,
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
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_SHARED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WL_MARSHAL_FLAG_DESTROY: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
    << 0 as ::core::ffi::c_int;
pub const WL_SHM_POOL_CREATE_BUFFER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const WL_SHM_POOL_DESTROY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_shm_pool_create_buffer(
    mut wl_shm_pool: *mut wl_shm_pool,
    mut offset: int32_t,
    mut width: int32_t,
    mut height: int32_t,
    mut stride: int32_t,
    mut format: uint32_t,
) -> *mut wl_buffer {
    let mut id: *mut wl_proxy = ::core::ptr::null_mut::<wl_proxy>();
    id = wl_proxy_marshal_flags(
        wl_shm_pool as *mut wl_proxy,
        WL_SHM_POOL_CREATE_BUFFER as uint32_t,
        &raw const wl_buffer_interface,
        wl_proxy_get_version(wl_shm_pool as *mut wl_proxy),
        0 as uint32_t,
        NULL,
        offset,
        width,
        height,
        stride,
        format,
    );
    return id as *mut wl_buffer;
}
#[inline]
unsafe extern "C" fn wl_shm_pool_destroy(mut wl_shm_pool: *mut wl_shm_pool) {
    wl_proxy_marshal_flags(
        wl_shm_pool as *mut wl_proxy,
        WL_SHM_POOL_DESTROY as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_shm_pool as *mut wl_proxy),
        WL_MARSHAL_FLAG_DESTROY as uint32_t,
    );
}
pub const WL_SHM_CREATE_POOL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_shm_create_pool(
    mut wl_shm: *mut wl_shm,
    mut fd: int32_t,
    mut size: int32_t,
) -> *mut wl_shm_pool {
    let mut id: *mut wl_proxy = ::core::ptr::null_mut::<wl_proxy>();
    id = wl_proxy_marshal_flags(
        wl_shm as *mut wl_proxy,
        WL_SHM_CREATE_POOL as uint32_t,
        &raw const wl_shm_pool_interface,
        wl_proxy_get_version(wl_shm as *mut wl_proxy),
        0 as uint32_t,
        NULL,
        fd,
        size,
    );
    return id as *mut wl_shm_pool;
}
#[inline]
unsafe extern "C" fn wl_buffer_add_listener(
    mut wl_buffer: *mut wl_buffer,
    mut listener: *const wl_buffer_listener,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return wl_proxy_add_listener(
        wl_buffer as *mut wl_proxy,
        listener as *mut Option<unsafe extern "C" fn() -> ()>,
        data,
    );
}
pub const WL_BUFFER_DESTROY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn wl_buffer_destroy(mut wl_buffer: *mut wl_buffer) {
    wl_proxy_marshal_flags(
        wl_buffer as *mut wl_proxy,
        WL_BUFFER_DESTROY as uint32_t,
        ::core::ptr::null::<wl_interface>(),
        wl_proxy_get_version(wl_buffer as *mut wl_proxy),
        WL_MARSHAL_FLAG_DESTROY as uint32_t,
    );
}
unsafe extern "C" fn anonymous_shm_open() -> ::core::ffi::c_int {
    let mut retries: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
    loop {
        let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        clock_gettime(CLOCK_MONOTONIC, &raw mut ts);
        let mut pid: pid_t = getpid();
        let mut name: [::core::ffi::c_char; 50] = [0; 50];
        snprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 50]>(),
            b"/sway-%x-%x\0".as_ptr() as *const ::core::ffi::c_char,
            pid as ::core::ffi::c_uint,
            ts.tv_nsec as ::core::ffi::c_uint,
        );
        let mut fd: ::core::ffi::c_int = shm_open(
            &raw mut name as *mut ::core::ffi::c_char,
            O_RDWR | O_CREAT | O_EXCL,
            0o600 as mode_t,
        );
        if fd >= 0 as ::core::ffi::c_int {
            shm_unlink(&raw mut name as *mut ::core::ffi::c_char);
            return fd;
        }
        retries -= 1;
        if !(retries > 0 as ::core::ffi::c_int && *__errno_location() == EEXIST) {
            break;
        }
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn buffer_release(
    mut data: *mut ::core::ffi::c_void,
    mut wl_buffer: *mut wl_buffer,
) {
    let mut buffer: *mut pool_buffer = data as *mut pool_buffer;
    (*buffer).busy = r#false != 0;
}
static mut buffer_listener: wl_buffer_listener = wl_buffer_listener {
    release: Some(
        buffer_release
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut wl_buffer) -> (),
    ),
};
unsafe extern "C" fn create_buffer(
    mut shm: *mut wl_shm,
    mut buf: *mut pool_buffer,
    mut width: int32_t,
    mut height: int32_t,
    mut format: uint32_t,
) -> *mut pool_buffer {
    let mut stride: uint32_t = (width * 4 as int32_t) as uint32_t;
    let mut size: size_t = stride.wrapping_mul(height as uint32_t) as size_t;
    let mut fd: ::core::ffi::c_int = anonymous_shm_open();
    if fd == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<pool_buffer>();
    }
    if ftruncate(fd, size as __off64_t) < 0 as ::core::ffi::c_int {
        close(fd);
        return ::core::ptr::null_mut::<pool_buffer>();
    }
    let mut data: *mut ::core::ffi::c_void = mmap(
        NULL,
        size,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        fd,
        0 as __off64_t,
    );
    let mut pool: *mut wl_shm_pool = wl_shm_create_pool(
        shm,
        fd as int32_t,
        size as int32_t,
    );
    (*buf).buffer = wl_shm_pool_create_buffer(
        pool,
        0 as int32_t,
        width,
        height,
        stride as int32_t,
        format,
    );
    wl_shm_pool_destroy(pool);
    close(fd);
    (*buf).size = size;
    (*buf).width = width as uint32_t;
    (*buf).height = height as uint32_t;
    (*buf).data = data;
    (*buf).surface = cairo_image_surface_create_for_data(
        data as *mut ::core::ffi::c_uchar,
        _cairo_format::CAIRO_FORMAT_ARGB32,
        width as ::core::ffi::c_int,
        height as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    (*buf).cairo = cairo_create((*buf).surface);
    (*buf).pango = pango_cairo_create_context((*buf).cairo);
    wl_buffer_add_listener(
        (*buf).buffer,
        &raw const buffer_listener,
        buf as *mut ::core::ffi::c_void,
    );
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn destroy_buffer(mut buffer: *mut pool_buffer) {
    if !(*buffer).buffer.is_null() {
        wl_buffer_destroy((*buffer).buffer);
    }
    if !(*buffer).cairo.is_null() {
        cairo_destroy((*buffer).cairo);
    }
    if !(*buffer).surface.is_null() {
        cairo_surface_destroy((*buffer).surface);
    }
    if !(*buffer).pango.is_null() {
        g_object_unref((*buffer).pango as gpointer);
    }
    if !(*buffer).data.is_null() {
        munmap((*buffer).data, (*buffer).size);
    }
    memset(
        buffer as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pool_buffer>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn get_next_buffer(
    mut shm: *mut wl_shm,
    mut pool: *mut pool_buffer,
    mut width: uint32_t,
    mut height: uint32_t,
) -> *mut pool_buffer {
    let mut buffer: *mut pool_buffer = ::core::ptr::null_mut::<pool_buffer>();
    let mut i: size_t = 0 as size_t;
    while i < 2 as size_t {
        if !(*pool.offset(i as isize)).busy {
            buffer = pool.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    if buffer.is_null() {
        return ::core::ptr::null_mut::<pool_buffer>();
    }
    if (*buffer).width != width || (*buffer).height != height {
        destroy_buffer(buffer);
    }
    if (*buffer).buffer.is_null() {
        if create_buffer(
                shm,
                buffer,
                width as int32_t,
                height as int32_t,
                wl_shm_format::WL_SHM_FORMAT_ARGB8888.0 as ::core::ffi::c_int as uint32_t,
            )
            .is_null()
        {
            return ::core::ptr::null_mut::<pool_buffer>();
        }
    }
    (*buffer).busy = r#true != 0;
    return buffer;
}
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
