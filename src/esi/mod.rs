use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::future::Future;

pub mod client;
pub use client::EveSwaggerClient;
pub mod api;
pub use api::EveApi;


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

pub trait Searchable<T> {
    type Output;
    fn load<I>(&self, names: Vec<I>) -> impl Future<Output = anyhow::Result<Self::Output>> + Send
    where
        I: Debug + for<'se> Serialize + Send;
}

pub trait Loadable<T> {
    type Output;
    fn load(&self) -> impl Future<Output = anyhow::Result<Self::Output>> + Send;
}

pub trait LoadableById<T> {
    type Output;
    fn load(&self, id: i32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send;
}

pub trait LoadableByIdAndHash<T> {
    type Output;
    fn load<S: Into<String>>(&self, id: i32, hash: S) -> impl Future<Output = anyhow::Result<Self::Output>> + Send;
}