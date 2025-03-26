use sqlx::PgPool;
use starfoundry_libs_eve_api::{CredentialCache, Error};
use std::sync::{Arc, Mutex};

use crate::utils;

pub async fn task(
    pool:             &PgPool,
    credential_cache: Arc<Mutex<CredentialCache>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let character_ids = utils::character_ids(pool).await?;

    for character_id in character_ids {
        let client = if let Some(client) = utils::eve_api_client(
                character_id,
                credential_cache.clone()
            )
            .await {
            client
        } else {
            return Ok(())
        };

        let mut id           = Vec::new();
        let mut amount       = Vec::new();
        let mut balance      = Vec::new();
        let mut date         = Vec::new();
        let mut first_party  = Vec::new();
        let mut second_party = Vec::new();
        let mut reason       = Vec::new();
        let mut ref_type     = Vec::new();
        let mut context_id   = Vec::new();

        let entries = match client
            .wallet_character()
            .await {

            Err(Error::DataNotExpired(_)) |
            Err(Error::NotModified(_))    =>  {
                continue;
            },
            Err(x)                               => {
                tracing::error!("Error fetching Character Wallet, {x}");
                continue;
            },
            Ok(x) => x,
        };

        for entry in entries.iter() {
            id.push(entry.id as i64);
            first_party.push(entry.first_party_id as i64);
            second_party.push(entry.second_party_id as i64);
            amount.push(entry.amount as f64);
            balance.push(entry.balance as f64);
            date.push(entry.date.clone());
            ref_type.push(entry.ref_type.clone());
            context_id.push(entry.context_id.map(|x| x as i64));

            if let Some("") = entry.reason.as_ref().map(|x| x.as_ref()) {
                reason.push(None);
            } else {
                reason.push(entry.reason.clone());
            }
        }

        sqlx::query!("
                INSERT INTO wallet_character(
                    character,
                    id,
                    receiver,
                    sender,
                    amount,
                    balance,
                    date,
                    ref_type,
                    reason,
                    context_id
                )
                SELECT $1, * FROM UNNEST(
                    $2::BIGINT[],
                    $3::BIGINT[],
                    $4::BIGINT[],
                    $5::FLOAT[],
                    $6::FLOAT[],
                    $7::VARCHAR[],
                    $8::VARCHAR[],
                    $9::VARCHAR[],
                    $10::BIGINT[]
                )
                ON CONFLICT (id)
                DO NOTHING
            ",
                *character_id as i64,
                &id,
                &first_party,
                &second_party,
                &amount,
                &balance,
                &date,
                &ref_type,
                &reason as _,
                &context_id as _,
            )
            .execute(pool)
            .await?;
    }

    Ok(())
}
