# Sway: first Rust translation

This tree translates Sway 1.13-dev to Rust, retaining the C implementation's
structure, allocation, callbacks, ABI, and external libraries. The complete
original project is preserved in `original/`. The four translated programs are
in `crates/sway`, `crates/swaymsg`, `crates/swaybar`, and `crates/swaynag`.
Cargo does not compile or link any of Sway's original C implementation.

Enter the pinned development environment and build:

```sh
nix develop
cargo build --workspace
```

Executables are in `target/debug/`. `nix build` builds the optimized package;
`result/bin/` contains all four programs, and the package also installs the
original configuration, session file, manuals, wallpapers, and completions.
`SWAY_SYSCONFDIR` selects the system configuration directory at compile time.
The Cargo default matches the original Meson default; the Nix package uses
its own `etc/` directory.

The flake locks Nixpkgs, wlroots 0.21, C2Rust, and TinyCBOR. It builds C2Rust
without network access and patches its handling of forward-declared enums.
The original dependency versions and the full default feature set are retained:
Xwayland, libinput, tray support through libsystemd, and GDK Pixbuf.

This first translation is generated for **aarch64 Linux** against those pinned
headers. C2Rust folds platform constants and structure offsets into the output.
Builds reject a different target rather than silently using an incompatible ABI.
To translate for another native Linux target, enter the flake there and regenerate:

```sh
meson setup build/reference original -Dbuildtype=debugoptimized
ninja -C build/reference
python3 tools/translate.py
cargo build --workspace
```

`tools/finalize.py` records the mechanical integration fixes: Cargo scaffolding,
current Rust variadic API names, enum identity, bitfield accessors, GCC-compatible
log line numbers, byte-preserving command-line arguments, and SIGPIPE disposition.
The retained opaque C types require `extern_types`; `.cargo/config.toml` enables
that compiler feature for this initial port. The generated sources are deliberately
unsafe and repetitive. They contain no unimplemented placeholders.

Compare the translation with a native C reference build:

```sh
meson setup build/reference original -Dbuildtype=debugoptimized
ninja -C build/reference
python3 tools/check-cli.py
python3 tools/check-ipc.py
python3 tools/check-headless.py
```

The checks compare CLI output and exit status, IPC framing and output, compositor
state with two real Wayland windows after tiling, floating, fullscreen, scratchpad,
workspace/output/input/seat/bar commands, and startup of swaybar and swaynag as
layer-shell clients. The flake includes Foot and test fonts. Headless sessions use separate runtime directories
and do not affect the running desktop. Hardware rendering, real input devices,
X11 clients, tray interactions, and a complete interactive session still need
validation. Set `SWAY_PARITY_BIN_DIR` to an absolute binary directory to run the
same comparisons against an optimized package. See `original/README.md` and the
original manuals for usage.

Run the NixOS VM test on an aarch64 Linux host with KVM:

```sh
nix build -L .#checks.aarch64-linux.vm -o result-vm
```

It boots separate C and Rust desktops with virtio GPUs and Pixman rendering,
injects keyboard input through QEMU, and exercises Wayland and X11 windows,
floating/fullscreen/scratchpad behavior, workspaces, bar and tray registration,
swaynag, reload, and clean exit. It compares resulting state between the two
guests. Screenshots and guest logs are saved in `result-vm/`.
