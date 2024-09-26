use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::future::Future;

pub mod api;
pub use api::EveApi;
pub use api::Uid;
pub use api::Uri;

pub const STATUS: &'static str = "https://esi.evetech.net/latest/status";
pub const MARKETS: &'static str = "https://esi.evetech.net/latest/markets";
pub const KILLMAILS: &'static str = "https://esi.evetech.net/latest/killmails";
pub const UNIVERSE: &'static str = "https://esi.evetech.net/latest/universe";

pub const ALLIANCES: &'static str = "https://esi.evetech.net/latest/alliances";
pub const CORPORATIONS: &'static str = "https://esi.evetech.net/latest/corporations";
pub const CHARACTERS: &'static str = "https://esi.evetech.net/latest/characters";

pub const PARAM: &'static str = "datasource=tranquility&language=en";


pub trait ApiClient {

    fn process<T>(request: RequestBuilder) -> impl Future<Output = anyhow::Result<T>> + Send
    where
        T: Debug + for<'de> Deserialize<'de>;

    fn get<T>(&self, url: String) -> impl Future<Output = anyhow::Result<T>> + Send
    where
        T: Debug + for<'de> Deserialize<'de>;

    fn post<I, O>(&self, url: String, names: I) -> impl Future<Output = anyhow::Result<O>> + Send
    where
        I: Debug + for<'se> Serialize + Send,
        O: Debug + for<'de> Deserialize<'de>;
}
