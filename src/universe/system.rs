use std::fmt;
use crate::common::Position;
use crate::universe::utils;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct System {
    pub system_id: u32,
    pub name: String,
    pub constellation_id: u32,
    pub position: Position,
    pub security_status: f32,
    pub security_class: Option<String>,
    pub star_id: Option<u32>,
    pub planets: Option<Vec<CelestialBodies>>,
    pub stargates: Option<Vec<u32>>,
    pub stations: Option<Vec<u32>>,
}
impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.system_id, self.name)?;
        writeln!(f, "Constellation Id: {}", self.constellation_id)?;
        writeln!(f, "Security Status: {}", self.security_status)?;
        if let Some(ref sc) = self.security_class {
            writeln!(f, "Security Class: {}", sc)?;
        }
        if let Some(ref id) = self.star_id {
            writeln!(f, "Star Id: {}", id)?;
        }
        writeln!(f, "Position: {}", self.position)?;
        if let Some(planets) = &self.planets {
            for planet in planets {
                write!(f, "{}", planet)?;
            }
        }
        utils::write(" Stargates", &self.stargates, f)?;
        utils::write(" Stations", &self.stations, f)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, Eq)]
pub struct CelestialBodies {
    pub planet_id: u32,
    pub asteroid_belts: Option<Vec<u32>>,
    pub moons: Option<Vec<u32>>,
}
impl fmt::Display for CelestialBodies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "  Planet Id: {}", self.planet_id)?;
        utils::write("  Belts Ids", &self.asteroid_belts, f)?;
        utils::write("  Moons Ids", &self.moons, f)?;
        write!(f, "")
    }
}
