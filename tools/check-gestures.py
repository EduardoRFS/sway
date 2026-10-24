#!/usr/bin/env python3
"""Differentially check the gesture C ABI against the unchanged original C."""
import ctypes as c
import itertools
import math
import os
from pathlib import Path
import random
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
build = ROOT / 'build'
build.mkdir(exist_ok=True)
subprocess.run(['cc', '-shared', '-fPIC', '-O2', '-I' + str(ROOT / 'original/include'),
                *[str(ROOT / 'original/common' / (name + '.c'))
                  for name in ('gesture', 'list', 'stringop', 'log', 'ipc-client')],
                '-lm', '-o', str(build / 'libgesture-reference.so')], check=True)

class Gesture(c.Structure):
    _fields_ = [('kind', c.c_uint), ('fingers', c.c_ubyte), ('directions', c.c_uint)]

class Tracker(c.Structure):
    _fields_ = [('kind', c.c_uint), ('fingers', c.c_ubyte), ('dx', c.c_double),
                ('dy', c.c_double), ('scale', c.c_double), ('rotation', c.c_double)]

def values(value):
    return tuple('NaN' if isinstance(item := getattr(value, name), float) and math.isnan(item) else item
                 for name, _ in value._fields_)

libc = c.CDLL(None)
libc.free.argtypes = [c.c_void_p]
libraries = [c.CDLL(str(build / 'libgesture-reference.so')),
             c.CDLL(os.environ.get('SWAY_COMMON_LIBRARY', str(ROOT / 'target/debug/libsway_common.so')))]
prototypes = {
    'gesture_parse': ([c.c_char_p, c.POINTER(Gesture)], c.c_void_p),
    'gesture_to_string': ([c.POINTER(Gesture)], c.c_void_p),
    'gesture_type_string': ([c.c_uint], c.c_void_p),
    'gesture_direction_string': ([c.c_uint], c.c_void_p),
    'gesture_check': ([c.POINTER(Gesture), c.c_uint, c.c_ubyte], c.c_bool),
    'gesture_match': ([c.POINTER(Gesture), c.POINTER(Gesture), c.c_bool], c.c_bool),
    'gesture_equal': ([c.POINTER(Gesture), c.POINTER(Gesture)], c.c_bool),
    'gesture_compare': ([c.POINTER(Gesture), c.POINTER(Gesture)], c.c_int8),
    'gesture_tracker_begin': ([c.POINTER(Tracker), c.c_uint, c.c_ubyte], None),
    'gesture_tracker_update': ([c.POINTER(Tracker), c.c_double, c.c_double, c.c_double, c.c_double], None),
    'gesture_tracker_cancel': ([c.POINTER(Tracker)], None),
    'gesture_tracker_end': ([c.POINTER(Tracker)], c.POINTER(Gesture)),
}
for library in libraries:
    for name, (args, result) in prototypes.items():
        function = getattr(library, name)
        function.argtypes, function.restype = args, result

def allocated_string(pointer):
    if not pointer:
        return None
    result = c.string_at(pointer)
    libc.free(pointer)
    return result

count = 0
inputs = [b'', b':', b'::::', b'swipe:3:up:extra', b'hold:\xff', b'pinch:3:up+\xff']
for parts in itertools.product([b'hold', b'pinch', b'swipe', b'bad', b'none'],
                               [b'1', b'9', b'0', b'10', b'', b'any', b'up'],
                               [b'up', b'down+left', b'inward+clockwise', b'any', b'up+bad', b'+up++', b'++']):
    text = b':'.join(parts)
    inputs.extend((text, b'::' + text + b'::'))
for text in inputs:
    results = []
    for library in libraries:
        gesture = Gesture(99, 99, 99)
        error = allocated_string(library.gesture_parse(text, c.byref(gesture)))
        rendered = allocated_string(library.gesture_to_string(c.byref(gesture)))
        results.append((values(gesture), error, rendered))
    assert results[0] == results[1], (text, results)
    count += 1

rng = random.Random(42)
for _ in range(512):
    left = Gesture(rng.randrange(6), rng.randrange(10), rng.getrandbits(32))
    right = Gesture(rng.randrange(6), rng.randrange(10), rng.getrandbits(32))
    for name, args in [('gesture_check', (c.byref(left), right.kind, right.fingers)),
                       ('gesture_match', (c.byref(left), c.byref(right), False)),
                       ('gesture_match', (c.byref(left), c.byref(right), True)),
                       ('gesture_equal', (c.byref(left), c.byref(right))),
                       ('gesture_compare', (c.byref(left), c.byref(right)))]:
        results = [getattr(library, name)(*args) for library in libraries]
        assert results[0] == results[1], (name, values(left), values(right), results)
        count += 1
    results = [allocated_string(library.gesture_to_string(c.byref(left))) for library in libraries]
    assert results[0] == results[1], (values(left), results)
    count += 1

updates = [(0., 0., 1.1, 5.), (10., 1., .8, -6.), (-3., 3., 1.2, 20.),
           (math.nan, math.inf, math.nan, -math.inf)]
for kind, fingers, update in itertools.product((1, 2, 3, 5), (0, 3, 9, 255), updates):
    results = []
    for library in libraries:
        tracker = Tracker()
        library.gesture_tracker_begin(c.byref(tracker), kind, fingers)
        if kind != 1:  # Hold updates intentionally assert in the C original.
            library.gesture_tracker_update(c.byref(tracker), *update)
        result = library.gesture_tracker_end(c.byref(tracker))
        results.append((values(result.contents), values(tracker)))
        libc.free(result)
        library.gesture_tracker_cancel(c.byref(tracker))
        assert tracker.kind == 0
    assert results[0] == results[1], (kind, fingers, update, results)
    count += 1
print(f'{count} C/Rust gesture comparisons passed')

# The idiomatic Rust socket writer suppresses SIGPIPE internally; the C API
# must still retain write(2)'s signal behavior. Use isolated processes so the
# test runner's own SIGPIPE setting is not changed.
closed_peer = '''
import ctypes as c, signal, socket, sys
library = c.CDLL(sys.argv[1])
library.ipc_single_command.argtypes = [c.c_int, c.c_uint, c.c_char_p, c.POINTER(c.c_uint)]
client, peer = socket.socketpair()
peer.close()
signal.signal(signal.SIGPIPE, signal.SIG_DFL)
length = c.c_uint(0)
library.ipc_single_command(client.fileno(), 0, None, c.byref(length))
raise AssertionError('must terminate on SIGPIPE')
'''
statuses = [subprocess.run([sys.executable, '-c', closed_peer, library._name], capture_output=True).returncode
            for library in libraries]
assert statuses == [-13, -13], statuses
print('C/Rust IPC SIGPIPE behavior matches')
