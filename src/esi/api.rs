use log::{debug, error};
use reqwest::{header, Client, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{common, esi::ApiClient};

use std::fmt::Debug;

pub enum Uid {
    Empty,
    Id(i32),
    Killmail(i32, String),
}

pub trait Uri: Clone + Send + Sync + 'static {
    fn uri(id: &Uid) -> anyhow::Result<String>;
}

pub struct EveApi {
    client: reqwest::Client,
}
impl EveApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn load<T>(&self, id: &Uid) -> anyhow::Result<T>
    where
        T: Uri + Debug + for<'de> Deserialize<'de>,
    {
        let uri = T::uri(id)?;
        let object = self.get::<T>(uri).await?;
        Ok(object)
    }

    pub async fn search(&self, names: &Vec<String>) -> anyhow::Result<common::SearchResult> {
        let uri = common::SearchResult::uri(&Uid::Empty)?;
        let object = self.post(uri, names).await?;
        Ok(object)
    }
}

impl ApiClient for EveApi {
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
