
![bed](./assets/output.gif)

# Linux build

[just](https://github.com/casey/just), [rust toolchain](https://www.rust-lang.org/tools/install)
and [Qt6 Framework](https://qt-project.org/) are needed for build:

First install rust toolchain.

Then you can install just:
```
cargo install just
```

And Qt6 Framework:
 - Debian: `sudo apt install qt6-base-dev`
 - Arch: `sudo pacman -S qt6-base`

## Build commands

```shell
just build # build
just run   # run the binary
just br    # build & run
just clean # clean build assets
```

Note: the built binary will be located at `./build/Bed`

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
