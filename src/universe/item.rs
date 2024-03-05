use crate::universe::utils;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Type {
    pub type_id: u32,
    pub name: String,
    pub group_id: u32,
    pub published: bool,

    pub graphic_id: Option<u32>,
    pub icon_id: Option<u32>,
    pub market_group_id: Option<u32>,

    pub mass: Option<f32>,
    pub radius: Option<f32>,
    pub capacity: Option<f32>,

    pub volume: Option<f32>,
    pub packaged_volume: Option<f32>,
    pub portion_size: Option<u32>,

    pub description: String,

    pub dogma_attributes: Option<Vec<DogmaAttributes>>,
    pub dogma_effects: Option<Vec<DogmaEffects>>,
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.type_id, self.name)?;
        writeln!(f, "Published: {}", self.published)?;
        writeln!(f, "Group Id: {}", self.group_id)?;
        if let Some(id) = self.icon_id {
            writeln!(f, "Icon Id: {}", id)?;
        }
        if let Some(value) = self.graphic_id {
            writeln!(f, "Graphic Id: {}", value)?;
        }
        if let Some(id) = self.market_group_id {
            writeln!(f, "Market Group Id: {}", id)?;
        }
        if let Some(value) = self.volume {
            writeln!(f, "Volume: {}", value)?;
        }
        if let Some(value) = self.packaged_volume {
            writeln!(f, "Packaged Volume: {}", value)?;
        }
        if let Some(value) = self.portion_size {
            writeln!(f, "Portion Size: {}", value)?;
        }
        if let Some(value) = self.mass {
            writeln!(f, "Mass: {}", value)?;
        }
        if let Some(value) = self.radius {
            writeln!(f, "Radius: {}", value)?;
        }
        if let Some(value) = self.capacity {
            writeln!(f, "Capacity: {}", value)?;
        }
        writeln!(f, "Description: {}", self.description)?;

        utils::write("Dogma Attributes", &self.dogma_attributes, f)?;
        utils::write("Dogma Effects", &self.dogma_effects, f)?;

        write!(f, "")
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct DogmaAttributes {
    pub attribute_id: u32,
    pub value: f32,
}
impl fmt::Display for DogmaAttributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.attribute_id, self.value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct DogmaEffects {
    pub effect_id: u32,
    pub is_default: bool,
}
impl fmt::Display for DogmaEffects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.effect_id, self.is_default)
    }
}
