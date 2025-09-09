use sqlx::PgPool;
use sqlx::types::chrono::Utc;
use sqlx::types::Uuid;
use starfoundry_lib_items::{load_items, load_type_ids, parse};
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;

use crate::{generate_code, Error, Persistance, Result};
use super::{Appraisal, AppraisalItem, AppraisalTotal, MarketEntry, MarketEntyPerItem};

pub async fn create_raw(
    pool:    &PgPool,
    raw:     String,
    options: Option<AppraisalOptions>,
) -> Result<Appraisal> {
    let item_cache = load_items(&pool)
        .await
        .map_err(Error::LoadItemCache)?;

    let parsed = parse(&item_cache, &raw);

    let mut items = HashMap::new();
    parsed
        .items
        .clone()
        .into_iter()
        .for_each(|x| {
            items
                .entry(x.type_id)
                .and_modify(|y: &mut i64| *y += x.quantity)
                .or_insert(x.quantity);
        });

    let order = parsed
        .items
        .iter()
        .map(|x| x.type_id)
        .collect::<Vec<_>>();

    let mut appraisal = create(
            &pool,
            items,
            Some(raw),
            order,
            options
        )
        .await?;
    appraisal.invalid = parsed.invalid;

    sqlx::query!("
            INSERT INTO appraisal_invalid(
                appraisal_id,

                raw
            )
            SELECT $1, * FROM UNNEST(
                $2::VARCHAR[]
            )
        ",
            appraisal.id,
            &appraisal.invalid,
        )
        .execute(pool)
        .await
        .map_err(Error::DatabaseError)?;

    Ok(appraisal)
}

pub async fn create_type_ids(
    pool:    &PgPool,
    items:   HashMap<TypeId, i64>,
    options: Option<AppraisalOptions>,
) -> Result<Appraisal> {
    let mut order = items
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    order.sort();

    let appraisal = create(
            &pool,
            items,
            None,
            order,
            options
        )
        .await?;

    Ok(appraisal)
}

