use sqlx::PgPool;
use starfoundry_libs_eve_api::EveApiClient;
use tracing::instrument;

use super::{AuthError, Intention};

#[instrument(level = "error", skip(pool))]
pub async fn login(
    pool: &PgPool,
) -> Result<String, AuthError> {
    let token = sqlx::query!("
            INSERT INTO credential (intention, credential_type)
            VALUES ($1, 'CHARACTER')
            RETURNING token
        ",
            Intention::Login.to_string(),
        )
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
