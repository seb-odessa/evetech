use crate::common::Position;
use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::client::PARAM;
use crate::esi::client::UNIVERSE;

use std::fmt;

use anyhow::anyhow;

impl Uri for AsteroidBelt {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Id(id) = id {
            Ok(format!("{UNIVERSE}/asteroid_belts/{id}/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Id(i32)"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AsteroidBelt {
    pub name: String,
    pub position: Position,
    pub system_id: i32,
}
impl fmt::Display for AsteroidBelt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "Position: {}", self.position)?;
        writeln!(f, "System Id: {}", self.system_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::esi::EveApi;

    #[tokio::test]
    async fn load() -> anyhow::Result<()> {
        let api = EveApi::new();
        let obj = api.load::<AsteroidBelt>(&Uid::Id(40132822)).await?;
        assert_eq!(obj.system_id, 30002080);
        assert_eq!(obj.name, "Arifsdald II - Asteroid Belt 1");
        Ok(())
    }
}
