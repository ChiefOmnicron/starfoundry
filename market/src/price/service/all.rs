use sqlx::PgPool;
use starfoundry_lib_market::PriceResponse;

use crate::price::Result;
use crate::price::error::PriceError;

pub async fn all(
    pool: &PgPool,
) -> Result<Vec<PriceResponse>> {
    let prices = sqlx::query!("
            SELECT
                type_id,
                adjusted_price,
                average_price
            FROM market_price
        ")
        .fetch_all(pool)
        .await
        .map_err(PriceError::BulkMarketPrice)?
        .into_iter()
        .map(|x| PriceResponse {
            adjusted_price: x.adjusted_price,
            average_price:  x.average_price,
            type_id:        x.type_id.into(),
        }).collect::<Vec<_>>();

    Ok(prices)
}

