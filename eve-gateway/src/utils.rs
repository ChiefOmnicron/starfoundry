use sqlx::PgPool;
use starfoundry_lib_eve_client::{EveApiClient, EveApiClientMetric, EveApiError, Result};
use starfoundry_lib_types::CharacterId;
use std::sync::Arc;

pub async fn api_client_auth(
    pool:           &PgPool,
    metric:         Arc<EveApiClientMetric>,
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
        metric,
        character_id,
        refresh_token,
    )
    .map(Option::Some)
}
