rustup component add rls --toolchain nightly
rustup component add rust-analysis --toolchain nightly
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-emscripten

  nightly-x86_64-apple-darwin unchanged - rustc 1.22.0-nightly (14039a42a 2017-09-22)

rustup override set nightly



* main()でstdoutするとおかしくなる？？
* コンパイルは`cargo build && find . -regex ".*deps.*wasm" -exec cp {} ./hello.wasm \;`
