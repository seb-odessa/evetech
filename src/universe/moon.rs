use crate::common::Position;
use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::PARAM;
use crate::esi::UNIVERSE;
use std::fmt;

use anyhow::anyhow;

impl Uri for Moon {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Id(id) = id {
            Ok(format!("{UNIVERSE}/moons/{id}/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Id(i32)"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Moon {
    pub moon_id: i32,
    pub name: String,
    pub position: Position,
    pub system_id: i32,
}
impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.moon_id, self.name)?;
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
        let obj = api.load::<Moon>(&Uid::Id(40132803)).await?;
        assert_eq!(obj.system_id, 30002080);
        assert_eq!(obj.name, "Arifsdald I - Moon 1");
        Ok(())
    }
}
