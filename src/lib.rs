#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liter_to_gallon() {
        let val = convert(1.0, Unit::Liter, Unit::Gallon).unwrap();
        assert!((val - 0.2641720524).abs() < 1e-8);
    }

    #[test]
    fn test_gallon_to_liter() {
        let val = convert(1.0, Unit::Gallon, Unit::Liter).unwrap();
        assert!((val - 3.785411784).abs() < 1e-8);
    }

    #[test]
    fn test_identity() {
        let val = convert(42.0, Unit::Liter, Unit::Liter).unwrap();
        assert!((val - 42.0).abs() < 1e-8);
    }

    #[test]
    fn test_unsupported() {
        // Placeholder for unsupported units
        // Extend with more units to test this
        assert!(convert(1.0, Unit::Liter, Unit::Liter).is_some());
    }
}
/// Core unit conversion library (WASM compatible)

use wasm_bindgen::prelude::*;

/// Supported unit categories
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Liter,
    Gallon,
    // Extend with more units as needed
}

/// Convert a value from one unit to another
/// Returns None if conversion is not supported
pub fn convert(value: f64, from: Unit, to: Unit) -> Option<f64> {
    match (from, to) {
        (Unit::Liter, Unit::Gallon) => Some(value * 0.2641720524), // 1 liter = 0.2641720524 US gallons
        (Unit::Gallon, Unit::Liter) => Some(value / 0.2641720524),
        (u1, u2) if u1 == u2 => Some(value),
        _ => None,
    }
}

/// WASM-friendly conversion function
#[wasm_bindgen]
pub fn convert_units(value: f64, from: Unit, to: Unit) -> f64 {
    convert(value, from, to).unwrap_or(f64::NAN)
}