#!/bin/sh

cargo run README.md > ../README.md
cargo run 16 > ../src/clamp_16.rs
cargo run 32 > ../src/clamp_32.rs
cargo run 64 > ../src/clamp_64.rs
