#!/bin/bash

# Copyright 2022-2024 RecBox, Inc.
#
# This file is part of the c_controlpads repository.
#
# c_controlpads is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by the 
# Free Software Foundation, either version 3 of the License, or (at your option)
# any later version.
# 
# c_controlpads is distributed in the hope that it will be useful, but
# WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
# or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
# more details.
# 
# You should have received a copy of the GNU General Public License along with
# c_controlpads. If not, see <https://www.gnu.org/licenses/>.

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
