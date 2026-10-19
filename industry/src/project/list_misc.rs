use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;

use crate::api_docs::{
    BadRequest, InternalServerError, NotFound, Unauthorized
};
use crate::project::error::{ProjectError, Result};
use crate::project::fetch::ProjectMisc;
use crate::{AppState, ProjectUuid};

/// List Misc
///
/// - Alternative route: `/latest/projects/{ProjectUuid}/misc`
/// - Alternative route: `/v1/projects/{ProjectUuid}/misc`
///
/// ---
///
/// Lists all misc entries that belong to the project
///
/// ## Security
/// - authenticated
/// - project:read
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/misc",
    tag = "projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = Vec<ProjectMisc>,
            description = "List all misc entries for a project",
            status = OK,
        ),
        (
            description = "There aren't any misc entries",
            status = NO_CONTENT,
        ),
        NotFound,
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    _identity: ExtractIdentity,
    State(state): State<AppState>,
    Path(project_id): Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    let data = list_misc(&state.postgres, project_id).await?;

    if data.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(data)).into_response())
    } else {
        Ok((StatusCode::OK, Json(data)).into_response())
    }
}

pub async fn list_misc(
    pool: &PgPool,
    project_id: ProjectUuid,
) -> Result<Vec<ProjectMisc>> {
    let entries = sqlx::query!(
        r#"
            SELECT
                id,
                item,
                cost,
                quantity,
                description
            FROM project_misc
            WHERE project_id = $1
        "#,
        *project_id,
    )
    .fetch_all(pool)
    .await
    .map_err(ProjectError::ListMisc)?
    .into_iter()
    .map(|x| {
        ProjectMisc {
            id:   x.id,
            item: x.item,
            cost: x.cost,

            description: x.description,
            quantity:    x.quantity,
        }
    })
    .collect::<Vec<_>>();

    Ok(entries)
}
