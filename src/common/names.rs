use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::PARAM;
use crate::esi::UNIVERSE;

use anyhow::anyhow;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Names {
    pub id: i32,
    pub name: String,
    pub category: Category,
}
impl Uri for Names {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Empty = id {
            Ok(format!("{UNIVERSE}/names/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Empty"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Category {
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
    Faction,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::esi::EveApi;

    #[tokio::test]
    async fn load() -> anyhow::Result<()> {
        let api = EveApi::new();
        let ids = vec![21918];
        let obj = api.names(&ids).await?;
        let expected = vec![Names {
            id: 21918,
            name: String::from("Republic Fleet Phased Plasma L"),
            category: Category::InventoryType,
        }];

        assert_eq!(obj, expected);

        Ok(())
    }
}
