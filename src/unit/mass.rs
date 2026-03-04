use crate::{
    graph::{Dimension, UnitGraph, UnitInfo},
    unit::Conversion,
};

const DIM: Dimension = Dimension("mass");

pub fn register(g: &mut UnitGraph) {
    // Base unit: kilogram
    g.add_unit(UnitInfo {
        name: "kilogram",
        aliases: &["kilogram", "kilograms", "kg"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "gram",
        aliases: &["gram", "grams", "g"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "milligram",
        aliases: &["milligram", "milligrams", "mg"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "tonne",
        aliases: &["tonne", "tonnes", "metric ton", "metric tons", "t"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "pound",
        aliases: &["pound", "pounds", "lb", "lbs"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "ounce",
        aliases: &["ounce", "ounces", "oz"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "stone",
        aliases: &["stone", "stones", "st"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "microgram",
        aliases: &["microgram", "micrograms", "µg", "ug"],
        dimension: DIM.clone(),
    });

    g.add_edge("kilogram", "gram", Conversion::Linear(1_000.0));
    g.add_edge("kilogram", "milligram", Conversion::Linear(1_000_000.0));
    g.add_edge("kilogram", "microgram", Conversion::Linear(1_000_000_000.0));
    g.add_edge("kilogram", "tonne", Conversion::Linear(0.001));
    g.add_edge("kilogram", "pound", Conversion::Linear(2.204_622_6));
    g.add_edge("kilogram", "ounce", Conversion::Linear(35.274_0));
    g.add_edge("kilogram", "stone", Conversion::Linear(0.157_473));
}
