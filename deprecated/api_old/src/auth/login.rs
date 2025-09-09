use sqlx::PgPool;
use starfoundry_lib_eve_api::EveApiClient;
use tracing::instrument;

use super::AuthError;

#[instrument(level = "error", skip(pool))]
pub async fn login(
    pool: &PgPool,
) -> Result<String, AuthError> {
    let token = sqlx::query!("
            INSERT INTO credential (credential_type)
            VALUES ('CHARACTER')
            RETURNING token
        ")
        .fetch_one(pool)
        .await?
        .token;

    Ok(
        EveApiClient::auth_uri(
            &token.to_string(),
            &crate::auth::ESI_CHARACTER.join(" ")
        )?
        .to_string()
    )
}
