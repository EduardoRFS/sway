use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn getline(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn wordexp(
        __words: *const ::core::ffi::c_char,
        __pwordexp: *mut wordexp_t,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn wordfree(__wordexp: *mut wordexp_t);
    fn create_list() -> *mut list_t;
    fn list_free(list: *mut list_t);
    fn list_add(list: *mut list_t, item: *mut ::core::ffi::c_void);
    fn list_cat(list: *mut list_t, source: *mut list_t);
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
    fn split_string(
        str: *const ::core::ffi::c_char,
        delims: *const ::core::ffi::c_char,
    ) -> *mut list_t;
    fn format_str(fmt: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
}
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino64_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_int;
pub type __blkcnt64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const _ISupper: Self = Self(256);
    pub const _ISlower: Self = Self(512);
    pub const _ISalpha: Self = Self(1024);
    pub const _ISdigit: Self = Self(2048);
    pub const _ISxdigit: Self = Self(4096);
    pub const _ISspace: Self = Self(8192);
    pub const _ISprint: Self = Self(16384);
    pub const _ISgraph: Self = Self(32768);
    pub const _ISblank: Self = Self(1);
    pub const _IScntrl: Self = Self(2);
    pub const _ISpunct: Self = Self(4);
    pub const _ISalnum: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type DIR = __dirstream;
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino64_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,
    pub __pad1: __dev_t,
    pub st_size: __off64_t,
    pub st_blksize: __blksize_t,
    pub __pad2: ::core::ffi::c_int,
    pub st_blocks: __blkcnt64_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [::core::ffi::c_int; 2],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const WRDE_DOOFFS: Self = Self(1);
    pub const WRDE_APPEND: Self = Self(2);
    pub const WRDE_NOCMD: Self = Self(4);
    pub const WRDE_REUSE: Self = Self(8);
    pub const WRDE_SHOWERR: Self = Self(16);
    pub const WRDE_UNDEF: Self = Self(32);
    pub const __WRDE_FLAGS: Self = Self(63);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordexp_t {
    pub we_wordc: size_t,
    pub we_wordv: *mut *mut ::core::ffi::c_char,
    pub we_offs: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icon_theme_subdir {
    pub name: *mut ::core::ffi::c_char,
    pub size: ::core::ffi::c_int,
    pub r#type: C2Rust_Unnamed_1,
    pub max_size: ::core::ffi::c_int,
    pub min_size: ::core::ffi::c_int,
    pub threshold: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const THRESHOLD: Self = Self(0);
    pub const SCALABLE: Self = Self(1);
    pub const FIXED: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icon_theme {
    pub name: *mut ::core::ffi::c_char,
    pub comment: *mut ::core::ffi::c_char,
    pub inherits: *mut list_t,
    pub directories: *mut list_t,
    pub dir: *mut ::core::ffi::c_char,
    pub subdirs: *mut list_t,
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
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn cmp_id(
    mut item: *const ::core::ffi::c_void,
    mut cmp_to: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        item as *const ::core::ffi::c_char,
        cmp_to as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn dir_exists(mut path: *mut ::core::ffi::c_char) -> bool {
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        __pad1: 0,
        st_size: 0,
        st_blksize: 0,
        __pad2: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 2],
    };
    return stat(path, &raw mut sb) == 0 as ::core::ffi::c_int
        && sb.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t;
}
unsafe extern "C" fn get_basedirs() -> *mut list_t {
    let mut basedirs: *mut list_t = create_list();
    list_add(
        basedirs,
        strdup(b"$HOME/.icons\0".as_ptr() as *const ::core::ffi::c_char)
            as *mut ::core::ffi::c_void,
    );
    let mut data_home: *mut ::core::ffi::c_char = getenv(
        b"XDG_DATA_HOME\0".as_ptr() as *const ::core::ffi::c_char,
    );
    list_add(
        basedirs,
        strdup(
            if !data_home.is_null() && *data_home as ::core::ffi::c_int != 0 {
                b"$XDG_DATA_HOME/icons\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"$HOME/.local/share/icons\0".as_ptr() as *const ::core::ffi::c_char
            },
        ) as *mut ::core::ffi::c_void,
    );
    list_add(
        basedirs,
        strdup(b"/usr/share/pixmaps\0".as_ptr() as *const ::core::ffi::c_char)
            as *mut ::core::ffi::c_void,
    );
    let mut data_dirs: *mut ::core::ffi::c_char = getenv(
        b"XDG_DATA_DIRS\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !(!data_dirs.is_null() && *data_dirs as ::core::ffi::c_int != 0) {
        data_dirs = b"/usr/local/share:/usr/share\0".as_ptr()
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    data_dirs = strdup(data_dirs);
    let mut dir: *mut ::core::ffi::c_char = strtok(
        data_dirs,
        b":\0".as_ptr() as *const ::core::ffi::c_char,
    );
    loop {
        let mut path: *mut ::core::ffi::c_char = format_str(
            b"%s/icons\0".as_ptr() as *const ::core::ffi::c_char,
            dir,
        );
        list_add(basedirs, path as *mut ::core::ffi::c_void);
        dir = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b":\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if dir.is_null() {
            break;
        }
    }
    free(data_dirs as *mut ::core::ffi::c_void);
    let mut basedirs_expanded: *mut list_t = create_list();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*basedirs).length {
        let mut p: wordexp_t = wordexp_t {
            we_wordc: 0,
            we_wordv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            we_offs: 0,
        };
        if wordexp(
            *(*basedirs).items.offset(i as isize) as *const ::core::ffi::c_char,
            &raw mut p,
            C2Rust_Unnamed_0::WRDE_UNDEF.0 as ::core::ffi::c_int,
        ) == 0 as ::core::ffi::c_int
        {
            if dir_exists(*p.we_wordv.offset(0isize)) {
                list_add(
                    basedirs_expanded,
                    strdup(*p.we_wordv.offset(0isize)) as *mut ::core::ffi::c_void,
                );
            }
            wordfree(&raw mut p);
        }
        i += 1;
    }
    list_free_items_and_destroy(basedirs);
    return basedirs_expanded;
}
unsafe extern "C" fn destroy_theme(mut theme: *mut icon_theme) {
    if theme.is_null() {
        return;
    }
    free((*theme).name as *mut ::core::ffi::c_void);
    free((*theme).comment as *mut ::core::ffi::c_void);
    list_free_items_and_destroy((*theme).inherits);
    list_free_items_and_destroy((*theme).directories);
    free((*theme).dir as *mut ::core::ffi::c_void);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*theme).subdirs).length {
        let mut subdir: *mut icon_theme_subdir = *(*(*theme).subdirs)
            .items
            .offset(i as isize) as *mut icon_theme_subdir;
        free((*subdir).name as *mut ::core::ffi::c_void);
        free(subdir as *mut ::core::ffi::c_void);
        i += 1;
    }
    list_free((*theme).subdirs);
    free(theme as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn group_handler(
    mut old_group: *mut ::core::ffi::c_char,
    mut new_group: *mut ::core::ffi::c_char,
    mut theme: *mut icon_theme,
) -> *const ::core::ffi::c_char {
    if old_group.is_null() {
        return if !new_group.is_null()
            && strcmp(new_group, b"Icon Theme\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            ::core::ptr::null::<::core::ffi::c_char>()
        } else {
            b"first group must be 'Icon Theme'\0".as_ptr() as *const ::core::ffi::c_char
        };
    }
    if strcmp(old_group, b"Icon Theme\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if (*theme).name.is_null() {
            return b"missing required key 'Name'\0".as_ptr()
                as *const ::core::ffi::c_char
        } else if (*theme).comment.is_null() {
            return b"missing required key 'Comment'\0".as_ptr()
                as *const ::core::ffi::c_char
        } else if (*theme).directories.is_null() {
            return b"missing required key 'Directories'\0".as_ptr()
                as *const ::core::ffi::c_char
        } else {
            let mut c: *mut ::core::ffi::c_char = (*theme).name;
            while *c != 0 {
                if *c as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                    || *c as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                {
                    return b"malformed theme name\0".as_ptr()
                        as *const ::core::ffi::c_char;
                }
                c = c.offset(1);
            }
        }
    } else {
        if (*(*theme).subdirs).length == 0 as ::core::ffi::c_int {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        let mut subdir: *mut icon_theme_subdir = *(*(*theme).subdirs)
            .items
            .offset(((*(*theme).subdirs).length - 1 as ::core::ffi::c_int) as isize)
            as *mut icon_theme_subdir;
        if (*subdir).size == 0 {
            return b"missing required key 'Size'\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        match (*subdir).r#type {
            C2Rust_Unnamed_1::FIXED => {
                (*subdir).min_size = (*subdir).size;
                (*subdir).max_size = (*subdir).min_size;
            }
            C2Rust_Unnamed_1::SCALABLE => {
                if (*subdir).max_size == 0 {
                    (*subdir).max_size = (*subdir).size;
                }
                if (*subdir).min_size == 0 {
                    (*subdir).min_size = (*subdir).size;
                }
            }
            C2Rust_Unnamed_1::THRESHOLD => {
                (*subdir).max_size = (*subdir).size + (*subdir).threshold;
                (*subdir).min_size = (*subdir).size - (*subdir).threshold;
            }
            _ => {}
        }
    }
    if !new_group.is_null()
        && list_seq_find(
            (*theme).directories,
            Some(
                cmp_id
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            new_group as *const ::core::ffi::c_void,
        ) != -1 as ::core::ffi::c_int
    {
        let mut subdir_0: *mut icon_theme_subdir = calloc(
            1 as size_t,
            ::core::mem::size_of::<icon_theme_subdir>(),
        ) as *mut icon_theme_subdir;
        if subdir_0.is_null() {
            return b"out of memory\0".as_ptr() as *const ::core::ffi::c_char;
        }
        (*subdir_0).name = strdup(new_group);
        (*subdir_0).threshold = 2 as ::core::ffi::c_int;
        list_add((*theme).subdirs, subdir_0 as *mut ::core::ffi::c_void);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn entry_handler(
    mut group: *mut ::core::ffi::c_char,
    mut key: *mut ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_char,
    mut theme: *mut icon_theme,
) -> *const ::core::ffi::c_char {
    if strcmp(group, b"Icon Theme\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if strcmp(key, b"Name\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*theme).name = strdup(value);
        } else if strcmp(key, b"Comment\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*theme).comment = strdup(value);
        } else if strcmp(key, b"Inherits\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*theme).inherits = split_string(
                value,
                b",\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if strcmp(key, b"Directories\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*theme).directories = split_string(
                value,
                b",\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    } else {
        if (*(*theme).subdirs).length == 0 as ::core::ffi::c_int {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        let mut subdir: *mut icon_theme_subdir = *(*(*theme).subdirs)
            .items
            .offset(((*(*theme).subdirs).length - 1 as ::core::ffi::c_int) as isize)
            as *mut icon_theme_subdir;
        if strcmp((*subdir).name, group) != 0 as ::core::ffi::c_int {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        if strcmp(key, b"Context\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return ::core::ptr::null::<::core::ffi::c_char>()
        } else if strcmp(key, b"Type\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            if strcmp(value, b"Fixed\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                (*subdir).r#type = C2Rust_Unnamed_1::FIXED;
            } else if strcmp(value, b"Scalable\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                (*subdir).r#type = C2Rust_Unnamed_1::SCALABLE;
            } else if strcmp(
                value,
                b"Threshold\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*subdir).r#type = C2Rust_Unnamed_1::THRESHOLD;
            } else {
                return b"invalid value - expected 'Fixed', 'Scalable' or 'Threshold'\0"
                    .as_ptr() as *const ::core::ffi::c_char
            }
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut n: ::core::ffi::c_int = strtol(
            value,
            &raw mut end,
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        if *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            return b"invalid value - expected a number\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        if strcmp(key, b"Size\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*subdir).size = n;
        } else if strcmp(key, b"MaxSize\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*subdir).max_size = n;
        } else if strcmp(key, b"MinSize\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*subdir).min_size = n;
        } else if strcmp(key, b"Threshold\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*subdir).threshold = n;
        }
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn read_theme_file(
    mut basedir: *mut ::core::ffi::c_char,
    mut theme_name: *mut ::core::ffi::c_char,
) -> *mut icon_theme {
    let mut path: *mut ::core::ffi::c_char = format_str(
        b"%s/%s/index.theme\0".as_ptr() as *const ::core::ffi::c_char,
        basedir,
        theme_name,
    );
    let mut theme_file: *mut FILE = fopen(
        path,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    free(path as *mut ::core::ffi::c_void);
    if theme_file.is_null() {
        return ::core::ptr::null_mut::<icon_theme>();
    }
    let mut theme: *mut icon_theme = calloc(
        1 as size_t,
        ::core::mem::size_of::<icon_theme>(),
    ) as *mut icon_theme;
    if theme.is_null() {
        fclose(theme_file);
        return ::core::ptr::null_mut::<icon_theme>();
    }
    (*theme).subdirs = create_list();
    let mut groups: *mut list_t = create_list();
    let mut error: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut line_no: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut full_line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut full_len: size_t = 0 as size_t;
    let mut nread: ssize_t = 0;
    loop {
        nread = getline(&raw mut full_line, &raw mut full_len, theme_file) as ssize_t;
        if nread == -1 as ssize_t {
            break;
        }
        line_no += 1;
        let mut line: *mut ::core::ffi::c_char = full_line
            .offset(-(1 as ::core::ffi::c_int as isize));
        loop {
            line = line.offset(1);
            if *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int
                    as ::core::ffi::c_ushort as ::core::ffi::c_int == 0
            {
                break;
            }
        }
        if *line == 0
            || *line.offset(0isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int
        {
            continue;
        }
        let mut len: ::core::ffi::c_int = (nread
            - line.offset_from(full_line) as ssize_t) as ::core::ffi::c_int;
        loop {
            len -= 1;
            if *(*__ctype_b_loc())
                .offset(*line.offset(len as isize) as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int
                    as ::core::ffi::c_ushort as ::core::ffi::c_int == 0
            {
                break;
            }
        }
        len += 1;
        *line.offset(len as isize) = '\0' as ::core::ffi::c_char;
        if *line.offset(0isize) as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while *(*__ctype_b_loc())
                .offset(*line.offset(i as isize) as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int
                    as ::core::ffi::c_ushort as ::core::ffi::c_int == 0
                && *line.offset(i as isize) as ::core::ffi::c_int
                    != '[' as ::core::ffi::c_int
                && *line.offset(i as isize) as ::core::ffi::c_int
                    != ']' as ::core::ffi::c_int
            {
                i += 1;
            }
            len -= 1;
            if i != len
                || *line.offset(i as isize) as ::core::ffi::c_int
                    != ']' as ::core::ffi::c_int
            {
                error = b"malformed group header\0".as_ptr()
                    as *const ::core::ffi::c_char;
                break;
            } else {
                *line.offset(len as isize) = '\0' as ::core::ffi::c_char;
                if list_seq_find(
                    groups,
                    Some(
                        cmp_id
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_void,
                                *const ::core::ffi::c_void,
                            ) -> ::core::ffi::c_int,
                    ),
                    line.offset(1isize) as *const ::core::ffi::c_void,
                ) != -1 as ::core::ffi::c_int
                {
                    error = b"duplicate group\0".as_ptr() as *const ::core::ffi::c_char;
                    break;
                } else {
                    let mut last_group: *mut ::core::ffi::c_char = (if (*groups).length
                        > 0 as ::core::ffi::c_int
                    {
                        *(*groups)
                            .items
                            .offset(
                                ((*groups).length - 1 as ::core::ffi::c_int) as isize,
                            )
                    } else {
                        NULL
                    }) as *mut ::core::ffi::c_char;
                    error = group_handler(last_group, line.offset(1isize), theme);
                    if !error.is_null() {
                        break;
                    }
                    list_add(
                        groups,
                        strdup(line.offset(1isize)) as *mut ::core::ffi::c_void,
                    );
                }
            }
        } else if (*groups).length == 0 as ::core::ffi::c_int {
            error = b"unexpected content before first header\0".as_ptr()
                as *const ::core::ffi::c_char;
            break;
        } else {
            let mut eok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while *(*__ctype_b_loc())
                .offset(*line.offset(eok as isize) as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int
                    as ::core::ffi::c_ushort as ::core::ffi::c_int != 0
                || *line.offset(eok as isize) as ::core::ffi::c_int
                    == '-' as ::core::ffi::c_int
            {
                eok += 1;
            }
            let mut i_0: ::core::ffi::c_int = eok - 1 as ::core::ffi::c_int;
            loop {
                i_0 += 1;
                if *(*__ctype_b_loc())
                    .offset(*line.offset(i_0 as isize) as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int
                        as ::core::ffi::c_ushort as ::core::ffi::c_int == 0
                {
                    break;
                }
            }
            if *line.offset(i_0 as isize) as ::core::ffi::c_int
                != '=' as ::core::ffi::c_int
            {
                error = b"malformed key-value pair\0".as_ptr()
                    as *const ::core::ffi::c_char;
                break;
            } else {
                *line.offset(eok as isize) = '\0' as ::core::ffi::c_char;
                let mut value: *mut ::core::ffi::c_char = line.offset(i_0 as isize);
                loop {
                    value = value.offset(1);
                    if *(*__ctype_b_loc()).offset(*value as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int
                            as ::core::ffi::c_ushort as ::core::ffi::c_int == 0
                    {
                        break;
                    }
                }
                error = entry_handler(
                    *(*groups)
                        .items
                        .offset(((*groups).length - 1 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_char,
                    line,
                    value,
                    theme,
                );
                if !error.is_null() {
                    break;
                }
            }
        }
    }
    if error.is_null() {
        if (*groups).length > 0 as ::core::ffi::c_int {
            error = group_handler(
                *(*groups)
                    .items
                    .offset(((*groups).length - 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                theme,
            );
        } else {
            error = b"empty file\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    if error.is_null() {
        (*theme).dir = strdup(theme_name);
    } else {
        let mut last_group_0: *mut ::core::ffi::c_char = (if (*groups).length
            > 0 as ::core::ffi::c_int
        {
            *(*groups)
                .items
                .offset(((*groups).length - 1 as ::core::ffi::c_int) as isize)
        } else {
            b"n/a\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void
        }) as *mut ::core::ffi::c_char;
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            b"[%s:%d] Failed to load theme '%s' - parsing of file '%s/%s/index.theme' failed on line %d (group '%s'): %s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"swaybar/tray/icon.c\0".as_ptr() as *const ::core::ffi::c_char,
            304 as ::core::ffi::c_int,
            theme_name,
            basedir,
            theme_name,
            line_no,
            last_group_0,
            error,
        );
        destroy_theme(theme);
        theme = ::core::ptr::null_mut::<icon_theme>();
    }
    free(full_line as *mut ::core::ffi::c_void);
    list_free_items_and_destroy(groups);
    fclose(theme_file);
    return theme;
}
unsafe extern "C" fn load_themes_in_dir(
    mut basedir: *mut ::core::ffi::c_char,
) -> *mut list_t {
    let mut dir: *mut DIR = ::core::ptr::null_mut::<DIR>();
    dir = opendir(basedir);
    if dir.is_null() {
        return ::core::ptr::null_mut::<list_t>();
    }
    let mut themes: *mut list_t = create_list();
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        if (*entry).d_name[0usize] as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
            continue;
        }
        let mut theme: *mut icon_theme = read_theme_file(
            basedir,
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
        );
        if !theme.is_null() {
            list_add(themes, theme as *mut ::core::ffi::c_void);
        }
    }
    closedir(dir);
    return themes;
}
unsafe extern "C" fn log_loaded_themes(mut themes: *mut list_t) {
    if (*themes).length == 0 as ::core::ffi::c_int {
        _sway_log(
            sway_log_importance_t::SWAY_INFO,
            b"[%s:%d] Warning: no icon themes loaded\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"swaybar/tray/icon.c\0".as_ptr() as *const ::core::ffi::c_char,
            339 as ::core::ffi::c_int,
        );
        return;
    }
    let sep: [::core::ffi::c_char; 3] = ::core::mem::transmute::<
        [u8; 3],
        [::core::ffi::c_char; 3],
    >(*b", \0");
    let mut sep_len: size_t = strlen(&raw const sep as *const ::core::ffi::c_char);
    let mut len: size_t = 0 as size_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*themes).length {
        let mut theme: *mut icon_theme = *(*themes).items.offset(i as isize)
            as *mut icon_theme;
        len = len.wrapping_add(strlen((*theme).name).wrapping_add(sep_len));
        i += 1;
    }
    let mut str: *mut ::core::ffi::c_char = malloc(len.wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
    if str.is_null() {
        return;
    }
    let mut p: *mut ::core::ffi::c_char = str;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*themes).length {
        if i_0 > 0 as ::core::ffi::c_int {
            memcpy(
                p as *mut ::core::ffi::c_void,
                &raw const sep as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                sep_len,
            );
            p = p.offset(sep_len as isize);
        }
        let mut theme_0: *mut icon_theme = *(*themes).items.offset(i_0 as isize)
            as *mut icon_theme;
        let mut name_len: size_t = strlen((*theme_0).name);
        memcpy(
            p as *mut ::core::ffi::c_void,
            (*theme_0).name as *const ::core::ffi::c_void,
            name_len,
        );
        p = p.offset(name_len as isize);
        i_0 += 1;
    }
    *p = '\0' as ::core::ffi::c_char;
    _sway_log(
        sway_log_importance_t::SWAY_DEBUG,
        b"[%s:%d] Loaded icon themes: %s\0".as_ptr() as *const ::core::ffi::c_char,
        b"swaybar/tray/icon.c\0".as_ptr() as *const ::core::ffi::c_char,
        370 as ::core::ffi::c_int,
        str,
    );
    free(str as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn init_themes(
    mut themes: *mut *mut list_t,
    mut basedirs: *mut *mut list_t,
) {
    *basedirs = get_basedirs();
    *themes = create_list();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (**basedirs).length {
        let mut dir_themes: *mut list_t = load_themes_in_dir(
            *(**basedirs).items.offset(i as isize) as *mut ::core::ffi::c_char,
        );
        if !dir_themes.is_null() {
            list_cat(*themes, dir_themes);
            list_free(dir_themes);
        }
        i += 1;
    }
    log_loaded_themes(*themes);
}
#[no_mangle]
pub unsafe extern "C" fn finish_themes(
    mut themes: *mut list_t,
    mut basedirs: *mut list_t,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*themes).length {
        destroy_theme(*(*themes).items.offset(i as isize) as *mut icon_theme);
        i += 1;
    }
    list_free(themes);
    list_free_items_and_destroy(basedirs);
}
unsafe extern "C" fn find_icon_in_subdir(
    mut name: *mut ::core::ffi::c_char,
    mut basedir: *mut ::core::ffi::c_char,
    mut theme: *mut ::core::ffi::c_char,
    mut subdir: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    static mut extensions: [*const ::core::ffi::c_char; 3] = [
        b"svg\0".as_ptr() as *const ::core::ffi::c_char,
        b"png\0".as_ptr() as *const ::core::ffi::c_char,
        b"xpm\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        let mut path: *mut ::core::ffi::c_char = format_str(
            b"%s/%s/%s/%s.%s\0".as_ptr() as *const ::core::ffi::c_char,
            basedir,
            theme,
            subdir,
            name,
            extensions[i],
        );
        if access(path, R_OK) == 0 as ::core::ffi::c_int {
            return path;
        }
        free(path as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn theme_exists_in_basedir(
    mut theme: *mut ::core::ffi::c_char,
    mut basedir: *mut ::core::ffi::c_char,
) -> bool {
    let mut path: *mut ::core::ffi::c_char = format_str(
        b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
        basedir,
        theme,
    );
    let mut ret: bool = dir_exists(path);
    free(path as *mut ::core::ffi::c_void);
    return ret;
}
unsafe extern "C" fn find_icon_with_theme(
    mut basedirs: *mut list_t,
    mut themes: *mut list_t,
    mut name: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut theme_name: *mut ::core::ffi::c_char,
    mut min_size: *mut ::core::ffi::c_int,
    mut max_size: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut theme: *mut icon_theme = ::core::ptr::null_mut::<icon_theme>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*themes).length {
        theme = *(*themes).items.offset(i as isize) as *mut icon_theme;
        if strcmp((*theme).name, theme_name) == 0 as ::core::ffi::c_int {
            break;
        }
        theme = ::core::ptr::null_mut::<icon_theme>();
        i += 1;
    }
    if theme.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut icon: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*basedirs).length {
        if theme_exists_in_basedir(
            (*theme).dir,
            *(*basedirs).items.offset(i_0 as isize) as *mut ::core::ffi::c_char,
        ) {
            let mut j: ::core::ffi::c_int = (*(*theme).subdirs).length
                - 1 as ::core::ffi::c_int;
            while j >= 0 as ::core::ffi::c_int {
                let mut subdir: *mut icon_theme_subdir = *(*(*theme).subdirs)
                    .items
                    .offset(j as isize) as *mut icon_theme_subdir;
                if size >= (*subdir).min_size && size <= (*subdir).max_size {
                    icon = find_icon_in_subdir(
                        name,
                        *(*basedirs).items.offset(i_0 as isize)
                            as *mut ::core::ffi::c_char,
                        (*theme).dir,
                        (*subdir).name,
                    );
                    if !icon.is_null() {
                        *min_size = (*subdir).min_size;
                        *max_size = (*subdir).max_size;
                        return icon;
                    }
                }
                j -= 1;
            }
        }
        i_0 += 1;
    }
    let mut smallest_error: ::core::ffi::c_uint = -1 as ::core::ffi::c_int
        as ::core::ffi::c_uint;
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < (*basedirs).length {
        if theme_exists_in_basedir(
            (*theme).dir,
            *(*basedirs).items.offset(i_1 as isize) as *mut ::core::ffi::c_char,
        ) {
            let mut j_0: ::core::ffi::c_int = (*(*theme).subdirs).length
                - 1 as ::core::ffi::c_int;
            while j_0 >= 0 as ::core::ffi::c_int {
                let mut subdir_0: *mut icon_theme_subdir = *(*(*theme).subdirs)
                    .items
                    .offset(j_0 as isize) as *mut icon_theme_subdir;
                let mut error: ::core::ffi::c_uint = (if size > (*subdir_0).max_size {
                    size - (*subdir_0).max_size
                } else {
                    0 as ::core::ffi::c_int
                }
                    + if size < (*subdir_0).min_size {
                        (*subdir_0).min_size - size
                    } else {
                        0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint;
                if error < smallest_error {
                    let mut test_icon: *mut ::core::ffi::c_char = find_icon_in_subdir(
                        name,
                        *(*basedirs).items.offset(i_1 as isize)
                            as *mut ::core::ffi::c_char,
                        (*theme).dir,
                        (*subdir_0).name,
                    );
                    if !test_icon.is_null() {
                        icon = test_icon;
                        smallest_error = error;
                        *min_size = (*subdir_0).min_size;
                        *max_size = (*subdir_0).max_size;
                    }
                }
                j_0 -= 1;
            }
        }
        i_1 += 1;
    }
    if icon.is_null() && !(*theme).inherits.is_null() {
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 < (*(*theme).inherits).length {
            icon = find_icon_with_theme(
                basedirs,
                themes,
                name,
                size,
                *(*(*theme).inherits).items.offset(i_2 as isize)
                    as *mut ::core::ffi::c_char,
                min_size,
                max_size,
            );
            if !icon.is_null() {
                break;
            }
            i_2 += 1;
        }
    }
    return icon;
}
unsafe extern "C" fn find_fallback_icon(
    mut basedirs: *mut list_t,
    mut name: *mut ::core::ffi::c_char,
    mut min_size: *mut ::core::ffi::c_int,
    mut max_size: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*basedirs).length {
        let mut icon: *mut ::core::ffi::c_char = find_icon_in_subdir(
            name,
            *(*basedirs).items.offset(i as isize) as *mut ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        if !icon.is_null() {
            *min_size = 1 as ::core::ffi::c_int;
            *max_size = 512 as ::core::ffi::c_int;
            return icon;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn find_icon(
    mut themes: *mut list_t,
    mut basedirs: *mut list_t,
    mut name: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut theme: *mut ::core::ffi::c_char,
    mut min_size: *mut ::core::ffi::c_int,
    mut max_size: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut icon: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if !theme.is_null() {
        icon = find_icon_with_theme(
            basedirs,
            themes,
            name,
            size,
            theme,
            min_size,
            max_size,
        );
    }
    if icon.is_null()
        && !(!theme.is_null()
            && strcmp(theme, b"Hicolor\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int)
    {
        icon = find_icon_with_theme(
            basedirs,
            themes,
            name,
            size,
            b"Hicolor\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            min_size,
            max_size,
        );
    }
    if icon.is_null() {
        icon = find_fallback_icon(basedirs, name, min_size, max_size);
    }
    return icon;
}
