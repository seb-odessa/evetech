use log::{debug, error};
use reqwest::{header, Client, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::future::Future;

use crate::esi::ApiClient;
use crate::esi::Loadable;
use crate::esi::LoadableById;
use crate::esi::Searchable;
use crate::{common, universe};

pub const STATUS: &'static str = "https://esi.evetech.net/latest/status";
pub const UNIVERSE: &'static str = "https://esi.evetech.net/latest/universe";
pub const PARAM: &'static str = "datasource=tranquility&language=en";

pub struct EveSwaggerClient {
    client: reqwest::Client,
}

impl EveSwaggerClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl ApiClient for EveSwaggerClient {
    async fn get<T>(&self, url: String) -> anyhow::Result<T>
    where
        T: Debug + for<'de> Deserialize<'de>,
    {
        debug!("<- {}", url);
        let request = self
            .client
            .get(url)
            .header(header::ACCEPT, "application/json")
            .header(header::CACHE_CONTROL, "no-cache");

        Self::process(request).await
    }

    async fn post<I, O>(&self, url: String, names: I) -> anyhow::Result<O>
    where
        I: Debug + for<'se> Serialize + Send,
        O: Debug + for<'de> Deserialize<'de>,
    {
        debug!("url: {url}");

        let request = self
            .client
            .post(url)
            .json(&names)
            .header(header::ACCEPT, "application/json")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::CACHE_CONTROL, "no-cache");

        Self::process(request).await
    }

    async fn process<T>(request: RequestBuilder) -> anyhow::Result<T>
    where
        T: Debug + for<'de> Deserialize<'de>,
    {
        let response = request.send().await.inspect_err(|e| error!("{:?}", e))?;

        let status = response.status();
        if status == StatusCode::OK {
            let object = response
                .json::<T>()
                .await
                .inspect_err(|e| error!("{:?}", e))?;
            debug!("-> {:?}", object);
            Ok(object)
        } else {
            let error = format!("{}", status);
            error!("{}", error);
            Err(anyhow::anyhow!(error))
        }
    }
}

impl Searchable<common::SearchResult> for EveSwaggerClient {
    type Output = common::SearchResult;
    fn load(
        &self,
        names: Vec<String>,
    ) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.post(format!("{UNIVERSE}/ids/?{PARAM}"), names)
    }
}

impl Loadable<common::Status> for EveSwaggerClient {
    type Output = common::Status;
    fn load(&self) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{STATUS}/?{PARAM}"))
    }
}

impl LoadableById<universe::System> for EveSwaggerClient {
    type Output = universe::System;
    fn load(&self, id: u32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{UNIVERSE}/systems/{id}/?{PARAM}"))
    }
}

impl LoadableById<universe::Constellation> for EveSwaggerClient {
    type Output = universe::Constellation;
    fn load(&self, id: u32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{UNIVERSE}/constellations/{id}/?{PARAM}"))
    }
}

impl LoadableById<universe::Region> for EveSwaggerClient {
    type Output = universe::Region;
    fn load(&self, id: u32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{UNIVERSE}/regions/{id}/?{PARAM}"))
    }
}


impl LoadableById<universe::Star> for EveSwaggerClient {
    type Output = universe::Star;
    fn load(&self, id: u32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{UNIVERSE}/stars/{id}/?{PARAM}"))
    }
}

impl LoadableById<universe::Planet> for EveSwaggerClient {
    type Output = universe::Planet;
    fn load(&self, id: u32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{UNIVERSE}/planets/{id}/?{PARAM}"))
    }
}

impl LoadableById<universe::AsteroidBelts> for EveSwaggerClient {
    type Output = universe::AsteroidBelts;
    fn load(&self, id: u32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{UNIVERSE}/asteroid_belts/{id}/?{PARAM}"))
    }
}

impl LoadableById<universe::Moon> for EveSwaggerClient {
    type Output = universe::Moon;
    fn load(&self, id: u32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{UNIVERSE}/moons/{id}/?{PARAM}"))
    }
}

impl LoadableById<universe::Stargate> for EveSwaggerClient {
    type Output = universe::Stargate;
    fn load(&self, id: u32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{UNIVERSE}/stargates/{id}/?{PARAM}"))
    }
}

impl LoadableById<universe::Station> for EveSwaggerClient {
    type Output = universe::Station;
    fn load(&self, id: u32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{UNIVERSE}/stations/{id}/?{PARAM}"))
    }
}

impl LoadableById<universe::Type> for EveSwaggerClient {
    type Output = universe::Type;
    fn load(&self, id: u32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{UNIVERSE}/types/{id}/?{PARAM}"))
    }
}

impl LoadableById<universe::Group> for EveSwaggerClient {
    type Output = universe::Group;
    fn load(&self, id: u32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{UNIVERSE}/groups/{id}/?{PARAM}"))
    }
}

