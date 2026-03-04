use wasm_bindgen::prelude::wasm_bindgen;

mod constants;
mod error;
mod graph;
mod input;
mod parser;
mod pouet;
mod unit;

#[wasm_bindgen]
pub fn greet(request: &str) -> f64 {
    let pouet = pouet::Converter::new();
    pouet.convert_str(request).unwrap_or(0.0)
}
