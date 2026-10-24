#!/usr/bin/env python3
"""Compare command-line behavior of the translated programs with the C build."""
from pathlib import Path
import os
import re
import subprocess

root = Path(__file__).resolve().parents[1]
translated_dir = Path(os.environ.get('SWAY_PARITY_BIN_DIR', root / 'target/debug'))
env = dict(os.environ, SWAYSOCK='/nonexistent/sway-parity.sock', I3SOCK='/nonexistent/sway-parity.sock')

def normalized(data):
    return re.sub(rb'\d\d:\d\d:\d\d\.\d{3}', b'<time>', data)

count = 0
for name in ('sway', 'swaymsg', 'swaybar', 'swaynag'):
    for args in (['--help'], ['--version'], ['--not-an-option'], ['--help', b'\xff']):
        results = []
        for binary in (root / 'build/reference' / name / name, translated_dir / name):
            # argv[0] is observable (getopt error messages, usage text).
            result = subprocess.run([name, *args], executable=binary, env=env,
                                    capture_output=True, timeout=10)
            results.append((result.returncode, normalized(result.stdout), normalized(result.stderr)))
        assert results[0] == results[1], (name, args, results)
        count += 1
    statuses = []
    for binary in (root / 'build/reference' / name / name, translated_dir / name):
        read_fd, write_fd = os.pipe()
        os.close(read_fd)
        try:
            result = subprocess.run([name, '--version'], executable=binary, env=env,
                                    stdout=write_fd, stderr=subprocess.PIPE, timeout=10)
            statuses.append(result.returncode)
        finally:
            os.close(write_fd)
    assert statuses == [-13, -13], (name, 'SIGPIPE', statuses)
    count += 1
print(f'{count} CLI comparisons passed')
