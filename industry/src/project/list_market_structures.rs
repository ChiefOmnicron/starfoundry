use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayApiClient;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{
    BadRequest, InternalServerError, NotFound, Unauthorized
};
use crate::project::error::{ProjectError, Result};
use crate::structure::Structure;
use crate::structure::fetch::FetchStructureQuery;
use crate::{AppState, ProjectUuid, eve_gateway_api_client};

/// List Market
///
/// - Alternative route: `/latest/projects/{ProjectUuid}/market/structures`
/// - Alternative route: `/v1/projects/{ProjectUuid}/market/structures`
///
/// ---
///
/// Lists all default markets for a project
///
/// ## Security
/// - authenticated
/// - project:read
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/market/structures",
    tag = "projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = Vec<Structure>,
            description = "List all market structures for a project",
            status = OK,
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
    identity: ExtractIdentity,
    State(state): State<AppState>,
    Path(project_id): Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    let data = list_market_structures(
        &state.postgres,
        identity.character_id,
        &eve_gateway_api_client()?,
        project_id,
    )
    .await?;

    if data.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(data)).into_response())
    } else {
        Ok((StatusCode::OK, Json(data)).into_response())
    }
}

pub async fn list_market_structures(
    pool: &PgPool,
    character_id: CharacterId,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_id: ProjectUuid,
) -> Result<Vec<Structure>> {
    let markets = sqlx::query!("
            SELECT structure_id
            FROM project p
            JOIN project_group_default_market pgdm ON pgdm.project_group_id = p.project_group_id
            WHERE p.id = $1
        ",
            *project_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ProjectError::Fetch(e, project_id))?
        .into_iter()
        .map(|x| x.structure_id.into())
        .collect::<Vec<_>>();

    let mut structures = Vec::new();
    for market in markets {
        if let Some(x) = crate::structure::fetch::fetch(
            pool,
            eve_gateway_api_client,
            character_id,
            market,
            FetchStructureQuery::default(),
        )
        .await?
        {
            structures.push(x);
        } else {
            // silently ignore errors
            tracing::debug!("Couldn't find structure {}", market);
            continue;
        }
    }

    Ok(structures)
}
