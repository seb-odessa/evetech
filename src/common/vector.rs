use super::position::Position;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Vector {
    pub beg: Position,
    pub end: Position,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

impl Vector {
    pub fn new(beg: Position, end: Position) -> Self {
        // https://ru.onlinemschool.com/math/assistance/vector/p_to_vector/
        let dx = end.x - beg.x;
        let dy = end.y - beg.y;
        let dz = end.z - beg.z;
        Self {
            beg,
            end,
            dx,
            dy,
            dz,
        }
    }

    pub fn length(&self) -> f64 {
        // https://ru.onlinemschool.com/math/assistance/vector/length/
        (self.dx.powi(2) + self.dy.powi(2) + self.dz.powi(2)).sqrt()
    }

    pub fn scalar_product(a: &Self, b: &Self) -> f64 {
        // https://ru.onlinemschool.com/math/assistance/vector/multiply/
        a.dx * b.dx + a.dy * b.dy + a.dz * b.dz
    }

    pub fn cos_angl(a: &Self, b: &Self) -> f64 {
        // https://ru.onlinemschool.com/math/library/vector/angl/
        Self::scalar_product(a, b) / a.length() / b.length()
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}; {}; {} }}", self.dx, self.dy, self.dz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn length() {
        let zero = Position::zero();
        let one_x = Position::new(1.0, 0.0, 0.0);
        let one_y = Position::new(0.0, 1.0, 0.0);
        let one_z = Position::new(0.0, 0.0, 1.0);

        assert_relative_eq!(Vector::new(zero.clone(), zero.clone()).length(), 0.0);
        assert_relative_eq!(Vector::new(zero.clone(), one_x).length(), 1.0);
        assert_relative_eq!(Vector::new(zero.clone(), one_y).length(), 1.0);
        assert_relative_eq!(Vector::new(zero.clone(), one_z).length(), 1.0);
    }

    #[test]
    fn scalar_product() {
        let zero = Position::zero();
        let one = Vector::new(zero.clone(), Position::new(2.0, 3.0, 4.0));
        let two = Vector::new(zero.clone(), Position::new(1.0, 2.0, 3.0));

        assert_relative_eq!(Vector::scalar_product(&one, &two), 20.0);
    }

    #[test]
    fn cos_angl() {
        let zero = Position::zero();
        let one = Vector::new(zero.clone(), Position::new(2.0, 3.0, 4.0));
        let two = Vector::new(zero.clone(), Position::new(1.0, 2.0, 3.0));

        assert_relative_eq!(Vector::cos_angl(&one, &two), 0.9925833339709302);
    }

}
