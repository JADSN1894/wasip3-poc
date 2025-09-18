_default:
    just --list

plugin-dev:
    rm -fv wasm/plugin.wasm
    mkdir -p wasm
    
    cargo build --package plugin --target wasm32-wasip2

    wasm-tools strip -vvv --all target/wasm32-wasip2/debug/plugin.wasm --output ./wasm/plugin.wasm
    wasm-tools validate --features cm-async ./wasm/plugin.wasm

    rm -fv input.txt
    wasm-tools  print --skeleton ./wasm/plugin.wasm --output input.txt

wit-deps:
    rm -rvf wit/deps wit/deps.lock
    wit-deps

plugin-and-app: wit-deps plugin-dev
    sleep 1
    cargo run --package app

app:
    cargo run --package app
