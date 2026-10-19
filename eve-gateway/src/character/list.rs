use std::collections::HashMap;
use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use reqwest::header::HOST;
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_eve_client::EveApiClientMetric;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::InternalServerError;
use crate::auth::error::{AuthError, Result};
use crate::character::AuthedCharacterInfo;
use crate::character::fetch_bulk::fetch_character_bulk;
use crate::state::AppState;

/// List Characters
///
/// Alternative route: `/latest/characters/corporation`
/// Alternative route: `/v1/characters/corporation`
///
/// ---
///
/// Lists all characters the main character has authed.
#[utoipa::path(
    get,
    path = "/",
    tag = "Character",
    responses(
        (
            status = OK,
            description = "List of characters that are authed",
            body = Vec<AuthedCharacterInfo>,
        ),
        InternalServerError,
    ),
)]
pub async fn api(
    identity: ExtractIdentity,
    State(state): State<AppState>,
    header: HeaderMap,
) -> Result<impl IntoResponse> {
    let host = if let Some(x) = header.get(HOST) {
        x.to_str()
            .unwrap_or_default()
    } else {
        tracing::error!("{HOST} header not present");
        return Ok((StatusCode::BAD_REQUEST,).into_response())
    };

    let entry = list_authed_characters(
        &state.postgres,
        state.eve_api_metric,
        identity.character_id,
        host.into(),
    )
    .await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

/// Fetches the character information for the given ids from the database.
/// If the character does not exist yet, it will be fetched using the EVE-API.
async fn list_authed_characters(
    pool: &PgPool,
    metric: Arc<EveApiClientMetric>,
    character_main: CharacterId,
    host: String,
) -> Result<Vec<AuthedCharacterInfo>> {
    let characters_db = sqlx::query!(
        "
            SELECT
                character_id,
                scopes
            FROM eve_credential
            WHERE
                (
                    character_id = $1 OR
                    character_main = $1
                ) AND
                domain = $2
        ",
        *character_main,
        host,
    )
    .fetch_all(pool)
    .await
    .map_err(AuthError::InsertRefreshToken)?
    .into_iter()
    .map(|x| {
        (
            x.character_id
                .into(),
            x.scopes,
        )
    })
    .collect::<HashMap<_, _>>();

    let character_ids = characters_db
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let characters = fetch_character_bulk(pool, metric, character_ids)
        .await?
        .into_iter()
        .map(|x| (x.character_id, x))
        .collect::<HashMap<_, _>>();

    let mut result = Vec::new();
    for (character_id, scopes) in characters_db {
        let info = if let Some(x) = characters.get(&character_id) {
            x
        } else {
            continue;
        };

        result.push(AuthedCharacterInfo {
            alliance_id:      info.alliance_id,
            alliance_name:    info
                .alliance_name
                .clone(),
            character_id:     info.character_id,
            character_name:   info
                .character_name
                .clone(),
            corporation_id:   info.corporation_id,
            corporation_name: info
                .corporation_name
                .clone(),
            scopes:           scopes
                .clone()
                .unwrap_or_default(),
        });
    }
    result.sort_by_key(|x| {
        x.character_name
            .clone()
    });

    Ok(result)
}
