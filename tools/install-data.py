#!/usr/bin/env python3
"""Install the original runtime data, manuals, and completions unchanged."""
from pathlib import Path
import argparse
import os
import shutil
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('--prefix', default='/usr/local')
parser.add_argument('--sysconfdir')
parser.add_argument('--destdir', default=os.environ.get('DESTDIR', ''))
args = parser.parse_args()
source = Path(__file__).resolve().parents[1] / 'original'
prefix = Path(args.prefix)
sysconfdir = Path(args.sysconfdir) if args.sysconfdir else prefix / 'etc'

def destination(path):
    return Path(args.destdir) / str(path).lstrip('/') if args.destdir else path

def install(src, path):
    path = destination(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(src, path)
    path.chmod(0o644)

config = (source / 'config.in').read_text()
for name, value in {'prefix': prefix, 'datadir': prefix / 'share', 'sysconfdir': sysconfdir}.items():
    config = config.replace(f'@{name}@', str(value))
config_path = destination(sysconfdir / 'sway/config')
config_path.parent.mkdir(parents=True, exist_ok=True)
config_path.write_text(config)
config_path.chmod(0o644)
install(source / 'sway.desktop', prefix / 'share/wayland-sessions/sway.desktop')
for wallpaper in (source / 'assets').glob('Sway_Wallpaper_*.png'):
    install(wallpaper, prefix / 'share/backgrounds/sway' / wallpaper.name)
for shell, target in [('bash', 'bash-completion/completions'), ('fish', 'fish/vendor_completions.d'),
                      ('zsh', 'zsh/site-functions')]:
    for completion in (source / 'completions' / shell).iterdir():
        if completion.name == 'meson.build':
            continue
        if shell == 'zsh' and completion.name == '_swaybar':
            continue  # Present in the source, but not installed by Meson.
        install(completion, prefix / 'share' / target / completion.name)
for manual in source.glob('*/*.scd'):
    filename = manual.name.removesuffix('.scd')
    section = filename.rsplit('.', 1)[1]
    output = destination(prefix / 'share/man' / ('man' + section) / filename)
    output.parent.mkdir(parents=True, exist_ok=True)
    with manual.open('rb') as stdin, output.open('wb') as stdout:
        subprocess.run(['scdoc'], stdin=stdin, stdout=stdout, check=True)
