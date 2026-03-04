use crate::{
    graph::{Dimension, UnitGraph, UnitInfo},
    unit::Conversion,
};

const DIM: Dimension = Dimension("time");

pub fn register(g: &mut UnitGraph) {
    // Base unit: second
    g.add_unit(UnitInfo {
        name: "second",
        aliases: &["second", "seconds", "sec", "s"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "millisecond",
        aliases: &["millisecond", "milliseconds", "ms"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "microsecond",
        aliases: &["microsecond", "microseconds", "µs", "us"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "nanosecond",
        aliases: &["nanosecond", "nanoseconds", "ns"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "minute",
        aliases: &["minute", "minutes", "min"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "hour",
        aliases: &["hour", "hours", "h", "hr"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "day",
        aliases: &["day", "days", "d"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "week",
        aliases: &["week", "weeks", "wk"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "month",
        aliases: &["month", "months"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "year",
        aliases: &["year", "years", "yr"],
        dimension: DIM.clone(),
    });

    g.add_edge("second", "millisecond", Conversion::Linear(1_000.0));
    g.add_edge("second", "microsecond", Conversion::Linear(1_000_000.0));
    g.add_edge("second", "nanosecond", Conversion::Linear(1_000_000_000.0));
    g.add_edge("second", "minute", Conversion::Linear(1.0 / 60.0));
    g.add_edge("second", "hour", Conversion::Linear(1.0 / 3_600.0));
    g.add_edge("second", "day", Conversion::Linear(1.0 / 86_400.0));
    g.add_edge("second", "week", Conversion::Linear(1.0 / 604_800.0));
    g.add_edge("second", "month", Conversion::Linear(1.0 / 2_629_746.0)); // avg month
    g.add_edge("second", "year", Conversion::Linear(1.0 / 31_556_952.0)); // avg year
}
