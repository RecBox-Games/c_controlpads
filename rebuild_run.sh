#!/bin/bash

set -e

cargo build
gcc test.c target/debug/libc_controlpads.a
./a.out
