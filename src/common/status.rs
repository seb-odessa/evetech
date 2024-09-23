use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::client::PARAM;
use crate::esi::client::STATUS;

use anyhow::anyhow;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Status {
    pub players: i32,
    pub server_version: String,
    pub start_time: String,
    pub vip: Option<bool>,
}
impl Uri for Status {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Empty = id {
            Ok(format!("{STATUS}/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Id(i32)"))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::esi::EveApi;

    #[tokio::test]
    async fn load() -> anyhow::Result<()> {
        let api = EveApi::new();
        let obj = api.load::<Status>(&Uid::Empty).await?;
        assert!(obj.players > 0);
        assert!(!obj.server_version.is_empty());
        Ok(())
    }
}

