{
  description = "Sway's C-to-Rust translation environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/02c087f3fe673c9e317f206a627cb6b910336dbe";
    wlroots = {
      url = "git+https://gitlab.freedesktop.org/wlroots/wlroots.git?rev=f9d2520f90e1026152ecb3178a7d390621fcee03";
      flake = false;
    };
    c2rust = {
      url = "github:immunant/c2rust/820a9408154c006e2c9dcc90a1bad81385bbfaf1";
      flake = false;
    };
    tinycbor = {
      url = "github:intel/tinycbor/d393c16f3eb30d0c47e6f9d92db62272f0ec4dc7";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, wlroots, c2rust, tinycbor }@inputs:
    let
      systems = [ "aarch64-linux" "x86_64-linux" ];
      forSystems = nixpkgs.lib.genAttrs systems;
      packagesFor = system:
        let
          pkgs = import nixpkgs { inherit system; };
          llvm = pkgs.llvmPackages;
          rustSource = pkgs.lib.cleanSourceWith {
            src = self;
            filter = path: type:
              let relative = pkgs.lib.removePrefix "${self}/" (toString path);
              in type == "directory"
                || builtins.elem relative [ "Cargo.toml" "Cargo.lock" ".cargo/config.toml" "tools/link.rs" "tools/install-data.py" ]
                || pkgs.lib.hasPrefix "crates/" relative
                || pkgs.lib.hasPrefix "original/" relative;
          };
        in rec {
          wlroots = pkgs.wlroots.overrideAttrs (old: {
            version = "0.21.0-dev";
            src = inputs.wlroots;
            outputs = [ "out" ];
            mesonFlags = old.mesonFlags ++ [ "-Dexamples=false" ];
            postFixup = "";
          });
          c2rust = pkgs.rustPlatform.buildRustPackage {
            pname = "c2rust";
            version = "0.22.1-${builtins.substring 0 7 inputs.c2rust.rev}";
            src = inputs.c2rust;
            patches = [ ./nix/patches/c2rust-enum-definition.patch ];
            cargoLock.lockFile = inputs.c2rust + "/Cargo.lock";
            postPatch = ''
              cp -r ${tinycbor} c2rust-ast-exporter/src/tinycbor-src
              chmod -R u+w c2rust-ast-exporter/src/tinycbor-src
              substituteInPlace c2rust-ast-exporter/src/CMakeLists.txt \
                --replace-fail 'GIT_REPOSITORY ''${TINYCBOR_REPO}' 'SOURCE_DIR ''${CMAKE_CURRENT_SOURCE_DIR}/tinycbor-src' \
                --replace-fail 'GIT_TAG ''${TINYCBOR_TAG}' 'DOWNLOAD_COMMAND ""'
            '';
            nativeBuildInputs = [ pkgs.cmake pkgs.pkg-config llvm.clang ];
            buildInputs = [ llvm.llvm.dev llvm.libclang pkgs.zlib pkgs.ncurses pkgs.libxml2 pkgs.libffi ];
            LIBCLANG_PATH = "${llvm.libclang.lib}/lib";
            LLVM_CONFIG_PATH = "${llvm.llvm.dev}/bin/llvm-config";
            dontUseCmakeConfigure = true;
            cargoBuildFlags = [ "-p" "c2rust" ];
            doCheck = false;
          };
          sway = pkgs.rustPlatform.buildRustPackage {
            pname = "sway-rust";
            version = "1.13.0-dev";
            src = rustSource;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = with pkgs; [ pkg-config python3 scdoc ];
            buildInputs = with pkgs; [
              wlroots wayland libxkbcommon cairo pango gdk-pixbuf pixman
              libevdev libinput libdrm libxcb libxcb-wm json_c pcre2 systemd
            ];
            RUSTC_BOOTSTRAP = "1";
            preBuild = ''export SWAY_SYSCONFDIR="$out/etc"'';
            cargoBuildFlags = [ "--workspace" ];
            doCheck = false;
            postInstall = ''python3 tools/install-data.py --prefix "$out"'';
          };
          reference = pkgs.stdenv.mkDerivation {
            pname = "sway-c-reference";
            version = "1.13-dev";
            src = pkgs.lib.cleanSource (self + "/original");
            nativeBuildInputs = with pkgs; [ meson ninja pkg-config wayland-scanner scdoc ];
            mesonFlags = [ "-Dsd-bus-provider=libsystemd" ];
            buildInputs = with pkgs; [
              wlroots wayland wayland-protocols libxkbcommon cairo pango gdk-pixbuf pixman
              libevdev libinput libdrm libxcb libxcb-wm json_c pcre2 systemd
            ];
          };
          default = sway;
        };
    in {
      packages = forSystems packagesFor;
      checks.aarch64-linux = let
        pkgs = import nixpkgs { system = "aarch64-linux"; };
        packages = packagesFor "aarch64-linux";
      in {
        vm = import ./nix/tests/vm.nix { inherit pkgs packages; };
        common = packages.sway.overrideAttrs {
          pname = "sway-common-tests";
          src = self;
          cargoBuildFlags = [ "-p" "sway-common" ];
          doCheck = true;
          cargoTestFlags = [ "-p" "sway-common" "--lib" ];
          postCheck = ''
            cargo rustc --offline --release --target aarch64-unknown-linux-gnu -p sway-common --lib --crate-type cdylib
            export SWAY_COMMON_LIBRARY="$PWD/target/aarch64-unknown-linux-gnu/release/libsway_common.so"
            python3 tools/check-gestures.py
          '';
          installPhase = ''mkdir -p "$out"; touch "$out/passed"'';
          postInstall = "";
        };
      };
      devShells = forSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          packages = packagesFor system;
        in {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              cargo rustc clang llvmPackages.llvm.dev llvmPackages.libclang
              cmake ninja meson pkg-config python3 git curl wayland-scanner scdoc
              foot swaybg rustfmt clippy
              packages.c2rust
            ];
            buildInputs = with pkgs; [
              packages.wlroots wayland wayland-protocols libxkbcommon cairo pango
              gdk-pixbuf pixman libevdev libinput libdrm libGL libgbm
              libdisplay-info libliftoff seatd libxcb libxcb-wm libxcb-render-util
              libxcb-errors xwayland json_c pcre2 systemd hwdata lcms2
              vulkan-loader vulkan-headers glslang
            ];
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            LLVM_CONFIG_PATH = "${pkgs.llvmPackages.llvm.dev}/bin/llvm-config";
            FONTCONFIG_FILE = pkgs.makeFontsConf { fontDirectories = [ pkgs.dejavu_fonts ]; };
          };
        });
    };
}
