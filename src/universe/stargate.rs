use crate::common::Position;
use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::client::PARAM;
use crate::esi::client::UNIVERSE;

use anyhow::anyhow;

use std::fmt;

impl Uri for Stargate {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Id(id) = id {
            Ok(format!("{UNIVERSE}/stargates/{id}/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Id(i32)"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Stargate {
    pub stargate_id: i32,
    pub name: String,
    pub position: Position,
    pub destination: StargateDestination,
    pub system_id: i32,
    pub type_id: i32,
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
pub struct StargateDestination {
    pub stargate_id: i32,
    pub system_id: i32,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::esi::EveApi;

    #[tokio::test]
    async fn load() -> anyhow::Result<()> {
        let api = EveApi::new();
        let obj = api.load::<Stargate>(&Uid::Id(50011094)).await?;
        assert_eq!(obj.stargate_id, 50011094);
        assert_eq!(obj.system_id, 30002080);
        assert_eq!(obj.name, "Stargate (Dudreda)");
        Ok(())
    }
}
