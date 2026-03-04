use crate::{
    graph::{Dimension, UnitGraph, UnitInfo},
    unit::Conversion,
};

const DIM: Dimension = Dimension("area");

pub fn register(g: &mut UnitGraph) {
    // Base unit: square meter
    g.add_unit(UnitInfo {
        name: "square meter",
        aliases: &[
            "square meter",
            "square meters",
            "square metre",
            "square metres",
            "m²",
            "m2",
        ],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "square kilometer",
        aliases: &[
            "square kilometer",
            "square kilometers",
            "square kilometre",
            "square kilometres",
            "km²",
            "km2",
        ],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "square centimeter",
        aliases: &["square centimeter", "square centimeters", "cm²", "cm2"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "square millimeter",
        aliases: &["square millimeter", "square millimeters", "mm²", "mm2"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "square mile",
        aliases: &["square mile", "square miles", "mi²", "mi2"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "square yard",
        aliases: &["square yard", "square yards", "yd²", "yd2"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "square foot",
        aliases: &["square foot", "square feet", "ft²", "ft2"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "square inch",
        aliases: &["square inch", "square inches", "in²", "in2"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "hectare",
        aliases: &["hectare", "hectares", "ha"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "acre",
        aliases: &["acre", "acres"],
        dimension: DIM.clone(),
    });

    g.add_edge("square meter", "square kilometer", Conversion::Linear(1e-6));
    g.add_edge(
        "square meter",
        "square centimeter",
        Conversion::Linear(10_000.0),
    );
    g.add_edge(
        "square meter",
        "square millimeter",
        Conversion::Linear(1_000_000.0),
    );
    g.add_edge(
        "square meter",
        "square mile",
        Conversion::Linear(3.861_022e-7),
    );
    g.add_edge("square meter", "square yard", Conversion::Linear(1.195_990));
    g.add_edge("square meter", "square foot", Conversion::Linear(10.763_91));
    g.add_edge("square meter", "square inch", Conversion::Linear(1_550.003));
    g.add_edge("square meter", "hectare", Conversion::Linear(1e-4));
    g.add_edge("square meter", "acre", Conversion::Linear(0.000_247_105));
}
