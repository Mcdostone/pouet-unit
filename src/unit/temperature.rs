use crate::{
    graph::{Dimension, UnitGraph, UnitInfo},
    unit::Conversion,
};

const DIM: Dimension = Dimension("temperature");

pub fn register(g: &mut UnitGraph) {
    // Base unit: celsius (affine conversions go via celsius)
    g.add_unit(UnitInfo {
        name: "celsius",
        aliases: &["celsius", "°C", "C", "degC"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "fahrenheit",
        aliases: &["fahrenheit", "°F", "F", "degF"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "kelvin",
        aliases: &["kelvin", "K"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "rankine",
        aliases: &["rankine", "°R", "R"],
        dimension: DIM.clone(),
    });

    // celsius <-> fahrenheit: F = C * 9/5 + 32
    g.add_edge(
        "celsius",
        "fahrenheit",
        Conversion::Affine {
            scale: 9.0 / 5.0,
            offset: 32.0,
        },
    );
    // celsius <-> kelvin: K = C + 273.15
    g.add_edge(
        "celsius",
        "kelvin",
        Conversion::Affine {
            scale: 1.0,
            offset: 273.15,
        },
    );
    // celsius <-> rankine: R = (C + 273.15) * 9/5
    g.add_edge(
        "celsius",
        "rankine",
        Conversion::Affine {
            scale: 9.0 / 5.0,
            offset: 273.15 * 9.0 / 5.0,
        },
    );
    // fahrenheit <-> kelvin goes via celsius (BFS will find path)
    // rankine <-> fahrenheit: F = R - 459.67  (direct shortcut)
    g.add_edge(
        "rankine",
        "fahrenheit",
        Conversion::Affine {
            scale: 1.0,
            offset: -459.67,
        },
    );
}
