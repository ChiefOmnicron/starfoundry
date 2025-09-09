mod model;
mod service;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_types::StructureId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;

use crate::auth::ExtractIdentity;
use crate::eve_client::error::EveApiError;
use crate::universe::error::Result;
use crate::universe::resolve_structure::model::ResolveStructureResponse;
use crate::universe::resolve_structure::service::resolve_structure;

/// Resolve Structure
/// 
/// - Alternative route: `/latest/universe/structures/{StructureId}`
/// - Alternative route: `/v1/universe/structures/{StructureId}`
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
    path = "/structures/{StructureId}",
    tag = "Universe",
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
pub async fn api(
    State(state):       State<AppState>,
    identity:           ExtractIdentity,
    Path(structure_id): Path<StructureId>,
) -> Result<impl IntoResponse> {
    let eve_api_client = identity
        .eve_api_client(&state.postgres)
        .await?
        .ok_or(EveApiError::ClientNotAuthenticated)?;

    let entry = resolve_structure(
        &state.postgres,
        eve_api_client,
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
