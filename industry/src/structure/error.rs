use starfoundry_lib_types::{CharacterId, TypeId};
use thiserror::Error;

use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};

use crate::api_docs::{format_json_errors, ErrorResponse};
use crate::structure::StructureUuid;

pub type Result<T, E = StructureError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StructureError {
    #[error("general ItemError, error: '{0}'")]
    ItemError(crate::item::ItemError),

    #[error("the character '{1}' is not allowed to access '{0}'")]
    Forbidden(StructureUuid, CharacterId),
    #[error("structure with id '{0}' not found")]
    NotFound(StructureUuid),
    #[error("Validating the input data failed, '{0}'")]
    ValidationError(String),

    #[error("error while creating structure, error: '{0}'")]
    Create(sqlx::Error),

    #[error("error while fetching structure '{1}', error: '{0}'")]
    FetchStructure(sqlx::Error, StructureUuid),
    #[error("error while listing structures, error: '{0}'")]
    ListStructures(sqlx::Error),

    #[error("error while fetching permissions for structure '{1}', error: '{0}'")]
    FetchStructurePermission(sqlx::Error, StructureUuid),

    #[error("error fetching rigs bonuses for TypeId '{1}', error: {0}")]
    FetchRigBonusByTypeId(sqlx::Error, TypeId),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    EveGatewayLibError(#[from] starfoundry_lib_eve_gateway::Error),
}

impl From<crate::item::ItemError> for StructureError {
    fn from(e: crate::item::ItemError) -> Self {
        Self::ItemError(e)
    }
}

impl IntoResponse for StructureError {
    fn into_response(self) -> Response {
        match self {
            Self::Forbidden(_, _) |
            Self::FetchStructurePermission(_, _) => {
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
