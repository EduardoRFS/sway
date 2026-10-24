#!/usr/bin/env python3
"""Apply repeatable integration fixes to C2Rust's Cargo scaffolding."""
from pathlib import Path
import re
import tomllib

ROOT = Path(__file__).resolve().parents[1]

def log_lines(path):
    """GCC expands __LINE__ at a multiline macro's start, Clang at its end."""
    source = path.read_text()
    lines = {}
    tokens = re.compile(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|//[^\n]*|/\*.*?\*/|[()]', re.S)
    for call in re.finditer(r'\bsway_(?:log|log_errno|vlog|assert|abort)\s*\(', source):
        depth = 1
        for token in tokens.finditer(source, call.end()):
            if token.group() == '(':
                depth += 1
            elif token.group() == ')':
                depth -= 1
            if depth == 0:
                first = source.count('\n', 0, call.start()) + 1
                last = source.count('\n', 0, token.end()) + 1
                for line in range(first, last + 1):
                    lines[line] = first
                break
    return lines

LOG_LINES = {str(path.relative_to(ROOT / 'original')): log_lines(path)
             for path in (ROOT / 'original').rglob('*.c')}

def fix_log_line(match):
    filename, line = match.group(2), int(match.group(3))
    fixed = LOG_LINES.get(filename, {}).get(line, line)
    return match.group(1) + str(fixed) + match.group(4)

def finalize(binary):
    crate = ROOT / 'crates' / binary
    manifest = crate / 'Cargo.toml'
    text = manifest.read_text()
    text = re.sub(r'\[workspace\]\s*members\s*=\s*\[[^\]]*\]\s*', '', text)
    text = re.sub(r'\[profile\.release\]\s*strip = "debuginfo"', '', text)
    text = text.replace('version = "0.0.0"', 'version = "1.13.0-dev"')
    text = text.replace('name = "main"', f'name = "{binary}"')
    text = text.replace('crate-type = ["staticlib", "rlib"]', 'crate-type = ["rlib"]')
    if 'build = ' not in text:
        text = text.replace('[package]', '[package]\nbuild = "../../tools/link.rs"\nlicense = "MIT"')
    manifest.write_text(text.rstrip() + '\n')
    entry = crate / tomllib.loads(text)['bin'][0]['path']
    for filename in ('rust-toolchain', 'rust-toolchain.toml'):
        (crate / filename).unlink(missing_ok=True)
    for source in crate.rglob('*.rs'):
        text = source.read_text()
        # C2Rust's edition-2021 emitter targets the old unstable variadic API.
        # Rust 1.99 unified VaListImpl with VaList; clone is C va_copy.
        text = text.replace('::core::ffi::VaListImpl', "::core::ffi::VaList<'_>")
        text = text.replace('.as_va_list()', '.clone()')
        text = text.replace('::core::ptr::from_exposed_addr_mut', '::core::ptr::with_exposed_provenance_mut')
        # The released bitfield derive does not parse raw keyword identifiers.
        text = text.replace('name = "r#type"', 'name = "type_value"')
        text = text.replace('.set_type(', '.set_type_value(')
        text = re.sub(r'b"//usr/local/etc(/[^"\n]*)"',
                      lambda m: 'concat!(env!("SWAY_SYSCONFDIR"), "' + m.group(1) + '")', text)
        text = re.sub(r'pub const SYSCONFDIR: \[::core::ffi::c_char; 16\] = unsafe \{.*?\n\};',
                      'pub const SYSCONFDIR: &str = concat!(env!("SWAY_SYSCONFDIR"), "\\\\0");', text, flags=re.S)
        text = text.replace('prefix: SYSCONFDIR.as_ptr(),', 'prefix: SYSCONFDIR.as_ptr() as *const ::core::ffi::c_char,')
        text = re.sub(r'(b"([^"\n]+\.c)\\0"\.as_ptr\(\)\s+as \*const ::core::ffi::c_char,\s*)(\d+)( as ::core::ffi::c_int)',
                      fix_log_line, text)
        # Both names designate the same C enum (a forward declaration and its
        # definition). Unify the emitted types without changing representation.
        text = re.sub(r'pub struct sway_container_layout_0\(pub ::core::ffi::c_uint\);\nimpl sway_container_layout_0 \{.*?\n\}',
                      'pub type sway_container_layout_0 = sway_container_layout;', text, flags=re.S)
        text = re.sub(r'#\[derive\([^\n]*\)\]\n#\[repr\(transparent\)\]\n(pub type sway_container_layout_0)', r'\1', text)
        # Suppress mechanical style warnings; retain compiler diagnostics.
        if source == crate / 'lib.rs' or source.name == 'main.rs':
            text = '#![allow(unused_imports, unused_unsafe, stable_features, deprecated)]\n' + text if '#![allow(unused_imports, unused_unsafe' not in text else text
        source.write_text(text)
    text = entry.read_text()
    text = text.replace(f'use ::{binary};', f'use ::{binary} as c2rust_ffi;')
    # C argv is arbitrary bytes. std::env::args() would reject non-UTF-8 input.
    text = text.replace('::std::env::args()', '::std::env::args_os()')
    text = text.replace('::std::ffi::CString::new(arg)',
                        '::std::ffi::CString::new(::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()))')
    if 'fn signal(number:' not in text:
        text = text.replace('pub fn main() {', '''pub fn main() {
    // Match the C runtime's default SIGPIPE disposition.
    unsafe {
        extern "C" { fn signal(number: i32, handler: usize) -> usize; }
        signal(13, 0);
    }''')
    entry.write_text(text)

if __name__ == '__main__':
    for binary in ('sway', 'swaymsg', 'swaybar', 'swaynag'):
        finalize(binary)
