use sqlx::PgPool;
use starfoundry_libs_eve_api::EveApiClient;
use starfoundry_libs_types::{CharacterId, CorporationId};
use tracing::instrument;

use super::{AuthError, Intention};

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
                intention,
                credential_type
            )
            VALUES (
                $1,
                $2,
                $3,
                'CORPORATION'
            )
            RETURNING token
        ",
            *corporation_id,
            *character_id,
            Intention::LoginCorporation.to_string(),
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
