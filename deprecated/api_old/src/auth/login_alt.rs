use sqlx::PgPool;
use starfoundry_lib_eve_api::EveApiClient;
use starfoundry_lib_types::CharacterId;
use tracing::instrument;

use super::AuthError;

#[instrument(level = "error", skip(pool))]
pub async fn login_alt(
    pool: &PgPool,
    main: CharacterId,
) -> Result<String, AuthError> {
    let token = sqlx::query!("
            INSERT INTO credential (character_main, credential_type)
            VALUES ($1, 'CHARACTER')
            RETURNING token
        ",
            *main,
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
