use crate::{common, universe};

use log::{debug, error};
use reqwest::{header, Client, RequestBuilder, StatusCode};
use serde::Deserialize;
use std::fmt::Debug;

pub const ROOT: &'static str = "https://esi.evetech.net/latest";
pub const PARAM: &'static str = "datasource=tranquility&language=en";

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

async fn load<T>(client: &Client, url: &String) -> anyhow::Result<T>
where
    T: Debug + for<'de> Deserialize<'de>,
{
    debug!("<- {}", url);

    let request = client
        .get(url)
        .header(header::ACCEPT, "application/json")
        .header(header::CACHE_CONTROL, "no-cache");

    process(request).await
}

pub async fn search(
    client: &Client,
    url: &String,
    names: &Vec<String>,
) -> anyhow::Result<common::SearchResult> {
    debug!("url: {url}");

    let request = client
        .post(url)
        .json(names)
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CACHE_CONTROL, "no-cache");

    process(request).await
}

pub struct Esi {
    client: Client,
}
impl Esi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn description(&self) -> String {
        String::from("EVE Swagger Interface (ESI)")
    }

    // Retrieve the uptime and player counts
    pub async fn status(&self) -> anyhow::Result<common::Status> {
        load(&self.client, &format!("{ROOT}/status/?{PARAM}")).await
    }

    // Get ids by names
    pub async fn search(&self, names: &Vec<String>) -> anyhow::Result<common::SearchResult> {
        search(
            &self.client,
            &format!("{ROOT}/universe/ids/?{PARAM}"),
            names,
        )
        .await
    }

    // Get Asteroid belt information
    pub async fn asteroid_belt(&self, id: u32) -> anyhow::Result<universe::AsteroidBelts> {
        load(
            &self.client,
            &format!("{ROOT}/universe/asteroid_belts/{id}/?{PARAM}"),
        )
        .await
    }

    // Get Solar System information
    pub async fn system(&self, id: u32) -> anyhow::Result<universe::System> {
        load(
            &self.client,
            &format!("{ROOT}/universe/systems/{id}/?{PARAM}"),
        )
        .await
    }

    // Get Star information
    pub async fn star(&self, id: u32) -> anyhow::Result<universe::Star> {
        load(
            &self.client,
            &format!("{ROOT}/universe/stars/{id}/?{PARAM}"),
        )
        .await
    }

    // Get Planet information
    pub async fn planet(&self, id: u32) -> anyhow::Result<universe::Planet> {
        load(
            &self.client,
            &format!("{ROOT}/universe/planets/{id}/?{PARAM}"),
        )
        .await
    }

    // Get Moon information
    pub async fn moon(&self, id: u32) -> anyhow::Result<universe::Moon> {
        load(
            &self.client,
            &format!("{ROOT}/universe/moons/{id}/?{PARAM}"),
        )
        .await
    }

    // Get Stargate information
    pub async fn stargate(&self, id: u32) -> anyhow::Result<universe::Stargate> {
        load(
            &self.client,
            &format!("{ROOT}/universe/stargates/{id}/?{PARAM}"),
        )
        .await
    }

    // Get Station information
    pub async fn station(&self, id: u32) -> anyhow::Result<universe::Station> {
        load(
            &self.client,
            &format!("{ROOT}/universe/stations/{id}/?{PARAM}"),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn status() {
        let status = Esi::new().status().await;
        assert!(status.is_ok());
    }
}