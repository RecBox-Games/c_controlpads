#!/bin/bash

rustup default nightly
if cargo build; then
    echo "rebuild_run.sh: building a.out"
    gcc test.c target/debug/libc_controlpads.a -lpthread -ldl
    echo "rebuild_run.sh: running"
    ./a.out
else
    echo "rebuild_run.sh: not building final executable or running it since cargo build failed"
fi
rustup default stable
