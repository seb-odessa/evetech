use std::fmt;
use crate::common::Position;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct WayPoint {
    pub id: i32,
    pub name: String,
    pub position: Position,
}
impl WayPoint {
    pub fn new(id: i32, name: &str, position: &Position) -> Self {
        Self {
            id,
            name: name.to_string(),
            position: position.clone(),
        }
    }
    pub fn distance_to(&self, other: &Self) -> f64 {
        self.position.distance_to(&other.position)
    }
}
impl fmt::Display for WayPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.id, self.name)
    }
}