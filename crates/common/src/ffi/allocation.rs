//! Allocations returned through the legacy API must remain compatible with free.
#![deny(unsafe_op_in_unsafe_fn)]

use std::{ffi::c_char, mem, ptr};

pub fn c_string(bytes: &[u8]) -> *mut c_char {
    let Some(size) = bytes.len().checked_add(1) else {
        return ptr::null_mut();
    };
    // SAFETY: malloc owns `size` writable bytes, including the terminator. The
    // caller receives ownership and releases it with the existing C free API.
    unsafe {
        let output = libc::malloc(size).cast::<u8>();
        if !output.is_null() {
            ptr::copy_nonoverlapping(bytes.as_ptr(), output, bytes.len());
            output.add(bytes.len()).write(0);
        }
        output.cast()
    }
}

pub fn c_value<T: Copy>(value: T) -> *mut T {
    assert!(mem::align_of::<T>() <= mem::align_of::<libc::max_align_t>());
    // SAFETY: T is Copy and not over-aligned, and malloc provides its storage.
    unsafe {
        let output = libc::malloc(mem::size_of::<T>()).cast::<T>();
        if !output.is_null() {
            output.write(value);
        }
        output
    }
}
