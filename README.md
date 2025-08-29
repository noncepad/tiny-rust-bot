# Example Bot for Catscope

## Rust Environment

```bash
rustup override set nightly
rustup component add rust-analyzer
rustup target add wasm32-wasip2 --toolchain nightly
```

Build with:

```bash
cargo +nightly build --target wasm32-wasip2
```
