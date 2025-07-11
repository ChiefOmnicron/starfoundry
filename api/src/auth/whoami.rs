use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

use crate::auth::ExtractIdentity;
use crate::api_docs::Unauthorized;
use starfoundry_libs_types::CharacterId;

/// Whoami
/// 
/// Alternative route: `/latest/auth/whoami`
/// Alternative route: `/v1/auth/whoami`
/// 
/// ---
/// 
/// Gets the character id from the requesting character
/// 
#[utoipa::path(
    get,
    operation_id = "whoami",
    path = "/whoami",
    tag = "auth",
    responses(
        (
            status = OK,
            description = "Information about the requesting user",
            body = WhoAmI,
            content_type = "application/json",
        ),
        Unauthorized,
    ),
)]
// TODO: return more user information
pub async fn whoami(
    ExtractIdentity(identity): ExtractIdentity,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(WhoAmI {
            character_id: identity.character_id(),
        })
    )
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WhoAmI {
    character_id: CharacterId,
}