async fn create(
    pool:    &PgPool,
    items:   HashMap<TypeId, i64>,
    raw:     Option<String>,
    order:   Vec<TypeId>,
    options: Option<AppraisalOptions>,
) -> Result<Appraisal> {
    let options = options.unwrap_or_default();

    let mut appraisal_items = HashMap::new();
    let item_cache = load_type_ids(&pool)
        .await
        .map_err(Error::LoadItemCache)?;

    for (type_id, quantity) in items.iter() {
        if let Some(item) = item_cache.get(&type_id) {
            appraisal_items
                .entry(item.type_id)
                .and_modify(|x: &mut AppraisalItem| x.quantity += quantity)
                .or_insert(AppraisalItem {
                    quantity: *quantity,
                    type_id:  item.type_id,
                    meta:     item.clone(),

                    low_data: false,

                    volume: 0f32,

                    buy:  MarketEntry::default(),
                    sell: MarketEntry::default(),
                });
        } else {
            continue;
        };
    }

    let mut total_buy = 0f64;
    let mut total_sell = 0f64;
    let mut total_volume = 0f32;
    for (type_id, entry) in appraisal_items.iter_mut() {
        let buy = sqlx::query!(r#"
                SELECT
                    AVG(price) AS "avg!",
                    MIN(price) AS "min!",
                    MAX(price) AS "max!",
                    SUM(remaining) AS "total_orders!"
                FROM market_order_latest
                WHERE is_buy = true
                AND structure_id = $1
                AND type_id = $2
            "#,
                options.market_id,
                **type_id,
            )
            .fetch_one(pool)
            .await;

        let sell = sqlx::query!(r#"
                SELECT
                    AVG(price) AS "avg!",
                    MIN(price) AS "min!",
                    MAX(price) AS "max!",
                    SUM(remaining) AS "total_orders!"
                FROM market_order_latest
                WHERE is_buy = false
                AND structure_id = $1
                AND type_id = $2
            "#,
                options.market_id,
                **type_id,
            )
            .fetch_one(pool)
            .await;

        let volume = if let Some(y) = entry.meta.repackaged {
            entry.quantity as f32 * y as f32
        } else {
            entry.quantity as f32 * entry.meta.volume
        };
        entry.volume = volume;
        total_volume += volume;

        if let Ok(x) = buy {
            let max_buy = x.max * entry.quantity as f64 * (options.price_modifier as f64 / 100f64);
            total_buy += max_buy;

            entry.buy = MarketEntry {
                per_item: MarketEntyPerItem {
                    avg:      x.avg,
                    max:      x.max,
                    min:      x.min,
                },
                max:          x.max * entry.quantity as f64 * (options.price_modifier as f64 / 100f64),
                min:          x.min * entry.quantity as f64 * (options.price_modifier as f64 / 100f64),
                total_orders: x.total_orders,
            };
        } else {
            entry.low_data = true;
            entry.buy = MarketEntry {
                per_item: MarketEntyPerItem {
                    avg:      0f64,
                    max:      0f64,
                    min:      0f64,
                },

                max:          0f64,
                min:          0f64,
                total_orders: 0,
            };
        };

        if let Ok(x) = sell {
            let min_sell = x.min * entry.quantity as f64 * (options.price_modifier as f64 / 100f64);
            total_sell += min_sell;

            entry.sell = MarketEntry {
                per_item: MarketEntyPerItem {
                    avg:      x.avg,
                    max:      x.max,
                    min:      x.min,
                },
                max:          x.max * entry.quantity as f64 * (options.price_modifier as f64 / 100f64),
                min:          x.min * entry.quantity as f64 * (options.price_modifier as f64 / 100f64),
                total_orders: x.total_orders,
            };
        } else {
            entry.low_data = true;
            entry.sell = MarketEntry {
                per_item: MarketEntyPerItem {
                    avg:      0f64,
                    max:      0f64,
                    min:      0f64,
                },

                max:          0f64,
                min:          0f64,
                total_orders: 0,
            };
        };
    }

    let mut sorted_items = Vec::new();
    for type_id in order.iter() {
        if let Some(x) = appraisal_items.get(&type_id) {
            sorted_items.push(x.clone());
            appraisal_items.remove(&type_id);
        } else {
            continue;
        }
    }

    let mut appraisal_id = Uuid::default();
    let (code, timestamp) = if options.store.is_persistent() {
        let code = generate_code();
            let mut transaction = pool
            .begin()
            .await
            .map_err(Error::DatabaseError)?;

        let new_appraisal = sqlx::query!("
                INSERT INTO appraisal(
                    code,
                    structure_id,

                    price_modifier,
                    comment,

                    raw
                )
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, created_at
            ",
                code,
                options.market_id,
                options.price_modifier,
                options.comment,
                raw,
            )
            .fetch_one(&mut *transaction)
            .await
            .map_err(Error::DatabaseError)?;
        appraisal_id = new_appraisal.id;

        let buy_ids = sqlx::query!("
                INSERT INTO appraisal_market_price(
                    is_buy,
                    appraisal_id,
                    type_id,

                    min,
                    max,
                    avg,
                    total_orders
                )
                SELECT true, $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::DOUBLE PRECISION[],
                    $4::DOUBLE PRECISION[],
                    $5::DOUBLE PRECISION[],
                    $6::BIGINT[]
                )
                RETURNING id
            ",
                new_appraisal.id,
                &sorted_items.iter().map(|x| *x.type_id).collect::<Vec<_>>(),
                &sorted_items.iter().map(|x| x.buy.per_item.min).collect::<Vec<_>>(),
                &sorted_items.iter().map(|x| x.buy.per_item.max).collect::<Vec<_>>(),
                &sorted_items.iter().map(|x| x.buy.per_item.avg).collect::<Vec<_>>(),
                &sorted_items.iter().map(|x| x.buy.total_orders).collect::<Vec<_>>(),
            )
            .fetch_all(&mut *transaction)
            .await
            .map_err(Error::DatabaseError)?
            .into_iter()
            .map(|x| x.id)
            .collect::<Vec<_>>();

        let sell_ids = sqlx::query!("
                INSERT INTO appraisal_market_price(
                    is_buy,
                    appraisal_id,
                    type_id,

                    min,
                    max,
                    avg,
                    total_orders
                )
                SELECT false, $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::DOUBLE PRECISION[],
                    $4::DOUBLE PRECISION[],
                    $5::DOUBLE PRECISION[],
                    $6::BIGINT[]
                )
                RETURNING id
            ",
                new_appraisal.id,
                &sorted_items.iter().map(|x| *x.type_id).collect::<Vec<_>>(),
                &sorted_items.iter().map(|x| x.sell.per_item.min).collect::<Vec<_>>(),
                &sorted_items.iter().map(|x| x.sell.per_item.max).collect::<Vec<_>>(),
                &sorted_items.iter().map(|x| x.sell.per_item.avg).collect::<Vec<_>>(),
                &sorted_items.iter().map(|x| x.sell.total_orders).collect::<Vec<_>>(),
            )
            .fetch_all(&mut *transaction)
            .await
            .map_err(Error::DatabaseError)?
            .into_iter()
            .map(|x| x.id)
            .collect::<Vec<_>>();

        sqlx::query!("
                INSERT INTO appraisal_item (
                    appraisal_id,

                    type_id,
                    quantity,

                    buy,
                    sell,

                    low_data
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::BIGINT[],
                    $4::UUID[],
                    $5::UUID[],
                    $6::BOOLEAN[]
                )
            ",
                new_appraisal.id,
                &sorted_items.iter().map(|x| *x.type_id).collect::<Vec<_>>(),
                &sorted_items.iter().map(|x| x.quantity).collect::<Vec<_>>(),
                &buy_ids,
                &sell_ids,
                &sorted_items.iter().map(|x| x.low_data).collect::<Vec<_>>(),
            )
            .execute(&mut *transaction)
            .await
            .map_err(Error::DatabaseError)?;

        transaction
            .commit()
            .await
            .map_err(Error::DatabaseError)?;

        (Some(code), new_appraisal.created_at.and_utc().timestamp_millis())
    } else {
        (None, Utc::now().naive_utc().and_utc().timestamp_millis())
    };

    let total = AppraisalTotal {
        buy:    total_buy,
        sell:   total_sell,
        volume: total_volume,
    };

    let appraisal = Appraisal {
        // id is skiped on serialization and deserialization
        id:             appraisal_id,
        created_at:     timestamp,
        code:           code,

        items:          sorted_items,
        invalid:        Vec::new(),

        comment:        options.comment,
        price_modifier: options.price_modifier,

        raw:            raw,

        market_id:      options.market_id,

        total:          total,
    };

    Ok(appraisal)
}

