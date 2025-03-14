#!bin/sh
rustup target add wasm32-unknown-unknown;
cargo install wasm-bindgen-cli;
cargo build --release --target wasm32-unknown-unknown;
wasm-bindgen --out-name jump_cube --out-dir wasm/target --target web ./target/wasm32-unknown-unknown/release/jump_cube.wasm;
cargo install simple-http-server;
simple-http-server -i -b 192.168.11.6 -p 4000;
