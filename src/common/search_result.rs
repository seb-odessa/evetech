use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Object {
    pub id: u32,
    pub name: String,
}
impl Object {
    pub fn new<T: Into<String>>(id: u32, name: T) -> Self {
        Self {
            id: id,
            name: name.into(),
        }
    }
}
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.name)
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
