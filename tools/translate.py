#!/usr/bin/env python3
"""Translate the exact Meson compilation units, preserving their C ABI."""
import json
import argparse
import os
from pathlib import Path
import shlex
import subprocess
from finalize import finalize

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output-dir', type=Path, default=ROOT / 'build/retranslated',
                    help='Fresh comparison scaffold; the maintained Rust workspace is not overwritten')
args = parser.parse_args()
database = json.loads((ROOT / 'build/reference/compile_commands.json').read_text())
extra = shlex.split(os.environ.get('NIX_CFLAGS_COMPILE', ''))
extra += ['-I' + str(ROOT / 'tools/translation-headers')]
search = subprocess.run(['clang', '-E', '-v', '-x', 'c', '/dev/null'],
                        capture_output=True, text=True, check=True).stderr
includes = search.split('#include <...> search starts here:\n')[1].split('End of search list.')[0]
for include in includes.splitlines():
    extra += ['-isystem', include.strip()]
for binary in ('sway', 'swaymsg', 'swaybar', 'swaynag'):
    entries = []
    seen = set()
    for entry in database:
        output = entry['output']
        if not (output.startswith(f'{binary}/{binary}.p/')
                or output.startswith('common/')
                or (binary in ('swaybar', 'swaynag') and output.startswith('client/'))):
            continue
        source = (Path(entry['directory']) / entry['file']).resolve()
        if source in seen:
            continue
        seen.add(source)
        args = shlex.split(entry['command'])
        # libclang is not launched through Nix's compiler wrapper.
        args += extra
        args += ['-Wno-error']
        entries.append(dict(entry, arguments=args))
        del entries[-1]['command']
    dbdir = ROOT / 'build/translate' / binary
    dbdir.mkdir(parents=True, exist_ok=True)
    db = dbdir / 'compile_commands.json'
    db.write_text(json.dumps(entries, indent=2))
    output = args.output_dir.resolve() / binary
    if output.is_relative_to(ROOT / 'crates'):
        raise SystemExit('Translate into a separate directory, then port changes into the maintained Rust sources.')
    output.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run([
        'c2rust', 'transpile',
        '--emit-build-files', '--binary', 'main', '--edition', '2021',
        '--overwrite-existing', '--disable-rustfmt',
        '--output-dir', str(output), str(db),
    ], check=True, cwd=ROOT)
    version = subprocess.check_output(['rustc', '-vV'], text=True)
    target = next(line.removeprefix('host: ') for line in version.splitlines() if line.startswith('host: '))
    (output / 'translation-target').write_text(target + '\n')
    finalize(binary, output)
