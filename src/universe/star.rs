use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::PARAM;
use crate::esi::UNIVERSE;
use std::fmt;

use anyhow::anyhow;

impl Uri for Star {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Id(id) = id {
            Ok(format!("{UNIVERSE}/stars/{id}/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Id(i32)"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, Eq)]
pub struct Star {
    pub name: String,
    pub age: u64,
    pub spectral_class: String,
    pub temperature: i32,
    pub radius: i64,
    pub solar_system_id: i32,
    pub type_id: i32,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::esi::EveApi;

    #[tokio::test]
    async fn load() -> anyhow::Result<()> {
        let api = EveApi::new();
        let obj = api.load::<Star>(&Uid::Id(40132801)).await?;
        assert_eq!(obj.solar_system_id, 30002080);
        assert_eq!(obj.name, "Arifsdald - Star");
        Ok(())
    }
}
