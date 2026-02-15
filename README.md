# pouet_jane: WASM-Compatible Unit Conversion Library

## Features
- Convert between supported units (e.g., liters ↔ gallons)
- Designed for WASM (WebAssembly) export using wasm-bindgen
- Extensible for more unit types

## Usage (Rust)

Add to your Cargo.toml:
```toml
[dependencies]
pouet_jane = { path = "." }
```

Example:
```rust
use pouet_jane::{convert, Unit};
let gallons = convert(10.0, Unit::Liter, Unit::Gallon).unwrap();
println!("10 liters = {:.4} gallons", gallons);
```

## Usage (WASM)

Build for WASM with wasm-pack:
```sh
wasm-pack build --target web
```
This will generate JS bindings in `pkg/` for use in web apps.

## Testing

Run all tests:
```sh
cargo test
```

## Extending
- Add new units to the `Unit` enum in `src/lib.rs`
- Extend the `convert` function with new conversion logic

## License
MIT
liters to gallons
tons a fuels to liter fuels (density fuel)
cubic meters to liters


currency conversion combined 
year 2018, brazilien rerss to 2025 USD dollars