//! Compatibility with gesture.h. Raw pointers and C allocation stop here.
#![deny(unsafe_op_in_unsafe_fn)]

use super::{
    allocation::{c_string, c_value},
    log::{_sway_assert, _sway_log, sway_log_importance_t},
};
use crate::gesture::{Gesture, Kind, Tracker};
use std::{
    ffi::{c_char, CStr, CString},
    ptr,
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RawGesture {
    pub kind: u32,
    pub fingers: u8,
    pub directions: u32,
}

impl From<RawGesture> for Gesture {
    fn from(raw: RawGesture) -> Self {
        Self {
            kind: raw.kind.into(),
            fingers: raw.fingers,
            directions: raw.directions,
        }
    }
}

impl From<Gesture> for RawGesture {
    fn from(gesture: Gesture) -> Self {
        Self {
            kind: gesture.kind.as_raw(),
            fingers: gesture.fingers,
            directions: gesture.directions,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RawTracker {
    pub kind: u32,
    pub fingers: u8,
    pub dx: f64,
    pub dy: f64,
    pub scale: f64,
    pub rotation: f64,
}

impl From<RawTracker> for Tracker {
    fn from(raw: RawTracker) -> Self {
        Self {
            kind: raw.kind.into(),
            fingers: raw.fingers,
            dx: raw.dx,
            dy: raw.dy,
            scale: raw.scale,
            rotation: raw.rotation,
        }
    }
}

impl From<Tracker> for RawTracker {
    fn from(tracker: Tracker) -> Self {
        Self {
            kind: tracker.kind.as_raw(),
            fingers: tracker.fingers,
            dx: tracker.dx,
            dy: tracker.dy,
            scale: tracker.scale,
            rotation: tracker.rotation,
        }
    }
}

#[no_mangle]
pub static GESTURE_FINGERS_ANY: u8 = 0;

// SAFETY for the exported functions: the translated callers provide valid,
// initialized pointers with the layouts from gesture.h, and exclusive access
// to outputs/trackers. No references are retained across calls.
/// # Safety
/// input must be NUL-terminated; output must be aligned, writable storage.
/// The returned error string is owned by the caller and freed with C free.
#[no_mangle]
pub unsafe extern "C" fn gesture_parse(
    input: *const c_char,
    output: *mut RawGesture,
) -> *mut c_char {
    match Gesture::parse(unsafe { CStr::from_ptr(input) }.to_bytes()) {
        Ok(gesture) => {
            unsafe { output.write(gesture.into()) };
            ptr::null_mut()
        }
        Err(error) => {
            unsafe { output.write(error.partial.into()) };
            c_string(&error.message)
        }
    }
}

#[no_mangle]
pub extern "C" fn gesture_type_string(kind: u32) -> *const c_char {
    match Kind::from(kind) {
        Kind::None => c"none".as_ptr(),
        Kind::Hold => c"hold".as_ptr(),
        Kind::Pinch => c"pinch".as_ptr(),
        Kind::Swipe => c"swipe".as_ptr(),
        Kind::Unknown(_) => ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn gesture_direction_string(direction: u32) -> *const c_char {
    match direction {
        0 => c"none".as_ptr(),
        1 => c"up".as_ptr(),
        2 => c"down".as_ptr(),
        4 => c"left".as_ptr(),
        8 => c"right".as_ptr(),
        16 => c"inward".as_ptr(),
        32 => c"outward".as_ptr(),
        64 => c"clockwise".as_ptr(),
        128 => c"counterclockwise".as_ptr(),
        _ => ptr::null(),
    }
}

/// # Safety
/// gesture must point to an initialized RawGesture. The returned string
/// is owned by the caller and freed with C free.
#[no_mangle]
pub unsafe extern "C" fn gesture_to_string(gesture: *mut RawGesture) -> *mut c_char {
    c_string(Gesture::from(unsafe { *gesture }).to_string().as_bytes())
}

/// # Safety
/// target must point to an initialized RawGesture for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn gesture_check(target: *mut RawGesture, kind: u32, fingers: u8) -> bool {
    Gesture::from(unsafe { *target }).accepts(kind.into(), fingers)
}

/// # Safety
/// target and input must point to initialized RawGesture values.
#[no_mangle]
pub unsafe extern "C" fn gesture_match(
    target: *mut RawGesture,
    input: *mut RawGesture,
    exact: bool,
) -> bool {
    Gesture::from(unsafe { *target }).matches(&Gesture::from(unsafe { *input }), exact)
}

/// # Safety
/// left and right must point to initialized RawGesture values.
#[no_mangle]
pub unsafe extern "C" fn gesture_equal(left: *mut RawGesture, right: *mut RawGesture) -> bool {
    Gesture::from(unsafe { *left }) == Gesture::from(unsafe { *right })
}

/// # Safety
/// left and right must point to initialized RawGesture values.
#[no_mangle]
pub unsafe extern "C" fn gesture_compare(left: *mut RawGesture, right: *mut RawGesture) -> i8 {
    Gesture::from(unsafe { *left }).specificity() as i8
        - Gesture::from(unsafe { *right }).specificity() as i8
}

/// # Safety
/// raw must point to aligned, exclusively writable RawTracker storage.
#[no_mangle]
pub unsafe extern "C" fn gesture_tracker_begin(raw: *mut RawTracker, kind: u32, fingers: u8) {
    let mut tracker = Tracker::default();
    tracker.begin(kind.into(), fingers);
    unsafe {
        raw.write(tracker.into());
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            c"[%s:%d] begin tracking %s:%u:? gesture".as_ptr(),
            c"common/gesture.c".as_ptr(),
            235,
            gesture_type_string(kind),
            i32::from(fingers),
        );
    }
}

/// # Safety
/// raw must point to an initialized RawTracker for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn gesture_tracker_check(raw: *mut RawTracker, kind: u32) -> bool {
    unsafe { (*raw).kind == kind }
}

/// # Safety
/// raw must point to an initialized, exclusively borrowed RawTracker.
#[no_mangle]
pub unsafe extern "C" fn gesture_tracker_update(
    raw: *mut RawTracker,
    dx: f64,
    dy: f64,
    scale: f64,
    rotation: f64,
) {
    let mut tracker = Tracker::from(unsafe { *raw });
    if !tracker.update(dx, dy, scale, rotation) {
        unsafe {
            _sway_assert(false, c"[%s:%d] %s:hold does not update.".as_ptr(), c"common/gesture.c".as_ptr(),
                         246, c"void gesture_tracker_update(struct gesture_tracker *, double, double, double, double)".as_ptr());
        }
        return;
    }
    unsafe {
        raw.write(tracker.into());
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            c"[%s:%d] update tracking %s:%u:? gesture: %f %f %f %f".as_ptr(),
            c"common/gesture.c".as_ptr(),
            258,
            gesture_type_string(tracker.kind.as_raw()),
            i32::from(tracker.fingers),
            tracker.dx,
            tracker.dy,
            tracker.scale,
            tracker.rotation,
        );
    }
}

/// # Safety
/// raw must point to an initialized, exclusively borrowed RawTracker.
#[no_mangle]
pub unsafe extern "C" fn gesture_tracker_cancel(raw: *mut RawTracker) {
    let tracker = unsafe { &mut *raw };
    unsafe {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            c"[%s:%d] cancel tracking %s:%u:? gesture".as_ptr(),
            c"common/gesture.c".as_ptr(),
            266,
            gesture_type_string(tracker.kind),
            i32::from(tracker.fingers),
        );
    }
    tracker.kind = 0;
}

/// # Safety
/// raw must point to an initialized, exclusively borrowed RawTracker.
/// The returned gesture is owned by the caller and freed with C free.
#[no_mangle]
pub unsafe extern "C" fn gesture_tracker_end(raw: *mut RawTracker) -> *mut RawGesture {
    let mut tracker = Tracker::from(unsafe { *raw });
    let Some(gesture) = tracker.end() else {
        unsafe {
            _sway_assert(
                false,
                c"[%s:%d] %s:Not tracking any gesture.".as_ptr(),
                c"common/gesture.c".as_ptr(),
                318,
                c"struct gesture *gesture_tracker_end(struct gesture_tracker *)".as_ptr(),
            );
        }
        return c_value(Gesture::default().into());
    };
    let description = CString::new(gesture.to_string()).expect("gesture names contain no NUL");
    unsafe {
        _sway_log(
            sway_log_importance_t::SWAY_DEBUG,
            c"[%s:%d] end tracking gesture: %s".as_ptr(),
            c"common/gesture.c".as_ptr(),
            326,
            description.as_ptr(),
        );
        raw.write(tracker.into());
    }
    c_value(gesture.into())
}
