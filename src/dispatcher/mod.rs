mod weighted_rr;

use std::net::SocketAddr;

use eyre::Result;

pub use weighted_rr::{WeightedAddress, WeightedRoundRobinDispatcher};

#[async_trait::async_trait]
pub trait Dispatch {
    async fn dispatch(&self, remote_address: &SocketAddr) -> Result<WeightedAddress>;
}
