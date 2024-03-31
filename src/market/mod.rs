use crate::esi::client::{MARKETS, PARAM};
use crate::esi::{ApiClient, EveSwaggerClient, LoadableById};
use std::future::Future;

pub mod group;
pub mod orders;

pub use group::Group;
pub use orders::Orders;

impl LoadableById<Group> for EveSwaggerClient {
    type Output = Group;
    fn load(&self, id: u32) -> impl Future<Output = anyhow::Result<Self::Output>> + Send {
        self.get::<Self::Output>(format!("{MARKETS}/groups/{id}/?{PARAM}"))
    }
}
