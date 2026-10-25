//! The remaining C API owns file descriptors and frees returned allocations.
#![deny(unsafe_op_in_unsafe_fn)]

use super::{
    allocation::{c_string, c_value},
    log::{_sway_abort, _sway_log, sway_log_importance_t},
};
use crate::{
    ipc::{Frame, Stage},
    socket_path,
};
use std::{
    ffi::{c_char, CStr, OsStr},
    io,
    os::{
        fd::{BorrowedFd, IntoRawFd},
        unix::{ffi::OsStrExt, net::UnixStream},
    },
    path::Path,
    ptr, slice,
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct IpcResponse {
    pub size: u32,
    pub kind: u32,
    pub payload: *mut c_char,
}

// SAFETY: each caller supplies a live Unix socket. Duplicate it into a Rust
// owner for the duration of the operation, leaving the caller's fd untouched.
unsafe fn stream(fd: i32) -> io::Result<UnixStream> {
    unsafe { BorrowedFd::borrow_raw(fd) }
        .try_clone_to_owned()
        .map(UnixStream::from)
}

fn report_abort(line: i32, message: &CStr) {
    unsafe {
        _sway_abort(
            c"[%s:%d] %s".as_ptr(),
            c"common/ipc-client.c".as_ptr(),
            line,
            message.as_ptr(),
        )
    };
}

fn allocation_failed() {
    unsafe {
        _sway_log(
            sway_log_importance_t::SWAY_ERROR,
            c"[%s:%d] Unable to allocate memory for IPC response".as_ptr(),
            c"common/ipc-client.c".as_ptr(),
            119,
        );
    }
}

unsafe fn receive(fd: i32) -> Option<Frame> {
    let mut stream = match unsafe { stream(fd) } {
        Ok(stream) => stream,
        Err(_) => {
            report_abort(86, c"Unable to receive IPC response");
            return None;
        }
    };
    receive_from(&mut stream)
}

fn receive_from(stream: &mut UnixStream) -> Option<Frame> {
    match Frame::read_from(stream) {
        Ok(frame) => Some(frame),
        Err(error) => {
            let line = match error.stage {
                Stage::Header => 86,
                Stage::Payload => 108,
            };
            report_abort(line, c"Unable to receive IPC response");
            None
        }
    }
}

#[no_mangle]
pub extern "C" fn get_socketpath() -> *mut c_char {
    socket_path::discover().map_or(ptr::null_mut(), |path| c_string(path.as_bytes()))
}

/// # Safety
/// `socket_path` must point to a NUL-terminated byte string.
#[no_mangle]
pub unsafe extern "C" fn ipc_open_socket(socket_path: *const c_char) -> i32 {
    let bytes = unsafe { CStr::from_ptr(socket_path) }.to_bytes();
    // The legacy sockaddr_un path buffer truncates to 107 bytes on Linux.
    let path = Path::new(OsStr::from_bytes(&bytes[..bytes.len().min(107)]));
    match UnixStream::connect(path) {
        Ok(stream) => {
            let fd = stream.into_raw_fd();
            // std creates CLOEXEC sockets; preserve the original API's flags.
            unsafe { libc::fcntl(fd, libc::F_SETFD, 0) };
            fd
        }
        Err(error) => {
            if matches!(
                error.raw_os_error(),
                Some(libc::EMFILE | libc::ENFILE | libc::ENOMEM)
            ) {
                report_abort(59, c"Unable to open Unix socket");
            } else {
                unsafe {
                    _sway_abort(
                        c"[%s:%d] Unable to connect to %s".as_ptr(),
                        c"common/ipc-client.c".as_ptr(),
                        66,
                        socket_path,
                    );
                }
            }
            -1
        }
    }
}

/// # Safety
/// `fd` must be a live socket. The timeval has the original platform C layout.
#[no_mangle]
pub unsafe extern "C" fn ipc_set_recv_timeout(fd: i32, timeout: libc::timeval) -> bool {
    let result = unsafe {
        libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_RCVTIMEO,
            (&timeout as *const libc::timeval).cast(),
            size_of::<libc::timeval>() as libc::socklen_t,
        )
    };
    if result == -1 {
        let errno = io::Error::last_os_error().raw_os_error().unwrap_or(0);
        unsafe {
            _sway_log(
                sway_log_importance_t::SWAY_ERROR,
                c"[%s:%d] Failed to set ipc recv timeout: %s".as_ptr(),
                c"common/ipc-client.c".as_ptr(),
                73,
                libc::strerror(errno),
            );
        }
        return false;
    }
    true
}

