// Shared build script for the translated crates. Only external libraries are
// linked; none of Sway's original C implementation is compiled or linked here.
use std::{env, fs, process::Command};

fn main() {
    let name = env::var("CARGO_PKG_NAME").unwrap();
    let translated_target = fs::read_to_string("translation-target").unwrap();
    assert_eq!(translated_target.trim(), env::var("TARGET").unwrap(),
               "The C ABI translation is target-specific. Regenerate it with tools/translate.py on this target.");
    println!("cargo:rerun-if-changed=translation-target");
    println!(
        "cargo:rustc-env=SWAY_SYSCONFDIR={}",
        env::var("SWAY_SYSCONFDIR").unwrap_or_else(|_| "//usr/local/etc".into())
    );
    println!("cargo:rerun-if-env-changed=SWAY_SYSCONFDIR");
    let mut dependencies = vec!["cairo", "pango", "pangocairo", "wayland-client"];
    match name.as_str() {
        "sway-common" => {}
        "sway" => dependencies.extend([
            "wlroots-0.21",
            "libdrm",
            "json-c",
            "libevdev",
            "libinput",
            "libudev",
            "libpcre2-8",
            "pixman-1",
            "wayland-server",
            "xkbcommon",
            "xcb",
            "xcb-icccm",
        ]),
        "swaybar" => {
            dependencies.extend(["gdk-pixbuf-2.0", "json-c", "wayland-cursor", "libsystemd"])
        }
        "swaynag" => dependencies.push("wayland-cursor"),
        "swaymsg" => dependencies.push("json-c"),
        _ => panic!("unknown program: {name}"),
    }
    let output = Command::new("pkg-config")
        .arg("--libs")
        .args(dependencies)
        .output()
        .expect("pkg-config is required; enter nix develop");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    for arg in String::from_utf8(output.stdout).unwrap().split_whitespace() {
        if let Some(path) = arg.strip_prefix("-L") {
            println!("cargo:rustc-link-search=native={path}");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{path}");
        } else if let Some(lib) = arg.strip_prefix("-l") {
            println!("cargo:rustc-link-lib={lib}");
        }
    }
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!("cargo:rerun-if-changed=../../tools/link.rs");
}
