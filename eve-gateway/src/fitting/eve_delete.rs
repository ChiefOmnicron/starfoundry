use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::EveFitResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::{CharacterId, FittingId};

use crate::api_docs::{InternalServerError, NotFound};
use crate::market::error::Result;
use crate::state::AppState;
use crate::utils::api_client_auth;

const SCOPE: &str = "esi-fittings.write_fittings.v1";

/// Fetch Player Market
/// 
/// - Alternative route: `/latest/eve/character/{CharacterId}/fittings/{FittingId}`
/// - Alternative route: `/latest/eve/character/{CharacterId}/fittings/{FittingId}`
/// 
/// ---
/// 
/// Fetches the running character industry jobs
/// 
#[utoipa::path(
    delete,
    path = "/character/{CharacterId}/fittings/{FittingId}",
    tag = "Fittings",
    params(
        CharacterId,
        FittingId,
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
    identity:                           ExtractIdentity,
    Path((character_id, fitting_id)):   Path<(CharacterId, FittingId)>,
    State(state):                       State<AppState>,
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
        "latest/characters/{}/fittings/{}",
        character_id,
        fitting_id,
    );
    let fitting = api_client
        .delete::<EveFitResponse>(&path)
        .await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(fitting),
        )
        .into_response()
    )
}
