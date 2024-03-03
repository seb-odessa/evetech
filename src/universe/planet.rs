use std::fmt;
use crate::common::Position;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Planet {
    pub planet_id: u32,
    pub name: String,
    pub position: Position,
    pub system_id: u32,
    pub type_id: u32,
}
impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.planet_id, self.name)?;
        writeln!(f, "Position: {}", self.position)?;
        writeln!(f, "System Id: {}", self.system_id)?;
        writeln!(f, "type_id: {}", self.type_id)
    }
}