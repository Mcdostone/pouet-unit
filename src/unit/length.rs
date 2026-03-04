use crate::{
    graph::{Dimension, UnitGraph, UnitInfo},
    unit::Conversion,
};

const DIM: Dimension = Dimension("length");

pub fn register(g: &mut UnitGraph) {
    // Base unit: meter
    g.add_unit(UnitInfo {
        name: "meter",
        aliases: &["meter", "meters", "metre", "metres", "m"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "kilometer",
        aliases: &["kilometer", "kilometers", "kilometre", "kilometres", "km"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "centimeter",
        aliases: &[
            "centimeter",
            "centimeters",
            "centimetre",
            "centimetres",
            "cm",
        ],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "millimeter",
        aliases: &[
            "millimeter",
            "millimeters",
            "millimetre",
            "millimetres",
            "mm",
        ],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "micrometer",
        aliases: &[
            "micrometer",
            "micrometers",
            "micrometre",
            "micrometres",
            "µm",
            "um",
        ],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "nanometer",
        aliases: &["nanometer", "nanometers", "nanometre", "nanometres", "nm"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "mile",
        aliases: &["mile", "miles", "mi"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "yard",
        aliases: &["yard", "yards", "yd"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "foot",
        aliases: &["foot", "feet", "ft"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "inch",
        aliases: &["inch", "inches", "in"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "nautical mile",
        aliases: &["nautical mile", "nautical miles", "nmi", "NM"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "light year",
        aliases: &["light year", "light years", "ly"],
        dimension: DIM.clone(),
    });

    // Edges (relative to meter as base)
    g.add_edge("meter", "kilometer", Conversion::Linear(0.001));
    g.add_edge("meter", "centimeter", Conversion::Linear(100.0));
    g.add_edge("meter", "millimeter", Conversion::Linear(1_000.0));
    g.add_edge("meter", "micrometer", Conversion::Linear(1_000_000.0));
    g.add_edge("meter", "nanometer", Conversion::Linear(1_000_000_000.0));
    g.add_edge("meter", "mile", Conversion::Linear(0.000_621_371));
    g.add_edge("meter", "yard", Conversion::Linear(1.093_613));
    g.add_edge("meter", "foot", Conversion::Linear(3.280_840));
    g.add_edge("meter", "inch", Conversion::Linear(39.370_079));
    g.add_edge("meter", "nautical mile", Conversion::Linear(1.0 / 1852.0));
    g.add_edge("meter", "light year", Conversion::Linear(1.0 / 9.461e15));
}
