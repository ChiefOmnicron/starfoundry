use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("More than 25 fields were added to the message")]
    TooManyFields,
    #[error("More than 256 characters for a field name")]
    TooManyFieldCharacters,
    #[error("More than 1024 characters for a field value")]
    TooManyValueCharacters,

    #[error("invalid target expected '{0}'. url: '{1}'")]
    InvalidTarget(String, String),
    #[error("error while sending notification, error: '{0}', body: '{1}'")]
    ResponseError(String, serde_json::Value),

    #[error("error in reqwest, '{0}'")]
    GenericReqwestError(#[from] reqwest::Error),
    #[error("error in parsing struct to json, '{0}'")]
    GenericSerdeParseError(#[from] serde_json::Error),
}
