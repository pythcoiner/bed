clean:
    rm -fRd ./build
    rm -fRd ./rust/bed/target
    rm -fRd compile_commands.json
    rm -fRd ./.cache
    rm -fRd CMakeCache.txt
    rm -fRd CMakeFiles
    rm -fRd ./lib/linux
    rm -fRd ./lib/include
    rm -fRd ./lib/windows
    rm -fRd CMakeFiles
    rm -fRd CMakeFiles

clear:
    just clean

build:
    nix develop --ignore-environment --keep HOME --command python3 ./contrib/build.py release linux

buildw:
    python ./contrib/build.py release windows

run:
    ./build/Bed

br:
    just build
    just run

offline:
    export OFFLINE="true"

online:
    export OFFLINE=""
