use std::collections::{HashMap, hash_map};
use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_eve_client::EveApiClientMetric;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::character::CharacterInfo;
use crate::character::error::{CharacterError, Result};
use crate::character::fetch::refresh_character_in_db;
use crate::state::AppState;

/// Fetch Bulk Characters
///
/// - Alternative route: `/latest/characters/bulk`
/// - Alternative route: `/v1/characters/bulk`
///
/// ---
///
/// Fetches information about the list of given character ids.
/// The response order is not guaranteed to be be the same order the ids came in
#[utoipa::path(
    post,
    path = "/bulk",
    tag = "Character",
    request_body = Vec<CharacterId>,
    responses(
        (
            body = Vec<CharacterInfo>,
            description = "General information about the character",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Json(character_ids): Json<Vec<CharacterId>>,
) -> Result<impl IntoResponse> {
    let entry = fetch_character_bulk(
        &state.postgres,
        state.eve_api_metric,
        character_ids,
    )
    .await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

/// Fetches the character information for the given ids from the database.
/// If the character does not exist yet, it will be fetched using the EVE-API.
pub(crate) async fn fetch_character_bulk(
    pool: &PgPool,
    metric: Arc<EveApiClientMetric>,
    character_ids: Vec<CharacterId>,
) -> Result<Vec<CharacterInfo>> {
    let mut db_lookup_result = sqlx::query!(
        "
            SELECT
                character_id,
                character_name,
                corporation_id,
                corporation_name,
                alliance_id,
                alliance_name
            FROM character
            WHERE character_id = ANY($1)
        ",
        &character_ids
            .clone()
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<_>>(),
    )
    .fetch_all(pool)
    .await
    .map_err(CharacterError::FetchCharacterBulk)?
    .into_iter()
    .map(|x| {
        (
            x.character_id,
            CharacterInfo {
                character_name:   x.character_name,
                character_id:     x
                    .character_id
                    .into(),
                corporation_name: x.corporation_name,
                corporation_id:   x
                    .corporation_id
                    .into(),
                alliance_name:    x.alliance_name,
                alliance_id:      x
                    .alliance_id
                    .map(Into::into),
            },
        )
    })
    .collect::<HashMap<_, _>>();

    if db_lookup_result.len() != character_ids.len() {
        for character_id in character_ids {
            if let hash_map::Entry::Vacant(lookup_result) =
                db_lookup_result.entry(*character_id)
            {
                // ignore errors
                if let Ok(x) =
                    refresh_character_in_db(pool, metric.clone(), character_id)
                        .await
                {
                    lookup_result.insert(x);
                }
            }
        }
    }

    Ok(db_lookup_result
        .values()
        .cloned()
        .collect::<Vec<_>>())
}
