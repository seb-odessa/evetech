use crate::common::Position;
use std::fmt;
use std::fmt::Display;


pub mod system;

pub use system::System;
pub use system::PlanetSurrounding;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, Eq)]
pub struct Star {
    pub name: String,
    pub age: u64,
    pub spectral_class: String,
    pub temperature: u32,
    pub radius: i64,
    pub solar_system_id: u32,
    pub type_id: u32,
}
impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "Age: {}", self.age)?;
        writeln!(f, "Spectral Class: {}", self.spectral_class)?;
        writeln!(f, "Temperature: {}", self.temperature)?;
        writeln!(f, "Radius: {}", self.radius)?;
        writeln!(f, "type_id: {}", self.type_id)?;
        writeln!(f, "System Id: {}", self.solar_system_id)
    }
}

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct StargateDestination {
    pub stargate_id: u32,
    pub system_id: u32,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Stargate {
    pub stargate_id: u32,
    pub name: String,
    pub position: Position,
    pub destination: StargateDestination,
    pub system_id: u32,
    pub type_id: u32,
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
pub struct Moon {
    pub moon_id: u32,
    pub name: String,
    pub position: Position,
    pub system_id: u32,
}
impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.moon_id, self.name)?;
        writeln!(f, "Position: {}", self.position)?;
        writeln!(f, "System Id: {}", self.system_id)
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Station {
    pub station_id: u32,
    pub name: String,
    pub position: Position,
    pub max_dockable_ship_volume: f32,
    pub office_rental_cost: f32,
    pub owner: Option<u32>,
    pub race_id: Option<u32>,
    pub reprocessing_efficiency: Option<f32>,
    pub reprocessing_station_take: Option<f32>,
    pub services: Option<Vec<String>>,
    pub system_id: u32,
    pub type_id: u32,
}
impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.station_id, self.name)?;
        writeln!(f, "Position: {}", self.position)?;
        writeln!(f, "Max Dockable Ship Volume: {}", self.max_dockable_ship_volume)?;
        writeln!(f, "Office Rental Cost: {}", self.office_rental_cost)?;
        if let Some(id) = self.owner {
            writeln!(f, "Owner: {}", id)?;
        }
        if let Some(id) = self.race_id {
            writeln!(f, "Race Id: {}", id)?;
        }
        if let Some(num) = self.reprocessing_efficiency {
            writeln!(f, "Reprocessing Efficiency: {}", num)?;
        }
        if let Some(num) = self.reprocessing_station_take {
            writeln!(f, "Reprocessing Station Take: {}", num)?;
        }
        write::<String>(" Services", &self.services, f)?;
        // if let Some(services) = &self.services {
        //     writeln!(f, "Services: {}", services)?;
        // }
        writeln!(f, "System Id: {}", self.system_id)?;
        writeln!(f, "Type Id: {}", self.type_id)
    }
}

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

fn write<T>(title: &str, vec: &Option<Vec<T>>, f: &mut fmt::Formatter) -> fmt::Result
where
    T: Display + ToString
{
    if let Some(ids) = &vec {
        let joined = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(f, "{title}: {}", joined)
    } else {
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn system() {
        let system = Esi::new().system().await;
        assert!(system.is_ok());
    }
}
