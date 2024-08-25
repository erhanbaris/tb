#!/bin/bash
set -e


arch -x86_64 zsh
cargo run > test.asm
gcc -arch x86_64 test.asm -o test
./test ; echo $?
