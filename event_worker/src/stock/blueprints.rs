use sqlx::PgPool;
use starfoundry_libs_eve_api::Credentials;
use starfoundry_libs_notification::StockBlueprint;

use crate::error::{Error, Result};
use crate::task::Task;

pub async fn task(
    task:        &mut Task,
    pool:        &PgPool,
    credentials: &Credentials,
) -> Result<()> {
    let mut has_error = false;

    let ids = sqlx::query!("
            SELECT id
            FROM stock_blueprints
        ")
        .fetch_all(pool)
        .await
        .map(|x| {
            x.into_iter()
                .map(|y| y.id)
                .collect::<Vec<_>>()
        })
        .map_err(Error::FetchStockBlueprints)?;

    for id in ids {
        let stock = StockBlueprint::new(id);
        let status = stock.send(
            pool,
            credentials
        )
        .await;

        if let Err(e) = status {
            task.add_error(e.to_string());
            tracing::error!("{e}");
            has_error = true;
        }
    }

    if has_error {
        return Err(Error::SendMessageError)
    } else {
        Ok(())
    }
}
