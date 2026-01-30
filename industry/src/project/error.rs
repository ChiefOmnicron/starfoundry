use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_types::CharacterId;
use thiserror::Error;

use crate::api_docs::{format_json_errors, ErrorResponse};
use crate::project::ProjectUuid;
use crate::project_group::ProjectGroupError;

pub type Result<T, E = ProjectError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ProjectError {
    #[error("the character '{1}' is not allowed to access '{0}'")]
    Forbidden(ProjectUuid, CharacterId),
    #[error("project group with id '{0}' not found")]
    NotFound(ProjectUuid),
    #[error("Validating the input data failed, '{0}'")]
    ValidationError(String),

    #[error("error while listing projects, error: '{0}'")]
    List(sqlx::Error),
    #[error("error while listing project jobs, error: '{0}'")]
    ListJobs(sqlx::Error),
    #[error("error while listing project misc, error: '{0}'")]
    ListMisc(sqlx::Error),

    #[error("error while fetching project '{1}', error: '{0}'")]
    FetchProject(sqlx::Error, ProjectUuid),

    #[error("error while creating project, error: '{0}'")]
    CreateProject(sqlx::Error),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    ProjectGroupError(#[from] ProjectGroupError),
    #[error(transparent)]
    EveGatewayLibError(#[from] starfoundry_lib_eve_gateway::Error),
    #[error(transparent)]
    StructureError(#[from] crate::structure::StructureError),
}

impl IntoResponse for ProjectError {
    fn into_response(self) -> Response {
        match self {
            Self::Forbidden(_, _) => {
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
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::BAD_REQUEST,
                    Json(
                        ErrorResponse {
                            error: "INVALID_RESPONSE".into(),
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
