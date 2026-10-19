pub mod api_docs;
pub mod config;
pub mod healthcheck;
pub mod metrics;

pub mod filter;
pub mod industry;
pub mod industry_hub;
pub mod job_assignments;
pub mod project;
pub mod project_group;
pub mod state;
pub mod structure;
pub mod tag;

mod client;
mod sort;
mod utils;

mod error;
pub use self::error::{IndustryError, Result};
pub use self::state::*;

pub const SERVICE_NAME: &str = "SF_INDUSTRY";
pub const ENV_INDUSTRY_API: &str = "STARFOUNDRY_INDUSTRY_API_URL";

starfoundry_uuid!(FilterUuid, "FilterUuid");
starfoundry_uuid!(IndustryHubUuid, "IndustryHubUuid");
starfoundry_uuid!(MarketUuid, "MarketUuid");
starfoundry_uuid!(ProjectGroupUuid, "ProjectGroupUuid");
starfoundry_uuid!(ProjectJobUuid, "ProjectJobUuid");
starfoundry_uuid!(ProjectUuid, "ProjectUuid");
starfoundry_uuid!(SolutionUuid, "SolutionUuid");
starfoundry_uuid!(StructureUuid, "StructureUuid");
starfoundry_uuid!(TagUuid, "TagUuid");

#[cfg(not(test))]
use starfoundry_eve_gateway::EveGatewayClient;
#[cfg(not(test))]
pub fn eve_gateway_api_client()
-> Result<EveGatewayClient, starfoundry_eve_gateway::EveGatewayError> {
    EveGatewayClient::new(SERVICE_NAME)
}
#[cfg(not(test))]
use starfoundry_market::MarketClient;
#[cfg(not(test))]
pub fn market_api_client()
-> Result<MarketClient, starfoundry_market::MarketError> {
    MarketClient::new(SERVICE_NAME)
}

#[cfg(test)]
mod test_util;
#[cfg(test)]
use crate::test_util::EveGatewayTestApiClient;
#[cfg(test)]
pub fn eve_gateway_api_client()
-> Result<EveGatewayTestApiClient, starfoundry_eve_gateway::EveGatewayError> {
    Ok(EveGatewayTestApiClient {})
}
use starfoundry_lib_types::starfoundry_uuid;

#[cfg(test)]
use crate::test_util::MarketTestApiClient;
#[cfg(test)]
pub fn market_api_client()
-> Result<MarketTestApiClient, starfoundry_market::MarketError> {
    Ok(MarketTestApiClient {})
}
