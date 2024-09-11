use std::fmt;
use crate::common::Position;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct WayPoint {
    pub id: i32,
    pub name: String,
    pub position: Position,
}
impl WayPoint {
    pub fn new(id: i32, name: &String, position: &Position) -> Self {
        Self {
            id: id,
            name: name.clone(),
            position: position.clone(),
        }
    }
}
impl fmt::Display for WayPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.id, self.name, self.position)
    }
}