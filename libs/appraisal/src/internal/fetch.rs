use sqlx::PgPool;
use sqlx::types::Uuid;
use starfoundry_libs_items::Item;

use crate::internal::MarketEntyPerItem;
use crate::{Error, Result};
use super::{Appraisal, AppraisalItem, MarketEntry};

pub async fn fetch(
    pool: &PgPool,
    code: String,
) -> Result<Option<Appraisal>> {
    let appraisal = sqlx::query!("
            SELECT
                id,
                created_at,
                structure_id,
                comment,
                price_modifier
            FROM appraisals
            WHERE code = $1
        ",
            code,
        )
        .fetch_optional(pool)
        .await
        .map_err(Error::DatabaseError)?;

    let appraisal = if let Some(x) = appraisal {
        x
    } else {
        return Ok(None);
    };

    let items = sqlx::query!("
            SELECT
                ai.*,
                i.name,
                i.volume,
                i.category_id,
                i.group_id,
                i.meta_group_id,
                i.repackaged,
                ampb.min AS buy_min,
                ampb.max AS buy_max,
                ampb.avg AS buy_avg,
                ampb.total_orders AS buy_total_orders,
                amps.min AS sell_min,
                amps.max AS sell_max,
                amps.avg AS sell_avg,
                amps.total_orders AS sell_total_orders
            FROM appraisal_items ai
            JOIN items i ON i.type_id = ai.type_id
            JOIN appraisal_market_prices ampb ON ampb.appraisal_id = $1 AND ampb.type_id = i.type_id AND ampb.is_buy = true
            JOIN appraisal_market_prices amps ON amps.appraisal_id = $1 AND amps.type_id = i.type_id AND amps.is_buy = false
            WHERE ai.appraisal_id = $1
        ",
            appraisal.id,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::DatabaseError);

    let items = items?
        .into_iter()
        .map(|x| AppraisalItem {
            quantity: x.quantity,
            type_id:  x.type_id.into(),
            low_data: x.low_data,
            meta: Item {
                name:          x.name,
                volume:        x.volume,
                category_id:   x.category_id.into(),
                group_id:      x.group_id.into(),
                type_id:       x.type_id.into(),
                meta_group_id: x.meta_group_id.map(|x| x.into()),
                repackaged:    x.repackaged,
            },
            buy: MarketEntry {
                per_item: MarketEntyPerItem {
                    avg:      x.buy_avg,
                    max:      x.buy_max,
                    min:      x.buy_min,
                },
                max:          x.buy_max * x.quantity as f64 * (appraisal.price_modifier as f64 / 100f64),
                min:          x.buy_min * x.quantity as f64 * (appraisal.price_modifier as f64 / 100f64),
                total_orders: x.buy_total_orders,
            },
            sell: MarketEntry {
                per_item: MarketEntyPerItem {
                    avg:      x.sell_avg,
                    max:      x.sell_max,
                    min:      x.sell_min,
                },
                max:          x.sell_max * x.quantity as f64 * (appraisal.price_modifier as f64 / 100f64),
                min:          x.sell_min * x.quantity as f64 * (appraisal.price_modifier as f64 / 100f64),
                total_orders: x.sell_total_orders,
            }
        })
        .collect::<Vec<_>>();

    let invalid = sqlx::query!("
            SELECT *
            FROM appraisal_invalid
            WHERE appraisal_id = $1
        ",
            appraisal.id,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::DatabaseError)?;

    let invalid = invalid
        .into_iter()
        .map(|x| x.raw)
        .collect::<Vec<_>>();

    let appraisal = Appraisal {
        id:             Uuid::default(),
        created_at:     appraisal.created_at.and_utc().timestamp_millis(),
        code:           Some(code),

        market_id:      appraisal.structure_id,

        comment:        appraisal.comment,
        price_modifier: appraisal.price_modifier,

        items:          items,
        invalid,
    };

    Ok(Some(appraisal))
}
