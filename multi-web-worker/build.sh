#!/bin/bash -e
WASMBINDGEN_VERSION=0.2.76
OUT=./pkg

echo "Running cargo build"
RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
  cargo +nightly build --release --target wasm32-unknown-unknown \
  -Z build-std=panic_abort,std

if [ -d $OUT ]; then
  echo "Clearing output directory '$OUT'"
  rm -rf $OUT
fi

if ! [ -x "$(command -v wasm-bindgen)" ]; then
  echo "Installing wasm-bindgen-cli via cargo"
  cargo install wasm-bindgen-cli --version 0.2.76
fi

echo "Generating wasm-bindings"

# add supports for Weak References, see [1].
# TLDR: Structs passed from Rust to JS will be deallocated
# automatically, no need to call `.free` in JS.
#
# [1]: https://rustwasm.github.io/docs/wasm-bindgen/reference/weak-references.html
wasm-bindgen ./target/wasm32-unknown-unknown/release/wasm_hello_world.wasm \
  --out-dir $OUT \
  --target web \
  --typescript \
  --weak-refs

echo "Find your wasm package in $OUT"
