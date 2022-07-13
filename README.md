# Bevy Example

## run in browser directly

```cargo run --target wasm32-unknown-unknown```

## build

```sh
cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-name practice --out-dir ./webapp/out --target web target/wasm32-unknown-unknown/release/bevy_practice.wasm

cd webapp
yarn dev
```
