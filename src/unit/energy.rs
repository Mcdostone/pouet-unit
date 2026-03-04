use crate::{
    graph::{Dimension, UnitGraph, UnitInfo},
    unit::Conversion,
};

const DIM: Dimension = Dimension("energy");

pub fn register(g: &mut UnitGraph) {
    // Base unit: joule
    g.add_unit(UnitInfo {
        name: "joule",
        aliases: &["joule", "joules", "J"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "kilojoule",
        aliases: &["kilojoule", "kilojoules", "kJ"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "megajoule",
        aliases: &["megajoule", "megajoules", "MJ"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "calorie",
        aliases: &["calorie", "calories", "cal"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "kilocalorie",
        aliases: &["kilocalorie", "kilocalories", "kcal", "Cal"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "watt hour",
        aliases: &["watt hour", "watt hours", "Wh"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "kilowatt hour",
        aliases: &["kilowatt hour", "kilowatt hours", "kWh"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "electronvolt",
        aliases: &["electronvolt", "electronvolts", "eV"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "british thermal unit",
        aliases: &[
            "british thermal unit",
            "british thermal units",
            "BTU",
            "btu",
        ],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "foot pound",
        aliases: &["foot pound", "foot pounds", "ft·lb", "ft-lb"],
        dimension: DIM.clone(),
    });
    g.add_unit(UnitInfo {
        name: "erg",
        aliases: &["erg", "ergs"],
        dimension: DIM.clone(),
    });

    g.add_edge("joule", "kilojoule", Conversion::Linear(0.001));
    g.add_edge("joule", "megajoule", Conversion::Linear(1e-6));
    g.add_edge("joule", "calorie", Conversion::Linear(0.239_006));
    g.add_edge("joule", "kilocalorie", Conversion::Linear(0.000_239_006));
    g.add_edge("joule", "watt hour", Conversion::Linear(1.0 / 3_600.0));
    g.add_edge(
        "joule",
        "kilowatt hour",
        Conversion::Linear(1.0 / 3_600_000.0),
    );
    g.add_edge("joule", "electronvolt", Conversion::Linear(6.241_509e18));
    g.add_edge(
        "joule",
        "british thermal unit",
        Conversion::Linear(0.000_947_817),
    );
    g.add_edge("joule", "foot pound", Conversion::Linear(0.737_562));
    g.add_edge("joule", "erg", Conversion::Linear(1e7));
}
