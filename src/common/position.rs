use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance(a: &Self, b: &Self) -> f64 {
        ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
    }

    pub fn distance_to(self: &Self, other: &Self) -> f64 {
        Position::distance(&self, other)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

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

    #[test]
    fn distance_to() {
        let zero = Position::new(0.0, 0.0, 0.0);
        let one_x = Position::new(1.0, 0.0, 0.0);
        let one_y = Position::new(0.0, 1.0, 0.0);
        let one_z = Position::new(0.0, 0.0, 1.0);

        assert_relative_eq!(zero.distance_to(&zero), 0.0);
        assert_relative_eq!(zero.distance_to(&one_x), 1.0);
        assert_relative_eq!(zero.distance_to(&one_y), 1.0);
        assert_relative_eq!(zero.distance_to(&one_z), 1.0);
    }
}
