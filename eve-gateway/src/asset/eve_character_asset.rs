use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::Asset;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{InternalServerError, Unauthorized};
use crate::market::error::Result;
use crate::state::AppState;
use crate::utils::api_client_auth;

const SCOPE: &str = "esi-assets.read_assets.v1";

/// Fetch Player Market
/// 
/// - Alternative route: `/latest/eve/characters/{CharacterId}/assets`
/// - Alternative route: `/v1/eve/characters/{CharacterId}/assets`
/// 
/// ---
/// 
/// Resolves the market data for the given region
/// 
#[utoipa::path(
    get,
    path = "/eve/{CharacterId}/assets",
    tag = "Assets",
    params(
        CharacterId,
    ),
    responses(
        (
            body = Vec<Asset>,
            description = "Character assets",
            status = OK,
        ),
        (
            body = Vec<Asset>,
            description = "The character has no assets",
            status = NO_CONTENT,
        ),
        Unauthorized,
        InternalServerError,
    ),
)]
pub async fn fetch_character_asset_api(
    identity:           ExtractIdentity,
    State(state):       State<AppState>,
    Path(character_id): Path<CharacterId>,
) -> Result<impl IntoResponse> {
    let api_client = api_client_auth(
            &state.postgres,
            identity.host()?,
            character_id,
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

    let path = format!("latest/characters/{character_id}/assets");
    let assets = api_client
        .fetch_page_auth::<Asset>(&path)
        .await?;

    if assets.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(assets),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(assets),
            )
            .into_response()
        )
    }
}
