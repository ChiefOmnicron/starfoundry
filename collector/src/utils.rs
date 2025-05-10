use sqlx::PgPool;
use starfoundry_libs_eve_api::{CredentialCache, EveApiClient};
use starfoundry_libs_types::{CharacterId, CorporationId};
use std::sync::{Arc, Mutex};

use crate::error::{Error, Result};

pub async fn corporation_ids(
    pool: &PgPool,
) -> Result<Vec<CorporationId>> {
    let corporation_id = sqlx::query!(r#"
            SELECT character_id AS "corporation_id!: CorporationId"
            FROM credential
            WHERE credential_type = 'CORPORATION'
              AND character_main IS NOT NULL
              AND character_id IS NOT NULL
        "#)
        .fetch_all(pool)
        .await
        .map_err(Error::FetchCorporationIds)?
        .iter()
        .map(|x| x.corporation_id)
        .collect::<Vec<_>>();
    Ok(corporation_id)
}

pub async fn character_ids(
    pool: &PgPool,
) -> Result<Vec<CharacterId>> {
    let character_ids = sqlx::query!(r#"
            SELECT character_id AS "character_id!: CharacterId"
            FROM credential
            WHERE credential_type = 'CHARACTER'
              AND character_id IS NOT NULL
        "#)
        .fetch_all(pool)
        .await
        .map_err(Error::FetchCharacterIds)?
        .iter()
        .map(|x| x.character_id)
        .collect::<Vec<_>>();
    Ok(character_ids)
}

pub async fn eve_api_client(
    character_id:     CharacterId,
    credential_cache: Arc<Mutex<CredentialCache>>,
) -> Option<EveApiClient> {
    let cache = {
        credential_cache
            .lock()
            .unwrap()
            .clone()
    };

    if let Ok(client) = cache
        .get((*character_id).into())
        .await {
        Some(client)
    } else {
        tracing::warn!(
            "Failed to get valid credentials for {}. Skipping",
            character_id
        );
        None
    }
}