impl LoadableById<universe::Category> for EveSwaggerClient {
    type Output = universe::Category;
    fn load(&self, id: u32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{UNIVERSE}/categories/{id}/?{PARAM}"))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn status() -> anyhow::Result<()> {
        type Object = crate::common::Status;
        let esc = EveSwaggerClient::new();
        let result = <EveSwaggerClient as Loadable<Object>>::load(&esc).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn search() -> anyhow::Result<()> {
        use crate::common::Object;
        use crate::common::SearchResult;
        let esc = EveSwaggerClient::new();
        let names = vec![String::from("Arifsdald"), String::from("Seb Odessa")];
        let obj = <EveSwaggerClient as Searchable<SearchResult>>::load(&esc, names).await?;
        assert_eq!(obj.systems, Some(vec![Object::new(30002080, "Arifsdald")]));
        assert_eq!(
            obj.characters,
            Some(vec![Object::new(2114350216, "Seb Odessa")])
        );
        Ok(())
    }

    #[tokio::test]
    async fn system() -> anyhow::Result<()> {
        type Object = crate::universe::System;
        let esc = EveSwaggerClient::new();
        let obj = <EveSwaggerClient as LoadableById<Object>>::load(&esc, 30002080).await?;
        assert_eq!(obj.system_id, 30002080);
        assert_eq!(obj.name, "Arifsdald");
        Ok(())
    }

    #[tokio::test]
    async fn constellation() -> anyhow::Result<()> {
        type Object = crate::universe::Constellation;
        let esc = EveSwaggerClient::new();
        let obj = <EveSwaggerClient as LoadableById<Object>>::load(&esc, 20000306).await?;
        assert_eq!(obj.constellation_id, 20000306);
        assert_eq!(obj.name, "Fribrodi");
        Ok(())
    }

    #[tokio::test]
    async fn region() -> anyhow::Result<()> {
        type Object = crate::universe::Region;
        let esc = EveSwaggerClient::new();
        let obj = <EveSwaggerClient as LoadableById<Object>>::load(&esc, 10000042).await?;
        assert_eq!(obj.region_id, 10000042);
        assert_eq!(obj.name, "Metropolis");
        Ok(())
    }

    #[tokio::test]
    async fn star() -> anyhow::Result<()> {
        type Object = crate::universe::Star;
        let esc = EveSwaggerClient::new();
        let obj = <EveSwaggerClient as LoadableById<Object>>::load(&esc, 40132801).await?;
        assert_eq!(obj.solar_system_id, 30002080);
        assert_eq!(obj.name, "Arifsdald - Star");
        Ok(())
    }

    #[tokio::test]
    async fn planet() -> anyhow::Result<()> {
        type Object = crate::universe::Planet;
        let esc = EveSwaggerClient::new();
        let obj = <EveSwaggerClient as LoadableById<Object>>::load(&esc, 40132802).await?;
        assert_eq!(obj.planet_id, 40132802);
        assert_eq!(obj.system_id, 30002080);
        assert_eq!(obj.name, "Arifsdald I");
        Ok(())
    }

    #[tokio::test]
    async fn asteroid_belt() -> anyhow::Result<()> {
        type Object = crate::universe::AsteroidBelts;
        let esc = EveSwaggerClient::new();
        let obj = <EveSwaggerClient as LoadableById<Object>>::load(&esc, 40132822).await?;
        assert_eq!(obj.system_id, 30002080);
        assert_eq!(obj.name, "Arifsdald II - Asteroid Belt 1");
        Ok(())
    }

    #[tokio::test]
    async fn moon() -> anyhow::Result<()> {
        type Object = crate::universe::Moon;
        let esc = EveSwaggerClient::new();
        let obj = <EveSwaggerClient as LoadableById<Object>>::load(&esc, 40132803).await?;
        assert_eq!(obj.system_id, 30002080);
        assert_eq!(obj.name, "Arifsdald I - Moon 1");
        Ok(())
    }

    #[tokio::test]
    async fn stargate() -> anyhow::Result<()> {
        type Object = crate::universe::Stargate;
        let esc = EveSwaggerClient::new();
        let obj = <EveSwaggerClient as LoadableById<Object>>::load(&esc, 50011094).await?;
        assert_eq!(obj.stargate_id, 50011094);
        assert_eq!(obj.system_id, 30002080);
        assert_eq!(obj.name, "Stargate (Dudreda)");
        Ok(())
    }

    #[tokio::test]
    async fn station() -> anyhow::Result<()> {
        type Object = crate::universe::Station;
        let esc = EveSwaggerClient::new();
        let obj = <EveSwaggerClient as LoadableById<Object>>::load(&esc, 60004528).await?;
        assert_eq!(obj.station_id, 60004528);
        assert_eq!(obj.system_id, 30002080);
        assert_eq!(obj.name, "Arifsdald III - Moon 10 - Krusual Tribe Bureau");
        Ok(())
    }

    #[tokio::test]
    async fn types() -> anyhow::Result<()> {
        type Object = crate::universe::Type;
        let esc = EveSwaggerClient::new();
        let obj = <EveSwaggerClient as LoadableById<Object>>::load(&esc, 2502).await?;
        assert_eq!(obj.type_id, 2502);
        assert_eq!(obj.name, "Minmatar Trade Post");
        assert_eq!(obj.group_id, 15);
        assert_eq!(obj.graphic_id, Some(1138));
        Ok(())
    }

    #[tokio::test]
    async fn group() -> anyhow::Result<()> {
        type Object = crate::universe::Group;
        let esc = EveSwaggerClient::new();
        let obj = <EveSwaggerClient as LoadableById<Object>>::load(&esc, 15).await?;
        assert_eq!(obj.group_id, 15);
        assert_eq!(obj.name, "Station");
        assert_eq!(obj.category_id, 3);
        Ok(())
    }

    #[tokio::test]
    async fn category() -> anyhow::Result<()> {
        type Object = crate::universe::Category;
        let esc = EveSwaggerClient::new();
        let obj = <EveSwaggerClient as LoadableById<Object>>::load(&esc, 3).await?;
        assert_eq!(obj.category_id, 3);
        assert_eq!(obj.name, "Station");
        Ok(())
    }
}
