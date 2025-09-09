use sqlx::PgPool;
use sqlx::types::Uuid;
use starfoundry_lib_items::Item;

use crate::internal::MarketEntyPerItem;
use crate::{Error, Result};
use super::{Appraisal, AppraisalItem, AppraisalTotal, MarketEntry};

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
                raw,
                price_modifier
            FROM appraisal
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
            FROM appraisal_item ai
            JOIN item i ON i.type_id = ai.type_id
            JOIN appraisal_market_price ampb ON ampb.appraisal_id = $1 AND ampb.type_id = i.type_id AND ampb.is_buy = true
            JOIN appraisal_market_price amps ON amps.appraisal_id = $1 AND amps.type_id = i.type_id AND amps.is_buy = false
            WHERE ai.appraisal_id = $1
        ",
            appraisal.id,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::DatabaseError);

    let mut total_buy = 0f64;
    let mut total_sell = 0f64;
    let mut total_volume = 0f32;
    let items = items?
        .into_iter()
        .map(|x| {
            let max_buy = x.buy_max * x.quantity as f64 * (appraisal.price_modifier as f64 / 100f64);
            total_buy += max_buy;

            let min_sell = x.sell_min * x.quantity as f64 * (appraisal.price_modifier as f64 / 100f64);
            total_sell += min_sell;

            let volume = if let Some(y) = x.repackaged {
                x.quantity as f32 * y as f32
            } else {
                x.quantity as f32 * x.volume
            };
            total_volume += volume;

            AppraisalItem {
                quantity: x.quantity,
                type_id:  x.type_id.into(),
                low_data: x.low_data,
                volume:   volume,
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

    let total = AppraisalTotal {
        buy:    total_buy,
        sell:   total_sell,
        volume: total_volume,
    };

    let appraisal = Appraisal {
        id:             Uuid::default(),
        created_at:     appraisal.created_at.and_utc().timestamp_millis(),
        code:           Some(code),

        market_id:      appraisal.structure_id,

        comment:        appraisal.comment,
        price_modifier: appraisal.price_modifier,

        items:          items,
        invalid:        invalid,

        raw:            appraisal.raw,

        total:          total,
    };

    Ok(Some(appraisal))
}
