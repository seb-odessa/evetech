use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::client::PARAM;
use crate::esi::client::UNIVERSE;

use anyhow::anyhow;

impl Uri for SearchResult {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Empty = id {
            Ok(format!("{UNIVERSE}/ids/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Empty"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Object {
    pub id: i32,
    pub name: String,
}
impl Object {
    pub fn new<T: Into<String>>(id: i32, name: T) -> Self {
        Self {
            id: id,
            name: name.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct SearchResult {
    pub agents: Option<Vec<Object>>,
    pub alliances: Option<Vec<Object>>,
    pub characters: Option<Vec<Object>>,
    pub constellations: Option<Vec<Object>>,
    pub corporations: Option<Vec<Object>>,
    pub factions: Option<Vec<Object>>,
    pub inventory_types: Option<Vec<Object>>,
    pub regions: Option<Vec<Object>>,
    pub stations: Option<Vec<Object>>,
    pub systems: Option<Vec<Object>>,
}
pub enum ItemType {
    Agents,
    Alliances,
    Characters,
    Constellations,
    Corporations,
    Factions,
    Inventory,
    Regions,
    Stations,
    Systems,
}
impl SearchResult {
    pub fn one(&self, requested_type: ItemType) -> Option<Object> {
        match requested_type {
            ItemType::Agents => self.agents.as_ref().and_then(|v| v.first().cloned()),
            ItemType::Alliances => self.agents.as_ref().and_then(|v| v.first().cloned()),
            ItemType::Characters => self.agents.as_ref().and_then(|v| v.first().cloned()),
            ItemType::Constellations => self.agents.as_ref().and_then(|v| v.first().cloned()),
            ItemType::Corporations => self.agents.as_ref().and_then(|v| v.first().cloned()),
            ItemType::Factions => self.agents.as_ref().and_then(|v| v.first().cloned()),
            ItemType::Inventory => self.agents.as_ref().and_then(|v| v.first().cloned()),
            ItemType::Regions => self.agents.as_ref().and_then(|v| v.first().cloned()),
            ItemType::Stations => self.agents.as_ref().and_then(|v| v.first().cloned()),
            ItemType::Systems => self.agents.as_ref().and_then(|v| v.first().cloned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::esi::EveApi;

    #[tokio::test]
    async fn load() -> anyhow::Result<()> {
        let api = EveApi::new();
        let names = vec![String::from("Jovian Researcher")];
        let obj = api.search(&names).await?;
        let expected = SearchResult {
            agents: None,
            alliances: None,
            characters: Some(vec![Object {
                id: 2115657646,
                name: String::from("Jovian Researcher"),
            }]),
            constellations: None,
            corporations: None,
            factions: None,
            inventory_types: None,
            regions: None,
            stations: None,
            systems: None,
        };
        assert_eq!(obj, expected);

        Ok(())
    }
}
