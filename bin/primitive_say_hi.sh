#!/bin/zsh
#set -x

SUBJECT=primitive_say_hi

cargo clean --manifest-path=$SUBJECT/Cargo.toml
cargo build --manifest-path=$SUBJECT/Cargo.toml --target wasm32-unknown-unknown --release

export WASM=$(find $SUBJECT -name "*.wasm" -print -quit)
#wasm2wat $WASM | grep memory --color

cargo run --bin $SUBJECT