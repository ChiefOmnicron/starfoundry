use starfoundry_lib_types::CharacterId;
use thiserror::Error;

use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};

use crate::api_docs::{format_json_errors, ErrorResponse};
use crate::industry_hub::IndustryHubUuid;
use crate::structure::StructureError;

pub type Result<T, E = IndustryHubError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum IndustryHubError {
    #[error("the character '{1}' is not allowed to access '{0}'")]
    Forbidden(IndustryHubUuid, CharacterId),
    #[error("industry hub with id '{0}' not found")]
    NotFound(IndustryHubUuid),
    #[error("Validating the input data failed, '{0}'")]
    ValidationError(String),
    #[error("error while fetching permissions for industry hub '{1}', error: '{0}'")]
    FetchPermission(sqlx::Error, IndustryHubUuid),

    #[error("error while creating industry hub, error: '{0}'")]
    CreateIndustryHub(sqlx::Error),
    #[error("error while fetching industry hub '{1}', error: '{0}'")]
    FetchIndustryHub(sqlx::Error, IndustryHubUuid),
    #[error("error while fetching industry hub structures '{1}', error: '{0}'")]
    FetchIndustryHubStructures(sqlx::Error, IndustryHubUuid),
    #[error("error while listing industry hubs, error: '{0}'")]
    ListIndustryHubs(sqlx::Error),
    #[error("error while deleting industry hub '{1}', error: '{0}'")]
    DeleteIndustryHub(sqlx::Error, IndustryHubUuid),
    #[error("error while updating industry hub '{1}', error: '{0}'")]
    UpdateIndustryHub(sqlx::Error, IndustryHubUuid),
    #[error("error while cloning industry hub '{1}', error: '{0}'")]
    CloneIndustryHub(sqlx::Error, IndustryHubUuid),

    #[error("BeginTransactionError, error: '{0}'")]
    BeginTransactionError(sqlx::Error),
    #[error("CommitTransactionError, error: '{0}'")]
    CommitTransactionError(sqlx::Error),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    StructureError(#[from] StructureError),
    #[error(transparent)]
    GatewayLibError(#[from] starfoundry_lib_gateway::error::Error),
    #[error(transparent)]
    EveGatewayLibError(#[from] starfoundry_lib_eve_gateway::Error),
}

impl IntoResponse for IndustryHubError {
    fn into_response(self) -> Response {
        match self {
            Self::Forbidden(_, _) |
            Self::FetchPermission(_, _) => {
                tracing::info!("{}", self.to_string());
                (
                    StatusCode::FORBIDDEN,
                    Json(
                        ErrorResponse {
                            error: "FORBIDDEN".into(),
                            description: "You are not allowed this resource".into(),
                        }
                    )
                ).into_response()
            },

            Self::NotFound(_) => {
                tracing::info!("{}", self.to_string());
                (
                    StatusCode::NOT_FOUND,
                    Json(
                        ErrorResponse {
                            error: "NOT_FOUND".into(),
                            description: self.to_string(),
                        }
                    )
                ).into_response()
            },

            Self::ValidationError(_) => {
                tracing::info!("{}", self.to_string());
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(
                        ErrorResponse {
                            error: "UNPROCESSABLE_ENTITY".into(),
                            description: self.to_string(),
                        }
                    )
                ).into_response()
            },

            Self::JsonExtractorRejection(x) => {
                format_json_errors(x).into_response()
            },

            _ => {
                tracing::error!("{}", self.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        ErrorResponse {
                            error: "UNKNOWN".into(),
                            description: "An unknown error occurred, please try again later.".into(),
                        }
                    )
                ).into_response()
            },
        }
        .into_response()
    }
}
