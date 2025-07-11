use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use starfoundry_libs_types::CharacterId;
use thiserror::Error;

use crate::api_docs::ErrorResponse;
use crate::utils::JsonWrapper;
use crate::project_group::ProjectGroupUuid;

pub type Result<T, E = ProjectGroupError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ProjectGroupError {
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
    #[error("error while updating group defaults '{1}', error: '{0}'")]
    UpdateGroupDefaults(sqlx::Error, ProjectGroupUuid),

    #[error("error while fetching permissions for project group '{1}', error: '{0}'")]
    FetchGroupPermissions(sqlx::Error, ProjectGroupUuid),

    #[error("error while accepting invite to project group '{1}', error: '{0}'")]
    AcceptGroupInvite(sqlx::Error, ProjectGroupUuid),
    #[error("error while accepting member to project group '{1}', error: '{0}'")]
    AcceptGroupMember(sqlx::Error, ProjectGroupUuid),
    #[error("error while fetching project group members for group '{1}', error: '{0}'")]
    ListGroupMembers(sqlx::Error, ProjectGroupUuid),
    #[error("error while updating group member '{1}', error: '{0}'")]
    UpdateGroupMember(sqlx::Error, ProjectGroupUuid, CharacterId),
    #[error("error while removing group member '{1}', error: '{0}'")]
    RemoveGroupMember(sqlx::Error, ProjectGroupUuid, CharacterId),

    #[error("error while beginning transaction, error: '{0}'")]
    TransactionBeginError(sqlx::Error),
    #[error("error while committing transaction, error: '{0}'")]
    TransactionCommitError(sqlx::Error),
}

impl IntoResponse for ProjectGroupError {
    fn into_response(self) -> Response {
        match self {
            Self::Forbidden(_, _) => {
                tracing::info!("{}", self.to_string());
                (
                    StatusCode::FORBIDDEN,
                    JsonWrapper(
                        ErrorResponse {
                            error: "FORBIDDEN".into(),
                            description: "You are not allowed this resource".into(),
                        }
                    )
                )
            },

            Self::NotFound(_) => {
                tracing::info!("{}", self.to_string());
                (
                    StatusCode::NOT_FOUND,
                    JsonWrapper(
                        ErrorResponse {
                            error: "NOT_FOUND".into(),
                            description: self.to_string(),
                        }
                    )
                )
            },

            Self::ValidationError(_) => {
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::BAD_REQUEST,
                    JsonWrapper(
                        ErrorResponse {
                            error: "INVALID_RESPONSE".into(),
                            description: self.to_string(),
                        }
                    )
                )
            },

            _ => {
                tracing::error!("{}", self.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    JsonWrapper(
                        ErrorResponse {
                            error: "UNKNOWN".into(),
                            description: "An unknown error occurred, please try again later.".into(),
                        }
                    )
                )
            },
        }
        .into_response()
    }
}
