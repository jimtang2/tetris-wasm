# Tetris (Rust-WASM)

This is Tetris in Rust for WebAssembly. 

## Get Started

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo test
wasm-pack build --target web

# copy to and run web app 
cp pkg/* ../webapp/public/wasm/
cd ../webapp && npm run dev
```
