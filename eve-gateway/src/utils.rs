use sqlx::PgPool;
use starfoundry_lib_types::{CharacterId, CorporationId};

use crate::eve_client::EveApiClient;
use crate::eve_client::error::{EveApiError, Result};

pub async fn api_client_auth(
    pool:           &PgPool,
    host:           String,
    character_id:   CharacterId,
    min_scopes:     Vec<String>,
) -> Result<Option<EveApiClient>> {
    let result = sqlx::query!("
            SELECT refresh_token
            FROM eve_credential
            WHERE
                domain = $1 AND
                (
                    character_id = $2
                ) AND
                scopes && $3::VARCHAR[]
        ",
            host,
            *character_id,
            &min_scopes,
        )
        .fetch_optional(pool)
        .await
        .map_err(|_| EveApiError::NoIdentity)?;

    let refresh_token = if let Some(x) = result {
        x.refresh_token
    } else {
        tracing::warn!("No character found with scope {} for {}", min_scopes.join(","), *character_id);
        return Ok(None)
    };

    EveApiClient::new_with_refresh_token(
        character_id,
        refresh_token,
    )
    .map(|x| Some(x))
    .map_err(Into::into)
}

pub async fn api_client_corporation_auth(
    pool:           &PgPool,
    host:           String,
    character_id:   CharacterId,
    corporation_id: CorporationId,
    min_scopes:     Vec<String>,
) -> Result<Option<EveApiClient>> {
    let result = sqlx::query!("
            SELECT refresh_token
            FROM eve_credential
            WHERE
                domain = $1 AND
                (
                    character_main = $2 AND
                    character_id = $3
                ) AND
                scopes && $4::VARCHAR[]
        ",
            host,
            *character_id,
            *corporation_id,
            &min_scopes,
        )
        .fetch_optional(pool)
        .await
        .map_err(|_| EveApiError::NoIdentity)?;

    let refresh_token = if let Some(x) = result {
        x.refresh_token
    } else {
        tracing::warn!("No corporation found with scope {} for {}", min_scopes.join(","), corporation_id);
        return Ok(None)
    };

    EveApiClient::new_with_refresh_token(
        (*corporation_id).into(),
        refresh_token,
    )
    .map(|x| Some(x))
    .map_err(Into::into)
}
