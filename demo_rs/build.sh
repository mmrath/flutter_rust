#!/usr/bin/env bash
#cargo clean
cargo build --target aarch64-linux-android --release &&
cargo build --target armv7-linux-androideabi --release &&
cargo build --target i686-linux-android --release
cargo build --target x86_64-linux-android --release


