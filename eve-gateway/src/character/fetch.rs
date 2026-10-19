use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_eve_client::{
    EveApiClient, EveApiClientMetric, EveCharacterInfo, EveCorporationInfo
};
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};

use crate::api_docs::{InternalServerError, NotFound};
use crate::character::CharacterInfo;
use crate::character::error::{CharacterError, Result};
use crate::state::AppState;

/// Fetch Character
///
/// - Alternative route: `/latest/characters/{CharacterId}`
/// - Alternative route: `/v1/characters/{CharacterId}`
///
/// ---
///
/// Fetches information about a character
#[utoipa::path(
    get,
    path = "/{CharacterId}",
    tag = "Character",
    params(
        CharacterId,
    ),
    responses(
        (
            body = CharacterInfo,
            description = "General information about the character",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path(character_id): Path<CharacterId>,
) -> Result<impl IntoResponse> {
    let entry =
        fetch_character(&state.postgres, state.eve_api_metric, character_id)
            .await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

/// Fetches the character information from the database.
/// If the character does not exist yet, it will be fetched using the EVE-API.
pub async fn fetch_character(
    pool: &PgPool,
    metric: Arc<EveApiClientMetric>,
    character_id: CharacterId,
) -> Result<CharacterInfo> {
    let db_lookup_result = sqlx::query!(
        "
            SELECT
                character_id,
                character_name,
                corporation_id,
                corporation_name,
                alliance_id,
                alliance_name
            FROM character
            WHERE character_id = $1
        ",
        *character_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(CharacterError::FetchCharacter)?;

    let character_info = if let Some(x) = db_lookup_result {
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
        }
    } else {
        refresh_character_in_db(pool, metric, character_id).await?
    };

    Ok(character_info)
}

/// Fetches the public information about a character from the Eve-API and
/// inserts it into the database.
pub(crate) async fn refresh_character_in_db(
    pool: &PgPool,
    metric: Arc<EveApiClientMetric>,
    character_id: CharacterId,
) -> Result<CharacterInfo> {
    let client = EveApiClient::new(metric)?;

    let eve_character = fetch_character_from_eve(&client, character_id).await?;
    let eve_character = if let Some(x) = eve_character {
        x
    } else {
        return Err(CharacterError::NotFound);
    };

    let eve_corporation =
        fetch_corporation_from_eve(&client, eve_character.corporation_id)
            .await?;
    let eve_corporation = if let Some(x) = eve_corporation {
        x
    } else {
        return Err(CharacterError::NotFound);
    };

    let alliance_name =
        fetch_alliance_name_from_eve(&client, eve_character.alliance_id)
            .await?;

    insert_character_into_db(
        pool,
        character_id,
        eve_character.clone(),
        eve_corporation.clone(),
        alliance_name.clone(),
    )
    .await?;

    Ok(CharacterInfo {
        character_name:   eve_character.name,
        character_id:     character_id,
        corporation_name: eve_corporation.name,
        corporation_id:   eve_character.corporation_id,
        alliance_name:    alliance_name,
        alliance_id:      eve_character.alliance_id,
    })
}

async fn fetch_character_from_eve(
    client: &EveApiClient,
    character_id: CharacterId,
) -> Result<Option<EveCharacterInfo>> {
    client
        .character_info(character_id)
        .await
        .map_err(Into::into)
}

async fn fetch_corporation_from_eve(
    client: &EveApiClient,
    corporation_id: CorporationId,
) -> Result<Option<EveCorporationInfo>> {
    client
        .corporation_info_by_id(corporation_id)
        .await
        .map_err(Into::into)
}

async fn fetch_alliance_name_from_eve(
    client: &EveApiClient,
    alliance_id: Option<AllianceId>,
) -> Result<Option<String>> {
    if let Some(x) = alliance_id {
        if let Ok(x) = client
            .alliance_name_by_id(x)
            .await
        {
            Ok(x)
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

async fn insert_character_into_db(
    pool: &PgPool,
    character_id: CharacterId,
    character: EveCharacterInfo,
    corporation: EveCorporationInfo,
    alliance_name: Option<String>,
) -> Result<()> {
    sqlx::query!(
        "
            INSERT INTO character (
                character_id,
                character_name,
                corporation_id,
                corporation_name,
                alliance_id,
                alliance_name
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (character_id)
            DO UPDATE SET
                corporation_id = EXCLUDED.corporation_id,
                corporation_name = EXCLUDED.corporation_name,
                alliance_id = EXCLUDED.alliance_id,
                alliance_name = EXCLUDED.alliance_name,
                updated_at = NOW()
        ",
        *character_id,
        character.name,
        *character.corporation_id,
        corporation.name,
        character
            .alliance_id
            .map(|x| *x),
        alliance_name,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(CharacterError::InsertCharacter)
}
