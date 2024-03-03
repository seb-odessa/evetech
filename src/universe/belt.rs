use std::fmt;
use crate::common::Position;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AsteroidBelts {
    pub name: String,
    pub position: Position,
    pub system_id: u32,
}
impl fmt::Display for AsteroidBelts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "Position: {}", self.position)?;
        writeln!(f, "System Id: {}", self.system_id)
    }
}

