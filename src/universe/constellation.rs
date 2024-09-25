use crate::common::Position;
use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::PARAM;
use crate::esi::UNIVERSE;
use crate::universe::utils;
use std::fmt;

use anyhow::anyhow;

impl Uri for Constellation {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Id(id) = id {
            Ok(format!("{UNIVERSE}/constellations/{id}/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Id(i32)"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Constellation {
    pub constellation_id: i32,
    pub name: String,
    pub position: Position,
    pub region_id: i32,
    pub systems: Vec<i32>,
}
impl fmt::Display for Constellation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.constellation_id, self.name)?;
        writeln!(f, "Position: {}", self.position)?;
        writeln!(f, "Region Id: {}", self.region_id)?;
        utils::write("  Systems", &Some(self.systems.clone()), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::esi::EveApi;

    #[tokio::test]
    async fn load() -> anyhow::Result<()> {
        let api = EveApi::new();
        let obj = api.load::<Constellation>(&Uid::Id(20000306)).await?;
        assert_eq!(obj.constellation_id, 20000306);
        assert_eq!(obj.name, "Fribrodi");
        Ok(())
    }
}
