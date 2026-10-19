use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::api_docs::{
    Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType
};
use crate::project::error::{ProjectError, Result};
use crate::project::update::UpdateJob;
use crate::{AppState, ProjectJobUuid, ProjectUuid};

/// Update Project Job
///
/// - Alternative route: `/v1/projects/{ProjectUuid}/jobs/{ProjectJobUuid}`
/// - Alternative route: `/latest/projects/{ProjectUuid}/jobs/{ProjectJobUuid}`
///
/// ---
///
/// Updates the project jobs
///
/// ## Security
/// - authenticated
/// - project_group:write
#[utoipa::path(
    put,
    path = "/{ProjectUuid}/jobs/{ProjectJobUuid}",
    tag = "Project",
    request_body = UpdateJob,
    params(
        ProjectUuid,
        ProjectJobUuid,
    ),
    responses(
        (
            description = "The job was updated",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        UnsupportedMediaType,
        UnprocessableEntity,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path((project_id, project_job_id)): Path<(ProjectUuid, ProjectJobUuid)>,
    Json(update_info): Json<UpdateJob>,
) -> Result<impl IntoResponse> {
    update_job(&state.postgres, project_id, project_job_id, update_info)
        .await?;

    Ok((StatusCode::NO_CONTENT,))
}

pub async fn update_job(
    pool: &PgPool,
    project_id: ProjectUuid,
    project_job_id: ProjectJobUuid,
    update: UpdateJob,
) -> Result<()> {
    // make sure to reset the job if the cost is set reset, otherwise the job
    // detection won't pick it up again
    let job_id = if update
        .cost
        .is_none()
        && update
            .job_id
            .is_some()
    {
        None
    } else {
        update.job_id
    };

    let changes = sqlx::query!(
        "
            UPDATE project_job
            SET
                cost = $3,
                status = $4,
                job_id = $5,
                runs = COALESCE($6, runs)
            WHERE project_id = $1
            AND id = $2
        ",
        *project_id,
        *project_job_id,
        update.cost,
        update.status as _,
        job_id,
        update.runs,
    )
    .execute(pool)
    .await
    .map_err(ProjectError::Update)?;

    if changes.rows_affected() == 0 {
        return Err(ProjectError::JobNotFound(project_id, project_job_id));
    }

    Ok(())
}
