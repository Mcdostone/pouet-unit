use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ConvertError {
    /// The input string could not be parsed.
    ParseError(String),
    /// A unit name or alias was not found in the graph.
    UnknownUnit(String),
    /// No conversion path exists between the two units.
    NoPathFound(String, String),
    /// The two units belong to different dimensions.
    DimensionMismatch(String, String),
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvertError::ParseError(s) => write!(f, "Parse error: {}", s),
            ConvertError::UnknownUnit(u) => write!(f, "Unknown unit: '{}'", u),
            ConvertError::NoPathFound(a, b) => {
                write!(f, "No conversion path found from '{}' to '{}'", a, b)
            }
            ConvertError::DimensionMismatch(a, b) => {
                write!(f, "Dimension mismatch: '{}' vs '{}'", a, b)
            }
        }
    }
}

impl std::error::Error for ConvertError {}
