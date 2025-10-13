use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("{0}")]
    EveGatewayClientError(#[from] crate::api_client::error::EveGatewayClientError),
}
