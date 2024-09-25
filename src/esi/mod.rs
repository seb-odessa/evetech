use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::future::Future;

pub mod api;
pub use api::EveApi;
pub use api::Uid;

pub const STATUS: &'static str = "https://esi.evetech.net/latest/status";
pub const MARKETS: &'static str = "https://esi.evetech.net/latest/markets";
pub const KILLMAILS: &'static str = "https://esi.evetech.net/latest/killmails";
pub const UNIVERSE: &'static str = "https://esi.evetech.net/latest/universe";
pub const PARAM: &'static str = "datasource=tranquility&language=en";


pub trait ApiClient {
    // Process prepared request
    fn process<T>(request: RequestBuilder) -> impl Future<Output = anyhow::Result<T>> + Send
    where
        T: Debug + for<'de> Deserialize<'de>;

    // Prepare and handle GET request
    fn get<T>(&self, url: String) -> impl Future<Output = anyhow::Result<T>> + Send
    where
        T: Debug + for<'de> Deserialize<'de>;

    // Prepare and handle POST request
    fn post<I, O>(&self, url: String, names: I) -> impl Future<Output = anyhow::Result<O>> + Send
    where
        I: Debug + for<'se> Serialize + Send,
        O: Debug + for<'de> Deserialize<'de>;
}
