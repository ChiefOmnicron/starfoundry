mod create_structure;
mod service;

pub use self::create_structure::*;
pub use self::service::*;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use starfoundry_lib_gateway::ExtractIdentity;
use utoipa::ToSchema;

use crate::AppState;
use crate::structure::StructureUuid;
use crate::structure::error::Result;
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized, UnprocessableEntity, UnsupportedMediaType};

/// Create Structure
/// 
/// - Alternative route: `/latest/structure`
/// - Alternative route: `/v1/structure`
/// 
/// ---
/// 
/// Creates a new structure
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    post,
    path = "/",
    tag = "Structures",
    request_body = CreateStructure,
    responses(
        (
            body = CreateStructureResponse,
            description = "Id of the new structure",
            status = CREATED,
        ),
        BadRequest,
        Unauthorized,
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
    identity:     ExtractIdentity,
    Json(info):   Json<CreateStructure>,
) -> Result<impl IntoResponse> {
    let id = create(
        &state.pool,
        identity.character_id,
        info,
    ).await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(CreateStructureResponse {
                id: id.into(),
            })
        )
    )
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct CreateStructureResponse {
    id: StructureUuid,
}
