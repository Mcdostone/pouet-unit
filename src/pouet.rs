use crate::{
    error::ConvertError,
    graph::{self, UnitGraph},
    parser, unit,
};

pub struct Converter {
    graph: graph::UnitGraph,
}

impl Converter {
    /// Create a converter with all built-in units registered.
    pub fn new() -> Self {
        let mut graph = UnitGraph::new();
        unit::mass::register(&mut graph);
        unit::length::register(&mut graph);
        unit::temperature::register(&mut graph);
        unit::volume::register(&mut graph);
        unit::time::register(&mut graph);
        unit::speed::register(&mut graph);
        unit::area::register(&mut graph);
        unit::energy::register(&mut graph);
        unit::pressure::register(&mut graph);
        Converter { graph }
    }

    pub fn convert(&self, value: f64, from: &str, to: &str) -> Result<f64, ConvertError> {
        self.graph.convert(value, from, to)
    }

    pub fn convert_str(&self, input: &str) -> Result<f64, ConvertError> {
        let req = parser::parse(input)?;
        self.graph.convert(req.value, &req.from_unit, &req.to_unit)
    }

    /// Return a sorted list of all known unit names and aliases.
    pub fn known_units(&self) -> Vec<&'static str> {
        self.graph.known_units()
    }
}

impl Default for Converter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c() -> Converter {
        Converter::new()
    }
}
