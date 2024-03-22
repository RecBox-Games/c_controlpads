#!/bin/bash

rustup default nightly
cargo build --release
rustup default stable
