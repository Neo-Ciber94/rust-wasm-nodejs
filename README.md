# Calling Rust from NodeJS

This is a example for calling **wasm** `Rust` function from `NodeJS`.

## Requirements

- **node**
    - https://nodejs.org/
- **wasm-bindgen-cli**
    - https://crates.io/crates/wasm-bindgen-cli
- **cargo make (optional, can be run manually)**
    - https://crates.io/crates/cargo-make

## How to run?

1. Install the node dependencies

    ```bash
    npm install
    ```

2. Build the wasm

    ```bash
    cargo make build
    ```

3. Starts the nodejs server

    ```bash
    npm run start
    ```

> After that **GET** or **POST** to the `http://localhost:3000`

## Manual run

After installing the node dependencies:

1. Build to wasm32
    - `cargo build --target wasm32-unknown-unknown`
2. Bundle the wasm
    - `wasm-bindgen --typescript --target nodejs --out-dir src/pkg target/wasm32-unknown-unknown/debug/rust_wasm_nodejs.wasm`
3. Starts the node server
    - `npx tsx src/index.ts`
