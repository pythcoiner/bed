clean:
    rm -fRd ./build
    rm -fRd ./rust/bed/target
    rm -fRd ./lib
    rm -fRd compile_commands.json
    rm -fRd ./.cache
    rm -fRd CMakeCache.txt
    rm -fRd CMakeFiles

clear:
    just clean

build:
    python3 ./contrib/build.py release linux

buildw:
    python3 ./contrib/build.py release windows

run:
    ./build/Bed

br:
    just build
    just run

offline:
    export OFFLINE="true"

online:
    export OFFLINE=""
