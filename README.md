# Pixel Sorting with Rust + Wasm

![image](https://github.com/user-attachments/assets/6f0be2d1-e81f-4f8a-a680-bf2f0d0c5787)

## Special features
- Arbitrary sorting angle support! (0-360°)
- Luminance histogram!
- ...
- Fully static - images stay in browser!
- ...
- 🚀 Rust 🔥

### Compiling
1. Install rust
2. Install wasm-pack `cargo install wasm-pack`
3. Build with wasm-pack `wasm-pack build --target web --release --out-dir web/pkg`
4. Everything should now be in ./web/
5. Serve the ./web/ directory
