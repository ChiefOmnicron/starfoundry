use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_gateway::ErrorResponse;
use starfoundry_lib_types::CharacterId;
use thiserror::Error;

use crate::api_docs::format_json_errors;
use crate::industry_hub::IndustryHubError;
use crate::project_group::ProjectGroupError;
use crate::project::ProjectUuid;
use crate::project::service::ProjectJobUuid;

pub type Result<T, E = ProjectError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ProjectError {
    #[error("the character '{1}' is not allowed to access '{0}'")]
    Forbidden(ProjectUuid, CharacterId),
    #[error("project with id '{0}' not found")]
    NotFound(ProjectUuid),
    #[error("project job with id '{1}' not found in project '{0}'")]
    JobNotFound(ProjectUuid, ProjectJobUuid),
    #[error("Validating the input data failed, '{0}'")]
    ValidationError(String),
    #[error("No industry hub is set, but the operation requires it")]
    NoIndustryHub,
    #[error("The solution could not be found, but the operation requires it")]
    SolutionNotFound,

    #[error("error while adding market entries, error: '{0}'")]
    AddExcessEntry(sqlx::Error),
    #[error("error while adding market entries, error: '{0}'")]
    AddJobEntry(sqlx::Error),
    #[error("error while adding market entries, error: '{0}'")]
    AddMarketEntry(sqlx::Error),

    #[error("error while listing projects, error: '{0}'")]
    List(sqlx::Error),
    #[error("error while listing project excess, error: '{0}'")]
    ListExcess(sqlx::Error),
    #[error("error while listing project jobs, error: '{0}'")]
    ListJobs(sqlx::Error),
    #[error("error while listing project misc, error: '{0}'")]
    ListMisc(sqlx::Error),

    #[error("error while fetching project '{1}', error: '{0}'")]
    Fetch(sqlx::Error, ProjectUuid),

    #[error("error while creating project, error: '{0}'")]
    Create(sqlx::Error),
    #[error("error while initializing project, error: '{0}'")]
    Initialize(sqlx::Error),

    #[error("error while updating project, error: '{0}'")]
    Update(sqlx::Error),

    #[error("transaction error, '{0}'")]
    TransactionError(sqlx::Error),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    ProjectGroupError(#[from] ProjectGroupError),
    #[error(transparent)]
    IndustryHubError(#[from] IndustryHubError),
    #[error(transparent)]
    EveGatewayLibError(#[from] starfoundry_lib_eve_gateway::Error),
    #[error(transparent)]
    MarketLibError(#[from] starfoundry_lib_market::Error),
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
