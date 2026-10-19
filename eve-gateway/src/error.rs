use starfoundry_gateway::boxed_from;
use thiserror::Error;

pub type Result<T, E = EveGatewayError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum EveGatewayError {
    #[error("{0:?}")]
    GatewayClientError(Box<starfoundry_gateway::GatewayError>),

    #[error("the env {0} is not set")]
    EnvNotSet(&'static str),
    #[error(
        "error while parsing url. Validate the environment variables, error: '{0}'"
    )]
    UrlParseError(url::ParseError),
    #[error("error while parsing from reqwest, error: '{0:?}'")]
    ReqwestError(reqwest::Error),

    #[error(
        "the given category '{0}' is not valid, it must be one of: 'agent', 'alliance', 'character', 'constellation', 'corporation', 'faction', 'inventory_type', 'region', 'solar_system', 'station', 'structure'"
    )]
    InvalidSearchCategory(String),
}

boxed_from!(
    EveGatewayError::GatewayClientError,
    starfoundry_gateway::GatewayError
);
