mod model;
mod service;

pub use self::model::*;
pub use self::service::*;

use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::structure::error::Result;

/// Fetch structure rigs
/// 
/// - Alternative route: `/latest/structures/{TypeId}/rigs`
/// - Alternative route: `/v1/structures/{TypeId}/rigs`
/// 
/// ---
/// 
/// Returns all rigs that can be installed in the structure
/// 
#[utoipa::path(
    get,
    path = "/{TypeId}/rigs",
    tag = "Structures",
    params(
        TypeId,
    ),
    responses(
        (
            body = StructureRigResponse,
            description = "General information about a rig",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):            State<AppState>,
    Path(structure_type_id): Path<TypeId>,
) -> Result<impl IntoResponse> {
    let entry = list_structure_rigs(
        &state.postgres,
        structure_type_id,
    ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
