use crate::common::Position;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AsteroidBelts {
    pub name: String,
    pub position: Position,
    pub system_id: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Object {
    pub id: i32,
    pub name: String,
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn dummy() {
        assert_relative_eq!(0.0, 0.0);
    }

    #[test]
    fn distance() {
        let zero = Position::new(0.0, 0.0, 0.0);
        let one_x = Position::new(1.0, 0.0, 0.0);
        let one_y = Position::new(0.0, 1.0, 0.0);
        let one_z = Position::new(0.0, 0.0, 1.0);

        assert_relative_eq!(Position::distance(&zero, &zero), 0.0);
        assert_relative_eq!(Position::distance(&zero, &one_x), 1.0);
        assert_relative_eq!(Position::distance(&zero, &one_y), 1.0);
        assert_relative_eq!(Position::distance(&zero, &one_z), 1.0);
    }
}
