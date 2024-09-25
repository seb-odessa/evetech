use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::PARAM;
use crate::esi::UNIVERSE;
use crate::universe::utils;
use std::fmt;

use anyhow::anyhow;

impl Uri for Region {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Id(id) = id {
            Ok(format!("{UNIVERSE}/regions/{id}/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Id(i32)"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Region {
    pub region_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub constellations: Vec<i32>,
}
impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.region_id, self.name)?;
        utils::write("  Constellations", &Some(self.constellations.clone()), f)?;
        if let Some(description) = &self.description {
            writeln!(f, "Description: {}", description)?;
        }
        write!(f, "")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::esi::EveApi;

    #[tokio::test]
    async fn load() -> anyhow::Result<()> {
        let api = EveApi::new();
        let obj = api.load::<Region>(&Uid::Id(10000042)).await?;
        assert_eq!(obj.region_id, 10000042);
        assert_eq!(obj.name, "Metropolis");
        Ok(())
    }
}
