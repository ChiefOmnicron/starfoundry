use sqlx::PgPool;
use starfoundry_libs_eve_api::Credentials;
use starfoundry_libs_types::CharacterId;

use crate::error::{Error, Result};
use crate::task::Task;

pub async fn task(
    task:        &mut Task,
    pool:        &PgPool,
    credentials: &Credentials,
) -> Result<()> {
    let client = if let Some(client) = crate::utils::eve_api_client(
            credentials.clone(),
            CharacterId(0),
        )
        .await {
        client
    } else {
        // The client with CharacterId 0 will always be there, as we add him
        // when initializing the credential cache
        task.add_error("no default credentials");
        return Ok(())
    };

    let mut type_ids = Vec::new();
    let mut adjusted_prices = Vec::new();
    let mut average_prices = Vec::new();

    let entries = match client
        .market_prices()
        .await
        .map_err(|e| Error::ApiError(e)) {
            Ok(x) => x,
            Err(e) => {
                task.add_error(e.to_string());
                return Err(Error::NoOp);
            }
        };

    entries
        .into_iter()
        .for_each(|x| {
            adjusted_prices.push(x.adjusted_price);
            average_prices.push(x.average_price);
            type_ids.push(*x.type_id);
        });

    sqlx::query!("
            INSERT INTO market_price
            (
                adjusted_price,
                average_price,
                type_id
            )
            SELECT * FROM UNNEST(
                $1::DOUBLE PRECISION[],
                $2::DOUBLE PRECISION[],
                $3::INTEGER[]
            )
            ON CONFLICT (type_id)
            DO UPDATE SET
                adjusted_price = EXCLUDED.adjusted_price,
                average_price = EXCLUDED.average_price
        ",
            &adjusted_prices,
            &average_prices,
            &type_ids
        )
        .execute(pool)
        .await
        .map_err(Error::InsertMarketPrices)?;

    Ok(())
}
