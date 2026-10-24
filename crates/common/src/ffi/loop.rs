//! Owned storage for the legacy, single-threaded poll loop.
//!
//! The C header exposes only opaque handles. Registrations are owned by the
//! loop; descriptors and callback data remain borrowed from its caller.
#![deny(unsafe_op_in_unsafe_fn)]

use std::{cell::RefCell, ffi::c_void, ptr};

type FdCallback = Option<unsafe extern "C" fn(i32, i16, *mut c_void)>;
type TimerCallback = Option<unsafe extern "C" fn(*mut c_void)>;

#[derive(Clone, Copy)]
struct Registration {
    descriptor: libc::pollfd,
    callback: FdCallback,
    data: *mut c_void,
}

/// Opaque C handle. Interior mutability permits callbacks to change registrations.
#[derive(Default)]
pub struct EventLoop {
    state: RefCell<State>,
}

#[derive(Default)]
struct State {
    registrations: Vec<Registration>,
    // Box keeps the opaque handle stable when the collection grows or shrinks.
    #[allow(clippy::vec_box)]
    timers: Vec<Box<Timer>>,
}

/// Opaque C timer handle, owned and released by its loop.
pub struct Timer {
    callback: TimerCallback,
    data: *mut c_void,
    expiry: Timestamp,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Timestamp {
    seconds: i64,
    nanoseconds: i64,
}

impl Timestamp {
    fn now() -> Self {
        let mut time = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        // SAFETY: time is writable and correctly aligned for clock_gettime.
        unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut time) };
        Self {
            seconds: time.tv_sec,
            nanoseconds: time.tv_nsec,
        }
    }

    fn after_millis(self, millis: i32) -> Self {
        let mut seconds = self.seconds + i64::from(millis / 1000);
        let mut nanoseconds = self.nanoseconds + i64::from(millis % 1000) * 1_000_000;
        if nanoseconds >= 1_000_000_000 {
            seconds += 1;
            nanoseconds -= 1_000_000_000;
        }
        // Preserve the C API's unnormalized negative subsecond delays. Instant
        // arithmetic would change their expiration ordering near a second edge.
        Self {
            seconds,
            nanoseconds,
        }
    }

    fn millis_since(self, now: Self) -> i32 {
        // The original narrows the seconds term before adding the nanos term.
        let seconds = ((self.seconds - now.seconds) * 1000) as i32;
        seconds.wrapping_add(((self.nanoseconds - now.nanoseconds) / 1_000_000) as i32)
    }
}

impl EventLoop {
    fn poll_descriptors(&self) {
        let mut state = self.state.borrow_mut();
        let timeout = if state.timers.is_empty() {
            i32::MAX
        } else {
            let now = Timestamp::now();
            state
                .timers
                .iter()
                .map(|timer| timer.expiry.millis_since(now))
                .min()
                .unwrap_or(i32::MAX)
                .max(0)
        };
        let mut descriptors: Vec<_> = state
            .registrations
            .iter()
            .map(|registration| registration.descriptor)
            .collect();
        // SAFETY: the vector holds exactly len initialized pollfd entries.
        // No callbacks run while poll or the state borrow is active. As in C,
        // errors (including EINTR) are left for the next iteration to handle.
        unsafe {
            libc::poll(
                descriptors.as_mut_ptr(),
                descriptors.len() as libc::nfds_t,
                timeout,
            )
        };
        for (registration, descriptor) in state.registrations.iter_mut().zip(descriptors) {
            registration.descriptor = descriptor;
        }
    }

    fn remove_timer(&self, timer: *mut Timer) -> bool {
        let mut state = self.state.borrow_mut();
        let Some(index) = state
            .timers
            .iter()
            .position(|entry| ptr::eq(entry.as_ref(), timer))
        else {
            return false;
        };
        state.timers.remove(index);
        true
    }
}

#[no_mangle]
pub extern "C" fn loop_create() -> *mut EventLoop {
    Box::into_raw(Box::default())
}

/// # Safety
/// `handle` must come from loop_create, be live, and have no active poll or callback.
#[no_mangle]
pub unsafe extern "C" fn loop_destroy(handle: *mut EventLoop) {
    // SAFETY: the caller transfers back the unique allocation from loop_create.
    drop(unsafe { Box::from_raw(handle) });
}

