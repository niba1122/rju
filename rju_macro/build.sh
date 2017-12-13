find . -regex ".*deps.*wasm" -exec rm {} \;
cargo build;
find . -regex ".*deps.*wasm" -exec cp {} ./hello.wasm \;
