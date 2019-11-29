#!/bin/bash
cargo build --release
cp target/release/libsoulinjector.dylib ./soulinjector.so
