use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_types::CharacterId;
use thiserror::Error;

use crate::api_docs::{format_json_errors, ErrorResponse};
use crate::project_group::ProjectGroupUuid;

pub type Result<T, E = ProjectGroupError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ProjectGroupError {
    #[error("general StructureError, error: '{0}'")]
    StructureError(crate::structure::StructureError),

    #[error("the character '{1}' is not allowed to access '{0}'")]
    Forbidden(ProjectGroupUuid, CharacterId),
    #[error("project group with id '{0}' not found")]
    NotFound(ProjectGroupUuid),
    #[error("Validating the input data failed, '{0}'")]
    ValidationError(String),

    #[error("error while creating project group, error: '{0}'")]
    CreateGroup(sqlx::Error),
    #[error("error while fetching all project groups, error: '{0}'")]
    ListGroups(sqlx::Error),
    #[error("error while fetching project '{1}', error: '{0}'")]
    FetchGroup(sqlx::Error, ProjectGroupUuid),
    #[error("error while deleting project group '{1}', error: '{0}'")]
    DeleteGroup(sqlx::Error, ProjectGroupUuid),
    #[error("error while updating project group '{1}', error: '{0}'")]
    UpdateGroup(sqlx::Error, ProjectGroupUuid),

    #[error("error while fetching defaults for project group '{1}', error: '{0}'")]
    FetchGroupDefaults(sqlx::Error, ProjectGroupUuid),

    #[error("error while fetching permissions for project group '{1}', error: '{0}'")]
    FetchGroupPermissions(sqlx::Error, ProjectGroupUuid),

    #[error("error while accepting member to project group '{1}', error: '{0}'")]
    AcceptGroupMember(sqlx::Error, ProjectGroupUuid),
    #[error("error while fetching project group member self for group '{1}', error: '{0}'")]
    FetchGroupMembersSelf(sqlx::Error, ProjectGroupUuid),
    #[error("error while fetching project group members for group '{1}', error: '{0}'")]
    ListGroupMembers(sqlx::Error, ProjectGroupUuid),

    #[error("error while beginning transaction, error: '{0}'")]
    TransactionBeginError(sqlx::Error),
    #[error("error while committing transaction, error: '{0}'")]
    TransactionCommitError(sqlx::Error),

    #[error("a project is assigned to the project group")]
    ProjectIsAssignedToGroup,

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    EveGatewayLibError(#[from] starfoundry_lib_eve_gateway::Error),
}

impl From<crate::structure::StructureError> for ProjectGroupError {
    fn from(e: crate::structure::StructureError) -> Self {
        Self::StructureError(e)
    }
}

impl IntoResponse for ProjectGroupError {
    fn into_response(self) -> Response {
        match self {
            Self::Forbidden(_, _) |
            Self::FetchGroupPermissions(_, _) => {
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

            Self::ValidationError(_) |
            Self::ProjectIsAssignedToGroup => {
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
