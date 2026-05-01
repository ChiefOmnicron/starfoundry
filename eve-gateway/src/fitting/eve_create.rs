use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::{EveFit, EveFitResponse};
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::{CharacterId, FittingId};

use crate::api_docs::{InternalServerError, NotFound};
use crate::market::error::Result;
use crate::state::AppState;
use crate::utils::api_client_auth;

const SCOPE: &str = "esi-fittings.write_fittings.v1";

/// Fetch Player Market
/// 
/// - Alternative route: `/latest/eve/characters/{CharacterId}/fittings`
/// - Alternative route: `/latest/eve/characters/{CharacterId}/fittings`
/// 
/// ---
/// 
/// Fetches the running character industry jobs
/// 
#[utoipa::path(
    post,
    path = "/characters/{CharacterId}/fittings",
    tag = "Fittings",
    request_body = EveFit,
    params(
        CharacterId,
    ),
    responses(
        (
            body = FittingId,
            description = "Character industry jobs",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    identity:           ExtractIdentity,
    State(state):       State<AppState>,
    Path(character_id): Path<CharacterId>,
    Json(fit):          Json<EveFit>,
) -> Result<impl IntoResponse> {
    let api_client = api_client_auth(
            &state.postgres,
            state.eve_api_metric,
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

    let path = format!(
        "latest/characters/{}/fittings",
        character_id,
    );
    let fitting = api_client
        .post::<_, EveFitResponse>(&path, fit)
        .await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(fitting),
        )
        .into_response()
    )
}
