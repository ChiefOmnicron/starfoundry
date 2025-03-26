use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("error while fetching notifications for blueprint stock '{1}', error '{0}'")]
    FetchStockBlueprintNotifications(sqlx::Error, Uuid),

    #[error("error from api, '{0}'")]
    ConnectError(starfoundry_libs_eve_api::Error),
    #[error("invalid target expected '{0}'. url: '{1}'")]
    InvalidTarget(String, String),
    #[error("error while sending notification, error: '{0}', body: '{1}'")]
    ResponseError(String, serde_json::Value),

    #[error("error in reqwest, '{0}'")]
    GenericReqwestError(reqwest::Error),
    #[error("error in parsing struct to json, '{0}'")]
    GenericSerdeParseError(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::GenericReqwestError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::GenericSerdeParseError(value)
    }
}
