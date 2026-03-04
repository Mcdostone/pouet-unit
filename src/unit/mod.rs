pub mod area;
pub mod energy;
pub mod length;
pub mod mass;
pub mod pressure;
pub mod speed;
pub mod temperature;
pub mod time;
pub mod volume;

/// Represents a conversion step between two directly connected units.
#[derive(Clone, Debug)]
pub enum Conversion {
    /// Multiply by a constant factor. Covers all SI-style conversions.
    Linear(f64),
    /// Apply scale then offset: result = value * scale + offset.
    /// Used for temperature and similar affine conversions.
    Affine { scale: f64, offset: f64 },
}

impl Conversion {
    /// Apply this conversion to a value.
    pub fn apply(&self, value: f64) -> f64 {
        match self {
            Conversion::Linear(factor) => value * factor,
            Conversion::Affine { scale, offset } => value * scale + offset,
        }
    }

    /// Return the inverse of this conversion (for bidirectional edges).
    pub fn inverse(&self) -> Conversion {
        match self {
            Conversion::Linear(factor) => Conversion::Linear(1.0 / factor),
            Conversion::Affine { scale, offset } => Conversion::Affine {
                scale: 1.0 / scale,
                offset: -offset / scale,
            },
        }
    }
}

/// Chain a sequence of conversions, applying them left to right.
pub fn chain_conversions(value: f64, path: &[Conversion]) -> f64 {
    path.iter().fold(value, |v, conv| conv.apply(v))
}
