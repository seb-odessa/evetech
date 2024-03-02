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
) -> anyhow::Result<universe::SearchResult> {
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
    pub async fn search(&self, names: &Vec<String>) -> anyhow::Result<universe::SearchResult> {
        search(&self.client, &format!("{ROOT}/universe/ids/?{PARAM}"), names).await
    }

    // Get Asteroid belt information
    pub async fn asteroid_belts(&self, id: u32) -> anyhow::Result<universe::AsteroidBelts> {
        load(
            &self.client,
            &format!("{ROOT}/universe/asteroid_belts/{id}/?{PARAM}"),
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
