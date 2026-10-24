#!/usr/bin/env python3
"""Compare owned Rust event-loop behavior with the unchanged C implementation."""
import contextlib
import ctypes as c
import os
from pathlib import Path
import random
import select
import signal
import socket
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
BUILD = ROOT / 'build'
BUILD.mkdir(exist_ok=True)
subprocess.run(['cc', '-shared', '-fPIC', '-O2', '-I' + str(ROOT / 'original/include'),
                *[str(ROOT / 'original/common' / (name + '.c')) for name in ('loop', 'list', 'log')],
                '-o', str(BUILD / 'libloop-reference.so')], check=True)
FD_CALLBACK = c.CFUNCTYPE(None, c.c_int, c.c_short, c.c_void_p)
TIMER_CALLBACK = c.CFUNCTYPE(None, c.c_void_p)
LIBRARIES = [c.CDLL(str(BUILD / 'libloop-reference.so')),
             c.CDLL(os.environ.get('SWAY_COMMON_LIBRARY', ROOT / 'target/debug/libsway_common.so'))]
for library in LIBRARIES:
    for name, args, result in (
        ('loop_create', [], c.c_void_p),
        ('loop_destroy', [c.c_void_p], None),
        ('loop_poll', [c.c_void_p], None),
        ('loop_add_fd', [c.c_void_p, c.c_int, c.c_short, FD_CALLBACK, c.c_void_p], None),
        ('loop_remove_fd', [c.c_void_p, c.c_int], c.c_bool),
        ('loop_add_timer', [c.c_void_p, c.c_int, TIMER_CALLBACK, c.c_void_p], c.c_void_p),
        ('loop_remove_timer', [c.c_void_p, c.c_void_p], c.c_bool),
    ):
        function = getattr(library, name)
        function.argtypes, function.restype = args, result


class Loop:
    def __init__(self, library):
        self.library = library
        self.handle = library.loop_create()
        assert self.handle
        self.events = []
        self.callbacks = []
        self.errors = []
        self.resources = contextlib.ExitStack()
        self.descriptors = {}

    def __enter__(self):
        return self

    def __exit__(self, *exc):
        self.library.loop_destroy(self.handle)
        # Destruction must not close caller-owned descriptors.
        for fd in self.descriptors:
            os.fstat(fd)
        self.resources.close()
        assert not self.errors, self.errors

    def ready_fd(self):
        reader, writer = socket.socketpair()
        self.resources.enter_context(reader)
        self.resources.enter_context(writer)
        writer.sendall(b'ready')
        fd = reader.fileno()
        self.descriptors[fd] = len(self.descriptors)
        return fd

    def callback(self, kind, name, action):
        def invoke(*args):
            if kind == 'fd':
                fd, mask, data = args
                self.events.append((name, self.descriptors.get(fd, 'invalid'), mask, data))
            else:
                self.events.append((name, args[0]))
            try:
                if action:
                    action()
            except BaseException as error:
                self.errors.append(repr(error))
        callback = (FD_CALLBACK if kind == 'fd' else TIMER_CALLBACK)(invoke)
        self.callbacks.append(callback)  # Keep C callback trampolines alive.
        return callback

    def add_fd(self, fd, name, action=None, mask=select.POLLIN, data=0):
        self.library.loop_add_fd(self.handle, fd, mask, self.callback('fd', name, action), data)

    def timer(self, name, action=None, millis=-10_000, data=0):
        return self.library.loop_add_timer(self.handle, millis, self.callback('timer', name, action), data)

    def remove_fd(self, fd):
        return self.library.loop_remove_fd(self.handle, fd)

    def remove_timer(self, timer):
        return self.library.loop_remove_timer(self.handle, timer)

    def poll(self):
        self.library.loop_poll(self.handle)
        assert not self.errors, self.errors


SCENARIOS = []


def scenario(function):
    SCENARIOS.append(function)
    return function


@scenario
def registration_order(loop):
    fd = loop.ready_fd()
    loop.add_fd(fd, 'first', data=17)
    loop.add_fd(fd, 'duplicate', data=23)
    loop.add_fd(-1, 'ignored')
    loop.timer('overdue', data=19)
    loop.timer('future', millis=60_000)
    loop.poll()
    assert [event[0] for event in loop.events] == ['first', 'duplicate', 'overdue']
    assert loop.remove_fd(fd) and loop.remove_fd(fd) and not loop.remove_fd(fd)


@scenario
def remove_current_fd(loop):
    first, second = loop.ready_fd(), loop.ready_fd()
    loop.add_fd(first, 'remove-self', lambda: loop.remove_fd(first))
    loop.add_fd(second, 'shifted')
    loop.poll()
    assert [event[0] for event in loop.events] == ['remove-self']
    loop.poll()
    assert loop.events[-1][0] == 'shifted'


@scenario
def remove_next_and_append(loop):
    first, second, third = [loop.ready_fd() for _ in range(3)]
    def change():
        assert loop.remove_fd(second)
        loop.add_fd(second, 'appended')
    loop.add_fd(first, 'first', change)
    loop.add_fd(second, 'removed')
    loop.add_fd(third, 'third')
    loop.poll()
    assert [event[0] for event in loop.events] == ['first', 'third']


