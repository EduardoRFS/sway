#!/usr/bin/env python3
"""Compare compositor state and IPC replies in isolated headless sessions."""
from pathlib import Path
import json
import os
import socket
import struct
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
CONFIG = '''xwayland disable
font monospace 10
output HEADLESS-1 mode 1280x720 position 0 0
output HEADLESS-2 mode 1280x720 position 1280 0
seat seat0 fallback true
focus_follows_mouse no
gaps inner 5
bar {
    id parity
    swaybar_command true
    status_command true
}
'''

def receive(peer, size):
    data = b''
    while len(data) < size:
        part = peer.recv(size - len(data))
        if not part:
            raise EOFError('short IPC reply')
        data += part
    return data

def request(path, kind, payload=''):
    data = payload.encode()
    with socket.socket(socket.AF_UNIX) as peer:
        peer.settimeout(5)
        peer.connect(str(path))
        peer.sendall(struct.pack('=6sII', b'i3-ipc', len(data), kind) + data)
        magic, length, reply_kind = struct.unpack('=6sII', receive(peer, 14))
        assert magic == b'i3-ipc' and reply_kind == kind
        return json.loads(receive(peer, length))

def canonical(value):
    if isinstance(value, dict):
        return {key: '<client-pid>' if key == 'pid' else
                '<random-id>' if key == 'foreign_toplevel_identifier' else canonical(item)
                for key, item in value.items()}
    if isinstance(value, list):
        return [canonical(item) for item in value]
    return value

def has_window(value, title):
    if isinstance(value, dict):
        return (value.get('app_id') == 'parity-client' and value.get('name') == title
                or any(has_window(item, title) for item in value.values()))
    return isinstance(value, list) and any(has_window(item, title) for item in value)

