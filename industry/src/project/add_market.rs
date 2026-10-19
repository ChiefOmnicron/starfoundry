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
use crate::{AppState, ProjectUuid};

/// List Groups
///
/// - Alternative route: `/latest/projects/{ProjectUuid}/market`
/// - Alternative route: `/v1/projects/{ProjectUuid}/market`
///
/// ---
///
/// Adds additional market entries.
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    post,
    path = "/{ProjectUuid}/market",
    tag = "projects",
    request_body = AddMarketEntryRequest,
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
    Json(entries): Json<Vec<AddMarketEntryRequest>>,
) -> Result<impl IntoResponse> {
    add_market(&state.postgres, project_id, entries).await?;

    Ok((StatusCode::CREATED, Json(())))
}

pub async fn add_market(
    pool: &PgPool,
    project_id: ProjectUuid,
    entries: Vec<AddMarketEntryRequest>,
) -> Result<()> {
    let type_ids = entries
        .iter()
        .map(|x| *x.type_id)
        .collect::<Vec<_>>();
    let quantities = entries
        .iter()
        .map(|x| x.quantity)
        .collect::<Vec<_>>();

    sqlx::query!(
        "
            INSERT INTO project_market
            (
                project_id,
                type_id,
                quantity
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[]
            )
        ",
        *project_id,
        &type_ids,
        &quantities,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(ProjectError::AddMarketEntry)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AddMarketEntryRequest {
    pub type_id:  TypeId,
    pub quantity: i32,
}
