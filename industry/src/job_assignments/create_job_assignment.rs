use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::job_assignments::JobAssignmentUuid;
use crate::job_assignments::error::{JobAssignmentError, Result};
use crate::{AppState, ProjectJobUuid, ProjectUuid};

/// Create Job Assignment
///
/// - Alternative route: `/latest/job-assignments`
/// - Alternative route: `/v1/job-assignments`
///
/// ---
///
/// Creates a new job assignment
#[utoipa::path(
    post,
    path = "/",
    tag = "job_assignment",
    request_body = Vec<CreateProjectJobAssignmentResponse>,
    responses(
        (
            body = CreateProjectJobAssignmentResponse,
            description = "Creates a new job assignment",
            status = OK,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Json(jobs): Json<Vec<CreateProjectJobAssignment>>,
) -> Result<impl IntoResponse> {
    let id = create_job_assignment(&state.postgres, jobs).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateProjectJobAssignmentResponse {
            id,
        }),
    ))
}

pub async fn create_job_assignment(
    pool: &PgPool,
    project_info: Vec<CreateProjectJobAssignment>,
) -> Result<JobAssignmentUuid> {
    let id = Uuid::now_v7();
    let mut project_ids = Vec::new();
    let mut job_ids = Vec::new();

    project_info
        .iter()
        .for_each(|x| {
            project_ids.push(*x.project_id);
            job_ids.push(*x.job_id);
        });

    sqlx::query!(
        r#"
            INSERT INTO project_job_assignment
            (
                id,
                project_id,
                job_id
            )
            SELECT $1, * FROM UNNEST(
                $2::UUID[],
                $3::UUID[]
            )
        "#,
        id,
        &project_ids,
        &job_ids,
    )
    .execute(pool)
    .await
    .map_err(JobAssignmentError::Create)?;

    Ok(id.into())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectJobAssignment {
    project_id: ProjectUuid,
    job_id:     ProjectJobUuid,
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct CreateProjectJobAssignmentResponse {
    id: JobAssignmentUuid,
}