def snapshot(binary):
    with tempfile.TemporaryDirectory(prefix='sway-headless-') as temp:
        directory = Path(temp)
        config = directory / 'config'
        config.write_text(CONFIG)
        env = {k: v for k, v in os.environ.items()
               if k not in ('SWAYSOCK', 'I3SOCK', 'DISPLAY', 'WAYLAND_DISPLAY')}
        env.update(XDG_RUNTIME_DIR=temp, WLR_BACKENDS='headless',
                   WLR_HEADLESS_OUTPUTS='2', WLR_RENDERER='pixman')
        with (directory / 'log').open('w+') as log:
            process = subprocess.Popen([str(binary), '-c', str(config)], env=env,
                                       stdout=log, stderr=log)
            windows = []
            try:
                deadline = time.monotonic() + 15
                path = None
                while time.monotonic() < deadline and process.poll() is None:
                    sockets = list(directory.glob('sway-ipc.*.sock'))
                    if sockets:
                        path = sockets[0]
                        break
                    time.sleep(.05)
                if path is None:
                    log.seek(0)
                    raise AssertionError(f'{binary} did not start:\n{log.read()}')
                display = next(p.name for p in directory.glob('wayland-*') if p.is_socket())
                client_env = dict(env, SWAYSOCK=str(path), WAYLAND_DISPLAY=display)
                for title in ('parity-one', 'parity-two'):
                    window = subprocess.Popen(['foot', '-c', '/dev/null', '--app-id', 'parity-client',
                                               '--title', title, 'sleep', '30'],
                                              env=client_env, stdout=log, stderr=log)
                    windows.append(window)
                    deadline = time.monotonic() + 10
                    while time.monotonic() < deadline:
                        if has_window(request(path, 4), title):
                            break
                        if window.poll() is not None:
                            log.seek(0)
                            raise AssertionError(f'test window failed:\n{log.read()}')
                        time.sleep(.05)
                    else:
                        raise AssertionError(f'test window {title} did not map')
                time.sleep(.1)
                transcript = []
                def queries():
                    for kind in (1, 3, 4, 5, 6, 7, 8, 9, 12, 100, 101):
                        result = request(path, kind)
                        if kind == 7:
                            result['loaded_config_file_name'] = '<config>'
                        transcript.append((kind, canonical(result)))
                queries()
                commands = [
                    '[title="parity-one"] focus', 'mark first',
                    'floating enable', 'resize set width 400 px height 250 px',
                    'move position 60 px 80 px', 'sticky enable',
                    'fullscreen enable', 'fullscreen disable',
                    'border pixel 3', 'opacity 0.8', 'title_format "%title"',
                    'move scratchpad', 'scratchpad show', 'floating disable',
                    '[title="parity-two"] focus', 'layout tabbed',
                    'split vertical', 'layout toggle split',
                    'swap container with mark first',
                    'workspace 2',
                    'move workspace to output HEADLESS-2',
                    'rename workspace 2 to "2:translated"',
                    'workspace 3', 'workspace back_and_forth',
                    'gaps inner all set 9', 'gaps outer all set 4',
                    'input type:keyboard repeat_delay 400',
                    'input type:keyboard repeat_rate 30',
                    'seat seat0 hide_cursor 1000',
                    'bar mode hide parity', 'bar hidden_state show parity',
                    'output HEADLESS-1 scale 1.25',
                    'output HEADLESS-2 transform 90',
                    'set $parity value', 'nop translation parity',
                    'not_a_command',
                ]
                for command in commands:
                    transcript.append((command, request(path, 0, command)))
                time.sleep(.1)
                queries()
                transcript.append(('bar', request(path, 6, 'parity')))
                # Reading configuration must return exactly the original text.
                assert request(path, 9)['config'] == CONFIG
                # Exercise layer-shell setup, shared buffers, text rendering,
                # IPC subscriptions, and event loops in the companion clients.
                if binary.parent.name == 'sway' and binary.parent.parent.name == 'reference':
                    programs = ROOT / 'build/reference'
                    bar_binary = programs / 'swaybar/swaybar'
                    nag_binary = programs / 'swaynag/swaynag'
                else:
                    bar_binary = binary.parent / 'swaybar'
                    nag_binary = binary.parent / 'swaynag'
                clients = []
                try:
                    for args in ([str(bar_binary), '-b', 'parity'],
                                 [str(nag_binary), '-m', 'Translation parity', '-b', 'Dismiss', 'true']):
                        clients.append(subprocess.Popen(args, env=client_env, stdout=log, stderr=log))
                    time.sleep(.5)
                    for client in clients:
                        if client.poll() is not None:
                            log.seek(0)
                            raise AssertionError(f'Wayland client exited: {client.args}\n{log.read()}')
                    transcript.append(('layer-shell clients', 'running'))
                finally:
                    for client in clients:
                        if client.poll() is None:
                            client.terminate()
                        client.wait(timeout=5)
                return transcript
            finally:
                for window in windows:
                    if window.poll() is None:
                        window.terminate()
                    window.wait(timeout=5)
                if process.poll() is None:
                    try:
                        if path:
                            request(path, 0, 'exit')
                    except (OSError, EOFError):
                        pass
                    try:
                        process.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        process.kill()
                        process.wait()

reference = snapshot(ROOT / 'build/reference/sway/sway')
translated = snapshot(Path(os.environ.get('SWAY_PARITY_BIN_DIR', ROOT / 'target/debug')) / 'sway')
def compare(expected, actual, path=''):
    if isinstance(expected, dict) and isinstance(actual, dict) and expected.keys() == actual.keys():
        for key in expected:
            compare(expected[key], actual[key], path + '/' + key)
    elif isinstance(expected, (list, tuple)) and isinstance(actual, (list, tuple)) and len(expected) == len(actual):
        for index, (left, right) in enumerate(zip(expected, actual)):
            compare(left, right, path + '/' + str(index))
    else:
        assert expected == actual, (path, expected, actual)
for expected, actual in zip(reference, translated, strict=True):
    compare(expected, actual, str(expected[0]))
print(f'{len(reference)} headless compositor comparisons passed')
