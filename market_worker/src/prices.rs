use sqlx::PgPool;
use starfoundry_lib_eve_gateway::eve_market::EveGatewayApiClientEveMarket;
use starfoundry_lib_eve_gateway::EveGatewayClient;
use starfoundry_lib_worker::Task;

use crate::error::{Error, Result};
use crate::metric::WorkerMetric;
use crate::{SERVICE_NAME, WorkerMarketTask};

pub async fn prices(
    pool: &PgPool,
    task: &mut Task<WorkerMetric, WorkerMarketTask>,
) -> Result<()> {
    let client = EveGatewayClient::new(SERVICE_NAME.into())?;
    let entries = match client
        .fetch_prices()
        .await {

        Ok(x) => {
            x
        },
        Err(e) => {
            tracing::error!("Error while fetching market data, {}", e);
            task.append_error(e.to_string());
            return Err(e.into());
        }
    };

    let mut type_ids = Vec::new();
    let mut adjusted_prices = Vec::new();
    let mut average_prices = Vec::new();

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
