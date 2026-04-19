use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::ResolvedItem;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::{CharacterId, LocationId};

use crate::api_docs::{InternalServerError, Unauthorized};
use crate::market::error::Result;
use crate::state::AppState;
use crate::utils::api_client_auth;

const SCOPE: &str = "esi-assets.read_assets.v1";

/// Fetch Player Market
/// 
/// - Alternative route: `/latest/eve/corporations/{CharacterId}/assets`
/// - Alternative route: `/v1/eve/corporations/{CharacterId}/assets`
/// 
/// ---
/// 
/// Resolves the market data for the given region
/// 
#[utoipa::path(
    post,
    path = "/characters/{CharacterId}/assets",
    tag = "Assets",
    params(
        CharacterId,
    ),
    responses(
        (
            body = Vec<ResolvedItem>,
            description = "Resolved asset names",
            status = OK,
        ),
        Unauthorized,
        InternalServerError,
    ),
)]
pub async fn api(
    identity:               ExtractIdentity,
    State(state):           State<AppState>,
    Path(character_id):     Path<CharacterId>,
    Json(assets):           Json<Vec<LocationId>>,
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

    let path = format!("latest/characters/{character_id}/assets/names");
    let asset_names:  Vec<ResolvedItem> = api_client
        .post::<_, Vec<ResolvedItem>>(&path, assets)
        .await?;

    if asset_names.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(asset_names)
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(asset_names),
            )
            .into_response()
        )
    }
}
