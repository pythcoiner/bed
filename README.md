
![bed](./assets/output.gif)

# Build dependencies

[just](https://github.com/casey/just), [rust toolchain](https://www.rust-lang.org/tools/install) and [Qt6 Framework](https://qt-project.org/) are needed for build:

First install rust toolchain.

Then you can install just:
```
cargo install just
```

And Qt6 Framework:
 - Debian: `sudo apt install qt6-base-dev`
 - Arch: `sudo pacman -S qt6-base`

# Build

```shell
just build # build
just run   # run the binary
just br    # build & run
just clean # clean build assets
```

Note: the built binary will be located at `./build/Bed`
