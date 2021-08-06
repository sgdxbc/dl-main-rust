#!/bin/bash -x

set -e

mkdir -p build

cc -o build/run runtime.c -ldl

cc -o build/libc_example.so c_example.c -fPIC -shared

pushd dloadable
cargo build
popd

./build/run ./build/libc_example.so
./build/run ./dloadable/target/debug/libdloadable.so
