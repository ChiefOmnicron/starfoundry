use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_gateway::ErrorResponse;
use thiserror::Error;

pub type Result<T, E = JobAssignmentError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JobAssignmentError {
    #[error("error while listing jobs, error: '{0}'")]
    List(sqlx::Error),

    #[error("error while creating job assignment, error: '{0}'")]
    Create(sqlx::Error),

    #[error("error while updating job assignment, error: '{0}'")]
    Update(sqlx::Error),

    #[error(transparent)]
    EveGatewayLibError(#[from] starfoundry_lib_eve_gateway::Error),
}

impl IntoResponse for JobAssignmentError {
    fn into_response(self) -> Response {
        match self {
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
