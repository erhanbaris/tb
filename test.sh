#!/bin/bash
set -e

arch -x86_64 zsh
cargo run > test.s
gcc -arch x86_64 test.s -o test
./test ; echo $?
