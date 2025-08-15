clean:
    rm -fRd ./build
    rm -fRd ./rust/bed/target
    rm -fRd ./lib

clear:
    just clean
    rm -fRd ./.cache

make:
    just build

build:
    python3 ./contrib/build.py

run:
    ./build/Bed

br:
    just build
    just run
