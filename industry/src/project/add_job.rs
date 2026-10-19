use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::project::error::{ProjectError, Result};
use crate::{AppState, ProjectUuid, StructureUuid};

/// List Groups
///
/// - Alternative route: `/latest/projects/{ProjectUuid}/job`
/// - Alternative route: `/v1/projects/{ProjectUuid}/job`
///
/// ---
///
/// Adds additional job entries.
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    post,
    path = "/{ProjectUuid}/job",
    tag = "projects",
    request_body = AddJobEntryRequest,
    params(
        ProjectUuid,
    ),
    responses(
        (
            description = "The entries were added",
            status = CREATED,
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
    Path(project_id): Path<ProjectUuid>,
    Json(entries): Json<Vec<AddJobEntryRequest>>,
) -> Result<impl IntoResponse> {
    add_job(&state.postgres, project_id, entries).await?;

    Ok((StatusCode::CREATED, Json(())))
}

pub async fn add_job(
    pool: &PgPool,
    project_id: ProjectUuid,
    entries: Vec<AddJobEntryRequest>,
) -> Result<()> {
    let type_ids = entries
        .iter()
        .map(|x| *x.type_id)
        .collect::<Vec<_>>();
    let runs = entries
        .iter()
        .map(|x| x.runs)
        .collect::<Vec<_>>();
    let structure_ids = entries
        .iter()
        .map(|x| *x.structure_id)
        .collect::<Vec<_>>();

    sqlx::query!(
        "
            INSERT INTO project_job
            (
                project_id,
                type_id,
                runs,
                structure_id
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[],
                $4::UUID[]
            )
        ",
        *project_id,
        &type_ids,
        &runs,
        &structure_ids,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(ProjectError::AddJobEntry)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AddJobEntryRequest {
    pub type_id:      TypeId,
    pub runs:         i32,
    pub structure_id: StructureUuid,
}