@scenario
def timers_mutate_registrations(loop):
    fd = loop.ready_fd()
    loop.add_fd(fd, 'descriptor')
    handles = {}
    def expire():
        assert loop.remove_timer(handles['self'])
        assert loop.remove_timer(handles['cancelled'])
        assert not loop.remove_timer(handles['cancelled'])
        assert loop.remove_fd(fd)
        loop.timer('new-overdue')
    handles['self'] = loop.timer('self', expire)
    handles['cancelled'] = loop.timer('cancelled')
    loop.timer('retained')
    loop.poll()
    assert [event[0] for event in loop.events] == ['descriptor', 'self', 'retained', 'new-overdue']


@scenario
def fd_callback_changes_timers(loop):
    handles = {}
    def change():
        assert loop.remove_timer(handles['cancelled'])
        loop.timer('added')
    loop.add_fd(loop.ready_fd(), 'fd', change)
    handles['cancelled'] = loop.timer('cancelled')
    loop.timer('remaining')
    loop.poll()
    assert [event[0] for event in loop.events] == ['fd', 'remaining', 'added']


@scenario
def timer_handle_stability(loop):
    handles = [loop.timer(str(index)) for index in range(512)]
    assert len(set(handles)) == len(handles)
    for handle in handles[::2]:
        assert loop.remove_timer(handle)
    for handle in handles[::2]:
        assert not loop.remove_timer(handle)
    loop.poll()
    assert [event[0] for event in loop.events] == [str(index) for index in range(1, 512, 2)]


@scenario
def descriptor_growth(loop):
    fd = loop.ready_fd()
    for index in range(128):
        loop.add_fd(fd, str(index))
    loop.poll()
    assert len(loop.events) == 128


@scenario
def nested_poll(loop):
    entered = False
    def reenter():
        nonlocal entered
        if not entered:
            entered = True
            loop.poll()
    loop.add_fd(loop.ready_fd(), 'first', reenter)
    loop.add_fd(loop.ready_fd(), 'second')
    loop.timer('timer')
    loop.poll()
    assert [event[0] for event in loop.events] == ['first', 'first', 'second', 'timer', 'second']


@scenario
def hangup_and_invalid(loop):
    reader, writer = socket.socketpair()
    loop.resources.enter_context(reader)
    loop.descriptors[reader.fileno()] = 0
    writer.close()
    loop.add_fd(reader.fileno(), 'hangup', mask=0)
    loop.add_fd(1_000_000, 'invalid-ignored')
    loop.add_fd(1_000_000, 'invalid-requested', mask=select.POLLNVAL)
    loop.poll()
    assert [event[0] for event in loop.events] == ['hangup', 'invalid-requested']
    assert loop.events[0][2] & select.POLLHUP


@scenario
def error_without_requested_mask(loop):
    reader, writer = os.pipe()
    os.close(reader)
    loop.resources.callback(os.close, writer)
    loop.descriptors[writer] = 0
    loop.add_fd(writer, 'error', mask=0)
    loop.poll()
    assert loop.events[0][2] & select.POLLERR


@scenario
def wait_for_timer(loop):
    loop.timer('wake', millis=10)
    start = time.monotonic()
    # Millisecond truncation can make poll return just before the deadline.
    while not loop.events:
        loop.poll()
    assert 0.005 <= time.monotonic() - start < 2
    assert loop.events == [('wake', None)]


@scenario
def cancelled_and_unknown_handles(loop):
    assert not loop.remove_timer(None)
    timer = loop.timer('cancelled')
    assert loop.remove_timer(timer)
    assert not loop.remove_timer(timer)
    loop.add_fd(loop.ready_fd(), 'wake')
    loop.poll()
    assert [event[0] for event in loop.events] == ['wake']


@scenario
def interrupted_poll(loop):
    loop.timer('not-yet', millis=60_000)
    previous = signal.signal(signal.SIGALRM, lambda *_: loop.events.append(('interrupted',)))
    start = time.monotonic()
    try:
        signal.setitimer(signal.ITIMER_REAL, 0.01)
        loop.poll()
    finally:
        signal.setitimer(signal.ITIMER_REAL, 0)
        signal.signal(signal.SIGALRM, previous)
        signal.alarm(10)
    assert time.monotonic() - start < 2
    assert loop.events == [('interrupted',)]


def randomized(seed):
    def exercise(loop):
        rng = random.Random(seed)
        descriptors = [loop.ready_fd() for _ in range(12)]
        for index, fd in enumerate(descriptors):
            target = rng.choice(descriptors)
            operation = rng.randrange(3)
            def action(target=target, operation=operation, index=index):
                if operation == 0:
                    loop.events.append(('removed', loop.remove_fd(target)))
                elif operation == 1:
                    loop.add_fd(target, 'added-' + str(index))
                else:
                    loop.timer('timer-' + str(index))
            loop.add_fd(fd, str(index), action)
        loop.poll()
    return exercise


# A regression must fail rather than block a build indefinitely in poll.
signal.signal(signal.SIGALRM, lambda *_: (_ for _ in ()).throw(TimeoutError('event-loop test hung')))
checks = [*SCENARIOS, *(randomized(seed) for seed in range(64))]
for index, exercise in enumerate(checks):
    results = []
    for library in LIBRARIES:
        signal.alarm(10)
        with Loop(library) as loop:
            exercise(loop)
            results.append(loop.events)
        signal.alarm(0)
    assert results[0] == results[1], (index, exercise.__name__, results)
print(f'{len(checks)} event-loop C/Rust comparisons passed')
