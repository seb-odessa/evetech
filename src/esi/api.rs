use std::{
    collections::HashMap, sync::{Arc, Mutex}
};

use crate::esi::client::KILLMAILS;
use crate::esi::client::PARAM;
use crate::esi::client::UNIVERSE;
use crate::{killmails::killmail, universe};

use super::{ApiClient, EveSwaggerClient};
pub struct EveApi {
    client: EveSwaggerClient,
    cache: Arc<Mutex<HashMap<u32, universe::Station>>>,
}

impl EveApi {
    pub fn new() -> Self {
        Self {
            client: EveSwaggerClient::new(),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn load(&self, id: u32) -> anyhow::Result<universe::Station> {
        // Проверяем, есть ли данные в кэше
        if let Some(station) = self.cache.lock().unwrap().get(&id) {
            return Ok(station.clone());
        }

        // Если данных нет в кэше, загружаем их
        let station = self
            .client
            .get::<universe::Station>(format!("{UNIVERSE}/stations/{id}/?{PARAM}"))
            .await?;

        // Сохраняем данные в кэш
        self.cache.lock().unwrap().insert(id, station.clone());

        Ok(station)
    }

    pub async fn load_killmail<S: Into<String>>(
        &self,
        id: u32,
        hash: S,
    ) -> anyhow::Result<killmail::Killmail> {
        let hash = hash.into();
        let obj = self
            .client
            .get::<killmail::Killmail>(format!("{KILLMAILS}/{id}/{hash}/?{PARAM}"))
            .await?;
        Ok(obj)
    }
}

// #[async_trait]
// impl LoadableById<universe::Station> for CachedClient {
//     type Output = ;

// }
