use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::MARKETS;
use crate::esi::PARAM;

use anyhow::anyhow;

impl Uri for Group {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Id(id) = id {
            Ok(format!("{MARKETS}/groups/{id}/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Id(i32)"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Group {
    pub market_group_id: i32,
    pub name: String,
    pub parent_group_id: Option<i32>,
    pub description: String,
    pub types: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::esi::EveApi;

    #[tokio::test]
    async fn load() -> anyhow::Result<()> {
        let api = EveApi::new();
        let obj = api.load::<Group>(&Uid::Id(4)).await?;
        assert_eq!(obj.market_group_id, 4);
        assert_eq!(obj.name, "Ships");
        assert_eq!(obj.parent_group_id, None);
        assert_eq!(obj.description, "Capsuleer spaceships of all sizes and roles, including advanced and faction variants of many hull types");
        assert_eq!(obj.types, Vec::<i32>::new());
        Ok(())
    }
}
