use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_eve_client::EveApiClient;
use starfoundry_lib_types::{CharacterId, StructureId, SystemId, TypeId};
use utoipa::ToSchema;

use crate::api_docs::{InternalServerError, NotFound};
use crate::asset::Item;
use crate::item::fetch_item;
use crate::state::AppState;
use crate::structure::error::{Result, StructureError};
use crate::structure::fetch_rig::StructureRigResponse;
use crate::structure::fetch_service::fetch_services;
use crate::structure::list_structure_rigs::list_structure_rigs;
use crate::structure::{StructurePosition, StructureServiceResponse};
use crate::system::{System, fetch_system};
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
    identity: ExtractIdentity,
    State(state): State<AppState>,
    Path(structure_id): Path<StructureId>,
) -> Result<impl IntoResponse> {
    let api_client = api_client_auth(
        &state.postgres,
        state.eve_api_metric,
        identity.host()?,
        identity.character_id,
        vec![SCOPE.into()],
    )
    .await?;

    let api_client = if let Some(x) = api_client {
        x
    } else {
        return Ok((StatusCode::UNAUTHORIZED,).into_response())
    };

    let entry =
        resolve_structure(&state.postgres, api_client, structure_id).await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

/// Fetches information about the given location id.
/// The structure id must be larger than 1_000_000_000_000.
///
/// # Errors
///
/// - If the EVE API is not available
/// - If the [EveAuthClient] is not valid
/// - If the character does not have access to the structure
/// - If the structure does not exist
/// - If the [StructureId] is not a valid id
///
/// # Returns
/// Information about the structure
pub(crate) async fn resolve_structure(
    pool: &PgPool,
    eve_api_client: EveApiClient,
    structure_id: StructureId,
) -> Result<Option<ResolveStructureResponse>> {
    let path = format!("latest/universe/structures/{}", structure_id,);

    let response: Option<EveStructure> = eve_api_client
        .fetch_auth(&path, &())
        .await?;

    let response = if let Some(x) = response {
        x
    } else {
        return Ok(None)
    };

    let item = fetch_item(pool, response.type_id)
        .await?
        .ok_or(StructureError::ItemNotFound)?;
    let system = fetch_system(pool, response.system_id)
        .await?
        .ok_or(StructureError::SystemNotFound)?;
    let rigs = list_structure_rigs(pool, item.type_id).await?;
    let services = fetch_services(pool, item.type_id).await?;

    Ok(Some(ResolveStructureResponse {
        owner_id:             response.owner_id,
        structure_id:         structure_id,
        name:                 response.name,
        system:               system,
        item:                 item,
        position:             response.position,
        installable_rigs:     rigs,
        installable_services: services,
    }))
}

/// Represents a structure
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct EveStructure {
    /// Name of the structure
    pub owner_id:  CharacterId,
    /// Name of the structure
    pub name:      String,
    /// Id of the system the structure is located in
    #[serde(alias = "solar_system_id")]
    pub system_id: SystemId,
    /// [TypeId] of the structure
    pub type_id:   TypeId,
    /// Position of the structure in space
    pub position:  StructurePosition,
}

/// Return message for resolving a structure
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ResolveStructureResponse {
    /// ID of the in-game structure
    pub owner_id:             CharacterId,
    /// ID of the in-game structure
    pub structure_id:         StructureId,
    /// Name of the structure
    pub name:                 String,
    /// Id of the system the structure is located in
    pub system:               System,
    /// [TypeId] of the structure
    pub item:                 Item,
    /// Location in the universe
    pub position:             StructurePosition,
    /// All rigs that can be installed into the structure
    pub installable_rigs:     Vec<StructureRigResponse>,
    /// All services that can be installed into the structure
    pub installable_services: StructureServiceResponse,
}
