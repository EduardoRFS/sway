extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn _sway_assert(condition: bool, format: *const ::core::ffi::c_char, ...) -> bool;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub capacity: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[no_mangle]
pub unsafe extern "C" fn create_list() -> *mut list_t {
    let mut list: *mut list_t = malloc(::core::mem::size_of::<list_t>()) as *mut list_t;
    if list.is_null() {
        return ::core::ptr::null_mut::<list_t>();
    }
    (*list).capacity = 10 as ::core::ffi::c_int;
    (*list).length = 0 as ::core::ffi::c_int;
    (*list).items = malloc(
        ::core::mem::size_of::<*mut ::core::ffi::c_void>()
            .wrapping_mul((*list).capacity as size_t),
    ) as *mut *mut ::core::ffi::c_void;
    return list;
}
unsafe extern "C" fn list_resize(mut list: *mut list_t) {
    if (*list).length == (*list).capacity {
        (*list).capacity *= 2 as ::core::ffi::c_int;
        (*list).items = realloc(
            (*list).items as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                .wrapping_mul((*list).capacity as size_t),
        ) as *mut *mut ::core::ffi::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn list_free(mut list: *mut list_t) {
    if list.is_null() {
        return;
    }
    free((*list).items as *mut ::core::ffi::c_void);
    free(list as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn list_add(
    mut list: *mut list_t,
    mut item: *mut ::core::ffi::c_void,
) {
    list_resize(list);
    let c2rust_fresh0 = (*list).length;
    (*list).length += 1;
    *(*list).items.offset(c2rust_fresh0 as isize) = item;
}
#[no_mangle]
pub unsafe extern "C" fn list_insert(
    mut list: *mut list_t,
    mut index: ::core::ffi::c_int,
    mut item: *mut ::core::ffi::c_void,
) {
    list_resize(list);
    memmove(
        (*list).items.offset((index + 1 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_void,
        (*list).items.offset(index as isize) as *const ::core::ffi::c_void,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>()
            .wrapping_mul(((*list).length - index) as size_t),
    );
    (*list).length += 1;
    *(*list).items.offset(index as isize) = item;
}
#[no_mangle]
pub unsafe extern "C" fn list_del(mut list: *mut list_t, mut index: ::core::ffi::c_int) {
    (*list).length -= 1;
    memmove(
        (*list).items.offset(index as isize) as *mut ::core::ffi::c_void,
        (*list).items.offset((index + 1 as ::core::ffi::c_int) as isize)
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>()
            .wrapping_mul(((*list).length - index) as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn list_cat(mut list: *mut list_t, mut source: *mut list_t) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*source).length {
        list_add(list, *(*source).items.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn list_qsort(
    mut list: *mut list_t,
    mut compare: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*list).items as *mut ::core::ffi::c_void,
        (*list).length as size_t,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>(),
        compare,
    );
}
#[no_mangle]
pub unsafe extern "C" fn list_seq_find(
    mut list: *mut list_t,
    mut compare: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut data: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*list).length {
        let mut item: *mut ::core::ffi::c_void = *(*list).items.offset(i as isize);
        if compare.expect("non-null function pointer")(item, data)
            == 0 as ::core::ffi::c_int
        {
            return i;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn list_find(
    mut list: *mut list_t,
    mut item: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*list).length {
        if *(*list).items.offset(i as isize) == item as *mut ::core::ffi::c_void {
            return i;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn list_swap(
    mut list: *mut list_t,
    mut src: ::core::ffi::c_int,
    mut dest: ::core::ffi::c_int,
) {
    let mut tmp: *mut ::core::ffi::c_void = *(*list).items.offset(src as isize);
    *(*list).items.offset(src as isize) = *(*list).items.offset(dest as isize);
    *(*list).items.offset(dest as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn list_move_to_end(
    mut list: *mut list_t,
    mut item: *mut ::core::ffi::c_void,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*list).length {
        if *(*list).items.offset(i as isize) == item {
            break;
        }
        i += 1;
    }
    if !_sway_assert(
        i < (*list).length,
        b"[%s:%d] %s:Item not found in list\0".as_ptr() as *const ::core::ffi::c_char,
        b"common/list.c\0".as_ptr() as *const ::core::ffi::c_char,
        92 as ::core::ffi::c_int,
        b"void list_move_to_end(list_t *, void *)\0".as_ptr()
            as *const ::core::ffi::c_char,
    ) {
        return;
    }
    list_del(list, i);
    list_add(list, item);
}
unsafe extern "C" fn list_rotate(
    mut list: *mut list_t,
    mut from: ::core::ffi::c_int,
    mut to: ::core::ffi::c_int,
) {
    let mut tmp: *mut ::core::ffi::c_void = *(*list).items.offset(to as isize);
    while to > from {
        *(*list).items.offset(to as isize) = *(*list)
            .items
            .offset((to - 1 as ::core::ffi::c_int) as isize);
        to -= 1;
    }
    *(*list).items.offset(from as isize) = tmp;
}
unsafe extern "C" fn list_inplace_merge(
    mut list: *mut list_t,
    mut left: ::core::ffi::c_int,
    mut last: ::core::ffi::c_int,
    mut mid: ::core::ffi::c_int,
    mut compare: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
) {
    let mut right: ::core::ffi::c_int = mid + 1 as ::core::ffi::c_int;
    if compare
        .expect(
            "non-null function pointer",
        )(
        (*list).items.offset(mid as isize) as *const ::core::ffi::c_void,
        (*list).items.offset(right as isize) as *const ::core::ffi::c_void,
    ) <= 0 as ::core::ffi::c_int
    {
        return;
    }
    while left <= mid && right <= last {
        if compare
            .expect(
                "non-null function pointer",
            )(
            (*list).items.offset(left as isize) as *const ::core::ffi::c_void,
            (*list).items.offset(right as isize) as *const ::core::ffi::c_void,
        ) <= 0 as ::core::ffi::c_int
        {
            left += 1;
        } else {
            list_rotate(list, left, right);
            left += 1;
            mid += 1;
            right += 1;
        }
    }
}
unsafe extern "C" fn list_inplace_sort(
    mut list: *mut list_t,
    mut first: ::core::ffi::c_int,
    mut last: ::core::ffi::c_int,
    mut compare: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
) {
    if first >= last {
        return
    } else if last - first == 1 as ::core::ffi::c_int {
        if compare
            .expect(
                "non-null function pointer",
            )(
            (*list).items.offset(first as isize) as *const ::core::ffi::c_void,
            (*list).items.offset(last as isize) as *const ::core::ffi::c_void,
        ) > 0 as ::core::ffi::c_int
        {
            list_swap(list, first, last);
        }
    } else {
        let mut mid: ::core::ffi::c_int = (last + first) / 2 as ::core::ffi::c_int;
        list_inplace_sort(list, first, mid, compare);
        list_inplace_sort(list, mid + 1 as ::core::ffi::c_int, last, compare);
        list_inplace_merge(list, first, last, mid, compare);
    };
}
#[no_mangle]
pub unsafe extern "C" fn list_stable_sort(
    mut list: *mut list_t,
    mut compare: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
) {
    if (*list).length > 1 as ::core::ffi::c_int {
        list_inplace_sort(
            list,
            0 as ::core::ffi::c_int,
            (*list).length - 1 as ::core::ffi::c_int,
            compare,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn list_free_items_and_destroy(mut list: *mut list_t) {
    if list.is_null() {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*list).length {
        free(*(*list).items.offset(i as isize));
        i += 1;
    }
    list_free(list);
}
