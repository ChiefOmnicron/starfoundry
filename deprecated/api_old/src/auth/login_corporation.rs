use sqlx::PgPool;
use starfoundry_lib_eve_api::EveApiClient;
use starfoundry_lib_types::{CharacterId, CorporationId};
use tracing::instrument;

use super::AuthError;

#[instrument(level = "error", skip(pool))]
pub async fn login_corporation(
    pool:           &PgPool,
    character_id:   CharacterId,
    corporation_id: CorporationId,
) -> Result<String, AuthError> {
    let token = sqlx::query!("
            INSERT INTO credential (
                character_id,
                character_main,
                credential_type
            )
            VALUES (
                $1,
                $2,
                'CORPORATION'
            )
            RETURNING token
        ",
            *corporation_id,
            *character_id,
        )
        .fetch_one(pool)
        .await?
        .token;

    Ok(
        EveApiClient::auth_uri(
            &token.to_string(),
            &crate::auth::ESI_CORPORATION.join(" ")
        )?
        .to_string()
    )
}