/// # Safety
/// `fd` must be a connected Unix socket. The result is owned by the caller.
#[no_mangle]
pub unsafe extern "C" fn ipc_recv_response(fd: i32) -> *mut IpcResponse {
    let Some(frame) = (unsafe { receive(fd) }) else {
        return ptr::null_mut();
    };
    let payload = c_string(&frame.payload);
    if payload.is_null() {
        allocation_failed();
        return ptr::null_mut();
    }
    let response = c_value(IpcResponse {
        size: frame.payload.len() as u32,
        kind: frame.kind.into(),
        payload,
    });
    if response.is_null() {
        unsafe { libc::free(payload.cast()) };
        allocation_failed();
    }
    response
}

/// # Safety
/// A non-null response must come from ipc_recv_response and be freed once.
#[no_mangle]
pub unsafe extern "C" fn free_ipc_response(response: *mut IpcResponse) {
    if !response.is_null() {
        unsafe {
            libc::free((*response).payload.cast());
            libc::free(response.cast());
        }
    }
}

/// # Safety
/// `fd` is connected, `len` is writable, and `payload` contains *len bytes.
/// The returned buffer must be freed with the C allocator.
#[no_mangle]
pub unsafe extern "C" fn ipc_single_command(
    fd: i32,
    kind: u32,
    payload: *const c_char,
    len: *mut u32,
) -> *mut c_char {
    let mut stream = match unsafe { stream(fd) } {
        Ok(stream) => stream,
        Err(_) => {
            report_abort(135, c"Unable to send IPC header");
            return ptr::null_mut();
        }
    };
    let length = unsafe { *len } as usize;
    // C write(fd, NULL, 0) is valid; Rust slices always need a non-null pointer.
    let bytes = if length == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(payload.cast(), length) }
    };
    if let Err(error) = (Frame {
        kind: kind.into(),
        payload: bytes,
    })
    .write_to(&mut stream)
    {
        // Rust's socket writer uses MSG_NOSIGNAL. The legacy API used write,
        // which also delivered SIGPIPE to the calling thread on EPIPE.
        if error.source.kind() == io::ErrorKind::BrokenPipe {
            unsafe { libc::raise(libc::SIGPIPE) };
        }
        match error.stage {
            Stage::Header => report_abort(135, c"Unable to send IPC header"),
            Stage::Payload => report_abort(139, c"Unable to send IPC payload"),
        }
        return ptr::null_mut();
    }
    let Some(frame) = receive_from(&mut stream) else {
        return ptr::null_mut();
    };
    unsafe { *len = frame.payload.len() as u32 };
    let response = c_string(&frame.payload);
    if response.is_null() {
        allocation_failed();
    }
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{os::fd::AsRawFd, thread};

    #[test]
    fn empty_null_payload_and_caller_fd_ownership_are_preserved() {
        let (client, mut peer) = UnixStream::pair().unwrap();
        let server = thread::spawn(move || {
            assert_eq!(
                Frame::read_from(&mut peer).unwrap(),
                Frame {
                    kind: crate::ipc::MessageType::GET_BAR_CONFIG,
                    payload: vec![]
                }
            );
            Frame {
                kind: crate::ipc::MessageType::GET_BAR_CONFIG,
                payload: b"\xff\0bytes",
            }
            .write_to(&mut peer)
            .unwrap();
        });
        let mut length = 0;
        // SAFETY: the descriptor is live, length writable, and NULL is allowed
        // for an empty payload. Only the returned C allocation is freed.
        unsafe {
            let result = ipc_single_command(client.as_raw_fd(), 6, ptr::null(), &mut length);
            assert!(!result.is_null());
            assert_eq!(
                slice::from_raw_parts(result.cast::<u8>(), length as usize),
                b"\xff\0bytes"
            );
            libc::free(result.cast());
        }
        assert!(client.peer_addr().is_ok());
        server.join().unwrap();
    }
}
