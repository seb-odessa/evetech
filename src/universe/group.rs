use crate::universe::utils;
use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::PARAM;
use crate::esi::UNIVERSE;
use std::fmt;

use anyhow::anyhow;

impl Uri for Group {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Id(id) = id {
            Ok(format!("{UNIVERSE}/groups/{id}/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Id(i32)"))
        }
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Group {
    pub group_id: i32,
    pub name: String,
    pub category_id: i32,
    pub published: bool,
    pub types: Vec<i32>,
}
impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.group_id, self.name)?;
        writeln!(f, "Published: {}", self.published)?;
        writeln!(f, "Category Id: {}", self.category_id)?;
        utils::write("Types", &Some(self.types.clone()), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::esi::EveApi;

    #[tokio::test]
    async fn load() -> anyhow::Result<()> {
        let api = EveApi::new();
        let obj = api.load::<Group>(&Uid::Id(15)).await?;
        assert_eq!(obj.group_id, 15);
        assert_eq!(obj.name, "Station");
        assert_eq!(obj.category_id, 3);
        Ok(())
    }
}