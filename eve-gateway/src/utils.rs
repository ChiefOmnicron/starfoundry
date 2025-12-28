use sqlx::PgPool;
use starfoundry_lib_types::{CharacterId, CorporationId};

use crate::eve_client::EveApiClient;
use crate::eve_client::error::{EveApiError, Result};

pub async fn api_client_auth(
    pool:           &PgPool,
    host:           String,
    character_id:   CharacterId,
    corporation_id: Option<CorporationId>,
    min_scopes:     Vec<String>,
) -> Result<Option<EveApiClient>> {
    let result = sqlx::query!("
            SELECT refresh_token
            FROM eve_credential
            WHERE
                domain = $1 AND
                (
                    character_id = $2 OR
                    character_id = $3
                ) AND
                scopes && $4::VARCHAR[]
        ",
            host,
            *character_id,
            *corporation_id.unwrap_or(CorporationId(0)),
            &min_scopes,
        )
        .fetch_optional(pool)
        .await
        .map_err(|_| EveApiError::NoIdentity)?;

    let refresh_token = if let Some(x) = result {
        x.refresh_token
    } else {
        return Ok(None)
    };

    EveApiClient::new_with_refresh_token(
        character_id,
        corporation_id.unwrap_or(CorporationId(0)),
        refresh_token,
    )
    .map(|x| Some(x))
    .map_err(Into::into)
}
