use serde::Deserialize;
use std::fmt::Debug;

use crate::esi::client::KILLMAILS;
use crate::esi::client::PARAM;
use crate::killmails::killmail;

use super::{ApiClient, EveSwaggerClient};

pub enum Uid {
    Id(i32),
    Killmail(i32, String),
}

pub trait Uri: Clone + Send + Sync + 'static {
    fn uri(id: &Uid) -> anyhow::Result<String>;
}

pub struct EveApi {
    client: EveSwaggerClient,
}
impl EveApi {
    pub fn new() -> Self {
        Self {
            client: EveSwaggerClient::new(),
        }
    }

    pub async fn load<T>(&self, id: &Uid) -> anyhow::Result<T>
    where
        T: Uri + Debug + for<'de> Deserialize<'de>,
    {
        let uri = T::uri(id)?;
        let object = self.client.get::<T>(uri).await?;
        Ok(object)
    }

    pub async fn load_killmail<S: Into<String>>(
        &self,
        id: i32,
        hash: S,
    ) -> anyhow::Result<killmail::Killmail> {
        let hash = hash.into();
        let uri = format!("{KILLMAILS}/{id}/{hash}/?{PARAM}");
        let obj = self.client.get::<killmail::Killmail>(uri).await?;
        Ok(obj)
    }
}

// #[async_trait]
// impl LoadableById<universe::Station> for CachedClient {
//     type Output = ;

// }
