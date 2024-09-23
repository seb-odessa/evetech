use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::client::PARAM;
use crate::esi::client::UNIVERSE;
use crate::universe::utils;
use std::fmt;

use anyhow::anyhow;

impl Uri for Category {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Id(id) = id {
            Ok(format!("{UNIVERSE}/categories/{id}/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Id(i32)"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Category {
    pub category_id: i32,
    pub name: String,
    pub published: bool,
    pub groups: Vec<i32>,
}
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.category_id, self.name)?;
        writeln!(f, "Published: {}", self.published)?;
        utils::write("Types", &Some(self.groups.clone()), f)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::esi::EveApi;

    #[tokio::test]
    async fn load() -> anyhow::Result<()> {
        let api = EveApi::new();
        let obj = api.load::<Category>(&Uid::Id(3)).await?;
        assert_eq!(obj.category_id, 3);
        assert_eq!(obj.name, "Station");
        Ok(())
    }
}
