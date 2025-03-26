use sqlx::PgPool;
use starfoundry_libs_eve_api::{CredentialCache, Error};
use std::sync::{Arc, Mutex};

use crate::utils;

pub async fn task(
    pool:             &PgPool,
    credential_cache: Arc<Mutex<CredentialCache>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let corporation_ids = utils::corporation_ids(pool).await?;

    for corporation_id in corporation_ids {
        let client = if let Some(client) = utils::eve_api_client(
                (*corporation_id).into(),
                credential_cache.clone()
            )
            .await {
            client
        } else {
            return Ok(())
        };

        let wallets = match client
            .wallets()
            .await {

            Err(Error::DataNotExpired(_)) |
            Err(Error::NotModified(_))    =>  {
                continue;
            },
            Err(x)                               => {
                tracing::error!("Error fetching Corporation Wallets, {x}");
                continue;
            },
            Ok(x) => x,
        };

        for wallet in wallets {
            let mut id           = Vec::new();
            let mut amount       = Vec::new();
            let mut balance      = Vec::new();
            let mut date         = Vec::new();
            let mut first_party  = Vec::new();
            let mut second_party = Vec::new();
            let mut reason       = Vec::new();
            let mut ref_type     = Vec::new();
            let mut context_id   = Vec::new();

            let entries = match client.wallet_corporation(
                wallet.division
            )
            .await {

                Err(Error::DataNotExpired(_)) |
                Err(Error::NotModified(_))    =>  {
                    continue;
                },
                Err(x)                               => {
                    tracing::error!("Error fetching Corporation Wallet Journal, {x}");
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
                reason.push(entry.reason.clone());
                context_id.push(entry.context_id.map(|x| x as i64));
            }

            sqlx::query!("
                    INSERT INTO wallet_corporation(
                        corporation,
                        division,
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
                    SELECT $1, $2, * FROM UNNEST(
                        $3::BIGINT[],
                        $4::BIGINT[],
                        $5::BIGINT[],
                        $6::FLOAT[],
                        $7::FLOAT[],
                        $8::VARCHAR[],
                        $9::VARCHAR[],
                        $10::VARCHAR[],
                        $11::BIGINT[]
                    )
                    ON CONFLICT (id)
                    DO NOTHING
                ",
                    *corporation_id as i64,
                    wallet.division as i32,
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
    }

    Ok(())
}
