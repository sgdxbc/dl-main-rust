#!/bin/bash -x

set -e

mkdir -p build

cc -o build/run runtime.c -ldl

cc -o build/c_exe c_example.c
cc -o build/libc_example.so c_example.c -fPIC -shared

pushd dloadable
cargo build
popd

./build/c_exe hello world 42
./build/run ./build/libc_example.so
./build/run ./dloadable/target/debug/libdloadable.so
