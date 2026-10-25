# Sway: Rust translation

This tree translates Sway 1.13-dev to Rust, retaining the C implementation's
external behavior, callbacks, ABI, and external libraries. The complete
original project is preserved in `original/`. The four translated programs are
in `crates/sway`, `crates/swaymsg`, `crates/swaybar`, and `crates/swaynag`.
Their common runtime is shared through `crates/common`. Gesture parsing, matching
and tracking, socket discovery, and IPC framing use maintained Rust APIs with
owned values, enums, slices, `Result`, and RAII. Those core modules forbid unsafe
code; compatibility with the remaining translated callers lives in `ffi/`.
The client event loop uses `Vec` for registrations, stable `Box` timer handles,
and automatic cleanup. Callbacks can change registrations or poll recursively;
no collection borrow survives a callback. Its remaining unsafe operations are
C handle access, callback invocation, and the `poll`/monotonic-clock system calls.
The clock arithmetic retains C's negative-delay and truncation behavior.
Like standard Rust collections, the owned event loop aborts on allocation failure;
the C implementation could return null or leave partially updated state.
The maintained gesture API distinguishes `Kind`, `FingerCount`, `Directions`
(using `bitflags`), and `MatchMode`. Parse errors have matchable variants while
retaining partial results and arbitrary diagnostic bytes. Equality and hashing
use full gesture identity; specificity is a separate sorting key. IPC frames
use a nominal `MessageType`, `Frame::read_from`, and `Frame::write_to`; encoding
accepts owned or borrowed payloads through `AsRef<[u8]>`, and both operations
accept `Read`/`Write` trait objects. The C adapters retain the raw ABI layouts.
Unknown wire values and direction bits remain lossless. These core modules use
normal compiler and Clippy lints; generated-code exemptions are scoped to their
remaining compatibility modules.
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
To generate a fresh comparison scaffold on a native Linux target:

```sh
meson setup build/reference original -Dbuildtype=debugoptimized
ninja -C build/reference
python3 tools/translate.py
```

The scaffold is written to `build/retranslated/`. It does not overwrite the
maintained workspace. Port ABI and binding changes from that scaffold explicitly
when adding another target.

`tools/finalize.py` records the mechanical integration fixes: Cargo scaffolding,
current Rust variadic API names, enum identity, bitfield accessors, GCC-compatible
log line numbers, byte-preserving command-line arguments, and SIGPIPE disposition.
The retained opaque C types require `extern_types`; `.cargo/config.toml` enables
that compiler feature for this initial port. The generated sources are deliberately
unsafe and repetitive outside the refactored modules. They contain no
unimplemented placeholders.

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
and do not affect the running desktop. Physical GPUs and input devices,
accelerated rendering, and full tray interactions still need validation.
Set `SWAY_PARITY_BIN_DIR` to an absolute binary directory to run the
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

The common-runtime check runs Rust unit tests, 3,632 differential gesture
comparisons, and 77 event-loop comparisons against the unchanged C implementation.
The event-loop checks cover mutation during callbacks, nested polling, timer
identity and cancellation, descriptor ownership, hangup/error masks, and EINTR:

```sh
nix build -L .#checks.aarch64-linux.common -o result-common
```

For local development, use `cargo test -p sway-common --lib`. The IPC tests cover
fragmented reads and writes, binary payloads, frame boundaries, truncation, and
the C callers' null pointer for empty requests. Partial writes are completed and
mid-payload disconnects return an error instead of spinning indefinitely.
