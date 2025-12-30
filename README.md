
![bed](./assets/output.gif)

# Linux build

## Requirements

- [Nix](https://nixos.org/download.html) package manager
- [just](https://github.com/casey/just) command runner

## Static Qt6

Qt6 is vendored and statically linked for Linux builds. The static Qt6 library is located at `lib/qt6/` and was built using [qt_static](https://github.com/pythcoiner/qt_static) at commit `9fde2bc`.

- **Qt version**: 6.6.3
- **glibc target**: 2.35 (compatible with Ubuntu 22.04+, Fedora 36+, Debian 12+)
- **ICU**: disabled (uses system locale)
- **MSRV**: 1.82

### Runtime Dependencies

The binary is statically linked against Qt6 but requires standard system libraries. On a typical desktop Linux installation, all dependencies should already be present.

On minimal systems, you may need to install:

```bash
# Ubuntu/Debian
sudo apt install libb2-1 libdouble-conversion3 libwacom9 \
  libxcb-cursor0 libxcb-util1 libxcb-render-util0 \
  libxcb-icccm4 libxcb-image0 libxcb-keysyms1

# Fedora
sudo dnf install libb2 double-conversion libwacom \
  xcb-util-cursor xcb-util xcb-util-renderutil \
  xcb-util-wm xcb-util-image xcb-util-keysyms
```

## Build commands

```shell
just build # build (uses nix develop)
just run   # run the binary
just br    # build & run
just clean # clean build assets
```

Note: the built binary will be located at `./build/bin/bed-x86_64-linux-gnu`

# Windows build

## Build dependencies:

  - [Rust toolchain](https://www.rust-lang.org/tools/install)

  - [GNU Toolchain](https://winlibs.com) is used to build on windows, see
installation
  instructions on the link.

  - [Python3](https://www.python.org/) is used for the build script.

  - [Qt6 Framework](https://qt-project.org/ must be installed and you must declare
  the `Qt_PATH` in environment variables: `set Qt_PATH=C:\Qt\<version>\mingw_64`.

## Build

Run `just buildw` if just is installed or `python3 ./contrib/build.py release
windows` from the repo root.

Note: `Bed.exe` and its dependencies will be installed in `./build/bin`, and you can
move the `bin` folder where you want.
