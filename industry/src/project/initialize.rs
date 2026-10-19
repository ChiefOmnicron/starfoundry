use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::api_docs::{
    Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType
};
use crate::project::error::{ProjectError, Result};
use crate::{AppState, ProjectUuid, SolutionUuid};

/// Update Default Blacklist
///
/// - Alternative route: `/v1/projects/{ProjectUuid}/initialize`
/// - Alternative route: `/latest/projects/{ProjectUuid}/initialize`
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
    path = "/{ProjectUuid}/initialize",
    tag = "Project",
    request_body = InitializeProject,
    params(
        ProjectUuid,
    ),
    responses(
        (
            description = "The group was updated",
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
    Path(project_id): Path<ProjectUuid>,
    Json(info): Json<InitializeProject>,
) -> Result<impl IntoResponse> {
    initialize(&state.postgres, project_id, info.solution_id).await?;

    Ok((StatusCode::NO_CONTENT,))
}

pub async fn initialize(
    pool: &PgPool,
    project_id: ProjectUuid,
    solution_id: SolutionUuid,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectError::TransactionError)?;

    sqlx::query!(
        "
            UPDATE project
            SET
                solution_id = $2,
                status = 'READY_TO_START'
            WHERE id = $1
        ",
        *project_id,
        *solution_id,
    )
    .execute(&mut *transaction)
    .await
    .map_err(ProjectError::Initialize)?;

    sqlx::query!(
        "
            INSERT INTO project_market
            (
                project_id,
                type_id,
                quantity
            )
            SELECT $1, * FROM (
                SELECT type_id, quantity
                FROM solution_material
                WHERE solution_id = $2
            )
        ",
        *project_id,
        *solution_id,
    )
    .execute(&mut *transaction)
    .await
    .map_err(ProjectError::Initialize)?;

    sqlx::query!(
        "
            INSERT INTO project_stock
            (
                project_id,
                type_id,
                quantity
            )
            SELECT $1, * FROM (
                SELECT type_id, quantity
                FROM solution_stock
                WHERE solution_id = $2
            )
        ",
        *project_id,
        *solution_id,
    )
    .execute(&mut *transaction)
    .await
    .map_err(ProjectError::Initialize)?;

    sqlx::query!(
        "
            INSERT INTO project_excess
            (
                project_id,
                type_id,
                quantity
            )
            SELECT $1, * FROM (
                SELECT type_id, quantity
                FROM solution_excess
                WHERE solution_id = $2
                AND quantity > 0
            )
            ON CONFLICT (project_id, type_id)
            DO UPDATE SET
                quantity = EXCLUDED.quantity
        ",
        *project_id,
        *solution_id,
    )
    .execute(&mut *transaction)
    .await
    .map_err(ProjectError::Initialize)?;

    sqlx::query!(
        "
            INSERT INTO project_job
            (
                project_id,
                type_id,
                runs,
                structure_id
            )
            SELECT $1, * FROM (
                SELECT type_id, runs, structure_id
                FROM solution_manufacturing
                WHERE solution_id = $2
            )
        ",
        *project_id,
        *solution_id,
    )
    .execute(&mut *transaction)
    .await
    .map_err(ProjectError::Initialize)?;

    transaction
        .commit()
        .await
        .map_err(ProjectError::TransactionError)?;

    Ok(())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct InitializeProject {
    pub solution_id: SolutionUuid,
}
