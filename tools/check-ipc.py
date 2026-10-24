#!/usr/bin/env python3
"""Compare wire requests, output bytes, and exit codes against mock IPC peers."""
from pathlib import Path
import json
import os
import socket
import struct
import subprocess
import tempfile
import threading

root = Path(__file__).resolve().parents[1]
translated_dir = Path(os.environ.get('SWAY_PARITY_BIN_DIR', root / 'target/debug'))

def receive(peer, size):
    data = b''
    while len(data) < size:
        chunk = peer.recv(size - len(data))
        if not chunk:
            raise EOFError('short IPC message')
        data += chunk
    return data

def run(binary, options, payload):
    with tempfile.TemporaryDirectory(prefix='sway-ipc-') as temp:
        path = temp + '/socket'
        listener = socket.socket(socket.AF_UNIX)
        listener.bind(path)
        listener.listen()
        listener.settimeout(5)
        requests = []
        errors = []
        def serve():
            try:
                with listener.accept()[0] as peer:
                    peer.settimeout(5)
                    header = receive(peer, 14)
                    magic, length, kind = struct.unpack('=6sII', header)
                    assert magic == b'i3-ipc'
                    requests.append((kind, receive(peer, length)))
                    data = payload if isinstance(payload, bytes) else json.dumps(payload).encode()
                    # Exercise partial reads as well as IPC framing.
                    reply = struct.pack('=6sII', b'i3-ipc', len(data), kind) + data
                    for offset in range(0, len(reply), 3):
                        peer.sendall(reply[offset:offset + 3])
            except BaseException as error:
                errors.append(error)
        thread = threading.Thread(target=serve, daemon=True)
        thread.start()
        result = subprocess.run(['swaymsg', '-s', path, *options], executable=binary,
                                capture_output=True, timeout=10)
        thread.join(6)
        listener.close()
        assert not thread.is_alive() and not errors, errors
        return (requests, result.returncode, result.stdout, result.stderr)

cases = [
    ([], [{'success': True}]),
    (['-p', 'workspace', '2'], [{'success': True}]),
    (['-p', 'invalid command'], [{'success': False, 'error': 'Unknown command', 'parse_error': True}]),
    (['-q', 'invalid'], [{'success': False}]),
    (['-t', 'get_marks'], ['one', 'two']),
    (['-t', 'get_binding_modes'], ['default', 'resize']),
    (['-t', 'get_binding_state'], {'name': 'default'}),
    (['-t', 'get_workspaces'], []),
    (['-t', 'get_outputs'], []),
    (['-t', 'get_inputs'], []),
    (['-t', 'get_seats'], []),
    (['-t', 'get_tree'], {'id': 1, 'type': 'root', 'nodes': []}),
    (['-t', 'get_config'], {'config': 'set $mod Mod4\n'}),
    (['-t', 'get_bar_config'], ['bar-0']),
    (['-t', 'get_version'], {'major': 1, 'minor': 13, 'patch': 0, 'human_readable': '1.13-dev'}),
    (['-t', 'send_tick', 'test payload'], {'success': True}),
    (['-r'], b'{invalid json'),
]
for options, payload in cases:
    reference = run(root / 'build/reference/swaymsg/swaymsg', options, payload)
    translated = run(translated_dir / 'swaymsg', options, payload)
    assert reference == translated, (options, reference, translated)
print(f'{len(cases)} IPC comparisons passed')
