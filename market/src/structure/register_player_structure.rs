use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::{CharacterId, RegionId, StructureId};
use utoipa::ToSchema;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::AppState;
use crate::structure::error::{Result, StructureError};

/// Add Structure
/// 
/// - Alternative route: `/latest/structures`
/// - Alternative route: `/v1/structures`
/// 
/// ---
/// 
/// Adds a new player structure to the list of structures that are periodically
/// fetched
/// 
#[utoipa::path(
    post,
    path = "/",
    tag = "structures",
    request_body = RegisterStructureRequest,
    responses(
        (
            description = "The structure was added",
            status = CREATED,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    _identity:       ExtractIdentity,
    State(state):    State<AppState>,
    Json(structure): Json<RegisterStructureRequest>,
) -> Result<impl IntoResponse> {
    sqlx::query!("
            INSERT INTO structure (
                main_character,
                character_id,
                structure_id,
                region_id,
                source
            )
            VALUES ($1, $2, $3, $4, $5)
        ",
            *structure.main_character,
            *structure.character_id,
            *structure.structure_id,
            *structure.region_id,
            structure.source,
        )
        .execute(&state.postgres)
        .await
        .map_err(StructureError::GenericSqlxError)?;

    Ok(
        (
            StatusCode::CREATED,
            Json({})
        )
    )
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct RegisterStructureRequest {
    main_character: CharacterId,
    character_id:   CharacterId,
    structure_id:   StructureId,
    region_id:      RegionId,
    source:         String,
}