/// # Safety
/// `handle` must be live, used only on this thread, and stay live through callbacks.
/// Registered callbacks and their data must remain valid until removed.
#[no_mangle]
pub unsafe extern "C" fn loop_poll(handle: *mut EventLoop) {
    // SAFETY: the caller guarantees a live loop. Only shared references are held
    // across callbacks; RefCell borrows are released before invoking user code.
    let event_loop = unsafe { &*handle };
    event_loop.poll_descriptors();
    let mut index = 0;
    loop {
        let registration = event_loop.state.borrow().registrations.get(index).copied();
        let Some(Registration {
            descriptor,
            callback,
            data,
        }) = registration
        else {
            break;
        };
        if descriptor.revents & (descriptor.events | libc::POLLHUP | libc::POLLERR) != 0 {
            // SAFETY: registration validity is the caller's contract.
            unsafe {
                callback.expect("non-null function pointer")(
                    descriptor.fd,
                    descriptor.revents,
                    data,
                )
            };
        }
        // Deliberately advance even if a callback removed this registration:
        // the original C iteration skips the entry shifted into its place.
        index += 1;
    }

    if event_loop.state.borrow().timers.is_empty() {
        return;
    }
    let now = Timestamp::now();
    let mut index = 0;
    loop {
        let timer = event_loop.state.borrow().timers.get(index).map(|timer| {
            (
                ptr::from_ref(timer.as_ref()).cast_mut(),
                timer.expiry,
                timer.callback,
                timer.data,
            )
        });
        let Some((handle, expiry, callback, data)) = timer else {
            break;
        };
        if expiry < now {
            // SAFETY: callback and data remain valid by the registration contract.
            unsafe { callback.expect("non-null function pointer")(data) };
            event_loop.remove_timer(handle);
            // Revisit the same index, including when a callback removed a timer.
        } else {
            index += 1;
        }
    }
}

/// # Safety
/// `handle` must be live and used only on this thread. The callback and its data
/// must remain valid until removal; the caller retains ownership of `fd`.
#[no_mangle]
pub unsafe extern "C" fn loop_add_fd(
    handle: *mut EventLoop,
    fd: i32,
    mask: i16,
    callback: FdCallback,
    data: *mut c_void,
) {
    // SAFETY: the caller guarantees a live loop; callbacks may reenter this API.
    let event_loop = unsafe { &*handle };
    event_loop
        .state
        .borrow_mut()
        .registrations
        .push(Registration {
            descriptor: libc::pollfd {
                fd,
                events: mask,
                revents: 0,
            },
            callback,
            data,
        });
}

/// # Safety
/// `handle` must be live and used only on this thread. The callback and its data
/// must remain valid until the timer fires or is removed.
#[no_mangle]
pub unsafe extern "C" fn loop_add_timer(
    handle: *mut EventLoop,
    millis: i32,
    callback: TimerCallback,
    data: *mut c_void,
) -> *mut Timer {
    // SAFETY: the caller guarantees a live loop.
    let event_loop = unsafe { &*handle };
    let mut timer = Box::new(Timer {
        callback,
        data,
        expiry: Timestamp::now().after_millis(millis),
    });
    let handle = ptr::from_mut(timer.as_mut());
    event_loop.state.borrow_mut().timers.push(timer);
    handle
}

/// # Safety
/// `handle` must be live and used only on this thread.
#[no_mangle]
pub unsafe extern "C" fn loop_remove_fd(handle: *mut EventLoop, fd: i32) -> bool {
    // SAFETY: the caller guarantees a live loop.
    let event_loop = unsafe { &*handle };
    let mut state = event_loop.state.borrow_mut();
    let Some(index) = state
        .registrations
        .iter()
        .position(|entry| entry.descriptor.fd == fd)
    else {
        return false;
    };
    state.registrations.remove(index);
    true
}

/// # Safety
/// `handle` must be live and used only on this thread. `timer` is an opaque
/// identity, never dereferenced, and may already have been removed.
#[no_mangle]
pub unsafe extern "C" fn loop_remove_timer(handle: *mut EventLoop, timer: *mut Timer) -> bool {
    // SAFETY: the caller guarantees a live loop.
    unsafe { &*handle }.remove_timer(timer)
}

#[cfg(test)]
mod tests {
    use super::Timestamp;

    #[test]
    fn preserves_c_deadline_arithmetic() {
        let now = Timestamp {
            seconds: 10,
            nanoseconds: 900_000_000,
        };
        assert_eq!(now.after_millis(200).seconds, 11);
        assert_eq!(now.after_millis(200).nanoseconds, 100_000_000);
        let negative = Timestamp {
            seconds: 10,
            nanoseconds: 100_000_000,
        }
        .after_millis(-200);
        assert_eq!(negative.seconds, 10);
        assert_eq!(negative.nanoseconds, -100_000_000);
        assert_eq!(now.after_millis(200).millis_since(now), 200);
        // Truncate the subsecond difference toward zero, as C does.
        assert_eq!(
            Timestamp {
                seconds: 11,
                nanoseconds: 100_100_000
            }
            .millis_since(now),
            201
        );
    }
}
