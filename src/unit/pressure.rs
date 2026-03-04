use crate::{
    graph::{Dimension, UnitGraph, UnitInfo},
    unit::Conversion,
};

const DIM: Dimension = Dimension("pressure");

pub fn register(g: &mut UnitGraph) {
    // Base unit: pascal
    g.add_unit(UnitInfo {
        name: "pascal",
        aliases: &["pascal", "pascals", "Pa"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "kilopascal",
        aliases: &["kilopascal", "kilopascals", "kPa"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "megapascal",
        aliases: &["megapascal", "megapascals", "MPa"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "bar",
        aliases: &["bar", "bars"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "millibar",
        aliases: &["millibar", "millibars", "mbar"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "atmosphere",
        aliases: &["atmosphere", "atmospheres", "atm"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "torr",
        aliases: &["torr"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "mmhg",
        aliases: &["mmhg", "mmHg", "mm Hg", "millimeter of mercury"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "psi",
        aliases: &["psi", "PSI"],
        dimension: DIM.clone(),
    });

    g.add_edge("pascal", "kilopascal", Conversion::Linear(0.001));
    g.add_edge("pascal", "megapascal", Conversion::Linear(1e-6));
    g.add_edge("pascal", "bar", Conversion::Linear(1e-5));
    g.add_edge("pascal", "millibar", Conversion::Linear(0.01));
    g.add_edge("pascal", "atmosphere", Conversion::Linear(1.0 / 101_325.0));
    g.add_edge("pascal", "torr", Conversion::Linear(0.007_500_617));
    g.add_edge("pascal", "mmhg", Conversion::Linear(0.007_500_617)); // 1 torr ≈ 1 mmHg
    g.add_edge("pascal", "psi", Conversion::Linear(0.000_145_038));
}
