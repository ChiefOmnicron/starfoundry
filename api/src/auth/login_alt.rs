use sqlx::PgPool;
use starfoundry_libs_eve_api::EveApiClient;
use starfoundry_libs_types::CharacterId;
use tracing::instrument;

use super::{AuthError, Intention};

#[instrument(level = "error", skip(pool))]
pub async fn login_alt(
    pool: &PgPool,
    main: CharacterId,
) -> Result<String, AuthError> {
    let token = sqlx::query!("
            INSERT INTO credential (character_main, intention, credential_type)
            VALUES ($1, $2, 'CHARACTER')
            RETURNING token
        ",
            *main,
            Intention::LoginAlt.to_string(),
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
