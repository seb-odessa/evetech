use crate::common::Position;
use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::client::PARAM;
use crate::esi::client::UNIVERSE;
use crate::universe::utils;

use anyhow::anyhow;

use std::fmt;

impl Uri for Station {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Id(id) = id {
            Ok(format!("{UNIVERSE}/stations/{id}/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Id(i32)"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Station {
    pub station_id: i32,
    pub name: String,
    pub position: Position,
    pub max_dockable_ship_volume: f32,
    pub office_rental_cost: f32,
    pub owner: Option<i32>,
    pub race_id: Option<i32>,
    pub reprocessing_efficiency: Option<f32>,
    pub reprocessing_station_take: Option<f32>,
    pub services: Option<Vec<String>>,
    pub system_id: i32,
    pub type_id: i32,
}
impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.station_id, self.name)?;
        writeln!(f, "Position: {}", self.position)?;
        writeln!(
            f,
            "Max Dockable Ship Volume: {}",
            self.max_dockable_ship_volume
        )?;
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
        utils::write::<String>(" Services", &self.services, f)?;
        writeln!(f, "System Id: {}", self.system_id)?;
        writeln!(f, "Type Id: {}", self.type_id)
    }
}
