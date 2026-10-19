use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_eve_gateway::item::{
    GasDecompressionEfficiency, OreReprocessingEfficiency
};
use starfoundry_lib_types::{StructureId, TypeId};
use utoipa::ToSchema;

use crate::api_docs::{
    Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType
};
use crate::project::error::{ProjectError, Result};
use crate::project::{ProjectJobStatus, ProjectStatus};
use crate::{AppState, ProjectUuid, TagUuid};

/// Update
///
/// - Alternative route: `/v1/project/{ProjectUuid}`
/// - Alternative route: `/latest/project/{ProjectUuid}`
///
/// ---
///
/// Updates the miscellaneous entries
///
/// ## Security
/// - authenticated
/// - project_group:write
#[utoipa::path(
    put,
    path = "/{ProjectUuid}",
    tag = "Project",
    request_body = UpdateProject,
    params(
        ProjectUuid,
    ),
    responses(
        (
            description = "The project was updated",
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
    Json(update_info): Json<UpdateProject>,
) -> Result<impl IntoResponse> {
    update(&state.postgres, project_id, update_info).await?;

    Ok((StatusCode::NO_CONTENT,))
}

pub async fn update(
    pool: &PgPool,
    project_id: ProjectUuid,
    update: UpdateProject,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectError::TransactionError)?;

    let result = sqlx::query!(
        "
            UPDATE project
            SET
                sell_price = $2,
                orderer = $3,
                name = $4,
                note = $5,
                status = $6
            WHERE id = $1
        ",
        *project_id,
        update.sell_price,
        update.orderer,
        update.name,
        update.note,
        update.status as _,
    )
    .execute(&mut *transaction)
    .await
    .map_err(ProjectError::Update)?;

    if result.rows_affected() == 0 {
        return Err(ProjectError::NotFound(project_id));
    }

    sqlx::query!(
        "
            DELETE FROM project_tag
            WHERE project_id = $1
        ",
        *project_id,
    )
    .execute(&mut *transaction)
    .await
    .map_err(ProjectError::Update)?;

    sqlx::query!(
        "
            INSERT INTO project_tag
            (
                project_id,
                tag_id
            )
            SELECT $1, * FROM UNNEST(
                $2::UUID[]
            )
        ",
        *project_id,
        &update
            .tags
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<_>>(),
    )
    .execute(&mut *transaction)
    .await
    .map_err(ProjectError::Update)?;

    sqlx::query!(
        "
            DELETE FROM project_sub_project
            WHERE project_id = $1
        ",
        *project_id,
    )
    .execute(&mut *transaction)
    .await
    .map_err(ProjectError::Update)?;

    sqlx::query!(
        "
            INSERT INTO project_sub_project
            (
                project_id,
                sub_project_id
            )
            SELECT $1, * FROM UNNEST(
                $2::UUID[]
            )
        ",
        *project_id,
        &update
            .sub_projects
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<_>>(),
    )
    .execute(&mut *transaction)
    .await
    .map_err(ProjectError::Update)?;

    transaction
        .commit()
        .await
        .map_err(ProjectError::TransactionError)?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateProject {
    pub orderer: String,
    pub name:    String,
    pub status:  ProjectStatus,
    pub tags:    Vec<TagUuid>,

    #[serde(default)]
    pub sub_projects: Vec<ProjectUuid>,

    pub sell_price: Option<f64>,
    pub note:       Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateJob {
    pub cost:   Option<f64>,
    pub job_id: Option<i32>,
    pub runs:   Option<i32>,
    pub status: ProjectJobStatus,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateMarketBulk {
    pub source:              String,
    pub entries:             Vec<UpdateMarketBulkEntry>,
    #[serde(default)]
    pub gas_decompression:   Option<GasDecompressionEfficiency>,
    #[serde(default)]
    pub mineral_compression: Option<OreReprocessingEfficiency>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateMarketBulkEntry {
    #[serde(default)]
    pub type_id:      Option<TypeId>,
    #[serde(default)]
    pub name:         Option<String>,
    #[serde(default)]
    pub structure_id: Option<StructureId>,

    pub cost:     f32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateMarketEntry {
    pub quantity: i32,
    pub cost:     Option<f64>,
    pub source:   Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateMisc {
    pub item: String,
    pub cost: f32,

    pub description: Option<String>,
    pub quantity:    Option<i32>,
}
