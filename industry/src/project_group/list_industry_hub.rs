use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayApiClient;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::industry_hub::{IndustryHub, fetch};
use crate::project_group::ProjectGroupError;
use crate::project_group::error::Result;
use crate::{AppState, ProjectGroupUuid, eve_gateway_api_client};

/// List Industry Hubs
///
/// - Alternative route:
///   `/latest/project-groups/{ProjectGroupUuid}/industry-hubs`
/// Alternative route: `/v1/project-groups/{ProjectGroupUuid}/industry-hubs`
///
/// ---
///
/// Fetches all configured industry hubs
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    get,
    path = "/{ProjectGroupUuid}/industry-hubs",
    tag = "Project Groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            body = Vec<IndustryHub>,
            description = "All industry hubs configured in the project group",
            status = OK,
        ),
        (
            description = "No industry hubs configured",
            status = NO_CONTENT,
        ),
        NotFound,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity: ExtractIdentity,
    State(state): State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
) -> Result<impl IntoResponse> {
    let data = list_industry_hubs(
        &state.postgres,
        &eve_gateway_api_client()?,
        identity.character_id,
        project_group_uuid,
    )
    .await?;

    if data.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(data)).into_response())
    } else {
        Ok((StatusCode::OK, Json(data)).into_response())
    }
}

pub async fn list_industry_hubs(
    pool: &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id: CharacterId,
    project_group_uuid: ProjectGroupUuid,
) -> Result<Vec<IndustryHub>> {
    let entries = sqlx::query!(
        "
            SELECT industry_hub_id
            FROM project_group_industry_hub pgm
            WHERE project_group_id = $1
        ",
        *project_group_uuid,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ProjectGroupError::ListIndustryHubs(e, project_group_uuid))?;

    let mut hubs = Vec::new();
    for entry in entries {
        let hub = fetch(
            pool,
            eve_gateway_api_client,
            character_id,
            entry
                .industry_hub_id
                .into(),
        )
        .await?;

        if let Some(x) = hub {
            hubs.push(x);
        }
    }

    Ok(hubs)
}