#[derive(Clone, Debug)]
pub struct AppraisalOptions {
    pub store:          Persistance,
    pub market_id:      i64,
    pub comment:        Option<String>,
    pub price_modifier: i16,
}

impl AppraisalOptions {
    const DEFAULT_MARKET: i64         = 60003760;
    const DEFAULT_PRICE_MODIFIER: i16 = 100i16;

    pub fn set_persist(
        &mut self,
        store: Option<Persistance>,
    ) {
        if let Some(x) = store {
            self.store = x;
        }
    }

    pub fn set_market_id(
        &mut self,
        market_id: Option<i64>,
    ) {
        if let Some(x) = market_id {
            self.market_id = x;
        }
    }

    pub fn set_comment(
        &mut self,
        comment: Option<String>,
    ) {
        self.comment = comment;
    }

    pub fn set_price_modifier(
        &mut self,
        price_modifier: Option<i16>,
    ) {
        if let Some(x) = price_modifier {
            self.price_modifier = x;
        }
    }
}

impl Default for AppraisalOptions {
    fn default() -> Self {
        Self {
            store:          Persistance::Persist,
            market_id:      Self::DEFAULT_MARKET,
            comment:        None,
            price_modifier: Self::DEFAULT_PRICE_MODIFIER,
        }
    }
}
