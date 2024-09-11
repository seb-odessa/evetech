use std::fmt;
use crate::common::Position;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Stargate {
    pub stargate_id: i32,
    pub name: String,
    pub position: Position,
    pub destination: StargateDestination,
    pub system_id: i32,
    pub type_id: i32,
}
impl fmt::Display for Stargate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.stargate_id, self.name)?;
        writeln!(f, "Position: {}", self.position)?;
        writeln!(f, "System Id: {}", self.system_id)?;
        writeln!(f, "type_id: {}", self.type_id)?;
        writeln!(f, "Destination: {}", self.destination)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct StargateDestination {
    pub stargate_id: i32,
    pub system_id: i32,
}
impl fmt::Display for StargateDestination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            " Stargate Id: {} System Id: {}",
            self.stargate_id, self.system_id
        )
    }
}
