use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("{0}")]
    GatewayClientError(#[from] starfoundry_lib_gateway::error::Error),

    #[error("the env {0} is not set")]
    EnvNotSet(&'static str),
    #[error("error while parsing url. Validate the environment variables, error: '{0}'")]
    UrlParseError(url::ParseError),
}
