use std::fmt;
use crate::common::Position;
use crate::universe::utils;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Constellation {
    pub constellation_id: u32,
    pub name: String,
    pub position: Position,
    pub region_id: u32,
    pub systems: Vec<u32>,
}
impl fmt::Display for Constellation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.constellation_id, self.name)?;
        writeln!(f, "Position: {}", self.position)?;
        writeln!(f, "Region Id: {}", self.region_id)?;
        utils::write("  Systems", &Some(self.systems.clone()), f)
    }
}