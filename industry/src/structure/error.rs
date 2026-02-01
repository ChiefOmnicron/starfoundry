use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_industry::StructureUuid;
use starfoundry_lib_types::CharacterId;
use thiserror::Error;

use crate::api_docs::{format_json_errors, ErrorResponse};

pub type Result<T, E = StructureError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StructureError {
    #[error("error creating transaction, '{0}'")]
    BeginTransaction(sqlx::Error),
    #[error("error committing transaction, '{0}'")]
    CommitTransaction(sqlx::Error),

    #[error("the character '{1}' is not allowed to access '{0}'")]
    Forbidden(StructureUuid, CharacterId),
    #[error("structure with id '{0}' not found")]
    NotFound(StructureUuid),
    #[error("Validating the input data failed, '{0}'")]
    ValidationError(String),
    #[error("error while fetching permissions for structure '{1}', error: '{0}'")]
    FetchPermission(sqlx::Error, StructureUuid),

    #[error("error while creating structure, error: '{0}'")]
    CreateStructure(sqlx::Error),
    #[error("error while fetching structure '{1:?}', error: '{0}'")]
    FetchStructures(sqlx::Error, Vec<StructureUuid>),
    #[error("error while fetching structure tax '{1}', error: '{0}'")]
    FetchStructureTax(sqlx::Error, StructureUuid),
    #[error("error while listing structures, error: '{0}'")]
    ListStructures(sqlx::Error),
    #[error("error while deleting structure '{1}', error: '{0}'")]
    DeleteStructure(sqlx::Error, StructureUuid),
    #[error("error while updating structure '{1}', error: '{0}'")]
    UpdateStructure(sqlx::Error, StructureUuid),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    GatewayLibError(#[from] starfoundry_lib_gateway::error::Error),
    #[error(transparent)]
    EveGatewayLibError(#[from] starfoundry_lib_eve_gateway::Error),
}

impl IntoResponse for StructureError {
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
