use log::{debug, error};
use reqwest::{header, Client, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::esi::ApiClient;
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

    pub async fn status(&self) -> anyhow::Result<common::Status> {
        self.get(&format!("{STATUS}/?{PARAM}")).await
    }

    // Get ids by names
    pub async fn search(&self, names: Vec<String>) -> anyhow::Result<common::SearchResult> {
        self.post(&format!("{UNIVERSE}/ids/?{PARAM}"), names)
            .await
    }

    // Get Asteroid belt information
    pub async fn asteroid_belt(&self, id: u32) -> anyhow::Result<universe::AsteroidBelts> {
        self.get(&format!("{UNIVERSE}/asteroid_belts/{id}/?{PARAM}"))
            .await
    }
}

impl ApiClient for EveSwaggerClient {
    async fn get<T>(&self, url: &String) -> anyhow::Result<T>
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

    async fn post<I, O>(&self, url: &String, names: I) -> anyhow::Result<O>
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


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn status() {
        let status = EveSwaggerClient::new().status().await;
        assert!(status.is_ok());
    }
}