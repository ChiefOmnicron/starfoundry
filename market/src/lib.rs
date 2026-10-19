pub mod api_docs;
pub mod config;
pub mod healthcheck;
pub mod market;
pub mod metrics;
pub mod price;
pub mod state;
pub mod structure;

mod client;
pub use self::client::{MarketApiClient, MarketClient};

mod lp;

mod error;
pub use self::error::{MarketError, Result};
pub use self::state::*;

pub const SERVICE_NAME: &str = "SF_MARKET";
pub const ENV_MARKET_API: &str = "STARFOUNDRY_MARKET_API_URL";

#[cfg(not(test))]
use starfoundry_eve_gateway::EveGatewayClient;
#[cfg(not(test))]
pub fn eve_gateway_api_client()
-> Result<EveGatewayClient, starfoundry_eve_gateway::EveGatewayError> {
    EveGatewayClient::new(SERVICE_NAME)
}

#[cfg(test)]
use starfoundry_eve_gateway::EveGatewayClient;
#[cfg(test)]
pub fn eve_gateway_api_client()
-> Result<EveGatewayClient, starfoundry_eve_gateway::EveGatewayError> {
    EveGatewayClient::new("MARKET_TESTING")
}
