use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getline(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn pclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn popen(
        __command: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn connect(
        __fd: ::core::ffi::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn recv(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn _sway_log(
        verbosity: sway_log_importance_t,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn _sway_abort(filename: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = isize;
pub type uint32_t = u32;
pub type socklen_t = __socklen_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct __socket_type(pub ::core::ffi::c_uint);
impl __socket_type {
    pub const SOCK_STREAM: Self = Self(1);
    pub const SOCK_DGRAM: Self = Self(2);
    pub const SOCK_RAW: Self = Self(3);
    pub const SOCK_RDM: Self = Self(4);
    pub const SOCK_SEQPACKET: Self = Self(5);
    pub const SOCK_DCCP: Self = Self(6);
    pub const SOCK_PACKET: Self = Self(10);
    pub const SOCK_CLOEXEC: Self = Self(524288);
    pub const SOCK_NONBLOCK: Self = Self(2048);
}
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_response {
    pub size: uint32_t,
    pub r#type: uint32_t,
    pub payload: *mut ::core::ffi::c_char,
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_RCVTIMEO: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut ipc_magic: [::core::ffi::c_char; 6] = [
    'i' as ::core::ffi::c_char,
    '3' as ::core::ffi::c_char,
    '-' as ::core::ffi::c_char,
    'i' as ::core::ffi::c_char,
    'p' as ::core::ffi::c_char,
    'c' as ::core::ffi::c_char,
];
pub const IPC_HEADER_SIZE: usize = ::core::mem::size_of::<[::core::ffi::c_char; 6]>()
    .wrapping_add(8usize);
#[no_mangle]
pub unsafe extern "C" fn get_socketpath() -> *mut ::core::ffi::c_char {
    let mut swaysock: *const ::core::ffi::c_char = getenv(
        b"SWAYSOCK\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !swaysock.is_null() {
        return strdup(swaysock);
    }
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut line_size: size_t = 0 as size_t;
    let mut fp: *mut FILE = popen(
        b"sway --get-socketpath 2>/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut nret: ssize_t = getline(&raw mut line, &raw mut line_size, fp)
            as ssize_t;
        pclose(fp);
        if nret > 0 as ssize_t {
            if *line.offset(nret - 1 as ssize_t) as ::core::ffi::c_int
                == '\n' as ::core::ffi::c_int
            {
                *line.offset(nret - 1 as ssize_t) = '\0' as ::core::ffi::c_char;
            }
            return line;
        }
    }
    let mut i3sock: *const ::core::ffi::c_char = getenv(
        b"I3SOCK\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !i3sock.is_null() {
        free(line as *mut ::core::ffi::c_void);
        return strdup(i3sock);
    }
    fp = popen(
        b"i3 --get-socketpath 2>/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut nret_0: ssize_t = getline(&raw mut line, &raw mut line_size, fp)
            as ssize_t;
        pclose(fp);
        if nret_0 > 0 as ssize_t {
            if *line.offset(nret_0 - 1 as ssize_t) as ::core::ffi::c_int
                == '\n' as ::core::ffi::c_int
            {
                *line.offset(nret_0 - 1 as ssize_t) = '\0' as ::core::ffi::c_char;
            }
            return line;
        }
    }
    free(line as *mut ::core::ffi::c_void);
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn ipc_open_socket(
    mut socket_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut addr: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut socketfd: ::core::ffi::c_int = 0;
    socketfd = socket(
        AF_UNIX,
        __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if socketfd == -1 as ::core::ffi::c_int {
        _sway_abort(
            b"[%s:%d] Unable to open Unix socket\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"common/ipc-client.c\0".as_ptr() as *const ::core::ffi::c_char,
            59 as ::core::ffi::c_int,
        );
    }
    addr.sun_family = AF_UNIX as sa_family_t;
    strncpy(
        &raw mut addr.sun_path as *mut ::core::ffi::c_char,
        socket_path,
        ::core::mem::size_of::<[::core::ffi::c_char; 108]>().wrapping_sub(1 as size_t),
    );
    addr
        .sun_path[::core::mem::size_of::<[::core::ffi::c_char; 108]>()
        .wrapping_sub(1usize)] = 0 as ::core::ffi::c_char;
    let mut l: ::core::ffi::c_int = ::core::mem::size_of::<sockaddr_un>()
        as ::core::ffi::c_int;
    if connect(socketfd, &raw mut addr as *mut sockaddr, l as socklen_t)
        == -1 as ::core::ffi::c_int
    {
        _sway_abort(
            b"[%s:%d] Unable to connect to %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"common/ipc-client.c\0".as_ptr() as *const ::core::ffi::c_char,
            66 as ::core::ffi::c_int,
            socket_path,
        );
    }
    return socketfd;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_set_recv_timeout(
    mut socketfd: ::core::ffi::c_int,
    mut tv: timeval,
) -> bool {
    if setsockopt(
        socketfd,
        SOL_SOCKET,
        SO_RCVTIMEO,
        &raw mut tv as *const ::core::ffi::c_void,
        ::core::mem::size_of::<timeval>() as socklen_t,
    ) == -1 as ::core::ffi::c_int
    {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            b"[%s:%d] Failed to set ipc recv timeout: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"common/ipc-client.c\0".as_ptr() as *const ::core::ffi::c_char,
            73 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        return r#false != 0;
    }
    return r#true != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_recv_response(
    mut socketfd: ::core::ffi::c_int,
) -> *mut ipc_response {
    let mut payload: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut data: [::core::ffi::c_char; 14] = [0; 14];
    let mut total: size_t = 0 as size_t;
    while total < IPC_HEADER_SIZE {
        let mut received: ssize_t = recv(
            socketfd,
            (&raw mut data as *mut ::core::ffi::c_char).offset(total as isize)
                as *mut ::core::ffi::c_void,
            IPC_HEADER_SIZE.wrapping_sub(total),
            0 as ::core::ffi::c_int,
        );
        if received <= 0 as ssize_t {
            _sway_abort(
                b"[%s:%d] Unable to receive IPC response\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"common/ipc-client.c\0".as_ptr() as *const ::core::ffi::c_char,
                86 as ::core::ffi::c_int,
            );
        }
        total = total.wrapping_add(received as size_t);
    }
    let mut response: *mut ipc_response = malloc(::core::mem::size_of::<ipc_response>())
        as *mut ipc_response;
    if !response.is_null() {
        memcpy(
            &raw mut (*response).size as *mut ::core::ffi::c_void,
            (&raw mut data as *mut ::core::ffi::c_char)
                .offset(::core::mem::size_of::<[::core::ffi::c_char; 6]>() as isize)
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<uint32_t>(),
        );
        memcpy(
            &raw mut (*response).r#type as *mut ::core::ffi::c_void,
            (&raw mut data as *mut ::core::ffi::c_char)
                .offset(::core::mem::size_of::<[::core::ffi::c_char; 6]>() as isize)
                .offset(::core::mem::size_of::<uint32_t>() as isize)
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<uint32_t>(),
        );
        payload = malloc((*response).size.wrapping_add(1 as uint32_t) as size_t)
            as *mut ::core::ffi::c_char;
        if payload.is_null() {
            free(response as *mut ::core::ffi::c_void);
        } else {
            total = 0 as size_t;
            while total < (*response).size as size_t {
                let mut received_0: ssize_t = recv(
                    socketfd,
                    payload.offset(total as isize) as *mut ::core::ffi::c_void,
                    ((*response).size as size_t).wrapping_sub(total),
                    0 as ::core::ffi::c_int,
                );
                if received_0 < 0 as ssize_t {
                    _sway_abort(
                        b"[%s:%d] Unable to receive IPC response\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"common/ipc-client.c\0".as_ptr() as *const ::core::ffi::c_char,
                        108 as ::core::ffi::c_int,
                    );
                }
                total = total.wrapping_add(received_0 as size_t);
            }
            *payload.offset((*response).size as isize) = '\0' as ::core::ffi::c_char;
            (*response).payload = payload;
            return response;
        }
    }
    _sway_log(
        sway_log_importance_t::SWAY_ERROR,
        b"[%s:%d] Unable to allocate memory for IPC response\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"common/ipc-client.c\0".as_ptr() as *const ::core::ffi::c_char,
        119 as ::core::ffi::c_int,
    );
    return ::core::ptr::null_mut::<ipc_response>();
}
#[no_mangle]
pub unsafe extern "C" fn free_ipc_response(mut response: *mut ipc_response) {
    free((*response).payload as *mut ::core::ffi::c_void);
    free(response as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_single_command(
    mut socketfd: ::core::ffi::c_int,
    mut r#type: uint32_t,
    mut payload: *const ::core::ffi::c_char,
    mut len: *mut uint32_t,
) -> *mut ::core::ffi::c_char {
    let mut data: [::core::ffi::c_char; 14] = [0; 14];
    memcpy(
        &raw mut data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        &raw const ipc_magic as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 6]>(),
    );
    memcpy(
        (&raw mut data as *mut ::core::ffi::c_char)
            .offset(::core::mem::size_of::<[::core::ffi::c_char; 6]>() as isize)
            as *mut ::core::ffi::c_void,
        len as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>(),
    );
    memcpy(
        (&raw mut data as *mut ::core::ffi::c_char)
            .offset(::core::mem::size_of::<[::core::ffi::c_char; 6]>() as isize)
            .offset(::core::mem::size_of::<uint32_t>() as isize)
            as *mut ::core::ffi::c_void,
        &raw mut r#type as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>(),
    );
    if write(
        socketfd,
        &raw mut data as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        IPC_HEADER_SIZE,
    ) == -1 as ssize_t
    {
        _sway_abort(
            b"[%s:%d] Unable to send IPC header\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"common/ipc-client.c\0".as_ptr() as *const ::core::ffi::c_char,
            135 as ::core::ffi::c_int,
        );
    }
    if write(socketfd, payload as *const ::core::ffi::c_void, *len as size_t)
        == -1 as ssize_t
    {
        _sway_abort(
            b"[%s:%d] Unable to send IPC payload\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"common/ipc-client.c\0".as_ptr() as *const ::core::ffi::c_char,
            139 as ::core::ffi::c_int,
        );
    }
    let mut resp: *mut ipc_response = ipc_recv_response(socketfd);
    let mut response: *mut ::core::ffi::c_char = (*resp).payload;
    *len = (*resp).size;
    free(resp as *mut ::core::ffi::c_void);
    return response;
}
