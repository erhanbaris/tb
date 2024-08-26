#!/bin/bash
set -e

cargo run > test.s
gcc -arch x86_64 test.s -o test
./test ; echo $?
