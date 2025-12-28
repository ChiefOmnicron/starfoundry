use starfoundry_lib_types::CharacterId;
use thiserror::Error;

use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};

use crate::api_docs::{format_json_errors, ErrorResponse};
use crate::structure_group::StructureGroupUuid;
use crate::structure::StructureError;

pub type Result<T, E = StructureGroupError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StructureGroupError {
    #[error("the character '{1}' is not allowed to access '{0}'")]
    Forbidden(StructureGroupUuid, CharacterId),
    #[error("structure group with id '{0}' not found")]
    NotFound(StructureGroupUuid),
    #[error("Validating the input data failed, '{0}'")]
    ValidationError(String),
    #[error("error while fetching permissions for structure '{1}', error: '{0}'")]
    FetchPermission(sqlx::Error, StructureGroupUuid),

    #[error("error while creating structure, error: '{0}'")]
    CreateStructureGroup(sqlx::Error),
    #[error("error while fetching structure '{1}', error: '{0}'")]
    FetchStructureGroup(sqlx::Error, StructureGroupUuid),
    #[error("error while fetching group structures '{1}', error: '{0}'")]
    FetchGroupStructures(sqlx::Error, StructureGroupUuid),
    #[error("error while listing structures, error: '{0}'")]
    ListStructureGroups(sqlx::Error),
    #[error("error while deleting structure '{1}', error: '{0}'")]
    DeleteStructureGroup(sqlx::Error, StructureGroupUuid),
    #[error("error while updating structure '{1}', error: '{0}'")]
    UpdateStructureGroup(sqlx::Error, StructureGroupUuid),

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

impl IntoResponse for StructureGroupError {
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
