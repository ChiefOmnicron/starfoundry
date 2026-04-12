use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::ResolveStructureResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::StructureId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;

use crate::structure::error::Result;
use crate::structure::services::resolve_structure;
use crate::utils::api_client_auth;

const SCOPE: &str = "esi-universe.read_structures.v1";

/// Resolve Structure
/// 
/// - Alternative route: `/latest/structures/{StructureId}`
/// - Alternative route: `/v1/structures/{StructureId}`
/// 
/// ---
/// 
/// Resolves information about a given structure id.
/// 
/// Note: The eve character needs to have access to the structure.
/// If you can search for it in-game, and find it, you are good, otherwise
/// it won't show up and return an error.
/// 
/// The `StructureId` needs to be larger than 1_000_000_000_000.
/// 
#[utoipa::path(
    get,
    path = "/{StructureId}",
    tag = "Structures",
    params(
        StructureId,
    ),
    responses(
        (
            body = ResolveStructureResponse,
            description = "General information about the structure",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
#[axum::debug_handler]
pub async fn api(
    identity:           ExtractIdentity,
    State(state):       State<AppState>,
    Path(structure_id): Path<StructureId>,
) -> Result<impl IntoResponse> {
    let api_client = api_client_auth(
            &state.postgres,
            state.eve_api_metric,
            identity.host()?,
            identity.character_id,
            vec![
                SCOPE.into(),
            ],
        )
        .await?;

    let api_client = if let Some(x) = api_client {
        x
    } else {
        return Ok(
            (
                StatusCode::UNAUTHORIZED,
            )
            .into_response()
        )
    };

    let entry = resolve_structure(
        &state.postgres,
        api_client,
        structure_id,
    ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
