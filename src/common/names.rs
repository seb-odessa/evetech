use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Names {
    pub id: i32,
    pub name: String,
    pub category: Categories,
}

// #[serde(rename_all = "snake_case")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Categories {
    #[serde(rename = "alliance")]
    Alliance,
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "constellation")]
    Constellation,
    #[serde(rename = "corporation")]
    Corporation,
    #[serde(rename = "inventory_type")]
    InventoryType,
    #[serde(rename = "region")]
    Region,
    #[serde(rename = "solar_system")]
    SolarSystem,
    #[serde(rename = "station")]
    Station,
    #[serde(rename = "faction")]
    Faction
}

impl fmt::Display for Names {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {:?})", self.id, self.name, self.category)
    }
}