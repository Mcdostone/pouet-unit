use crate::{
    graph::{Dimension, UnitGraph, UnitInfo},
    unit::Conversion,
};

const DIM: Dimension = Dimension("speed");

pub fn register(g: &mut UnitGraph) {
    // Base unit: meter per second
    g.add_unit(UnitInfo {
        name: "meter per second",
        aliases: &["meter per second", "meters per second", "m/s", "mps"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "kilometer per hour",
        aliases: &["kilometer per hour", "kilometers per hour", "km/h", "kph"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "mile per hour",
        aliases: &["mile per hour", "miles per hour", "mph"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "knot",
        aliases: &["knot", "knots", "kt", "kn"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "foot per second",
        aliases: &["foot per second", "feet per second", "fps", "ft/s"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "mach",
        aliases: &["mach", "mach number"],
        dimension: DIM.clone(),
    });

    g.add_edge(
        "meter per second",
        "kilometer per hour",
        Conversion::Linear(3.6),
    );
    g.add_edge(
        "meter per second",
        "mile per hour",
        Conversion::Linear(2.236_936),
    );
    g.add_edge("meter per second", "knot", Conversion::Linear(1.943_844));
    g.add_edge(
        "meter per second",
        "foot per second",
        Conversion::Linear(3.280_840),
    );
    g.add_edge("meter per second", "mach", Conversion::Linear(1.0 / 343.0)); // at sea level, 20°C
}
