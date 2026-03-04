use crate::error::ConvertError;

/// Parsed representation of a conversion request.
#[derive(Debug, PartialEq)]
pub struct ConversionRequest {
    pub value: f64,
    pub from_unit: String,
    pub to_unit: String,
}

/// Parse a string like:
///   "3 tonnes to kg"
///   "3.5 km in miles"
///   "100 °C as °F"
///   "3 tonnes kg"        (no connector word)
pub fn parse(input: &str) -> Result<ConversionRequest, ConvertError> {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Must have at least: <value> <from> <to>
    if tokens.len() < 3 {
        return Err(ConvertError::ParseError(format!(
            "Expected '<value> <unit> [to|in|as] <unit>', got: '{}'",
            input
        )));
    }

    // First token must be the numeric value
    let value: f64 = tokens[0]
        .parse()
        .map_err(|_| ConvertError::ParseError(format!("'{}' is not a valid number", tokens[0])))?;

    // Connector words we skip
    const CONNECTORS: &[&str] = &["to", "in", "as", "into", "->", "="];

    match tokens.len() {
        // "3 kg m"  (3 tokens, no connector)
        3 => Ok(ConversionRequest {
            value,
            from_unit: tokens[1].to_string(),
            to_unit: tokens[2].to_string(),
        }),
        // "3 kg to m"  (4 tokens with connector)
        4 if CONNECTORS.contains(&tokens[2].to_lowercase().as_str()) => Ok(ConversionRequest {
            value,
            from_unit: tokens[1].to_string(),
            to_unit: tokens[3].to_string(),
        }),
        // "3 kg to m"  (4 tokens, no connector — treat tokens[1] as from, tokens[3] as to)
        4 => Ok(ConversionRequest {
            value,
            from_unit: tokens[1].to_string(),
            to_unit: tokens[3].to_string(),
        }),
        // Multi-word units: e.g. "3 nautical miles to km"
        // Strategy: find the connector, split around it.
        _ => {
            let connector_pos = tokens[2..]
                .iter()
                .position(|t| CONNECTORS.contains(&t.to_lowercase().as_str()))
                .map(|i| i + 2); // adjust index relative to full token list

            if let Some(pos) = connector_pos {
                let from_unit = tokens[1..pos].join(" ");
                let to_unit = tokens[pos + 1..].join(" ");
                if to_unit.is_empty() {
                    return Err(ConvertError::ParseError(
                        "Missing target unit after connector".to_string(),
                    ));
                }
                Ok(ConversionRequest {
                    value,
                    from_unit,
                    to_unit,
                })
            } else {
                Err(ConvertError::ParseError(format!(
                    "Could not determine source and target units in: '{}'",
                    input
                )))
            }
        }
    }
}
