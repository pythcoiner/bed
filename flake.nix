{
  description = "Bitcoin Encrypted Backup - Desktop application";

  inputs = {
    # Use nixos-22.11 for glibc 2.35 (Ubuntu 22.04+ compatibility)
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    # Unstable for Rust 1.82
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, nixpkgs-unstable, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        # Use unstable for rust-overlay
        pkgsUnstable = import nixpkgs-unstable {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        # MSRV: Rust 1.82
        rustToolchain = pkgsUnstable.rust-bin.stable."1.82.0".default;

        # Static Qt6 from vendored lib/qt6
        qt6Static = ./lib/qt6;

      in {
        devShells.default = pkgs.mkShell {
          name = "bed-dev";

          buildInputs = [
            # Build tools
            pkgs.cmake
            pkgs.gnumake
            pkgs.pkg-config
            pkgs.python3
            pkgs.git

            # Rust
            rustToolchain

            # Linux system deps
            pkgs.systemd  # libudev

            # X11/Wayland deps required by static Qt6
            pkgs.xorg.libX11
            pkgs.xorg.libXext
            pkgs.xorg.libXrender
            pkgs.xorg.libxcb
            pkgs.xorg.xcbutil
            pkgs.xorg.xcbutilwm
            pkgs.xorg.xcbutilimage
            pkgs.xorg.xcbutilkeysyms
            pkgs.xorg.xcbutilrenderutil
            pkgs.xorg.xcbutilcursor
            pkgs.xorg.libXi
            pkgs.xorg.libXrandr
            pkgs.xorg.libXcursor
            pkgs.xorg.libXinerama
            pkgs.xorg.libXfixes
            pkgs.xorg.libXcomposite
            pkgs.xorg.libXdamage
            pkgs.xorg.libXtst
            pkgs.libxkbcommon

            # Graphics
            pkgs.libGL
            pkgs.mesa

            # Wayland
            pkgs.wayland
            pkgs.wayland-protocols

            # Fonts
            pkgs.fontconfig
            pkgs.freetype
            pkgs.harfbuzz

            # Other Qt deps
            pkgs.dbus
            pkgs.dbus.dev
            pkgs.dbus.lib
            pkgs.openssl
            pkgs.zlib
            pkgs.libpng
            pkgs.libjpeg
            pkgs.pcre2
            pkgs.double-conversion
            pkgs.libb2
            pkgs.glib
            pkgs.mtdev
            pkgs.libinput
            pkgs.xorg.libSM
            pkgs.xorg.libICE
            pkgs.vulkan-headers
            pkgs.vulkan-loader
            pkgs.libdrm
          ];

          shellHook = ''
            echo "BED Development Shell (glibc 2.35)"
            echo "==================================="
            echo "Static Qt6: ${qt6Static}"
            echo ""
            export CMAKE_PREFIX_PATH="${qt6Static}"
            # Help CMake find DBus
            export DBus1_DIR="${pkgs.dbus.dev}/lib/cmake/DBus1"
            export CMAKE_LIBRARY_PATH="${pkgs.dbus.lib}/lib:$CMAKE_LIBRARY_PATH"
            export CMAKE_INCLUDE_PATH="${pkgs.dbus.dev}/include/dbus-1.0:${pkgs.dbus.lib}/lib/dbus-1.0/include:$CMAKE_INCLUDE_PATH"
          '';
        };

        packages.default = pkgs.stdenv.mkDerivation {
          pname = "bed";
          version = "0.0.1";
          src = ./.;

          nativeBuildInputs = [
            pkgs.cmake
            pkgs.pkg-config
            rustToolchain
          ];

          buildInputs = [
            pkgs.systemd
            pkgs.xorg.libX11
            pkgs.xorg.libxcb
            pkgs.xorg.xcbutil
            pkgs.xorg.xcbutilwm
            pkgs.xorg.xcbutilimage
            pkgs.xorg.xcbutilkeysyms
            pkgs.xorg.xcbutilrenderutil
            pkgs.xorg.xcbutilcursor
            pkgs.libxkbcommon
            pkgs.libGL
            pkgs.wayland
            pkgs.fontconfig
            pkgs.freetype
            pkgs.harfbuzz
            pkgs.dbus
            pkgs.dbus.dev
            pkgs.dbus.lib
            pkgs.openssl
            pkgs.zlib
            pkgs.libpng
            pkgs.libjpeg
            pkgs.pcre2
            pkgs.double-conversion
            pkgs.libb2
            pkgs.glib
            pkgs.mtdev
            pkgs.libinput
            pkgs.xorg.libSM
            pkgs.xorg.libICE
            pkgs.vulkan-headers
            pkgs.vulkan-loader
            pkgs.libdrm
          ];

          CMAKE_PREFIX_PATH = qt6Static;

          buildPhase = ''
            python3 ./contrib/build.py release linux
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp build/bin/bed-x86_64-linux-gnu $out/bin/bed
          '';
        };
      }
    );
}
