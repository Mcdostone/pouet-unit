use crate::{
    graph::{Dimension, UnitGraph, UnitInfo},
    unit::Conversion,
};

const DIM: Dimension = Dimension("volume");

pub fn register(g: &mut UnitGraph) {
    // Base unit: liter
    g.add_unit(UnitInfo {
        name: "liter",
        aliases: &["liter", "liters", "litre", "litres", "l", "L"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "milliliter",
        aliases: &[
            "milliliter",
            "milliliters",
            "millilitre",
            "millilitres",
            "ml",
            "mL",
        ],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "centiliter",
        aliases: &[
            "centiliter",
            "centiliters",
            "centilitre",
            "centilitres",
            "cl",
        ],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "deciliter",
        aliases: &["deciliter", "deciliters", "decilitre", "decilitres", "dl"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "cubic meter",
        aliases: &[
            "cubic meter",
            "cubic meters",
            "cubic metre",
            "cubic metres",
            "m³",
            "m3",
        ],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "cubic centimeter",
        aliases: &["cubic centimeter", "cubic centimeters", "cc", "cm³", "cm3"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "gallon",
        aliases: &["gallon", "gallons", "gal"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "quart",
        aliases: &["quart", "quarts", "qt"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "pint",
        aliases: &["pint", "pints", "pt"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "cup",
        aliases: &["cup", "cups"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "fluid ounce",
        aliases: &["fluid ounce", "fluid ounces", "fl oz", "floz"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "tablespoon",
        aliases: &["tablespoon", "tablespoons", "tbsp"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "teaspoon",
        aliases: &["teaspoon", "teaspoons", "tsp"],
        dimension: DIM.clone(),
    });

    // Edges (all relative to liter)
    g.add_edge("liter", "milliliter", Conversion::Linear(1_000.0));
    g.add_edge("liter", "centiliter", Conversion::Linear(100.0));
    g.add_edge("liter", "deciliter", Conversion::Linear(10.0));
    g.add_edge("liter", "cubic meter", Conversion::Linear(0.001));
    g.add_edge("liter", "cubic centimeter", Conversion::Linear(1_000.0));
    g.add_edge("liter", "gallon", Conversion::Linear(0.264_172));
    g.add_edge("liter", "quart", Conversion::Linear(1.056_688));
    g.add_edge("liter", "pint", Conversion::Linear(2.113_376));
    g.add_edge("liter", "cup", Conversion::Linear(4.226_753));
    g.add_edge("liter", "fluid ounce", Conversion::Linear(33.814_02));
    g.add_edge("liter", "tablespoon", Conversion::Linear(67.628_04));
    g.add_edge("liter", "teaspoon", Conversion::Linear(202.884_1));
}
