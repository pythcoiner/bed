#!/bin/bash

echo "OFFLINE=$OFFLINE"

set -e

# create ./lib if not existing
if ! [ -d "./lib" ]; then
    mkdir lib
else
    # cleanup lib
    rm -fRd ./lib/*
fi

mkdir ./lib/include

cd ./rust/bed

# cargo will:
#   - first generate c++ bindings
#   - then build library
if [ "$OFFLINE" = false ]; then
    cargo build --release
else
    cargo build --release --offline
fi

cd ../../
# copy bindings into ./lib/include/
cp -L ./rust/bed/target/cxxbridge/bed/src/lib.rs.h ./lib/include/bed.h
cp -L ./rust/bed/target/cxxbridge/rust/cxx.h ./lib/include/cxx.h

# copy libraries into ./lib/
cp ./rust/bed/target/release/libbed.a ./lib/libbed.a
cp ./rust/bed/target/release/libbed.rlib ./lib/libbed.rlib
cp ./rust/bed/target/release/libbed.so ./lib/libbed.so
cp ./rust/bed/target/release/libbed.d ./lib/libbed.d
