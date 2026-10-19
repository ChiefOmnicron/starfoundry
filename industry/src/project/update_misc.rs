use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::api_docs::{
    Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType
};
use crate::project::error::{ProjectError, Result};
use crate::project::update::UpdateMisc;
use crate::{AppState, ProjectUuid};

/// Update Misc
///
/// - Alternative route: `/v1/project/{ProjectUuid}/misc`
/// - Alternative route: `/latest/project/{ProjectUuid}/misc`
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
    path = "/{ProjectUuid}/misc",
    tag = "Project",
    request_body = UpdateMisc,
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
    Json(update_info): Json<Vec<UpdateMisc>>,
) -> Result<impl IntoResponse> {
    update_misc(&state.postgres, project_id, update_info).await?;

    Ok((StatusCode::NO_CONTENT,))
}

pub async fn update_misc(
    pool: &PgPool,
    project_id: ProjectUuid,
    update: Vec<UpdateMisc>,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectError::TransactionError)?;

    sqlx::query!(
        "
            DELETE FROM project_misc
            WHERE project_id = $1
        ",
        *project_id,
    )
    .execute(&mut *transaction)
    .await
    .map_err(ProjectError::Update)?;

    let items = update
        .iter()
        .map(|x| x.item.clone())
        .collect::<Vec<_>>();
    let quantity = update
        .iter()
        .map(|x| x.quantity)
        .collect::<Vec<_>>();
    let description = update
        .iter()
        .map(|x| {
            x.description
                .clone()
        })
        .collect::<Vec<_>>();
    let cost = update
        .iter()
        .map(|x| x.cost as f64)
        .collect::<Vec<_>>();

    sqlx::query!(
        "
            INSERT INTO project_misc
            (
                project_id,
                item,
                quantity,
                description,
                cost
            )
            SELECT $1, * FROM UNNEST(
                $2::VARCHAR[],
                $3::INTEGER[],
                $4::VARCHAR[],
                $5::DOUBLE PRECISION[]
            )
        ",
        *project_id,
        &items,
        &quantity as _,
        &description as _,
        &cost,
    )
    .execute(pool)
    .await
    .map_err(ProjectError::Update)?;

    transaction
        .commit()
        .await
        .map(drop)
        .map_err(ProjectError::Update)
}
