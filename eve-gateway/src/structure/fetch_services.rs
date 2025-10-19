mod model;
mod service;

pub use self::service::fetch_services;
pub use self::model::StructureServiceResponse;

use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::structure::error::Result;
use crate::item::Item;

/// Fetch structure services
/// 
/// - Alternative route: `/latest/structures/{TypeId}/services`
/// - Alternative route: `/v1/structures/{TypeId}/services`
/// 
/// ---
/// 
/// Returns all services that can be installed in the structure
/// 
#[utoipa::path(
    get,
    path = "/{TypeId}/services",
    tag = "Structures",
    params(
        TypeId,
    ),
    responses(
        (
            body = Vec<Item>,
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
    let entry = fetch_services(
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
