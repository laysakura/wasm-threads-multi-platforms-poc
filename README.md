## Building `.wasm` (`wasm32-wasi`)

Install [`cargo-wasi`](https://github.com/bytecodealliance/cargo-wasi) first.

```console
# debug build
cargo wasi build
ls target/wasm32-wasi/debug/*.wasm

# release build
cargo wasi build --release
ls target/wasm32-wasi/debug/*.wasm
```
